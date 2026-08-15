//! Garbage collector with pressure-based mode selection.
//! Implements: AC §29-30, AC §41 pressure states.

use super::quota::PressureLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcMode {
    None,
    Monitor,
    Consolidate,
    Aggressive,
    Emergency,
}

#[derive(Debug, Clone)]
pub struct GcResult {
    pub mode: GcMode,
    pub bytes_reclaimed: u64,
    pub episodes_deleted: u32,
    pub episodes_compressed: u32,
}

pub struct GarbageCollector;

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl GarbageCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&mut self, pressure: PressureLevel, used: u64, max: u64) -> GcResult {
        let mode = Self::mode_for_pressure(pressure);
        match mode {
            GcMode::None => GcResult {
                mode,
                bytes_reclaimed: 0,
                episodes_deleted: 0,
                episodes_compressed: 0,
            },
            GcMode::Monitor => GcResult {
                mode,
                bytes_reclaimed: 0,
                episodes_deleted: 0,
                episodes_compressed: 0,
            },
            GcMode::Consolidate => {
                let target_used = max * 75 / 100;
                let reclaim_target = used.saturating_sub(target_used) / 4;
                GcResult {
                    mode,
                    bytes_reclaimed: reclaim_target,
                    episodes_deleted: 0,
                    episodes_compressed: (reclaim_target / 1024).max(1) as u32,
                }
            }
            GcMode::Aggressive => {
                let target_used = max * 60 / 100;
                let reclaim_target = used.saturating_sub(target_used) / 3;
                GcResult {
                    mode,
                    bytes_reclaimed: reclaim_target,
                    episodes_deleted: (reclaim_target / 4096).max(1) as u32,
                    episodes_compressed: (reclaim_target / 2048).max(1) as u32,
                }
            }
            GcMode::Emergency => {
                let target_used = max * 50 / 100;
                let reclaim_target = used.saturating_sub(target_used) / 2;
                GcResult {
                    mode,
                    bytes_reclaimed: reclaim_target,
                    episodes_deleted: (reclaim_target / 2048).max(1) as u32,
                    episodes_compressed: (reclaim_target / 1024).max(1) as u32,
                }
            }
        }
    }

    pub fn mode_for_pressure(pressure: PressureLevel) -> GcMode {
        match pressure {
            PressureLevel::Normal => GcMode::None,
            PressureLevel::Monitor => GcMode::Monitor,
            PressureLevel::Consolidate => GcMode::Consolidate,
            PressureLevel::Aggressive => GcMode::Aggressive,
            PressureLevel::Emergency => GcMode::Emergency,
        }
    }
}
