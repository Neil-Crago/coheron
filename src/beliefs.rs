use crate::traits::BeliefTensor;

#[derive(Debug, Clone)]
pub struct GaussianBelief {
    pub mean: f64,
    pub variance: f64,
    pub drift: f64, // optional: models semantic drift over time
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub signal: f64,
    pub noise: f64,
}

impl BeliefTensor for GaussianBelief {
    type State = f64;
    type Observation = Observation;
    type Posterior = Self;

    fn observe(&self) -> Self::Observation {
        let signal = self.mean + self.drift + 0.1 * rand::random::<f64>();
        Observation { signal, noise: 0.1 }
    }

    fn prior(&self) -> Self::Posterior {
        self.clone()
    }

    fn update(&mut self, obs: &Self::Observation) {
        let kalman_gain = self.variance / (self.variance + obs.noise);
        self.mean += kalman_gain * (obs.signal - self.mean);
        self.variance *= 1.0 - kalman_gain;
    }

    fn entropy(&self) -> f64 {
        0.5 * (2.0 * std::f64::consts::PI * self.variance).ln()
    }

    fn mean(&self) -> f64 {
        self.mean
    }
}

#[derive(Debug, Clone)]
pub struct KalmanBelief {
    pub state: [f64; 2], // [position, velocity]
    pub covariance: [[f64; 2]; 2],
    pub process_noise: f64,
    pub measurement_noise: f64,
}

#[derive(Debug, Clone)]
pub struct KalmanObservation {
    pub measurement: f64,
}

impl BeliefTensor for KalmanBelief {
    type State = [f64; 2];
    type Observation = KalmanObservation;
    type Posterior = Self;

    fn observe(&self) -> Self::Observation {
        KalmanObservation {
            measurement: self.state[0] + 0.1 * rand::random::<f64>(),
        }
    }

    fn prior(&self) -> Self::Posterior {
        self.clone()
    }

    fn update(&mut self, obs: &Self::Observation) {
        // Simplified 1D Kalman update
        let k = self.covariance[0][0] / (self.covariance[0][0] + self.measurement_noise);
        self.state[0] += k * (obs.measurement - self.state[0]);
        self.covariance[0][0] *= 1.0 - k;
    }

    fn entropy(&self) -> f64 {
        self.covariance[0][0].ln()
    }

    fn mean(&self) -> f64 {
        self.state[0]
    }
}

#[derive(Debug, Clone)]
pub struct PolynomialBelief {
    pub coeffs: Vec<f64>, // e.g. [a0, a1, a2] for quadratic
    pub noise: f64,
}

#[derive(Debug, Clone)]
pub struct PolyObservation {
    pub input: f64,
    pub output: f64,
}

impl BeliefTensor for PolynomialBelief {
    type State = Vec<f64>;
    type Observation = PolyObservation;
    type Posterior = Self;

    fn observe(&self) -> Self::Observation {
        let x = rand::random::<f64>();
        let y = self
            .coeffs
            .iter()
            .enumerate()
            .map(|(i, c)| c * x.powi(i as i32))
            .sum::<f64>();
        PolyObservation {
            input: x,
            output: y + self.noise * rand::random::<f64>(),
        }
    }

    fn prior(&self) -> Self::Posterior {
        self.clone()
    }

    fn update(&mut self, obs: &Self::Observation) {
        // Crude gradient descent update
        for (i, c) in self.coeffs.iter_mut().enumerate() {
            let grad = obs.input.powi(i as i32) * (obs.output - *c);
            *c += 0.01 * grad;
        }
    }

    fn entropy(&self) -> f64 {
        self.coeffs.iter().map(|c| c.abs().ln()).sum()
    }

    fn mean(&self) -> f64 {
        // Return the constant term as the mean (or customize as needed)
        *self.coeffs.first().unwrap_or(&0.0)
    }
}

#[derive(Debug, Clone)]
pub struct DirichletBelief {
    pub alpha: Vec<f64>, // concentration parameters
}

#[derive(Debug, Clone)]
pub struct CategoryObservation {
    pub category: usize,
}

impl BeliefTensor for DirichletBelief {
    type State = Vec<f64>; // normalized probabilities
    type Observation = CategoryObservation;
    type Posterior = Self;

    fn observe(&self) -> Self::Observation {
        let category = {
            use rand::Rng;
            let mut rng = rand::rng();
            rng.random_range(0..self.alpha.len())
        };
        CategoryObservation { category }
    }

    fn prior(&self) -> Self::Posterior {
        self.clone()
    }

    fn update(&mut self, obs: &Self::Observation) {
        self.alpha[obs.category] += 1.0;
    }

    fn entropy(&self) -> f64 {
        let sum: f64 = self.alpha.iter().sum();
        -self.alpha.iter().map(|a| a / sum * a.ln()).sum::<f64>()
    }

    fn mean(&self) -> f64 {
        let sum: f64 = self.alpha.iter().sum();
        if self.alpha.is_empty() {
            0.0
        } else {
            self.alpha.iter().map(|a| a / sum).sum::<f64>() / self.alpha.len() as f64
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticBelief<T> {
    pub state: T,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct SemanticObservation<T> {
    pub signal: T,
}

impl<T: Clone + std::fmt::Debug> BeliefTensor for SemanticBelief<T> {
    type State = T;
    type Observation = SemanticObservation<T>;
    type Posterior = Self;

    fn observe(&self) -> Self::Observation {
        SemanticObservation {
            signal: self.state.clone(), // placeholder
        }
    }

    fn prior(&self) -> Self::Posterior {
        self.clone()
    }

    fn update(&mut self, obs: &Self::Observation) {
        self.state = obs.signal.clone();
        self.confidence *= 1.1;
    }

    fn entropy(&self) -> f64 {
        1.0 / self.confidence
    }

    fn mean(&self) -> f64 {
        self.confidence
    }
}
