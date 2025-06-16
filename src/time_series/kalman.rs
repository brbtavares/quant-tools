// This file implements the Kalman Filter for state estimation.

pub struct KalmanFilter {
    // State estimate
    x: f64,
    // Estimate uncertainty
    p: f64,
    // Measurement noise
    r: f64,
    // Process noise
    q: f64,
}

impl KalmanFilter {
    pub fn new(initial_estimate: f64, initial_uncertainty: f64, measurement_noise: f64, process_noise: f64) -> Self {
        KalmanFilter {
            x: initial_estimate,
            p: initial_uncertainty,
            r: measurement_noise,
            q: process_noise,
        }
    }

    pub fn update(&mut self, measurement: f64) {
        // Prediction update
        self.p += self.q;

        // Measurement update
        let k = self.p / (self.p + self.r); // Kalman gain
        self.x += k * (measurement - self.x);
        self.p *= 1.0 - k;
    }

    pub fn get_estimate(&self) -> f64 {
        self.x
    }
}