/// Neural Cell Implementation
/// Implements: AC §12 Cell Contract, SD-06
/// Individual neuron unit with activation, potential, threshold, and refractory dynamics

use serde::{Deserialize, Serialize};

const DEFAULT_THRESHOLD: f32 = 0.5;
const REFRACTORY_PERIOD_CYCLES: u64 = 2;
const DECAY_RATE: f32 = 0.95;
const SPIKE_MAGNITUDE: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellState {
    Resting,      // No recent activity
    Integrating,  // Accumulating potential
    Firing,       // Active spike
    Refractory,   // Spike just fired, not receptive
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub id: u32,
    pub activation: f32,           // [0.0, 1.0] - current firing rate
    pub potential: f32,            // Membrane potential  
    pub threshold: f32,            // Threshold for firing
    pub state: CellState,          // Current state
    pub refractory_until: u64,     // Cycle count until refractory ends
    pub last_fired: u64,           // When last fired
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

    /// Update cell during simulation cycle
    /// AC §12.3: Cell dynamics
    pub fn update(&mut self, current_cycle: u64, input_current: f32) -> bool {
        // Check refractory period
        if current_cycle < self.refractory_until {
            self.state = CellState::Refractory;
            self.activation *= DECAY_RATE; // Decay activation in refractory
            return false;
        }

        // Integrate input current with decay
        self.potential = self.potential * DECAY_RATE + input_current;

        // Update state based on potential
        if self.potential >= self.threshold {
            // Fire!
            self.state = CellState::Firing;
            self.activation = SPIKE_MAGNITUDE;
            self.last_fired = current_cycle;
            self.refractory_until = current_cycle + REFRACTORY_PERIOD_CYCLES;
            true
        } else if self.potential > 0.0 {
            // Integrating
            self.state = CellState::Integrating;
            self.activation = (self.potential / self.threshold).min(1.0);
            false
        } else {
            // Resting
            self.state = CellState::Resting;
            self.activation = (self.activation * DECAY_RATE).max(0.0);
            false
        }
    }

    /// Old-style update for backward compatibility
    pub fn update_simple(&mut self) {
        self.activation *= DECAY_RATE;
        if self.potential >= self.threshold {
            self.activation = 1.0;
        }
        self.potential *= DECAY_RATE;
    }

    /// Fire cell immediately (used for testing/injection)
    pub fn fire(&mut self) -> bool {
        if self.state == CellState::Refractory {
            return false;
        }
        
        self.state = CellState::Firing;
        self.activation = SPIKE_MAGNITUDE;
        self.potential = self.threshold + 0.1;
        true
    }

    /// Reset cell to resting state
    pub fn reset(&mut self) {
        self.activation = 0.0;
        self.potential = 0.0;
        self.state = CellState::Resting;
    }

    /// Check if cell recently fired
    pub fn is_recently_active(&self, current_cycle: u64, lookback: u64) -> bool {
        current_cycle.saturating_sub(self.last_fired) < lookback
    }

    /// Get cell's current activity as probability [0, 1]
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

/// SoA-style cell pool for efficient memory layout
/// AC §16 SoA Layout Contract
pub struct CellPool {
    pub ids: Vec<u32>,
    pub activation: Vec<f32>,
    pub potential: Vec<f32>,
    pub threshold: Vec<f32>,
    pub state: Vec<CellState>,
    pub refractory_until: Vec<u64>,
    pub last_fired: Vec<u64>,
    pub usage: Vec<u32>,  // Reference count for GC
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

    /// Update all active cells in pool
    pub fn update_all(&mut self, current_cycle: u64, input_currents: &[f32]) -> Vec<u32> {
        let mut fired = Vec::new();
        
        for idx in 0..self.activation.len() {
            if self.usage[idx] == 0 {
                continue; // Skip unused cells
            }

            let input = input_currents.get(idx).copied().unwrap_or(0.0);
            
            // Check refractory
            if current_cycle < self.refractory_until[idx] {
                self.state[idx] = CellState::Refractory;
                self.activation[idx] *= DECAY_RATE;
                continue;
            }

            // Integrate
            self.potential[idx] = self.potential[idx] * DECAY_RATE + input;

            // Check threshold
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

    /// Get specific cell as struct
    pub fn get(&self, idx: usize) -> Cell {
        Cell {
            id: self.ids.get(idx).copied().unwrap_or(0),
            activation: self.activation.get(idx).copied().unwrap_or(0.0),
            potential: self.potential.get(idx).copied().unwrap_or(0.0),
            threshold: self.threshold.get(idx).copied().unwrap_or(DEFAULT_THRESHOLD),
            state: self.state.get(idx).copied().unwrap_or(CellState::Resting),
            refractory_until: self.refractory_until.get(idx).copied().unwrap_or(0),
            last_fired: self.last_fired.get(idx).copied().unwrap_or(0),
        }
    }

    /// Reset all cells
    pub fn reset_all(&mut self) {
        for i in 0..self.activation.len() {
            self.activation[i] = 0.0;
            self.potential[i] = 0.0;
            self.state[i] = CellState::Resting;
        }
    }
}
