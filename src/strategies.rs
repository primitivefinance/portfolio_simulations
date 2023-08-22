#![warn(missing_docs)]

//! Contains the two strategies used in the simulation.
//! - `PriceChanger` updates the price of the `LiquidExchange` contract.
//! - `Arbitrageur` detects and executes arbitrage opportunities.

use super::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PriceChanger
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl PriceChanger {
    /// Create a new `PriceChanger` with the given `LiquidExchange` contract
    /// bound to the admin `Client`. The `PriceChanger` will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    pub fn new(liquid_exchange: LiquidExchange<RevmMiddleware>) -> Self {
        let process = OrnsteinUhlenbeck::new(PRICE_MEAN, PRICE_STD_DEV, PRICE_THETA);
        let trajectory = process.euler_maruyama(INITIAL_PRICE, T_0, T_N, NUM_STEPS, 1, false);
        Self {
            trajectory,
            liquid_exchange,
            index: 1, /* start after the initial price since it is already set on contract
                       * deployment */
        }
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Arbitrageur
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// The `Arbitrageur` holds the data and has methods that allow it to detect and
/// execute arbitrage opportunities. It uses the `LiquidExchange` and
/// `Portfolio` contracts bound to the arbitrageur `Client`.
#[derive(Debug)]
pub struct Arbitrageur {
    /// The current prices of the `LiquidExchange` and `Portfolio` contracts.
    pub prices: [U256; 2],

    /// The `LiquidExchange` contract with the arbitrageur `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The `Portfolio` contract with the arbitrageur `Client`.
    pub portfolio: Portfolio<RevmMiddleware>,

    /// The `ArbiterMath` contract with the admin `Client`.
    /// Note that this is only used to compute mathematical functions and not
    /// write to state!
    pub arbiter_math: ArbiterMath<RevmMiddleware>,

    /// The pool ID of the Portfolio pool.
    pub pool_id: u64,

    /// The gamma parameter of the Portfolio pool.
    /// Equal to 1 - fee in WAD units.
    pub gamma_wad: U256,

    /// The address of the arbitrageur.
    pub address: Address,

    /// ARBX address.
    pub arbx_address: Address,

    /// ARBY address.
    pub arby_address: Address,
}

impl Arbitrageur {
    /// Creates a new `Arbitrageur` with the given `LiquidExchange` and
    /// `Portfolio` contracts bound to the arbitrageur `Client`.
    /// The `Arbitrageur` will use the `ArbiterMath` contract bound to the admin
    /// `Client` to compute mathematical functions. The `pool_id` is used to
    /// swap on the correct pool.
    pub async fn new(
        liquid_exchange: LiquidExchange<RevmMiddleware>,
        portfolio: Portfolio<RevmMiddleware>,
        arbiter_math: ArbiterMath<RevmMiddleware>,
        pool_id: u64,
    ) -> Result<Self> {
        // Get the address from the `Client` attached to the `LiquidExchange` contract.
        let address = liquid_exchange.client().default_sender().unwrap();

        // Get the two token addresses from the `LiquidExchange` contract.
        let arbx_address = liquid_exchange.arbiter_token_x().call().await?;
        let arby_address = liquid_exchange.arbiter_token_y().call().await?;

        // Get the current (initial) prices of the `LiquidExchange` and `Portfolio`
        // contracts.
        let prices = [
            liquid_exchange.price().call().await?,
            portfolio.get_spot_price(pool_id).call().await?,
        ];

        // Compute the gamma parameter of the Portfolio pool.
        let gamma_wad = WAD - FEE_BASIS_POINTS as u128 * 10_u128.pow(14);

        println!(
            "thing: {:?}",
            I256::from_raw(WAD) * I256::from_raw(WAD) / I256::from_raw(gamma_wad)
        );

        Ok(Self {
            prices,
            liquid_exchange,
            portfolio,
            arbiter_math,
            pool_id,
            gamma_wad,
            address,
            arbx_address,
            arby_address,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    pub async fn detect_arbitrage(&mut self) -> Result<SwapDirection> {
        // Update the prices the for the arbitrageur.
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

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * portfolio_price / self.gamma_wad;
        info!("Upper bound: {}", upper_arb_bound);

        let lower_arb_bound = portfolio_price * self.gamma_wad / WAD;
        info!("Lower bound: {}", lower_arb_bound);

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
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

    /// Executes an arbitrage opportunity.
    /// If the swap direction is `XtoY`, then the arbitrageur will sell ARBX for
    /// ARBY on the Portfolio contract and vice-versa. Then the arbitrageur
    /// will swap the output ARBY for ARBX on the Liquid Exchange contract (and
    /// vice-versa, respectively).
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

        // The initial order size to be sent to Portfolio.
        info!("Order: {:?}", order);

        // Loop and decrease the output size until the swap succeeds (should not take
        // many iterations, if any).
        // This is necessary because of tiny math rounding errors that can occur in the
        // Portfolio contract.
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
                info!("Portfolio swap successful");
                break;
            }
        }

        if order.sell_asset {
            info!("Swapping ARBY for ARBX on Liquid Exchange");
            self.liquid_exchange
                .swap(self.arbx_address, U256::from(order.output))
                .send()
                .await?
                .await?;
        } else {
            info!("Swapping ARBX for ARBY on Liquid Exchange");
            self.liquid_exchange
                .swap(self.arby_address, U256::from(order.output))
                .send()
                .await?
                .await?;
        }

        info!("LiquidExchange swap successfull; arbitrage successful");

        Ok(())
    }

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

        // Rescale back to the real input amount and multiply by 1/gamma to account for
        // the swap fee.
        let final_scaling_wad = rescaling * (iwad * iwad / I256::from_raw(self.gamma_wad));
        let input = virtual_input_x * final_scaling_wad / iwad;
        info!("Input ARBX: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the
        // output amount
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

        // Rescale back to the real input amount and multiply by 1/gamma to account for
        // the swap fee.
        let final_scaling_wad = rescaling * (iwad * iwad / I256::from_raw(self.gamma_wad));
        let input = virtual_input_y * final_scaling_wad / iwad;
        info!("Input ARBY: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the
        // output amount
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

/// Used to label the direction (if any) of a the Portfolio swap in a two-swap
/// arbitrage between the Portfolio and LiquidExchange contracts.
#[derive(Debug)]
pub enum SwapDirection {
    /// Swap ARBX for ARBY on the Portfolio contract with a target price of some
    /// `U256`.
    XToY(U256),

    /// Swap ARBY for ARBX on the Portfolio contract with a target price of some
    /// `U256`.
    YToX(U256),

    /// No swap is occuring.
    None,
}
