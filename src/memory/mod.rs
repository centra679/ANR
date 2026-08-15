/// Memory Module - Subsystems
/// Implements: AC §6-9 (Cortex, Cerebellum, Hippocampus), AC §10 (memory isolation), AC §41 (pressure)
pub mod allocator;
pub mod gc;
pub mod quota;

pub use allocator::{AllocId, AllocPriority, Allocator};
pub use gc::{GarbageCollector, GcMode, GcResult};
pub use quota::{MemoryQuota, PressureLevel, SectionMemoryState};

use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Cortex,
    Cerebellum,
    Hippocampus,
}

pub struct MemoryManager {
    cortex_quota: MemoryQuota,
    cerebellum_quota: MemoryQuota,
    hippocampus_quota: MemoryQuota,
    cortex_state: SectionMemoryState,
    cerebellum_state: SectionMemoryState,
    hippocampus_state: SectionMemoryState,
}

impl MemoryManager {
    pub fn new(cortex: MemoryQuota, cerebellum: MemoryQuota, hippocampus: MemoryQuota) -> Self {
        Self {
            cortex_quota: cortex,
            cerebellum_quota: cerebellum,
            hippocampus_quota: hippocampus,
            cortex_state: SectionMemoryState {
                used_bytes: 0,
                reserved_bytes: 0,
                tier_hot_bytes: 0,
                tier_warm_bytes: 0,
                tier_cold_bytes: 0,
            },
            cerebellum_state: SectionMemoryState {
                used_bytes: 0,
                reserved_bytes: 0,
                tier_hot_bytes: 0,
                tier_warm_bytes: 0,
                tier_cold_bytes: 0,
            },
            hippocampus_state: SectionMemoryState {
                used_bytes: 0,
                reserved_bytes: 0,
                tier_hot_bytes: 0,
                tier_warm_bytes: 0,
                tier_cold_bytes: 0,
            },
        }
    }

    pub fn allocate(&mut self, section: Section, bytes: u64) -> Result<()> {
        let (quota, state) = self.section_mut(section);
        if quota.would_exceed(state.used_bytes, bytes) {
            return Err(crate::Error::MemoryQuotaExceeded {
                section: format!("{:?}", section),
                used: state.used_bytes + bytes,
                max: quota.max(),
            });
        }
        state.used_bytes += bytes;
        Ok(())
    }

    pub fn free(&mut self, section: Section, bytes: u64) -> Result<()> {
        let state = self.section_state_mut(section);
        state.used_bytes = state.used_bytes.saturating_sub(bytes);
        Ok(())
    }

    pub fn pressure(&self, section: Section) -> f64 {
        let (quota, state) = self.section_ref(section);
        state.pressure(quota)
    }

    pub fn pressure_level(&self, section: Section) -> PressureLevel {
        let p = self.pressure(section);
        PressureLevel::from_pressure(p)
    }

    pub fn can_allocate(&self, section: Section, bytes: u64) -> bool {
        let (quota, state) = self.section_ref(section);
        !quota.would_exceed(state.used_bytes, bytes)
    }

    pub fn total_used(&self) -> u64 {
        self.cortex_state.used_bytes
            + self.cerebellum_state.used_bytes
            + self.hippocampus_state.used_bytes
    }

    fn section_ref(&self, section: Section) -> (&MemoryQuota, &SectionMemoryState) {
        match section {
            Section::Cortex => (&self.cortex_quota, &self.cortex_state),
            Section::Cerebellum => (&self.cerebellum_quota, &self.cerebellum_state),
            Section::Hippocampus => (&self.hippocampus_quota, &self.hippocampus_state),
        }
    }

    fn section_mut(&mut self, section: Section) -> (&MemoryQuota, &mut SectionMemoryState) {
        match section {
            Section::Cortex => (&self.cortex_quota, &mut self.cortex_state),
            Section::Cerebellum => (&self.cerebellum_quota, &mut self.cerebellum_state),
            Section::Hippocampus => (&self.hippocampus_quota, &mut self.hippocampus_state),
        }
    }

    fn section_state_mut(&mut self, section: Section) -> &mut SectionMemoryState {
        match section {
            Section::Cortex => &mut self.cortex_state,
            Section::Cerebellum => &mut self.cerebellum_state,
            Section::Hippocampus => &mut self.hippocampus_state,
        }
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        let default_quota = |min_frac: f64, target_frac: f64, max: u64| {
            MemoryQuota::new(
                (max as f64 * min_frac) as u64,
                (max as f64 * target_frac) as u64,
                max,
            )
            .expect("default quota params are valid")
        };
        let max_each: u64 = 100 * 1024 * 1024;
        Self::new(
            default_quota(0.25, 0.5, max_each),
            default_quota(0.25, 0.5, max_each),
            default_quota(0.25, 0.5, max_each),
        )
    }
}
