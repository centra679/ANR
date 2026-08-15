use anr::memory::{GarbageCollector, GcMode, PressureLevel};

#[test]
fn tc_u_gc_001() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Normal, 1000, 10000);
    assert_eq!(result.bytes_reclaimed, 0);
    assert_eq!(result.episodes_deleted, 0);
}

#[test]
fn tc_u_gc_002() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Emergency, 9700, 10000);
    assert!(result.bytes_reclaimed > 0);
    assert!(result.episodes_deleted > 0);
}

#[test]
fn tc_u_gc_003() {
    assert_eq!(
        GarbageCollector::mode_for_pressure(PressureLevel::Normal),
        GcMode::None
    );
}

#[test]
fn tc_u_gc_004() {
    assert_eq!(
        GarbageCollector::mode_for_pressure(PressureLevel::Emergency),
        GcMode::Emergency
    );
}

#[test]
fn tc_u_gc_005() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Emergency, 9700, 10000);
    assert!(result.bytes_reclaimed >= 0);
}

#[test]
fn tc_u_gc_006() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Aggressive, 9000, 10000);
    assert_eq!(result.mode, GcMode::Aggressive);
}

#[test]
fn tc_u_gc_007() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Monitor, 7000, 10000);
    assert_eq!(result.bytes_reclaimed, 0);
}

#[test]
fn tc_u_gc_008() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Consolidate, 8500, 10000);
    assert!(result.bytes_reclaimed > 0);
    assert_eq!(result.mode, GcMode::Consolidate);
}

#[test]
fn tc_u_gc_009() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Aggressive, 9200, 10000);
    assert!(result.bytes_reclaimed > 0);
    assert_eq!(result.mode, GcMode::Aggressive);
}

#[test]
fn tc_u_gc_010() {
    let mut gc = GarbageCollector::new();
    let r1 = gc.collect(PressureLevel::Emergency, 9700, 10000);
    let r2 = gc.collect(PressureLevel::Emergency, 9700, 10000);
    assert!(r1.bytes_reclaimed > 0);
    assert!(r2.bytes_reclaimed > 0);
}

#[test]
fn tc_u_gc_011() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Normal, 0, 10000);
    assert_eq!(result.bytes_reclaimed, 0);
    assert_eq!(result.episodes_deleted, 0);
}

#[test]
fn tc_u_gc_012() {
    let mut gc = GarbageCollector::new();
    let result = gc.collect(PressureLevel::Emergency, 10000, 10000);
    assert!(result.bytes_reclaimed > 0);
    assert!(result.episodes_deleted > 0);
    assert!(result.episodes_compressed > 0);
}
