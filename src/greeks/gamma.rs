use crate::{bs::standardized_moneyness, models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset}, stats::normal::normal_pdf};

/// Γ -> Gamma is the velocity at whitch Delta moves when spot changes
pub fn gamma(
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
    normal_pdf(d1) / (s * sigma * t.sqrt())
}