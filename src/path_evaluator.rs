#[derive(Debug)]
pub struct PathMetrics {
    pub length: f64,
    pub manhattan_distance: f64,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

pub struct TrajectoryPath {
    pub dz_dt: f64, // optional z-bias
}

impl TrajectoryPath {
    pub fn evaluate(&self, curvature: &[f64], dt: f64) -> PathMetrics {
        let mut theta = Vec::with_capacity(curvature.len());
        let mut x = Vec::with_capacity(curvature.len());
        let mut y = Vec::with_capacity(curvature.len());

        let mut angle = 0.0;
        let mut px = 0.0;
        let mut py = 0.0;

        for &k in curvature {
            angle += k * dt;
            px += angle.cos() * dt;
            py += angle.sin() * dt;

            theta.push(angle);
            x.push(px);
            y.push(py);
        }

        let length = curvature.len() as f64 * dt;
        let dx = x.last().unwrap_or(&0.0) - x.first().unwrap_or(&0.0);
        let dy = y.last().unwrap_or(&0.0) - y.first().unwrap_or(&0.0);
        let manhattan = dx.abs() + dy.abs();

        PathMetrics {
            length,
            manhattan_distance: manhattan,
            x,
            y,
        }
    }
}

