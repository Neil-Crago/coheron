use coheron::field::{GridField, Position};
use coheron::belief::{GaussianBelief};
use coheron::synth::Synth;
use coheron::fusion::EntropyWeightedFusion;
use coheron::traits::{SemanticEngine, FusionStrategy, ResonanceField, LawSynthEngine};

fn main() {
    let field = GridField::new(20, 20); // assuming you’ve defined a constructor
    let position = Position { x: 10.0, y: 10.0 };

    let beliefs = vec![
        GaussianBelief { mean: 0.4, variance: 0.1, drift: 0.01 },
        GaussianBelief { mean: 0.6, variance: 0.3, drift: -0.01 },
    ];

    let engine = SemanticEngine {
        beliefs,
        fusion_strategy: Box::new(EntropyWeightedFusion),
        field,
        entanglement: (),
        synthesizer: Synth,
        position,
    };

    for step in 0..20 {
        engine. Step();
    }
}