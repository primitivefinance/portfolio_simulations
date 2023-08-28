#![warn(missing_docs)]

//! This module contains all the functions that are used to initialize the
//! simulation. This includes:
//! - Initializing the manager with an environment, an admin client, and the
//!   arbitrageur client.
//! - Deploying the contracts that we need for the simulation.
//! - Allocating out funds to all the relevant contracts and approve them all to
//!   spend.
//! - Initializing a `Portfolio` pair and pool.

use super::*;

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

/// Initialize the manager with an environment, an admin client, and the
/// arbitrageur client.
pub fn initialize(
    environment_parameters: EnvironmentParameters,
) -> Result<(Manager, Client, Client)> {
    // Create a manager and single environment using our predefined constants.
    let mut manager = Manager::new();
    manager.add_environment(environment_parameters.clone())?;

    // Create the admin client with our predefined constants.
    let admin = Arc::new(RevmMiddleware::new(
        manager
            .environments
            .get(&environment_parameters.label.clone())
            .unwrap(),
        Some(ADMIN_LABEL.to_string()),
    ));
    info!(
        "Admin client with address {:?}",
        admin.default_sender().unwrap()
    );
    manager.start_environment(environment_parameters.label.clone())?;

    // Create the arbitrageur client using our predefined constants.
    let arbitrageur = Arc::new(RevmMiddleware::new(
        manager
            .environments
            .get(&environment_parameters.label)
            .unwrap(),
        Some(ARBITRAGEUR_LABEL.to_string()),
    ));
    info!(
        "Arbitrageur client with address {:?}",
        arbitrageur.default_sender().unwrap()
    );

    Ok((manager, admin, arbitrageur))
}

/// Deploy the contracts that we need for the simulations.
pub async fn deploy_contracts(
    client: Client,
    asset_token_parameters: TokenParameters,
    quote_token_parameters: TokenParameters,
    initial_liquid_exchange_price: f64,
) -> Result<SimulationContracts> {
    // Deploy the WETH contract; needed for Portfolio, but not needed actively in
    // the simulation. Hence, we don't return it from this function!
    let weth = WETH::deploy(client.clone(), ())?.send().await?;
    info!("WETH contract deployed at {:?}", weth.address());

    // Deploy an instance of the `ArbiterToken` contract using our predefined
    // constants.
    let arbx = ArbiterToken::deploy(
        client.clone(),
        (
            asset_token_parameters.name,
            asset_token_parameters.symbol,
            asset_token_parameters.decimals,
        ),
    )?
    .send()
    .await?;
    info!("Arbiter Token X contract deployed at {:?}", arbx.address());

    // Deploy the second instance of the `ArbiterToken` contract using our
    // predefined constants.
    let arby = ArbiterToken::deploy(
        client.clone(),
        (
            quote_token_parameters.name,
            quote_token_parameters.symbol,
            quote_token_parameters.decimals,
        ),
    )?
    .send()
    .await?;
    info!("Arbiter Token Y contract deployed at {:?}", arby.address());

    // Deploy the `LiquidExchange` contract with the two `ArbiterToken` contracts as
    // the underlying tokens.
    // Set the initial price based on the predefined constants that define our price
    // process.
    let liquid_exchange = LiquidExchange::deploy(
        client.clone(),
        (
            arbx.address(),
            arby.address(),
            float_to_wad(initial_liquid_exchange_price),
        ),
    )?
    .send()
    .await?;
    info!(
        "liquid exchange contract deployed at {:?}",
        liquid_exchange.address()
    );

    // Deploy the `Portfolio` contract
    let portfolio = Portfolio::deploy(
        client.clone(),
        (weth.address(), Address::default(), Address::default()),
    )?
    .send()
    .await?;
    info!("Portfolio contract deployed at {:?}", portfolio.address());

    // Deploy the `NormalStrategy` contract which is used by the `Portfolio`
    // contract to determine a specific trading rule for the pool.
    let normal_strategy = NormalStrategy::deploy(client.clone(), portfolio.address())?
        .send()
        .await?;
    info!(
        "normal strategy contract deployed at {:?}",
        normal_strategy.address()
    );

    // Deploy the `ArbiterMath` contract which is does not have any memory and is
    // only used to perform onchain-type mathematical operations (e.g., from
    // `solmate` and `solsat`).
    let arbiter_math = ArbiterMath::deploy(client, ())?.send().await?;

    Ok(SimulationContracts {
        arbx,
        arby,
        liquid_exchange,
        portfolio,
        normal_strategy,
        arbiter_math,
    })
}

