pub mod builder;
pub mod checksum;
/// Storage Module - brain.anr binary format and IO
/// Implements: AC §5 (Single Brain Contract), SD-03, AC §44-45
pub mod header;
pub mod inspect;
pub mod recovery;
pub mod transaction;
pub mod validate;

#[cfg(test)]
pub mod fixture_tests;

pub use builder::BrainBuilder;
pub use checksum::{compute_blake3, verify_blake3, ChecksumScope};
pub use header::BrainHeader;
pub use inspect::{dump_header_json, dump_header_text, inspect_brain, InspectFormat};
pub use recovery::Recovery;
pub use transaction::{TransactionDescriptor, TransactionManager, TxState};
pub use validate::validate_header;

use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

/// Magic bytes for brain.anr files
pub const BRAIN_MAGIC: &[u8] = b"ANRB";
pub const BLOCK_SIZE: u64 = 4096;
pub const FORMAT_VERSION: u32 = 1;

/// Primary superblock location
pub const SUPERBLOCK_OFFSET: u64 = 0;
pub const SUPERBLOCK_BACKUP_OFFSET: u64 = BLOCK_SIZE;

/// Section type identifier for brain.anr sections (SD-03 §3.4.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionType {
    Cortex = 1,
    Cerebellum = 2,
    Hippocampus = 3,
}

impl SectionType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(SectionType::Cortex),
            2 => Some(SectionType::Cerebellum),
            3 => Some(SectionType::Hippocampus),
            _ => None,
        }
    }
}

pub struct BrainFile {
    path: std::path::PathBuf,
    header: BrainHeader,
}

impl BrainFile {
    pub fn open(path: &Path) -> Result<Self> {
        let header = BrainHeader::read(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            header,
        })
    }

    pub fn validate(&self) -> Result<()> {
        let header = BrainHeader::read(&self.path)?;
        header.validate()
    }

    pub fn header(&self) -> &BrainHeader {
        &self.header
    }
}

pub struct BrainWriter {
    path: PathBuf,
    header: BrainHeader,
    tx_manager: TransactionManager,
}

impl BrainWriter {
    pub fn open(path: &Path) -> Result<Self> {
        let header = BrainHeader::read(path)?;
        let tm = TransactionManager::new(header.generation);
        Ok(Self {
            path: path.to_path_buf(),
            header,
            tx_manager: tm,
        })
    }

    pub fn header(&self) -> &BrainHeader {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut BrainHeader {
        &mut self.header
    }

    pub fn begin_transaction(&mut self) -> Result<()> {
        self.tx_manager.begin(&self.header, &self.path)?;
        Ok(())
    }

    pub fn commit_transaction(&mut self) -> Result<()> {
        self.tx_manager.commit(&mut self.header, &self.path)
    }

    pub fn rollback_transaction(&mut self) -> Result<()> {
        self.tx_manager.rollback(&self.path)
    }

    pub fn write_section(&mut self, section_type: SectionType, data: &[u8]) -> Result<()> {
        let offset = match section_type {
            SectionType::Cortex => self.header.cortex_offset,
            SectionType::Cerebellum => self.header.cerebellum_offset,
            SectionType::Hippocampus => self.header.hippocampus_offset,
        };

        if offset == 0 {
            return Err(Error::StorageWriteFailed(format!(
                "Section {:?} has zero offset",
                section_type
            )));
        }

        BrainHeader::write_section_data(&self.path, offset, data)?;

        match section_type {
            SectionType::Cortex => self.header.cortex_size = data.len() as u64,
            SectionType::Cerebellum => self.header.cerebellum_size = data.len() as u64,
            SectionType::Hippocampus => self.header.hippocampus_size = data.len() as u64,
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.header.write_atomic(&self.path)
    }
}
