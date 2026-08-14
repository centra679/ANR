/// Runtime Configuration
/// Aligns with: AC §37, DEC-004
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Safe default runtime configuration.
/// All fields are chosen to minimize risk on first boot.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RuntimeConfig {
    pub state_trace: bool,
    pub shutdown_timeout_ms: u64,
    pub emergency_stop_timeout_ms: u64,
    pub allow_volatile_degraded_mode: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            state_trace: false,
            shutdown_timeout_ms: 5000,
            emergency_stop_timeout_ms: 100,
            allow_volatile_degraded_mode: false,
        }
    }
}

impl RuntimeConfig {
    /// Load configuration from a TOML file.
    /// Returns error if file missing or TOML invalid.
    pub fn load_from_toml(path: &PathBuf) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).map_err(|e| Error::ConfigNotFound(e.to_string()))?;
        let config: Self =
            toml::from_str(&content).map_err(|e| Error::ConfigInvalid(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration constraints.
    /// Reject values that violate safety invariants.
    pub fn validate(&self) -> Result<()> {
        if self.emergency_stop_timeout_ms == 0 {
            return Err(Error::ConfigInvalid(
                "emergency_stop_timeout_ms must be > 0".into(),
            ));
        }
        if self.shutdown_timeout_ms == 0 {
            return Err(Error::ConfigInvalid(
                "shutdown_timeout_ms must be > 0".into(),
            ));
        }
        if self.emergency_stop_timeout_ms > self.shutdown_timeout_ms {
            return Err(Error::ConfigInvalid(
                "emergency_stop_timeout_ms must be <= shutdown_timeout_ms".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_safe() {
        let cfg = RuntimeConfig::default();
        assert!(!cfg.allow_volatile_degraded_mode);
        assert_eq!(cfg.emergency_stop_timeout_ms, 100);
        assert_eq!(cfg.shutdown_timeout_ms, 5000);
        assert!(!cfg.state_trace);
    }

    #[test]
    fn test_validate_rejects_zero_emergency_timeout() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 0,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_zero_shutdown_timeout() {
        let cfg = RuntimeConfig {
            shutdown_timeout_ms: 0,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_inverted_timeouts() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 6000,
            shutdown_timeout_ms: 5000,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_accepts_valid_config() {
        let cfg = RuntimeConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_load_from_toml_valid() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            r#"
state_trace = true
shutdown_timeout_ms = 3000
emergency_stop_timeout_ms = 50
allow_volatile_degraded_mode = false
"#,
        )
        .unwrap();
        let cfg = RuntimeConfig::load_from_toml(&tmp.path().to_path_buf()).unwrap();
        assert!(cfg.state_trace);
        assert_eq!(cfg.shutdown_timeout_ms, 3000);
        assert_eq!(cfg.emergency_stop_timeout_ms, 50);
    }

    #[test]
    fn test_load_from_toml_invalid_toml() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), r#"not valid toml {{{"#).unwrap();
        let err = RuntimeConfig::load_from_toml(&tmp.path().to_path_buf()).unwrap_err();
        match err {
            Error::ConfigInvalid(_) => {}
            _ => panic!("expected ConfigInvalid"),
        }
    }

    #[test]
    fn test_load_from_toml_missing_file() {
        let err =
            RuntimeConfig::load_from_toml(&PathBuf::from("/nonexistent/config.toml")).unwrap_err();
        match err {
            Error::ConfigNotFound(_) => {}
            _ => panic!("expected ConfigNotFound"),
        }
    }

    #[test]
    fn test_load_from_toml_validation_fails() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            r#"
shutdown_timeout_ms = 0
emergency_stop_timeout_ms = 0
"#,
        )
        .unwrap();
        let err = RuntimeConfig::load_from_toml(&tmp.path().to_path_buf()).unwrap_err();
        match err {
            Error::ConfigInvalid(_) => {}
            _ => panic!("expected ConfigInvalid"),
        }
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original = RuntimeConfig::default();
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: RuntimeConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_config_partial_eq() {
        let a = RuntimeConfig::default();
        let b = RuntimeConfig::default();
        assert_eq!(a, b);
    }

    #[test]
    fn test_config_fields_are_safe_defaults() {
        let cfg = RuntimeConfig::default();
        assert!(!cfg.allow_volatile_degraded_mode);
        assert!(cfg.emergency_stop_timeout_ms <= 1000);
        assert!(cfg.shutdown_timeout_ms >= 1000);
    }

    #[test]
    fn test_config_validate_error_message_contains_code() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 0,
            ..RuntimeConfig::default()
        };
        let err = cfg.validate().unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-CONFIG-001"));
    }

    #[test]
    fn test_load_from_toml_not_found_error_message() {
        let err = RuntimeConfig::load_from_toml(&PathBuf::from("/nonexistent.toml")).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-CONFIG-002"));
    }

    #[test]
    fn test_load_from_toml_with_whitespace() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            r#"
state_trace = true
shutdown_timeout_ms = 3000
emergency_stop_timeout_ms = 50
allow_volatile_degraded_mode = false
"#,
        )
        .unwrap();
        let cfg = RuntimeConfig::load_from_toml(&tmp.path().to_path_buf()).unwrap();
        assert!(cfg.state_trace);
    }

    #[test]
    fn test_load_from_toml_with_comments() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            r#"# This is a comment
state_trace = false
shutdown_timeout_ms = 5000
emergency_stop_timeout_ms = 100
allow_volatile_degraded_mode = false
"#,
        )
        .unwrap();
        let cfg = RuntimeConfig::load_from_toml(&tmp.path().to_path_buf()).unwrap();
        assert!(!cfg.state_trace);
    }

    #[test]
    fn test_config_default_emergency_timeout() {
        let cfg = RuntimeConfig::default();
        assert_eq!(cfg.emergency_stop_timeout_ms, 100);
    }

    #[test]
    fn test_config_default_shutdown_timeout() {
        let cfg = RuntimeConfig::default();
        assert_eq!(cfg.shutdown_timeout_ms, 5000);
    }

    #[test]
    fn test_validate_rejects_negative_timeouts() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 0,
            shutdown_timeout_ms: 0,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_emergency_greater_than_shutdown() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 6000,
            shutdown_timeout_ms: 5000,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_default_config_passes() {
        let cfg = RuntimeConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_state_trace_can_be_true() {
        let cfg = RuntimeConfig {
            state_trace: true,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_allow_volatile_degraded_mode_can_be_true() {
        let cfg = RuntimeConfig {
            allow_volatile_degraded_mode: true,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_emergency_timeout_exactly_one() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 1,
            shutdown_timeout_ms: 5000,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_shutdown_timeout_exactly_one() {
        let cfg = RuntimeConfig {
            emergency_stop_timeout_ms: 1,
            shutdown_timeout_ms: 1,
            ..RuntimeConfig::default()
        };
        assert!(cfg.validate().is_ok());
    }
}
