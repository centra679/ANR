use anr::error::{Error, Severity};
use std::error::Error as StdError;

fn io_err(msg: &str) -> std::io::Error {
    std::io::Error::other(msg)
}

// ── Display / ANR-E-* code tests ──────────────────────────────────────────

#[test]
fn tc_u_error_display_config_invalid() {
    let e = Error::ConfigInvalid("bad".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-CONFIG-001"));
    assert!(s.contains("bad"));
}

#[test]
fn tc_u_error_display_config_not_found() {
    let e = Error::ConfigNotFound("/etc/anr.toml".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-CONFIG-002"));
    assert!(s.contains("/etc/anr.toml"));
}

#[test]
fn tc_u_error_display_storage_header_corrupt() {
    let e = Error::StorageHeaderCorrupt("magic".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-001"));
    assert!(s.contains("magic"));
}

#[test]
fn tc_u_error_display_storage_checksum_mismatch() {
    let e = Error::StorageChecksumMismatch;
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-002"));
}

#[test]
fn tc_u_error_display_storage_corrupt_generation() {
    let e = Error::StorageCorruptGeneration;
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-003"));
}

#[test]
fn tc_u_error_display_storage_io() {
    let e = Error::StorageIo(io_err("disk full"));
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-004"));
    assert!(s.contains("disk full"));
}

#[test]
fn tc_u_error_display_storage_write_failed() {
    let e = Error::StorageWriteFailed("write err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-005"));
    assert!(s.contains("write err"));
}

#[test]
fn tc_u_error_display_storage_fsync_failed() {
    let e = Error::StorageFsyncFailed("fsync err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-006"));
    assert!(s.contains("fsync err"));
}

#[test]
fn tc_u_error_display_storage_backup_corrupt() {
    let e = Error::StorageBackupCorrupt("bk err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-007"));
    assert!(s.contains("bk err"));
}

#[test]
fn tc_u_error_display_storage_transaction_conflict() {
    let e = Error::StorageTransactionConflict;
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-008"));
}

#[test]
fn tc_u_error_display_storage_recovery_failed() {
    let e = Error::StorageRecoveryFailed("rec err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-STORAGE-009"));
    assert!(s.contains("rec err"));
}

#[test]
fn tc_u_error_display_brain_not_valid() {
    let e = Error::BrainNotValid("bad brain".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-BRAIN-001"));
    assert!(s.contains("bad brain"));
}

#[test]
fn tc_u_error_display_brain_error() {
    let e = Error::BrainError("read err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-BRAIN-002"));
    assert!(s.contains("read err"));
}

#[test]
fn tc_u_error_display_brain_validation() {
    let e = Error::BrainValidation("schema err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-BRAIN-003"));
    assert!(s.contains("schema err"));
}

#[test]
fn tc_u_error_display_brain_recovery() {
    let e = Error::BrainRecovery("recover err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-BRAIN-004"));
    assert!(s.contains("recover err"));
}

#[test]
fn tc_u_error_display_validation_invalid() {
    let e = Error::ValidationInvalid("null field".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-VALIDATION-001"));
    assert!(s.contains("null field"));
}

#[test]
fn tc_u_error_display_validation_schema() {
    let e = Error::ValidationSchema("v2 mismatch".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-VALIDATION-002"));
    assert!(s.contains("v2 mismatch"));
}

#[test]
fn tc_u_error_display_memory_quota_exceeded() {
    let e = Error::MemoryQuotaExceeded {
        section: "cortex".into(),
        used: 1024,
        max: 512,
    };
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-MEMORY-001"));
    assert!(s.contains("cortex"));
    assert!(s.contains("1024"));
    assert!(s.contains("512"));
}

#[test]
fn tc_u_error_display_memory_allocation_failed() {
    let e = Error::MemoryAllocationFailed("oom".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-MEMORY-002"));
    assert!(s.contains("oom"));
}

#[test]
fn tc_u_error_display_memory_gc_failed() {
    let e = Error::MemoryGCFailed("gc err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-MEMORY-003"));
    assert!(s.contains("gc err"));
}

#[test]
fn tc_u_error_display_neural_core() {
    let e = Error::NeuralCore("init err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-NEURAL-001"));
    assert!(s.contains("init err"));
}

#[test]
fn tc_u_error_display_neural_cell_error() {
    let e = Error::NeuralCellError("cell err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-NEURAL-002"));
    assert!(s.contains("cell err"));
}

