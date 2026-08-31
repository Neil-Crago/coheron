use coheron::beliefs::GaussianBelief;
use coheron::engine::SemanticEngine;
use coheron::fusion::{BeliefFusion, EntropyWeightedFusion, GaussianFusion};

fn main() {
    let mut engine = SemanticEngine::new(
        vec![
            GaussianBelief { mean: 0.4, variance: 0.1, drift: 0.01 },
            GaussianBelief { mean: 0.6, variance: 0.3, drift: -0.01 },
            GaussianBelief { mean: 0.9, variance: 0.2, drift: 0.02 },
        ],
        EntropyWeightedFusion,
    );

    for step in 0..8 {
        let (posterior, law) = engine.step_with_control();
        println!(
            "step {step:>2}: coherence={:.4}, phase={:.4}, fused_mean={:.4}, fused_variance={:.4}, torque={:.4}, alignment={:.4}",
            engine.state().coherence,
            engine.state().phase,
            posterior.mean,
            posterior.variance,
            law.torque,
            law.alignment
        );

        engine.push(GaussianBelief {
            mean: posterior.mean,
            variance: posterior.variance,
            drift: 0.01 * (step as f64 + 1.0),
        });
    }

    let fused = GaussianFusion::fuse(&[
        GaussianBelief { mean: 0.2, variance: 0.5, drift: 0.0 },
        GaussianBelief { mean: 0.8, variance: 0.2, drift: 0.0 },
    ]);
    println!(
        "final static fusion: mean={:.4}, variance={:.4}",
        fused.mean,
        fused.variance
    );
}