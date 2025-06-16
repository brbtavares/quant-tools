// This file implements the Value at Risk (VaR) metric.

use crate::common::{Compute, Fit, Predict};

/// Struct representing the Value at Risk (VaR) calculation.
pub struct VaR {
    confidence_level: f64,
    historical_returns: Vec<f64>,
}

impl VaR {
    /// Creates a new VaR instance.
    ///
    /// # Arguments
    ///
    /// * `confidence_level` - The confidence level for the VaR calculation (e.g., 0.95 for 95%).
    /// * `historical_returns` - A vector of historical returns.
    pub fn new(confidence_level: f64, historical_returns: Vec<f64>) -> Self {
        VaR {
            confidence_level,
            historical_returns,
        }
    }

    /// Calculates the Value at Risk (VaR).
    ///
    /// # Returns
    ///
    /// The calculated VaR value.
    pub fn calculate(&self) -> f64 {
        let mut sorted_returns = self.historical_returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = (1.0 - self.confidence_level) * sorted_returns.len() as f64;
        sorted_returns[index.floor() as usize]
    }
}

// Implementing the Compute trait for VaR
impl Compute for VaR {
    fn compute(&self) -> f64 {
        self.calculate()
    }
}