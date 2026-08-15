use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataOrigin {
    Seed = 0,
    Learned = 1,
    Consolidated = 2,
    Imported = 3,
}

impl DataOrigin {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(DataOrigin::Seed),
            1 => Some(DataOrigin::Learned),
            2 => Some(DataOrigin::Consolidated),
            3 => Some(DataOrigin::Imported),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub id: String,
    pub pattern: String,
    pub confidence: f32,
    pub origin: DataOrigin,
    pub created_at: u64,
}

pub struct Cortex {
    knowledge: Vec<Knowledge>,
    max_capacity: usize,
}

impl Cortex {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            knowledge: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_knowledge(&mut self, k: Knowledge) -> Result<()> {
        if self.knowledge.len() >= self.max_capacity {
            return Err(Error::MemoryQuotaExceeded {
                section: "cortex".to_string(),
                used: self.knowledge.len() as u64,
                max: self.max_capacity as u64,
            });
        }
        self.knowledge.push(k);
        Ok(())
    }

    pub fn get_knowledge(&self, id: &str) -> Option<&Knowledge> {
        self.knowledge.iter().find(|k| k.id == id)
    }

    pub fn knowledge_count(&self) -> usize {
        self.knowledge.len()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn is_full(&self) -> bool {
        self.knowledge.len() >= self.max_capacity
    }

    pub fn query_by_pattern(&self, pattern: &str) -> Vec<&Knowledge> {
        self.knowledge
            .iter()
            .filter(|k| k.pattern == pattern)
            .collect()
    }
}

impl Default for Cortex {
    fn default() -> Self {
        Self::new(1024)
    }
}
