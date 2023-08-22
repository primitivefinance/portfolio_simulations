use config::*;
use startup::*;
use strategies::*;

pub mod bindings;
pub mod config;
pub mod startup;
pub mod strategies;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize the logger
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();

    let (mut manager, admin, arbitrageur) = initialize()?;
    let SimulationContracts {
        arbx,
        arby,
        liquid_exchange,
        portfolio,
        normal_strategy,
        arbiter_math,
    } = deploy_contracts(admin.clone()).await?;

    allocate_and_approve(
        admin.clone(),
        arbitrageur.clone(),
        arbx.address(),
        arby.address(),
        liquid_exchange.address(),
        portfolio.address(),
    )
    .await?;

    let pool_id = initialize_portfolio(
        &portfolio,
        &normal_strategy,
        arbx.address(),
        arby.address(),
        admin.default_sender().unwrap(),
    )
    .await?;

    // This copy of the liquid exchange used here is the one with the admin client.
    let price_changer = PriceChanger::new(liquid_exchange.clone());

    let arbitrageur = Arbitrageur::new(
        LiquidExchange::new(liquid_exchange.address(), arbitrageur.clone()),
        Portfolio::new(portfolio.address(), arbitrageur.clone()),
        arbiter_math,
        pool_id,
        arbitrageur.default_sender().unwrap(),
    )
    .await?;

    run(price_changer, arbitrageur).await?;

    // Stop the environment
    info!("Stopping the environment");
    manager.stop_environment(ENV_LABEL)?;

    Ok(())
}

async fn run(mut price_changer: PriceChanger, mut arbitrageur: Arbitrageur) -> Result<()> {
    // Run a loop to change the prices
    for index in 0..NUM_STEPS {
        info!("\n\tStep {}", index);
        price_changer.update_price().await?;

        let swap_direction = arbitrageur.detect_arbitrage().await?;

        arbitrageur.execute_arbitrage(swap_direction).await?;
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
    }
    Ok(())
}
