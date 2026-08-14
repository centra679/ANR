use crate::error::{Error, Result};
/// Runtime State Machine
/// Implements: AC §18 Autonomous Loop Contract, SD-01
/// Strict 16-state finite state machine with safety invariants
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeState {
    PoweredOff,
    Boot,
    ConfigLoad,
    BrainOpen,
    BrainValidate,
    Recovery,
    CpuSimdDetect,
    MemoryInit,
    HalInit,
    PluginInit,
    NeuralInit,
    SchedulerInit,
    Running,
    Degraded,
    EmergencyStopped,
    ShuttingDown,
    Fault,
}

impl fmt::Display for RuntimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeState::PoweredOff => write!(f, "PoweredOff"),
            RuntimeState::Boot => write!(f, "Boot"),
            RuntimeState::ConfigLoad => write!(f, "ConfigLoad"),
            RuntimeState::BrainOpen => write!(f, "BrainOpen"),
            RuntimeState::BrainValidate => write!(f, "BrainValidate"),
            RuntimeState::Recovery => write!(f, "Recovery"),
            RuntimeState::CpuSimdDetect => write!(f, "CpuSimdDetect"),
            RuntimeState::MemoryInit => write!(f, "MemoryInit"),
            RuntimeState::HalInit => write!(f, "HalInit"),
            RuntimeState::PluginInit => write!(f, "PluginInit"),
            RuntimeState::NeuralInit => write!(f, "NeuralInit"),
            RuntimeState::SchedulerInit => write!(f, "SchedulerInit"),
            RuntimeState::Running => write!(f, "Running"),
            RuntimeState::Degraded => write!(f, "Degraded"),
            RuntimeState::EmergencyStopped => write!(f, "EmergencyStopped"),
            RuntimeState::ShuttingDown => write!(f, "ShuttingDown"),
            RuntimeState::Fault => write!(f, "Fault"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    PowerOn,
    ConfigLoaded,
    BrainOpened,
    BrainValid,
    BrainInvalid,
    RecoveryComplete,
    RecoveryFailed,
    PluginsReady,
    PluginFailed,
    NeuralReady,
    SchedulerReady,
    SafetyTriggered,
    EmergencyStopRequested,
    ShutdownRequested,
    FatalError(String),
}

