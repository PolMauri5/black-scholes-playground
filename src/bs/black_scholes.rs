use crate::models::{market_param::MarketParams, option::Option, option_type::OptionType, underlying_asset::UnderlyingAsset};

// Black-Scholes engine (skeleton only).
// This module defines the public entry point for option pricing.
// At this stage, everything is a placeholder: no formulas, no probability distributions.
// Future sprints will introduce d1/d2 logic, normal CDF/PDF, and the full closed-form model.

pub fn price_option(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    match option.option_type {
        OptionType::Call => price_call_sub(),
        OptionType::Put  => price_put_sub(),
    }
}

/// Placeholder for the call price.
/// Will later implement:
///   S * N(d1) − K * e^(−rT) * N(d2)
fn price_call_sub() -> f64 {
    0.0
}

/// Placeholder for the put price.
/// Will later implement:
///   K * e^(−rT) * N(−d2) − S * N(−d1)
fn price_put_sub() -> f64 {
    0.0
}

/// d1 — also called “standardized moneyness, z-index...”.
/// It measures how many standard deviations the log-price is above the strike
/// once drift and volatility scaling are taken into account.
/// This is the core input to all Black-Scholes Greeks.
/// Its normaly a number between -3 / +3.
/// If we are operating a call, a positive d1 is good for us.
fn standardized_moneyness(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike;
    let r = market.rate;
    // IV: Implied volatility.
    let sigma = market.volatility;
    let t = option.time_to_expiry;

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    // Logarithmic distance between spot ans strike.
    let log_moneyness = (s / k).ln();

    // How the expected log-price evolves over time
    let carry_term = (r + 0.5 * sigma * sigma) * t;

    // Standard deviation of the asset's log return over the time.
    let denom = sigma * t.sqrt();

    (log_moneyness + carry_term) / denom
}

/// d2 — also called “forward-adjusted standardized moneyness”.
/// It represents the risk-neutral probability-weighted distance to the strike.
/// Financially, d2 determines the probability of expiring ITM under the model.
fn standardized_moneyness_forward(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams, 
) -> f64 {
    let sigma = market.volatility;
    let t = option.time_to_expiry;

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    let d1 = standardized_moneyness(option, underlying, market);

    // This represents one "volatility shock" over the whol horizon.
    let vol_shock = sigma * t.sqrt();

    // D2 is simply d1 shifted down by one volatility shock
    d1 - vol_shock
}
