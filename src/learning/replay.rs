use crate::brain::hippocampus::Episode;

/// Replay selection priority score (AC §34.2)
#[derive(Debug, Clone)]
pub struct ReplayCandidate {
    pub episode: Episode,
    pub score: f32,
}

/// Replay engine: selects episodes for replay (AC §34)
pub struct Replay {
    queue_capacity: usize,
    candidates: Vec<ReplayCandidate>,
}

impl Replay {
    pub fn new(queue_capacity: usize) -> Self {
        Self {
            queue_capacity,
            candidates: Vec::with_capacity(queue_capacity),
        }
    }

    pub fn score_episode(
        &self,
        episode: &Episode,
        prediction_error: f32,
        novelty: f32,
        recurrence: u32,
    ) -> f32 {
        let failure_score = if episode.reward < 0.3 { 1.0 } else { 0.0 };
        let recurrence_norm = (recurrence as f32 / 100.0).min(1.0);
        0.25 * prediction_error
            + 0.20 * novelty
            + 0.15 * episode.reward
            + 0.15 * failure_score
            + 0.15 * recurrence_norm
            + 0.10 * recurrence_norm
    }

    pub fn enqueue(&mut self, episode: Episode, score: f32) {
        if self.candidates.len() < self.queue_capacity {
            self.candidates.push(ReplayCandidate { episode, score });
        }
    }

    pub fn select(&self, k: usize) -> Vec<&ReplayCandidate> {
        let mut indices: Vec<usize> = (0..self.candidates.len()).collect();
        indices.sort_by(|&a, &b| {
            self.candidates[b]
                .score
                .partial_cmp(&self.candidates[a].score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        indices
            .into_iter()
            .take(k)
            .map(|i| &self.candidates[i])
            .collect()
    }

    pub fn acknowledge(&mut self, count: usize) {
        if count >= self.candidates.len() {
            self.candidates.clear();
            return;
        }
        let mut score_index_pairs: Vec<(usize, f32)> = self
            .candidates
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.score))
            .collect();
        score_index_pairs
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let remove_set: std::collections::HashSet<usize> = score_index_pairs
            .into_iter()
            .take(count)
            .map(|(i, _)| i)
            .collect();
        self.candidates = self
            .candidates
            .drain(..)
            .enumerate()
            .filter(|(i, _)| !remove_set.contains(i))
            .map(|(_, c)| c)
            .collect();
    }

    pub fn queue_len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}
