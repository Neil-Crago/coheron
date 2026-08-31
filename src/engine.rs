use crate::fusion::FusionStrategy;
use crate::structs::{ControlLaw, SemanticState};
use crate::traits::BeliefTensor;

pub struct SemanticEngine<B, F>
where
    B: BeliefTensor,
    F: FusionStrategy<B>,
{
    beliefs: Vec<B>,
    fusion: F,
    state: SemanticState,
    step: usize,
}

impl<B, F> SemanticEngine<B, F>
where
    B: BeliefTensor + Clone,
    B::Posterior: BeliefTensor,
    F: FusionStrategy<B>,
{
    pub fn new(beliefs: Vec<B>, fusion: F) -> Self {
        let mut engine = Self {
            beliefs,
            fusion,
            state: SemanticState {
                coherence: 0.0,
                phase: 0.0,
            },
            step: 0,
        };
        engine.recompute_state();
        engine
    }

    pub fn beliefs(&self) -> &[B] {
        &self.beliefs
    }

    pub fn state(&self) -> &SemanticState {
        &self.state
    }

    pub fn push(&mut self, belief: B) {
        self.beliefs.push(belief);
    }

    pub fn step(&mut self) -> B::Posterior {
        let posterior = self.fusion.fuse(&self.beliefs);
        self.recompute_state();
        self.step += 1;
        posterior
    }

    pub fn step_with_control(&mut self) -> (B::Posterior, ControlLaw) {
        let posterior = self.fusion.fuse(&self.beliefs);
        let law = self.synthesize_control(&posterior);
        self.recompute_state();
        self.step += 1;
        (posterior, law)
    }

    pub fn synthesize_control(&self, belief: &B::Posterior) -> ControlLaw {
        let mean = belief.mean();
        let entropy = belief.entropy();

        let torque = (mean - 0.5) * 2.0 * (1.0 - (entropy / (entropy + 1.0)));
        let alignment = self.state.coherence * (1.0 - mean.abs()) * 2.0;

        ControlLaw {
            torque: torque.clamp(-1.0, 1.0),
            alignment: alignment.clamp(-1.0, 1.0),
        }
    }

    fn recompute_state(&mut self) {
        if self.beliefs.is_empty() {
            self.state = SemanticState {
                coherence: 0.0,
                phase: 0.0,
            };
            return;
        }

        let average_entropy: f64 = self
            .beliefs
            .iter()
            .map(BeliefTensor::entropy)
            .sum::<f64>()
            / self.beliefs.len() as f64;

        let coherence = 1.0 / (1.0 + average_entropy.max(1e-12));
        let phase = (self.step as f64 * 0.55).sin();

        self.state = SemanticState {
            coherence,
            phase,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::beliefs::GaussianBelief;
    use crate::fusion::{BeliefFusion, EntropyWeightedFusion, GaussianFusion};
    use super::SemanticEngine;

    #[test]
    fn semantic_engine_fuses_and_tracks_coherence() {
        let mut engine = SemanticEngine::new(
            vec![
                GaussianBelief { mean: 0.2, variance: 0.5, drift: 0.0 },
                GaussianBelief { mean: 0.8, variance: 0.2, drift: 0.0 },
            ],
            EntropyWeightedFusion,
        );

        let fused = engine.step();
        assert!(fused.mean.is_finite());
        assert!(engine.state().coherence >= 0.0);
        assert!(engine.state().phase.is_finite());
    }

    #[test]
    fn semantic_engine_synthesizes_control_law_from_fused_belief() {
        let mut engine = SemanticEngine::new(
            vec![
                GaussianBelief { mean: 0.2, variance: 0.5, drift: 0.0 },
                GaussianBelief { mean: 0.8, variance: 0.2, drift: 0.0 },
            ],
            EntropyWeightedFusion,
        );

        let (posterior, law) = engine.step_with_control();
        assert!(posterior.mean.is_finite());
        assert!(law.torque.is_finite());
        assert!(law.alignment.is_finite());
    }

    #[test]
    fn gaussian_fusion_matches_inverse_variance_expectation() {
        let beliefs = vec![
            GaussianBelief { mean: 0.0, variance: 1.0, drift: 0.0 },
            GaussianBelief { mean: 2.0, variance: 0.25, drift: 0.0 },
        ];

        let fused = GaussianFusion::fuse(&beliefs);
        assert!((fused.mean - 1.6).abs() < 1e-12);
        assert!((fused.variance - 0.2).abs() < 1e-12);
    }
}
