/// ANR Error Type Taxonomy
/// Aligns with: AC §21

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Core Runtime Errors
    #[error("Runtime state error: {0}")]
    RuntimeState(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Initialization failed: {0}")]
    InitError(String),

    // Brain/Storage Errors
    #[error("Brain file error: {0}")]
    BrainError(String),

    #[error("Brain validation failed: {0}")]
    BrainValidation(String),

    #[error("Brain recovery failed: {0}")]
    BrainRecovery(String),

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Corrupt generation")]
    CorruptGeneration,

    // Neural Core Errors
    #[error("Neural core error: {0}")]
    NeuralCore(String),

    #[error("Cell operation failed: {0}")]
    CellError(String),

    #[error("Column operation failed: {0}")]
    ColumnError(String),

    #[error("Synapse operation failed: {0}")]
    SynapseError(String),

    // Memory Errors
    #[error("Memory quota exceeded: {section} (used: {used}/{max})")]
    MemoryQuotaExceeded { section: String, used: u64, max: u64 },

    #[error("Allocation failed: {0}")]
    AllocationFailed(String),

    #[error("Garbage collection failed: {0}")]
    GCFailed(String),

    // Learning Errors
    #[error("Learning error: {0}")]
    LearningError(String),

    #[error("Replay error: {0}")]
    ReplayError(String),

    #[error("Consolidation error: {0}")]
    ConsolidationError(String),

    // Perception Errors
    #[error("Sensor error: {0}")]
    SensorError(String),

    #[error("Camera buffer error: {0}")]
    CameraError(String),

    #[error("Audio buffer error: {0}")]
    AudioError(String),

    // Plugin/HAL Errors
    #[error("Plugin error: {plugin}: {reason}")]
    PluginError { plugin: String, reason: String },

    #[error("Plugin failed to load: {0}")]
    PluginLoadError(String),

    #[error("HAL error: {0}")]
    HalError(String),

    // Decision/Safety Errors
    #[error("Safety constraint violation: {0}")]
    SafetyViolation(String),

    #[error("Decision error: {0}")]
    DecisionError(String),

    #[error("Actuator command rejected")]
    ActuatorRejected,

    // Storage/IO Errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Recovery impossible: {0}")]
    RecoveryImpossible(String),

    // System Errors
    #[error("Fatal error: {0}")]
    Fatal(String),

    #[error("Resource unavailable: {0}")]
    ResourceUnavailable(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    // Generic
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Error::Fatal(_)
                | Error::BrainRecovery(_)
                | Error::CorruptGeneration
                | Error::RecoveryImpossible(_)
                | Error::ChecksumMismatch
        )
    }

    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }
}
