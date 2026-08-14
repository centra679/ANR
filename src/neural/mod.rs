/// Neural Core Module
/// Implements: AC §10-14 (Cell, Column, Block, Synapse)

pub mod cell;
pub mod column;
pub mod block;
pub mod synapse;
pub mod graph;

pub use cell::Cell;
pub use column::Column;
pub use block::Block;
pub use synapse::Synapse;
pub use graph::SparseGraph;

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
