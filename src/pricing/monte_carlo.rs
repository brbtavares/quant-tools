// This file implements the Monte Carlo simulation for option pricing.

pub struct MonteCarlo {
    pub num_simulations: usize,
    pub strike_price: f64,
    pub spot_price: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub time_to_maturity: f64,
}

impl MonteCarlo {
    pub fn new(num_simulations: usize, strike_price: f64, spot_price: f64, volatility: f64, risk_free_rate: f64, time_to_maturity: f64) -> Self {
        MonteCarlo {
            num_simulations,
            strike_price,
            spot_price,
            volatility,
            risk_free_rate,
            time_to_maturity,
        }
    }

    pub fn simulate(&self) -> f64 {
        let mut total_payoff = 0.0;

        for _ in 0..self.num_simulations {
            let z = rand::random::<f64>(); // Generate a random number
            let price_at_maturity = self.spot_price * (1.0 + self.volatility * z.sqrt() * self.time_to_maturity);
            let payoff = (price_at_maturity - self.strike_price).max(0.0);
            total_payoff += payoff;
        }

        let average_payoff = total_payoff / self.num_simulations as f64;
        average_payoff * (-self.risk_free_rate * self.time_to_maturity).exp()
    }
}