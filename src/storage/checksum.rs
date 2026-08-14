/// BLAKE3 Checksum for brain.anr
/// Implements: AC §48, SD-03 §3.4.1, SD-03 §3.6
use blake3::Hash;

/// Checksum scope for brain.anr sections
/// AC §48: checksum covers specified regions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumScope {
    /// Checksum covers entire file
    FullFile = 0,
    /// Checksum covers header only
    HeaderOnly = 1,
    /// Checksum covers header + section data
    HeaderAndSections = 2,
}

impl ChecksumScope {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ChecksumScope::FullFile),
            1 => Some(ChecksumScope::HeaderOnly),
            2 => Some(ChecksumScope::HeaderAndSections),
            _ => None,
        }
    }
}

/// Compute BLAKE3 checksum over data
pub fn compute_blake3(data: &[u8]) -> Hash {
    blake3::hash(data)
}

/// Verify BLAKE3 checksum
pub fn verify_blake3(data: &[u8], expected: &[u8; 32]) -> bool {
    let hash = compute_blake3(data);
    hash.as_bytes() == expected
}

/// Compute checksum for brain.anr header region
/// SD-03: checksum covers header up to checksum field (256 bytes)
pub fn compute_header_checksum(header_bytes: &[u8]) -> [u8; 32] {
    let data = if header_bytes.len() >= 256 {
        &header_bytes[0..256]
    } else {
        header_bytes
    };
    let hash = blake3::hash(data);
    *hash.as_bytes()
}

/// Verify header checksum
pub fn verify_header_checksum(header_bytes: &[u8], expected: &[u8; 32]) -> bool {
    let computed = compute_header_checksum(header_bytes);
    &computed == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_blake3_deterministic() {
        let data = b"hello brain.anr";
        let h1 = compute_blake3(data);
        let h2 = compute_blake3(data);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_verify_blake3_correct() {
        let data = b"test data for checksum";
        let hash = compute_blake3(data);
        assert!(verify_blake3(data, hash.as_bytes()));
    }

    #[test]
    fn test_verify_blake3_wrong() {
        let data = b"test data";
        let wrong = [0u8; 32];
        assert!(!verify_blake3(data, &wrong));
    }

    #[test]
    fn test_compute_header_checksum_length() {
        let header = vec![0u8; 288];
        let hash = compute_header_checksum(&header);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_verify_header_checksum_valid() {
        let header = vec![0u8; 288];
        let hash = compute_header_checksum(&header);
        assert!(verify_header_checksum(&header, &hash));
    }

    #[test]
    fn test_checksum_scope_from_u8() {
        assert_eq!(ChecksumScope::from_u8(0), Some(ChecksumScope::FullFile));
        assert_eq!(ChecksumScope::from_u8(1), Some(ChecksumScope::HeaderOnly));
        assert_eq!(
            ChecksumScope::from_u8(2),
            Some(ChecksumScope::HeaderAndSections)
        );
        assert_eq!(ChecksumScope::from_u8(99), None);
    }

    #[test]
    fn test_blake3_different_data_different_hash() {
        let h1 = compute_blake3(b"data1");
        let h2 = compute_blake3(b"data2");
        assert_ne!(h1, h2);
    }
}
