// This file implements the Black-Scholes model for option pricing.

/// Calculates the Black-Scholes option price.
///
/// # Arguments
///
/// * `S` - Current stock price
/// * `K` - Option strike price
/// * `T` - Time to expiration in years
/// * `r` - Risk-free interest rate
/// * `sigma` - Volatility of the underlying stock
///
/// # Returns
///
/// The price of the European call option.
pub fn black_scholes_call(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = (S.ln() + (r + 0.5 * sigma.powi(2)) * T) / (sigma * T.sqrt());
    let d2 = d1 - sigma * T.sqrt();
    
    let call_price = S * normal_cdf(d1) - K * (-r * T).exp() * normal_cdf(d2);
    call_price
}

/// Calculates the cumulative distribution function for the standard normal distribution.
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + (x / (2.0_f64).sqrt())).erf(x / (2.0_f64).sqrt())
}