/// Storage Module - brain.anr binary format and IO
/// Implements: AC §5 (Single Brain Contract), SD-03, AC §44-45

pub mod header;
pub mod format;
pub mod validator;
pub mod builder;
pub mod transaction;
pub mod recovery;

pub use header::BrainHeader;
pub use validator::BrainValidator;
pub use builder::BrainBuilder;
pub use transaction::Transaction;
pub use recovery::Recovery;

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
