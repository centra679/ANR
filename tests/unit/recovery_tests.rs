use anr::storage::{BrainHeader, Recovery, TransactionManager};
use std::io::{Seek, SeekFrom, Write};

fn make_valid_header(gen: u64) -> BrainHeader {
    let mut header = BrainHeader::new();
    header.cortex_offset = 0;
    header.total_size = 288;
    header.generation = gen;
    header.compute_checksum();
    header
}

fn corrupt_bytes(path: &std::path::Path, offset: u64, data: &[u8]) {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    file.write_all(data).unwrap();
    file.sync_all().unwrap();
}

#[test]
fn tc_u_recovery_001() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_001_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    corrupt_bytes(&path, 0, &[0xFF, 0xFF, 0xFF, 0xFF]);

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.generation, 1);
    assert_eq!(recovered.magic, *b"ANRB");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_002() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_002_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    corrupt_bytes(&path, 4096, &[0xFF, 0xFF, 0xFF, 0xFF]);

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.generation, 1);
    assert_eq!(recovered.magic, *b"ANRB");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_003() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_003_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();

    corrupt_bytes(&path, 24, &[0x00; 8]);

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.generation, 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_004() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_004_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    corrupt_bytes(&path, 0, &[0xFF, 0xFF, 0xFF, 0xFF]);

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.generation, 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_005() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_005_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    corrupt_bytes(&path, 0, &[0xFF, 0xFF, 0xFF, 0xFF]);
    corrupt_bytes(&path, 4096, &[0xFF, 0xFF, 0xFF, 0xFF]);

    let result = Recovery::recover(&path);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_006() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_006_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();

    corrupt_bytes(&path, 24, &[0x00; 8]);

    let needed = Recovery::recovery_needed(&path).unwrap();
    assert!(needed);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_007() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_007_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    tm.commit(&mut header, &path).unwrap();

    let needed = Recovery::recovery_needed(&path).unwrap();
    assert!(!needed);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_008() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_008_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");

    let zeros = vec![0u8; 8192];
    std::fs::write(&path, &zeros).unwrap();

    let result = Recovery::recover(&path);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_009() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_009_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let header = make_valid_header(1);
    header.write(&path).unwrap();

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();
    file.set_len(100).unwrap();
    drop(file);

    let result = Recovery::recover(&path);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_010() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_010_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");

    let mut header = BrainHeader::new();
    header.cortex_offset = 8192;
    header.cortex_size = 4096;
    header.total_size = 12288;
    header.generation = 1;
    header.compute_checksum();
    header.write_atomic(&path).unwrap();

    corrupt_bytes(&path, 0, &[0xFF, 0xFF, 0xFF, 0xFF]);

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.cortex_offset, 8192);
    assert_eq!(recovered.cortex_size, 4096);
    assert_eq!(recovered.generation, 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_011() {
    let dir = std::env::temp_dir().join(format!("anr_recovery_011_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = make_valid_header(1);
    header.write_atomic(&path).unwrap();

    let recovered = Recovery::recover(&path).unwrap();
    assert_eq!(recovered.generation, 1);
    assert_eq!(recovered.magic, *b"ANRB");
    assert_eq!(recovered.format_version, 1);
    assert_eq!(recovered.header_size, 288);
    assert_eq!(recovered.block_size, 4096);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_recovery_012() {
    let path = std::env::temp_dir().join("anr_nonexistent_recovery_test_xxx");
    let _ = std::fs::remove_file(&path);

    let result = Recovery::recover(&path);
    assert!(result.is_err());
}
