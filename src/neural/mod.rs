pub mod block;
/// Neural Core Module
/// Implements: AC §10-14 (Cell, Column, Block, Synapse)
pub mod cell;
pub mod column;
pub mod graph;
pub mod synapse;

pub use block::{Block, BlockPool};
pub use cell::Cell;
pub use column::Column;
pub use graph::SparseGraph;
pub use synapse::{Synapse, SynapseTable};

use crate::Result;

pub struct NeuralCore {
    graph: SparseGraph,
}

impl NeuralCore {
    pub fn new() -> Result<Self> {
        Ok(Self {
            graph: SparseGraph::new(),
        })
    }
}

impl Default for NeuralCore {
    fn default() -> Self {
        Self {
            graph: SparseGraph::new(),
        }
    }
}
