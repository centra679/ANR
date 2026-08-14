/// Sparse Graph - sparse neural topology
/// Implements: AC §15
use super::synapse::Synapse;
use std::collections::HashMap;

pub struct SparseGraph {
    synapses: HashMap<u32, Synapse>,
    adjacency: HashMap<u32, Vec<u32>>, // source -> [target_synapse_ids]
}

impl SparseGraph {
    pub fn new() -> Self {
        Self {
            synapses: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        let target_list = self
            .adjacency
            .entry(synapse.source)
            .or_insert_with(Vec::new);
        target_list.push(synapse.id);
        self.synapses.insert(synapse.id, synapse);
    }

    pub fn get_outgoing(&self, source: u32) -> Vec<Synapse> {
        self.adjacency
            .get(&source)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.synapses.get(id).copied())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn update_synapse(&mut self, id: u32, learning_rate: f32, is_active: bool) {
        if let Some(synapse) = self.synapses.get_mut(&id) {
            synapse.update(learning_rate, is_active);
        }
    }

    pub fn prune_weak_synapses(&mut self, threshold: f32) {
        let to_remove: Vec<u32> = self
            .synapses
            .iter()
            .filter(|(_, s)| !s.is_connected() || s.permanence < threshold)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            self.synapses.remove(&id);
            for targets in self.adjacency.values_mut() {
                targets.retain(|&sid| sid != id);
            }
        }
    }

    pub fn synapse_count(&self) -> usize {
        self.synapses.len()
    }
}

impl Default for SparseGraph {
    fn default() -> Self {
        Self::new()
    }
}
