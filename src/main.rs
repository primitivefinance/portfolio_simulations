use std::io::Read;
use std::{collections::BTreeMap, env, fs, sync::Arc};

use anyhow::Result;
use arbiter_core::{
    bindings::{
        arbiter_math::ArbiterMath, arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange,
    },
    environment::EnvironmentParameters,
    manager::Manager,
    math::{float_to_wad, ornstein_uhlenbeck::OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::{RevmMiddleware, RevmMiddlewareError},
};
use config::*;
use data_collection::*;
use ethers::{
    abi::{AbiDecode, RawLog},
    prelude::EthLogDecode,
    providers::Middleware,
    types::{Address, Log, I256, U256},
};
use log::{info, warn};
use parameters::*;
use polars::{
    prelude::{CsvWriter, DataFrame, NamedFrom, SerWriter},
    series::Series,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use startup::*;
use strategies::*;
use tokio::task::JoinHandle;

use crate::bindings::{
    normal_strategy::NormalStrategy,
    portfolio::{
        AllocateCall, CreatePoolCall, Portfolio, PortfolioErrors, PortfolioEvents,
        Portfolio_InvalidInvariant,
    },
    shared_types::Order,
    weth::WETH,
};

mod bindings;
mod config;
mod data_collection;
mod parameters;
mod startup;
mod strategies;

/// The entry point of the simulation.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger to print out all the logs.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();

    // Read from the config file.
    let configs_with_filenames = parse_config()?;

    for config_with_filename in configs_with_filenames {
        let config = config_with_filename.0;
        let mut handles: Vec<JoinHandle<Result<()>>> = vec![];

        for index in 0..config.simulation_parameters.number_of_paths {
            let filename = config_with_filename.1.clone();
            let environment_parameters = config.environment_parameters.clone();
            // let price_process_parameters = price_process_parameters.clone();
            let asset_token_parameters = config.asset_token_parameters.clone();
            let quote_token_parameters = config.quote_token_parameters.clone();
            let portfolio_pool_parameters = config.portfolio_pool_parameters.clone();
            handles.push(tokio::spawn(async move {
                // Initialize the manager with a single environment and the admin and
                // arbitrageur clients.
                let (mut manager, admin, arbitrageur) = initialize(environment_parameters.clone())?;

                // Deploy the contracts that we need for the simulation.
                let simulation_contracts = deploy_contracts(
                    admin.clone(),
                    asset_token_parameters,
                    quote_token_parameters,
                    config.price_process_parameters.initial_price,
                )
                .await?;
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
                    portfolio_pool_parameters.clone(),
                    arbx.address(),
                    arby.address(),
                    admin.default_sender().unwrap(),
                )
                .await?;

                // Create a `PriceChanger` which will update the price of the `LiquidExchange`
                // contract. This copy of the `LiquidExchange` used here contains the admin
                // client. This means the admin is taking the job as the price changer.
                let price_changer =
                    PriceChanger::new(liquid_exchange.clone(), config.price_process_parameters);

                // Create an `Arbitrageur` which will detect and execute arbitrage
                // opportunities. We create new copies of the `LiquidExchange` and
                // `Portfolio` contracts that use the arbitrageur client.
                let arbitrageur = Arbitrageur::new(
                    LiquidExchange::new(liquid_exchange.address(), arbitrageur.clone()),
                    Portfolio::new(portfolio.address(), arbitrageur.clone()),
                    arbiter_math,
                    portfolio_pool_parameters,
                    pool_id,
                )
                .await?;

                // Prepare the data collection struct and get the initial data.
                let mut simulation_output = SimulationOutput::new();
                simulation_output
                    .update_output(&simulation_contracts, pool_id, arbitrageur.address, None)
                    .await?;

                // Run the simulation.
                run(
                    price_changer,
                    arbitrageur,
                    simulation_contracts,
                    &mut simulation_output,
                )
                .await?;

                // Stop the environment once the simulation completes.
                manager.stop_environment(environment_parameters.label.clone())?;

                // Print out the data collected to a CSV.
                let final_filename = format!("{}_{}.csv", filename.clone(), index);
                simulation_output.finalize(final_filename)?;
                Ok(())
            }));
        }

        for handle in handles {
            handle.await??;
        }
    }

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
    // Run a loop over all the possible prices, start with index of 1 since we
    // already set initial prices.
    for index in 1..price_changer.trajectory.paths[0].len() {
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
