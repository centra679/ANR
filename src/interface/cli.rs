/// CLI Argument Parsing and Command Routing
/// Aligns with: AC §56, DEC-005
use crate::error::{Error, Result};
use crate::interface::diagnostics;
use crate::storage::{self, BrainHeader, InspectFormat};
use clap::Parser;
use std::path::PathBuf;

pub async fn run() -> crate::Result<()> {
    let args = Cli::parse();

    if args.verbose {
        crate::core::logging::set_log_level("debug");
    } else {
        crate::core::logging::init_logging();
    }

    args.validate()?;

    match args.command {
        Some(Commands::Run { maintenance }) => {
            let mut runtime = crate::core::Runtime::new(&args.brain, args.config)?;
            runtime.run(maintenance).await?;
        }
        Some(Commands::Brain { action }) => match action {
            BrainAction::Init { output } => {
                let mut header = BrainHeader::new();
                header.cortex_offset = 0;
                header.compute_checksum();
                header.write(&output)?;
                println!("Brain initialized: {}", output.display());
            }
            BrainAction::Verify { brain } => {
                let header = BrainHeader::read(&brain)?;
                header.validate()?;
                println!("Brain verification: OK");
            }
            BrainAction::Inspect { brain, format } => {
                let fmt = match format.as_str() {
                    "json" => InspectFormat::Json,
                    _ => InspectFormat::Text,
                };
                let output = storage::inspect_brain(&brain, fmt)?;
                println!("{}", output);
            }
        },
        Some(Commands::Diag { action }) => {
            let runtime = crate::core::Runtime::new(&args.brain, args.config)?;
            diagnostics::run_diagnostic(&runtime, &action).await?;
        }
        None => {
            let mut runtime = crate::core::Runtime::new(&args.brain, args.config)?;
            runtime.run(false).await?;
        }
    }

    Ok(())
}

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
    /// Brain management commands
    Brain {
        #[command(subcommand)]
        action: BrainAction,
    },
    /// Diagnostics
    Diag {
        /// Diagnostic action
        #[arg(value_parser = ["status", "memory", "storage", "neural", "safety"])]
        action: String,
    },
}

#[derive(Parser, Debug, Clone)]
pub enum BrainAction {
    /// Initialize a new brain.anr file
    Init {
        /// Output path for brain.anr
        output: PathBuf,
    },
    /// Verify brain.anr integrity
    Verify {
        /// Brain file path
        brain: PathBuf,
    },
    /// Inspect brain.anr header
    Inspect {
        /// Brain file path
        brain: PathBuf,
        /// Output format
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

impl Cli {
    /// Parse CLI arguments.
    pub fn parse() -> Self {
        Self::parse_from(std::env::args_os())
    }

    /// Return list of available top-level commands.
    pub fn available_commands() -> Vec<&'static str> {
        vec!["run", "brain", "diag"]
    }

    /// Validate that the parsed CLI is semantically valid.
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
            Some(Commands::Brain {
                action: BrainAction::Inspect { format, .. },
            }) => {
                let valid = ["text", "json"].contains(&format.as_str());
                if !valid {
                    return Err(Error::ConfigInvalid(format!(
                        "Unknown format: '{}'. Available formats: text, json",
                        format
                    )));
                }
                Ok(())
            }
            Some(Commands::Brain { .. }) | Some(Commands::Run { .. }) | None => Ok(()),
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
        assert!(cmds.contains(&"brain"));
        assert!(cmds.contains(&"diag"));
        assert_eq!(cmds.len(), 3);
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
    fn test_validate_brain_init() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Init {
                    output: PathBuf::from("/tmp/brain.anr"),
                },
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_brain_verify() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Verify {
                    brain: PathBuf::from("/tmp/brain.anr"),
                },
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_brain_inspect_text() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Inspect {
                    brain: PathBuf::from("/tmp/brain.anr"),
                    format: "text".into(),
                },
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_brain_inspect_json() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Inspect {
                    brain: PathBuf::from("/tmp/brain.anr"),
                    format: "json".into(),
                },
            }),
        };
        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_validate_brain_inspect_invalid_format() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Inspect {
                    brain: PathBuf::from("/tmp/brain.anr"),
                    format: "xml".into(),
                },
            }),
        };
        let err = cli.validate().unwrap_err();
        match err {
            Error::ConfigInvalid(msg) => {
                assert!(msg.contains("xml"));
                assert!(msg.contains("text"));
                assert!(msg.contains("json"));
            }
            _ => panic!("expected ConfigInvalid"),
        }
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
    fn test_validate_diag_invalid_action() {
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
            Error::ConfigInvalid(_) => {}
            _ => panic!("expected ConfigInvalid"),
        }
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
        assert_eq!(cmds.len(), 3);
    }

    #[test]
    fn test_available_commands_contains_run() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"run"));
    }

    #[test]
    fn test_available_commands_contains_brain() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"brain"));
    }

    #[test]
    fn test_available_commands_contains_diag() {
        let cmds = Cli::available_commands();
        assert!(cmds.contains(&"diag"));
    }

    #[test]
    fn test_brain_inspect_error_code() {
        let cli = Cli {
            brain: PathBuf::from("/opt/anr/brain.anr"),
            config: None,
            verbose: false,
            command: Some(Commands::Brain {
                action: BrainAction::Inspect {
                    brain: PathBuf::from("/tmp/brain.anr"),
                    format: "xml".into(),
                },
            }),
        };
        let err = cli.validate().unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("ANR-E-CONFIG-001"));
    }
}
