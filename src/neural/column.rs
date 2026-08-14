/// Column - collection of cells
/// Implements: AC §12

use super::cell::Cell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: u32,
    pub cells: Vec<Cell>,
    pub inhibition_level: f32,
}

impl Column {
    pub fn new(id: u32, cell_count: usize) -> Self {
        let cells = (0..cell_count as u32)
            .map(|i| Cell::new(i, 0.5))
            .collect();

        Self {
            id,
            cells,
            inhibition_level: 0.0,
        }
    }

    pub fn activate(&mut self, input: f32) {
        for cell in &mut self.cells {
            cell.potential += input * (1.0 - self.inhibition_level);
            cell.fire();
        }
    }

    pub fn update(&mut self) {
        for cell in &mut self.cells {
            cell.update();
        }
        self.inhibition_level *= 0.95; // Decay inhibition
    }

    pub fn winner_take_all(&mut self) {
        // Find most active cell
        if let Some(max_idx) = self
            .cells
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.activation.partial_cmp(&b.activation).unwrap())
            .map(|(i, _)| i)
        {
            // Reset all but winner
            for (i, cell) in self.cells.iter_mut().enumerate() {
                if i != max_idx {
                    cell.activation = 0.0;
                }
            }
            self.inhibition_level = 0.8; // Set inhibition for other columns
        }
    }

    pub fn active_cells(&self) -> Vec<u32> {
        self.cells
            .iter()
            .filter(|c| c.is_firing())
            .map(|c| c.id)
            .collect()
    }
}
