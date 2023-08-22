pub use std::sync::Arc;

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

pub use crate::bindings::{
    normal_strategy::NormalStrategy,
    portfolio::{
        AllocateCall, CreatePoolCall, Portfolio, PortfolioErrors, PortfolioEvents,
        Portfolio_InvalidInvariant,
    },
    shared_types::Order,
    weth::WETH,
};
pub use crate::strategies::*;

pub type Client = Arc<RevmMiddleware>;

// Environment
pub const ENV_LABEL: &str = "portfolio";

// Admin
pub const ADMIN_LABEL: &str = "admin";

// Secondary client
pub const CLIENT_LABEL: &str = "client";

// Tokens
pub const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
pub const ARBITER_TOKEN_X_SYMBOL: &str = "ARBX";
pub const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

pub const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
pub const ARBITER_TOKEN_Y_SYMBOL: &str = "ARBY";
pub const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

pub const BLOCK_RATE: f64 = 10.0;
pub const BLOCK_SEED: u64 = 0;

// Price and time
pub const INITIAL_PRICE: f64 = 1.0;
pub const PRICE_MEAN: f64 = 1.0;
pub const PRICE_STD_DEV: f64 = 0.01;
pub const PRICE_THETA: f64 = 0.5;
pub const T_0: f64 = 0.0;
pub const T_N: f64 = 1.0;
pub const NUM_STEPS: usize = 5;

// Portfolio pool settings
pub const VOLATILITY_BASIS_POINTS: u16 = 100;
pub const STRIKE_PRICE: f64 = 1.0;
pub const TIME_REMAINING_YEARS: u64 = 1;
pub const IS_PERPETUAL: bool = true;
pub const FEE_BASIS_POINTS: u16 = 10;
pub const PRIORITY_FEE_BASIS_POINTS: u16 = 0;
pub const SECONDS_PER_YEAR: u64 = 31556953;
pub const LIQUIDITY: u128 = 10_u128.pow(22);

// Other pub constants
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

pub struct SimulationContracts {
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    pub portfolio: Portfolio<RevmMiddleware>,
    pub normal_strategy: NormalStrategy<RevmMiddleware>,
    pub arbiter_math: ArbiterMath<RevmMiddleware>,
}