#[test]
fn tc_u_error_display_neural_column_error() {
    let e = Error::NeuralColumnError("col err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-NEURAL-003"));
    assert!(s.contains("col err"));
}

#[test]
fn tc_u_error_display_neural_synapse_error() {
    let e = Error::NeuralSynapseError("syn err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-NEURAL-004"));
    assert!(s.contains("syn err"));
}

#[test]
fn tc_u_error_display_learning_error() {
    let e = Error::LearningError("learn err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-LEARNING-001"));
    assert!(s.contains("learn err"));
}

#[test]
fn tc_u_error_display_learning_replay_error() {
    let e = Error::LearningReplayError("replay err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-LEARNING-002"));
    assert!(s.contains("replay err"));
}

#[test]
fn tc_u_error_display_learning_consolidation_error() {
    let e = Error::LearningConsolidationError("cons err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-LEARNING-003"));
    assert!(s.contains("cons err"));
}

#[test]
fn tc_u_error_display_perception_sensor_error() {
    let e = Error::PerceptionSensorError("sensor err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-PERCEPTION-001"));
    assert!(s.contains("sensor err"));
}

#[test]
fn tc_u_error_display_perception_camera_error() {
    let e = Error::PerceptionCameraError("cam err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-PERCEPTION-002"));
    assert!(s.contains("cam err"));
}

#[test]
fn tc_u_error_display_perception_audio_error() {
    let e = Error::PerceptionAudioError("audio err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-PERCEPTION-003"));
    assert!(s.contains("audio err"));
}

#[test]
fn tc_u_error_display_plugin_error() {
    let e = Error::PluginError {
        plugin: "cam".into(),
        reason: "timeout".into(),
    };
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-PLUGIN-001"));
    assert!(s.contains("cam"));
    assert!(s.contains("timeout"));
}

#[test]
fn tc_u_error_display_plugin_load_error() {
    let e = Error::PluginLoadError("load err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-PLUGIN-002"));
    assert!(s.contains("load err"));
}

#[test]
fn tc_u_error_display_hal_error() {
    let e = Error::HalError("hal err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-HAL-001"));
    assert!(s.contains("hal err"));
}

#[test]
fn tc_u_error_display_actuator_rejected() {
    let e = Error::ActuatorRejected;
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-ACTUATOR-001"));
}

#[test]
fn tc_u_error_display_actuator_error() {
    let e = Error::ActuatorError("act err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-ACTUATOR-002"));
    assert!(s.contains("act err"));
}

#[test]
fn tc_u_error_display_safety_violation() {
    let e = Error::SafetyViolation("safety err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-SAFETY-001"));
    assert!(s.contains("safety err"));
}

#[test]
fn tc_u_error_display_safety_not_ready() {
    let e = Error::SafetyNotReady("not ready".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-SAFETY-002"));
    assert!(s.contains("not ready"));
}

#[test]
fn tc_u_error_display_internal_runtime_state() {
    let e = Error::InternalRuntimeState("state err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-001"));
    assert!(s.contains("state err"));
}

#[test]
fn tc_u_error_display_internal_runtime_state_transition_invalid() {
    let e = Error::InternalRuntimeStateTransitionInvalid("bad transition".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-002"));
    assert!(s.contains("bad transition"));
}

#[test]
fn tc_u_error_display_internal_runtime_boot_failed() {
    let e = Error::InternalRuntimeBootFailed("boot err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-003"));
    assert!(s.contains("boot err"));
}

#[test]
fn tc_u_error_display_internal_runtime_shutdown_failed() {
    let e = Error::InternalRuntimeShutdownFailed("shutdown err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-004"));
    assert!(s.contains("shutdown err"));
}

#[test]
fn tc_u_error_display_internal_runtime_emergency_stop_failed() {
    let e = Error::InternalRuntimeEmergencyStopFailed("estop err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-005"));
    assert!(s.contains("estop err"));
}

#[test]
fn tc_u_error_display_internal_init_error() {
    let e = Error::InternalInitError("init err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-006"));
    assert!(s.contains("init err"));
}

