pub struct ResonanceNode {
    id: usize,
    coherence: f64,
    phase: f64,
}

pub struct ResonanceEdge {
    from: usize,
    to: usize,
    amplitude: f64,
    frequency: f64,
}

pub struct GraphKernel {
    nodes: Vec<ResonanceNode>,
    edges: Vec<ResonanceEdge>,
}