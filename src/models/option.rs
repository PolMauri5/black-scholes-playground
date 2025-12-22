use crate::models::option_type::OptionType;

pub struct Option {
    pub strike: Vec<f64>,
    pub implied_volatility: Vec<f64>,
    pub time_to_expiry: Vec<f64>, // years
    pub option_type: Vec<OptionType>
}

impl Option {
    pub fn new(strike: Vec<f64>, implied_volatility: Vec<f64>, time_to_expiry: Vec<f64>, option_type: Vec<OptionType>) -> Self {
        Self { strike, implied_volatility, time_to_expiry, option_type }
    }
}