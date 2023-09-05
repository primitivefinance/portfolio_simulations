#![warn(missing_docs)]

//! Contains the two strategies used in the simulation.
//! - `PriceChanger` updates the price of the `LiquidExchange` contract.
//! - `Arbitrageur` detects and executes arbitrage opportunities.

use arbiter_core::bindings::liquid_exchange::LiquidExchangeEvents;
use ethers::etherscan::contract;

use crate::bindings::normal_strategy::NormalStrategyErrors;

use super::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PriceChanger
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl PriceChanger {
    /// Create a new `PriceChanger` with the given `LiquidExchange` contract
    /// bound to the admin `Client`. The `PriceChanger` will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    /// Ornstein-Uhlenbeck processes are useful for modeling the price of stable
    /// tokens.
    pub fn new(
        liquid_exchange: LiquidExchange<RevmMiddleware>,
        price_process_params: PriceProcessParameters,
    ) -> Self {
        let PriceProcessParameters {
            initial_price,
            mean,
            std_dev,
            theta,
            t_0,
            t_n,
            num_steps,
            seed,
        } = price_process_params;
        let process = OrnsteinUhlenbeck::new(mean, std_dev, theta);

        let trajectory = match seed {
            Some(seed) => {
                process.seedable_euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false, seed)
            }
            None => process.euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false),
        };

        Self {
            trajectory,
            liquid_exchange,
            index: 1, /* start after the initial price since it is already set on contract
                       * deployment */
        }
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        info!("Updating price of liquid_exchange to: {}", price);
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Arbitrageur
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// The `Arbitrageur` holds the data and has methods that allow it to detect and
/// execute arbitrage opportunities. It uses the `LiquidExchange` and
/// `Portfolio` contracts bound to the arbitrageur `Client`.
#[derive(Debug)]
pub struct Arbitrageur {
    /// The current prices of the `LiquidExchange` and `Portfolio` contracts.
    pub prices: [U256; 2],

    /// The `LiquidExchange` contract with the arbitrageur `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The `Portfolio` contract with the arbitrageur `Client`.
    pub portfolio: Portfolio<RevmMiddleware>,

    pub atomic_arb: AtomicArb<RevmMiddleware>,

    /// The `ArbiterMath` contract with the admin `Client`.
    /// Note that this is only used to compute mathematical functions and not
    /// write to state!
    pub arbiter_math: ArbiterMath<RevmMiddleware>,

    /// The pool ID of the Portfolio pool.
    pub pool_id: u64,

    /// The address of the arbitrageur.
    pub address: Address,

    /// ARBX address.
    pub arbx_address: Address,

    /// ARBY address.
    pub arby_address: Address,

    pub gamma_wad: U256,

    pub k_iwad: I256,

    pub sigma_iwad: I256,
}