/// State machine transition validator
pub struct StateMachine {
    current: RuntimeState,
    previous: RuntimeState,
    brain_valid: bool,
    safety_ready: bool,
    _maintenance_active: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current: RuntimeState::PoweredOff,
            previous: RuntimeState::PoweredOff,
            brain_valid: false,
            safety_ready: false,
            _maintenance_active: false,
        }
    }

    pub fn current_state(&self) -> RuntimeState {
        self.current
    }

    pub fn brain_is_valid(&self) -> bool {
        self.brain_valid
    }

    pub fn set_brain_valid(&mut self, valid: bool) {
        self.brain_valid = valid;
    }

    pub fn safety_is_ready(&self) -> bool {
        self.safety_ready
    }

    pub fn set_safety_ready(&mut self, ready: bool) {
        self.safety_ready = ready;
    }

    /// Main state transition function with strict invariant enforcement
    pub fn transition(&mut self, event: RuntimeEvent) -> Result<()> {
        let next_state = self.compute_next_state(&event)?;

        // Validate transition is legal
        self.validate_transition(self.current, next_state, &event)?;

        // Apply state-specific entry logic
        self.apply_entry_logic(next_state)?;

        // Record transition
        self.previous = self.current;
        self.current = next_state;

        Ok(())
    }

    fn compute_next_state(&self, event: &RuntimeEvent) -> Result<RuntimeState> {
        match (self.current, event) {
            // Boot sequence
            (RuntimeState::PoweredOff, RuntimeEvent::PowerOn) => Ok(RuntimeState::Boot),
            (RuntimeState::Boot, RuntimeEvent::ConfigLoaded) => Ok(RuntimeState::ConfigLoad),
            (RuntimeState::ConfigLoad, RuntimeEvent::BrainOpened) => Ok(RuntimeState::BrainOpen),
            (RuntimeState::BrainOpen, RuntimeEvent::BrainValid) => Ok(RuntimeState::BrainValidate),
            (RuntimeState::BrainOpen, RuntimeEvent::BrainInvalid) => Ok(RuntimeState::Recovery),
            (RuntimeState::BrainValidate, RuntimeEvent::RecoveryComplete) => {
                Ok(RuntimeState::CpuSimdDetect)
            }
            (RuntimeState::Recovery, RuntimeEvent::RecoveryComplete) => {
                Ok(RuntimeState::CpuSimdDetect)
            }
            (RuntimeState::Recovery, RuntimeEvent::RecoveryFailed) => Ok(RuntimeState::Fault),

            // Hardware init sequence
            (RuntimeState::CpuSimdDetect, RuntimeEvent::ConfigLoaded) => {
                Ok(RuntimeState::MemoryInit)
            }
            (RuntimeState::MemoryInit, RuntimeEvent::ConfigLoaded) => Ok(RuntimeState::HalInit),
            (RuntimeState::HalInit, RuntimeEvent::ConfigLoaded) => Ok(RuntimeState::PluginInit),

            // Plugin and neural init
            (RuntimeState::PluginInit, RuntimeEvent::PluginsReady) => Ok(RuntimeState::NeuralInit),
            (RuntimeState::PluginInit, RuntimeEvent::PluginFailed) => {
                // If non-critical plugin fails, try degraded
                Ok(RuntimeState::NeuralInit)
            }
            (RuntimeState::NeuralInit, RuntimeEvent::NeuralReady) => {
                Ok(RuntimeState::SchedulerInit)
            }
            (RuntimeState::SchedulerInit, RuntimeEvent::SchedulerReady) => {
                Ok(RuntimeState::Running)
            }

            // Running state transitions
            (RuntimeState::Running, RuntimeEvent::EmergencyStopRequested) => {
                Ok(RuntimeState::EmergencyStopped)
            }
            (RuntimeState::Running, RuntimeEvent::SafetyTriggered) => {
                Ok(RuntimeState::EmergencyStopped)
            }
            (RuntimeState::Running, RuntimeEvent::FatalError(_)) => Ok(RuntimeState::Fault),
            (RuntimeState::Running, RuntimeEvent::ShutdownRequested) => {
                Ok(RuntimeState::ShuttingDown)
            }

            // Degraded mode (optional lower-capacity operation)
            (RuntimeState::Degraded, RuntimeEvent::EmergencyStopRequested) => {
                Ok(RuntimeState::EmergencyStopped)
            }
            (RuntimeState::Degraded, RuntimeEvent::SafetyTriggered) => {
                Ok(RuntimeState::EmergencyStopped)
            }
            (RuntimeState::Degraded, RuntimeEvent::FatalError(_)) => Ok(RuntimeState::Fault),
            (RuntimeState::Degraded, RuntimeEvent::ShutdownRequested) => {
                Ok(RuntimeState::ShuttingDown)
            }

            // Emergency stop (always reachable, final safety state)
            (RuntimeState::EmergencyStopped, RuntimeEvent::ShutdownRequested) => {
                Ok(RuntimeState::ShuttingDown)
            }
            (RuntimeState::EmergencyStopped, RuntimeEvent::FatalError(_)) => {
                Ok(RuntimeState::Fault)
            }

            // Graceful shutdown
            (RuntimeState::ShuttingDown, RuntimeEvent::ConfigLoaded) => {
                Ok(RuntimeState::PoweredOff)
            }

            // Fault is terminal until power-off
            (RuntimeState::Fault, RuntimeEvent::PowerOn) => Ok(RuntimeState::Boot),

            // Invalid transition
            _ => Err(Error::InternalRuntimeStateTransitionInvalid(format!(
                "{} -> {}",
                self.current,
                event.name()
            ))),
        }
    }

    fn validate_transition(
        &self,
        from: RuntimeState,
        to: RuntimeState,
        event: &RuntimeEvent,
    ) -> Result<()> {
        // AC §31: Safety invariants

        // Cannot activate actuators before safety ready
        if matches!(to, RuntimeState::Running) && !self.safety_ready {
            return Err(Error::SafetyNotReady(
                "Cannot enter Running without safety initialization".to_string(),
            ));
        }

        // Cannot learn before brain valid
        if matches!(to, RuntimeState::Running) && !self.brain_valid {
            return Err(Error::BrainNotValid(
                "Cannot enter Running without valid brain".to_string(),
            ));
        }

        // Degraded mode must not disable safety
        if matches!(to, RuntimeState::Degraded) && !self.safety_ready {
            return Err(Error::SafetyNotReady(
                "Degraded mode requires active safety".to_string(),
            ));
        }

        // Emergency stop must be reachable from Running/Degraded
        if matches!(event, RuntimeEvent::EmergencyStopRequested)
            && !matches!(from, RuntimeState::Running | RuntimeState::Degraded)
        {
            tracing::warn!("Emergency stop from non-operational state: {}", from);
        }

        Ok(())
    }

    fn apply_entry_logic(&mut self, state: RuntimeState) -> Result<()> {
        match state {
            RuntimeState::Running => {
                // Verify all prerequisites before entering Running
                if !self.brain_valid {
                    return Err(Error::BrainNotValid(
                        "Cannot run with invalid brain".to_string(),
                    ));
                }
                if !self.safety_ready {
                    return Err(Error::SafetyNotReady(
                        "Cannot run without safety ready".to_string(),
                    ));
                }
                tracing::info!("Runtime entering Running state");
            }

            RuntimeState::Degraded => {
                tracing::warn!("Runtime entering Degraded mode - operating at reduced capacity");
            }

            RuntimeState::EmergencyStopped => {
                tracing::error!("Runtime entered EmergencyStopped - triggering safety shutdown");
            }

            RuntimeState::Fault => {
                tracing::error!("Runtime entered Fault state - immediate safe shutdown required");
            }

            RuntimeState::ShuttingDown => {
                tracing::info!("Runtime shutting down - finalizing brain state");
            }

            _ => {}
        }

        Ok(())
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeState {
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            RuntimeState::PoweredOff
                | RuntimeState::Degraded
                | RuntimeState::EmergencyStopped
                | RuntimeState::ShuttingDown
                | RuntimeState::Fault
        )
    }

    pub fn is_operational(&self) -> bool {
        matches!(self, RuntimeState::Running | RuntimeState::Degraded)
    }

    pub fn can_activate_actuators(&self) -> bool {
        // AC §31: No actuator before safety ready
        matches!(self, RuntimeState::Running)
    }

    pub fn can_learn(&self) -> bool {
        // AC §31: No learning before brain valid
        matches!(self, RuntimeState::Running)
    }

    pub fn is_bootable_state(&self) -> bool {
        matches!(
            self,
            RuntimeState::Boot | RuntimeState::ConfigLoad | RuntimeState::BrainOpen
        )
    }
}

