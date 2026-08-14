/// brain.anr Header - Primary Superblock
/// Implements: AC §48, SD-03 §3.4.1
/// Complete binary format with BLAKE3 checksum and validation
use crate::error::{Error, Result};

const BRAIN_MAGIC: &[u8; 4] = b"ANRB";
const BRAIN_FORMAT_VERSION: u32 = 1;
const BRAIN_BLOCK_SIZE: u32 = 4096;
const BRAIN_HEADER_SIZE: u32 = 288;

#[derive(Debug, Clone)]
pub struct BrainHeader {
    // Magic and version
    pub magic: [u8; 4],
    pub format_version: u32,
    pub header_size: u32,
    pub flags: u32,

    // File size
    pub total_size: u64,

    // Generation (transaction number - must be monotonic)
    pub generation: u64,

    // Section offsets and sizes (must be 4096-aligned)
    pub cortex_offset: u64,
    pub cortex_size: u64,
    pub cerebellum_offset: u64,
    pub cerebellum_size: u64,
    pub hippocampus_offset: u64,
    pub hippocampus_size: u64,

    // Index and metadata
    pub index_offset: u64,
    pub index_size: u64,
    pub metadata_offset: u64,
    pub metadata_size: u64,
    pub allocation_table_offset: u64,
    pub allocation_table_size: u64,

    // Section table
    pub section_table_offset: u64,
    pub section_table_count: u32,
    pub block_size: u32,

    // Checksum config
    pub checksum_algo: u8, // 0=BLAKE3, 1=CRC32C
    pub checksum_scope: u8,

    // Checksums (calculated over entire superblock + sections)
    pub header_crc: u32,
    pub checksum: [u8; 32], // BLAKE3 or CRC32 result
}

impl BrainHeader {
    pub fn new() -> Self {
        Self {
            magic: *BRAIN_MAGIC,
            format_version: BRAIN_FORMAT_VERSION,
            header_size: BRAIN_HEADER_SIZE,
            flags: 0,
            total_size: 0,
            generation: 0,
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
            checksum_algo: 0, // BLAKE3
            checksum_scope: 0,
            header_crc: 0,
            checksum: [0u8; 32],
        }
    }

    /// Read header from file path
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

    /// Write header to file path
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

