/// Recovery from corrupt generation
/// Implements: AC §43, AC §46
use crate::error::Error;
use crate::Result;
use std::path::Path;

pub struct Recovery;

impl Recovery {
    /// Attempt recovery from backup superblock.
    /// Implements AC §46 recovery contract.
    pub fn recover(path: &Path) -> Result<crate::storage::BrainHeader> {
        let primary_result = crate::storage::BrainHeader::read(path);
        let backup_result = crate::storage::BrainHeader::read_backup(path);

        match (primary_result, backup_result) {
            (Ok(primary), Ok(_backup)) => {
                if primary.validate().is_ok() {
                    return Ok(primary);
                }
                Self::restore_from_backup(path)
            }
            (Ok(primary), Err(_)) => {
                if primary.validate().is_ok() {
                    return Ok(primary);
                }
                Err(Error::StorageRecoveryFailed(
                    "Primary header invalid, no valid backup".to_string(),
                ))
            }
            (Err(_), Ok(backup)) => {
                if backup.validate().is_ok() {
                    Self::write_backup_to_primary(path, &backup)?;
                    return Ok(backup);
                }
                Err(Error::StorageRecoveryFailed(
                    "Both primary and backup headers are invalid".to_string(),
                ))
            }
            (Err(_), Err(_)) => Err(Error::StorageRecoveryFailed(
                "Both primary and backup headers are unreadable".to_string(),
            )),
        }
    }

    /// Check if recovery is needed (primary and backup differ in generation)
    pub fn recovery_needed(path: &Path) -> Result<bool> {
        let primary = crate::storage::BrainHeader::read(path)?;
        let backup = crate::storage::BrainHeader::read_backup(path)?;

        let primary_valid = primary.validate().is_ok();
        let backup_valid = backup.validate().is_ok();

        match (primary_valid, backup_valid) {
            (true, true) => Ok(primary.generation != backup.generation),
            (true, false) => Ok(false),
            (false, true) => Ok(true),
            (false, false) => Err(Error::StorageRecoveryFailed(
                "Both primary and backup headers are invalid".to_string(),
            )),
        }
    }

    fn restore_from_backup(path: &Path) -> Result<crate::storage::BrainHeader> {
        let backup = crate::storage::BrainHeader::read_backup(path)?;
        backup
            .validate()
            .map_err(|e| Error::StorageBackupCorrupt(format!("Backup validation failed: {}", e)))?;
        Self::write_backup_to_primary(path, &backup)?;
        Ok(backup)
    }

    fn write_backup_to_primary(path: &Path, backup: &crate::storage::BrainHeader) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::{Seek, SeekFrom, Write};

        let data = backup.serialize();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| {
                Error::StorageWriteFailed(format!("Cannot open file for recovery: {}", e))
            })?;

        file.seek(SeekFrom::Start(0)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to primary offset: {}", e))
        })?;
        file.write_all(&data).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot write primary during recovery: {}", e))
        })?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync during recovery failed: {}", e))
        })?;

        Ok(())
    }
}
