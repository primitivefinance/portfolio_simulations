# Bug Description


## Setup

Let the `config.toml` be set as follows:
```toml
[environment_parameters]
label = "portfolio" 
block_rate = 10.0
seed = 0

[price_process_parameters]
initial_price = 1.0
mean = 1.0
std_dev = 0.01
theta = 3.0
t_0 = 0.0
t_n = 100
num_steps = 2500
seed = 1

[asset_token_parameters]
name = "Arbiter Token X"
symbol = "ARBX"
decimals = 18

[quote_token_parameters]
name = "Arbiter Token Y"
symbol = "ARBY"
decimals = 18

[portfolio_pool_parameters]
volatility_basis_points = 10
strike_price = 1.0
time_remaining_years = 1
is_perpetual = true
fee_basis_points = 3
priority_fee_basis_points = 0

liquidity_mantissa = 1
liquidity_exponent = 20
initial_price = 1.0

[simulation_parameters]
number_of_paths = 1
```

With this, we can run the simulation with the following command:
```bash
cargo install --path .
RUST_LOG=info portfolio_simulation
```

## Results

Upon running the above, we will see that the last few lines of the log output are:
```
[2023-09-05T17:09:56Z INFO  portfolio_simulation] 
        Step 399
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Updating price of liquid_exchange to: 0.9909226840689883
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Arbitrageur sees prices:
        Liquid Exchange: 990922684068988288
        Portfolio: 992609179860061220
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Upper bound: 992907051975653916
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Lower bound: 992311397106103201
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Raw reserves: 99999999999994044500, 1506697900
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Liquidity: 100000000000000000000
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Virtual reserves: 999999999999940445, 15066979
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] S/K: 991220050084013492
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Inside term: -8818220780573245000
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Target virtual reserve: 1000000000000000000
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Virtual input: 59555
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Input ARBX: 5955500
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Output ARBY: 1505698000
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Swapping 5901441 ARBY for ARBX on Liquid Exchange
[2023-09-05T17:09:56Z INFO  portfolio_simulation::strategies] Portfolio swap successful
[2023-09-05T17:09:56Z INFO  portfolio_simulation] Portfolio price after swap is: 0
[2023-09-05T17:09:56Z INFO  portfolio_simulation] Reserves after swap are: (100000000000000000000, 999900)
[2023-09-05T17:09:56Z INFO  portfolio_simulation::data_collection] adding invariant to output
Error: execution failed to succeed due to revert!
 gas used is: 40603
 output is b"\x19F\x05\x88"
 ```

 If we look at `run()` in `main.rs` we will see that after the swap is done, we check the price and reserves, then step into the function:
 ```rust
 simulation_output
            .update_output(
                &simulation_contracts,
                arbitrageur.pool_id,
                arbitrageur.address,
                swap_event,
            )
            .await?;
```

This function is defined in `data_collection.rs` and it specifically errors out on retrieving the invariant:
```rust
self.invariant.push(
            simulation_contracts
                .portfolio
                .get_invariant(pool_id)
                .call()
                .await?
                .to_string(),
        );
```