impl RuntimeEvent {
    pub fn name(&self) -> String {
        match self {
            RuntimeEvent::PowerOn => "PowerOn".to_string(),
            RuntimeEvent::ConfigLoaded => "ConfigLoaded".to_string(),
            RuntimeEvent::BrainOpened => "BrainOpened".to_string(),
            RuntimeEvent::BrainValid => "BrainValid".to_string(),
            RuntimeEvent::BrainInvalid => "BrainInvalid".to_string(),
            RuntimeEvent::RecoveryComplete => "RecoveryComplete".to_string(),
            RuntimeEvent::RecoveryFailed => "RecoveryFailed".to_string(),
            RuntimeEvent::PluginsReady => "PluginsReady".to_string(),
            RuntimeEvent::PluginFailed => "PluginFailed".to_string(),
            RuntimeEvent::NeuralReady => "NeuralReady".to_string(),
            RuntimeEvent::SchedulerReady => "SchedulerReady".to_string(),
            RuntimeEvent::SafetyTriggered => "SafetyTriggered".to_string(),
            RuntimeEvent::EmergencyStopRequested => "EmergencyStopRequested".to_string(),
            RuntimeEvent::ShutdownRequested => "ShutdownRequested".to_string(),
            RuntimeEvent::FatalError(msg) => format!("FatalError: {}", msg),
        }
    }
}
