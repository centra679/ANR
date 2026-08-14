use crate::error::Error;
/// Transaction management for brain.anr
/// Implements: AC §45 Transactional Write Contract, SD-03
/// Ensures atomic writes with power-loss recovery
use crate::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    Idle,
    Preparing,
    Writing,
    Flushing,
    Validating,
    Committed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct TransactionDescriptor {
    pub generation: u64,
    pub parent_generation: u64,
    pub started_at: u64,
    pub committed_at: Option<u64>,
    pub checksum: [u8; 32],
    pub state: TxState,
}

impl TransactionDescriptor {
    pub fn new(generation: u64, parent_generation: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            generation,
            parent_generation,
            started_at: now,
            committed_at: None,
            checksum: [0u8; 32],
            state: TxState::Preparing,
        }
    }
}

pub struct TransactionManager {
    current_generation: u64,
    current_tx: Option<TransactionDescriptor>,
}

impl TransactionManager {
    pub fn new(initial_generation: u64) -> Self {
        Self {
            current_generation: initial_generation,
            current_tx: None,
        }
    }

    /// Begin a new transaction: validate no active tx, create descriptor,
    /// write backup superblock as snapshot of current state
    pub fn begin(
        &mut self,
        header: &crate::storage::BrainHeader,
        path: &std::path::Path,
    ) -> Result<&TransactionDescriptor> {
        if self.current_tx.is_some() {
            return Err(Error::StorageTransactionConflict);
        }

        let tx = TransactionDescriptor::new(self.current_generation + 1, self.current_generation);

        header.write_backup(path)?;

        self.current_tx = Some(tx);
        Ok(self.current_tx.as_ref().unwrap())
    }

    /// Commit transaction: update header with new generation,
    /// write atomic (backup + primary), validate, return committed descriptor
    pub fn commit(
        &mut self,
        header: &mut crate::storage::BrainHeader,
        path: &std::path::Path,
    ) -> Result<()> {
        let mut tx = self
            .current_tx
            .take()
            .ok_or(Error::StorageTransactionConflict)?;

        header.generation = tx.generation;
        let _hash = header.compute_checksum();

        header.write_atomic(path)?;

        header.validate()?;

        tx.checksum = header.checksum;
        tx.state = TxState::Committed;
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        tx.committed_at = Some(now_secs);

        self.current_generation = tx.generation;
        Ok(())
    }

    /// Rollback: restore from backup superblock
    pub fn rollback(&mut self, path: &std::path::Path) -> Result<()> {
        let _tx = self
            .current_tx
            .take()
            .ok_or(Error::StorageTransactionConflict)?;

        let backup_header = crate::storage::BrainHeader::read_backup(path)?;

        backup_header.validate()?;

        let data = backup_header.serialize();
        use std::fs::OpenOptions;
        use std::io::{Seek, SeekFrom, Write};

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| {
                Error::StorageWriteFailed(format!("Cannot open file for rollback: {}", e))
            })?;

        file.seek(SeekFrom::Start(0)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to primary offset: {}", e))
        })?;
        file.write_all(&data).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot write primary during rollback: {}", e))
        })?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync during rollback failed: {}", e))
        })?;

        Ok(())
    }

    pub fn current_generation(&self) -> u64 {
        self.current_generation
    }

    pub fn active_transaction(&self) -> Option<&TransactionDescriptor> {
        self.current_tx.as_ref()
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new(0)
    }
}
