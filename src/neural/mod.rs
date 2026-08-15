pub mod block;
pub mod cell;
pub mod column;
pub mod graph;
pub mod synapse;

pub use block::{Block, BlockPool};
pub use cell::Cell;
pub use column::Column;
pub use graph::SparseGraph;
pub use synapse::{Synapse, SynapseTable};

use crate::simd::{detect_backend, SimdBackend};

pub struct NeuralCore {
    pub cell_pool: cell::CellPool,
    pub column_pool: column::ColumnPool,
    pub block_pool: block::BlockPool,
    pub synapse_table: synapse::SynapseTable,
    pub graph: SparseGraph,
    pub backend: SimdBackend,
}

impl NeuralCore {
    pub fn new(
        cell_capacity: usize,
        column_capacity: usize,
        block_capacity: usize,
        synapse_capacity: usize,
    ) -> Self {
        Self {
            cell_pool: cell::CellPool::new(cell_capacity),
            column_pool: column::ColumnPool::new(column_capacity),
            block_pool: block::BlockPool::new(block_capacity),
            synapse_table: synapse::SynapseTable::new(synapse_capacity),
            graph: SparseGraph::new(),
            backend: detect_backend(),
        }
    }

    pub fn cycle(&mut self, current_cycle: u64, input_currents: &[f32]) -> Vec<u32> {
        let fired = self.cell_pool.update_all(current_cycle, input_currents);

        let cap = self.cell_pool.capacity();
        let mut activations = vec![0.0f32; cap];
        activations[..cap].copy_from_slice(&self.cell_pool.activation[..cap]);
        self.column_pool.winner_take_all_all(&activations);

        fired
    }

    pub fn active_columns(&self) -> Vec<u32> {
        self.column_pool
            .state
            .iter()
            .enumerate()
            .filter(|(_, &s)| matches!(s, column::ColumnState::Winner))
            .map(|(i, _)| self.column_pool.ids[i])
            .collect()
    }

    pub fn backend(&self) -> SimdBackend {
        self.backend
    }
}

impl Default for NeuralCore {
    fn default() -> Self {
        Self::new(64, 8, 16, 128)
    }
}
