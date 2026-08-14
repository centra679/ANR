/// Memory quota management
/// Implements: AC §26, AC §27

pub struct MemoryQuota {
    min_bytes: u64,
    target_bytes: u64,
    max_bytes: u64,
}

impl MemoryQuota {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            min_bytes: max_bytes / 4,
            target_bytes: max_bytes / 2,
            max_bytes,
        }
    }
}
