pub struct MarketParams {
    pub rate: f64,
    pub dividend_yield: f64,
    pub volatility: f64,
}

impl MarketParams {
    pub fn new(rate: f64, dividend_yield: f64, volatility: f64) -> Self {
        Self { rate, dividend_yield, volatility }
    }
}