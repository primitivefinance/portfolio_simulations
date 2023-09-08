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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub simulation_parameters: SimulationParameters,
}

pub fn read_config() -> Result<SimulationConfig> {
    let mut file = fs::File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str(&contents)?)
}

pub fn parse_config() -> Result<Vec<(SimulationConfig, String)>> {
    let simulation_config = read_config()?;
    let mut configs_with_filenames = vec![];

    if let Some(sweep_parameters) = simulation_config
        .simulation_parameters
        .sweep_parameters
        .clone()
    {
        let mut all_sweep_values = vec![];

        for (key, linspace_parameters) in sweep_parameters.sweeps.iter() {
            let sweep = linspace(
                linspace_parameters.start,
                linspace_parameters.end,
                linspace_parameters.steps,
            );
            all_sweep_values.push((key, sweep));
        }

        let mut combinations = vec![vec![]];

        for (key, values) in all_sweep_values.iter() {
            let mut new_combinations = vec![];

            for value in values {
                for existing_combination in &combinations {
                    let mut new_combination = existing_combination.clone();
                    new_combination.push((key, *value));
                    new_combinations.push(new_combination);
                }
            }

            combinations = new_combinations;
        }

        for combination in combinations {
            let mut config_json: Value = serde_json::to_value(&simulation_config)?;

            let mut filename = simulation_config.environment_parameters.label.clone();

            for (key, value) in &combination {
                if let Some(pool_parameters) = config_json.get_mut("portfolio_pool_parameters") {
                    if let Some(field) = pool_parameters.get_mut(key) {
                        *field = Value::from(*value as u16);
                    }
                }

                filename.push_str(&format!("_{}_{}", key, *value as u16));
            }

            let new_config: SimulationConfig = serde_json::from_value(config_json)?;
            configs_with_filenames.push((new_config, filename));
        }
    } else {
        info!("Not sweeping over any variables.");
        let filename = simulation_config.clone().environment_parameters.label;
        configs_with_filenames.push((simulation_config, filename));
    }

    Ok(configs_with_filenames)
}

fn linspace(start: f64, end: f64, num_points: usize) -> Vec<f64> {
    let step = (end - start) / (num_points as f64 - 1.0);
    (0..num_points).map(|i| start + step * i as f64).collect()
}
