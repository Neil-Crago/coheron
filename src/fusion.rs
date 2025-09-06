use crate::beliefs::{DirichletBelief, GaussianBelief, PolynomialBelief};
use crate::traits::BeliefTensor;

pub trait BeliefFusion<B: BeliefTensor> {
    fn fuse(beliefs: &[B]) -> B::Posterior;
}

pub struct GaussianFusion;

impl BeliefFusion<GaussianBelief> for GaussianFusion {
    fn fuse(beliefs: &[GaussianBelief]) -> GaussianBelief {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for b in beliefs {
            let weight = 1.0 / b.variance.max(1e-6); // inverse variance
            weighted_sum += b.mean * weight;
            total_weight += weight;
        }

        let fused_mean = weighted_sum / total_weight;
        let fused_variance = 1.0 / total_weight;

        GaussianBelief {
            mean: fused_mean,
            variance: fused_variance,
            drift: 0.0,
        }
    }
}

pub struct PolynomialFusion;

impl BeliefFusion<PolynomialBelief> for PolynomialFusion {
    fn fuse(beliefs: &[PolynomialBelief]) -> PolynomialBelief {
        if beliefs.is_empty() {
            return PolynomialBelief {
                coeffs: vec![],
                noise: 0.1,
            };
        }

        let degree = beliefs[0].coeffs.len();
        let mut fused = vec![0.0; degree];

        for b in beliefs {
            for (i, c) in b.coeffs.iter().enumerate() {
                fused[i] += *c;
            }
        }

        for c in &mut fused {
            *c /= beliefs.len() as f64;
        }

        PolynomialBelief {
            coeffs: fused,
            noise: beliefs[0].noise,
        }
    }
}

pub struct DirichletFusion;

impl BeliefFusion<DirichletBelief> for DirichletFusion {
    fn fuse(beliefs: &[DirichletBelief]) -> DirichletBelief {
        if beliefs.is_empty() {
            return DirichletBelief { alpha: vec![] };
        }

        let len = beliefs[0].alpha.len();
        let mut fused = vec![0.0; len];

        for b in beliefs {
            for (i, a) in b.alpha.iter().enumerate() {
                fused[i] += *a;
            }
        }

        DirichletBelief { alpha: fused }
    }
}

pub trait FusionStrategy<B: BeliefTensor> {
    fn fuse(&self, beliefs: &[B]) -> B::Posterior;
    fn name(&self) -> &'static str; // optional: for logging or introspection
}

pub struct EntropyWeightedFusion;

impl FusionStrategy<GaussianBelief> for EntropyWeightedFusion {
    fn fuse(&self, beliefs: &[GaussianBelief]) -> GaussianBelief {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for b in beliefs {
            let weight = 1.0 / b.variance.max(1e-6);
            weighted_sum += b.mean * weight;
            total_weight += weight;
        }

        let fused_mean = weighted_sum / total_weight;
        let fused_variance = 1.0 / total_weight;

        GaussianBelief {
            mean: fused_mean,
            variance: fused_variance,
            drift: 0.0,
        }
    }

    fn name(&self) -> &'static str {
        "EntropyWeightedFusion"
    }
}

pub struct ResonanceModulatedFusion {
    pub resonance_amplitude: f64,
}

impl FusionStrategy<GaussianBelief> for ResonanceModulatedFusion {
    fn fuse(&self, beliefs: &[GaussianBelief]) -> GaussianBelief {
        let mut sum = 0.0;
        let mut count = 0.0;

        for b in beliefs {
            let modulated = b.mean * self.resonance_amplitude;
            sum += modulated;
            count += 1.0;
        }

        GaussianBelief {
            mean: sum / count,
            variance: 0.1,
            drift: 0.0,
        }
    }

    fn name(&self) -> &'static str {
        "ResonanceModulatedFusion"
    }
}
