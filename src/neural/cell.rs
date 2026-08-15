//! Neural Cell Implementation
//! Implements: AC §12 Cell Contract, SD-06

const DEFAULT_THRESHOLD: f32 = 0.5;
const REFRACTORY_PERIOD_CYCLES: u64 = 2;
const DECAY_RATE: f32 = 0.95;
const SPIKE_MAGNITUDE: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Resting,
    Integrating,
    Firing,
    Refractory,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub id: u32,
    pub activation: f32,
    pub potential: f32,
    pub threshold: f32,
    pub state: CellState,
    pub refractory_until: u64,
    pub last_fired: u64,
}

impl Cell {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            activation: 0.0,
            potential: 0.0,
            threshold: DEFAULT_THRESHOLD,
            state: CellState::Resting,
            refractory_until: 0,
            last_fired: 0,
        }
    }

    pub fn new_with_threshold(id: u32, threshold: f32) -> Self {
        Self {
            id,
            activation: 0.0,
            potential: 0.0,
            threshold: threshold.clamp(0.0, 1.0),
            state: CellState::Resting,
            refractory_until: 0,
            last_fired: 0,
        }
    }

    pub fn update(&mut self, current_cycle: u64, input_current: f32) -> bool {
        if current_cycle < self.refractory_until {
            self.state = CellState::Refractory;
            self.activation *= DECAY_RATE;
            return false;
        }

        self.potential = self.potential * DECAY_RATE + input_current;

        if self.potential >= self.threshold {
            self.state = CellState::Firing;
            self.activation = SPIKE_MAGNITUDE;
            self.last_fired = current_cycle;
            self.refractory_until = current_cycle + REFRACTORY_PERIOD_CYCLES;
            true
        } else if self.potential > 0.0 {
            self.state = CellState::Integrating;
            self.activation = (self.potential / self.threshold).min(1.0);
            false
        } else {
            self.state = CellState::Resting;
            self.activation = (self.activation * DECAY_RATE).max(0.0);
            false
        }
    }

    pub fn update_simple(&mut self) {
        self.activation *= DECAY_RATE;
        if self.potential >= self.threshold {
            self.activation = 1.0;
        }
        self.potential *= DECAY_RATE;
    }

    pub fn fire(&mut self) -> bool {
        if self.state == CellState::Refractory {
            return false;
        }

        self.state = CellState::Firing;
        self.activation = SPIKE_MAGNITUDE;
        self.potential = self.threshold + 0.1;
        true
    }

    pub fn reset(&mut self) {
        self.activation = 0.0;
        self.potential = 0.0;
        self.state = CellState::Resting;
    }

    pub fn is_recently_active(&self, current_cycle: u64, lookback: u64) -> bool {
        current_cycle.saturating_sub(self.last_fired) < lookback
    }

    pub fn get_activity_probability(&self) -> f32 {
        self.activation.clamp(0.0, 1.0)
    }

    pub fn is_firing(&self) -> bool {
        self.state == CellState::Firing
    }

    pub fn is_refractory(&self) -> bool {
        self.state == CellState::Refractory
    }
}

pub struct CellPool {
    pub ids: Vec<u32>,
    pub activation: Vec<f32>,
    pub potential: Vec<f32>,
    pub threshold: Vec<f32>,
    pub state: Vec<CellState>,
    pub refractory_until: Vec<u64>,
    pub last_fired: Vec<u64>,
    pub usage: Vec<u32>,
}

impl CellPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            ids: vec![0; capacity],
            activation: vec![0.0; capacity],
            potential: vec![0.0; capacity],
            threshold: vec![DEFAULT_THRESHOLD; capacity],
            state: vec![CellState::Resting; capacity],
            refractory_until: vec![0; capacity],
            last_fired: vec![0; capacity],
            usage: vec![0; capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.activation.len()
    }

    pub fn update_all(&mut self, current_cycle: u64, input_currents: &[f32]) -> Vec<u32> {
        let mut fired = Vec::with_capacity(self.activation.len() / 4);

        for idx in 0..self.activation.len() {
            if self.usage[idx] == 0 {
                continue;
            }

            let input = input_currents.get(idx).copied().unwrap_or(0.0);

            if current_cycle < self.refractory_until[idx] {
                self.state[idx] = CellState::Refractory;
                self.activation[idx] *= DECAY_RATE;
                continue;
            }

            self.potential[idx] = self.potential[idx] * DECAY_RATE + input;

            if self.potential[idx] >= self.threshold[idx] {
                self.state[idx] = CellState::Firing;
                self.activation[idx] = SPIKE_MAGNITUDE;
                self.last_fired[idx] = current_cycle;
                self.refractory_until[idx] = current_cycle + REFRACTORY_PERIOD_CYCLES;
                fired.push(idx as u32);
            } else if self.potential[idx] > 0.0 {
                self.state[idx] = CellState::Integrating;
                self.activation[idx] = (self.potential[idx] / self.threshold[idx]).min(1.0);
            } else {
                self.state[idx] = CellState::Resting;
                self.activation[idx] = (self.activation[idx] * DECAY_RATE).max(0.0);
            }
        }

        fired
    }

    pub fn get(&self, idx: usize) -> Cell {
        Cell {
            id: self.ids.get(idx).copied().unwrap_or(0),
            activation: self.activation.get(idx).copied().unwrap_or(0.0),
            potential: self.potential.get(idx).copied().unwrap_or(0.0),
            threshold: self
                .threshold
                .get(idx)
                .copied()
                .unwrap_or(DEFAULT_THRESHOLD),
            state: self.state.get(idx).copied().unwrap_or(CellState::Resting),
            refractory_until: self.refractory_until.get(idx).copied().unwrap_or(0),
            last_fired: self.last_fired.get(idx).copied().unwrap_or(0),
        }
    }

    pub fn reset_all(&mut self) {
        for i in 0..self.activation.len() {
            self.activation[i] = 0.0;
            self.potential[i] = 0.0;
            self.state[i] = CellState::Resting;
        }
    }
}
