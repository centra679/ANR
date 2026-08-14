/// Brain validation and verification
/// Implements: AC §44
use crate::Result;
use std::path::Path;

pub struct BrainValidator;

impl BrainValidator {
    pub fn verify_file(path: &Path) -> Result<bool> {
        if !path.exists() {
            return Err(crate::Error::BrainError("Brain file not found".to_string()));
        }

        // Read and validate header
        let header = super::header::BrainHeader::read(path)?;
        header.validate()?;

        Ok(true)
    }

    pub fn validate(path: &Path) -> Result<()> {
        Self::verify_file(path)?;
        Ok(())
    }
}
