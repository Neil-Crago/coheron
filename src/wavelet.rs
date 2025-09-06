use crate::traits::ResonanceField;
use omni_wave::*;
use ndarray::ArrayViewMut1;

pub fn haar_transform(signal: &[f64]) -> Vec<f64> {
    let wavelet = wavelet::HAAR;
    let signal_len = signal.len();
    let buffer_len = signal_len + wavelet.window_size() - 2;

    // Convert signal to f32
    let mut signal_f32: Vec<f32> = signal.iter().map(|&x| x as f32).collect();
    let mut buffer_f32 = vec![0f32; buffer_len];

    // Convert Vec<f32> to ArrayViewMut1<f32> as required by omni_wave
    let signal_view = ArrayViewMut1::from_shape(signal_f32.len(), &mut signal_f32[..]).unwrap();
    let buffer_view = ArrayViewMut1::from_shape(buffer_f32.len(), &mut buffer_f32[..]).unwrap();

    decompose(
        signal_view,
        buffer_view,
        wavelet,
    );

    // Convert result back to f64
    signal_f32.iter().map(|&x| x as f64).collect()
}

/// A WaveletTransform must satisfy:
/// - Reversibility: reconstruct(decompose(s)) ≈ s
/// - Energy preservation: sum of squares of approximation + detail ≈ original signal energy
/// - Orthogonality: inner product of different wavelet functions is zero
pub trait WaveletTransform {
    type Signal;
    type Coefficients;
    type Error;
    

    fn decompose(signal: &Self::Signal) -> Result<Self::Coefficients, Self::Error>;
    fn reconstruct(coeffs: &Self::Coefficients) -> Result<Self::Signal, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Signal(Vec<f64>);

#[derive(Debug, Clone)]
pub struct Coefficients {
    pub approximation: Vec<f64>,
    pub detail: Vec<f64>,
}

#[derive(Debug)]
pub enum TransformError {
    InvalidLength,
    ReconstructionFailed,
}

pub struct HaarWavelet;

impl WaveletTransform for HaarWavelet {
    type Signal = Signal;
    type Coefficients = Coefficients;
    type Error = TransformError;

    fn decompose(signal: &Self::Signal) -> Result<Self::Coefficients, Self::Error> {
        let data = &signal.0;
        if data.len() % 2 != 0 {
            return Err(TransformError::InvalidLength);
        }

        let mut approximation = Vec::new();
        let mut detail = Vec::new();

        for i in (0..data.len()).step_by(2) {
            let a = (data[i] + data[i + 1]) / 2.0;
            let d = (data[i] - data[i + 1]) / 2.0;
            approximation.push(a);
            detail.push(d);
        }

        Ok(Coefficients { approximation, detail })
    }

    fn reconstruct(coeffs: &Self::Coefficients) -> Result<Self::Signal, Self::Error> {
        let mut signal = Vec::new();
        if coeffs.approximation.len() != coeffs.detail.len() {
            return Err(TransformError::ReconstructionFailed);
        }

        for (a, d) in coeffs.approximation.iter().zip(&coeffs.detail) {
            signal.push(a + d);
            signal.push(a - d);
        }

        Ok(Signal(signal))
    }
}

/// Represents the wavelet basis used for decomposition and reconstruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveletBasis {
    Haar,
    Daubechies(u8),       // e.g., db4, db8
    Biorthogonal(u8, u8), // (analysis order, synthesis order)
    Custom(String),       // For experimental or user-defined wavelets
}

/// Trait for fusing wavelet coefficients from multiple bases.
pub trait WaveletFusionStrategy {
    /// Fuse multiple wavelet decompositions into a unified representation.
    fn fuse(
        decompositions: &[WaveletDecomposition],
        context: &FusionContext,
    ) -> WaveletDecomposition;

    /// Optionally score each basis for its semantic fit.
    fn score_basis(
        basis: &WaveletBasis,
        signal: &[f64],
        context: &FusionContext,
    ) -> f64;
}


/// Contextual metadata for wavelet fusion.
#[derive(Debug, Clone)]
pub struct FusionContext {
    pub domain_entropy: f64,
    pub resonance_profile: Option<Vec<f64>>,
    pub semantic_tags: Vec<String>,
    pub coherence_map: Option<Vec<f64>>,     // Local coherence across signal
    pub curvature_profile: Option<Vec<f64>>, // Semantic curvature or drift
    pub domain_label: Option<String>,        // e.g. "biological", "quantum", "legal"
}


// Implement Default for FusionContext
impl Default for FusionContext {
    fn default() -> Self {
        FusionContext {
            domain_entropy: 0.0,
            resonance_profile: None,
            semantic_tags: Vec::new(),
            coherence_map: None,
            curvature_profile: None,
            domain_label: None,
        }
    }
}
pub struct EntropyWeightedFusion;

