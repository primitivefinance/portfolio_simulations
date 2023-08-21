use anyhow::Result;
use arbiter_core::bindings::arbiter_token::ArbiterToken;
use arbiter_core::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::bindings::*;
use arbiter_core::manager::Manager;
use arbiter_core::middleware::RevmMiddleware;
use bindings::portfolio::*;
use ethers::providers::Middleware;
use log::info;
use std::sync::Arc;

use ethers::types::I256;

mod bindings;
mod startup;
mod strategies;

pub type Client = Arc<RevmMiddleware>;

// Environment
const ENV_LABEL: &str = "portfolio";

// Admin
const ADMIN_LABEL: &str = "admin";

// Secondary client
const CLIENT_LABEL: &str = "client";

// Tokens
const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_SYMBOL: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_SYMBOL: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

const BLOCK_RATE: f64 = 10.0;
const BLOCK_SEED: u64 = 0;

// Price and time
const INITIAL_PRICE: f64 = 1.0;
const PRICE_MEAN: f64 = 1.0;
const PRICE_STD_DEV: f64 = 0.1;
const PRICE_THETA: f64 = 0.01;
const T_0: f64 = 0.0;
const T_N: f64 = 1.0;
const NUM_STEPS: usize = 3;

// Portfolio pool settings
const VOLATILITY_BASIS_POINTS: u16 = 10;
const STRIKE_PRICE: f64 = 1.0;
const TIME_REMAINING_YEARS: u64 = 1;
const IS_PERPETUAL: bool = true;
const FEE_BASIS_POINTS: u16 = 10;
const PRIORITY_FEE_BASIS_POINTS: u16 = 0;
const SECONDS_PER_YEAR: u64 = 31556953;
const LIQUIDITY: u128 = 10_u128.pow(20);

// Other
// const WAD: u128 = 10_u128.pow(18);
const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18),0,0,0]);

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("WAD: {:?}", WAD);
    // Initialize the logger
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let (_manager, admin, arbitrageur) = startup::initialize()?;
    let (_weth, arbx, arby, liquid_exchange, portfolio, normal_strategy, arbiter_math) =
        startup::deploy_contracts(admin.clone()).await?;

    startup::allocate_and_approve(admin.clone(), arbitrageur.clone(), arbx.address(), arby.address(), liquid_exchange.address(), portfolio.address()).await?;
    let pool_id =
        startup::initialize_portfolio(&portfolio, &normal_strategy, arbx.address(), arby.address(), admin.default_sender().unwrap()).await?;

    // This copy of the liquid exchange used here is the one with the admin client.
    let mut price_changer = strategies::PriceChanger::new(liquid_exchange.clone());

    let mut arbitrageur = strategies::Arbitrageur::new(
        LiquidExchange::new(liquid_exchange.address(), arbitrageur.clone()),
        bindings::portfolio::Portfolio::new(portfolio.address(), arbitrageur.clone()),
        arbiter_math,
        pool_id,
        arbitrageur.default_sender().unwrap(),
    )
    .await?;

    // Run a loop to change the prices
    for index in 0..NUM_STEPS {
        info!("\n\tStep {}", index);
        price_changer.update_price().await?;

        let swap_direction = arbitrageur.detect_arbitrage().await?;
        info!("swap direction: {:?}", swap_direction);
        arbitrageur.execute_arbitrage(swap_direction).await?;
        info!("Reserves are: {:?}", portfolio.get_pool_reserves(pool_id).call().await?);
    }

    Ok(())
}