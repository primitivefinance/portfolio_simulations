use crate::bindings::{
    normal_strategy::{self, NormalStrategy},
    portfolio::portfolio,
};

use super::*;

use arbiter_core::bindings::arbiter_token;
use ethers::{
    prelude::EthLogDecode,
    types::{Address, U256},
};
use portfolio::CreatePoolCall;

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
    arbiter_math::ArbiterMath<RevmMiddleware>,
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
        (
            arbx.address(),
            arby.address(),
            arbiter_core::math::float_to_wad(INITIAL_PRICE),
        ),
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
    let normal_strategy =
        normal_strategy::NormalStrategy::deploy(client.clone(), portfolio.address())?
            .send()
            .await?;
    info!(
        "normal strategy contract deployed at {:?}",
        normal_strategy.address()
    );

    let arbiter_math = arbiter_math::ArbiterMath::deploy(client, ())?
        .send()
        .await?;

    Ok((
        weth,
        arbx,
        arby,
        liquid_exchange,
        portfolio,
        normal_strategy,
        arbiter_math,
    ))
}

/// Allocate out funds to all the relevant contracts and approve them all to spend.
pub async fn allocate_and_approve(
    admin: Client,
    arbitrageur: Client,
    arbx_address: Address,
    arby_address: Address,
    liquid_exchange_address: Address,
    portfolio_address: Address,
) -> Result<()> {
    let admin_tokens = [ArbiterToken::new(arbx_address, admin.clone()), ArbiterToken::new(arby_address, admin.clone())];
    let arbitrageur_tokens = [ArbiterToken::new(arbx_address, arbitrageur.clone()), ArbiterToken::new(arby_address, arbitrageur.clone())];
    let admin_address = admin.default_sender().unwrap();
    let arbitrageur_address = arbitrageur.default_sender().unwrap();

    for token in admin_tokens {
        token.mint(admin_address, U256::from(u128::MAX)).send().await?.await?;
        token.mint(arbitrageur_address, U256::from(u128::MAX)).send().await?.await?;
        token.mint(liquid_exchange_address, U256::from(u128::MAX)).send().await?.await?;
        token.approve(portfolio_address, U256::from(u128::MAX)).send().await?.await?;
    }

    for token in arbitrageur_tokens {
        token.approve(liquid_exchange_address, U256::from(u128::MAX)).send().await?.await?;
        token.approve(portfolio_address, U256::from(u128::MAX)).send().await?.await?;
    }


    Ok(())
}

pub async fn initialize_portfolio(
    portfolio: &portfolio::Portfolio<RevmMiddleware>,
    normal_strategy: &NormalStrategy<RevmMiddleware>,
    asset: Address,
    quote: Address,
    lp_address: Address,
) -> Result<u64> {
    // Create a pair
    portfolio.create_pair(asset, quote).send().await?.await?;
    let pair_id = portfolio.get_pair_id(asset, quote).call().await?;
    info!("created a pair with pair_id: {:?}", pair_id);

    // Get the strategy_args
    let strike_price_wad = arbiter_core::math::float_to_wad(STRIKE_PRICE);
    let volatility_basis_points = ethers::types::U256::from(VOLATILITY_BASIS_POINTS);
    let duration_seconds = ethers::types::U256::from(TIME_REMAINING_YEARS * SECONDS_PER_YEAR);
    let price_wad = arbiter_core::math::float_to_wad(INITIAL_PRICE);
    let (strategy_args, reserve_x_per_wad, reserve_y_per_wad) = normal_strategy
        .get_strategy_data(
            strike_price_wad,
            volatility_basis_points,
            duration_seconds,
            IS_PERPETUAL,
            price_wad,
        )
        .call()
        .await?;

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
        .create_pool(
            pair_id,
            reserve_x_per_wad,
            reserve_y_per_wad,
            fee_basis_points,
            priority_fee_basis_points,
            controller,
            strategy,
            strategy_args,
        )
        .send()
        .await?
        .await?
        .unwrap();

    // The 2nd log is the one we want
    let create_pool_log = create_pool_output.logs[1].clone();
    let portfolio_event = portfolio::PortfolioEvents::decode_log(&create_pool_log.into()).unwrap();
    if let portfolio::PortfolioEvents::CreatePoolFilter(create_pool_filter) = portfolio_event {
        let pool_id = create_pool_filter.pool_id;
        info!("created a pool with pool_id: {:?}", pool_id);
        // Provide funds to the portfolio pool
     let AllocateCall {
        use_max,
        recipient,
        pool_id,
        delta_liquidity,
        max_delta_asset,
        max_delta_quote,
    } = AllocateCall {
        use_max: false,
        recipient: lp_address,
        pool_id,
        delta_liquidity: 10_u128.pow(18),
        max_delta_asset: u128::MAX / 2_u128,
        max_delta_quote: u128::MAX / 2_u128,
    };
    portfolio
        .allocate(
            use_max,
            recipient,
            pool_id,
            delta_liquidity,
            max_delta_asset,
            max_delta_quote,
        )
        .send()
        .await?
        .await?;
    let reserves = portfolio.get_pool_reserves(pool_id).call().await?;
    info!("allocated reserves: {:?}", reserves);
        Ok(pool_id)
    } else {
        panic!("expected a `CreatePool` event");
    }
}
