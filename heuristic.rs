pub struct HeuristicEngine;

impl HeuristicEngine {
    pub fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() { return 0.0; }
        let mut byte_counts = [0usize; 256];
        for &byte in data { byte_counts[byte as usize] += 1; }
        
        let mut entropy = 0.0;
        let len = data.len() as f64;
        for &count in &byte_counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        entropy
    }
}