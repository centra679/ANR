use super::cortex::DataOrigin;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: String,
    pub action: String,
    pub validated: bool,
    pub origin: DataOrigin,
    pub created_at: u64,
}

pub struct Cerebellum {
    skills: Vec<Skill>,
    max_capacity: usize,
}

impl Cerebellum {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            skills: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_skill(&mut self, s: Skill) -> Result<()> {
        if self.skills.len() >= self.max_capacity {
            return Err(Error::MemoryQuotaExceeded {
                section: "cerebellum".to_string(),
                used: self.skills.len() as u64,
                max: self.max_capacity as u64,
            });
        }
        self.skills.push(s);
        Ok(())
    }

    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.id == id)
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn is_full(&self) -> bool {
        self.skills.len() >= self.max_capacity
    }

    pub fn validated_skills(&self) -> Vec<&Skill> {
        self.skills.iter().filter(|s| s.validated).collect()
    }
}

impl Default for Cerebellum {
    fn default() -> Self {
        Self::new(1024)
    }
}
