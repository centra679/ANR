use crate::error::{Error, Result};
use serde::Deserialize;
use std::path::Path;

const RECORD_MAGIC: &[u8; 4] = b"ANRR";

#[derive(Deserialize)]
pub struct BrainSeed {
    pub meta: SeedMeta,
    pub cortex: Option<SeedCortex>,
    pub cerebellum: Option<SeedCerebellum>,
    pub hippocampus: Option<SeedHippocampus>,
}

#[derive(Deserialize)]
pub struct SeedMeta {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct SeedCortex {
    pub knowledge: SeedKnowledgeList,
}

#[derive(Deserialize)]
pub struct SeedKnowledgeList {
    pub items: Vec<SeedKnowledge>,
}

#[derive(Deserialize)]
pub struct SeedKnowledge {
    pub id: String,
    pub pattern: String,
    pub confidence: f32,
}

#[derive(Deserialize)]
pub struct SeedCerebellum {
    pub skills: SeedSkillList,
}

#[derive(Deserialize)]
pub struct SeedSkillList {
    pub items: Vec<SeedSkill>,
}

#[derive(Deserialize)]
pub struct SeedSkill {
    pub id: String,
    pub action: String,
    pub validated: bool,
}

#[derive(Deserialize)]
pub struct SeedHippocampus {
    pub episodes: SeedEpisodeList,
}

#[derive(Deserialize)]
pub struct SeedEpisodeList {
    pub items: Vec<SeedEpisode>,
}

#[derive(Deserialize)]
pub struct SeedEpisode {
    pub id: String,
    pub context: String,
    pub action: String,
    pub reward: f32,
}

pub struct BrainBuilder;

impl BrainBuilder {
    pub fn build_from_seed(seed_path: &Path, output_path: &Path) -> Result<()> {
        let seed = Self::parse_seed(seed_path)?;
        Self::validate_seed(&seed)?;

        let mut cortex_records = Vec::new();
        let mut cerebellum_records = Vec::new();
        let mut hippocampus_records = Vec::new();

        if let Some(cortex) = &seed.cortex {
            for k in &cortex.knowledge.items {
                cortex_records.extend(Self::build_knowledge_record(k));
            }
        }
        if let Some(cerebellum) = &seed.cerebellum {
            for s in &cerebellum.skills.items {
                cerebellum_records.extend(Self::build_skill_record(s));
            }
        }
        if let Some(hippocampus) = &seed.hippocampus {
            for e in &hippocampus.episodes.items {
                hippocampus_records.extend(Self::build_episode_record(e));
            }
        }

        let block_size = super::BLOCK_SIZE;

        let has_records = !cortex_records.is_empty()
            || !cerebellum_records.is_empty()
            || !hippocampus_records.is_empty();

        let mut current_offset = block_size;

        let cortex_offset = if !cortex_records.is_empty() {
            let off = current_offset;
            current_offset += cortex_records.len() as u64;
            off
        } else {
            0
        };

        let cerebellum_offset = if !cerebellum_records.is_empty() {
            let off = current_offset;
            current_offset += cerebellum_records.len() as u64;
            off
        } else {
            0
        };

        let hippocampus_offset = if !hippocampus_records.is_empty() {
            let off = current_offset;
            current_offset += hippocampus_records.len() as u64;
            off
        } else {
            0
        };

        let header_size = super::header::BrainHeader::new().header_size as u64;

        let total_size = if has_records {
            current_offset
        } else {
            header_size
        };

        let mut header = super::header::BrainHeader::new();
        header.total_size = total_size;
        header.cortex_offset = cortex_offset;
        header.cortex_size = cortex_records.len() as u64;
        header.cerebellum_offset = cerebellum_offset;
        header.cerebellum_size = cerebellum_records.len() as u64;
        header.hippocampus_offset = hippocampus_offset;
        header.hippocampus_size = hippocampus_records.len() as u64;
        header.compute_checksum();
        header.write(output_path)?;

        if has_records {
            use std::fs::OpenOptions;
            use std::io::{Seek, SeekFrom, Write};

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(output_path)
                .map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot open output for records: {}", e))
                })?;

