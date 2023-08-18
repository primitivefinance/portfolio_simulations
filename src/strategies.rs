use super::*;

use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange,
    math::{ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
};
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
