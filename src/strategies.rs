use super::*;

pub struct PriceChanger {
    pub trajectory: Trajectories,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub index: usize,
}

impl PriceChanger {
    pub fn new(liquid_exchange: LiquidExchange<RevmMiddleware>) -> Self {
        let process = OrnsteinUhlenbeck::new(PRICE_MEAN, PRICE_STD_DEV, PRICE_THETA);
        let trajectory = process.euler_maruyama(INITIAL_PRICE, T_0, T_N, NUM_STEPS, 1, false);
        Self {
            trajectory,
            liquid_exchange,
            index: 1, // start after the initial price since it is already set on contract deployment
        }
    }

    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        info!("Updating price of liquid_exchange to: {}", price);
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub enum SwapDirection {
    XToY(U256),
    YToX(U256),
    None,
}

#[derive(Debug)]
pub struct Arbitrageur {
    pub prices: [U256; 2],
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub portfolio: Portfolio<RevmMiddleware>,
    pub arbiter_math: ArbiterMath<RevmMiddleware>,
    pub pool_id: u64,
    pub gamma_wad: U256,
    pub address: Address,
}

impl Arbitrageur {
    pub async fn new(
        liquid_exchange: LiquidExchange<RevmMiddleware>,
        portfolio: Portfolio<RevmMiddleware>,
        arbiter_math: ArbiterMath<RevmMiddleware>,
        pool_id: u64,
        address: Address,
    ) -> Result<Self> {
        let prices = [
            liquid_exchange.price().call().await?,
            portfolio.get_spot_price(pool_id).call().await?,
        ];
        let gamma_wad = WAD - FEE_BASIS_POINTS as u128 * 10_u128.pow(14);
        Ok(Self {
            prices,
            liquid_exchange,
            portfolio,
            arbiter_math,
            pool_id,
            gamma_wad,
            address,
        })
    }

    pub async fn detect_arbitrage(&mut self) -> Result<SwapDirection> {
        // Update the prices the for the arbitrageur
        let new_prices = [
            self.liquid_exchange.price().call().await?,
            self.portfolio.get_spot_price(self.pool_id).call().await?,
        ];
        let liquid_exchange_price = new_prices[0];
        let portfolio_price = new_prices[1];
        info!(
            "Arbitrageur sees prices:\n\tLiquid Exchange: {}\n\tPortfolio: {}",
            liquid_exchange_price, portfolio_price
        );

        // Compute the no-arbitrage bounds
        let upper_arb_bound = WAD * portfolio_price / self.gamma_wad;
        info!("Upper bound: {}", upper_arb_bound);

        let lower_arb_bound = portfolio_price * self.gamma_wad / WAD;
        info!("Lower bound: {}", lower_arb_bound);

        // Check if we have an arbitrage opportunity by comparing against the bounds and current price
        if liquid_exchange_price > upper_arb_bound && liquid_exchange_price > portfolio_price {
            // Raise the portfolio price by selling ARBY for ARBX
            Ok(SwapDirection::YToX(liquid_exchange_price))
        } else if liquid_exchange_price < lower_arb_bound && liquid_exchange_price < portfolio_price
        {
            // Lower the portfolio price by selling ARBX for ARBY
            Ok(SwapDirection::XToY(liquid_exchange_price))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(SwapDirection::None)
        }
    }

    pub async fn execute_arbitrage(&mut self, swap_direction: SwapDirection) -> Result<()> {
        // Compute the order to send to Portfolio, if necessary
        let mut order = match swap_direction {
            SwapDirection::None => {
                info!("No swap occuring");
                return Ok(());
            }
            SwapDirection::XToY(target_price) => {
                info!("Swapping ARBX for ARBY on Portfolio");
                self.compute_order_input_x(target_price).await?
            }
            SwapDirection::YToX(target_price) => {
                info!("Swapping ARBY for ARBX on Portfolio");
                self.compute_order_input_y(target_price).await?
            }
        };

        // The initial order size to be sent to Portfolio
        info!("Order: {:?}", order);

        // Loop and decrease the output size until the swap succeeds (should not take many iterations, if any)
        loop {
            if let Err(contract_error) = self.portfolio.swap(order.clone()).send().await {
                if let RevmMiddlewareError::ExecutionRevert {
                    gas_used: _,
                    output,
                } = contract_error.as_middleware_error().unwrap()
                {
                    let error = PortfolioErrors::decode(output)?;
                    warn!("Swap failed due to revert: {:?}", error);
                    if let PortfolioErrors::Portfolio_InvalidInvariant(
                        Portfolio_InvalidInvariant { prev: _, next: _ },
                    ) = error
                    {
                        warn!("Shrinking order output size by 0.1%");
                        order.output = order.output * 999 / 1000;
                        continue;
                    }
                }
            } else {
                info!("Swap successful");
                break;
            }
        }
        Ok(())
    }

    // TODO: Need to scale the input by the inverse of the fee parameter (gamma)

