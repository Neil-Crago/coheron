use crate::curvature_signal::CurvatureSignal;
use crate::hotspot_detector::PercentileHotspot;
use crate::path_evaluator::{TrajectoryPath, PathMetrics};
use crate::wavelet_transform::WaveletTransform;

/// Demonstrate Wavelet Transform smoothing
fn wvt() {
    let raw_signal = vec![1.0, 1.5, 0.8, 2.0, 1.2, 0.9, 1.8, 2.2];
    let wavelet = WaveletTransform { levels: 2, threshold: 0.1 };
    let smoothed = wavelet.smooth(&raw_signal);

    println!("Smoothed signal: {:?}", smoothed);
}


fn main() {
    // Simulate sparse curvature
    let positions = vec![0.0, 0.2, 0.5, 0.7, 1.0];
    let values = vec![1.0, 1.5, 0.8, 2.0, 1.2];
    let signal = CurvatureSignal { positions, values };

    let recon = signal.reconstruct();
    println!("Reconstructed signal: {:?}", recon);

    let detector = PercentileHotspot { percentile: 80.0 };
    let hotspots = detector.detect(&recon);
    println!("Hotspot indices: {:?}", hotspots);

    let evaluator = TrajectoryPath { dz_dt: 0.1 };
    let metrics = evaluator.evaluate(&recon, 0.01);
    println!(
        "Path length: {:.2}, Manhattan distance: {:.2}",
        metrics.length, metrics.manhattan_distance
    );

    wvt();
}