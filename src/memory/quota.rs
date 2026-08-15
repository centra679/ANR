/// Memory quota management for brain sections.
/// Implements: AC §26, AC §27, AC §41
use crate::Error;

pub struct MemoryQuota {
    min_bytes: u64,
    target_bytes: u64,
    max_bytes: u64,
}

impl MemoryQuota {
    pub fn new(min: u64, target: u64, max: u64) -> crate::Result<Self> {
        if min > target {
            return Err(Error::ValidationInvalid(format!(
                "min ({}) exceeds target ({})",
                min, target
            )));
        }
        if target > max {
            return Err(Error::ValidationInvalid(format!(
                "target ({}) exceeds max ({})",
                target, max
            )));
        }
        Ok(Self {
            min_bytes: min,
            target_bytes: target,
            max_bytes: max,
        })
    }

    pub fn min(&self) -> u64 {
        self.min_bytes
    }

    pub fn target(&self) -> u64 {
        self.target_bytes
    }

    pub fn max(&self) -> u64 {
        self.max_bytes
    }

    pub fn would_exceed(&self, current_used: u64, additional: u64) -> bool {
        current_used.saturating_add(additional) > self.max_bytes
    }

    pub fn pressure(&self, used: u64) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        used as f64 / self.max_bytes as f64
    }
}

pub struct SectionMemoryState {
    pub used_bytes: u64,
    pub reserved_bytes: u64,
    pub tier_hot_bytes: u64,
    pub tier_warm_bytes: u64,
    pub tier_cold_bytes: u64,
}

impl SectionMemoryState {
    pub fn pressure(&self, quota: &MemoryQuota) -> f64 {
        quota.pressure(self.used_bytes)
    }

    pub fn total_tier_bytes(&self) -> u64 {
        self.tier_hot_bytes
            .saturating_add(self.tier_warm_bytes)
            .saturating_add(self.tier_cold_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PressureLevel {
    Normal,
    Monitor,
    Consolidate,
    Aggressive,
    Emergency,
}

impl PressureLevel {
    pub fn from_pressure(pressure: f64) -> Self {
        if pressure > 0.95 {
            PressureLevel::Emergency
        } else if pressure > 0.85 {
            PressureLevel::Aggressive
        } else if pressure > 0.75 {
            PressureLevel::Consolidate
        } else if pressure > 0.60 {
            PressureLevel::Monitor
        } else {
            PressureLevel::Normal
        }
    }
}