impl WaveletFusionStrategy for EntropyWeightedFusion {
    fn fuse(
        decompositions: &[WaveletDecomposition],
        context: &FusionContext,
    ) -> WaveletDecomposition {
        let mut total_weight = 0.0;
        let mut fused_coeffs = vec![0.0; decompositions[0].coefficients.len()];

        for decomp in decompositions {
            let entropy = compute_entropy(&decomp.coefficients);
            let weight = 1.0 / (entropy + 1e-6); // Avoid division by zero
            total_weight += weight;

            for (i, coeff) in decomp.coefficients.iter().enumerate() {
                fused_coeffs[i] += coeff * weight;
            }
        }

        for coeff in &mut fused_coeffs {
            *coeff /= total_weight;
        }

        WaveletDecomposition {
            basis: WaveletBasis::Custom("EntropyFused".into()),
            coefficients: fused_coeffs,
            level: decompositions[0].level,
        }
    }

    fn score_basis(
        basis: &WaveletBasis,
        signal: &[f64],
        _context: &FusionContext,
    ) -> f64 {
        let coeffs = match basis {
            WaveletBasis::Haar => haar_transform(signal),
            WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
            WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
            WaveletBasis::Custom(name) => custom_transform(signal, name),
        };
        let entropy = compute_entropy(&coeffs);
        1.0 / (entropy + 1e-6)
    }
}

