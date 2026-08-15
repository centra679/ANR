use anr::brain::cortex::DataOrigin;
use anr::brain::hippocampus::{Episode, Hippocampus};

fn make_episode(id: &str, context: &str, action: &str, reward: f32) -> Episode {
    Episode {
        id: id.to_string(),
        context: context.to_string(),
        action: action.to_string(),
        reward,
        origin: DataOrigin::Seed,
        created_at: 1000,
    }
}

#[test]
fn tc_u_hippocampus_001_new_creates_empty_hippocampus() {
    let hp = Hippocampus::new(10);
    assert_eq!(hp.episode_count(), 0);
}

#[test]
fn tc_u_hippocampus_002_add_episode_stores() {
    let mut hp = Hippocampus::new(10);
    let e = make_episode("e1", "lab", "move_forward", 0.8);
    hp.add_episode(e).unwrap();
    assert_eq!(hp.episode_count(), 1);
}

#[test]
fn tc_u_hippocampus_003_get_episode_by_id() {
    let mut hp = Hippocampus::new(10);
    let e = make_episode("e1", "lab", "move_forward", 0.8);
    hp.add_episode(e).unwrap();
    let found = hp.get_episode("e1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "e1");
}

#[test]
fn tc_u_hippocampus_004_get_episode_missing_returns_none() {
    let hp = Hippocampus::new(10);
    assert!(hp.get_episode("nonexistent").is_none());
}

#[test]
fn tc_u_hippocampus_005_episode_count_increments() {
    let mut hp = Hippocampus::new(10);
    assert_eq!(hp.episode_count(), 0);
    hp.add_episode(make_episode("e1", "lab", "a", 0.5)).unwrap();
    assert_eq!(hp.episode_count(), 1);
    hp.add_episode(make_episode("e2", "lab", "b", 0.6)).unwrap();
    assert_eq!(hp.episode_count(), 2);
}

#[test]
fn tc_u_hippocampus_006_capacity_respected() {
    let mut hp = Hippocampus::new(2);
    hp.add_episode(make_episode("e1", "lab", "a", 0.5)).unwrap();
    hp.add_episode(make_episode("e2", "lab", "b", 0.5)).unwrap();
    let result = hp.add_episode(make_episode("e3", "lab", "c", 0.5));
    assert!(result.is_err());
}

#[test]
fn tc_u_hippocampus_007_gc_eligible_returns_low_reward() {
    let mut hp = Hippocampus::new(10);
    hp.add_episode(make_episode("e1", "lab", "a", 0.1)).unwrap();
    hp.add_episode(make_episode("e2", "lab", "b", 0.2)).unwrap();
    hp.add_episode(make_episode("e3", "lab", "c", 0.8)).unwrap();
    let eligible = hp.gc_eligible();
    assert_eq!(eligible.len(), 2);
    assert!(eligible.iter().all(|e| e.reward < 0.3));
}

#[test]
fn tc_u_hippocampus_008_high_reward_not_in_gc_eligible() {
    let mut hp = Hippocampus::new(10);
    hp.add_episode(make_episode("e1", "lab", "a", 0.9)).unwrap();
    hp.add_episode(make_episode("e2", "lab", "b", 1.0)).unwrap();
    let eligible = hp.gc_eligible();
    assert!(eligible.is_empty());
}

#[test]
fn tc_u_hippocampus_009_multiple_episodes() {
    let mut hp = Hippocampus::new(10);
    for i in 0..5 {
        hp.add_episode(make_episode(&format!("e{}", i), "ctx", "act", 0.5))
            .unwrap();
    }
    assert_eq!(hp.episode_count(), 5);
}

#[test]
fn tc_u_hippocampus_010_context_action_reward_preserved() {
    let mut hp = Hippocampus::new(10);
    let e = make_episode("e1", "warehouse", "pick_item", 0.75);
    hp.add_episode(e).unwrap();
    let found = hp.get_episode("e1").unwrap();
    assert_eq!(found.context, "warehouse");
    assert_eq!(found.action, "pick_item");
    assert_eq!(found.reward, 0.75);
}

#[test]
fn tc_u_hippocampus_011_origin_recorded_as_seed() {
    let mut hp = Hippocampus::new(10);
    let e = make_episode("e1", "lab", "a", 0.5);
    hp.add_episode(e).unwrap();
    let found = hp.get_episode("e1").unwrap();
    assert_eq!(found.origin, DataOrigin::Seed);
}

#[test]
fn tc_u_hippocampus_012_is_full_when_at_capacity() {
    let mut hp = Hippocampus::new(2);
    assert!(!hp.is_full());
    hp.add_episode(make_episode("e1", "lab", "a", 0.5)).unwrap();
    assert!(!hp.is_full());
    hp.add_episode(make_episode("e2", "lab", "b", 0.5)).unwrap();
    assert!(hp.is_full());
}

#[test]
fn tc_u_hippocampus_capacity_reports_correct_value() {
    let hp = Hippocampus::new(77);
    assert_eq!(hp.capacity(), 77);
}

#[test]
fn tc_u_hippocampus_gc_eligible_boundary_zero_reward() {
    let mut hp = Hippocampus::new(10);
    hp.add_episode(make_episode("e1", "lab", "a", 0.0)).unwrap();
    let eligible = hp.gc_eligible();
    assert_eq!(eligible.len(), 1);
}
