use crate::wavelet::{
    FusionContext, WaveletBasis, WaveletDecomposition, WaveletEngine, WaveletFusionStrategy,
};

pub trait BeliefTensor {
    type State;
    type Observation;
    type Posterior;

    fn observe(&self) -> Self::Observation;
    fn prior(&self) -> Self::Posterior;
    fn update(&mut self, observation: &Self::Observation);
    fn entropy(&self) -> f64;
    fn mean(&self) -> f64;
}

pub trait GradientToObservation<G, O> {
    fn convert(gradient: &G) -> O;
}

pub trait ResonanceField {
    type Position;
    type Gradient;
    type Resonance;

    fn observe(&self, position: &Self::Position) -> Self::Gradient;
    fn compute_resonance(&self, position: &Self::Position) -> Self::Resonance;
    fn propagate(&mut self, position: &Self::Position, influence: &Self::Resonance);

    /// Returns the raw signal representing the resonance field.
    fn signal(&self) -> &[f64];

    /// Returns the semantic domain label (e.g. "quantum", "biological").
    fn domain_label(&self) -> &str;

    /// Returns the fusion context for spectral analysis.
    fn fusion_context(&self) -> FusionContext;

    /// Performs wavelet fusion and returns the fused decomposition.
    fn fused_spectrum<F: WaveletFusionStrategy>(
        &self,
        engine: &WaveletEngine<F>,
        level: usize,
    ) -> WaveletDecomposition {
        engine.fuse(self.signal(), &self.fusion_context(), level)
    }

    /// Optionally returns the dominant basis for this field.
    fn dominant_basis<F: WaveletFusionStrategy>(
        &self,
        engine: &WaveletEngine<F>,
    ) -> Option<WaveletBasis> {
        engine
            .score_bases(self.signal(), &self.fusion_context())
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(basis, _)| basis)
    }
}

/// Trait for entangling different semantic domains.
pub trait EntangleMap {
    type Domain;
    type Coupling;

    fn new() -> Self;
    fn get_coupling(&self, domain_a: &Self::Domain, domain_b: &Self::Domain) -> Self::Coupling;
    fn update_coupling(
        &mut self,
        domain_a: &Self::Domain,
        domain_b: &Self::Domain,
        delta: Self::Coupling,
    );
}

pub trait LawSynthEngine<B, R, E>
where
    B: BeliefTensor,
    R: ResonanceField,
    E: EntangleMap,
{
    type ControlLaw;

    fn synthesize(
        &self,
        belief: &B::Posterior,
        resonance: &R::Resonance,
        entanglement: &E,
    ) -> Self::ControlLaw;
}

pub trait CoherencePulse<B, E>
where
    B: BeliefTensor,
    E: EntangleMap,
{
    fn trigger(&mut self, belief: &mut B, entanglement: &mut E);
}

