/// Synapse - connection between cells/columns
/// Implements: AC §14, SD-06
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Synapse {
    pub id: u32,
    pub source: u32,
    pub target: u32,
    pub weight: f32,
    pub last_active: u64,
    pub permanence: f32,
}

impl Synapse {
    pub fn new(id: u32, source: u32, target: u32) -> Self {
        Self {
            id,
            source,
            target,
            weight: 0.5,
            last_active: 0,
            permanence: 0.5,
        }
    }

    pub fn update(&mut self, learning_rate: f32, is_active: bool) {
        if is_active {
            self.weight = (self.weight + learning_rate * (1.0 - self.weight)).min(1.0);
            self.permanence = (self.permanence + learning_rate * 0.1).min(1.0);
        } else {
            self.weight = (self.weight - learning_rate * self.weight).max(0.0);
            self.permanence = (self.permanence - learning_rate * 0.05).max(0.0);
        }
    }

    pub fn decay(&mut self) {
        self.weight *= 0.99;
        self.permanence *= 0.99;
    }

    pub fn is_connected(&self) -> bool {
        self.permanence > 0.1
    }
}

/// SynapseTable - SoA layout for synapses
/// AC §16, SD-06: Production data layout must use Structure of Arrays
pub struct SynapseTable {
    pub source_kind: Vec<u8>,
    pub source_id: Vec<u64>,
    pub target_kind: Vec<u8>,
    pub target_id: Vec<u64>,
    pub weight: Vec<f32>,
    pub strength: Vec<f32>,
    pub state: Vec<u8>,
    pub last_active: Vec<u64>,
    pub age: Vec<u32>,
    pub plasticity: Vec<f32>,
}

impl SynapseTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            source_kind: vec![0; capacity],
            source_id: vec![0; capacity],
            target_kind: vec![0; capacity],
            target_id: vec![0; capacity],
            weight: vec![0.0; capacity],
            strength: vec![0.0; capacity],
            state: vec![0; capacity],
            last_active: vec![0; capacity],
            age: vec![0; capacity],
            plasticity: vec![0.0; capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.weight.len()
    }

    pub fn len(&self) -> usize {
        self.weight.len()
    }

    pub fn is_empty(&self) -> bool {
        self.weight.is_empty()
    }

    pub fn add(
        &mut self,
        source_kind: u8,
        source_id: u64,
        target_kind: u8,
        target_id: u64,
    ) -> usize {
        let idx = self.weight.len();
        self.source_kind.push(source_kind);
        self.source_id.push(source_id);
        self.target_kind.push(target_kind);
        self.target_id.push(target_id);
        self.weight.push(0.5);
        self.strength.push(0.5);
        self.state.push(0);
        self.last_active.push(0);
        self.age.push(0);
        self.plasticity.push(1.0);
        idx
    }

    pub fn get(&self, idx: usize) -> Option<Synapse> {
        if idx >= self.weight.len() {
            return None;
        }
        Some(Synapse {
            id: idx as u32,
            source: self.source_id[idx] as u32,
            target: self.target_id[idx] as u32,
            weight: self.weight[idx],
            last_active: self.last_active[idx],
            permanence: self.strength[idx],
        })
    }

    pub fn update_weight(&mut self, idx: usize, learning_rate: f32, is_active: bool) {
        if idx < self.weight.len() {
            if is_active {
                self.weight[idx] =
                    (self.weight[idx] + learning_rate * (1.0 - self.weight[idx])).min(1.0);
                self.strength[idx] = (self.strength[idx] + learning_rate * 0.1).min(1.0);
            } else {
                self.weight[idx] = (self.weight[idx] - learning_rate * self.weight[idx]).max(0.0);
                self.strength[idx] = (self.strength[idx] - learning_rate * 0.05).max(0.0);
            }
        }
    }

    pub fn decay(&mut self, idx: usize) {
        if idx < self.weight.len() {
            self.weight[idx] *= 0.99;
            self.strength[idx] *= 0.99;
        }
    }

    pub fn prune_weak(&mut self, threshold: f32) -> usize {
        let original_len = self.weight.len();
        let mut write_idx = 0;
        for read_idx in 0..original_len {
            if self.strength[read_idx] > threshold {
                if write_idx != read_idx {
                    self.source_kind[write_idx] = self.source_kind[read_idx];
                    self.source_id[write_idx] = self.source_id[read_idx];
                    self.target_kind[write_idx] = self.target_kind[read_idx];
                    self.target_id[write_idx] = self.target_id[read_idx];
                    self.weight[write_idx] = self.weight[read_idx];
                    self.strength[write_idx] = self.strength[read_idx];
                    self.state[write_idx] = self.state[read_idx];
                    self.last_active[write_idx] = self.last_active[read_idx];
                    self.age[write_idx] = self.age[read_idx];
                    self.plasticity[write_idx] = self.plasticity[read_idx];
                }
                write_idx += 1;
            }
        }
        if write_idx < original_len {
            self.source_kind.truncate(write_idx);
            self.source_id.truncate(write_idx);
            self.target_kind.truncate(write_idx);
            self.target_id.truncate(write_idx);
            self.weight.truncate(write_idx);
            self.strength.truncate(write_idx);
            self.state.truncate(write_idx);
            self.last_active.truncate(write_idx);
            self.age.truncate(write_idx);
            self.plasticity.truncate(write_idx);
        }
        original_len - write_idx
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        (0..self.weight.len())
    }
}
