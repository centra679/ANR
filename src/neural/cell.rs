/// Cell - smallest neural unit
/// Implements: AC §11

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub id: u32,
    pub activation: f32,
    pub potential: f32,
    pub threshold: f32,
    pub refractory_count: u16,
}

impl Cell {
    pub fn new(id: u32, threshold: f32) -> Self {
        Self {
            id,
            activation: 0.0,
            potential: 0.0,
            threshold,
            refractory_count: 0,
        }
    }

    pub fn fire(&mut self) {
        if self.potential >= self.threshold && self.refractory_count == 0 {
            self.activation = 1.0;
            self.refractory_count = 2; // Refractory period
        }
    }

    pub fn update(&mut self) {
        self.activation *= 0.95; // Decay
        if self.refractory_count > 0 {
            self.refractory_count -= 1;
        }
        self.potential *= 0.9; // Decay
    }

    pub fn is_firing(&self) -> bool {
        self.activation > 0.5
    }

    pub fn is_refractory(&self) -> bool {
        self.refractory_count > 0
    }
}
