use crate::{bs::{standardized_moneyness, standardized_moneyness_forward}, models::{market_param::MarketParams, option::Option, underlying_asset::UnderlyingAsset}, stats::normal::{normal_cdf, normal_pdf}};

/// Indicates how much the option premium decreases every day due to the
/// passage of time, assuming all other parameters stay constant.
/// Call Theta is usually negative: time hurts the buyer.
pub fn theta_call(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike;
    let r = market.rate;
    let sigma = market.volatility;
    let t = option.time_to_expiry;

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    let d1 = standardized_moneyness(option, underlying, market);
    let d2 = standardized_moneyness_forward(option, underlying, market);

    let discounted_strike = k * (-r * t).exp();

    // First component: pure time decay (same for call and put)
    let part_time_decay = - (s * normal_pdf(d1) * sigma) / (2.0 * t.sqrt());

    // Second component: interest rate effect (different for call/put)
    let part_interest = - r * discounted_strike * normal_cdf(d2);

    part_time_decay + part_interest
}

pub fn theta_put(
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike;
    let r = market.rate;
    let sigma = market.volatility;
    let t = option.time_to_expiry;

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    let d1 = standardized_moneyness(option, underlying, market);
    let d2 = standardized_moneyness_forward(option, underlying, market);

    let discounted_strike = k * (-r * t).exp();

    // First component: pure time decay (same for call and put)
    let part_time_decay = - (s * normal_pdf(d1) * sigma) / (2.0 * t.sqrt());

    // Second component: interest rate term changes sign for PUT
    let part_interest = r * discounted_strike * normal_cdf(-d2);

    part_time_decay - part_interest
}