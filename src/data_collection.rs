#![warn(missing_docs)]

//! Contains the `SimulationOutput` struct which is used to collect data during
//! the simulation and write it out to a CSV file for post processing.

use std::fmt::Debug;

use arbiter_core::{bindings::liquid_exchange::LiquidExchangeEvents, middleware::Connection};
use ethers::{
    contract::{EthLogDecode, Event},
    providers::{FilterWatcher, StreamExt},
    types::Filter,
};

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

    // Liquid Exchange ARBX reserves.
    pub liquid_exchange_reserves_x: Vec<String>,

    // Liquid Exchange ARBY reserves.
    pub liquid_exchange_reserves_y: Vec<String>,

    /// The amount of LP fees collected in ARBX.
    pub lp_fees_x: Vec<String>,

    /// The amount of LP fees collected in ARBX.
    pub lp_fees_y: Vec<String>,

    /// The ARBX balances of the arbitrageur.
    pub arbitrageur_balances_x: Vec<String>,

    /// The ARBY balances of the arbitrageur.
    pub arbitrageur_balances_y: Vec<String>,

    // Invariant of pool
    pub invariant: Vec<String>,
}

impl SimulationOutput {
    /// Create a new `SimulationOutput` struct that is completely empty.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            liquid_exchange_prices: vec![],
            portfolio_prices: vec![],
            portfolio_reserves_x: vec![],
            portfolio_reserves_y: vec![],
            liquid_exchange_reserves_x: vec![],
            liquid_exchange_reserves_y: vec![],
            lp_fees_x: vec![],
            lp_fees_y: vec![],
            arbitrageur_balances_x: vec![],
            arbitrageur_balances_y: vec![],
            invariant: vec![],
        }
    }

    /// Call this function to update each of the pieces of data that we want to
    /// collect.
    pub async fn update_output(
        &mut self,
        simulation_contracts: &SimulationContracts,
        pool_id: u64,
        arbitrageur_address: Address,
        swap_event: Option<Log>,
    ) -> Result<()> {
        self.invariant.push(
            simulation_contracts
                .portfolio
                .get_invariant(pool_id)
                .call()
                .await?
                .to_string(),
        );

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

        // Update the reserves of the Liquid Exchange.
        self.liquid_exchange_reserves_x.push(
            simulation_contracts
                .arbx
                .balance_of(simulation_contracts.liquid_exchange.address())
                .call()
                .await?
                .to_string(),
        );
        self.liquid_exchange_reserves_y.push(
            simulation_contracts
                .arby
                .balance_of(simulation_contracts.liquid_exchange.address())
                .call()
                .await?
                .to_string(),
        );

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

        // Update the LP fees collected.
        if let Some(swap) = swap_event {
            let decoded_log = PortfolioEvents::decode_log(&RawLog::from(swap))?;
            if let PortfolioEvents::SwapFilter(swap) = decoded_log {
                if swap.sell_asset {
                    self.lp_fees_x.push(swap.fee_amount_dec.to_string());
                    self.lp_fees_y.push(0.to_string())
                } else {
                    self.lp_fees_x.push(0.to_string());
                    self.lp_fees_y.push(swap.fee_amount_dec.to_string());
                }
            } else {
                panic!("This should not happen.")
            }
        } else {
            self.lp_fees_x.push(0.to_string());
            self.lp_fees_y.push(0.to_string());
        };
        Ok(())
    }

    /// Call this function to finalize the data collection by writing everything
    /// out to a CSV for post processing.
    pub fn finalize(&mut self, label: String) -> Result<()> {
        // Serialize the `SimulationOutput`and deserialize into a JSON key/value pair.
        // The key represents the field names of the struct and the value will be the
        // data.
        // BTreeMap preserves (lexicographical) order of the columns which may not match
        // the struct ordering.
        let serialized = serde_json::to_string(&self)?;
        let deserialized: BTreeMap<String, Value> = serde_json::from_str(&serialized)?;

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

        // Create a directory in the CWD to store the CSV file.
        let current_dir = env::current_dir()?;
        let output_dir = current_dir.join("output");
        fs::create_dir_all(&output_dir)?;

        // Write out the CSV file using the environment label.
        let file_path = output_dir.join(format!("{}.csv", label));
        let file = fs::File::create(file_path)?;
        let mut writer = CsvWriter::new(file);
        writer.finish(&mut dataframe)?;

        Ok(())
    }
}

// ---

pub struct EventCaptureBuilder<F: EthLogDecode + Debug> {
    events: Vec<Event<Arc<RevmMiddleware>, RevmMiddleware, F>>,
    running: bool,
}

impl<F: EthLogDecode + Debug> EventCaptureBuilder<F> {
    fn new() -> Self {
        Self {
            events: vec![],
            running: false,
        }
    }

    pub fn with_event(mut self, event: Event<Arc<RevmMiddleware>, RevmMiddleware, F>) -> Self {
        self.events.push(event);
        self
    }

    pub fn with_events<E: Into<Vec<Event<Arc<RevmMiddleware>, RevmMiddleware, F>>>>(
        mut self,
        events: E,
    ) -> Self {
        self.events.extend(events.into());
        self
    }

    pub fn build(self) -> Result<EventCapture<F>> {
        Ok(EventCapture {
            events: self.events,
            running: self.running,
        })
    }
}

pub struct EventCapture<F: EthLogDecode + Debug + 'static> {
    events: Vec<Event<Arc<RevmMiddleware>, RevmMiddleware, F>>,
    running: bool,
}

impl<F: EthLogDecode + Debug> EventCapture<F> {
    pub fn builder() -> EventCaptureBuilder<F> {
        EventCaptureBuilder::new()
    }

    pub fn run(mut self) -> Vec<tokio::task::JoinHandle<()>> {
        self.running = true;
        let events = self.events;
        let mut handles = vec![];
        for event in events {
            handles.push(tokio::spawn(async move {
                println!("Listening for events");
                let mut stream = event.stream().await.unwrap();
                while let Some(Ok(event)) = stream.next().await {
                    println!("Event: {:?}", event);
                }
            }));
        }
        handles
    }
}
