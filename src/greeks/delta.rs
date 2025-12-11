use crate::models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset};
use crate::bs::standardized_moneyness;
use crate::stats::normal::normal_cdf;

/// Δ -> Delta is how much is premium going to move when
/// the undelying asset (spot) moves 1 point 
pub fn delta_call(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let d1 = standardized_moneyness(option, underlying, market);
    normal_cdf(d1)
}