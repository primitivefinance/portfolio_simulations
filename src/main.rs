use anyhow::Result;
use arbiter_core::bindings::*;
use arbiter_core::manager::Manager;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use log::info;
use std::sync::Arc;

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
const NUM_STEPS: usize = 10;

// Portfolio pool settings
const VOLATILITY_BASIS_POINTS: f64 = 10_f64;
const STRIKE_PRICE: f64 = 1.0;
const TIME_REMAINING_YEARS: f64 = 1.0;
const IS_PERPETUAL: bool = true;
const FEE_BASIS_POINTS: u16 = 10;
const PRIORITY_FEE_BASIS_POINTS: u16= 0;
const BASIS_POINT_DIVISOR: f64 = 10_000.0;
const SECONDS_PER_YEAR: f64 = 31556953_f64;



#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize the logger
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let (_manager, admin, client) = startup::initialize()?;
    let (_weth, arbx, arby, liquid_exchange, portfolio, normal_strategy) =
        startup::deploy_contracts(admin.clone()).await?;

    let tokens = vec![arbx.clone(), arby.clone()];
    let addresses_to_allocate_and_approve = vec![
        admin.default_sender().unwrap(),
        client.default_sender().unwrap(),
        liquid_exchange.address(),
        portfolio.address(),
    ];
    startup::allocate_and_approve(tokens, addresses_to_allocate_and_approve).await?;
    startup::initialize_portfolio(portfolio, normal_strategy, arbx.address(), arby.address()).await?;

    // This copy of the liquid exchange used here is the one with the admin client.
    let mut price_changer = strategies::PriceChanger::new(liquid_exchange.clone());

    // Run a loop to change the prices
    for index in 0..NUM_STEPS {
        info!("\n
        Step {}", index);
        price_changer.update_price().await?;
        info!("Price is now {}", liquid_exchange.price().call().await?);
    }

    Ok(())
}