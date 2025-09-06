use crate::traits::ResonanceField;

#[derive(Debug, Clone)]
pub struct Resonance {
    pub amplitude: f64,
    pub frequency: f64,
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub direction: [f64; 2],
    pub magnitude: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct GridField {
    pub coherence_map: Vec<Vec<f64>>, // 2D grid
    pub width: usize,
    pub height: usize,
}

impl ResonanceField for GridField {
    type Position = Position;
    type Gradient = Gradient;
    type Resonance = Resonance;

    fn observe(&self, pos: &Position) -> Gradient {
        let x = pos.x as usize;
        let y = pos.y as usize;

        let center = self.coherence_map[y][x];
        let dx = self.coherence_map[y][x.saturating_sub(1)] - center;
        let dy = self.coherence_map[y.saturating_sub(1)][x] - center;

        Gradient {
            direction: [dx, dy],
            magnitude: (dx.powi(2) + dy.powi(2)).sqrt(),
        }
    }

    fn compute_resonance(&self, pos: &Position) -> Resonance {
        let grad = self.observe(pos);
        Resonance {
            amplitude: grad.magnitude,
            frequency: grad.direction[0].abs() + grad.direction[1].abs(),
        }
    }

    fn propagate(&mut self, pos: &Position, influence: &Resonance) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let delta = influence.amplitude * 0.01;

        self.coherence_map[y][x] += delta;
    }
}

fn init_field(width: usize, height: usize) -> GridField {
    let coherence_map = vec![vec![0.5; width]; height];
    GridField {
        coherence_map,
        width,
        height,
    }
}

/*
impl ResonanceField for GridField {
    type Position = Position;
    type Gradient = Gradient;
    type Resonance = Resonance;

    fn observe(&self, pos: &Position) -> Gradient { /* ... */ }
    fn compute_resonance(&self, pos: &Position) -> Resonance { /* ... */ }
    fn propagate(&mut self, pos: &Position, influence: &Resonance) { /* ... */ }
}
*/
