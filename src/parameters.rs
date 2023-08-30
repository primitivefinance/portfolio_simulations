use super::*;

/// This struct contains constants are used to configure the Ornstein-Uhlenbeck
/// process
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PriceProcessParameters {
    /// The initial price of the asset.
    pub initial_price: f64,

    /// The mean (price) of the process.
    pub mean: f64,

    /// The standard deviation of the process.
    pub std_dev: f64,

    /// The theta parameter of the process.
    /// This describes how strongly the process will revert to the mean.
    pub theta: f64,

    /// The start time of the process.
    pub t_0: f64,

    /// The end time of the process.
    pub t_n: f64,

    /// The number of steps in the process.
    pub num_steps: usize,

    pub seed: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenParameters {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

/// All the possible settings for the Portfolio pool.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortfolioPoolParameters {
    /// The implied volatility parameter in the `NormalStrategy`.
    /// Sets the "width" of a Gaussian liquidity distribution.
    pub volatility_basis_points: u16,

    /// The strike price of the `NormalStrategy`.
    /// Sets the "center" of a Gaussian liquidity distribution.
    pub strike_price: f64,

    /// The time remaining in the `NormalStrategy`.
    /// For the purposes of this simulation, this is set to 1 year.
    pub time_remaining_years: u64,

    /// Whether the `NormalStrategy` is perpetual or will change over time.
    pub is_perpetual: bool,

    pub fee_basis_points: u16,

    /// The priority swap fee in basis points (not needed for this simulation).
    pub priority_fee_basis_points: u16,

    pub liquidity_mantissa: u64,
    pub liquidity_exponent: u32,

    /// The initial price of the Portfolio pool.
    pub initial_price: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub number_of_paths: u16,
    pub sweep_parameters: Option<SweepParameters>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SweepParameters {
    pub sweeps: BTreeMap<String, LinspaceParameters>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LinspaceParameters {
    pub start: f64,
    pub end: f64,
    pub steps: usize,
}
