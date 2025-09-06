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
}

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
