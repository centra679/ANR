/// brain.anr Validation Rules
/// Implements: AC §44, SD-03 §3.5
use crate::error::{Error, Result};
use crate::storage::{BrainHeader, BLOCK_SIZE};

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub rule: &'static str,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.rule, self.message)
    }
}

/// Validate brain.anr header against all AC §44 rules
pub fn validate_header(header: &BrainHeader) -> Result<()> {
    validate_magic(header)?;
    validate_version(header)?;
    validate_header_size(header)?;
    validate_block_size(header)?;
    validate_total_size(header)?;
    validate_generation(header)?;
    validate_section_offsets(header)?;
    validate_section_sizes(header)?;
    validate_section_boundaries(header)?;
    validate_section_table(header)?;
    validate_checksum(header)?;
    Ok(())
}

/// AC §44.1: Magic number must be ANRB
pub fn validate_magic(header: &BrainHeader) -> Result<()> {
    if header.magic != *b"ANRB" {
        return Err(Error::StorageHeaderCorrupt(
            "Invalid magic number".to_string(),
        ));
    }
    Ok(())
}

/// AC §44.2: Format version must be 1
pub fn validate_version(header: &BrainHeader) -> Result<()> {
    if header.format_version != 1 {
        return Err(Error::StorageHeaderCorrupt(format!(
            "Unsupported format version: {}",
            header.format_version
        )));
    }
    Ok(())
}

/// AC §44.3: Header size must be 288
pub fn validate_header_size(header: &BrainHeader) -> Result<()> {
    if header.header_size != 288 {
        return Err(Error::StorageHeaderCorrupt(format!(
            "Invalid header size: {}",
            header.header_size
        )));
    }
    Ok(())
}

/// AC §44.4: Block size must be 4096
pub fn validate_block_size(header: &BrainHeader) -> Result<()> {
    if header.block_size != 4096 {
        return Err(Error::StorageHeaderCorrupt(format!(
            "Invalid block size: {}",
            header.block_size
        )));
    }
    Ok(())
}

/// AC §44.5: Total size must be >= header size
pub fn validate_total_size(header: &BrainHeader) -> Result<()> {
    if header.total_size < header.header_size as u64 {
        return Err(Error::StorageHeaderCorrupt(format!(
            "Total size {} less than header size {}",
            header.total_size, header.header_size
        )));
    }
    Ok(())
}

/// AC §44.6: Generation must be > 0
pub fn validate_generation(header: &BrainHeader) -> Result<()> {
    if header.generation == 0 {
        return Err(Error::StorageCorruptGeneration);
    }
    Ok(())
}

/// AC §44.7: Section offsets must be 4096-aligned
pub fn validate_section_offsets(header: &BrainHeader) -> Result<()> {
    let offsets = [
        ("cortex", header.cortex_offset),
        ("cerebellum", header.cerebellum_offset),
        ("hippocampus", header.hippocampus_offset),
    ];
    for (name, offset) in offsets {
        if offset > 0 && !offset.is_multiple_of(BLOCK_SIZE) {
            return Err(Error::StorageHeaderCorrupt(format!(
                "{} offset not 4096-aligned: {}",
                name, offset
            )));
        }
    }
    Ok(())
}

/// AC §44.8: Section sizes must be > 0 if offset > 0
pub fn validate_section_sizes(header: &BrainHeader) -> Result<()> {
    let sections = [
        ("cortex", header.cortex_offset, header.cortex_size),
        (
            "cerebellum",
            header.cerebellum_offset,
            header.cerebellum_size,
        ),
        (
            "hippocampus",
            header.hippocampus_offset,
            header.hippocampus_size,
        ),
    ];
    for (name, offset, size) in sections {
        if offset > 0 && size == 0 {
            return Err(Error::StorageHeaderCorrupt(format!(
                "{} has offset but zero size",
                name
            )));
        }
    }
    Ok(())
}

