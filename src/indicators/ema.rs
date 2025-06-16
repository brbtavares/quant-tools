// This file implements the Exponential Moving Average (EMA) indicator.

pub struct EMA {
    period: usize,
    multiplier: f64,
    values: Vec<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        let multiplier = 2.0 / (period as f64 + 1.0);
        EMA {
            period,
            multiplier,
            values: Vec::new(),
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<f64> {
        if self.values.len() < self.period {
            self.values.push(price);
            return None; // Not enough data to calculate EMA
        }

        let previous_ema = if self.values.len() == self.period {
            self.values.iter().sum::<f64>() / self.period as f64
        } else {
            self.values.last().cloned().unwrap_or(price)
        };

        let ema = (price - previous_ema) * self.multiplier + previous_ema;
        self.values.push(ema);
        Some(ema)
    }
}