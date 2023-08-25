use config::*;
use data_collection::*;
use startup::*;
use strategies::*;

pub mod bindings;
pub mod config;
pub mod data_collection;
pub mod startup;
pub mod strategies;

/// The entry point of the simulation.
#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize the logger to print out all the logs.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();

    // Initialize the manager with a single environment and the admin and
    // arbitrageur clients.
    let (mut manager, admin, arbitrageur) = initialize()?;

    // Deploy the contracts that we need for the simulation.
    let simulation_contracts = deploy_contracts(admin.clone()).await?;
    let SimulationContracts {
        arbx,
        arby,
        liquid_exchange,
        portfolio,
        normal_strategy,
        arbiter_math,
    } = simulation_contracts.clone();

    // Allocate tokens and approve their spending.
    allocate_and_approve(
        admin.clone(),
        arbitrageur.clone(),
        arbx.address(),
        arby.address(),
        liquid_exchange.address(),
        portfolio.address(),
    )
    .await?;

    // Initialize a Portfolio pool.
    let pool_id = initialize_portfolio(
        &portfolio,
        &normal_strategy,
        arbx.address(),
        arby.address(),
        admin.default_sender().unwrap(),
    )
    .await?;

    // Create a `PriceChanger` which will update the price of the `LiquidExchange`
    // contract. This copy of the `LiquidExchange` used here contains the admin
    // client. This means the admin is taking the job as the price changer.
    let price_changer = PriceChanger::new(liquid_exchange.clone());

    // Create an `Arbitrageur` which will detect and execute arbitrage
    // opportunities. We create new copies of the `LiquidExchange` and
    // `Portfolio` contracts that use the arbitrageur client.
    let arbitrageur = Arbitrageur::new(
        LiquidExchange::new(liquid_exchange.address(), arbitrageur.clone()),
        Portfolio::new(portfolio.address(), arbitrageur.clone()),
        arbiter_math,
        pool_id,
    )
    .await?;

    // Prepare the data collection struct and get the initial data.
    let mut simulation_output = SimulationOutput::new();
    simulation_output.update_output(&simulation_contracts, pool_id, arbitrageur.address).await?;

    // Run the simulation.
    run(
        price_changer,
        arbitrageur,
        simulation_contracts,
        &mut simulation_output,
    )
    .await?;

    // Stop the environment once the simulation completes.
    manager.stop_environment(ENV_LABEL)?;

    // Print out the data collected to a CSV.
    simulation_output.finalize(ENV_LABEL)?;

    Ok(())
}

/// This function houses a basic simulation loop.
/// It will update the price, detect arbitrage opportunities, and execute them.
/// It will do this for a set number of steps chosen in `config.rs`.
/// It will print out the current state of the Portfolio pool after each step as
/// logs. It will return an error if any of the steps fail.
/// There is no need to check events or have the `Arbitrageur` or `PriceChanger`
/// on separate threads.
async fn run(
    mut price_changer: PriceChanger,
    mut arbitrageur: Arbitrageur,
    simulation_contracts: SimulationContracts,
    simulation_output: &mut SimulationOutput,
) -> Result<()> {
    // Run a loop over all the possible prices, start with index of 1 since we already set initial prices.
    for index in 1..NUM_STEPS {
        info!("\n\tStep {}", index);

        // Update the price
        price_changer.update_price().await?;

        // Detect if there is an arbitrage opportunity.
        let swap_direction = arbitrageur.detect_arbitrage().await?;

        // Execute the arbitrage
        let swap_event = arbitrageur.execute_arbitrage(swap_direction).await?;

        // Print out the current state of the Portfolio pool.
        info!(
            "Portfolio price after swap is: {:?}",
            arbitrageur
                .portfolio
                .get_spot_price(arbitrageur.pool_id)
                .call()
                .await?
        );
        info!(
            "Reserves after swap are: {:?}",
            arbitrageur
                .portfolio
                .get_pool_reserves(arbitrageur.pool_id)
                .call()
                .await?
        );

        simulation_output
            .update_output(
                &simulation_contracts,
                arbitrageur.pool_id,
                arbitrageur.address,
                swap_event,
            )
            .await?;
    }
    Ok(())
}
