use anr::brain::cortex::{Cortex, DataOrigin, Knowledge};

fn make_knowledge(id: &str, pattern: &str, confidence: f32) -> Knowledge {
    Knowledge {
        id: id.to_string(),
        pattern: pattern.to_string(),
        confidence,
        origin: DataOrigin::Seed,
        created_at: 1000,
    }
}

#[test]
fn tc_u_cortex_001_new_creates_empty_cortex() {
    let cortex = Cortex::new(10);
    assert_eq!(cortex.knowledge_count(), 0);
}

#[test]
fn tc_u_cortex_002_add_knowledge_stores() {
    let mut cortex = Cortex::new(10);
    let k = make_knowledge("k1", "pattern_a", 0.9);
    cortex.add_knowledge(k).unwrap();
    assert_eq!(cortex.knowledge_count(), 1);
}

#[test]
fn tc_u_cortex_003_get_knowledge_by_id() {
    let mut cortex = Cortex::new(10);
    let k = make_knowledge("k1", "pattern_a", 0.9);
    cortex.add_knowledge(k).unwrap();
    let found = cortex.get_knowledge("k1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "k1");
}

#[test]
fn tc_u_cortex_004_get_knowledge_missing_returns_none() {
    let cortex = Cortex::new(10);
    assert!(cortex.get_knowledge("nonexistent").is_none());
}

#[test]
fn tc_u_cortex_005_knowledge_count_increments() {
    let mut cortex = Cortex::new(10);
    assert_eq!(cortex.knowledge_count(), 0);
    cortex
        .add_knowledge(make_knowledge("k1", "p", 0.5))
        .unwrap();
    assert_eq!(cortex.knowledge_count(), 1);
    cortex
        .add_knowledge(make_knowledge("k2", "p", 0.5))
        .unwrap();
    assert_eq!(cortex.knowledge_count(), 2);
}

#[test]
fn tc_u_cortex_006_capacity_respected_rejects_when_full() {
    let mut cortex = Cortex::new(2);
    cortex
        .add_knowledge(make_knowledge("k1", "p", 0.5))
        .unwrap();
    cortex
        .add_knowledge(make_knowledge("k2", "p", 0.5))
        .unwrap();
    let result = cortex.add_knowledge(make_knowledge("k3", "p", 0.5));
    assert!(result.is_err());
}

#[test]
fn tc_u_cortex_007_is_full_when_at_capacity() {
    let mut cortex = Cortex::new(2);
    assert!(!cortex.is_full());
    cortex
        .add_knowledge(make_knowledge("k1", "p", 0.5))
        .unwrap();
    assert!(!cortex.is_full());
    cortex
        .add_knowledge(make_knowledge("k2", "p", 0.5))
        .unwrap();
    assert!(cortex.is_full());
}

#[test]
fn tc_u_cortex_008_query_by_pattern_matches() {
    let mut cortex = Cortex::new(10);
    cortex
        .add_knowledge(make_knowledge("k1", "obstacle.front", 0.9))
        .unwrap();
    cortex
        .add_knowledge(make_knowledge("k2", "obstacle.front", 0.8))
        .unwrap();
    cortex
        .add_knowledge(make_knowledge("k3", "obstacle.back", 0.7))
        .unwrap();
    let results = cortex.query_by_pattern("obstacle.front");
    assert_eq!(results.len(), 2);
}

#[test]
fn tc_u_cortex_009_query_by_pattern_no_match_returns_empty() {
    let mut cortex = Cortex::new(10);
    cortex
        .add_knowledge(make_knowledge("k1", "obstacle.front", 0.9))
        .unwrap();
    let results = cortex.query_by_pattern("nonexistent");
    assert!(results.is_empty());
}

#[test]
fn tc_u_cortex_010_multiple_knowledge_items() {
    let mut cortex = Cortex::new(10);
    for i in 0..5 {
        cortex
            .add_knowledge(make_knowledge(
                &format!("k{}", i),
                &format!("pattern_{}", i),
                0.5,
            ))
            .unwrap();
    }
    assert_eq!(cortex.knowledge_count(), 5);
}

#[test]
fn tc_u_cortex_011_confidence_value_preserved() {
    let mut cortex = Cortex::new(10);
    let k = make_knowledge("k1", "p", 0.42);
    cortex.add_knowledge(k).unwrap();
    let found = cortex.get_knowledge("k1").unwrap();
    assert_eq!(found.confidence, 0.42);
}

#[test]
fn tc_u_cortex_012_origin_recorded_as_seed() {
    let mut cortex = Cortex::new(10);
    let k = make_knowledge("k1", "p", 0.9);
    cortex.add_knowledge(k).unwrap();
    let found = cortex.get_knowledge("k1").unwrap();
    assert_eq!(found.origin, DataOrigin::Seed);
}

#[test]
fn tc_u_cortex_capacity_reports_correct_value() {
    let cortex = Cortex::new(42);
    assert_eq!(cortex.capacity(), 42);
}

#[test]
fn tc_u_cortex_query_by_pattern_empty_cortex() {
    let cortex = Cortex::new(10);
    let results = cortex.query_by_pattern("anything");
    assert!(results.is_empty());
}
