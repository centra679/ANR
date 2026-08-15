pub mod audio;
pub mod camera;
pub mod fusion;
pub mod sensor;

use crate::Result;

pub struct Perception {
    fusion: fusion::Fusion,
}

impl Perception {
    pub fn new(feature_dim: usize) -> Self {
        Self {
            fusion: fusion::Fusion::new(feature_dim),
        }
    }

    pub fn process_frames(
        &self,
        frames: &[sensor::SensorFrame],
    ) -> Result<fusion::PerceptionOutput> {
        Ok(self.fusion.fuse(frames))
    }
}
