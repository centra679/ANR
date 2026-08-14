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

    pub fn begin(&mut self) -> TransactionDescriptor {
        let tx = TransactionDescriptor::new(self.current_generation + 1, self.current_generation);
        self.current_tx = Some(tx.clone());
        tx
    }

    pub fn commit(&mut self, checksum: [u8; 32]) -> Result<TransactionDescriptor> {
        let mut tx = self
            .current_tx
            .clone()
            .ok_or_else(|| Error::TransactionError("No active transaction".to_string()))?;
        tx.checksum = checksum;
        tx.state = TxState::Committed;
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        tx.committed_at = Some(now_secs);
        self.current_generation = tx.generation;
        self.current_tx = None;
        Ok(tx)
    }

    pub fn rollback(&mut self) -> Result<()> {
        if let Some(_tx) = self.current_tx.take() {}
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