    async fn compute_order_input_x(&self, target_price_wad: U256) -> Result<Order> {
        // Get some necessary constants as I256
        let iwad = I256::from_raw(WAD);
        let sigma_iwad = I256::from(VOLATILITY_BASIS_POINTS as u64 * 10_u64.pow(14));
        let k_iwad = I256::from_raw(float_to_wad(STRIKE_PRICE));
        let target_price_iwad = I256::from_raw(target_price_wad);

        // Sell the asset (X) for the quote token (Y)
        let sell_asset = true;

        // Get the reserves and liquidity
        let (reserve_x, _reserve_y, liquidity, ..) =
            self.portfolio.pools(self.pool_id).call().await?;
        info!("Raw reserves: {}, {}", reserve_x, _reserve_y);
        info!("Liquidity: {}", liquidity);

        // Compute the virtual reserves (divide by the liquidity rescaling factor)
        let rescaling = I256::from(liquidity) / iwad;
        let virtual_reserve_x = I256::from(reserve_x) / rescaling;
        info!("Virtual reserve x: {}", virtual_reserve_x);

        // Note that in our units here, sqrt(tau) = 1.
        // R1 = 1 - CDF( ln( S / K ) / sigma + 0.5 * sigma)
        // S here is our target price
        let s_div_k_iwad = target_price_iwad * iwad / k_iwad;
        info!("S/K: {}", s_div_k_iwad);

        let inside_term_iwad = (self.arbiter_math.log(s_div_k_iwad).call().await? * iwad)
            / sigma_iwad
            + sigma_iwad / 2;
        info!("Inside term: {}", inside_term_iwad);

        let target_virtual_reserve_x =
            iwad - self.arbiter_math.cdf(inside_term_iwad).call().await?;
        info!("Target virtual reserve: {}", target_virtual_reserve_x);

        let virtual_input_x = target_virtual_reserve_x - virtual_reserve_x;
        info!("Virtual input: {}", virtual_input_x);

        let input = virtual_input_x * rescaling;
        info!("Input ARBX: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the output amount
        let output = self
            .portfolio
            .get_amount_out(
                self.pool_id,
                sell_asset,
                U256::try_from(input)?,
                self.address,
            )
            .call()
            .await?;
        info!("Output ARBY: {}", output);

        Ok(Order {
            input: input.as_u128(),
            output: output.as_u128(),
            use_max: false,
            pool_id: self.pool_id,
            sell_asset,
        })
    }

    async fn compute_order_input_y(&self, target_price_wad: U256) -> Result<Order> {
        // Get some necessary constants as I256
        let iwad = I256::from_raw(WAD);
        let sigma_iwad = I256::from(VOLATILITY_BASIS_POINTS as u64 * 10_u64.pow(14));
        let k_iwad = I256::from_raw(float_to_wad(STRIKE_PRICE));
        let target_price_iwad = I256::from_raw(target_price_wad);

        // Sell the quote token (Y) for the asset token (X)
        let sell_asset = false;

        // Get the reserves and liquidity
        let (_reserve_x, reserve_y, liquidity, ..) =
            self.portfolio.pools(self.pool_id).call().await?;
        info!("Raw reserves: {}, {}", _reserve_x, reserve_y);
        info!("Liquidity: {}", liquidity);

        // Compute the virtual reserves (divide by the liquidity rescaling factor)
        let rescaling = I256::from(liquidity) / iwad;
        let virtual_reserve_y = I256::from(reserve_y) / rescaling;
        info!("Virtual reserve y: {}", virtual_reserve_y);

        // Note that in our units here, sqrt(tau) = 1.
        // R2 = K CDF( ln( S / K ) / sigma - 0.5 * sigma)
        // S here is our target price
        let s_div_k_iwad = target_price_iwad * iwad / k_iwad;
        info!("S/K: {}", s_div_k_iwad);

        let inside_term_iwad = (self.arbiter_math.log(s_div_k_iwad).call().await? * iwad)
            / sigma_iwad
            - sigma_iwad / 2;
        info!("Inside term: {}", inside_term_iwad);

        let target_virtual_reserve_y =
            k_iwad * self.arbiter_math.cdf(inside_term_iwad).call().await? / iwad;
        info!("Target virtual reserve: {}", target_virtual_reserve_y);

        let virtual_input_y = target_virtual_reserve_y - virtual_reserve_y;
        info!("Virtual input: {}", virtual_input_y);

        let input = virtual_input_y * rescaling;
        info!("Input ARBY: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the output amount
        let output = self
            .portfolio
            .get_amount_out(
                self.pool_id,
                sell_asset,
                U256::try_from(input)?,
                self.address,
            )
            .call()
            .await?;
        info!("Output ARBX: {}", output);

        Ok(Order {
            input: input.as_u128(),
            output: output.as_u128(),
            use_max: false,
            pool_id: self.pool_id,
            sell_asset,
        })
    }
}
