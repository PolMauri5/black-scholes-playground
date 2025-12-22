use std::f64::consts::PI;

/// Probability Density Function
/// Returns the height of the Normal Distribution of "x" point 
pub fn normal_pdf(x: f64) -> f64 {
    (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp()
}

/// Accumulated Normal Distribution (standard normal CDF)
/// Intuition:
///     - Input: x = how many satandard deviations (z-score)
///     - Output: probability of being <= x under a normal distribution.
/// In Black Scholes:
///   - normal_cdf(d1) -> "probability the move goes in favour of the call"
///   - normal_cdf(d2) -> "probability of finishing ITM at expiry"
pub fn normal_cdf(x: f64) -> f64 {
    // we use a classical approximation (Abramowitz & Stegun).
    // Think of these as "magix tuning constants" thta make
    // the curve very close to the true normal CDF.
    let b1 =  0.319381530;
    let b2 = -0.356563782;
    let b3 =  1.781257088;
    let b4 = -1.821255978;
    let b5 =  1.330274429;
    let p  =  0.2316419;
    let c2 =  0.3989423; // ≈ 1 / sqrt(2π), scaling factor for the normal pdf

    let z = x.abs();
    let t = 1.0 / (1.0 + p * z);

    let poly = b1
        + t * (b2
        + t * (b3
        + t * (b4
        + t * b5)));

    let pdf = c2 * (-z * z / 2.0).exp();
    let cdf_pos = 1.0 - pdf * t * poly;

    if x >= 0.0 {
        cdf_pos
    } else {
        1.0 - cdf_pos
    }
}
