/// ANR Error Type Taxonomy
/// Aligns with: AC §32, SD-16 §16.4
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    // CONFIG
    #[error("ANR-E-CONFIG-001: invalid config: {0}")]
    ConfigInvalid(String),

    #[error("ANR-E-CONFIG-002: config file not found: {0}")]
    ConfigNotFound(String),

    // STORAGE
    #[error("ANR-E-STORAGE-001: header corrupt: {0}")]
    StorageHeaderCorrupt(String),

    #[error("ANR-E-STORAGE-002: checksum mismatch")]
    StorageChecksumMismatch,

    #[error("ANR-E-STORAGE-003: corrupt generation")]
    StorageCorruptGeneration,

    #[error("ANR-E-STORAGE-004: IO error: {0}")]
    StorageIo(#[from] std::io::Error),

    #[error("ANR-E-STORAGE-005: storage write failed: {0}")]
    StorageWriteFailed(String),

    #[error("ANR-E-STORAGE-006: storage fsync failed: {0}")]
    StorageFsyncFailed(String),

    #[error("ANR-E-STORAGE-007: storage backup corrupt: {0}")]
    StorageBackupCorrupt(String),

    #[error("ANR-E-STORAGE-008: storage transaction conflict")]
    StorageTransactionConflict,

    #[error("ANR-E-STORAGE-009: storage recovery failed: {0}")]
    StorageRecoveryFailed(String),

    // BRAIN
    #[error("ANR-E-BRAIN-001: brain not valid: {0}")]
    BrainNotValid(String),

    #[error("ANR-E-BRAIN-002: brain file error: {0}")]
    BrainError(String),

    #[error("ANR-E-BRAIN-003: validation failed: {0}")]
    BrainValidation(String),

    #[error("ANR-E-BRAIN-004: recovery failed: {0}")]
    BrainRecovery(String),

    // VALIDATION
    #[error("ANR-E-VALIDATION-001: invalid input: {0}")]
    ValidationInvalid(String),

    #[error("ANR-E-VALIDATION-002: schema mismatch: {0}")]
    ValidationSchema(String),

    // MEMORY
    #[error("ANR-E-MEMORY-001: quota exceeded: {section} (used: {used}/{max})")]
    MemoryQuotaExceeded {
        section: String,
        used: u64,
        max: u64,
    },

    #[error("ANR-E-MEMORY-002: allocation failed: {0}")]
    MemoryAllocationFailed(String),

    #[error("ANR-E-MEMORY-003: garbage collection failed: {0}")]
    MemoryGCFailed(String),

    // NEURAL
    #[error("ANR-E-NEURAL-001: neural core error: {0}")]
    NeuralCore(String),

    #[error("ANR-E-NEURAL-002: cell operation failed: {0}")]
    NeuralCellError(String),

    #[error("ANR-E-NEURAL-003: column operation failed: {0}")]
    NeuralColumnError(String),

    #[error("ANR-E-NEURAL-004: synapse operation failed: {0}")]
    NeuralSynapseError(String),

    // LEARNING
    #[error("ANR-E-LEARNING-001: learning error: {0}")]
    LearningError(String),

    #[error("ANR-E-LEARNING-002: replay error: {0}")]
    LearningReplayError(String),

    #[error("ANR-E-LEARNING-003: consolidation error: {0}")]
    LearningConsolidationError(String),

    // PERCEPTION
    #[error("ANR-E-PERCEPTION-001: sensor error: {0}")]
    PerceptionSensorError(String),

    #[error("ANR-E-PERCEPTION-002: camera buffer error: {0}")]
    PerceptionCameraError(String),

    #[error("ANR-E-PERCEPTION-003: audio buffer error: {0}")]
    PerceptionAudioError(String),

    // PLUGIN
    #[error("ANR-E-PLUGIN-001: plugin error: {plugin}: {reason}")]
    PluginError { plugin: String, reason: String },

    #[error("ANR-E-PLUGIN-002: plugin failed to load: {0}")]
    PluginLoadError(String),

    // HAL
    #[error("ANR-E-HAL-001: HAL error: {0}")]
    HalError(String),

    // ACTUATOR
    #[error("ANR-E-ACTUATOR-001: actuator command rejected")]
    ActuatorRejected,

    #[error("ANR-E-ACTUATOR-002: actuator error: {0}")]
    ActuatorError(String),

    // SAFETY
    #[error("ANR-E-SAFETY-001: safety constraint violation: {0}")]
    SafetyViolation(String),

    #[error("ANR-E-SAFETY-002: safety not ready: {0}")]
    SafetyNotReady(String),

    // INTERNAL
    #[error("ANR-E-INTERNAL-001: runtime state error: {0}")]
    InternalRuntimeState(String),

    #[error("ANR-E-INTERNAL-002: runtime state transition invalid: {0}")]
    InternalRuntimeStateTransitionInvalid(String),

    #[error("ANR-E-INTERNAL-003: runtime boot failed: {0}")]
    InternalRuntimeBootFailed(String),

    #[error("ANR-E-INTERNAL-004: runtime shutdown failed: {0}")]
    InternalRuntimeShutdownFailed(String),

    #[error("ANR-E-INTERNAL-005: runtime emergency stop failed: {0}")]
    InternalRuntimeEmergencyStopFailed(String),

    #[error("ANR-E-INTERNAL-006: initialization failed: {0}")]
    InternalInitError(String),

    #[error("ANR-E-INTERNAL-007: serialization error: {0}")]
    InternalSerializationError(String),

    #[error("ANR-E-INTERNAL-008: transaction error: {0}")]
    InternalTransactionError(String),

    #[error("ANR-E-INTERNAL-009: recovery impossible: {0}")]
    InternalRecoveryImpossible(String),

    #[error("ANR-E-INTERNAL-010: fatal error: {0}")]
    InternalFatal(String),

    #[error("ANR-E-INTERNAL-011: resource unavailable: {0}")]
    InternalResourceUnavailable(String),

    #[error("ANR-E-INTERNAL-012: timeout: {0}")]
    InternalTimeout(String),

    #[error("ANR-E-INTERNAL-013: other error: {0}")]
    InternalOther(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
        match (self, other) {
            (ConfigInvalid(a), ConfigInvalid(b))
            | (ConfigNotFound(a), ConfigNotFound(b))
            | (StorageHeaderCorrupt(a), StorageHeaderCorrupt(b))
            | (StorageWriteFailed(a), StorageWriteFailed(b))
            | (StorageFsyncFailed(a), StorageFsyncFailed(b))
            | (StorageBackupCorrupt(a), StorageBackupCorrupt(b))
            | (StorageRecoveryFailed(a), StorageRecoveryFailed(b))
            | (BrainNotValid(a), BrainNotValid(b))
            | (BrainError(a), BrainError(b))
            | (BrainValidation(a), BrainValidation(b))
            | (BrainRecovery(a), BrainRecovery(b))
            | (ValidationInvalid(a), ValidationInvalid(b))
            | (ValidationSchema(a), ValidationSchema(b))
            | (MemoryAllocationFailed(a), MemoryAllocationFailed(b))
            | (MemoryGCFailed(a), MemoryGCFailed(b))
            | (NeuralCore(a), NeuralCore(b))
            | (NeuralCellError(a), NeuralCellError(b))
            | (NeuralColumnError(a), NeuralColumnError(b))
            | (NeuralSynapseError(a), NeuralSynapseError(b))
            | (LearningError(a), LearningError(b))
            | (LearningReplayError(a), LearningReplayError(b))
            | (LearningConsolidationError(a), LearningConsolidationError(b))
            | (PerceptionSensorError(a), PerceptionSensorError(b))
            | (PerceptionCameraError(a), PerceptionCameraError(b))
            | (PerceptionAudioError(a), PerceptionAudioError(b))
            | (PluginLoadError(a), PluginLoadError(b))
            | (HalError(a), HalError(b))
            | (ActuatorError(a), ActuatorError(b))
            | (SafetyViolation(a), SafetyViolation(b))
            | (SafetyNotReady(a), SafetyNotReady(b))
            | (InternalRuntimeState(a), InternalRuntimeState(b))
            | (
                InternalRuntimeStateTransitionInvalid(a),
                InternalRuntimeStateTransitionInvalid(b),
            )
            | (InternalRuntimeBootFailed(a), InternalRuntimeBootFailed(b))
            | (InternalRuntimeShutdownFailed(a), InternalRuntimeShutdownFailed(b))
            | (InternalRuntimeEmergencyStopFailed(a), InternalRuntimeEmergencyStopFailed(b))
            | (InternalInitError(a), InternalInitError(b))
            | (InternalSerializationError(a), InternalSerializationError(b))
            | (InternalTransactionError(a), InternalTransactionError(b))
            | (InternalRecoveryImpossible(a), InternalRecoveryImpossible(b))
            | (InternalFatal(a), InternalFatal(b))
            | (InternalResourceUnavailable(a), InternalResourceUnavailable(b))
            | (InternalTimeout(a), InternalTimeout(b))
            | (InternalOther(a), InternalOther(b)) => a == b,
            (StorageChecksumMismatch, StorageChecksumMismatch)
            | (StorageCorruptGeneration, StorageCorruptGeneration)
            | (StorageTransactionConflict, StorageTransactionConflict)
            | (ActuatorRejected, ActuatorRejected) => true,
            (
                MemoryQuotaExceeded {
                    section: s1,
                    used: u1,
                    max: m1,
                },
                MemoryQuotaExceeded {
                    section: s2,
                    used: u2,
                    max: m2,
                },
            ) => s1 == s2 && u1 == u2 && m1 == m2,
            (
                PluginError {
                    plugin: p1,
                    reason: r1,
                },
                PluginError {
                    plugin: p2,
                    reason: r2,
                },
            ) => p1 == p2 && r1 == r2,
            (StorageIo(_), StorageIo(_)) => false,
            _ => false,
        }
    }
}

