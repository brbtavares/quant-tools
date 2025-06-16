// This file implements the Conditional Value at Risk (CVaR) metric.

pub struct CVaR {
    confidence_level: f64,
}

impl CVaR {
    pub fn new(confidence_level: f64) -> Self {
        CVaR { confidence_level }
    }

    pub fn calculate(&self, returns: &[f64]) -> f64 {
        let sorted_returns: Vec<f64> = {
            let mut temp = returns.to_vec();
            temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
            temp
        };

        let index = (self.confidence_level * sorted_returns.len() as f64).floor() as usize;
        let cvar = sorted_returns.iter().take(index).copied().sum::<f64>() / index as f64;

        cvar
    }
}