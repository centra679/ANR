/// Block - context/sequence/episode/prediction unit
/// Implements: AC §13, SD-06
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// BlockPool - SoA layout for blocks
/// AC §16, SD-06: Production data layout must use Structure of Arrays
pub struct BlockPool {
    pub block_id: Vec<u64>,
    pub context_tag: Vec<u64>,
    pub column_set_offset: Vec<u32>,
    pub column_set_len: Vec<u32>,
    pub temporal_depth: Vec<u16>,
    pub prediction_score: Vec<f32>,
    pub state: Vec<u8>,
}

impl BlockPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            block_id: vec![0; capacity],
            context_tag: vec![0; capacity],
            column_set_offset: vec![0; capacity],
            column_set_len: vec![0; capacity],
            temporal_depth: vec![0; capacity],
            prediction_score: vec![0.0; capacity],
            state: vec![0; capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.block_id.len()
    }

    pub fn len(&self) -> usize {
        self.block_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.block_id.is_empty()
    }

    pub fn push(&mut self, block_id: u64, context_tag: u64) -> usize {
        let idx = self.block_id.len();
        self.block_id.push(block_id);
        self.context_tag.push(context_tag);
        self.column_set_offset.push(0);
        self.column_set_len.push(0);
        self.temporal_depth.push(0);
        self.prediction_score.push(0.0);
        self.state.push(0);
        idx
    }

    pub fn get(&self, idx: usize) -> Option<Block> {
        if idx >= self.block_id.len() {
            return None;
        }
        Some(Block {
            id: self.block_id[idx] as u32,
            context_id: self.context_tag[idx] as u32,
            sequence_index: self.column_set_len[idx],
            prediction_state: self.prediction_score[idx],
            prediction_error: 0.0,
        })
    }

    pub fn update_prediction(&mut self, idx: usize, predicted: f32, actual: f32) {
        if idx < self.prediction_score.len() {
            self.prediction_score[idx] = predicted;
            let error = (predicted - actual).abs();
            let _current_error = self.state.get(idx).copied().unwrap_or(0) as f32 / 255.0;
            let new_error = (error * 255.0).clamp(0.0, 255.0) as u8;
            if let Some(s) = self.state.get_mut(idx) {
                *s = new_error;
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &BlockPool)> {
        (0..self.block_id.len()).map(move |i| (i, self))
    }
}