impl Eq for Error {}

impl Error {
    pub fn severity(&self) -> Severity {
        match self {
            Error::InternalFatal(_)
            | Error::StorageCorruptGeneration
            | Error::StorageChecksumMismatch
            | Error::StorageRecoveryFailed(_)
            | Error::InternalRuntimeBootFailed(_)
            | Error::InternalRuntimeEmergencyStopFailed(_)
            | Error::SafetyNotReady(_)
            | Error::BrainNotValid(_)
            | Error::InternalRecoveryImpossible(_) => Severity::Critical,

            Error::BrainValidation(_)
            | Error::BrainRecovery(_)
            | Error::MemoryQuotaExceeded { .. }
            | Error::MemoryAllocationFailed(_)
            | Error::MemoryGCFailed(_)
            | Error::SafetyViolation(_)
            | Error::StorageFsyncFailed(_)
            | Error::StorageBackupCorrupt(_)
            | Error::InternalRuntimeStateTransitionInvalid(_)
            | Error::ActuatorRejected => Severity::High,

            Error::ConfigInvalid(_)
            | Error::ConfigNotFound(_)
            | Error::StorageHeaderCorrupt(_)
            | Error::StorageWriteFailed(_)
            | Error::StorageTransactionConflict
            | Error::BrainError(_)
            | Error::ValidationInvalid(_)
            | Error::ValidationSchema(_)
            | Error::NeuralCore(_)
            | Error::NeuralCellError(_)
            | Error::NeuralColumnError(_)
            | Error::NeuralSynapseError(_)
            | Error::LearningError(_)
            | Error::LearningReplayError(_)
            | Error::LearningConsolidationError(_)
            | Error::PerceptionSensorError(_)
            | Error::PerceptionCameraError(_)
            | Error::PerceptionAudioError(_)
            | Error::PluginError { .. }
            | Error::PluginLoadError(_)
            | Error::HalError(_)
            | Error::ActuatorError(_)
            | Error::InternalRuntimeState(_)
            | Error::InternalRuntimeShutdownFailed(_)
            | Error::InternalInitError(_)
            | Error::InternalSerializationError(_)
            | Error::InternalTransactionError(_)
            | Error::InternalResourceUnavailable(_)
            | Error::InternalTimeout(_)
            | Error::InternalOther(_) => Severity::Medium,

            Error::StorageIo(_) => Severity::Low,
        }
    }

    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Error::InternalFatal(_)
                | Error::StorageCorruptGeneration
                | Error::StorageChecksumMismatch
                | Error::StorageRecoveryFailed(_)
                | Error::InternalRuntimeBootFailed(_)
                | Error::InternalRuntimeEmergencyStopFailed(_)
                | Error::SafetyNotReady(_)
                | Error::BrainNotValid(_)
                | Error::InternalRecoveryImpossible(_)
        )
    }

    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Unit tests for error taxonomy