/// Allocate out funds to all the relevant contracts and approve them all to
/// spend.
pub async fn allocate_and_approve(
    admin: Client,
    arbitrageur: Client,
    arbx_address: Address,
    arby_address: Address,
    liquid_exchange_address: Address,
    portfolio_address: Address,
) -> Result<()> {
    // Create two instances of the `ArbiterToken` contract that use the admin
    // `Client`. This way we can do the proper approvals later on.
    let admin_tokens = [
        ArbiterToken::new(arbx_address, admin.clone()),
        ArbiterToken::new(arby_address, admin.clone()),
    ];

    // Create two instances of the `ArbiterToken` contract that use the arbitrageur
    // `Client`. This way we can do the proper approvals later on.
    let arbitrageur_tokens = [
        ArbiterToken::new(arbx_address, arbitrageur.clone()),
        ArbiterToken::new(arby_address, arbitrageur.clone()),
    ];

    // Get the admin and arbitrageur addresses.
    let admin_address = admin.default_sender().unwrap();
    let arbitrageur_address = arbitrageur.default_sender().unwrap();

    // Loop over the tokens that have the admin `Client` and mint a large amount of
    // tokens to the admin and the `LiquidExchange`. Note that the
    // admin `Client` is the only one that can mint! Finally, we approve the
    // `Portfolio` contract to spend all the tokens from the admin since the admin
    // will be the sole LP in the `Portfolio` pool.
    for admin_token in admin_tokens.clone() {
        admin_token
            .mint(admin_address, U256::from(u128::MAX))
            .send()
            .await?
            .await?;
        admin_token
            .mint(liquid_exchange_address, U256::from(u128::MAX))
            .send()
            .await?
            .await?;
        admin_token
            .approve(portfolio_address, U256::from(u128::MAX))
            .send()
            .await?
            .await?;
    }

    // Give the arbitrageur just some token Y (quote).
    admin_tokens[1]
        .mint(arbitrageur_address, U256::from(u128::MAX))
        .send()
        .await?
        .await?;

    // Loop over the tokens that have the arbitrageur `Client` and approve the
    // `LiquidExchange` and `Portfolio` contracts to spend all the tokens from the
    // arbitrageur.
    for arbitrageur_token in arbitrageur_tokens {
        arbitrageur_token
            .approve(liquid_exchange_address, U256::from(u128::MAX))
            .send()
            .await?
            .await?;
        arbitrageur_token
            .approve(portfolio_address, U256::from(u128::MAX))
            .send()
            .await?
            .await?;
    }

    Ok(())
}

/// Initialize a `Portfolio` pair and pool.
pub async fn initialize_portfolio(
    portfolio: &Portfolio<RevmMiddleware>,
    normal_strategy: &NormalStrategy<RevmMiddleware>,
    portfolio_pool_parameters: PortfolioPoolParameters,
    asset: Address,
    quote: Address,
    lp_address: Address,
) -> Result<u64> {
    // Create a pair with the two tokens.
    // The asset token will be ARBX and the quote token will be ARBY.
    portfolio.create_pair(asset, quote).send().await?.await?;
    let pair_id = portfolio.get_pair_id(asset, quote).call().await?;
    info!("Created a pair with pair_id: {:?}", pair_id);

    // Given our choice of pool parameters, we need to get the strategy arguments
    // and the initial reserves (which depend on the initial price chosen). This
    // will be passed to the `CreatePool` call.
    let strike_price_wad = arbiter_core::math::float_to_wad(portfolio_pool_parameters.strike_price);
    let volatility_basis_points =
        ethers::types::U256::from(portfolio_pool_parameters.volatility_basis_points);
    let duration_seconds = ethers::types::U256::from(
        portfolio_pool_parameters.time_remaining_years * SECONDS_PER_YEAR,
    );
    let price_wad = arbiter_core::math::float_to_wad(portfolio_pool_parameters.initial_price);
    let (strategy_args, reserve_x_per_wad, reserve_y_per_wad) = normal_strategy
        .get_strategy_data(
            strike_price_wad,
            volatility_basis_points,
            duration_seconds,
            portfolio_pool_parameters.is_perpetual,
            price_wad,
        )
        .call()
        .await?;

    // Make an instance of the `CreatePoolCall` struct since the paramater list is
    // large. Sadly this cannot be directly passed into the call like this, but
    // it is a convenient way to see that we correctly set all the parameters.
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
        fee_basis_points: portfolio_pool_parameters.fee_basis_points,
        priority_fee_basis_points: portfolio_pool_parameters.priority_fee_basis_points,
        controller: Address::default(),
        strategy: Address::default(), // address(0) == default strategy
        strategy_args,
    };

    // Create the pool and get the `pool_id`.
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

    // The `pool_id` is contained in the `CreatePool` event which comes out as the
    // second log (indexed from zero).
    let create_pool_log = create_pool_output.logs[1].clone();
    let portfolio_event = PortfolioEvents::decode_log(&create_pool_log.into()).unwrap();

    // Check that we got the correct event, and if so, go ahead and have the admin
    // allocate liquidity to the pool that they created.
    if let PortfolioEvents::CreatePoolFilter(create_pool_filter) = portfolio_event {
        let pool_id = create_pool_filter.pool_id;
        info!("Created a pool with `pool_id`: {:?}", pool_id);

        // Allocate liquidity to the pool.
        // Again, using an instance of the `AllocateCall` struct to make sure we set all
        // the parameters correctly.
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
            delta_liquidity: (portfolio_pool_parameters.liquidity_mantissa as u128 * 10_u128).pow(portfolio_pool_parameters.liquidity_exponent),
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

        // Check that the reserves are allocated correctly and read them out.
        let reserves = portfolio.get_pool_reserves(pool_id).call().await?;
        info!("Allocated reserves: {:?}", reserves);

        Ok(pool_id)
    } else {
        // If we didn't get the correct event, then we have a problem.
        // This panic should never be hit!
        panic!("Expected a `CreatePool` event!");
    }
}