/// AC §44.9: Sections must not overlap and must fit in file
pub fn validate_section_boundaries(header: &BrainHeader) -> Result<()> {
    let sections = [
        ("cortex", header.cortex_offset, header.cortex_size),
        (
            "cerebellum",
            header.cerebellum_offset,
            header.cerebellum_size,
        ),
        (
            "hippocampus",
            header.hippocampus_offset,
            header.hippocampus_size,
        ),
    ];

    for (name, offset, size) in &sections {
        if *offset > 0 && *size > 0 {
            let end = offset + size;
            if end > header.total_size {
                return Err(Error::StorageHeaderCorrupt(format!(
                    "{} extends beyond file size",
                    name
                )));
            }
        }
    }

    for i in 0..sections.len() {
        for j in (i + 1)..sections.len() {
            let (n1, o1, s1) = sections[i];
            let (n2, o2, s2) = sections[j];
            if o1 > 0 && s1 > 0 && o2 > 0 && s2 > 0 {
                let e1 = o1 + s1;
                let e2 = o2 + s2;
                if o1 < e2 && o2 < e1 {
                    return Err(Error::StorageHeaderCorrupt(format!(
                        "{} and {} overlap",
                        n1, n2
                    )));
                }
            }
        }
    }

    Ok(())
}

/// AC §44.10: Section table must have 3 entries
pub fn validate_section_table(header: &BrainHeader) -> Result<()> {
    if header.section_table_count != 3 {
        return Err(Error::StorageHeaderCorrupt(format!(
            "Invalid section table count: {}",
            header.section_table_count
        )));
    }
    Ok(())
}

/// AC §44.11: Checksum must be valid
pub fn validate_checksum(header: &BrainHeader) -> Result<()> {
    let header_bytes = header.serialize();
    let computed = crate::storage::checksum::compute_header_checksum(&header_bytes);
    if computed != header.checksum {
        return Err(Error::StorageChecksumMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_magic_valid() {
        let mut header = BrainHeader::new();
        header.magic = *b"ANRB";
        assert!(validate_magic(&header).is_ok());
    }

    #[test]
    fn test_validate_magic_invalid() {
        let mut header = BrainHeader::new();
        header.magic = *b"XXXX";
        assert!(validate_magic(&header).is_err());
    }

    #[test]
    fn test_validate_version_valid() {
        let header = BrainHeader::new();
        assert!(validate_version(&header).is_ok());
    }

    #[test]
    fn test_validate_version_invalid() {
        let mut header = BrainHeader::new();
        header.format_version = 99;
        assert!(validate_version(&header).is_err());
    }

    #[test]
    fn test_validate_header_size_valid() {
        let header = BrainHeader::new();
        assert!(validate_header_size(&header).is_ok());
    }

    #[test]
    fn test_validate_header_size_invalid() {
        let mut header = BrainHeader::new();
        header.header_size = 100;
        assert!(validate_header_size(&header).is_err());
    }

    #[test]
    fn test_validate_generation_zero() {
        let mut header = BrainHeader::new();
        header.generation = 0;
        assert!(validate_generation(&header).is_err());
    }

    #[test]
    fn test_validate_generation_nonzero() {
        let mut header = BrainHeader::new();
        header.generation = 1;
        assert!(validate_generation(&header).is_ok());
    }

    #[test]
    fn test_validate_section_offsets_aligned() {
        let mut header = BrainHeader::new();
        header.cortex_offset = 4096;
        header.cerebellum_offset = 8192;
        header.hippocampus_offset = 12288;
        assert!(validate_section_offsets(&header).is_ok());
    }

    #[test]
    fn test_validate_section_offsets_unaligned() {
        let mut header = BrainHeader::new();
        header.cortex_offset = 4097;
        assert!(validate_section_offsets(&header).is_err());
    }

    #[test]
    fn test_validate_section_sizes_zero_with_offset() {
        let mut header = BrainHeader::new();
        header.cortex_offset = 4096;
        header.cortex_size = 0;
        assert!(validate_section_sizes(&header).is_err());
    }

    #[test]
    fn test_validate_section_boundaries_overlap() {
        let mut header = BrainHeader::new();
        header.total_size = 65536;
        header.cortex_offset = 4096;
        header.cortex_size = 8192;
        header.cerebellum_offset = 8192;
        header.cerebellum_size = 4096;
        assert!(validate_section_boundaries(&header).is_err());
    }

    #[test]
    fn test_validate_checksum_mismatch() {
        let header = BrainHeader::new();
        assert!(validate_checksum(&header).is_err());
    }
}
