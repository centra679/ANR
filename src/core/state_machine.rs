/// Runtime State Machine
/// Implements: AC §18 Autonomous Loop Contract, SD-01

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
    FatalError,
}

impl RuntimeState {
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            RuntimeState::PoweredOff
                | RuntimeState::Degraded
                | RuntimeState::EmergencyStopped
                | RuntimeState::ShuttingDown
        )
    }

    pub fn is_operational(&self) -> bool {
        matches!(self, RuntimeState::Running | RuntimeState::Degraded)
    }

    pub fn can_activate_actuators(&self) -> bool {
        matches!(self, RuntimeState::Running)
    }

    pub fn can_learn(&self) -> bool {
        matches!(self, RuntimeState::Running)
    }
}
