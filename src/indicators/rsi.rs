// This file implements the Relative Strength Index (RSI) indicator.

pub struct RSI {
    values: Vec<f64>,
    period: usize,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        Self {
            values: Vec::new(),
            period,            
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<f64> {
        self.values.push(price);
        if self.values.len() < self.period {
            return None;
        }

        let gains: f64 = self.values.windows(2)
            .map(|window| if window[1] > window[0] { window[1] - window[0] } else { 0.0 })
            .sum();

        let losses: f64 = self.values.windows(2)
            .map(|window| if window[1] < window[0] { window[0] - window[1] } else { 0.0 })
            .sum();

        let average_gain = gains / self.period as f64;
        let average_loss = losses / self.period as f64;

        let rs = if average_loss == 0.0 {
            100.0
        } else {
            average_gain / average_loss
        };

        Some(100.0 - (100.0 / (1.0 + rs)))
    }
}