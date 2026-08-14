/// Storage Read Path Regression Tests
/// Implements: AC §44, SD-03 §3.4-3.7
/// Golden brain + 10 corrupt fixtures
#[cfg(test)]
mod tests {
    use crate::storage::BrainHeader;
    use std::path::PathBuf;

    fn fixtures_dir() -> PathBuf {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("tests/fixtures/brains");
        dir
    }

    #[test]
    fn test_golden_brain_valid() {
        let path = fixtures_dir().join("valid_golden.anr");
        assert!(path.exists(), "golden fixture missing: {}", path.display());
        let header = BrainHeader::read(&path).expect("read golden brain");
        assert!(
            crate::storage::validate_header(&header).is_ok(),
            "golden brain should be valid"
        );
    }

    #[test]
    fn test_golden_brain_checksum() {
        let path = fixtures_dir().join("valid_golden.anr");
        let header = BrainHeader::read(&path).expect("read golden brain");
        assert!(
            crate::storage::checksum::verify_header_checksum(&header.serialize(), &header.checksum),
            "golden checksum should match"
        );
    }

    #[test]
    fn test_corrupt_invalid_magic() {
        let path = fixtures_dir().join("corrupt/invalid_magic.anr");
        let result = BrainHeader::read(&path);
        assert!(result.is_err(), "invalid magic should fail at read time");
    }

    #[test]
    fn test_corrupt_wrong_version() {
        let path = fixtures_dir().join("corrupt/wrong_version.anr");
        let result = BrainHeader::read(&path);
        assert!(result.is_err(), "wrong version should fail at read time");
    }

    #[test]
    fn test_corrupt_wrong_header_size() {
        let path = fixtures_dir().join("corrupt/wrong_header_size.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_header_size(&header).is_err());
    }

    #[test]
    fn test_corrupt_unaligned_offset() {
        let path = fixtures_dir().join("corrupt/unaligned_offset.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_section_offsets(&header).is_err());
    }

    #[test]
    fn test_corrupt_section_overlap() {
        let path = fixtures_dir().join("corrupt/section_overlap.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_section_boundaries(&header).is_err());
    }

    #[test]
    fn test_corrupt_generation_zero() {
        let path = fixtures_dir().join("corrupt/generation_zero.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_generation(&header).is_err());
    }

    #[test]
    fn test_corrupt_checksum_mismatch() {
        let path = fixtures_dir().join("corrupt/checksum_mismatch.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_checksum(&header).is_err());
    }

    #[test]
    fn test_corrupt_size_too_small() {
        let path = fixtures_dir().join("corrupt/size_too_small.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_total_size(&header).is_err());
    }

    #[test]
    fn test_corrupt_invalid_section_count() {
        let path = fixtures_dir().join("corrupt/invalid_section_count.anr");
        let header = BrainHeader::read(&path).expect("read corrupt");
        assert!(crate::storage::validate::validate_section_table(&header).is_err());
    }

    #[test]
    fn test_corrupt_truncated() {
        let path = fixtures_dir().join("corrupt/truncated.anr");
        let result = BrainHeader::read(&path);
        assert!(result.is_err(), "truncated file should fail to read");
    }
}
