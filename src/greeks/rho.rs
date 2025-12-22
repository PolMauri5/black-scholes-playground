use crate::{bs::standardized_moneyness_forward, models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset}, stats::normal::normal_cdf};

/// ρ (Rho) -> sensitivity of the option premium to changes in interest rates (r).
/// Interpretation:
///     If Rho = 0.25, then increasing interest rates by +1% (r + 0.01)
///     increases the Call premium by approximately +0.25.
/// Calls gain value when interest rates go up.
pub fn rho_call(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let k = option.strike[i];
    let r = market.rate;
    let t = option.time_to_expiry[i];

    if t <= 0.0 {
        return 0.0;
    }

    let d2 = standardized_moneyness_forward(i, option, underlying, market);
    let discounted_strike = k * (-r * t).exp();

    // Rho_call = K * T * e^(-rT) * N(d2)
    discounted_strike * t * normal_cdf(d2)
}

/// ρ (Rho) for a PUT option.
/// Puts lose value when interest rates go up.
/// Interpretation:
///     Rho_put = change in PUT premium for +1% increase in interest rates.
pub fn rho_put(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let k = option.strike[i];
    let r = market.rate;
    let t = option.time_to_expiry[i];

    if t <= 0.0 {
        return 0.0;
    }

    let d2 = standardized_moneyness_forward(i, option, underlying, market);
    let discounted_strike = k * (-r * t).exp();

    // Rho_put = -K * T * e^(-rT) * N(-d2)
    -discounted_strike * t * normal_cdf(-d2)
}
