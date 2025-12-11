use crate::{bs::standardized_moneyness_forward, models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset}, stats::normal::normal_cdf};

/// ρ (Rho) -> sensitivity of the option premium to changes in interest rates (r).
/// Interpretation:
///     If Rho = 0.25, then increasing interest rates by +1% (r + 0.01)
///     increases the Call premium by approximately +0.25.
/// Calls gain value when interest rates go up.
pub fn rho_call(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let k = option.strike;
    let r = market.rate;
    let t = option.time_to_expiry;

    if t <= 0.0 {
        return 0.0;
    }

    let d2 = standardized_moneyness_forward(option, underlying, market);
    let discounted_strike = k * (-r * t).exp();

    // Rho_call = K * T * e^(-rT) * N(d2)
    discounted_strike * t * normal_cdf(d2)
}

/// ρ (Rho) for a PUT option.
/// Puts lose value when interest rates go up.
/// Interpretation:
///     Rho_put = change in PUT premium for +1% increase in interest rates.
pub fn rho_put(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let k = option.strike;
    let r = market.rate;
    let t = option.time_to_expiry;

    if t <= 0.0 {
        return 0.0;
    }

    let d2 = standardized_moneyness_forward(option, underlying, market);
    let discounted_strike = k * (-r * t).exp();

    // Rho_put = -K * T * e^(-rT) * N(-d2)
    -discounted_strike * t * normal_cdf(-d2)
}
