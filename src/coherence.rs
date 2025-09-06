use crate::traits::{BeliefTensor, EntangleMap};
pub trait CoherencePulse<B, E>
where
    B: BeliefTensor,
    E: EntangleMap,
{
    fn trigger(&mut self, belief: &mut B, entanglement: &mut E);
    fn should_trigger(&self, belief: &B) -> bool;
}

pub struct EntropyPulse {
    pub threshold: f64,
}

impl<B, E> CoherencePulse<B, E> for EntropyPulse
where
    B: BeliefTensor,
    E: EntangleMap,
{
    fn should_trigger(&self, belief: &B) -> bool {
        belief.entropy() > self.threshold
    }

    fn trigger(&mut self, belief: &mut B, _entanglement: &mut E) {
        // Crude recoherence: reduce entropy artificially
        println!(
            "🔁 Coherence pulse triggered: entropy {:.2}",
            belief.entropy()
        );
        // Optional: reset variance, amplify signal, reweight entanglement
    }
}
