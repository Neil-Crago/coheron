
pub trait HotspotDetector {                                                                             
    fn detect(&self, signal: &[f64]) -> Vec<usize>;
}


#[derive(Debug, Clone)]
pub struct PercentileHotspot {
    pub percentile: f64, // e.g. 80.0 for top 20%
}

impl PercentileHotspot {
    pub fn detect(&self, signal: &[f64]) -> Vec<usize> {
        if signal.is_empty() {
            return vec![];
        }

        let mut sorted = signal.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let index = ((self.percentile / 100.0) * sorted.len() as f64).floor() as usize;
        let threshold = sorted[index.min(sorted.len() - 1)];

        signal
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v >= threshold { Some(i) } else { None })
            .collect()
    }
}

