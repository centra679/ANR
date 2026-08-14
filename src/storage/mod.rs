pub mod builder;
pub mod checksum;
pub mod format;
/// Storage Module - brain.anr binary format and IO
/// Implements: AC §5 (Single Brain Contract), SD-03, AC §44-45
pub mod header;
pub mod inspect;
pub mod recovery;
pub mod transaction;
pub mod validate;
pub mod validator;

#[cfg(test)]
pub mod fixture_tests;

pub use builder::BrainBuilder;
pub use checksum::{compute_blake3, verify_blake3, ChecksumScope};
pub use header::BrainHeader;
pub use inspect::{dump_header_json, dump_header_text, inspect_brain, InspectFormat};
pub use recovery::Recovery;
pub use transaction::TransactionManager;
pub use validate::validate_header;
pub use validator::BrainValidator;

use crate::Result;
use std::path::Path;

/// Magic bytes for brain.anr files
pub const BRAIN_MAGIC: &[u8] = b"ANRB";
pub const BLOCK_SIZE: u64 = 4096;
pub const FORMAT_VERSION: u32 = 1;

/// Primary superblock location
pub const SUPERBLOCK_OFFSET: u64 = 0;
pub const SUPERBLOCK_BACKUP_OFFSET: u64 = BLOCK_SIZE;

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
        BrainValidator::validate(&self.path)
    }

    pub fn header(&self) -> &BrainHeader {
        &self.header
    }
}