            if !cortex_records.is_empty() {
                file.seek(SeekFrom::Start(cortex_offset)).map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot seek to cortex offset: {}", e))
                })?;
                file.write_all(&cortex_records).map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot write cortex records: {}", e))
                })?;
            }

            if !cerebellum_records.is_empty() {
                file.seek(SeekFrom::Start(cerebellum_offset)).map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot seek to cerebellum offset: {}", e))
                })?;
                file.write_all(&cerebellum_records).map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot write cerebellum records: {}", e))
                })?;
            }

            if !hippocampus_records.is_empty() {
                file.seek(SeekFrom::Start(hippocampus_offset))
                    .map_err(|e| {
                        Error::StorageWriteFailed(format!(
                            "Cannot seek to hippocampus offset: {}",
                            e
                        ))
                    })?;
                file.write_all(&hippocampus_records).map_err(|e| {
                    Error::StorageWriteFailed(format!("Cannot write hippocampus records: {}", e))
                })?;
            }

            file.sync_all().map_err(|e| {
                Error::StorageFsyncFailed(format!("fsync after records write failed: {}", e))
            })?;
        }

        Ok(())
    }

    pub fn parse_seed(seed_path: &Path) -> Result<BrainSeed> {
        let content = std::fs::read_to_string(seed_path).map_err(|_| {
            Error::BrainError(format!("Seed file not found: {}", seed_path.display()))
        })?;

        toml::from_str::<BrainSeed>(&content)
            .map_err(|e| Error::BrainError(format!("Malformed TOML seed: {}", e)))
    }

    pub fn validate_seed(seed: &BrainSeed) -> Result<()> {
        if seed.meta.name.is_empty() {
            return Err(Error::ValidationInvalid(
                "Seed meta name is empty".to_string(),
            ));
        }
        if seed.meta.version.is_empty() {
            return Err(Error::ValidationInvalid(
                "Seed meta version is empty".to_string(),
            ));
        }

        if let Some(cortex) = &seed.cortex {
            for k in &cortex.knowledge.items {
                if k.id.is_empty() {
                    return Err(Error::ValidationInvalid(
                        "Knowledge id is empty".to_string(),
                    ));
                }
                if k.confidence < 0.0 || k.confidence > 1.0 {
                    return Err(Error::ValidationInvalid(format!(
                        "Knowledge confidence {} out of range [0,1]",
                        k.confidence
                    )));
                }
            }
        }

        if let Some(cerebellum) = &seed.cerebellum {
            for s in &cerebellum.skills.items {
                if s.id.is_empty() {
                    return Err(Error::ValidationInvalid("Skill id is empty".to_string()));
                }
            }
        }

        if let Some(hippocampus) = &seed.hippocampus {
            for e in &hippocampus.episodes.items {
                if e.id.is_empty() {
                    return Err(Error::ValidationInvalid("Episode id is empty".to_string()));
                }
                if e.reward < 0.0 || e.reward > 1.0 {
                    return Err(Error::ValidationInvalid(format!(
                        "Episode reward {} out of range [0,1]",
                        e.reward
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn build_knowledge_record(k: &SeedKnowledge) -> Vec<u8> {
        let id_bytes = k.id.as_bytes();
        let pattern_bytes = k.pattern.as_bytes();

        let mut payload = Vec::new();
        payload.extend_from_slice(&(id_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(id_bytes);
        payload.extend_from_slice(&(pattern_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(pattern_bytes);
        payload.extend_from_slice(&k.confidence.to_le_bytes());

        let payload_len = payload.len() as u32;
        let mut buf = Vec::new();

        buf.extend_from_slice(RECORD_MAGIC);
        buf.extend_from_slice(&0x0100u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&payload_len.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.push(0);
        buf.push(0);
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&payload);

        let hash = blake3::hash(&buf);
        buf.extend_from_slice(hash.as_bytes());

        buf
    }

    pub fn build_skill_record(s: &SeedSkill) -> Vec<u8> {
        let id_bytes = s.id.as_bytes();
        let action_bytes = s.action.as_bytes();

        let mut payload = Vec::new();
        payload.extend_from_slice(&(id_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(id_bytes);
        payload.extend_from_slice(&(action_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(action_bytes);
        payload.push(u8::from(s.validated));

        let payload_len = payload.len() as u32;
        let mut buf = Vec::new();

        buf.extend_from_slice(RECORD_MAGIC);
        buf.extend_from_slice(&0x0200u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&payload_len.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.push(0);
        buf.push(0);
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&payload);

        let hash = blake3::hash(&buf);
        buf.extend_from_slice(hash.as_bytes());

        buf
    }

    pub fn build_episode_record(e: &SeedEpisode) -> Vec<u8> {
        let id_bytes = e.id.as_bytes();
        let context_bytes = e.context.as_bytes();
        let action_bytes = e.action.as_bytes();

        let mut payload = Vec::new();
        payload.extend_from_slice(&(id_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(id_bytes);
        payload.extend_from_slice(&(context_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(context_bytes);
        payload.extend_from_slice(&(action_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(action_bytes);
        payload.extend_from_slice(&e.reward.to_le_bytes());

        let payload_len = payload.len() as u32;
        let mut buf = Vec::new();

        buf.extend_from_slice(RECORD_MAGIC);
        buf.extend_from_slice(&0x0300u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&payload_len.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.push(0);
        buf.push(0);
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&0u64.to_le_bytes());
        buf.extend_from_slice(&payload);

        let hash = blake3::hash(&buf);
        buf.extend_from_slice(hash.as_bytes());

        buf
    }
}
