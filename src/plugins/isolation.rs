use super::lifecycle::PluginHealth;

pub struct PluginIsolation {
    health: PluginHealth,
    failure_count: u32,
    max_failures: u32,
}

impl PluginIsolation {
    pub fn new(max_failures: u32) -> Self {
        Self {
            health: PluginHealth::Healthy,
            failure_count: 0,
            max_failures,
        }
    }

    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.health = PluginHealth::Healthy;
    }

    pub fn record_failure(&mut self) -> PluginHealth {
        self.failure_count += 1;
        if self.failure_count >= self.max_failures {
            self.health = PluginHealth::Failed;
        } else {
            self.health = PluginHealth::Degraded;
        }
        self.health
    }

    pub fn health(&self) -> PluginHealth {
        self.health
    }

    pub fn is_isolated(&self) -> bool {
        self.health == PluginHealth::Failed
    }
}
