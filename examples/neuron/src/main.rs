use coheron::traits::{BeliefTensor};
use curvature::entangle::{SimpleEntangleMap};
use curvature::sem_eng::SimpleBelief;
use coheron::beliefs::{Observation};   
use curvature::sem_eng::{Synth, Field};
use curvature::resonance::{EntangleMap, LawSynthEngine, ResonanceField, Position};


fn main() {
    let mut belief = SimpleBelief { mean: 0.5, variance: 0.2 };
    let mut field = Field;
    let synth = Synth;
    let mut position = Position { x: 0.0, y: 0.0 };

    for step in 0..10 {
        let signal = field.observe(&position);
        let obs = Observation { signal, noise: 0.1 };
        belief.update(&obs);


        let entanglement = SimpleEntangleMap::new(); // or however you instantiate it
        let resonance = field.compute_resonance(&position);
        let law = synth.synthesize(&belief, &resonance, &entanglement);

        position.x += law.torque * 0.1; // move agent
        position.y += law.alignment * 0.1; // move agent
        field.propagate(&position, &resonance);

        println!(
            "Step {:>2}: Pos ({:.2}, {:.2}), Belief {:.2}, Torque {:.2}, Align {:.2}",
            step, position.x, position.y, belief.mean, law.torque, law.alignment
        );
    }
}