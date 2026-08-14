/// CLI Argument Parsing and Command Routing
/// Aligns with: AC §56, DEC-005
use crate::error::{Error, Result};
use clap::Parser;
use std::path::PathBuf;

/// ANR - Autonomous Neural Runtime
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to brain.anr file
    #[arg(short, long, default_value = "/opt/anr/brain.anr")]
    pub brain: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug, Clone)]
pub enum Commands {
    /// Run the autonomous runtime
    Run {
        /// Enable maintenance mode
        #[arg(long)]
        maintenance: bool,
    },
    /// Verify brain.anr integrity
    Verify {
        /// Brain file path
        brain: PathBuf,
    },
    /// Build brain from seed
    Build {
        /// Seed file path
        seed: PathBuf,
        /// Output brain path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Diagnostics
    Diag {
        /// Diagnostic action
        #[arg(value_parser = ["status", "memory", "storage", "neural", "safety"])]
        action: String,
    },
}

impl Cli {
    /// Parse CLI arguments.
    /// On parse failure, clap automatically prints help/error.
    pub fn parse() -> Self {
        Self::parse_from(std::env::args_os())
    }

    /// Return list of available commands for error messages.
    pub fn available_commands() -> Vec<&'static str> {
        vec!["run", "verify", "build", "diag"]
    }

    /// Validate that the parsed CLI is semantically valid.
    /// Returns structured error if unavailable command is requested.
    pub fn validate(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Diag { action }) => {
                let valid =
                    ["status", "memory", "storage", "neural", "safety"].contains(&action.as_str());
                if !valid {
                    return Err(Error::ConfigInvalid(format!(
                        "Unknown diagnostic action: '{}'. Available actions: {}",
                        action,
                        ["status", "memory", "storage", "neural", "safety"].join(", ")
                    )));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_commands_list() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"run"));
        assert!(cmds.contains(&"verify"));
        assert!(cmds.contains(&"build"));
        assert!(cmds.contains(&"diag"));
        assert_eq!(cmds.len(), 4);
    }

    #[test]
    fn test_validate_run_command() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Run { maintenance: false }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_verify_command() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Verify {
                brain: PathBuf::from("/tmp/brain.anr"),
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_build_command() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Build {
                seed: PathBuf::from("/tmp/seed.toml"),
                output: PathBuf::from("/tmp/brain.anr"),
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_diag_valid_action() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Diag {
                action: "status".into(),
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_diag_invalid_action_returns_error() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Diag {
                action: "unknown".into(),
            }),
        };
        let err = cli.validate().unwrap_err();
        match err {
            Error::ConfigInvalid(msg) => {
                assert!(msg.contains("unknown"));
                assert!(msg.contains("status"));
                assert!(msg.contains("memory"));
            }
            _ => panic!("expected ConfigInvalid"),
        }
    }

    #[test]
    fn test_validate_diag_invalid_action_error_code() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Diag {
                action: "bogus".into(),
            }),
        };
        let err = cli.validate().unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-CONFIG-001"));
    }

    #[test]
    fn test_validate_no_command() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: None,
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_available_commands_count() {
        let cmds = Cli::available_commands();
        assert_eq!(cmds.len(), 4);
    }

    #[test]
    fn test_validate_diag_all_valid_actions() {
        for action in ["status", "memory", "storage", "neural", "safety"] {
            let cli = Cli {
                brain: PathBuf::from("/opt/anr/brain.anr"),
                config: None,
                verbose: false,
                command: Some(Commands::Diag {
                    action: action.into(),
                }),
            };
            assert!(cli.validate().is_ok(), "action {} should be valid", action);
        }
    }

    #[test]
    fn test_validate_diag_empty_action() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Diag { action: "".into() }),
        };
        let err = cli.validate().unwrap_err();
        match err {
            Error::ConfigInvalid(_) => {}
            _ => panic!("expected ConfigInvalid"),
        }
    }

    #[test]
    fn test_validate_diag_case_sensitive() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Diag {
                action: "STATUS".into(),
            }),
        };
        let err = cli.validate().unwrap_err();
        match err {
            Error::ConfigInvalid(_) => {}
            _ => panic!("expected ConfigInvalid"),
        }
    }

    #[test]
    fn test_available_commands_contains_run() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"run"));
    }

    #[test]
    fn test_available_commands_contains_verify() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"verify"));
    }

    #[test]
    fn test_available_commands_contains_build() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"build"));
    }

    #[test]
    fn test_available_commands_contains_diag() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"diag"));
    }
}
