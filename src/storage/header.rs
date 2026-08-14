/// brain.anr Header - Primary Superblock
/// Implements: AC §48, SD-03 §3.4.1
/// Complete binary format with BLAKE3 checksum and validation
use crate::error::{Error, Result};

use super::checksum::compute_header_checksum;
use super::validate::validate_header;
use super::{SUPERBLOCK_BACKUP_OFFSET, SUPERBLOCK_OFFSET};

const BRAIN_MAGIC: &[u8; 4] = b"ANRB";
const BRAIN_FORMAT_VERSION: u32 = 1;
const BRAIN_BLOCK_SIZE: u32 = 4096;
const BRAIN_HEADER_SIZE: u32 = 288;

#[derive(Debug, Clone)]
pub struct BrainHeader {
    pub magic: [u8; 4],
    pub format_version: u32,
    pub header_size: u32,
    pub flags: u32,
    pub total_size: u64,
    pub generation: u64,
    pub cortex_offset: u64,
    pub cortex_size: u64,
    pub cerebellum_offset: u64,
    pub cerebellum_size: u64,
    pub hippocampus_offset: u64,
    pub hippocampus_size: u64,
    pub index_offset: u64,
    pub index_size: u64,
    pub metadata_offset: u64,
    pub metadata_size: u64,
    pub allocation_table_offset: u64,
    pub allocation_table_size: u64,
    pub section_table_offset: u64,
    pub section_table_count: u32,
    pub block_size: u32,
    pub checksum_algo: u8,
    pub checksum_scope: u8,
    pub header_crc: u32,
    pub checksum: [u8; 32],
}

impl BrainHeader {
    pub fn new() -> Self {
        Self {
            magic: *BRAIN_MAGIC,
            format_version: BRAIN_FORMAT_VERSION,
            header_size: BRAIN_HEADER_SIZE,
            flags: 0,
            total_size: BRAIN_HEADER_SIZE as u64,
            generation: 1,
            cortex_offset: BRAIN_BLOCK_SIZE as u64,
            cortex_size: 0,
            cerebellum_offset: 0,
            cerebellum_size: 0,
            hippocampus_offset: 0,
            hippocampus_size: 0,
            index_offset: 0,
            index_size: 0,
            metadata_offset: 0,
            metadata_size: 0,
            allocation_table_offset: 0,
            allocation_table_size: 0,
            section_table_offset: 0,
            section_table_count: 3,
            block_size: BRAIN_BLOCK_SIZE,
            checksum_algo: 0,
            checksum_scope: 0,
            header_crc: 0,
            checksum: [0u8; 32],
        }
    }

    pub fn read(path: &std::path::Path) -> Result<Self> {
        use std::fs::File;
        use std::io::Read as StdRead;

        let mut file = File::open(path)
            .map_err(|e| Error::BrainError(format!("Cannot open brain file: {}", e)))?;

        let mut buf = vec![0u8; BRAIN_HEADER_SIZE as usize];
        file.read_exact(&mut buf)
            .map_err(|e| Error::BrainError(format!("Cannot read header: {}", e)))?;

        Self::deserialize(&buf)
    }

    pub fn write(&self, path: &std::path::Path) -> Result<()> {
        use std::fs::File;
        use std::io::Write as StdWrite;

        let mut file = File::create(path)
            .map_err(|e| Error::BrainError(format!("Cannot create brain file: {}", e)))?;

        let data = self.serialize();
        file.write_all(&data)
            .map_err(|e| Error::BrainError(format!("Cannot write header: {}", e)))?;

        Ok(())
    }

    /// Write header atomically: backup first, fsync, then primary, fsync.
    /// This implements AC §45 transactional write contract.
    pub fn write_atomic(&mut self, path: &std::path::Path) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::{Seek, SeekFrom, Write};

