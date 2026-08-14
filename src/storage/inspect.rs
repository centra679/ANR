/// brain.anr Inspection and Dump
/// Implements: AC §44, CLI `anr brain inspect`
use crate::error::{Error, Result};
use crate::storage::BrainHeader;
use serde::Serialize;
use std::fmt;

/// Inspection output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectFormat {
    Text,
    Json,
}

impl fmt::Display for InspectFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InspectFormat::Text => write!(f, "text"),
            InspectFormat::Json => write!(f, "json"),
        }
    }
}

/// Serializable header for JSON output
#[derive(Serialize)]
pub struct HeaderJson {
    pub magic: String,
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
    pub checksum: String,
}

impl From<&BrainHeader> for HeaderJson {
    fn from(h: &BrainHeader) -> Self {
        Self {
            magic: String::from_utf8_lossy(&h.magic).into_owned(),
            format_version: h.format_version,
            header_size: h.header_size,
            flags: h.flags,
            total_size: h.total_size,
            generation: h.generation,
            cortex_offset: h.cortex_offset,
            cortex_size: h.cortex_size,
            cerebellum_offset: h.cerebellum_offset,
            cerebellum_size: h.cerebellum_size,
            hippocampus_offset: h.hippocampus_offset,
            hippocampus_size: h.hippocampus_size,
            index_offset: h.index_offset,
            index_size: h.index_size,
            metadata_offset: h.metadata_offset,
            metadata_size: h.metadata_size,
            allocation_table_offset: h.allocation_table_offset,
            allocation_table_size: h.allocation_table_size,
            section_table_offset: h.section_table_offset,
            section_table_count: h.section_table_count,
            block_size: h.block_size,
            checksum_algo: h.checksum_algo,
            checksum_scope: h.checksum_scope,
            header_crc: h.header_crc,
            checksum: hex_encode(&h.checksum),
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Dump brain.anr header in text format
pub fn dump_header_text(header: &BrainHeader) -> String {
    let mut out = String::new();
    out.push_str("=== brain.anr Header ===\n");
    out.push_str(&format!(
        "Magic: {}\n",
        String::from_utf8_lossy(&header.magic)
    ));
    out.push_str(&format!("Format Version: {}\n", header.format_version));
    out.push_str(&format!("Header Size: {}\n", header.header_size));
    out.push_str(&format!("Flags: {:#010x}\n", header.flags));
    out.push_str(&format!("Total Size: {}\n", header.total_size));
    out.push_str(&format!("Generation: {}\n", header.generation));
    out.push_str(&format!("Block Size: {}\n", header.block_size));
    out.push_str("\n--- Sections ---\n");
    out.push_str(&format!(
        "Cortex: offset={}, size={}\n",
        header.cortex_offset, header.cortex_size
    ));
    out.push_str(&format!(
        "Cerebellum: offset={}, size={}\n",
        header.cerebellum_offset, header.cerebellum_size
    ));
    out.push_str(&format!(
        "Hippocampus: offset={}, size={}\n",
        header.hippocampus_offset, header.hippocampus_size
    ));
    out.push_str("\n--- Index ---\n");
    out.push_str(&format!(
        "Index: offset={}, size={}\n",
        header.index_offset, header.index_size
    ));
    out.push_str(&format!(
        "Metadata: offset={}, size={}\n",
        header.metadata_offset, header.metadata_size
    ));
    out.push_str(&format!(
        "Allocation Table: offset={}, size={}\n",
        header.allocation_table_offset, header.allocation_table_size
    ));
    out.push_str("\n--- Checksum ---\n");
    out.push_str(&format!(
        "Algorithm: {}\n",
        match header.checksum_algo {
            0 => "BLAKE3",
            1 => "CRC32C",
            _ => "Unknown",
        }
    ));
    out.push_str(&format!("Scope: {}\n", header.checksum_scope));
    out.push_str(&format!("Checksum: {}\n", hex_encode(&header.checksum)));
    out
}

/// Dump brain.anr header in JSON format
pub fn dump_header_json(header: &BrainHeader) -> Result<String> {
    let json = HeaderJson::from(header);
    serde_json::to_string_pretty(&json)
        .map_err(|e| Error::InternalSerializationError(e.to_string()))
}

/// Inspect brain.anr file
pub fn inspect_brain(path: &std::path::Path, format: InspectFormat) -> Result<String> {
    let header = BrainHeader::read(path)?;
    match format {
        InspectFormat::Text => Ok(dump_header_text(&header)),
        InspectFormat::Json => dump_header_json(&header),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_header_text_contains_magic() {
        let header = BrainHeader::new();
        let text = dump_header_text(&header);
        assert!(text.contains("ANRB"));
    }

    #[test]
    fn test_dump_header_text_contains_sections() {
        let header = BrainHeader::new();
        let text = dump_header_text(&header);
        assert!(text.contains("Cortex"));
        assert!(text.contains("Cerebellum"));
        assert!(text.contains("Hippocampus"));
    }

    #[test]
    fn test_dump_header_json_valid() {
        let header = BrainHeader::new();
        let json = dump_header_json(&header).unwrap();
        assert!(json.contains("ANRB"));
    }

    #[test]
    fn test_inspect_format_display() {
        assert_eq!(InspectFormat::Text.to_string(), "text");
        assert_eq!(InspectFormat::Json.to_string(), "json");
    }

    #[test]
    fn test_header_json_serialization() {
        let header = BrainHeader::new();
        let json = dump_header_json(&header).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["magic"], "ANRB");
    }
}
