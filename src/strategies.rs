use super::*;

use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange,
    math::{ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
};
use ethers::utils::parse_ether;
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

pub enum SwapDirection {
    XToY,
    YToX,
    None,
}

pub struct Arbitrageur {
    pub prices: [ethers::types::U256; 2],
    pub liquid_exchange: liquid_exchange::LiquidExchange<RevmMiddleware>,
    pub portfolio: bindings::portfolio::Portfolio<RevmMiddleware>,
    pub pool_id: u64,
    pub pool_fee: ethers::types::U256,
}

impl Arbitrageur {
    pub async fn new(
        liquid_exchange: liquid_exchange::LiquidExchange<RevmMiddleware>,
        portfolio: bindings::portfolio::Portfolio<RevmMiddleware>,
        pool_id: u64,
    ) -> Result<Self> {
        let prices = [
            liquid_exchange.price().call().await?,
            portfolio.get_spot_price(pool_id).call().await?,
        ];
        let pool_fee = ethers::types::U256::from(1_u128 + FEE_BASIS_POINTS as u128 * 10_u128.pow(14));
        Ok(Self {
            prices,
            liquid_exchange,
            portfolio,
            pool_id,
            pool_fee,
        })
    }

    pub async fn detect_arbitrage(&mut self) -> Result<SwapDirection> {
        let new_prices = [
            self.liquid_exchange.price().call().await?,
            self.portfolio.get_spot_price(self.pool_id).call().await?,
        ];
        let liquid_exchange_price = new_prices[0];
        let portfolio_price = new_prices[1];
        info!("Arbitrageur sees prices:\n\tLiquid Exchange: {}\n\tPortfolio: {}", liquid_exchange_price, portfolio_price);

        // Check the no-arbitrage bounds
        let upper_arb_bound = liquid_exchange_price
            .checked_div(self.pool_fee)
            .unwrap();
        let lower_arb_bound = liquid_exchange_price
            .checked_mul(self.pool_fee)
            .unwrap()
            .checked_div(parse_ether(1.0).unwrap())
            .unwrap();

        if (portfolio_price > upper_arb_bound) | (portfolio_price < lower_arb_bound) {
            // If the prices are outside of the no-arbitrage bounds, then we can arbitrage.
            let price_difference = liquid_exchange_price.checked_sub(portfolio_price);
            if price_difference.is_none() {
                // If this difference is `None`, then the subtraction overflowed so current_price<target_price.
                Ok(SwapDirection::XToY)
            } else {
                // If the price difference is still nonzero, then we must swap with price[0]>price[1].
                Ok(SwapDirection::YToX)
            }
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(SwapDirection::None)
        }
    }

    pub async fn execute_arbitrage(&mut self, swap_direction: SwapDirection) -> Result<()> {
        match swap_direction {
            SwapDirection::None => {
                info!("No swap occuring");
                Ok(())
            },
            SwapDirection::XToY => {
                info!("Swapping ARBX for ARBY on Portfolio");
                Ok(())
            },
            SwapDirection::YToX => {
                info!("Swapping ARBY for ARBX on Portfolio");
                Ok(())
            },
        }
    }
}
