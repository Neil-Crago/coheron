/// Various data structures used throughout the Coheron crate.
#[derive(Debug, Clone)]
pub struct SemanticState {
    pub coherence: f64,
    pub phase: f64,
}

#[derive(Debug, Clone)]
pub struct ControlLaw {
    pub torque: f64,
    pub alignment: f64,
}
