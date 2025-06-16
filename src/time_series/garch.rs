// This file implements the GARCH model for volatility forecasting.

pub struct GARCH {
    pub alpha: f64,
    pub beta: f64,
    pub omega: f64,
}

impl GARCH {
    pub fn new(alpha: f64, beta: f64, omega: f64) -> Self {
        GARCH { alpha, beta, omega }
    }

    pub fn forecast(&self, past_volatility: f64) -> f64 {
        self.omega + self.alpha * past_volatility + self.beta * past_volatility.powi(2)
    }
}