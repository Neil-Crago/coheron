use coheron::traits::{BeliefTensor, EntangleMap, LawSynthEngine, ResonanceField};
use coheron::entangle::SimpleEntangleMap;
use coheron::entangle::SemanticDomain;
use coheron::structs::{Observation, ControlLaw};
use coheron::beliefs::GaussianBelief;   
use coheron::sem_eng::SemanticEngine;


//fn get_mean(belief: &SimpleBelief) -> f64 {
//    belief.mean
//}

fn main() {
    let mut belief = SimpleBelief { mean: 0.5, variance: 0.2 };
    let mut field = Field;
    let synth = Synth;
    let mut position = 0.0;

    for step in 0..10 {
        let signal = field.observe(&position);
        let obs = Observation { signal, noise: 0.1 };
        belief.update(&obs);


        let entanglement = SimpleEntangleMap::new(); // or however you instantiate it
        let resonance = field.compute_resonance(&position);
        let law = synth.synthesize(&belief, &resonance, &entanglement);

        position += law.torque * 0.1; // move agent
        field.propagate(&position, &resonance);

        println!(
            "Step {:>2}: Pos {:.2}, Belief {:.2}, Torque {:.2}, Align {:.2}",
            step, position, belief.mean, law.torque, law.alignment
        );
    }
}