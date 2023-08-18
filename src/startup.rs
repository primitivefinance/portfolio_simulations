use crate::bindings::{portfolio::portfolio, normal_strategy::{self, NormalStrategy}};

use super::*;

use arbiter_core::bindings::arbiter_token;
use ethers::{types::{Address, U256}, prelude::EthLogDecode};
use portfolio::{CreatePoolCall, CreatePairCall, GetStrategyCall};

/// Initialize the environment and an admin client
pub fn initialize() -> Result<(Manager, Client, Client)> {
    // Create a manager and single environment using our predefined constants
    let mut manager = Manager::new();
    manager.add_environment(ENV_LABEL, BLOCK_RATE, BLOCK_SEED)?;

    let admin = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some(ADMIN_LABEL.to_string()),
    ));
    info!(
        "admin client with address {:?}",
        admin.default_sender().unwrap()
    );
    manager.start_environment(ENV_LABEL)?;

    // Create another client for the simulation
    let client = Arc::new(RevmMiddleware::new(
        manager.environments.get(ENV_LABEL).unwrap(),
        Some(CLIENT_LABEL.to_string()),
    ));
    info!(
        "client client with address {:?}",
        client.default_sender().unwrap()
    );

    Ok((manager, admin, client))
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
    normal_strategy::NormalStrategy<RevmMiddleware>,
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
    info!("arbiter token x contract deployed at {:?}", arbx.address());

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
    info!("arbiter token y contract deployed at {:?}", arby.address());

    // Deploy the liquid exchange contract
    let liquid_exchange = liquid_exchange::LiquidExchange::deploy(
        client.clone(),
        (arbx.address(), arby.address(), arbiter_core::math::float_to_wad(INITIAL_PRICE)),
    )?
    .send()
    .await?;
    info!(
        "liquid exchange contract deployed at {:?}",
        liquid_exchange.address()
    );

    // Deploy the portfolio contract
    let portfolio = portfolio::Portfolio::deploy(
        client.clone(),
        (weth.address(), Address::default(), Address::default()),
    )?
    .send()
    .await?;
    info!("portfolio contract deployed at {:?}", portfolio.address());

    // Deploy the normal strategy contract
    let normal_strategy = normal_strategy::NormalStrategy::deploy(
        client,
        portfolio.address(),
    )?.send().await?;
    info!("normal strategy contract deployed at {:?}", normal_strategy.address());

    Ok((weth, arbx, arby, liquid_exchange, portfolio, normal_strategy))
}

/// Allocate out funds to all the relevant contracts and approve them all to spend.
pub async fn allocate_and_approve(
    tokens: Vec<arbiter_token::ArbiterToken<RevmMiddleware>>,
    addresses: Vec<Address>,
) -> Result<()> {
    for address in addresses {
        for token in tokens.clone() {
            token.mint(address, U256::from(u128::MAX)).send().await?.await?;
            token.approve(address, U256::MAX).send().await?.await?;
        }
        info!(
            "allocated and approved address {:?} for both tokens.",
            address
        )
    }
    Ok(())
}

pub async fn initialize_portfolio(portfolio: portfolio::Portfolio<RevmMiddleware>, normal_strategy: NormalStrategy<RevmMiddleware>, asset: Address, quote: Address) -> Result<()> {
    // Create a pair
    portfolio
        .create_pair(asset, quote)
        .send()
        .await?
        .await?;
    let pair_id = portfolio.get_pair_id(asset, quote).call().await?;
    info!("created a pair with pair_id: {:?}", pair_id);

    // Get the strategy_args
    let strike_price_wad = arbiter_core::math::float_to_wad(STRIKE_PRICE);
    let volatility_basis_points = arbiter_core::math::float_to_wad(VOLATILITY_BASIS_POINTS);
    let duration_seconds = arbiter_core::math::float_to_wad(TIME_REMAINING_YEARS * 365.25 * 24.0 * 60.0 * 60.0);
    let price_wad = arbiter_core::math::float_to_wad(INITIAL_PRICE);
    let (strategy_args, reserve_x_per_wad, reserve_y_per_wad) = normal_strategy.get_strategy_data(strike_price_wad, volatility_basis_points, duration_seconds, IS_PERPETUAL, price_wad).call().await?;
    info!("got strategy args: {:?}", strategy_args);

    // Create a pool
    let CreatePoolCall {
        pair_id,
        reserve_x_per_wad,
        reserve_y_per_wad,
        fee_basis_points,
        priority_fee_basis_points,
        controller,
        strategy,
        strategy_args,
    } = CreatePoolCall {
        pair_id,
        reserve_x_per_wad,
        reserve_y_per_wad,
        fee_basis_points: FEE_BASIS_POINTS,
        priority_fee_basis_points: PRIORITY_FEE_BASIS_POINTS,
        controller: Address::default(),
        strategy: Address::default(), // address(0) == default strategy
        strategy_args,
    };
    
    let create_pool_output = portfolio
        .create_pool(pair_id, reserve_x_per_wad, reserve_y_per_wad, fee_basis_points, priority_fee_basis_points, controller, strategy, strategy_args)
        .send()
        .await?
        .await?
        .unwrap();
    let create_pool_log = create_pool_output.logs[0].clone();
    let portfolio_event = portfolio::PortfolioEvents::decode_log(&create_pool_log.into())?;
    
    info!("created a pool with address: {:?}", portfolio_event);

    Ok(())
}