use crate::brain::hippocampus::Episode;

/// Consolidation decision (AC §35.3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsolidationDecision {
    Keep,
    ConsolidateToCortex,
    ConsolidateToCerebellum,
    Compress,
    Delete,
}

/// Basic consolidation engine (detailed implementation in WP-11)
pub struct Consolidation {
    knowledge_confidence_min: f32,
    skill_success_rate_min: f32,
}

impl Consolidation {
    pub fn new() -> Self {
        Self {
            knowledge_confidence_min: 0.7,
            skill_success_rate_min: 0.6,
        }
    }

    pub fn evaluate(
        &self,
        episode: &Episode,
        recurrence: u32,
        context_diversity: u32,
    ) -> ConsolidationDecision {
        if episode.reward < 0.1 && recurrence == 0 {
            return ConsolidationDecision::Delete;
        }

        if recurrence >= 3
            && context_diversity >= 2
            && episode.reward >= self.knowledge_confidence_min
        {
            return ConsolidationDecision::ConsolidateToCortex;
        }

        if episode.reward >= self.skill_success_rate_min && recurrence >= 2 {
            return ConsolidationDecision::ConsolidateToCerebellum;
        }

        if recurrence >= 5 {
            return ConsolidationDecision::Compress;
        }

        ConsolidationDecision::Keep
    }
}

impl Default for Consolidation {
    fn default() -> Self {
        Self::new()
    }
}
