/// Block - context/sequence/episode/prediction unit
/// Implements: AC §13

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    pub context_id: u32,
    pub sequence_index: u32,
    pub prediction_state: f32,
    pub prediction_error: f32,
}

impl Block {
    pub fn new(id: u32, context_id: u32) -> Self {
        Self {
            id,
            context_id,
            sequence_index: 0,
            prediction_state: 0.0,
            prediction_error: 0.0,
        }
    }

    pub fn update_prediction(&mut self, predicted: f32, actual: f32) {
        self.prediction_error = (predicted - actual).abs();
        self.prediction_state = predicted;
    }
}
