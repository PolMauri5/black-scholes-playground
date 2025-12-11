use crate::{bs::{standardized_moneyness}, models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset}, stats::normal::{normal_pdf}};

/// Vega -> sensitivity of the option premium to changes in volatility (σ).
/// Interpretation:
///     If Vega = 0.30, then increasing volatility by +1% (σ + 0.01)
///     increases the option premium by approximately +0.30.
/// Vega is the same for Calls and Puts.
pub fn vega(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let sigma = market.volatility;
    let t = option.time_to_expiry;

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    let d1 = standardized_moneyness(option, underlying, market);

    // Vega = S * n(d1) * sqrt(T)
    s * normal_pdf(d1) * t.sqrt()
}
