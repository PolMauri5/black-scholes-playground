#![allow(warnings)]

use std::time::Instant;
use rand::Rng;

use crate::{
    bs::black_scholes::price_option,
    models::{
        market_param::MarketParams,
        option::Option,
        option_type::OptionType,
        underlying_asset::UnderlyingAsset,
    },
};

mod models;
mod bs;
mod stats;
mod greeks;

fn main() {
    let underlying = UnderlyingAsset {
        spot: 100.0,
        rate: 0.05,
        dividend_yiedl: 0.0,
    };

    let market = MarketParams {
        rate: 0.05,
        dividend_yield: 0.0,
        volatility: 0.0,
    };

    let n = 100_000_000;
    let mut rng = rand::thread_rng();

    let mut strike = Vec::with_capacity(n);
    let mut iv = Vec::with_capacity(n);
    let mut tte = Vec::with_capacity(n);
    let mut types = Vec::with_capacity(n);

    for i in 0..n {
        strike.push(rng.gen_range(60.0..140.0));
        iv.push(rng.gen_range(0.15..0.40));
        tte.push(rng.gen_range(0.1..3.0));
        types.push(if i % 2 == 0 {
            OptionType::Call
        } else {
            OptionType::Put
        });
    }

    let option = Option {
        strike,
        implied_volatility: iv,
        time_to_expiry: tte,
        option_type: types,
    };

    let start = Instant::now();

    let mut prices = Vec::with_capacity(n);
    for i in 0..n {
        prices.push(price_option(i, &option, &underlying, &market));
    }

    let elapsed = start.elapsed();

    println!("Elapsed time: {:.3?}", elapsed);

    for i in 0..10 {
        println!(
            "Option {:>2}: K={:>6.2}, T={:>4.2}, σ={:>4.2} -> price={:>6.2}",
            i,
            option.strike[i],
            option.time_to_expiry[i],
            option.implied_volatility[i],
            prices[i]
        );
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;

    for &p in &prices {
        min = min.min(p);
        max = max.max(p);
        sum += p;
    }

    let avg = sum / prices.len() as f64;

    println!("\n--- PRICE STATS ---");
    println!("Min : {:.2}", min);
    println!("Max : {:.2}", max);
    println!("Avg : {:.2}", avg);
}
