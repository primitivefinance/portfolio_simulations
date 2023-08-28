#![warn(missing_docs)]

//! Contains all the imports, configuration constants, type aliases, and a
//! struct to house contracts used in the simulation.

use super::*;

/// Used to label an admin `Client` the simulation is running with.
pub const ADMIN_LABEL: &str = "admin";

/// Used to label the arbitrageur `Client` the simulation is running with.
pub const ARBITRAGEUR_LABEL: &str = "arbitrageur";

// The following constants are used throughout the simulation.
/// The number of seconds in a year.
pub const SECONDS_PER_YEAR: u64 = 31556953;
/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

/// Rename the `Arc<RevmMiddleware>` type to `Client` for convenience and ease
/// of reference.
pub type Client = Arc<RevmMiddleware>;

#[derive(Clone, Debug, Deserialize)]
pub struct SimulationConfig {
    /// This struct contains some basic settings for the environment such its
    /// label, average number of transactions per block, and a seed for the
    /// transactions per block randomness.
    pub environment_parameters: EnvironmentParameters,

    /// Contains all the necessary data for the Orstein-Uhlenbeck process used
    /// in this simulation.]
    pub price_process_parameters: PriceProcessParameters,

    pub asset_token_parameters: TokenParameters,

    pub quote_token_parameters: TokenParameters,

    pub portfolio_pool_parameters: PortfolioPoolParameters,

    pub sweep_parameters: Option<SweepParameters>,
}

pub fn read_config() -> Result<SimulationConfig> {
    let mut file = fs::File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str(&contents)?)
}
