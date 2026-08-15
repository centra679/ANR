pub mod cerebellum;
/// Brain Module - Three Memory Subsystems
/// Implements: AC §7 (Cortex), AC §8 (Cerebellum), AC §9 (Hippocampus)
pub mod cortex;
pub mod hippocampus;

pub use cerebellum::{Cerebellum, Skill};
pub use cortex::{Cortex, DataOrigin, Knowledge};
pub use hippocampus::{Episode, Hippocampus};

pub struct Brain {
    pub cortex: Cortex,
    pub cerebellum: Cerebellum,
    pub hippocampus: Hippocampus,
}

impl Brain {
    pub fn new(cortex_cap: usize, cerebellum_cap: usize, hippocampus_cap: usize) -> Self {
        Self {
            cortex: Cortex::new(cortex_cap),
            cerebellum: Cerebellum::new(cerebellum_cap),
            hippocampus: Hippocampus::new(hippocampus_cap),
        }
    }

    pub fn total_objects(&self) -> usize {
        self.cortex.knowledge_count()
            + self.cerebellum.skill_count()
            + self.hippocampus.episode_count()
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new(1024, 1024, 1024)
    }
}
