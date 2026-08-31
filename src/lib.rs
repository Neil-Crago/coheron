/// Coheron: A Rust crate for coherent sensor fusion and belief representation.
/// This library provides various belief representations, fusion strategies,
/// and data structures to facilitate coherent reasoning under uncertainty.
pub mod beliefs;
pub mod engine;
pub mod fusion;
pub mod structs;
pub mod traits;

pub use beliefs::{
    DirichletBelief, GaussianBelief, KalmanBelief, Observation, PolynomialBelief,
    SemanticBelief,
};
pub use engine::SemanticEngine;
pub use fusion::{
    BeliefFusion, DirichletFusion, EntropyWeightedFusion, FusionStrategy, GaussianFusion,
    PolynomialFusion, ResonanceModulatedFusion,
};
pub use structs::{ControlLaw, SemanticState};
pub use traits::{BeliefTensor, GradientToObservation};

