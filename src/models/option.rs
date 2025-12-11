use crate::models::option_type::OptionType;

pub struct Option {
    pub strike: f64,
    pub implied_volatility: f64,
    pub time_to_expiry: f64, // years
    pub option_type: OptionType
}

impl Option {
    pub fn new(strike: f64, implied_volatility: f64, time_to_expiry: f64, option_type: OptionType) -> Self {
        Self { strike, implied_volatility, time_to_expiry, option_type }
    }
}