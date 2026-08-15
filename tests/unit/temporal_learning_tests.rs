use anr::learning::temporal::Temporal;

#[test]
fn tc_u_temporal_001_new_with_learning_rate_and_window() {
    let t = Temporal::new(0.1, 100);
    let assoc = t.associate(0, 10, 1, 15);
    assert!(assoc.is_some());
}

#[test]
fn tc_u_temporal_002_within_window_true_for_close_times() {
    let t = Temporal::new(0.1, 100);
    assert!(t.within_window(10, 50));
}

#[test]
fn tc_u_temporal_003_within_window_false_for_distant_times() {
    let t = Temporal::new(0.1, 100);
    assert!(!t.within_window(0, 200));
}

#[test]
fn tc_u_temporal_004_temporal_strength_decreases_with_delay() {
    let t = Temporal::new(0.1, 100);
    let s0 = t.temporal_strength(0);
    let s10 = t.temporal_strength(10);
    let s50 = t.temporal_strength(50);
    assert!(s0 > s10);
    assert!(s10 > s50);
}

#[test]
fn tc_u_temporal_005_temporal_strength_at_delay_zero_equals_1() {
    let t = Temporal::new(0.1, 100);
    let s = t.temporal_strength(0);
    assert!((s - 1.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_temporal_006_temporal_strength_at_max_delay_is_low() {
    let t = Temporal::new(0.1, 100);
    let s = t.temporal_strength(99);
    assert!(s < 0.1);
}

#[test]
fn tc_u_temporal_007_associate_within_window_returns_association() {
    let t = Temporal::new(0.5, 100);
    let assoc = t.associate(1, 10, 2, 20);
    assert!(assoc.is_some());
    let a = assoc.unwrap();
    assert_eq!(a.source_id, 1);
    assert_eq!(a.target_id, 2);
    assert_eq!(a.delay_cycles, 10);
}

#[test]
fn tc_u_temporal_008_associate_outside_window_returns_none() {
    let t = Temporal::new(0.1, 10);
    let assoc = t.associate(1, 0, 2, 100);
    assert!(assoc.is_none());
}

#[test]
fn tc_u_temporal_009_temporal_association_has_correct_fields() {
    let t = Temporal::new(0.5, 100);
    let assoc = t.associate(5, 0, 8, 50).unwrap();
    assert_eq!(assoc.source_id, 5);
    assert_eq!(assoc.target_id, 8);
    assert_eq!(assoc.delay_cycles, 50);
    assert!(assoc.strength > 0.0);
    assert!(assoc.strength <= 1.0);
}

#[test]
fn tc_u_temporal_010_learning_rate_affects_strength() {
    let t_low = Temporal::new(0.1, 100);
    let t_high = Temporal::new(0.9, 100);
    let s_low = t_low.associate(0, 0, 1, 50).unwrap().strength;
    let s_high = t_high.associate(0, 0, 1, 50).unwrap().strength;
    assert!(s_high > s_low);
}

#[test]
fn tc_u_temporal_011_zero_delay_max_strength() {
    let t = Temporal::new(1.0, 100);
    let assoc = t.associate(0, 42, 1, 42).unwrap();
    assert!((assoc.strength - 1.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_temporal_012_multiple_associations() {
    let t = Temporal::new(0.1, 100);
    let a1 = t.associate(0, 0, 1, 10);
    let a2 = t.associate(1, 10, 2, 20);
    let a3 = t.associate(2, 20, 3, 30);
    assert!(a1.is_some());
    assert!(a2.is_some());
    assert!(a3.is_some());
}
