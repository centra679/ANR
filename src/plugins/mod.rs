pub mod isolation;
pub mod lifecycle;

use crate::Result;

pub trait Plugin {
    fn name(&self) -> &str;
    fn init(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn health(&self) -> lifecycle::PluginHealth;
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    pub fn init_all(&mut self) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.init()?;
        }
        Ok(())
    }

    pub fn start_all(&mut self) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.start()?;
        }
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.stop()?;
        }
        Ok(())
    }
}
