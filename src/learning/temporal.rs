/// Temporal learning: link sequential activations (AC §33.1)
pub struct Temporal {
    learning_rate: f32,
    temporal_window: u64,
}

/// Temporal association record
#[derive(Debug, Clone)]
pub struct TemporalAssociation {
    pub source_id: u32,
    pub target_id: u32,
    pub delay_cycles: u64,
    pub strength: f32,
}

impl Temporal {
    pub fn new(learning_rate: f32, temporal_window: u64) -> Self {
        Self {
            learning_rate,
            temporal_window,
        }
    }

    pub fn within_window(&self, time_a: u64, time_b: u64) -> bool {
        let delay = time_a.abs_diff(time_b);
        delay <= self.temporal_window
    }

    pub fn temporal_strength(&self, delay: u64) -> f32 {
        if delay >= self.temporal_window {
            0.0
        } else {
            let ratio = 1.0 - (delay as f32 / self.temporal_window as f32);
            ratio.clamp(0.0, 1.0)
        }
    }

    pub fn associate(
        &self,
        source_id: u32,
        source_time: u64,
        target_id: u32,
        target_time: u64,
    ) -> Option<TemporalAssociation> {
        if !self.within_window(source_time, target_time) {
            return None;
        }
        let delay_cycles = source_time.abs_diff(target_time);
        let base_strength = self.temporal_strength(delay_cycles);
        let strength = (base_strength * self.learning_rate).clamp(0.0, 1.0);
        Some(TemporalAssociation {
            source_id,
            target_id,
            delay_cycles,
            strength,
        })
    }
}
