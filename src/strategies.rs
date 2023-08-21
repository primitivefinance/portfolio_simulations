use crate::bindings::shared_types::Order;

use super::*;

use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange,
    math::{ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories}, middleware::RevmMiddlewareError,
};
use ethers::{
    abi::AbiDecode,
    types::{Address, I256, U256},
};
use log::error;

pub struct PriceChanger {
    pub trajectory: Trajectories,
    pub liquid_exchange: liquid_exchange::LiquidExchange<RevmMiddleware>,
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
    pub liquid_exchange: liquid_exchange::LiquidExchange<RevmMiddleware>,
    pub portfolio: Portfolio<RevmMiddleware>,
    pub arbiter_math: arbiter_math::ArbiterMath<RevmMiddleware>,
    pub pool_id: u64,
    pub pool_fee: U256,
    pub address: Address,
}

impl Arbitrageur {
    pub async fn new(
        liquid_exchange: liquid_exchange::LiquidExchange<RevmMiddleware>,
        portfolio: Portfolio<RevmMiddleware>,
        arbiter_math: arbiter_math::ArbiterMath<RevmMiddleware>,
        pool_id: u64,
        address: Address,
    ) -> Result<Self> {
        let prices = [
            liquid_exchange.price().call().await?,
            portfolio.get_spot_price(pool_id).call().await?,
        ];
        let pool_fee = WAD - FEE_BASIS_POINTS as u128 * 10_u128.pow(14);
        Ok(Self {
            prices,
            liquid_exchange,
            portfolio,
            arbiter_math,
            pool_id,
            pool_fee,
            address,
        })
    }

    pub async fn detect_arbitrage(&mut self) -> Result<SwapDirection> {
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

        // Check the no-arbitrage bounds
        let upper_arb_bound = WAD * liquid_exchange_price / self.pool_fee;
        let lower_arb_bound = liquid_exchange_price * self.pool_fee / WAD;

        if portfolio_price > upper_arb_bound {
            // Lower the portfolio price by selling ARBX for ARBY
            Ok(SwapDirection::XToY(
                liquid_exchange_price * WAD / portfolio_price,
            ))
        } else if portfolio_price < lower_arb_bound {
            // Raise the portfolio price by selling ARBY for ARBX
            Ok(SwapDirection::YToX(
                liquid_exchange_price * WAD / portfolio_price,
            ))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            info!("No swap occuring");
            Ok(SwapDirection::None)
        }
    }

    pub async fn execute_arbitrage(&mut self, swap_direction: SwapDirection) -> Result<()> {
        match swap_direction {
            SwapDirection::None => Ok(()),
            SwapDirection::XToY(ratio) => {
                info!("Swapping ARBX for ARBY on Portfolio");
                let order = self.compute_order_input_x(ratio).await?;
                info!("Order: {:?}", order);
                match self.portfolio.swap(order).send().await {
                    Ok(_pending_tx) => {
                        info!("Swap successful");
                    },
                    Err(e) => {
                        let middleware_error = e.as_middleware_error().unwrap();
                        if let RevmMiddlewareError::ExecutionRevert { gas_used: _, output } = middleware_error {
                            error!("Swap failed due to revert: {:?}", PortfolioErrors::decode(output));
                        }
                    }
                }
                Ok(())
            }
            SwapDirection::YToX(ratio) => {
                info!("Swapping ARBY for ARBX on Portfolio");
                let order = self.compute_order_input_y(ratio).await?;
                info!("Order: {:?}", order);
                match self.portfolio.swap(order).send().await {
                    Ok(_pending_tx) => {
                        info!("Swap successful");
                    },
                    Err(e) => {
                        let middleware_error = e.as_middleware_error().unwrap();
                        if let RevmMiddlewareError::ExecutionRevert { gas_used: _, output } = middleware_error {
                            error!("Swap failed due to revert: {:?}", PortfolioErrors::decode(output));
                        }
                    }
                }
                Ok(())
            }
        }
    }

    async fn compute_order_input_x(&self, ratio: U256) -> Result<Order> {
        let wad = I256::from_raw(WAD);
        let fee_inv = wad / I256::from_raw(self.pool_fee);
        let (reserve_x, reserve_y, liquidity, ..) = self
            .portfolio
            .pools(self.pool_id)
            .call()
            .await?;
        let reserve_x = reserve_x * WAD.as_u128() / liquidity;
        let reserve_y = reserve_y * WAD.as_u128() / liquidity;
        info!("Reserves: {}, {}", reserve_x, reserve_y);
        let sigma = I256::from(VOLATILITY_BASIS_POINTS as u64 * 10_u64.pow(14));
        info!("Ratio: {}", ratio);
        // Note that in our units here, $\sqrt{\tau} = 1$.

        let mut innermost_term =
            (self.arbiter_math.log(I256::from_raw(ratio)).call().await? * wad) / sigma;
        info!("Innermost term first: {}", innermost_term);

        innermost_term += self
            .arbiter_math
            .ppf(wad - I256::from(reserve_x))
            .call()
            .await?;
        info!("Innermost term second: {}", innermost_term);

        let cdf_term = self.arbiter_math.cdf(innermost_term).call().await?;
        info!("CDF term: {}", cdf_term);

        let input = fee_inv * (wad - I256::from(reserve_x) - cdf_term);
        info!("Input ARBX: {}", input);

        let sell_asset = true;
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

    async fn compute_order_input_y(&self, ratio: U256) -> Result<Order> {
        let wad = I256::from_raw(WAD);
        let fee_inv = wad / I256::from_raw(self.pool_fee);
        let (reserve_x, reserve_y, liquidity, ..) = self
            .portfolio
            .pools(self.pool_id)
            .call()
            .await?;
        let reserve_x = reserve_x * WAD.as_u128() / liquidity;
        let reserve_y = reserve_y * WAD.as_u128() / liquidity;
        info!("Reserves: {}, {}", reserve_x, reserve_y);
        let sigma = I256::from(VOLATILITY_BASIS_POINTS as u64 * 10_u64.pow(14));
        let k = I256::from(STRIKE_PRICE as u64 * 10_u64.pow(18));
        println!("Ratio: {}", ratio);
        // Note that in our units here, $\sqrt{\tau} = 1$.
        let mut innermost_term =
            (self.arbiter_math.log(I256::from_raw(ratio)).call().await? * wad) / sigma;
        info!("Innermost term first: {}", innermost_term);
        innermost_term += self
            .arbiter_math
            .ppf(wad - I256::from(reserve_x))
            .call()
            .await?
            - sigma;
        info!("Innermost term second: {}", innermost_term);

        let cdf_term = self.arbiter_math.cdf(innermost_term).call().await?;
        info!("CDF term: {}", cdf_term);

        let input = fee_inv * (k * cdf_term - I256::from(reserve_y));
        info!("Input ARBY: {}", input);

        let sell_asset = false;
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
