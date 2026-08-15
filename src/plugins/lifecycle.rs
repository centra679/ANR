use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Discovered,
    Loaded,
    Initialized,
    Running,
    Degraded,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginHealth {
    Healthy,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Copy)]
pub enum PluginEvent {
    Load,
    Init,
    Start,
    Stop,
    Error,
    Restart,
}

pub struct PluginLifecycle {
    state: PluginState,
    restart_count: u32,
    max_restarts: u32,
}

impl PluginLifecycle {
    pub fn new(max_restarts: u32) -> Self {
        Self {
            state: PluginState::Discovered,
            restart_count: 0,
            max_restarts,
        }
    }

    pub fn state(&self) -> PluginState {
        self.state
    }

    pub fn transition(&mut self, event: PluginEvent) -> Result<PluginState> {
        self.state = match (self.state, event) {
            (PluginState::Discovered, PluginEvent::Load) => PluginState::Loaded,
            (PluginState::Loaded, PluginEvent::Init) => PluginState::Initialized,
            (PluginState::Initialized, PluginEvent::Start) => PluginState::Running,
            (PluginState::Running, PluginEvent::Stop) => PluginState::Stopped,
            (PluginState::Running, PluginEvent::Error) => PluginState::Failed,
            (PluginState::Failed, PluginEvent::Restart) => {
                if self.restart_count < self.max_restarts {
                    self.restart_count += 1;
                    PluginState::Discovered
                } else {
                    return Err(crate::Error::PluginError {
                        plugin: "lifecycle".into(),
                        reason: "max restarts exceeded".into(),
                    });
                }
            }
            (PluginState::Degraded, PluginEvent::Stop) => PluginState::Stopped,
            (PluginState::Degraded, PluginEvent::Error) => PluginState::Failed,
            (PluginState::Loaded, PluginEvent::Start) => PluginState::Running,
            (PluginState::Initialized, PluginEvent::Stop) => PluginState::Stopped,
            (PluginState::Stopped, PluginEvent::Restart) => {
                if self.restart_count < self.max_restarts {
                    self.restart_count += 1;
                    PluginState::Discovered
                } else {
                    return Err(crate::Error::PluginError {
                        plugin: "lifecycle".into(),
                        reason: "max restarts exceeded".into(),
                    });
                }
            }
            _ => {
                return Err(crate::Error::PluginError {
                    plugin: "lifecycle".into(),
                    reason: format!("invalid transition {:?} with {:?}", self.state, event),
                });
            }
        };
        Ok(self.state)
    }

    pub fn is_running(&self) -> bool {
        self.state == PluginState::Running
    }

    pub fn can_restart(&self) -> bool {
        self.restart_count < self.max_restarts
    }
}
