use crate::bindings::portfolio::portfolio;

use super::*;

use arbiter_core::bindings::arbiter_token;
use ethers::types::Address;

/// Initialize the environment and an admin client
pub fn initialize() -> Result<(Manager, Client)> {
    // Create a manager and single environment using our predefined constants
    let mut manager = Manager::new();
    manager.add_environment(ENV_LABEL, BLOCK_RATE, BLOCK_SEED)?;

    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some(ADMIN_LABEL.to_string()),
    ));
    info!(
        "admin client with address {:?}",
        client.default_sender().unwrap()
    );
    manager.start_environment(ENV_LABEL)?;
    Ok((manager, client))
}

/// Deploy the contracts that we need for the simulations.
pub async fn deploy_contracts(
    client: Client,
) -> Result<(
    bindings::weth::WETH<RevmMiddleware>,
    arbiter_token::ArbiterToken<RevmMiddleware>,
    arbiter_token::ArbiterToken<RevmMiddleware>,
    liquid_exchange::LiquidExchange<RevmMiddleware>,
    portfolio::Portfolio<RevmMiddleware>,
)> {
    // Deploy the weth contract
    let weth = bindings::weth::WETH::deploy(client.clone(), ())?
        .send()
        .await?;
    info!("weth contract deployed at {:?}", weth.address());

    // Deploy the arbiter token x contract
    let arbx = arbiter_token::ArbiterToken::deploy(
        client.clone(),
        (
            ARBITER_TOKEN_X_NAME.to_string(),
            ARBITER_TOKEN_X_SYMBOL.to_string(),
            ARBITER_TOKEN_X_DECIMALS,
        ),
    )?
    .send()
    .await?;
    info!(
        "arbiter token x contract deployed at {:?}",
        arbx.address()
    );

    // Deploy the arbiter token y contract
    let arby = arbiter_token::ArbiterToken::deploy(
        client.clone(),
        (
            ARBITER_TOKEN_Y_NAME.to_string(),
            ARBITER_TOKEN_Y_SYMBOL.to_string(),
            ARBITER_TOKEN_Y_DECIMALS,
        ),
    )?
    .send()
    .await?;
    info!(
        "arbiter token y contract deployed at {:?}",
        arby.address()
    );

    // Deploy the liquid exchange contract
    let liquid_exchange = liquid_exchange::LiquidExchange::deploy(
        client.clone(),
        (arbx.address(), arby.address(), INITIAL_PRICE),
    )?
    .send()
    .await?;
    info!(
        "liquid exchange contract deployed at {:?}",
        liquid_exchange.address()
    );

    // Deploy the portfolio contract
    let portfolio = portfolio::Portfolio::deploy(
        client,
        (weth.address(), Address::default(), Address::default()),
    )?
    .send()
    .await?;
    info!("portfolio contract deployed at {:?}", portfolio.address());

    Ok((weth, arbx, arby, liquid_exchange, portfolio))
}

/// Allocate out funds to all the relevant contracts and approve them all to spend.
pub async fn _allocate_and_approve() -> Result<()> {
    todo!("Allocate out funds to all the relevant contracts and clients then approve them all to spend.")
}