impl Arbitrageur {
    /// Creates a new `Arbitrageur` with the given `LiquidExchange` and
    /// `Portfolio` contracts bound to the arbitrageur `Client`.
    /// The `Arbitrageur` will use the `ArbiterMath` contract bound to the admin
    /// `Client` to compute mathematical functions. The `pool_id` is used to
    /// swap on the correct pool.
    pub async fn new(
        liquid_exchange: LiquidExchange<RevmMiddleware>,
        portfolio: Portfolio<RevmMiddleware>,
        arbiter_math: ArbiterMath<RevmMiddleware>,
        atomic_arb: AtomicArb<RevmMiddleware>,
        portfolio_pool_parameters: PortfolioPoolParameters,
        pool_id: u64,
    ) -> Result<Self> {
        // Get the address from the `Client` attached to the `LiquidExchange` contract.
        let address = liquid_exchange.client().default_sender().unwrap();

        // Get the two token addresses from the `LiquidExchange` contract.
        let arbx_address = liquid_exchange.arbiter_token_x().call().await?;
        let arby_address = liquid_exchange.arbiter_token_y().call().await?;

        // Get the current (initial) prices of the `LiquidExchange` and `Portfolio`
        // contracts.
        let prices = [
            liquid_exchange.price().call().await?,
            portfolio.get_spot_price(pool_id).call().await?,
        ];

        // Compute the gamma parameter of the Portfolio pool in U256 WAD.
        let gamma_wad = WAD - portfolio_pool_parameters.fee_basis_points as u128 * 10_u128.pow(14);

        // Compute the strike price parameter of the Portfolio pool in I256 WAD.
        let k_iwad = I256::from_raw(float_to_wad(portfolio_pool_parameters.strike_price));

        // Compute the volatility parameter of the Portfolio pool in I256 WAD.
        let sigma_iwad =
            I256::from(portfolio_pool_parameters.volatility_basis_points as u64 * 10_u64.pow(14));

        Ok(Self {
            prices,
            liquid_exchange,
            portfolio,
            arbiter_math,
            atomic_arb,
            pool_id,
            address,
            arbx_address,
            arby_address,
            gamma_wad,
            k_iwad,
            sigma_iwad,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    pub async fn detect_arbitrage(&mut self) -> Result<SwapDirection> {
        // Update the prices the for the arbitrageur.
        self.prices = [
            self.liquid_exchange.price().call().await?,
            self.portfolio.get_spot_price(self.pool_id).call().await?,
        ];
        let liquid_exchange_price = self.prices[0];
        let portfolio_price = self.prices[1];
        info!(
            "Arbitrageur sees prices:\n\tLiquid Exchange: {}\n\tPortfolio: {}",
            liquid_exchange_price, portfolio_price
        );

        // Compute the no-arbitrage bounds.
        let upper_arb_bound = WAD * portfolio_price / self.gamma_wad;
        info!("Upper bound: {}", upper_arb_bound);

        let lower_arb_bound = portfolio_price * self.gamma_wad / WAD;
        info!("Lower bound: {}", lower_arb_bound);

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price > upper_arb_bound && liquid_exchange_price > portfolio_price {
            // Raise the portfolio price by selling ARBY for ARBX
            Ok(SwapDirection::YToX(liquid_exchange_price))
        } else if liquid_exchange_price < lower_arb_bound && liquid_exchange_price < portfolio_price
        {
            // Lower the portfolio price by selling ARBX for ARBY
            Ok(SwapDirection::XToY(liquid_exchange_price))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(SwapDirection::None)
        }
    }

    /// Executes an arbitrage opportunity.
    /// If the swap direction is `XtoY`, then the arbitrageur will sell ARBX for
    /// ARBY on the Portfolio contract and vice-versa.
    ///
    /// Then the arbitrageur has a goal of ending up only with the quote token
    /// (ARBY) since this is viewed as a risk-free cash asset.
    /// This means that:
    /// - If the swap direction is `XtoY`, then the arbitrageur will sell ARBY
    ///   on the LiquidExchange for ARBX, then swap the ARBX for ARBY on
    ///   Portfolio such that the resulting Portfolio pirce matches the
    ///   LiquidExchange.
    /// The exact amount of ARBX to sell is computed by the
    /// `compute_order_input_x` function.
    /// - If the swap direction is `YtoX`, then the arbitrageur will sell ARBY
    ///   on Portfolio for ARBX, then swap the ARBX for ARBY on the
    ///   LiquidExchange after.
    pub async fn execute_arbitrage(
        &mut self,
        swap_direction: SwapDirection,
    ) -> Result<Option<Log>> {
        Ok(match swap_direction {
            SwapDirection::None => {
                info!("No swap occuring");
                return Ok(None);
            }
            SwapDirection::XToY(target_price) => {
                // Get how much ARBX we need to sell to get the target price on Portfolio.
                let order = self.compute_order_input_x(target_price).await?;
                // Swap on LE first
                // Get the amount to send to LE with quick math
                let le_input = U256::from(order.input) * self.prices[0] / WAD + 1;
                info!("Swapping {} ARBY for ARBX on Liquid Exchange", le_input);

                match self
                    .atomic_arb
                    .execute_x_to_y(
                        self.arbx_address,
                        self.arby_address,
                        self.portfolio.address(),
                        self.liquid_exchange.address(),
                        order,
                        le_input,
                    )
                    .send()
                    .await
                {
                    Ok(pending_tx) => {
                        info!("Portfolio swap successful");
                        let receipt = pending_tx.await?.unwrap();

                        Some(receipt.logs[8].clone())
                    }
                    Err(contract_error) => {
                        warn!("Portfolio swap failed: {:?}", contract_error);
                        return Ok(None);
                    }
                }
            }
            SwapDirection::YToX(target_price) => {
                let order = self.compute_order_input_y(target_price).await?;
                info!("Swapping {} ARBY for ARBX on Portfolio", order.input);
                match self
                    .atomic_arb
                    .execute_y_to_x(
                        self.arbx_address,
                        self.arby_address,
                        self.portfolio.address(),
                        self.liquid_exchange.address(),
                        order,
                    )
                    .send()
                    .await
                {
                    Ok(pending_tx) => {
                        info!("Portfolio swap successful");
                        let receipt = pending_tx.await?.unwrap();
                        Some(receipt.logs[5].clone())
                    }
                    Err(contract_error) => {
                        warn!("Portfolio swap failed: {:?}", contract_error);
                        return Ok(None);
                    }
                }
            }
        })
    }

    async fn compute_order_input_x(&self, target_price_wad: U256) -> Result<Order> {
        // Get some necessary constants as I256
        let iwad = I256::from_raw(WAD);
        let target_price_iwad =
            I256::from_raw(target_price_wad) * iwad / I256::from_raw(self.gamma_wad);

        // Sell the asset (X) for the quote token (Y)
        let sell_asset = true;

        // Get the reserves and liquidity
        let (reserve_x, reserve_y, liquidity, ..) =
            self.portfolio.pools(self.pool_id).call().await?;
        info!("Raw reserves: {}, {}", reserve_x, reserve_y);
        info!("Liquidity: {}", liquidity);

        // Compute the virtual reserves (divide by the liquidity rescaling factor)
        let rescaling = I256::from(liquidity) / iwad;
        let virtual_reserve_x = I256::from(reserve_x) / rescaling;
        let virtual_reserve_y = I256::from(reserve_y) / rescaling;
        info!(
            "Virtual reserves: {}, {}",
            virtual_reserve_x, virtual_reserve_y
        );

        // Note that in our units here, sqrt(tau) = 1.
        // R1 = 1 - CDF( ln( S / K ) / sigma + 0.5 * sigma)
        // S here is our target price
        let s_div_k_iwad = target_price_iwad * iwad / self.k_iwad;
        info!("S/K: {}", s_div_k_iwad);

        let inside_term_iwad = (self.arbiter_math.log(s_div_k_iwad).call().await? * iwad)
            / self.sigma_iwad
            + self.sigma_iwad / 2;
        info!("Inside term: {}", inside_term_iwad);

        let target_virtual_reserve_x =
            iwad - self.arbiter_math.cdf(inside_term_iwad).call().await?;
        info!("Target virtual reserve: {}", target_virtual_reserve_x);

        let virtual_input_x = target_virtual_reserve_x - virtual_reserve_x;
        info!("Virtual input: {}", virtual_input_x);

        // Rescale back to the real input amount and multiply by 1/gamma to account for
        // the swap fee.
        let final_scaling_wad = rescaling * iwad;
        let input = virtual_input_x * final_scaling_wad / iwad;
        info!("Input ARBX: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the
        // output amount
        let output = self
            .portfolio
            .get_amount_out(
                self.pool_id,
                sell_asset,
                U256::try_from(input)?,
                self.address,
            )
            .call()
            .await;
        let output = match output {
            Ok(output) => output,
            Err(contract_error) => {
                if let RevmMiddlewareError::ExecutionRevert {
                    gas_used: _,
                    output,
                } = contract_error.as_middleware_error().unwrap()
                {
                    if let NormalStrategyErrors::NormalStrategyLib_LowerReserveYBoundNotReached(
                        value,
                    ) = NormalStrategyErrors::decode(output)?
                    {
                        warn!(
                            "getAmountOut for x output failed due to revert: {:?}",
                            value
                        );
                    }
                }
                U256::from(reserve_y - 1)
            }
        };

        info!("Output ARBY: {}", output);

        Ok(Order {
            input: input.as_u128(),
            output: output.as_u128(),
            use_max: false,
            pool_id: self.pool_id,
            sell_asset,
        })
    }

    async fn compute_order_input_y(&self, target_price_wad: U256) -> Result<Order> {
        // Get some necessary constants as I256
        let iwad = I256::from_raw(WAD);
        let target_price_iwad =
            I256::from_raw(target_price_wad) * I256::from_raw(self.gamma_wad) / iwad;

        // Sell the quote token (Y) for the asset token (X)
        let sell_asset = false;

        // Get the reserves and liquidity
        let (reserve_x, reserve_y, liquidity, ..) =
            self.portfolio.pools(self.pool_id).call().await?;
        info!("Raw reserves: {}, {}", reserve_x, reserve_y);
        info!("Liquidity: {}", liquidity);

        // Get the pool's invariant
        let invariant = self.portfolio.get_invariant(self.pool_id).call().await?;
        info!("Invariant: {}", invariant);

        // Compute the virtual reserves (divide by the liquidity rescaling factor)
        let rescaling = I256::from(liquidity) / iwad;
        let virtual_reserve_x = I256::from(reserve_x) / rescaling;
        let virtual_reserve_y = I256::from(reserve_y) / rescaling;
        info!(
            "Virtual reserves: {}, {}",
            virtual_reserve_x, virtual_reserve_y
        );

        // Note that in our units here, sqrt(tau) = 1.
        // R2 = K CDF( ln( S / K ) / sigma - 0.5 * sigma) + k
        // S here is our target price and k is the invariant (not to be confused with K
        // the strike!)
        let s_div_k_iwad = target_price_iwad * iwad / self.k_iwad;
        info!("S/K: {}", s_div_k_iwad);

        let inside_term_iwad = (self.arbiter_math.log(s_div_k_iwad).call().await? * iwad)
            / self.sigma_iwad
            - self.sigma_iwad / 2;
        info!("Inside term: {}", inside_term_iwad);

        let target_virtual_reserve_y =
            self.k_iwad * self.arbiter_math.cdf(inside_term_iwad).call().await? / iwad;
        info!("Target virtual reserve: {}", target_virtual_reserve_y);

        let virtual_input_y = target_virtual_reserve_y - virtual_reserve_y + invariant;
        info!("Virtual input: {}", virtual_input_y);

        // Rescale back to the real input amount and multiply by 1/gamma to account for
        // the swap fee.
        let final_scaling_wad = rescaling * iwad;
        let input = virtual_input_y * final_scaling_wad / iwad;
        info!("Input ARBY: {}", input);

        // Call the `getAmountOut()` function on the Portfolio contract to get the
        // output amount
        let output = self
            .portfolio
            .get_amount_out(
                self.pool_id,
                sell_asset,
                U256::try_from(input)?,
                self.address,
            )
            .call()
            .await;

        let output = match output {
            Ok(output) => output,
            Err(contract_error) => {
                if let RevmMiddlewareError::ExecutionRevert {
                    gas_used: _,
                    output,
                } = contract_error.as_middleware_error().unwrap()
                {
                    if let NormalStrategyErrors::NormalStrategyLib_LowerReserveXBoundNotReached(
                        value,
                    ) = NormalStrategyErrors::decode(output)?
                    {
                        warn!(
                            "getAmountOut for x output failed due to revert: {:?}",
                            value
                        );
                    }
                }
                U256::from(reserve_x - 1)
            }
        };

        info!("Output ARBX: {}", output);

        Ok(Order {
            input: input.as_u128(),
            output: output.as_u128(),
            use_max: false,
            pool_id: self.pool_id,
            sell_asset,
        })
    }
}

/// Used to label the direction (if any) of a the Portfolio swap in a two-swap
/// arbitrage between the Portfolio and LiquidExchange contracts.
#[derive(Debug)]
pub enum SwapDirection {
    /// Swap ARBX for ARBY on the Portfolio contract with a target price of some
    /// `U256`.
    XToY(U256),

    /// Swap ARBY for ARBX on the Portfolio contract with a target price of some
    /// `U256`.
    YToX(U256),

    /// No swap is occuring.
    None,
}