#[test]
fn tc_u_error_display_internal_serialization_error() {
    let e = Error::InternalSerializationError("serde err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-007"));
    assert!(s.contains("serde err"));
}

#[test]
fn tc_u_error_display_internal_transaction_error() {
    let e = Error::InternalTransactionError("tx err".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-008"));
    assert!(s.contains("tx err"));
}

#[test]
fn tc_u_error_display_internal_recovery_impossible() {
    let e = Error::InternalRecoveryImpossible("dead end".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-009"));
    assert!(s.contains("dead end"));
}

#[test]
fn tc_u_error_display_internal_fatal() {
    let e = Error::InternalFatal("fatal".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-010"));
    assert!(s.contains("fatal"));
}

#[test]
fn tc_u_error_display_internal_resource_unavailable() {
    let e = Error::InternalResourceUnavailable("resource".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-011"));
    assert!(s.contains("resource"));
}

#[test]
fn tc_u_error_display_internal_timeout() {
    let e = Error::InternalTimeout("timeout".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-012"));
    assert!(s.contains("timeout"));
}

#[test]
fn tc_u_error_display_internal_other() {
    let e = Error::InternalOther("other".into());
    let s = format!("{}", e);
    assert!(s.contains("ANR-E-INTERNAL-013"));
    assert!(s.contains("other"));
}

// ── From<io::Error> conversion ─────────────────────────────────────────────

#[test]
fn tc_u_error_from_io_error_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let err: Error = io_err.into();
    match err {
        Error::StorageIo(inner) => {
            assert_eq!(inner.kind(), std::io::ErrorKind::NotFound);
        }
        _ => panic!("expected StorageIo variant"),
    }
}

#[test]
fn tc_u_error_from_io_error_display() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    let err: Error = io_err.into();
    let s = format!("{}", err);
    assert!(s.contains("ANR-E-STORAGE-004"));
    assert!(s.contains("denied"));
}

#[test]
fn tc_u_error_from_io_error_other_kind() {
    let io_err = io_err("custom io");
    let err: Error = io_err.into();
    match err {
        Error::StorageIo(_) => {}
        _ => panic!("expected StorageIo variant"),
    }
    assert_eq!(err.severity(), Severity::Low);
}

// ── Severity levels for EVERY variant ──────────────────────────────────────

#[test]
fn tc_u_error_severity_all_critical() {
    let criticals = [
        Error::InternalFatal("t".into()),
        Error::StorageCorruptGeneration,
        Error::StorageChecksumMismatch,
        Error::StorageRecoveryFailed("t".into()),
        Error::InternalRuntimeBootFailed("t".into()),
        Error::InternalRuntimeEmergencyStopFailed("t".into()),
        Error::SafetyNotReady("t".into()),
        Error::BrainNotValid("t".into()),
        Error::InternalRecoveryImpossible("t".into()),
    ];
    for e in &criticals {
        assert_eq!(
            e.severity(),
            Severity::Critical,
            "expected Critical for {:?}",
            e
        );
    }
}

#[test]
fn tc_u_error_severity_all_high() {
    let highs = [
        Error::BrainValidation("t".into()),
        Error::BrainRecovery("t".into()),
        Error::MemoryQuotaExceeded {
            section: "s".into(),
            used: 1,
            max: 1,
        },
        Error::MemoryAllocationFailed("t".into()),
        Error::MemoryGCFailed("t".into()),
        Error::SafetyViolation("t".into()),
        Error::StorageFsyncFailed("t".into()),
        Error::StorageBackupCorrupt("t".into()),
        Error::InternalRuntimeStateTransitionInvalid("t".into()),
        Error::ActuatorRejected,
    ];
    for e in &highs {
        assert_eq!(e.severity(), Severity::High, "expected High for {:?}", e);
    }
}

#[test]
fn tc_u_error_severity_all_medium() {
    let mediums = [
        Error::ConfigInvalid("t".into()),
        Error::ConfigNotFound("t".into()),
        Error::StorageHeaderCorrupt("t".into()),
        Error::StorageWriteFailed("t".into()),
        Error::StorageTransactionConflict,
        Error::BrainError("t".into()),
        Error::ValidationInvalid("t".into()),
        Error::ValidationSchema("t".into()),
        Error::NeuralCore("t".into()),
        Error::NeuralCellError("t".into()),
        Error::NeuralColumnError("t".into()),
        Error::NeuralSynapseError("t".into()),
        Error::LearningError("t".into()),
        Error::LearningReplayError("t".into()),
        Error::LearningConsolidationError("t".into()),
        Error::PerceptionSensorError("t".into()),
        Error::PerceptionCameraError("t".into()),
        Error::PerceptionAudioError("t".into()),
        Error::PluginError {
            plugin: "p".into(),
            reason: "r".into(),
        },
        Error::PluginLoadError("t".into()),
        Error::HalError("t".into()),
        Error::ActuatorError("t".into()),
        Error::InternalRuntimeState("t".into()),
        Error::InternalRuntimeShutdownFailed("t".into()),
        Error::InternalInitError("t".into()),
        Error::InternalSerializationError("t".into()),
        Error::InternalTransactionError("t".into()),
        Error::InternalResourceUnavailable("t".into()),
        Error::InternalTimeout("t".into()),
        Error::InternalOther("t".into()),
    ];
    for e in &mediums {
        assert_eq!(
            e.severity(),
            Severity::Medium,
            "expected Medium for {:?}",
            e
        );
    }
}

#[test]
fn tc_u_error_severity_storage_io_is_low() {
    assert_eq!(Error::StorageIo(io_err("t")).severity(), Severity::Low);
}

// ── is_fatal() for EVERY variant ───────────────────────────────────────────

#[test]
fn tc_u_error_is_fatal_true() {
    let fatal = [
        Error::InternalFatal("t".into()),
        Error::StorageCorruptGeneration,
        Error::StorageChecksumMismatch,
        Error::StorageRecoveryFailed("t".into()),
        Error::InternalRuntimeBootFailed("t".into()),
        Error::InternalRuntimeEmergencyStopFailed("t".into()),
        Error::SafetyNotReady("t".into()),
        Error::BrainNotValid("t".into()),
        Error::InternalRecoveryImpossible("t".into()),
    ];
    for e in &fatal {
        assert!(e.is_fatal(), "expected fatal for {:?}", e);
    }
}

#[test]
fn tc_u_error_is_fatal_false() {
    let non_fatal = [
        Error::ConfigInvalid("t".into()),
        Error::ConfigNotFound("t".into()),
        Error::StorageHeaderCorrupt("t".into()),
        Error::StorageWriteFailed("t".into()),
        Error::StorageFsyncFailed("t".into()),
        Error::StorageBackupCorrupt("t".into()),
        Error::StorageTransactionConflict,
        Error::StorageIo(io_err("t")),
        Error::BrainError("t".into()),
        Error::BrainValidation("t".into()),
        Error::BrainRecovery("t".into()),
        Error::ValidationInvalid("t".into()),
        Error::ValidationSchema("t".into()),
        Error::MemoryQuotaExceeded {
            section: "s".into(),
            used: 1,
            max: 1,
        },
        Error::MemoryAllocationFailed("t".into()),
        Error::MemoryGCFailed("t".into()),
        Error::NeuralCore("t".into()),
        Error::NeuralCellError("t".into()),
        Error::NeuralColumnError("t".into()),
        Error::NeuralSynapseError("t".into()),
        Error::LearningError("t".into()),
        Error::LearningReplayError("t".into()),
        Error::LearningConsolidationError("t".into()),
        Error::PerceptionSensorError("t".into()),
        Error::PerceptionCameraError("t".into()),
        Error::PerceptionAudioError("t".into()),
        Error::PluginError {
            plugin: "p".into(),
            reason: "r".into(),
        },
        Error::PluginLoadError("t".into()),
        Error::HalError("t".into()),
        Error::ActuatorRejected,
        Error::ActuatorError("t".into()),
        Error::SafetyViolation("t".into()),
        Error::InternalRuntimeState("t".into()),
        Error::InternalRuntimeStateTransitionInvalid("t".into()),
        Error::InternalRuntimeShutdownFailed("t".into()),
        Error::InternalInitError("t".into()),
        Error::InternalSerializationError("t".into()),
        Error::InternalTransactionError("t".into()),
        Error::InternalResourceUnavailable("t".into()),
        Error::InternalTimeout("t".into()),
        Error::InternalOther("t".into()),
    ];
    for e in &non_fatal {
        assert!(!e.is_fatal(), "expected non-fatal for {:?}", e);
    }
}

// ── is_recoverable() for EVERY variant ─────────────────────────────────────

#[test]
fn tc_u_error_is_recoverable_true_for_non_fatal() {
    let recoverable = [
        Error::ConfigInvalid("t".into()),
        Error::StorageIo(io_err("t")),
        Error::NeuralCore("t".into()),
        Error::LearningError("t".into()),
        Error::HalError("t".into()),
        Error::InternalTimeout("t".into()),
    ];
    for e in &recoverable {
        assert!(e.is_recoverable(), "expected recoverable for {:?}", e);
    }
}

#[test]
fn tc_u_error_is_recoverable_false_for_fatal() {
    let unrecoverable = [
        Error::InternalFatal("t".into()),
        Error::StorageCorruptGeneration,
        Error::StorageChecksumMismatch,
        Error::SafetyNotReady("t".into()),
        Error::BrainNotValid("t".into()),
    ];
    for e in &unrecoverable {
        assert!(!e.is_recoverable(), "expected unrecoverable for {:?}", e);
    }
}

// ── PartialEq / Eq ─────────────────────────────────────────────────────────

#[test]
fn tc_u_error_partial_eq_same_variant_same_msg() {
    assert_eq!(
        Error::ConfigInvalid("a".into()),
        Error::ConfigInvalid("a".into())
    );
}

#[test]
fn tc_u_error_partial_eq_same_variant_diff_msg() {
    assert_ne!(
        Error::ConfigInvalid("a".into()),
        Error::ConfigInvalid("b".into())
    );
}

#[test]
fn tc_u_error_partial_eq_diff_variant_same_msg() {
    assert_ne!(
        Error::ConfigInvalid("a".into()),
        Error::BrainError("a".into())
    );
}

#[test]
fn tc_u_error_partial_eq_unit_variants() {
    assert_eq!(
        Error::StorageChecksumMismatch,
        Error::StorageChecksumMismatch
    );
    assert_eq!(
        Error::StorageCorruptGeneration,
        Error::StorageCorruptGeneration
    );
    assert_eq!(
        Error::StorageTransactionConflict,
        Error::StorageTransactionConflict
    );
    assert_eq!(Error::ActuatorRejected, Error::ActuatorRejected);
}

#[test]
fn tc_u_error_partial_eq_memory_quota_exceeded() {
    let a = Error::MemoryQuotaExceeded {
        section: "c".into(),
        used: 1,
        max: 2,
    };
    let b = Error::MemoryQuotaExceeded {
        section: "c".into(),
        used: 1,
        max: 2,
    };
    assert_eq!(a, b);
}

#[test]
fn tc_u_error_partial_eq_memory_quota_exceeded_diff() {
    let a = Error::MemoryQuotaExceeded {
        section: "c".into(),
        used: 1,
        max: 2,
    };
    let b = Error::MemoryQuotaExceeded {
        section: "c".into(),
        used: 1,
        max: 3,
    };
    assert_ne!(a, b);
}

#[test]
fn tc_u_error_partial_eq_plugin_error() {
    let a = Error::PluginError {
        plugin: "p".into(),
        reason: "r".into(),
    };
    let b = Error::PluginError {
        plugin: "p".into(),
        reason: "r".into(),
    };
    assert_eq!(a, b);
}

#[test]
fn tc_u_error_partial_eq_plugin_error_diff() {
    let a = Error::PluginError {
        plugin: "p".into(),
        reason: "r".into(),
    };
    let b = Error::PluginError {
        plugin: "p".into(),
        reason: "s".into(),
    };
    assert_ne!(a, b);
}

#[test]
fn tc_u_error_partial_eq_storage_io_always_false() {
    let a = Error::StorageIo(io_err("x"));
    let b = Error::StorageIo(io_err("x"));
    assert_ne!(a, b);
}

// ── Error chaining (source()) ──────────────────────────────────────────────

#[test]
fn tc_u_error_source_storage_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
    let err: Error = io_err.into();
    assert!(err.source().is_some());
    let src = err.source().unwrap();
    assert_eq!(src.to_string(), "gone");
}

#[test]
fn tc_u_error_source_string_variants_none() {
    let err = Error::ConfigInvalid("bad".into());
    assert!(err.source().is_none());
}

#[test]
fn tc_u_error_source_unit_variants_none() {
    assert!(Error::StorageChecksumMismatch.source().is_none());
    assert!(Error::StorageCorruptGeneration.source().is_none());
    assert!(Error::StorageTransactionConflict.source().is_none());
    assert!(Error::ActuatorRejected.source().is_none());
}

// ── Severity as_str / Display ──────────────────────────────────────────────

#[test]
fn tc_u_error_severity_as_str() {
    assert_eq!(Severity::Low.as_str(), "LOW");
    assert_eq!(Severity::Medium.as_str(), "MEDIUM");
    assert_eq!(Severity::High.as_str(), "HIGH");
    assert_eq!(Severity::Critical.as_str(), "CRITICAL");
}

#[test]
fn tc_u_error_severity_display() {
    assert_eq!(format!("{}", Severity::Low), "LOW");
    assert_eq!(format!("{}", Severity::Medium), "MEDIUM");
    assert_eq!(format!("{}", Severity::High), "HIGH");
    assert_eq!(format!("{}", Severity::Critical), "CRITICAL");
}

// ── Variant count verification ─────────────────────────────────────────────

#[test]
fn tc_u_error_total_variant_count() {
    let all = [
        Error::ConfigInvalid("".into()),
        Error::ConfigNotFound("".into()),
        Error::StorageHeaderCorrupt("".into()),
        Error::StorageChecksumMismatch,
        Error::StorageCorruptGeneration,
        Error::StorageIo(io_err("")),
        Error::StorageWriteFailed("".into()),
        Error::StorageFsyncFailed("".into()),
        Error::StorageBackupCorrupt("".into()),
        Error::StorageTransactionConflict,
        Error::StorageRecoveryFailed("".into()),
        Error::BrainNotValid("".into()),
        Error::BrainError("".into()),
        Error::BrainValidation("".into()),
        Error::BrainRecovery("".into()),
        Error::ValidationInvalid("".into()),
        Error::ValidationSchema("".into()),
        Error::MemoryQuotaExceeded {
            section: "".into(),
            used: 0,
            max: 0,
        },
        Error::MemoryAllocationFailed("".into()),
        Error::MemoryGCFailed("".into()),
        Error::NeuralCore("".into()),
        Error::NeuralCellError("".into()),
        Error::NeuralColumnError("".into()),
        Error::NeuralSynapseError("".into()),
        Error::LearningError("".into()),
        Error::LearningReplayError("".into()),
        Error::LearningConsolidationError("".into()),
        Error::PerceptionSensorError("".into()),
        Error::PerceptionCameraError("".into()),
        Error::PerceptionAudioError("".into()),
        Error::PluginError {
            plugin: "".into(),
            reason: "".into(),
        },
        Error::PluginLoadError("".into()),
        Error::HalError("".into()),
        Error::ActuatorRejected,
        Error::ActuatorError("".into()),
        Error::SafetyViolation("".into()),
        Error::SafetyNotReady("".into()),
        Error::InternalRuntimeState("".into()),
        Error::InternalRuntimeStateTransitionInvalid("".into()),
        Error::InternalRuntimeBootFailed("".into()),
        Error::InternalRuntimeShutdownFailed("".into()),
        Error::InternalRuntimeEmergencyStopFailed("".into()),
        Error::InternalInitError("".into()),
        Error::InternalSerializationError("".into()),
        Error::InternalTransactionError("".into()),
        Error::InternalRecoveryImpossible("".into()),
        Error::InternalFatal("".into()),
        Error::InternalResourceUnavailable("".into()),
        Error::InternalTimeout("".into()),
        Error::InternalOther("".into()),
    ];
    assert_eq!(all.len(), 50, "expected 50 variants total");
}

// ── Error is Send + Sync ──────────────────────────────────────────────────

#[test]
fn tc_u_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}
