use anr::storage::{BrainHeader, TransactionManager, TxState};

fn valid_header() -> BrainHeader {
    let mut header = BrainHeader::new();
    header.cortex_offset = 0;
    header.total_size = 288;
    header.generation = 1;
    header.compute_checksum();
    header
}

#[test]
fn tc_u_transaction_001() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    let tx = tm.begin(&header, &path).unwrap();
    assert_eq!(tx.parent_generation, 1);
    assert_eq!(tx.generation, 2);
    assert_eq!(tx.state, TxState::Preparing);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_002() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    tm.commit(&mut header, &path).unwrap();
    let read_back = BrainHeader::read(&path).unwrap();
    assert_eq!(read_back.generation, 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_003() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    for expected_gen in 2..=4 {
        tm.begin(&header, &path).unwrap();
        tm.commit(&mut header, &path).unwrap();
        let read_back = BrainHeader::read(&path).unwrap();
        assert_eq!(read_back.generation, expected_gen);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_004() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    tm.rollback(&path).unwrap();
    let read_back = BrainHeader::read(&path).unwrap();
    assert_eq!(read_back.generation, 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_005() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    tm.commit(&mut header, &path).unwrap();
    let backup = BrainHeader::read_backup(&path).unwrap();
    assert_eq!(backup.generation, 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_006() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    assert!(tm.active_transaction().is_none());
    let tx = tm.begin(&header, &path).unwrap();
    assert_eq!(tx.state, TxState::Preparing);
    let _ = tx;
    tm.commit(&mut header, &path).unwrap();
    assert!(tm.active_transaction().is_none());
    assert_eq!(tm.current_generation(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_007() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    let result = tm.begin(&header, &path);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), anr::Error::StorageTransactionConflict);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_008() {
    let path = std::env::temp_dir()
        .join("anr_nonexistent_xyz_12345")
        .join("brain.anr");
    let header = valid_header();
    let mut tm = TransactionManager::new(1);
    let result = tm.begin(&header, &path);
    assert!(result.is_err());
}

#[test]
fn tc_u_transaction_009() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    tm.commit(&mut header, &path).unwrap();
    let read_back = BrainHeader::read(&path).unwrap();
    assert!(read_back.validate().is_ok());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_010() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("brain.anr");
    let mut header = valid_header();
    header.write_atomic(&path).unwrap();
    let mut tm = TransactionManager::new(header.generation);
    tm.begin(&header, &path).unwrap();
    assert!(tm.active_transaction().is_some());
    tm.rollback(&path).unwrap();
    assert!(tm.active_transaction().is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_011() {
    let dir = std::env::temp_dir().join(format!("anr_test_{}", line!()));
    std::fs::create_dir_all(&dir).unwrap();
    let path_a = dir.join("brain_a.anr");
    let path_b = dir.join("brain_b.anr");
    let mut header_a = valid_header();
    let mut header_b = valid_header();
    header_a.write_atomic(&path_a).unwrap();
    header_b.write_atomic(&path_b).unwrap();
    let mut tm_a = TransactionManager::new(1);
    let mut tm_b = TransactionManager::new(1);
    tm_a.begin(&header_a, &path_a).unwrap();
    tm_a.commit(&mut header_a, &path_a).unwrap();
    tm_b.begin(&header_b, &path_b).unwrap();
    tm_b.commit(&mut header_b, &path_b).unwrap();
    let read_a = BrainHeader::read(&path_a).unwrap();
    let read_b = BrainHeader::read(&path_b).unwrap();
    assert_eq!(read_a.generation, 2);
    assert_eq!(read_b.generation, 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn tc_u_transaction_012() {
    let tm = TransactionManager::new(0);
    assert_eq!(tm.current_generation(), 0);
    assert!(tm.active_transaction().is_none());
}
