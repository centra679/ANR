/// brain.anr Format specifications
/// Implements: AC §5, SD-03
use crate::storage::{BrainHeader, BLOCK_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    KnowledgePattern = 0x0100,
    SkillProcedure = 0x0200,
    Episode = 0x0300,
    CellPool = 0x0400,
    ColumnPool = 0x0500,
    BlockPool = 0x0600,
    SynapseTable = 0x0700,
    IndexEntry = 0x0800,
    Metadata = 0x0900,
}

impl RecordType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0100 => Some(RecordType::KnowledgePattern),
            0x0200 => Some(RecordType::SkillProcedure),
            0x0300 => Some(RecordType::Episode),
            0x0400 => Some(RecordType::CellPool),
            0x0500 => Some(RecordType::ColumnPool),
            0x0600 => Some(RecordType::BlockPool),
            0x0700 => Some(RecordType::SynapseTable),
            0x0800 => Some(RecordType::IndexEntry),
            0x0900 => Some(RecordType::Metadata),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Blake3 = 0,
    Crc32c = 1,
}

impl Default for ChecksumAlgorithm {
    fn default() -> Self {
        ChecksumAlgorithm::Blake3
    }
}

pub struct BrainFormat;

impl BrainFormat {
    pub fn default_block_size() -> u32 {
        BLOCK_SIZE as u32
    }

    pub fn default_header_size() -> u32 {
        288
    }

    pub fn magic() -> &'static [u8; 4] {
        b"ANRB"
    }

    pub fn format_version() -> u32 {
        1
    }

    pub fn section_count() -> u32 {
        3
    }
}
