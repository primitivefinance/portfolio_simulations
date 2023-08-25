# Stable Pool Simulation

The simulation in this repository is intended to demonstrate a basic simulation created with the [Arbiter](http://https://github.com/primitivefinance/arbiter) framework.
To do so, we have initialized a [Portfolio](https://github.com/primitivefinance/portfolio) pool with a single Liquidity Provider (LP) that has parameters chosen to be useful for a pair of two stable tokens.
We run a secondary market (`LiquidExchange`) with immense liquidity that changes price based on a stochastic price process.
A second client is initialized to behave as an arbitrageur between these two exchanges by moving the Portfolio pool price back into alignment with the secondary market. 

## Idea

### Smart Contract Testing

We first want to check and make sure that the `Portfolio` and `NormalStrategy` smart contracts are working as expected.
Note that by running a simulation using Arbiter, we gain the ability to find other potential bugs that may not be apparent from the smart contract code alone.
That is, bugs that may not cause a smart contract to fail, but may cause it to behave in an unexpected way.
For example, [Portfolio Issue #436](https://github.com/primitivefinance/portfolio/issues/436) was found as we built and ran this simulation.

### Portfolio Pool Performance

Suppose that we want to gauge the performance of a Portfolio pool with some chosen parameters and without any external flow, just the guaranteed arbitrage flow.
Furthermore, suppose that we want to see how this pool performs over various durations and market conditions.
By varying the parameters of the secondary market, we can simulate a variety of market conditions and by varying the Portfolio pool parameters, we can sweep over a range of configurations and optimize.

## Usage

To run the simulation, all you need to do is run the following:
```bash
git clone https://github.com/primitivefinance/portfolio_simulations.git
cd portfolio_simulations
cargo run
```
If all is working properly, then the simulation will take place.
By default, we output verbose logs to the console so that you can see what is happening.
This is useful for debugging and understanding the simulation.
For instance, you will likely see a stream of output that looks like this:
```
[INFO  arbiter_core::manager] Added environment labeled portfolio
[INFO  portfolio_simulation::startup] Admin client with address 0xcfaaf7059d10aee23d6e37455df0756603472ca9
[INFO  arbiter_core::manager] Started environment labeled portfolio
[INFO  portfolio_simulation::startup] Arbitrageur client with address 0x1f1629cbbdf810ae3ceaa6d38a41debe88aff0a7
[INFO  portfolio_simulation::startup] WETH contract deployed at 0xff9b0352145603223666c0d1a117925a913d3974
[INFO  portfolio_simulation::startup] Arbiter Token X contract deployed at 0x4a5939f004c93cdddea56677a735cecddb221a91
[INFO  portfolio_simulation::startup] Arbiter Token Y contract deployed at 0x8c9b3777b74b59cbbc8baf80b1ad5db8f284fd50
[INFO  portfolio_simulation::startup] liquid exchange contract deployed at 0x3307bc801ef829893b0a5a24c8d5a08127e07985
[INFO  portfolio_simulation::startup] Portfolio contract deployed at 0x80676843bbf0827d7fbf8ab0cb2405a0d3d1cfee
[INFO  portfolio_simulation::startup] normal strategy contract deployed at 0x56575e921da694e6b5fe3cafba407ec5c9273059
[INFO  portfolio_simulation::startup] Created a pair with pair_id: 1
[INFO  portfolio_simulation::startup] Created a pool with `pool_id`: 4294967297
[INFO  portfolio_simulation::startup] Allocated reserves: (4980053002098901240000, 4980053002098894690000)
[INFO  portfolio_simulation] 
        Step 0
[INFO  portfolio_simulation::strategies] Updating price of liquid_exchange to: 1.0071773385503369
[INFO  portfolio_simulation::strategies] Arbitrageur sees prices:
        Liquid Exchange: 1007177338550336896
        Portfolio: 999999999999999984
[INFO  portfolio_simulation::strategies] Upper bound: 1001001001001000984
[INFO  portfolio_simulation::strategies] Lower bound: 998999999999999984
[INFO  portfolio_simulation::strategies] Swapping ARBY for ARBX on Portfolio
[INFO  portfolio_simulation::strategies] Raw reserves: 4980053002098901240000, 4980053002098894690000
[INFO  portfolio_simulation::strategies] Liquidity: 10000000000000000000000
[INFO  portfolio_simulation::strategies] Virtual reserve y: 498005300209889469
[INFO  portfolio_simulation::strategies] S/K: 1007177338550336896
[INFO  portfolio_simulation::strategies] Inside term: 710170404128665100
[INFO  portfolio_simulation::strategies] Target virtual reserve: 761200778920208468
[INFO  portfolio_simulation::strategies] Virtual input: 263195478710318999
[INFO  portfolio_simulation::strategies] Input ARBY: 2634589376479669659657
[INFO  portfolio_simulation::strategies] Output ARBX: 2622952719975841770000
[INFO  portfolio_simulation::strategies] Order: Order { input: 2634589376479669659657, output: 2622952719975841770000, use_max: false, pool_id: 4294967297, sell_asset: false }
[INFO  portfolio_simulation::strategies] Portfolio swap successful
[INFO  portfolio_simulation::strategies] Swapping ARBX for ARBY on Liquid Exchange
[INFO  portfolio_simulation::strategies] LiquidExchange swap successfull; arbitrage successful
[INFO  portfolio_simulation] Portfolio price after swap is: 1007177338550336861
[INFO  portfolio_simulation] Reserves after swap are: (2357100282123059470000, 7614642378578564349657)
...
```

## Documentation

The code is documented using Rustdoc.
To view the documentation, run the following: 
```bash
cargo doc --no-deps --open
```
Also added are inline comments to help explain the code and why specific choices were made.

## Visualization

Using python, we can visualize the results of the simulation.

Set up a local python environment:
```bash
pyenv install 3.10.12
pyenv local 3.10.12
python -m venv py_env
```

Activate the environment by running:
```bash
source py_env/bin/activate
```

To deactivate this, just put
```bash
deactivate
```

Install the requirements:
```bash
pip install -r py_requirements.txt
```