    /// Serialize header to binary format
    /// AC §48: Header layout must be strict binary format
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BRAIN_HEADER_SIZE as usize);

        // Magic (4)
        buf.extend_from_slice(&self.magic);

        // Format version (4)
        buf.extend_from_slice(&self.format_version.to_le_bytes());

        // Header size (4)
        buf.extend_from_slice(&self.header_size.to_le_bytes());

        // Flags (4)
        buf.extend_from_slice(&self.flags.to_le_bytes());

        // Total size (8)
        buf.extend_from_slice(&self.total_size.to_le_bytes());

        // Generation (8)
        buf.extend_from_slice(&self.generation.to_le_bytes());

        // Cortex (8+8)
        buf.extend_from_slice(&self.cortex_offset.to_le_bytes());
        buf.extend_from_slice(&self.cortex_size.to_le_bytes());

        // Cerebellum (8+8)
        buf.extend_from_slice(&self.cerebellum_offset.to_le_bytes());
        buf.extend_from_slice(&self.cerebellum_size.to_le_bytes());

        // Hippocampus (8+8)
        buf.extend_from_slice(&self.hippocampus_offset.to_le_bytes());
        buf.extend_from_slice(&self.hippocampus_size.to_le_bytes());

        // Index (8+8)
        buf.extend_from_slice(&self.index_offset.to_le_bytes());
        buf.extend_from_slice(&self.index_size.to_le_bytes());

        // Metadata (8+8)
        buf.extend_from_slice(&self.metadata_offset.to_le_bytes());
        buf.extend_from_slice(&self.metadata_size.to_le_bytes());

        // Allocation table (8+8)
        buf.extend_from_slice(&self.allocation_table_offset.to_le_bytes());
        buf.extend_from_slice(&self.allocation_table_size.to_le_bytes());

        // Section table (8+4)
        buf.extend_from_slice(&self.section_table_offset.to_le_bytes());
        buf.extend_from_slice(&self.section_table_count.to_le_bytes());

        // Block size (4)
        buf.extend_from_slice(&self.block_size.to_le_bytes());

        // Checksum algo and scope (1+1)
        buf.push(self.checksum_algo);
        buf.push(self.checksum_scope);

        // Reserved (102)
        buf.extend_from_slice(&[0u8; 102]);

        // Header CRC (4)
        buf.extend_from_slice(&self.header_crc.to_le_bytes());

        // Reserved (4)
        buf.extend_from_slice(&[0u8; 4]);

        // Checksum (32)
        buf.extend_from_slice(&self.checksum);

        // Pad to header_size
        while buf.len() < BRAIN_HEADER_SIZE as usize {
            buf.push(0);
        }

        buf
    }

    /// Deserialize header from binary
    pub fn deserialize(buf: &[u8]) -> Result<Self> {
        if buf.len() < BRAIN_HEADER_SIZE as usize {
            return Err(Error::BrainValidation("Header too short".to_string()));
        }

        let mut pos = 0;

        // Magic
        let magic = [buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]];
        if magic != *BRAIN_MAGIC {
            return Err(Error::BrainValidation("Invalid magic number".to_string()));
        }
        pos += 4;

        // Version
        let format_version =
            u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        if format_version != BRAIN_FORMAT_VERSION {
            return Err(Error::BrainValidation(format!(
                "Unsupported format version: {}",
                format_version
            )));
        }
        pos += 4;

        // Parse remaining fields
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

        // Parse section offsets
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

        // Skip reserved (102 bytes)
        pos += 102;

        // Header CRC
        let header_crc = u32::from_le_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4 + 4; // Skip 4 bytes of reserved

        // Checksum
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

    /// Validate header integrity
    pub fn validate(&self) -> Result<()> {
        // Magic
        if self.magic != *BRAIN_MAGIC {
            return Err(Error::BrainValidation("Invalid magic".to_string()));
        }

        // Version
        if self.format_version != BRAIN_FORMAT_VERSION {
            return Err(Error::BrainValidation(
                "Unsupported format version".to_string(),
            ));
        }

        // Header size
        if self.header_size != BRAIN_HEADER_SIZE {
            return Err(Error::BrainValidation("Invalid header size".to_string()));
        }

        // Block size
        if self.block_size != BRAIN_BLOCK_SIZE {
            return Err(Error::BrainValidation("Invalid block size".to_string()));
        }

        // Offsets must be 4096-aligned for main sections
        if self.cortex_offset > 0 && !self.cortex_offset.is_multiple_of(BRAIN_BLOCK_SIZE as u64) {
            return Err(Error::BrainValidation(
                "Cortex offset not aligned".to_string(),
            ));
        }
        if self.cerebellum_offset > 0
            && !self
                .cerebellum_offset
                .is_multiple_of(BRAIN_BLOCK_SIZE as u64)
        {
            return Err(Error::BrainValidation(
                "Cerebellum offset not aligned".to_string(),
            ));
        }
        if self.hippocampus_offset > 0
            && !self
                .hippocampus_offset
                .is_multiple_of(BRAIN_BLOCK_SIZE as u64)
        {
            return Err(Error::BrainValidation(
                "Hippocampus offset not aligned".to_string(),
            ));
        }

        // Section counts
        if self.section_table_count != 3 {
            return Err(Error::BrainValidation(
                "Invalid section table count".to_string(),
            ));
        }

        Ok(())
    }

    /// Generate BLAKE3 checksum for header
    pub fn compute_checksum(&mut self) -> [u8; 32] {
        let data = self.serialize();
        // Only checksum up to the checksum field (256 bytes)
        let hash = blake3::hash(&data[0..256]);
        let result = *hash.as_bytes();
        self.checksum = result;
        result
    }
}

impl Default for BrainHeader {
    fn default() -> Self {
        Self::new()
    }
}
