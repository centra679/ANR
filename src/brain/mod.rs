/// Brain Module - Three Memory Subsystems
/// Implements: AC §7 (Cortex), AC §8 (Cerebellum), AC §9 (Hippocampus)

pub mod cortex;
pub mod cerebellum;
pub mod hippocampus;

pub use cortex::Cortex;
pub use cerebellum::Cerebellum;
pub use hippocampus::Hippocampus;

pub struct Brain {
    pub cortex: Cortex,
    pub cerebellum: Cerebellum,
    pub hippocampus: Hippocampus,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            cortex: Cortex::new(),
            cerebellum: Cerebellum::new(),
            hippocampus: Hippocampus::new(),
        }
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new()
    }
}
