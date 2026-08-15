use super::cortex::DataOrigin;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Episode {
    pub id: String,
    pub context: String,
    pub action: String,
    pub reward: f32,
    pub origin: DataOrigin,
    pub created_at: u64,
}

const GC_REWARD_THRESHOLD: f32 = 0.3;

pub struct Hippocampus {
    episodes: Vec<Episode>,
    max_capacity: usize,
}

impl Hippocampus {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            episodes: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_episode(&mut self, e: Episode) -> Result<()> {
        if self.episodes.len() >= self.max_capacity {
            return Err(Error::MemoryQuotaExceeded {
                section: "hippocampus".to_string(),
                used: self.episodes.len() as u64,
                max: self.max_capacity as u64,
            });
        }
        self.episodes.push(e);
        Ok(())
    }

    pub fn get_episode(&self, id: &str) -> Option<&Episode> {
        self.episodes.iter().find(|e| e.id == id)
    }

    pub fn get_episode_by_index(&self, index: usize) -> Option<&Episode> {
        self.episodes.get(index)
    }

    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn is_full(&self) -> bool {
        self.episodes.len() >= self.max_capacity
    }

    pub fn gc_eligible(&self) -> Vec<&Episode> {
        self.episodes
            .iter()
            .filter(|e| e.reward < GC_REWARD_THRESHOLD)
            .collect()
    }
}

impl Default for Hippocampus {
    fn default() -> Self {
        Self::new(1024)
    }
}
