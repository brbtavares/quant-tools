// This file contains common traits and types used across the project.

/// Trait for computing values.
pub trait Compute {
    fn compute(&self) -> f64;
}

/// Trait for fitting models to data.
pub trait Fit {
    fn fit(&mut self, data: &[f64]);
}

/// Trait for making predictions based on fitted models.
pub trait Predict {
    fn predict(&self, steps_ahead: usize) -> Vec<f64>;
}