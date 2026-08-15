use anr::brain::{Brain, DataOrigin};
use anr::storage::builder::{BrainBuilder, SeedEpisode, SeedKnowledge, SeedSkill};
use std::fs;
use tempfile::TempDir;

fn tmp_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

fn valid_seed_toml() -> &'static str {
    r#"
[meta]
name = "test-brain"
version = "1.0.0"

[cortex.knowledge]
items = [
  { id = "know_1", pattern = "obstacle.front", confidence = 0.9 },
]

[cerebellum.skills]
items = [
  { id = "skill_1", action = "move_forward", validated = true },
]

[hippocampus.episodes]
items = [
  { id = "ep_001", context = "lab", action = "move_forward", reward = 0.8 },
]
"#
}

#[test]
fn tc_u_brain_prov_001_parse_seed_reads_valid_toml() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    fs::write(&seed_path, valid_seed_toml()).unwrap();
    let seed = BrainBuilder::parse_seed(&seed_path).unwrap();
    assert_eq!(seed.meta.name, "test-brain");
    assert_eq!(seed.meta.version, "1.0.0");
}

#[test]
fn tc_u_brain_prov_002_parse_seed_rejects_missing_file() {
    let result = BrainBuilder::parse_seed(std::path::Path::new("/nonexistent/seed.toml"));
    assert!(result.is_err());
}

#[test]
fn tc_u_brain_prov_003_parse_seed_rejects_malformed_toml() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("bad.toml");
    fs::write(&seed_path, "this is not valid toml {{{").unwrap();
    let result = BrainBuilder::parse_seed(&seed_path);
    assert!(result.is_err());
}

#[test]
fn tc_u_brain_prov_004_validate_seed_accepts_valid() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    fs::write(&seed_path, valid_seed_toml()).unwrap();
    let seed = BrainBuilder::parse_seed(&seed_path).unwrap();
    assert!(BrainBuilder::validate_seed(&seed).is_ok());
}

#[test]
fn tc_u_brain_prov_005_validate_seed_rejects_empty_name() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    let toml_str = r#"
[meta]
name = ""
version = "1.0.0"
"#;
    fs::write(&seed_path, toml_str).unwrap();
    let seed = BrainBuilder::parse_seed(&seed_path).unwrap();
    let result = BrainBuilder::validate_seed(&seed);
    assert!(result.is_err());
}

#[test]
fn tc_u_brain_prov_006_validate_seed_rejects_confidence_gt_1() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    let toml_str = r#"
[meta]
name = "test"
version = "1.0.0"

[cortex.knowledge]
items = [
  { id = "k1", pattern = "p", confidence = 1.5 },
]
"#;
    fs::write(&seed_path, toml_str).unwrap();
    let seed = BrainBuilder::parse_seed(&seed_path).unwrap();
    let result = BrainBuilder::validate_seed(&seed);
    assert!(result.is_err());
}

#[test]
fn tc_u_brain_prov_007_validate_seed_rejects_reward_lt_0() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    let toml_str = r#"
[meta]
name = "test"
version = "1.0.0"

[hippocampus.episodes]
items = [
  { id = "e1", context = "c", action = "a", reward = -0.5 },
]
"#;
    fs::write(&seed_path, toml_str).unwrap();
    let seed = BrainBuilder::parse_seed(&seed_path).unwrap();
    let result = BrainBuilder::validate_seed(&seed);
    assert!(result.is_err());
}

#[test]
fn tc_u_brain_prov_008_build_knowledge_record_returns_bytes() {
    let k = SeedKnowledge {
        id: "know_1".to_string(),
        pattern: "obstacle.front".to_string(),
        confidence: 0.9,
    };
    let record = BrainBuilder::build_knowledge_record(&k);
    assert!(record.len() > 0);
    assert_eq!(&record[0..4], b"ANRR");
    assert_eq!(u16::from_le_bytes([record[4], record[5]]), 0x0100);
}

#[test]
fn tc_u_brain_prov_009_build_skill_record_returns_bytes() {
    let s = SeedSkill {
        id: "skill_1".to_string(),
        action: "move_forward".to_string(),
        validated: true,
    };
    let record = BrainBuilder::build_skill_record(&s);
    assert!(record.len() > 0);
    assert_eq!(&record[0..4], b"ANRR");
    assert_eq!(u16::from_le_bytes([record[4], record[5]]), 0x0200);
}

#[test]
fn tc_u_brain_prov_010_build_episode_record_returns_bytes() {
    let e = SeedEpisode {
        id: "ep_001".to_string(),
        context: "lab".to_string(),
        action: "move_forward".to_string(),
        reward: 0.8,
    };
    let record = BrainBuilder::build_episode_record(&e);
    assert!(record.len() > 0);
    assert_eq!(&record[0..4], b"ANRR");
    assert_eq!(u16::from_le_bytes([record[4], record[5]]), 0x0300);
}

#[test]
fn tc_u_brain_prov_011_build_from_seed_creates_output_file() {
    let dir = tmp_dir();
    let seed_path = dir.path().join("seed.toml");
    let output_path = dir.path().join("brain.anr");
    fs::write(&seed_path, valid_seed_toml()).unwrap();
    BrainBuilder::build_from_seed(&seed_path, &output_path).unwrap();
    assert!(output_path.exists());
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0);
}

#[test]
fn tc_u_brain_prov_012_brain_total_objects_sums_sections() {
    let mut brain = Brain::new(10, 10, 10);
    assert_eq!(brain.total_objects(), 0);

    brain
        .cortex
        .add_knowledge(anr::brain::Knowledge {
            id: "k1".into(),
            pattern: "p".into(),
            confidence: 0.5,
            origin: DataOrigin::Seed,
            created_at: 0,
        })
        .unwrap();
    assert_eq!(brain.total_objects(), 1);

    brain
        .cerebellum
        .add_skill(anr::brain::Skill {
            id: "s1".into(),
            action: "a".into(),
            validated: true,
            origin: DataOrigin::Seed,
            created_at: 0,
        })
        .unwrap();
    assert_eq!(brain.total_objects(), 2);

    brain
        .hippocampus
        .add_episode(anr::brain::Episode {
            id: "e1".into(),
            context: "c".into(),
            action: "a".into(),
            reward: 0.5,
            origin: DataOrigin::Seed,
            created_at: 0,
        })
        .unwrap();
    assert_eq!(brain.total_objects(), 3);
}

#[test]
fn tc_u_brain_prov_012_brain_default_creates_sections() {
    let brain = Brain::default();
    assert_eq!(brain.total_objects(), 0);
    assert_eq!(brain.cortex.capacity(), 1024);
    assert_eq!(brain.cerebellum.capacity(), 1024);
    assert_eq!(brain.hippocampus.capacity(), 1024);
}
