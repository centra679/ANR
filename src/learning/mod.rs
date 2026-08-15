use crate::brain::hippocampus::Hippocampus;
use crate::error::Result;

pub mod consolidation;
pub mod hebbian;
pub mod replay;
pub mod temporal;

pub use consolidation::{Consolidation, ConsolidationDecision};
pub use hebbian::{Hebbian, SynapseUpdate};
pub use replay::{Replay, ReplayCandidate};
pub use temporal::{Temporal, TemporalAssociation};

pub struct Learning {
    pub hebbian: Hebbian,
    pub temporal: Temporal,
    pub replay: Replay,
    pub consolidation: Consolidation,
}

#[derive(Debug, Clone)]
pub struct LearningResult {
    pub synapse_updates: usize,
    pub temporal_associations: usize,
    pub episodes_replayed: usize,
    pub consolidation_decisions: usize,
}

impl Learning {
    pub fn new() -> Self {
        Self {
            hebbian: Hebbian::new(0.01),
            temporal: Temporal::new(0.1, 100),
            replay: Replay::new(64),
            consolidation: Consolidation::new(),
        }
    }

    pub fn cycle(&mut self, hippocampus: &Hippocampus) -> Result<LearningResult> {
        let mut synapse_updates: usize = 0;
        let mut temporal_associations: usize = 0;
        let mut episodes_replayed: usize = 0;
        let mut consolidation_decisions: usize = 0;

        let episode_count = hippocampus.episode_count();
        for i in 0..episode_count {
            let episode = hippocampus.get_episode_by_index(i).unwrap();

            let updates = self.hebbian.batch_update(&[(0, 1, 0.5, true, true)]);
            synapse_updates += updates.len();

            if i > 0 {
                let prev_episode = hippocampus.get_episode_by_index(i - 1).unwrap();
                if let Some(_assoc) = self.temporal.associate(
                    i as u32 - 1,
                    prev_episode.created_at,
                    i as u32,
                    episode.created_at,
                ) {
                    temporal_associations += 1;
                }
            }

            let score = self.replay.score_episode(episode, 0.5, 0.3, 1);
            self.replay.enqueue(episode.clone(), score);
            episodes_replayed += 1;

            let _decision = self.consolidation.evaluate(episode, 1, 1);
            consolidation_decisions += 1;
        }

        Ok(LearningResult {
            synapse_updates,
            temporal_associations,
            episodes_replayed,
            consolidation_decisions,
        })
    }
}

impl Default for Learning {
    fn default() -> Self {
        Self::new()
    }
}
