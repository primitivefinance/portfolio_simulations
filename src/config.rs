#![warn(missing_docs)]

//! Contains all the imports, configuration constants, type aliases, and a
//! struct to house contracts used in the simulation.

pub use std::{collections::BTreeMap, fs::File, sync::Arc};

pub use anyhow::Result;
pub use arbiter_core::{
    bindings::{
        arbiter_math::ArbiterMath, arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange,
    },
    manager::Manager,
    math::{float_to_wad, ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::{RevmMiddleware, RevmMiddlewareError},
};
pub use ethers::{
    abi::AbiDecode,
    prelude::EthLogDecode,
    providers::Middleware,
    types::{Address, I256, U256},
};
pub use log::{info, warn};
pub use polars::{
    prelude::{CsvWriter, DataFrame, NamedFrom, SerWriter},
    series::Series,
};
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

/// Rename the `Arc<RevmMiddleware>` type to `Client` for convenience and ease
/// of reference.
pub type Client = Arc<RevmMiddleware>;

/// Used to label the `Environment` that the simulation is running in.
pub const ENV_LABEL: &str = "portfolio";

/// Used to label an admin `Client` the simulation is running with.
pub const ADMIN_LABEL: &str = "admin";

/// Used to label the arbitrageur `Client` the simulation is running with.
pub const ARBITRAGEUR_LABEL: &str = "arbitrageur";

// We will need two tokens for the simulation.
/// The name of the first token.
pub const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
/// The symbol of the first token.
pub const ARBITER_TOKEN_X_SYMBOL: &str = "ARBX";
/// The number of decimals of the first token.
pub const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

/// The name of the second token.
pub const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
/// The symbol of the second token.
pub const ARBITER_TOKEN_Y_SYMBOL: &str = "ARBY";
/// The number of decimals of the second token.
pub const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

// We need to set the average rate at which we mine blocks as well as a seed for
// the randomness used here.
/// The mean number of transactions per block.
pub const BLOCK_RATE: f64 = 10.0;
/// The seed for the randomness used in the block rate.
pub const BLOCK_SEED: u64 = 0;

// The following constants are used to configure the Ornstein-Uhlenbeck process
/// The initial price of the asset.
pub const INITIAL_PRICE: f64 = 1.0;
/// The mean (price) of the process.
pub const PRICE_MEAN: f64 = 1.0;
/// The standard deviation of the process.
pub const PRICE_STD_DEV: f64 = 0.01;
/// The theta parameter of the process.
/// This describes how strongly the process will revert to the mean.
pub const PRICE_THETA: f64 = 0.5;
/// The start time of the process.
pub const T_0: f64 = 0.0;
/// The end time of the process.
pub const T_N: f64 = 1.0;
/// The number of steps in the process.
pub const NUM_STEPS: usize = 5;

// All the possible settings for the Portfolio pool.
/// The implied volatility parameter in the `NormalStrategy`.
/// Sets the "width" of a Gaussian liquidity distribution.
pub const VOLATILITY_BASIS_POINTS: u16 = 100;
/// The strike price of the `NormalStrategy`.
/// Sets the "center" of a Gaussian liquidity distribution.
pub const STRIKE_PRICE: f64 = 1.0;
/// The time remaining in the `NormalStrategy`.
/// For the purposes of this simulation, this is set to 1 year.
pub const TIME_REMAINING_YEARS: u64 = 1;
/// Whether the `NormalStrategy` is perpetual or will change over time.
pub const IS_PERPETUAL: bool = true;
/// The LP fee in basis points.
pub const FEE_BASIS_POINTS: u16 = 10;
/// The priority fee in basis points (not needed for this simulation).
pub const PRIORITY_FEE_BASIS_POINTS: u16 = 0;
/// Liquidity the LP (admin) will provide to the pool.
pub const LIQUIDITY: u128 = 10_u128.pow(22);

// The following constants are used throughout the simulation.
/// The number of seconds in a year.
pub const SECONDS_PER_YEAR: u64 = 31556953;
/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

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
