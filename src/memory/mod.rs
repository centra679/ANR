/// Memory Module - Subsystems
/// Implements: AC §6-9 (Cortex, Cerebellum, Hippocampus)

pub mod allocator;
pub mod quota;
pub mod gc;

pub use quota::MemoryQuota;
pub use gc::GarbageCollector;

use crate::Result;

pub struct MemoryManager {
    quota: MemoryQuota,
}

impl MemoryManager {
    pub fn new(max_bytes: u64) -> Result<Self> {
        Ok(Self {
            quota: MemoryQuota::new(max_bytes),
        })
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            quota: MemoryQuota::new(1024 * 1024 * 100), // 100MB default
        }
    }
}
