//! Sparse Graph - sparse neural topology
//! Implements: AC §15
use super::synapse::Synapse;

pub struct SparseGraph {
    synapse_ids: Vec<u32>,
    synapse_sources: Vec<u32>,
    synapse_targets: Vec<u32>,
    synapse_weights: Vec<f32>,
    synapse_permanence: Vec<f32>,
    adj_offsets: Vec<u32>,
    adj_targets: Vec<u32>,
}

impl SparseGraph {
    pub fn new() -> Self {
        Self {
            synapse_ids: Vec::new(),
            synapse_sources: Vec::new(),
            synapse_targets: Vec::new(),
            synapse_weights: Vec::new(),
            synapse_permanence: Vec::new(),
            adj_offsets: Vec::new(),
            adj_targets: Vec::new(),
        }
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        let source = synapse.source;
        self.synapse_ids.push(synapse.id);
        self.synapse_sources.push(source);
        self.synapse_targets.push(synapse.target);
        self.synapse_weights.push(synapse.weight);
        self.synapse_permanence.push(synapse.permanence);

        let synapse_idx = (self.synapse_ids.len() - 1) as u32;
        let source_usize = source as usize;
        while self.adj_offsets.len() <= source_usize {
            self.adj_offsets.push(self.adj_targets.len() as u32);
        }
        self.adj_targets.push(synapse_idx);
        self.adj_offsets.push(self.adj_targets.len() as u32);
    }

    pub fn get_outgoing(&self, source: u32) -> Vec<Synapse> {
        let source_usize = source as usize;
        if source_usize >= self.adj_offsets.len() {
            return Vec::new();
        }

        let start = self.adj_offsets[source_usize] as usize;
        let end = if source_usize + 1 < self.adj_offsets.len() {
            self.adj_offsets[source_usize + 1] as usize
        } else {
            self.adj_targets.len()
        };

        self.adj_targets[start..end]
            .iter()
            .filter_map(|&syn_idx| {
                let idx = syn_idx as usize;
                if idx < self.synapse_ids.len() {
                    Some(Synapse {
                        id: self.synapse_ids[idx],
                        source: self.synapse_sources[idx],
                        target: self.synapse_targets[idx],
                        weight: self.synapse_weights[idx],
                        last_active: 0,
                        permanence: self.synapse_permanence[idx],
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn update_synapse(&mut self, id: u32, learning_rate: f32, is_active: bool) {
        if let Some(idx) = self.synapse_ids.iter().position(|&x| x == id) {
            if is_active {
                self.synapse_weights[idx] = (self.synapse_weights[idx]
                    + learning_rate * (1.0 - self.synapse_weights[idx]))
                    .min(1.0);
                self.synapse_permanence[idx] =
                    (self.synapse_permanence[idx] + learning_rate * 0.1).min(1.0);
            } else {
                self.synapse_weights[idx] = (self.synapse_weights[idx]
                    - learning_rate * self.synapse_weights[idx])
                    .max(0.0);
                self.synapse_permanence[idx] =
                    (self.synapse_permanence[idx] - learning_rate * 0.05).max(0.0);
            }
        }
    }

    pub fn prune_weak_synapses(&mut self, threshold: f32) {
        let to_remove: Vec<u32> = self
            .synapse_ids
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                self.synapse_permanence[*i] < threshold || self.synapse_permanence[*i] <= 0.1
            })
            .map(|(_, &id)| id)
            .collect();

        for id in to_remove {
            if let Some(idx) = self.synapse_ids.iter().position(|&x| x == id) {
                self.synapse_ids.remove(idx);
                self.synapse_sources.remove(idx);
                self.synapse_targets.remove(idx);
                self.synapse_weights.remove(idx);
                self.synapse_permanence.remove(idx);
            }
        }

        self.adj_targets.clear();
        self.adj_offsets.clear();

        for (i, &source) in self.synapse_sources.iter().enumerate() {
            let source_usize = source as usize;
            while self.adj_offsets.len() <= source_usize {
                self.adj_offsets.push(self.adj_targets.len() as u32);
            }
            self.adj_targets.push(i as u32);
        }
        let total = self.adj_targets.len() as u32;
        while self.adj_offsets.len()
            <= self
                .synapse_sources
                .iter()
                .map(|&s| s as usize)
                .max()
                .unwrap_or(0)
        {
            self.adj_offsets.push(total);
        }
        if self.adj_offsets.last() != Some(&total) {
            self.adj_offsets.push(total);
        }
    }

    pub fn synapse_count(&self) -> usize {
        self.synapse_ids.len()
    }
}

impl Default for SparseGraph {
    fn default() -> Self {
        Self::new()
    }
}