/// Implements: AC §32, SD-16 §16.4
/// 12 real tests covering all 13 SD-16 classes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_critical() {
        assert_eq!(
            Error::InternalFatal("test".into()).severity(),
            Severity::Critical
        );
        assert_eq!(
            Error::StorageCorruptGeneration.severity(),
            Severity::Critical
        );
        assert_eq!(
            Error::StorageChecksumMismatch.severity(),
            Severity::Critical
        );
        assert_eq!(
            Error::BrainNotValid("test".into()).severity(),
            Severity::Critical
        );
        assert_eq!(
            Error::SafetyNotReady("test".into()).severity(),
            Severity::Critical
        );
    }

    #[test]
    fn test_severity_high() {
        assert_eq!(
            Error::BrainValidation("test".into()).severity(),
            Severity::High
        );
        assert_eq!(
            Error::MemoryQuotaExceeded {
                section: "cortex".into(),
                used: 100,
                max: 50,
            }
            .severity(),
            Severity::High
        );
        assert_eq!(
            Error::SafetyViolation("test".into()).severity(),
            Severity::High
        );
    }

    #[test]
    fn test_severity_medium() {
        assert_eq!(
            Error::ConfigInvalid("test".into()).severity(),
            Severity::Medium
        );
        assert_eq!(
            Error::NeuralCore("test".into()).severity(),
            Severity::Medium
        );
        assert_eq!(
            Error::PluginError {
                plugin: "p".into(),
                reason: "r".into(),
            }
            .severity(),
            Severity::Medium
        );
    }

    #[test]
    fn test_severity_low() {
        let io_err = std::io::Error::other("test");
        assert_eq!(Error::StorageIo(io_err).severity(), Severity::Low);
    }

    #[test]
    fn test_is_fatal_critical() {
        assert!(Error::InternalFatal("test".into()).is_fatal());
        assert!(Error::StorageCorruptGeneration.is_fatal());
        assert!(Error::StorageChecksumMismatch.is_fatal());
        assert!(Error::BrainNotValid("test".into()).is_fatal());
        assert!(Error::SafetyNotReady("test".into()).is_fatal());
        assert!(Error::InternalRecoveryImpossible("test".into()).is_fatal());
    }

    #[test]
    fn test_is_recoverable_non_fatal() {
        assert!(Error::ConfigInvalid("test".into()).is_recoverable());
        assert!(Error::MemoryQuotaExceeded {
            section: "c".into(),
            used: 1,
            max: 1,
        }
        .is_recoverable());
        assert!(Error::PluginError {
            plugin: "p".into(),
            reason: "r".into(),
        }
        .is_recoverable());
    }

    #[test]
    fn test_display_contains_anr_code() {
        let err = Error::ConfigInvalid("missing field".into());
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-CONFIG-001"));
        assert!(msg.contains("missing field"));
    }

    #[test]
    fn test_display_memory_quota_fields() {
        let err = Error::MemoryQuotaExceeded {
            section: "cortex".into(),
            used: 100,
            max: 50,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-MEMORY-001"));
        assert!(msg.contains("cortex"));
        assert!(msg.contains("100"));
        assert!(msg.contains("50"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err: Error = io_err.into();
        match err {
            Error::StorageIo(_) => {}
            _ => panic!("expected StorageIo variant"),
        }
    }

    #[test]
    fn test_error_taxonomy_coverage() {
        let _ = Error::ConfigInvalid("test".into());
        let _ = Error::StorageHeaderCorrupt("test".into());
        let _ = Error::BrainNotValid("test".into());
        let _ = Error::ValidationInvalid("test".into());
        let _ = Error::MemoryQuotaExceeded {
            section: "s".into(),
            used: 0,
            max: 0,
        };
        let _ = Error::NeuralCore("test".into());
        let _ = Error::LearningError("test".into());
        let _ = Error::PerceptionSensorError("test".into());
        let _ = Error::PluginError {
            plugin: "p".into(),
            reason: "r".into(),
        };
        let _ = Error::HalError("test".into());
        let _ = Error::ActuatorRejected;
        let _ = Error::SafetyViolation("test".into());
        let _ = Error::InternalFatal("test".into());
    }

    #[test]
    fn test_partial_eq() {
        let err1 = Error::ConfigInvalid("test".into());
        let err2 = Error::ConfigInvalid("test".into());
        assert_eq!(err1, err2);
        assert_eq!(err1.severity(), err2.severity());
    }

    #[test]
    fn test_display_plugin_error_fields() {
        let err = Error::PluginError {
            plugin: "camera".into(),
            reason: "timeout".into(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-PLUGIN-001"));
        assert!(msg.contains("camera"));
        assert!(msg.contains("timeout"));
    }
}
