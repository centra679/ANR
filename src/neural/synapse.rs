/// Synapse - connection between cells/columns
/// Implements: AC §14

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Synapse {
    pub id: u32,
    pub source: u32,
    pub target: u32,
    pub weight: f32,
    pub last_active: u64,
    pub permanence: f32,
}

impl Synapse {
    pub fn new(id: u32, source: u32, target: u32) -> Self {
        Self {
            id,
            source,
            target,
            weight: 0.5,
            last_active: 0,
            permanence: 0.5,
        }
    }

    pub fn update(&mut self, learning_rate: f32, is_active: bool) {
        if is_active {
            self.weight = (self.weight + learning_rate * (1.0 - self.weight)).min(1.0);
            self.permanence = (self.permanence + learning_rate * 0.1).min(1.0);
        } else {
            self.weight = (self.weight - learning_rate * self.weight).max(0.0);
            self.permanence = (self.permanence - learning_rate * 0.05).max(0.0);
        }
    }

    pub fn decay(&mut self) {
        self.weight *= 0.99;
        self.permanence *= 0.99;
    }

    pub fn is_connected(&self) -> bool {
        self.permanence > 0.1
    }
}
