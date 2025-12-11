use crate::models::option_type::OptionType;

pub struct Option {
    pub strike: f64,
    pub implied_volatility: f64,
    pub time_to_expiry: f64, // years
    pub option_type: OptionType
}