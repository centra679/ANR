/// ANR Library
///
/// Architecture Contract: Final Architectural Baseline v1.1
///
/// This is the core library providing all ANR functionality.

pub mod error;
pub use error::{Error, Result};

pub mod core;
pub mod neural;
pub mod brain;
pub mod learning;
pub mod memory;
pub mod storage;
pub mod perception;
pub mod plugins;
pub mod hardware;
pub mod action;
pub mod simd;
pub mod interface;

// Re-export commonly used types
pub use core::Runtime;
pub use neural::NeuralCore;
pub use brain::Brain;
pub use storage::BrainFile;
