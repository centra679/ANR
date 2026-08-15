use super::sensor::SensorFrame;

#[derive(Debug, Clone)]
pub struct PerceptionOutput {
    pub timestamp: u64,
    pub sensor_count: u32,
    pub fused_features: Vec<f32>,
    pub confidence: f32,
}

pub struct Fusion {
    feature_dim: usize,
}

impl Fusion {
    pub fn new(feature_dim: usize) -> Self {
        Self { feature_dim }
    }

    pub fn fuse(&self, frames: &[SensorFrame]) -> PerceptionOutput {
        let sensor_count = frames.len() as u32;
        let timestamp = frames.iter().map(|f| f.timestamp).max().unwrap_or(0);
        let mut fused_features = vec![0.0f32; self.feature_dim];
        let mut total_quality = 0.0f32;
        for frame in frames {
            total_quality += frame.quality;
            let bytes_per_feature = frame
                .payload
                .len()
                .checked_div(self.feature_dim)
                .unwrap_or(0);
            for (i, feat) in fused_features.iter_mut().enumerate().take(self.feature_dim) {
                let byte_idx = i * bytes_per_feature;
                let val = if byte_idx < frame.payload.len() {
                    f32::from(frame.payload[byte_idx]) / 255.0
                } else {
                    0.0
                };
                *feat += val;
            }
        }
        let confidence = if sensor_count > 0 {
            total_quality / sensor_count as f32
        } else {
            0.0
        };
        PerceptionOutput {
            timestamp,
            sensor_count,
            fused_features,
            confidence,
        }
    }

    pub fn validate_frame(&self, frame: &SensorFrame) -> bool {
        frame.is_valid()
    }
}
