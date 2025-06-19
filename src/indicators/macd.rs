// This file implements the Moving Average Convergence Divergence (MACD) indicator.

pub struct MACD {
    pub short_period: usize,
    pub long_period: usize,
    pub signal_period: usize,
}

impl MACD {
    pub fn new(short_period: usize, long_period: usize, signal_period: usize) -> Self {
        MACD {
            short_period,
            long_period,
            signal_period,
        }
    }

    pub fn calculate(&self, prices: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let short_ema = self.ema(prices, self.short_period);
        let long_ema = self.ema(prices, self.long_period);
        let macd_line: Vec<f64> = short_ema
            .iter()
            .zip(long_ema.iter())
            .map(|(s, l)| s - l)
            .collect();
        let signal_line = self.ema(&macd_line, self.signal_period);
        let histogram = self.histogram(&macd_line, &signal_line);
        (macd_line, signal_line, histogram)
    }

    fn ema(&self, prices: &[f64], period: usize) -> Vec<f64> {
        let mut ema_values = vec![0.0; prices.len()];
        let multiplier = 2.0 / (period as f64 + 1.0);
        ema_values[0] = prices[0]; // Starting point

        for i in 1..prices.len() {
            ema_values[i] = ((prices[i] - ema_values[i - 1]) * multiplier) + ema_values[i - 1];
        }

        ema_values
    }

    fn histogram(&self, macd_line: &[f64], signal_line: &[f64]) -> Vec<f64> {
        macd_line
            .iter()
            .zip(signal_line.iter())
            .map(|(m, s)| m - s)
            .collect()
    }
}
