#![warn(missing_docs)]

//! Contains all the imports, configuration constants, type aliases, and a
//! struct to house contracts used in the simulation.

use std::io::Read;
pub use std::{collections::BTreeMap, env, fs, sync::Arc};

pub use anyhow::Result;
pub use arbiter_core::{
    bindings::{
        arbiter_math::ArbiterMath, arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange,
    },
    environment::EnvironmentParameters,
    manager::Manager,
    math::{float_to_wad, ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::{Connection, RevmMiddleware, RevmMiddlewareError},
};
pub use ethers::{
    abi::{AbiDecode, RawLog},
    prelude::EthLogDecode,
    providers::Middleware,
    types::{Address, Log, I256, U256},
};
pub use log::{info, warn};
pub use polars::{
    prelude::{CsvWriter, DataFrame, NamedFrom, SerWriter},
    series::Series,
};
use serde::Deserialize;
pub use serde::Serialize;
pub use serde_json::Value;

pub use crate::bindings::{
    normal_strategy::NormalStrategy,
    portfolio::{
        AllocateCall, CreatePoolCall, Portfolio, PortfolioErrors, PortfolioEvents,
        Portfolio_InvalidInvariant,
    },
    shared_types::Order,
    weth::WETH,
};

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
}

/// This struct contains constants are used to configure the Ornstein-Uhlenbeck
/// process
#[derive(Clone, Debug, Deserialize)]
pub struct PriceProcessParameters {
    /// The initial price of the asset.
    pub initial_price: f64,

    /// The mean (price) of the process.
    pub mean: f64,

    /// The standard deviation of the process.
    pub std_dev: f64,

    /// The theta parameter of the process.
    /// This describes how strongly the process will revert to the mean.
    pub theta: f64,

    /// The start time of the process.
    pub t_0: f64,

    /// The end time of the process.
    pub t_n: f64,

    /// The number of steps in the process.
    pub num_steps: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokenParameters {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// All the possible settings for the Portfolio pool.
#[derive(Clone, Debug, Deserialize)]
pub struct PortfolioPoolParameters {
    /// The implied volatility parameter in the `NormalStrategy`.
    /// Sets the "width" of a Gaussian liquidity distribution.
    pub volatility_basis_points: u16,

    /// The strike price of the `NormalStrategy`.
    /// Sets the "center" of a Gaussian liquidity distribution.
    pub strike_price: f64,

    /// The time remaining in the `NormalStrategy`.
    /// For the purposes of this simulation, this is set to 1 year.
    pub time_remaining_years: u64,

    /// Whether the `NormalStrategy` is perpetual or will change over time.
    pub is_perpetual: bool,

    /// The swap fee in basis points.
    pub fee_basis_points: u16,

    /// The priority swap fee in basis points (not needed for this simulation).
    pub priority_fee_basis_points: u16,

    pub liquidity_mantissa: u64,
    pub liquidity_exponent: u32,
    

    /// The initial price of the Portfolio pool.
    pub initial_price: f64,
}

/// All the possible contracts that this simulation will actively use, but not
/// all that are deployed!
/// Each is bound to a `Client` and can be used to interact with the contract.
/// The client in this case will be the admin.
#[derive(Clone, Debug)]
pub struct SimulationContracts {
    /// The `ArbiterToken` X contract.
    pub arbx: ArbiterToken<RevmMiddleware>,

    /// The `ArbiterToken` Y contract.
    pub arby: ArbiterToken<RevmMiddleware>,

    /// The `LiquidExchange` contract.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The `Portfolio` contract.
    pub portfolio: Portfolio<RevmMiddleware>,

    /// The `NormalStrategy` contract.
    pub normal_strategy: NormalStrategy<RevmMiddleware>,

    /// The `ArbiterMath` contract.
    pub arbiter_math: ArbiterMath<RevmMiddleware>,
}

pub fn read_config() -> Result<SimulationConfig> {
    let mut file = fs::File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(toml::from_str(&contents)?)
}
