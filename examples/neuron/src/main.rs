use coheron::beliefs::GaussianBelief;
use coheron::fusion::{BeliefFusion, GaussianFusion};
use coheron::traits::BeliefTensor;

fn main() {
    let mut belief = GaussianBelief {
        mean: 0.5,
        variance: 0.25,
        drift: 0.02,
    };

    let mut accumulated = Vec::new();

    for step in 0..10 {
        let observation = belief.observe();
        belief.update(&observation);
        accumulated.push(belief.mean());

        println!(
            "step {step:>2}: raw={:.4}, mean={:.4}, variance={:.4}",
            observation.signal,
            belief.mean,
            belief.variance
        );
    }

    let fused = GaussianFusion::fuse(&[
        GaussianBelief { mean: 0.2, variance: 0.5, drift: 0.0 },
        GaussianBelief { mean: 0.8, variance: 0.2, drift: 0.0 },
        GaussianBelief { mean: 0.6, variance: 0.3, drift: 0.0 },
    ]);

    println!(
        "fused estimate: mean={:.4}, variance={:.4}, history={:?}",
        fused.mean,
        fused.variance,
        accumulated
    );
}