use anr::storage::{BrainBuilder, BrainHeader};

fn make_seed(path: &std::path::Path) {
    let header = BrainHeader::new();
    header.write(path).unwrap();
}

#[test]
fn tc_u_builder_001() {
    let dir = std::env::temp_dir().join(format!("anr_builder_001_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    assert!(output.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_002() {
    let dir = std::env::temp_dir().join(format!("anr_builder_002_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("nonexistent_seed.anr");
    let output = dir.join("brain.anr");
    let result = BrainBuilder::build_from_seed(&seed, &output);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_003() {
    let dir = std::env::temp_dir().join(format!("anr_builder_003_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.magic, *b"ANRB");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_004() {
    let dir = std::env::temp_dir().join(format!("anr_builder_004_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.generation, 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_005() {
    let dir = std::env::temp_dir().join(format!("anr_builder_005_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.generation, 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_006() {
    let dir = std::env::temp_dir().join(format!("anr_builder_006_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert!(header.validate().is_ok());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_007() {
    let dir = std::env::temp_dir().join(format!("anr_builder_007_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("nonexistent_parent").join("brain.anr");
    make_seed(&seed);
    let result = BrainBuilder::build_from_seed(&seed, &output);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_008() {
    let dir = std::env::temp_dir().join(format!("anr_builder_008_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.format_version, 1);
    assert_eq!(header.header_size, 288);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_009() {
    let dir = std::env::temp_dir().join(format!("anr_builder_009_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.total_size, 288);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_010() {
    let dir = std::env::temp_dir().join(format!("anr_builder_010_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let h1 = BrainHeader::read(&output).unwrap();
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let h2 = BrainHeader::read(&output).unwrap();
    assert_eq!(h1.generation, h2.generation);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_011() {
    let dir = std::env::temp_dir().join(format!("anr_builder_011_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("empty_seed.anr");
    std::fs::write(&seed, &[]).unwrap();
    let output = dir.join("brain.anr");
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    assert!(output.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_builder_012() {
    let dir = std::env::temp_dir().join(format!("anr_builder_012_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let seed = dir.join("seed.anr");
    let output = dir.join("brain.anr");
    make_seed(&seed);
    BrainBuilder::build_from_seed(&seed, &output).unwrap();
    let header = BrainHeader::read(&output).unwrap();
    assert_eq!(header.block_size, 4096);
    assert_eq!(header.section_table_count, 3);
    let _ = std::fs::remove_dir_all(&dir);
}
