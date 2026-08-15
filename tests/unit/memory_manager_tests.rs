use anr::memory::{MemoryManager, MemoryQuota, PressureLevel, Section};

fn make_quota(max: u64) -> MemoryQuota {
    MemoryQuota::new(max / 4, max / 2, max).unwrap()
}

#[test]
fn tc_u_memory_manager_001() {
    let mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    assert_eq!(mm.total_used(), 0);
}

#[test]
fn tc_u_memory_manager_002() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 100).unwrap();
    assert_eq!(mm.total_used(), 100);
}

#[test]
fn tc_u_memory_manager_003() {
    let mut mm = MemoryManager::new(make_quota(100), make_quota(200), make_quota(300));
    let result = mm.allocate(Section::Cortex, 200);
    assert!(result.is_err());
}

#[test]
fn tc_u_memory_manager_004() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 200).unwrap();
    mm.free(Section::Cortex, 100).unwrap();
    assert_eq!(mm.total_used(), 100);
}

#[test]
fn tc_u_memory_manager_005() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 250).unwrap();
    let p = mm.pressure(Section::Cortex);
    assert!((p - 0.25).abs() < f64::EPSILON);
}

#[test]
fn tc_u_memory_manager_006() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 100).unwrap();
    assert_eq!(mm.pressure_level(Section::Cortex), PressureLevel::Normal);
    mm.allocate(Section::Cortex, 550).unwrap();
    assert_eq!(mm.pressure_level(Section::Cortex), PressureLevel::Monitor);
}

#[test]
fn tc_u_memory_manager_007() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 800).unwrap();
    assert_eq!(mm.pressure(Section::Cerebellum), 0.0);
    assert_eq!(mm.total_used(), 800);
}

#[test]
fn tc_u_memory_manager_008() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 200).unwrap();
    mm.allocate(Section::Cerebellum, 400).unwrap();
    mm.allocate(Section::Hippocampus, 600).unwrap();
    assert_eq!(mm.total_used(), 1200);
}

#[test]
fn tc_u_memory_manager_009() {
    let mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    assert!(mm.can_allocate(Section::Cortex, 1000));
    assert!(!mm.can_allocate(Section::Cortex, 1001));
}

#[test]
fn tc_u_memory_manager_010() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 200).unwrap();
    mm.allocate(Section::Cerebellum, 400).unwrap();
    mm.allocate(Section::Hippocampus, 600).unwrap();
    assert_eq!(mm.pressure(Section::Cortex), 0.2);
    assert_eq!(mm.pressure(Section::Cerebellum), 0.2);
    assert_eq!(mm.pressure(Section::Hippocampus), 0.2);
}

#[test]
fn tc_u_memory_manager_011() {
    let mut mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    mm.allocate(Section::Cortex, 500).unwrap();
    mm.free(Section::Cortex, 500).unwrap();
    mm.allocate(Section::Cortex, 750).unwrap();
    assert_eq!(mm.total_used(), 750);
}

#[test]
fn tc_u_memory_manager_012() {
    let mm = MemoryManager::new(make_quota(1000), make_quota(2000), make_quota(3000));
    assert_eq!(mm.pressure(Section::Cortex), 0.0);
    assert_eq!(mm.pressure(Section::Cerebellum), 0.0);
    assert_eq!(mm.pressure(Section::Hippocampus), 0.0);
    assert_eq!(mm.total_used(), 0);
}
