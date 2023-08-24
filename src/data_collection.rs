#![warn(missing_docs)]

//! Contains the `SimulationOutput` struct which is used to collect data during
//! the simulation and write it out to a CSV file for post processing.

use super::*;

/// The struct here is used to provide a collection for all the data we want to
/// collect in this simulation.
#[derive(Serialize)]
pub struct SimulationOutput {
    /// The prices of the `LiquidExchange` contract.
    pub liquid_exchange_prices: Vec<String>,

    /// The prices of the `Portfolio` contract.
    pub portfolio_prices: Vec<String>,

    /// The ARBX reserves of the `Portfolio` contract.
    pub portfolio_reserves_x: Vec<String>,

    /// The ARBY reserves of the `Portfolio` contract.
    pub portfolio_reserves_y: Vec<String>,

    /// The ARBX balances of the arbitrageur.
    pub arbitrageur_balances_x: Vec<String>,

    /// The ARBY balances of the arbitrageur.
    pub arbitrageur_balances_y: Vec<String>,
}

impl SimulationOutput {
    /// Create a new `SimulationOutput` struct that is completely empty.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            liquid_exchange_prices: Vec::with_capacity(NUM_STEPS),
            portfolio_prices: Vec::with_capacity(NUM_STEPS),
            portfolio_reserves_x: Vec::with_capacity(NUM_STEPS),
            portfolio_reserves_y: Vec::with_capacity(NUM_STEPS),
            arbitrageur_balances_x: Vec::with_capacity(NUM_STEPS),
            arbitrageur_balances_y: Vec::with_capacity(NUM_STEPS),
        }
    }

    /// Call this function to update each of the pieces of data that we want to
    /// collect.
    pub async fn update_output(
        &mut self,
        simulation_contracts: &SimulationContracts,
        pool_id: u64,
        arbitrageur_address: Address,
    ) -> Result<()> {
        // Update the prices of both exchanges.
        self.liquid_exchange_prices.push(
            simulation_contracts
                .liquid_exchange
                .price()
                .call()
                .await?
                .to_string(),
        );
        self.portfolio_prices.push(
            simulation_contracts
                .portfolio
                .get_spot_price(pool_id)
                .call()
                .await?
                .to_string(),
        );

        // Update the reserves of the Portfolio pool.
        let (reserve_x, reserve_y) = simulation_contracts
            .portfolio
            .get_pool_reserves(pool_id)
            .call()
            .await?;
        self.portfolio_reserves_x.push(reserve_x.to_string());
        self.portfolio_reserves_y.push(reserve_y.to_string());

        // Update the balances of the arbitrageur.
        self.arbitrageur_balances_x.push(
            simulation_contracts
                .arbx
                .balance_of(arbitrageur_address)
                .call()
                .await?
                .to_string(),
        );
        self.arbitrageur_balances_y.push(
            simulation_contracts
                .arby
                .balance_of(arbitrageur_address)
                .call()
                .await?
                .to_string(),
        );
        Ok(())
    }

    /// Call this function to finalize the data collection by writing everything
    /// out to a CSV for post processing.
    pub fn finalize(&mut self, label: &str) -> Result<()> {
        // Serialize the `SimulationOutput`and deserialize into a JSON key/value pair.
        // The key represents the field names of the struct and the value will be the
        // data.
        let serialized = serde_json::to_string(&self)?;
        let deserialized: HashMap<String, Value> = serde_json::from_str(&serialized)?;

        let mut series_vec = vec![];
        for (name, value) in deserialized.iter() {
            series_vec.push(Series::new(
                name,
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect::<Vec<String>>(),
            ));
        }
        let mut dataframe = DataFrame::new(series_vec)?;

        let file = File::create(format!("{label}.csv",))?;
        let mut writer = CsvWriter::new(file);
        writer.finish(&mut dataframe)?;
        Ok(())
    }
}
