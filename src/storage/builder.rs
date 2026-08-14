/// Brain builder from seed
use crate::Result;
use std::path::Path;

pub struct BrainBuilder;

impl BrainBuilder {
    pub fn build_from_seed(seed_path: &Path, output_path: &Path) -> Result<()> {
        if !seed_path.exists() {
            return Err(crate::Error::BrainError("Seed file not found".to_string()));
        }

        // Create new brain.anr from seed — no sections allocated yet
        let mut header = super::header::BrainHeader::new();
        header.cortex_offset = 0;
        header.compute_checksum();
        header.write(output_path)?;

        Ok(())
    }
}
