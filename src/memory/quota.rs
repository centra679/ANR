/// Memory quota management
/// Implements: AC §26, AC §27
pub struct MemoryQuota {
    _min_bytes: u64,
    _target_bytes: u64,
    _max_bytes: u64,
}

impl MemoryQuota {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            _min_bytes: max_bytes / 4,
            _target_bytes: max_bytes / 2,
            _max_bytes: max_bytes,
        }
    }
}