fn compute_entropy(coeffs: &[f64]) -> f64 {
    let norm: f64 = coeffs.iter().map(|c| c.abs()).sum();
    coeffs
        .iter()
        .map(|c| {
            let p = c.abs() / norm;
            if p > 0.0 {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum()
}

/*
/// Decomposes a signal using the specified wavelet basis.
/// Returns the wavelet coefficients.
pub fn decompose(signal: &[f64], basis: &WaveletBasis) -> Vec<f64> {
    match basis {
        WaveletBasis::Haar => {
            // Placeholder: Haar decomposition
            haar_transform(signal)
        }
        WaveletBasis::Daubechies(order) => {
            // Placeholder: Daubechies decomposition
            daubechies_transform(signal, *order)
        }
        WaveletBasis::Biorthogonal(a, s) => {
            // Placeholder: Biorthogonal decomposition
            biorthogonal_transform(signal, *a, *s)
        }
        WaveletBasis::Custom(name) => {
            // Placeholder: Custom wavelet
            custom_transform(signal, name)
        }
    }
}
*/


/*
pub fn haar_transform(signal: &[f64]) -> Vec<f64> {
    let mut coeffs = Vec::new();
    let mut i = 0;
    while i + 1 < signal.len() {
        let avg = (signal[i] + signal[i + 1]) / 2.0;
        let diff = (signal[i] - signal[i + 1]) / 2.0;
        coeffs.push(avg);
        coeffs.push(diff);
       
       i += 2;
    }
    coeffs
}
*/

pub fn daubechies_transform(signal: &[f64], order: u8) -> Vec<f64> {
    let window = order.max(2) as usize;
    let mut coeffs = Vec::new();

    for i in 0..(signal.len().saturating_sub(window)) {
        let slice = &signal[i..i + window];
        let weight = 1.0 / window as f64;
        let avg = slice.iter().map(|x| x * weight).sum::<f64>();
        coeffs.push(avg);
    }

    coeffs
}

pub fn biorthogonal_transform(signal: &[f64], a: u8, s: u8) -> Vec<f64> {
    let analysis_window = a.max(2) as usize;
    let synthesis_window = s.max(2) as usize;
    let mut coeffs = Vec::new();

    for i in 0..(signal.len().saturating_sub(analysis_window)) {
        let slice = &signal[i..i + analysis_window];
        let analysis = slice.iter().sum::<f64>() / analysis_window as f64;

        let synth_start = i.saturating_sub(synthesis_window / 2);
        let synth_end = (synth_start + synthesis_window).min(signal.len());
        let synth_slice = &signal[synth_start..synth_end];
        let synthesis = synth_slice.iter().sum::<f64>() / synthesis_window as f64;

        coeffs.push((analysis + synthesis) / 2.0);
    }

    coeffs
}

pub fn custom_transform(signal: &[f64], name: &str) -> Vec<f64> {
    match name {
        "identity" => signal.to_vec(),
        "reverse" => signal.iter().rev().cloned().collect(),
        "pulse" => signal.iter().map(|x| x.sin() * x).collect(),
        _ => signal.to_vec(), // fallback
    }
}


pub struct ResonanceWeightedFusion;

impl WaveletFusionStrategy for ResonanceWeightedFusion {
    fn fuse(
        decompositions: &[WaveletDecomposition],
        context: &FusionContext,
    ) -> WaveletDecomposition {
        let resonance = context.resonance_profile.as_ref();
        let len = decompositions[0].coefficients.len();
        let mut fused = vec![0.0; len];
        let mut total_weight = vec![0.0; len];

        for decomp in decompositions {
            for i in 0..len {
                let r = resonance.map_or(1.0, |rp| rp[i]);
                fused[i] += decomp.coefficients[i] * r;
                total_weight[i] += r;
            }
        }

        for i in 0..len {
            fused[i] /= total_weight[i].max(1e-6);
        }

        WaveletDecomposition {
            basis: WaveletBasis::Custom("ResonanceFused".into()),
            coefficients: fused,
            level: decompositions[0].level,
        }
    }

    fn score_basis(
        basis: &WaveletBasis,
        signal: &[f64],
        context: &FusionContext,
    ) -> f64 {
        let coeffs = match basis {
            WaveletBasis::Haar => haar_transform(signal),
            WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
            WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
            WaveletBasis::Custom(name) => custom_transform(signal, name),
        };
        let resonance = context.resonance_profile.as_ref();
        coeffs
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let r = resonance.map_or(1.0, |rp| rp[i]);
                r * c.abs()
            })
            .sum::<f64>()
    }
}

pub struct WaveletEngine<F: WaveletFusionStrategy> {
    pub basis_set: Vec<WaveletBasis>,
    pub fusion_strategy: F,
}

impl<F: WaveletFusionStrategy> WaveletEngine<F> {
    pub fn new(basis_set: Vec<WaveletBasis>, fusion_strategy: F) -> Self {
        Self { basis_set, fusion_strategy }
    }

    /// Decompose a signal using all bases in the set.
    pub fn decompose_all(&self, signal: &[f64], level: usize) -> Vec<WaveletDecomposition> {
        self.basis_set
            .iter()
            .map(|basis| {
                let coeffs = match basis {
                    WaveletBasis::Haar => haar_transform(signal),
                    WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
                    WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
                    WaveletBasis::Custom(name) => custom_transform(signal, name),
                };
                WaveletDecomposition {
                    basis: basis.clone(),
                    coefficients: coeffs,
                    level,
                }
            })
            .collect()
    }

    /// Fuse decompositions using the selected strategy.
    pub fn fuse(&self, signal: &[f64], context: &FusionContext, level: usize) -> WaveletDecomposition {
        let decompositions = self.decompose_all(signal, level);
        F::fuse(&decompositions, context)
    }

    /// Score each basis for semantic fit.
    pub fn score_bases(&self, signal: &[f64], context: &FusionContext) -> Vec<(WaveletBasis, f64)> {
        self.basis_set
            .iter()
            .map(|basis| {
                let score = F::score_basis(basis, signal, context);
                (basis.clone(), score)
            })
            .collect()
    }
}


/// Holds wavelet coefficients and metadata.
pub struct WaveletDecomposition {
    pub basis: WaveletBasis,
    pub coefficients: Vec<f64>,
    pub level: usize,
}


#[derive(Debug, Clone)]
pub struct WaveletTransformStruct {
    pub levels: usize,
    pub threshold: f64,
}

impl WaveletTransformStruct {
    pub fn smooth(&self, signal: &[f64]) -> Vec<f64> {
        let mut data = signal.to_vec();
        let mut temp = vec![0.0; data.len()];

        for _ in 0..self.levels {
            let mut i = 0;
            while i + 1 < data.len() {
                let avg = (data[i] + data[i + 1]) / 2.0;
                let diff = (data[i] - data[i + 1]) / 2.0;

                temp[i / 2] = avg;
                temp[data.len() / 2 + i / 2] = if diff.abs() > self.threshold { diff } else { 0.0 };
                i += 2;
            }
            data = temp.clone();
        }

        // Reconstruct smoothed signal
        let mut recon = vec![0.0; signal.len()];
        let mut i = 0;
        while i + 1 < recon.len() {
            let avg = data[i / 2];
            let diff = data[recon.len() / 2 + i / 2];
            recon[i] = avg + diff;
            recon[i + 1] = avg - diff;
            i += 2;
        }

        recon
    }
}


pub struct BiologicalField {
    pub signal: Vec<f64>,
    pub tags: Vec<String>,
    pub resonance: Vec<f64>,
    pub curvature: Vec<f64>,
}

impl ResonanceField for BiologicalField {
    type Position = usize;
    type Gradient = f64;
    type Resonance = f64;

    fn signal(&self) -> &[f64] {
        &self.signal
    }

    fn domain_label(&self) -> &str {
        "biological"
    }

    fn fusion_context(&self) -> FusionContext {
        FusionContext {
            domain_entropy: compute_entropy(&self.signal),
            resonance_profile: Some(self.resonance.clone()),
            semantic_tags: self.tags.clone(),
            coherence_map: None,
            curvature_profile: Some(self.curvature.clone()),
            domain_label: Some("biological".into()),
        }
    }

    fn observe(&self, position: &Self::Position) -> Self::Gradient {
        self.signal.get(*position).copied().unwrap_or(0.0)
    }

    fn compute_resonance(&self, position: &Self::Position) -> Self::Resonance {
        self.resonance.get(*position).copied().unwrap_or(0.0)
    }

    fn propagate(&mut self, position: &Self::Position, influence: &Self::Resonance) {
        if let Some(r) = self.resonance.get_mut(*position) {
            *r += *influence;
        }
    }
}