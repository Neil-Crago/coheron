/// Traits defining the behavior of belief tensors and gradient-to-observation conversions.
/// These traits are implemented by various belief representations.
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

