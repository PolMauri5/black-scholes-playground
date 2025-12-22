use crate::{models::{market_param::MarketParams, option::Option, option_type::OptionType, underlying_asset::UnderlyingAsset}, stats::normal::normal_cdf};

pub fn price_option(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    match option.option_type[i] {
        OptionType::Call => price_call(i, option, underlying, market),
        OptionType::Put  => price_put(i, option, underlying, market),
    }
}

///   S * N(d1) − K * e^(−rT) * N(d2)
fn price_call(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike[i];
    let r = market.rate;
    let t = option.time_to_expiry[i];
    let sigma = option.implied_volatility[i];

    if s <= 0.0 || t <= 0.0 || sigma <= 0.0 || k <= 0.0 {
        return 0.0;
    }

    let time = t.sqrt();
    let log_moneyness = (s / k).ln();
    let carry_term = (r + 0.5 * sigma * sigma) * t;
    let d1 = (log_moneyness + carry_term) / (sigma * time);

    let vol_shock = sigma * time;
    let d2 = d1 - vol_shock;

    // C = S * N(d1) − K * e^(−rT) * N(d2)
    // How much the strike is worth if you bring it from the future to the present.
    // Why -> Because the strike is payed in the futre (date of expiry)
    let discounted_strike = k * (-r * t).exp();

    // normal_cdf(d1) -> Porbability of moving in favour of CALL now.
    // noraml_cdf(d2) -> Probability of finishing ITM on expiry time.
    // what you can win - what will cost you to win = Value of call
    s * normal_cdf(d1) - discounted_strike * normal_cdf(d2)
}

///   K * e^(−rT) * N(−d2) − S * N(−d1)
fn price_put(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike[i];
    let r = market.rate;
    let t = option.time_to_expiry[i];
    let sigma = option.implied_volatility[i];

    if s <= 0.0 || t <= 0.0 || sigma <= 0.0 || k <= 0.0 {
        return 0.0;
    }

    let time = t.sqrt();
    let log_moneyness = (s / k).ln();
    let carry_term = (r + 0.5 * sigma * sigma) * t;

    let d1 = (log_moneyness + carry_term) / (sigma * time); 

    let vol_shock = sigma * time;
    let d2 = d1 - vol_shock;

    let discounted_strike = k * (-r * t).exp();

    discounted_strike * normal_cdf(-d2) - s * normal_cdf(-d1)
}

/// d1 — also called “standardized moneyness, z-score...”.
/// It measures how many standard deviations the log-price is above the strike
/// once drift and volatility scaling are taken into account.
/// This is the core input to all Black-Scholes Greeks.
/// Its normaly a number between -3 / +3.
/// If we are operating a call, a positive d1 is good for us.
pub fn standardized_moneyness(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams,
) -> f64 {
    let s = underlying.spot;
    let k = option.strike[i];
    let r = market.rate;
    // IV: Implied volatility.
    let sigma = option.implied_volatility[i];
    let t = option.time_to_expiry[i];

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
pub fn standardized_moneyness_forward(
    i: usize,
    option: &Option,
    underlying: &UnderlyingAsset,
    market: &MarketParams, 
) -> f64 {
    let sigma = option.implied_volatility[i];
    let t = option.time_to_expiry[i];

    if sigma <= 0.0 || t <= 0.0 {
        return 0.0;
    }

    let d1 = standardized_moneyness(i, option, underlying, market);

    // This represents one "volatility shock" over the whol horizon.
    let vol_shock = sigma * t.sqrt();

    // D2 is simply d1 shifted down by one volatility shock
    d1 - vol_shock
}
