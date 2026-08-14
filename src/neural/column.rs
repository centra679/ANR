/// Column Implementation
/// Implements: AC §13 Column Contract, SD-06
/// Collection of cells with Winner-Take-All (WTA) competition
use serde::{Deserialize, Serialize};

const WTA_THRESHOLD: f32 = 0.3;
const WTA_INHIBITION_STRENGTH: f32 = 0.9;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColumnState {
    Silent,    // No active cells
    Competing, // Multiple cells active
    Winner,    // Single winner selected
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: u32,
    pub cell_indices: Vec<u32>,  // Indices into CellPool
    pub winner_idx: Option<u32>, // Current winner cell index
    pub state: ColumnState,
    pub inhibition_level: f32,  // Current lateral inhibition
    pub competition_count: u32, // Cycles since last WTA
}

impl Column {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            cell_indices: Vec::new(),
            winner_idx: None,
            state: ColumnState::Silent,
            inhibition_level: 0.0,
            competition_count: 0,
        }
    }

    pub fn add_cell(&mut self, cell_idx: u32) {
        if !self.cell_indices.contains(&cell_idx) {
            self.cell_indices.push(cell_idx);
        }
    }

    /// Perform Winner-Take-All competition
    /// AC §13.2: Lateral inhibition and sparse activation
    pub fn winner_take_all(&mut self, cell_activations: &[f32]) -> Option<u32> {
        if self.cell_indices.is_empty() {
            self.state = ColumnState::Silent;
            self.winner_idx = None;
            return None;
        }

        // Find cell with highest activation in this column
        let mut max_activation = -1.0f32;
        let mut max_idx = None;

        for &cell_idx in &self.cell_indices {
            if let Some(activation) = cell_activations.get(cell_idx as usize) {
                if *activation > max_activation && *activation > WTA_THRESHOLD {
                    max_activation = *activation;
                    max_idx = Some(cell_idx);
                }
            }
        }

        if let Some(winner) = max_idx {
            self.winner_idx = Some(winner);
            self.state = ColumnState::Winner;
            self.inhibition_level = WTA_INHIBITION_STRENGTH;
            Some(winner)
        } else {
            self.state = ColumnState::Silent;
            self.winner_idx = None;
            self.inhibition_level = 0.0;
            None
        }
    }

    /// Get column activation (sum of active cells)
    pub fn get_activation(&self, cell_activations: &[f32]) -> f32 {
        self.cell_indices
            .iter()
            .map(|&idx| cell_activations.get(idx as usize).copied().unwrap_or(0.0))
            .sum()
    }

    /// Decay inhibition over time
    pub fn decay(&mut self) {
        self.inhibition_level *= 0.95;
        self.competition_count += 1;
    }

    pub fn is_active(&self) -> bool {
        !matches!(self.state, ColumnState::Silent)
    }

    pub fn has_winner(&self) -> bool {
        self.winner_idx.is_some()
    }
}

/// ColumnPool for SoA layout
pub struct ColumnPool {
    pub ids: Vec<u32>,
    pub cell_start: Vec<u32>, // Offset into cell array
    pub cell_len: Vec<u32>,   // Number of cells in this column
    pub winner_idx: Vec<Option<u32>>,
    pub state: Vec<ColumnState>,
    pub inhibition: Vec<f32>,
    pub usage: Vec<u32>,
}

impl ColumnPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            ids: vec![0; capacity],
            cell_start: vec![0; capacity],
            cell_len: vec![0; capacity],
            winner_idx: vec![None; capacity],
            state: vec![ColumnState::Silent; capacity],
            inhibition: vec![0.0; capacity],
            usage: vec![0; capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.ids.len()
    }

    /// Perform WTA across all active columns
    pub fn winner_take_all_all(&mut self, cell_activations: &[f32]) -> Vec<u32> {
        let mut winners = Vec::new();

        for idx in 0..self.ids.len() {
            if self.usage[idx] == 0 {
                continue;
            }

            // Get cells for this column
            let start = self.cell_start[idx] as usize;
            let len = self.cell_len[idx] as usize;

            let mut max_activation = -1.0f32;
            let mut max_cell = None;

            for cell_offset in 0..len {
                let cell_idx = start + cell_offset;
                if let Some(activation) = cell_activations.get(cell_idx) {
                    if *activation > max_activation && *activation > 0.3 {
                        max_activation = *activation;
                        max_cell = Some(cell_idx as u32);
                    }
                }
            }

            if let Some(winner) = max_cell {
                self.winner_idx[idx] = Some(winner);
                self.state[idx] = ColumnState::Winner;
                self.inhibition[idx] = 0.9;
                winners.push(winner);
            } else {
                self.winner_idx[idx] = None;
                self.state[idx] = ColumnState::Silent;
                self.inhibition[idx] = 0.0;
            }
        }

        winners
    }

    pub fn decay_all(&mut self) {
        for idx in 0..self.ids.len() {
            self.inhibition[idx] *= 0.95;
        }
    }
}
