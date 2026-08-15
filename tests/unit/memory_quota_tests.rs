use anr::memory::{MemoryQuota, PressureLevel, SectionMemoryState};

#[test]
fn tc_u_memory_quota_001() {
    let quota = MemoryQuota::new(100, 200, 300).unwrap();
    assert_eq!(quota.min(), 100);
    assert_eq!(quota.target(), 200);
    assert_eq!(quota.max(), 300);
}

#[test]
fn tc_u_memory_quota_002() {
    let quota = MemoryQuota::new(0, 100, 200).unwrap();
    assert!(quota.would_exceed(150, 60));
}

#[test]
fn tc_u_memory_quota_003() {
    let quota = MemoryQuota::new(0, 100, 200).unwrap();
    assert!(!quota.would_exceed(100, 50));
}

#[test]
fn tc_u_memory_quota_004() {
    let quota = MemoryQuota::new(0, 50, 1000).unwrap();
    let p = quota.pressure(500);
    assert!((p - 0.5).abs() < f64::EPSILON);
}

#[test]
fn tc_u_memory_quota_005() {
    let quota = MemoryQuota::new(0, 50, 1000).unwrap();
    let p = quota.pressure(0);
    assert!((p - 0.0).abs() < f64::EPSILON);
}

#[test]
fn tc_u_memory_quota_006() {
    let quota = MemoryQuota::new(0, 50, 1000).unwrap();
    let p = quota.pressure(1000);
    assert!((p - 1.0).abs() < f64::EPSILON);
}

#[test]
fn tc_u_memory_quota_007() {
    let state = SectionMemoryState {
        used_bytes: 100,
        reserved_bytes: 50,
        tier_hot_bytes: 30,
        tier_warm_bytes: 20,
        tier_cold_bytes: 10,
    };
    assert_eq!(state.total_tier_bytes(), 60);
}

#[test]
fn tc_u_memory_quota_008() {
    assert_eq!(PressureLevel::from_pressure(0.3), PressureLevel::Normal);
}

#[test]
fn tc_u_memory_quota_009() {
    assert_eq!(PressureLevel::from_pressure(0.65), PressureLevel::Monitor);
}

#[test]
fn tc_u_memory_quota_010() {
    assert_eq!(
        PressureLevel::from_pressure(0.80),
        PressureLevel::Consolidate
    );
}

#[test]
fn tc_u_memory_quota_011() {
    assert_eq!(
        PressureLevel::from_pressure(0.90),
        PressureLevel::Aggressive
    );
}

#[test]
fn tc_u_memory_quota_012() {
    assert_eq!(PressureLevel::from_pressure(0.97), PressureLevel::Emergency);
}