        let data = self.serialize();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| {
                Error::StorageWriteFailed(format!("Cannot open file for atomic write: {}", e))
            })?;

        let backup_offset = SUPERBLOCK_BACKUP_OFFSET;
        file.seek(SeekFrom::Start(backup_offset)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to backup offset: {}", e))
        })?;
        file.write_all(&data).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot write backup superblock: {}", e))
        })?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync after backup write failed: {}", e))
        })?;

        file.seek(SeekFrom::Start(SUPERBLOCK_OFFSET)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to primary offset: {}", e))
        })?;
        file.write_all(&data).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot write primary superblock: {}", e))
        })?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync after primary write failed: {}", e))
        })?;

        Ok(())
    }

    /// Write only the backup superblock (used during recovery prep)
    pub fn write_backup(&self, path: &std::path::Path) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::{Seek, SeekFrom, Write};

        let data = self.serialize();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| {
                Error::StorageWriteFailed(format!("Cannot open file for backup write: {}", e))
            })?;

        let backup_offset = SUPERBLOCK_BACKUP_OFFSET;
        file.seek(SeekFrom::Start(backup_offset)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to backup offset: {}", e))
        })?;
        file.write_all(&data).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot write backup superblock: {}", e))
        })?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync after backup write failed: {}", e))
        })?;

        Ok(())
    }

    /// Write section data at the specified offset
    pub fn write_section_data(path: &std::path::Path, offset: u64, data: &[u8]) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::{Seek, SeekFrom, Write};

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| {
                Error::StorageWriteFailed(format!("Cannot open file for section write: {}", e))
            })?;

        file.seek(SeekFrom::Start(offset)).map_err(|e| {
            Error::StorageWriteFailed(format!("Cannot seek to section offset: {}", e))
        })?;
        file.write_all(data)
            .map_err(|e| Error::StorageWriteFailed(format!("Cannot write section data: {}", e)))?;
        file.sync_all().map_err(|e| {
            Error::StorageFsyncFailed(format!("fsync after section write failed: {}", e))
        })?;

        Ok(())
    }

    /// Read header from backup superblock location
    pub fn read_backup(path: &std::path::Path) -> Result<Self> {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom};

        let mut file = File::open(path).map_err(|e| {
            Error::StorageBackupCorrupt(format!("Cannot open file for backup read: {}", e))
        })?;

        file.seek(SeekFrom::Start(SUPERBLOCK_BACKUP_OFFSET))
            .map_err(|e| {
                Error::StorageBackupCorrupt(format!("Cannot seek to backup offset: {}", e))
            })?;

        let mut buf = vec![0u8; BRAIN_HEADER_SIZE as usize];
        file.read_exact(&mut buf).map_err(|e| {
            Error::StorageBackupCorrupt(format!("Cannot read backup header: {}", e))
        })?;

        Self::deserialize(&buf)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BRAIN_HEADER_SIZE as usize);

        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.format_version.to_le_bytes());
        buf.extend_from_slice(&self.header_size.to_le_bytes());
        buf.extend_from_slice(&self.flags.to_le_bytes());
        buf.extend_from_slice(&self.total_size.to_le_bytes());
        buf.extend_from_slice(&self.generation.to_le_bytes());
        buf.extend_from_slice(&self.cortex_offset.to_le_bytes());
        buf.extend_from_slice(&self.cortex_size.to_le_bytes());
        buf.extend_from_slice(&self.cerebellum_offset.to_le_bytes());
        buf.extend_from_slice(&self.cerebellum_size.to_le_bytes());
        buf.extend_from_slice(&self.hippocampus_offset.to_le_bytes());
        buf.extend_from_slice(&self.hippocampus_size.to_le_bytes());
        buf.extend_from_slice(&self.index_offset.to_le_bytes());
        buf.extend_from_slice(&self.index_size.to_le_bytes());
        buf.extend_from_slice(&self.metadata_offset.to_le_bytes());
        buf.extend_from_slice(&self.metadata_size.to_le_bytes());
        buf.extend_from_slice(&self.allocation_table_offset.to_le_bytes());
        buf.extend_from_slice(&self.allocation_table_size.to_le_bytes());
        buf.extend_from_slice(&self.section_table_offset.to_le_bytes());
        buf.extend_from_slice(&self.section_table_count.to_le_bytes());
        buf.extend_from_slice(&self.block_size.to_le_bytes());
        buf.push(self.checksum_algo);
        buf.push(self.checksum_scope);
        buf.extend_from_slice(&[0u8; 102]);
        buf.extend_from_slice(&self.header_crc.to_le_bytes());
        buf.extend_from_slice(&[0u8; 4]);
        buf.extend_from_slice(&self.checksum);

        while buf.len() < BRAIN_HEADER_SIZE as usize {
            buf.push(0);
        }

        buf
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self> {
        if buf.len() < BRAIN_HEADER_SIZE as usize {
            return Err(Error::BrainValidation("Header too short".to_string()));
        }

        let mut pos = 0;

        let magic = [buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]];
        if magic != *BRAIN_MAGIC {
            return Err(Error::BrainValidation("Invalid magic number".to_string()));
        }
        pos += 4;

        let format_version =
            u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        if format_version != BRAIN_FORMAT_VERSION {
            return Err(Error::BrainValidation(format!(
                "Unsupported format version: {}",
                format_version
            )));
        }
        pos += 4;

        let header_size = u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let flags = u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let total_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let generation = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let cortex_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let cortex_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let cerebellum_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let cerebellum_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let hippocampus_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let hippocampus_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let index_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let index_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let metadata_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let metadata_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let allocation_table_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let allocation_table_size = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;

        let section_table_offset = u64::from_le_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let section_table_count =
            u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let block_size = u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let checksum_algo = buf[pos];
        pos += 1;
        let checksum_scope = buf[pos];
        pos += 1;

        pos += 102;

        let header_crc = u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4 + 4;

        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&buf[pos..pos + 32]);

        Ok(Self {
            magic,
            format_version,
            header_size,
            flags,
            total_size,
            generation,
            cortex_offset,
            cortex_size,
            cerebellum_offset,
            cerebellum_size,
            hippocampus_offset,
            hippocampus_size,
            index_offset,
            index_size,
            metadata_offset,
            metadata_size,
            allocation_table_offset,
            allocation_table_size,
            section_table_offset,
            section_table_count,
            block_size,
            checksum_algo,
            checksum_scope,
            header_crc,
            checksum,
        })
    }

    pub fn validate(&self) -> Result<()> {
        validate_header(self)
    }

    pub fn compute_checksum(&mut self) -> [u8; 32] {
        let header_bytes = self.serialize();
        let hash = compute_header_checksum(&header_bytes);
        self.checksum = hash;
        hash
    }
}

impl Default for BrainHeader {
    fn default() -> Self {
        Self::new()
    }
}
