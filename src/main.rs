use anyhow::Result;
use arbiter_core::bindings::*;
use arbiter_core::manager::Manager;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use std::sync::Arc;
use log::info;

mod bindings;
mod startup;

pub type Client = Arc<RevmMiddleware>;

// Environment
const ENV_LABEL: &str = "portfolio";

// Admin
const ADMIN_LABEL: &str = "admin";

// Tokens
const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_SYMBOL: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_SYMBOL: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

const BLOCK_RATE: f64 = 10.0;
const BLOCK_SEED: u64 = 0;

const INITIAL_PRICE: u64 = 1000;

#[tokio::main]
pub async fn main() -> Result<()> {
        // Initialize the logger
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug");
        }
        env_logger::init();


    let (_manager, admin) = startup::initialize()?;
    let (_weth, _arbx, _arby, _liquid_exchange, _portfolio) =
        startup::deploy_contracts(admin.clone()).await?;

    Ok(())
}
