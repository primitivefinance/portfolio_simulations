use anyhow::Result;
use arbiter_core::bindings::*;
use arbiter_core::manager::Manager;
use arbiter_core::middleware::RevmMiddleware;
use ethers::providers::Middleware;
use std::sync::Arc;

mod bindings;
mod startup;

// Environment
const ENV_LABEL: &str = "portfolio";

// Tokens
const ARBITER_TOKEN_X_NAME: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_SYMBOL: &str = "Arbiter Token X";
const ARBITER_TOKEN_X_DECIMALS: u8 = 18;

const ARBITER_TOKEN_Y_NAME: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_SYMBOL: &str = "Arbiter Token Y";
const ARBITER_TOKEN_Y_DECIMALS: u8 = 18;

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut manager = Manager::new();
    manager.add_environment(ENV_LABEL, 1.0, 1)?;

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some("client".to_string()),
    ));
    println!(
        "created client with address {:?}",
        client.default_sender().unwrap()
    );
    manager.start_environment(ENV_LABEL)?;
    startup::deploy_initial_contracts(client.clone()).await?;

    Ok(())
}
