// This file implements the Drawdown metric for financial analysis.

pub struct Drawdown {
    peak: f64,
    trough: f64,
}

impl Drawdown {
    pub fn new() -> Self {
        Drawdown {
            peak: f64::NEG_INFINITY,
            trough: f64::INFINITY,
        }
    }

    pub fn update(&mut self, value: f64) {
        if value > self.peak {
            self.peak = value;
            self.trough = value; // Reset trough when a new peak is found
        } else if value < self.trough {
            self.trough = value;
        }
    }

    pub fn max_drawdown(&self) -> f64 {
        if self.peak == f64::NEG_INFINITY || self.trough == f64::INFINITY {
            return 0.0; // No drawdown if no values have been processed
        }
        (self.peak - self.trough) / self.peak
    }
}