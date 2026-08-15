use anr::brain::cortex::DataOrigin;
use anr::brain::hippocampus::{Episode, Hippocampus};
use anr::learning::replay::Replay;

fn make_episode(id: &str, reward: f32) -> Episode {
    Episode {
        id: id.to_string(),
        context: "test".to_string(),
        action: "action".to_string(),
        reward,
        origin: DataOrigin::Seed,
        created_at: 1000,
    }
}

#[test]
fn tc_u_replay_001_new_with_capacity() {
    let r = Replay::new(10);
    assert_eq!(r.queue_len(), 0);
    assert!(r.is_empty());
}

#[test]
fn tc_u_replay_002_enqueue_adds_candidate() {
    let mut r = Replay::new(10);
    let e = make_episode("e1", 0.5);
    r.enqueue(e, 0.8);
    assert_eq!(r.queue_len(), 1);
    assert!(!r.is_empty());
}

#[test]
fn tc_u_replay_003_select_from_empty_returns_empty() {
    let r = Replay::new(10);
    let selected = r.select(5);
    assert!(selected.is_empty());
}

#[test]
fn tc_u_replay_004_select_returns_top_k_by_score() {
    let mut r = Replay::new(10);
    r.enqueue(make_episode("e1", 0.3), 0.3);
    r.enqueue(make_episode("e2", 0.8), 0.9);
    r.enqueue(make_episode("e3", 0.5), 0.5);
    let selected = r.select(2);
    assert_eq!(selected.len(), 2);
    assert!((selected[0].score - 0.9).abs() < f32::EPSILON);
    assert!((selected[1].score - 0.5).abs() < f32::EPSILON);
}

#[test]
fn tc_u_replay_005_select_k_greater_than_queue_returns_all() {
    let mut r = Replay::new(10);
    r.enqueue(make_episode("e1", 0.5), 0.5);
    r.enqueue(make_episode("e2", 0.6), 0.6);
    let selected = r.select(10);
    assert_eq!(selected.len(), 2);
}

#[test]
fn tc_u_replay_006_acknowledge_removes_from_queue() {
    let mut r = Replay::new(10);
    r.enqueue(make_episode("e1", 0.3), 0.3);
    r.enqueue(make_episode("e2", 0.8), 0.9);
    r.acknowledge(1);
    assert_eq!(r.queue_len(), 1);
    let remaining = r.select(10);
    assert!((remaining[0].score - 0.3).abs() < f32::EPSILON);
}

#[test]
fn tc_u_replay_007_score_episode_high_prediction_error() {
    let r = Replay::new(10);
    let e = make_episode("e1", 0.5);
    let score = r.score_episode(&e, 1.0, 0.0, 0);
    assert!(score > 0.2);
}

#[test]
fn tc_u_replay_008_score_episode_high_reward() {
    let r = Replay::new(10);
    let e_high = make_episode("e1", 0.8);
    let e_low = make_episode("e2", 0.8);
    let s_high = r.score_episode(&e_high, 0.0, 0.0, 5);
    let s_low = r.score_episode(&e_low, 0.0, 0.0, 1);
    assert!(s_high > s_low);
}

#[test]
fn tc_u_replay_009_score_episode_high_novelty() {
    let r = Replay::new(10);
    let e = make_episode("e1", 0.5);
    let s_high = r.score_episode(&e, 0.0, 1.0, 0);
    let s_low = r.score_episode(&e, 0.0, 0.0, 0);
    assert!(s_high > s_low);
}

#[test]
fn tc_u_replay_010_queue_len_tracks_additions() {
    let mut r = Replay::new(10);
    assert_eq!(r.queue_len(), 0);
    r.enqueue(make_episode("e1", 0.5), 0.5);
    assert_eq!(r.queue_len(), 1);
    r.enqueue(make_episode("e2", 0.6), 0.6);
    assert_eq!(r.queue_len(), 2);
}

#[test]
fn tc_u_replay_011_is_empty_when_no_candidates() {
    let mut r = Replay::new(10);
    assert!(r.is_empty());
    r.enqueue(make_episode("e1", 0.5), 0.5);
    assert!(!r.is_empty());
    r.acknowledge(1);
    assert!(r.is_empty());
}

#[test]
fn tc_u_replay_012_learning_cycle_runs_all_modes() {
    let mut hp = Hippocampus::new(10);
    hp.add_episode(make_episode("e1", 0.5)).unwrap();
    hp.add_episode(make_episode("e2", 0.8)).unwrap();
    let mut learning = anr::learning::Learning::new();
    let result = learning.cycle(&hp).unwrap();
    assert!(result.synapse_updates > 0);
    assert!(result.episodes_replayed > 0);
    assert!(result.consolidation_decisions > 0);
}
