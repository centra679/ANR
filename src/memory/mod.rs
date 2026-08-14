/// Memory Module - Subsystems
/// Implements: AC §6-9 (Cortex, Cerebellum, Hippocampus)
pub mod allocator;
pub mod gc;
pub mod quota;

pub use gc::GarbageCollector;
pub use quota::MemoryQuota;

use crate::Result;

pub struct MemoryManager {
    _quota: MemoryQuota,
}

impl MemoryManager {
    pub fn new(max_bytes: u64) -> Result<Self> {
        Ok(Self {
            _quota: MemoryQuota::new(max_bytes),
        })
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            _quota: MemoryQuota::new(1024 * 1024 * 100), // 100MB default
        }
    }
}
