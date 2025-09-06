/// Core traits and structures for semantic signal processing and wavelet fusion.

use crate::path_evaluator::PathMetrics;

/* 
pub trait CurvatureSignal {
    fn reconstruct(&self) -> Vec<f64>;
}
*/
/// Trait for evaluating paths based on signal characteristics.
pub trait PathEvaluator {
    fn evaluate(&self, signal: &[f64]) -> PathMetrics;
}


