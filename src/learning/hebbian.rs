/// Hebbian learning rule: "fire together, wire together" (AC §33.1)
pub struct Hebbian {
    learning_rate: f32,
    min_weight: f32,
    max_weight: f32,
}

/// Synapse update record
#[derive(Debug, Clone)]
pub struct SynapseUpdate {
    pub source_id: u32,
    pub target_id: u32,
    pub old_weight: f32,
    pub new_weight: f32,
}

impl Hebbian {
    pub fn new(learning_rate: f32) -> Self {
        Self {
            learning_rate,
            min_weight: 0.0,
            max_weight: 1.0,
        }
    }

    pub fn update(
        &self,
        current_weight: f32,
        source_active: bool,
        target_active: bool,
    ) -> SynapseUpdate {
        let old_weight = current_weight.clamp(self.min_weight, self.max_weight);
        let new_weight = if source_active && target_active {
            (old_weight + self.learning_rate).clamp(self.min_weight, self.max_weight)
        } else {
            old_weight
        };
        SynapseUpdate {
            source_id: 0,
            target_id: 0,
            old_weight,
            new_weight,
        }
    }

    pub fn batch_update(&self, weights: &[(u32, u32, f32, bool, bool)]) -> Vec<SynapseUpdate> {
        weights
            .iter()
            .map(
                |&(source_id, target_id, current_weight, source_active, target_active)| {
                    let mut update = self.update(current_weight, source_active, target_active);
                    update.source_id = source_id;
                    update.target_id = target_id;
                    update
                },
            )
            .collect()
    }
}
