/// ANR Library
///
/// Architecture Contract: Final Architectural Baseline v1.1
///
/// This is the core library providing all ANR functionality.
pub mod error;
pub use error::{Error, Result};

pub mod action;
pub mod brain;
pub mod core;
pub mod hardware;
pub mod interface;
pub mod learning;
pub mod memory;
pub mod neural;
pub mod perception;
pub mod plugins;
pub mod simd;
pub mod storage;

// Re-export commonly used types
pub use brain::Brain;
pub use core::Runtime;
pub use neural::NeuralCore;
pub use storage::{
    inspect_brain, validate_header, BrainFile, BrainHeader, BrainWriter, ChecksumScope,
    InspectFormat, Recovery, TransactionDescriptor, TransactionManager, TxState,
};
