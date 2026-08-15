use anr::brain::cerebellum::{Cerebellum, Skill};
use anr::brain::cortex::DataOrigin;

fn make_skill(id: &str, action: &str, validated: bool) -> Skill {
    Skill {
        id: id.to_string(),
        action: action.to_string(),
        validated,
        origin: DataOrigin::Seed,
        created_at: 1000,
    }
}

#[test]
fn tc_u_cerebellum_001_new_creates_empty_cerebellum() {
    let cb = Cerebellum::new(10);
    assert_eq!(cb.skill_count(), 0);
}

#[test]
fn tc_u_cerebellum_002_add_skill_stores() {
    let mut cb = Cerebellum::new(10);
    let s = make_skill("s1", "move_forward", true);
    cb.add_skill(s).unwrap();
    assert_eq!(cb.skill_count(), 1);
}

#[test]
fn tc_u_cerebellum_003_get_skill_by_id() {
    let mut cb = Cerebellum::new(10);
    let s = make_skill("s1", "move_forward", true);
    cb.add_skill(s).unwrap();
    let found = cb.get_skill("s1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "s1");
}

#[test]
fn tc_u_cerebellum_004_get_skill_missing_returns_none() {
    let cb = Cerebellum::new(10);
    assert!(cb.get_skill("nonexistent").is_none());
}

#[test]
fn tc_u_cerebellum_005_skill_count_increments() {
    let mut cb = Cerebellum::new(10);
    assert_eq!(cb.skill_count(), 0);
    cb.add_skill(make_skill("s1", "a", true)).unwrap();
    assert_eq!(cb.skill_count(), 1);
    cb.add_skill(make_skill("s2", "b", false)).unwrap();
    assert_eq!(cb.skill_count(), 2);
}

#[test]
fn tc_u_cerebellum_006_capacity_respected() {
    let mut cb = Cerebellum::new(2);
    cb.add_skill(make_skill("s1", "a", true)).unwrap();
    cb.add_skill(make_skill("s2", "b", true)).unwrap();
    let result = cb.add_skill(make_skill("s3", "c", true));
    assert!(result.is_err());
}

#[test]
fn tc_u_cerebellum_007_validated_skills_returns_only_validated() {
    let mut cb = Cerebellum::new(10);
    cb.add_skill(make_skill("s1", "a", true)).unwrap();
    cb.add_skill(make_skill("s2", "b", false)).unwrap();
    cb.add_skill(make_skill("s3", "c", true)).unwrap();
    let validated = cb.validated_skills();
    assert_eq!(validated.len(), 2);
    assert!(validated.iter().all(|s| s.validated));
}

#[test]
fn tc_u_cerebellum_008_unvalidated_skill_stored_but_not_in_validated_list() {
    let mut cb = Cerebellum::new(10);
    cb.add_skill(make_skill("s1", "a", false)).unwrap();
    assert_eq!(cb.skill_count(), 1);
    assert!(cb.validated_skills().is_empty());
}

#[test]
fn tc_u_cerebellum_009_multiple_skills() {
    let mut cb = Cerebellum::new(10);
    for i in 0..5 {
        cb.add_skill(make_skill(
            &format!("s{}", i),
            &format!("action_{}", i),
            true,
        ))
        .unwrap();
    }
    assert_eq!(cb.skill_count(), 5);
}

#[test]
fn tc_u_cerebellum_010_action_string_preserved() {
    let mut cb = Cerebellum::new(10);
    let s = make_skill("s1", "complex_action_name", true);
    cb.add_skill(s).unwrap();
    let found = cb.get_skill("s1").unwrap();
    assert_eq!(found.action, "complex_action_name");
}

#[test]
fn tc_u_cerebellum_011_origin_recorded_as_seed() {
    let mut cb = Cerebellum::new(10);
    let s = make_skill("s1", "a", true);
    cb.add_skill(s).unwrap();
    let found = cb.get_skill("s1").unwrap();
    assert_eq!(found.origin, DataOrigin::Seed);
}

#[test]
fn tc_u_cerebellum_012_is_full_when_at_capacity() {
    let mut cb = Cerebellum::new(2);
    assert!(!cb.is_full());
    cb.add_skill(make_skill("s1", "a", true)).unwrap();
    assert!(!cb.is_full());
    cb.add_skill(make_skill("s2", "b", true)).unwrap();
    assert!(cb.is_full());
}

#[test]
fn tc_u_cerebellum_capacity_reports_correct_value() {
    let cb = Cerebellum::new(50);
    assert_eq!(cb.capacity(), 50);
}
