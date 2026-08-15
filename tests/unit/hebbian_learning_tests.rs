use anr::learning::hebbian::Hebbian;

#[test]
fn tc_u_hebbian_001_new_with_learning_rate() {
    let h = Hebbian::new(0.05);
    let update = h.update(0.5, true, true);
    assert!(update.new_weight > update.old_weight);
}

#[test]
fn tc_u_hebbian_002_update_both_active_weight_increases() {
    let h = Hebbian::new(0.1);
    let update = h.update(0.3, true, true);
    assert!((update.new_weight - 0.4).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_003_update_source_inactive_no_change() {
    let h = Hebbian::new(0.1);
    let update = h.update(0.5, false, true);
    assert!((update.new_weight - 0.5).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_004_update_target_inactive_no_change() {
    let h = Hebbian::new(0.1);
    let update = h.update(0.5, true, false);
    assert!((update.new_weight - 0.5).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_005_update_neither_active_no_change() {
    let h = Hebbian::new(0.1);
    let update = h.update(0.5, false, false);
    assert!((update.new_weight - 0.5).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_006_weight_capped_at_max_weight() {
    let h = Hebbian::new(0.1);
    let update = h.update(0.95, true, true);
    assert!((update.new_weight - 1.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_007_weight_floored_at_min_weight() {
    let h = Hebbian::new(0.1);
    let update = h.update(-0.1, false, false);
    assert!((update.old_weight - 0.0).abs() < f32::EPSILON);
    assert!((update.new_weight - 0.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_008_batch_update_processes_multiple() {
    let h = Hebbian::new(0.1);
    let weights: [(u32, u32, f32, bool, bool); 3] = [
        (1, 2, 0.5, true, true),
        (3, 4, 0.3, false, true),
        (5, 6, 0.7, true, true),
    ];
    let updates = h.batch_update(&weights);
    assert_eq!(updates.len(), 3);
    assert!((updates[0].new_weight - 0.6).abs() < f32::EPSILON);
    assert!((updates[1].new_weight - 0.3).abs() < f32::EPSILON);
    assert!((updates[2].new_weight - 0.8).abs() < f32::EPSILON);
    assert_eq!(updates[0].source_id, 1);
    assert_eq!(updates[0].target_id, 2);
}

#[test]
fn tc_u_hebbian_009_synapse_update_has_correct_fields() {
    let h = Hebbian::new(0.05);
    let update = h.update(0.3, true, true);
    assert!((update.old_weight - 0.3).abs() < f32::EPSILON);
    assert!((update.new_weight - 0.35).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_010_learning_rate_affects_magnitude() {
    let h_small = Hebbian::new(0.01);
    let h_large = Hebbian::new(0.2);
    let u_small = h_small.update(0.5, true, true);
    let u_large = h_large.update(0.5, true, true);
    assert!(u_large.new_weight > u_small.new_weight);
}

#[test]
fn tc_u_hebbian_011_multiple_updates_accumulate() {
    let h = Hebbian::new(0.1);
    let u1 = h.update(0.3, true, true);
    let u2 = h.update(u1.new_weight, true, true);
    let u3 = h.update(u2.new_weight, true, true);
    assert!((u3.new_weight - 0.6).abs() < f32::EPSILON);
}

#[test]
fn tc_u_hebbian_012_zero_learning_rate_no_change() {
    let h = Hebbian::new(0.0);
    let update = h.update(0.5, true, true);
    assert!((update.new_weight - 0.5).abs() < f32::EPSILON);
}
