use anr::memory::{AllocId, AllocPriority, Allocator};

#[test]
fn tc_u_allocator_001() {
    let mut alloc = Allocator::new(1024);
    let id = alloc.allocate(64, AllocPriority::Normal);
    assert!(id.is_ok());
}

#[test]
fn tc_u_allocator_002() {
    let mut alloc = Allocator::new(1024);
    let id = alloc.allocate(64, AllocPriority::Normal).unwrap();
    assert!(alloc.free(id).is_ok());
    assert_eq!(alloc.used_bytes(), 0);
}

#[test]
fn tc_u_allocator_003() {
    let mut alloc = Allocator::new(1024);
    let _ = alloc.allocate(256, AllocPriority::Normal).unwrap();
    assert_eq!(alloc.used_bytes(), 256);
}

#[test]
fn tc_u_allocator_004() {
    let mut alloc = Allocator::new(100);
    let result = alloc.allocate(200, AllocPriority::Normal);
    assert!(result.is_err());
}

#[test]
fn tc_u_allocator_005() {
    let mut alloc = Allocator::new(1024);
    let id1 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    let _id2 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    alloc.free(id1).unwrap();
    let frag = alloc.fragmentation_ratio();
    assert!(frag >= 0.0 && frag <= 1.0);
}

#[test]
fn tc_u_allocator_006() {
    let mut alloc = Allocator::new(1024);
    let id1 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    let _id2 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    let id3 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    alloc.free(id1).unwrap();
    alloc.free(id3).unwrap();
    let frag_before = alloc.fragmentation_ratio();
    alloc.compact();
    let frag_after = alloc.fragmentation_ratio();
    assert!(frag_after <= frag_before);
}

#[test]
fn tc_u_allocator_007() {
    let alloc = Allocator::new(100);
    assert!(alloc.can_allocate(100));
    assert!(!alloc.can_allocate(101));
}

#[test]
fn tc_u_allocator_008() {
    let mut alloc = Allocator::new(1024);
    let id1 = alloc.allocate(128, AllocPriority::Low).unwrap();
    let id2 = alloc.allocate(256, AllocPriority::Normal).unwrap();
    let id3 = alloc.allocate(512, AllocPriority::High).unwrap();
    assert_eq!(alloc.used_bytes(), 896);
    alloc.free(id1).unwrap();
    alloc.free(id2).unwrap();
    alloc.free(id3).unwrap();
    assert_eq!(alloc.used_bytes(), 0);
}

#[test]
fn tc_u_allocator_009() {
    let mut alloc = Allocator::new(100);
    let _id1 = alloc.allocate(60, AllocPriority::High).unwrap();
    let _id2 = alloc.allocate(30, AllocPriority::Low).unwrap();
    assert!(alloc.can_allocate(9));
    assert!(!alloc.can_allocate(11));
}

#[test]
fn tc_u_allocator_010() {
    let mut alloc = Allocator::new(0);
    let result = alloc.allocate(1, AllocPriority::Normal);
    assert!(result.is_err());
}

#[test]
fn tc_u_allocator_011() {
    let mut alloc = Allocator::new(1024);
    let id = alloc.allocate(512, AllocPriority::Normal).unwrap();
    assert_eq!(alloc.free_bytes(), 512);
    alloc.free(id).unwrap();
    assert_eq!(alloc.free_bytes(), 1024);
}

#[test]
fn tc_u_allocator_012() {
    let alloc = Allocator::new(0);
    assert!((alloc.fragmentation_ratio() - 0.0).abs() < f64::EPSILON);
}
