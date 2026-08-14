use anr::error::Error;
use anr::interface::cli::{BrainAction, Cli, Commands};
use std::path::PathBuf;

fn make_cli(command: Option<Commands>) -> Cli {
    Cli {
        brain: PathBuf::from("/opt/anr/brain.anr"),
        config: None,
        verbose: false,
        command,
    }
}

// ── available_commands ─────────────────────────────────────────────────────

#[test]
fn tc_u_cli_available_commands_returns_three() {
    let cmds = Cli::available_commands();
    assert_eq!(cmds.len(), 3);
}

#[test]
fn tc_u_cli_available_commands_contains_run() {
    assert!(Cli::available_commands().contains(&"run"));
}

#[test]
fn tc_u_cli_available_commands_contains_brain() {
    assert!(Cli::available_commands().contains(&"brain"));
}

#[test]
fn tc_u_cli_available_commands_contains_diag() {
    assert!(Cli::available_commands().contains(&"diag"));
}

// ── validate: None command ────────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_no_command_ok() {
    let cli = make_cli(None);
    assert!(cli.validate().is_ok());
}

// ── validate: Run ─────────────────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_run_maintenance_true() {
    let cli = make_cli(Some(Commands::Run { maintenance: true }));
    assert!(cli.validate().is_ok());
}

#[test]
fn tc_u_cli_validate_run_maintenance_false() {
    let cli = make_cli(Some(Commands::Run { maintenance: false }));
    assert!(cli.validate().is_ok());
}

// ── validate: Brain Init ──────────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_brain_init() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Init {
            output: PathBuf::from("/tmp/brain.anr"),
        },
    }));
    assert!(cli.validate().is_ok());
}

// ── validate: Brain Verify ────────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_brain_verify() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Verify {
            brain: PathBuf::from("/tmp/brain.anr"),
        },
    }));
    assert!(cli.validate().is_ok());
}

// ── validate: Brain Inspect ───────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_brain_inspect_text() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Inspect {
            brain: PathBuf::from("/tmp/brain.anr"),
            format: "text".into(),
        },
    }));
    assert!(cli.validate().is_ok());
}

#[test]
fn tc_u_cli_validate_brain_inspect_json() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Inspect {
            brain: PathBuf::from("/tmp/brain.anr"),
            format: "json".into(),
        },
    }));
    assert!(cli.validate().is_ok());
}

#[test]
fn tc_u_cli_validate_brain_inspect_invalid_format() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Inspect {
            brain: PathBuf::from("/tmp/brain.anr"),
            format: "xml".into(),
        },
    }));
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

// ── validate: Diag ────────────────────────────────────────────────────────

#[test]
fn tc_u_cli_validate_diag_all_valid_actions() {
    for action in ["status", "memory", "storage", "neural", "safety"] {
        let cli = make_cli(Some(Commands::Diag {
            action: action.into(),
        }));
        assert!(
            cli.validate().is_ok(),
            "expected Ok for diag action '{}'",
            action
        );
    }
}

#[test]
fn tc_u_cli_validate_diag_invalid_action() {
    let cli = make_cli(Some(Commands::Diag {
        action: "unknown".into(),
    }));
    let err = cli.validate().unwrap_err();
    match err {
        Error::ConfigInvalid(msg) => {
            assert!(msg.contains("unknown"));
        }
        _ => panic!("expected ConfigInvalid"),
    }
}

// ── Error display codes ───────────────────────────────────────────────────

#[test]
fn tc_u_cli_inspect_invalid_format_error_code() {
    let cli = make_cli(Some(Commands::Brain {
        action: BrainAction::Inspect {
            brain: PathBuf::from("/tmp/brain.anr"),
            format: "xml".into(),
        },
    }));
    let err = cli.validate().unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("ANR-E-CONFIG-001"));
}

#[test]
fn tc_u_cli_diag_invalid_action_error_code() {
    let cli = make_cli(Some(Commands::Diag {
        action: "bogus".into(),
    }));
    let err = cli.validate().unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("ANR-E-CONFIG-001"));
}

// ── Cli struct fields ─────────────────────────────────────────────────────

#[test]
fn tc_u_cli_brain_default_path() {
    let cli = make_cli(None);
    assert_eq!(cli.brain, PathBuf::from("/opt/anr/brain.anr"));
}

#[test]
fn tc_u_cli_config_none_by_default() {
    let cli = make_cli(None);
    assert!(cli.config.is_none());
}

#[test]
fn tc_u_cli_verbose_false_by_default() {
    let cli = make_cli(None);
    assert!(!cli.verbose);
}

#[test]
fn tc_u_cli_verbose_true() {
    let cli = Cli {
        brain: PathBuf::from("/opt/anr/brain.anr"),
        config: None,
        verbose: true,
        command: None,
    };
    assert!(cli.verbose);
}

#[test]
fn tc_u_cli_config_some() {
    let cli = Cli {
        brain: PathBuf::from("/opt/anr/brain.anr"),
        config: Some(PathBuf::from("/etc/anr.toml")),
        verbose: false,
        command: None,
    };
    assert_eq!(cli.config, Some(PathBuf::from("/etc/anr.toml")));
}

// ── Clone / Debug ─────────────────────────────────────────────────────────

#[test]
fn tc_u_cli_clone() {
    let cli = make_cli(Some(Commands::Run { maintenance: true }));
    let cloned = cli.clone();
    assert_eq!(format!("{:?}", cli), format!("{:?}", cloned));
}

#[test]
fn tc_u_cli_debug() {
    let cli = make_cli(None);
    let dbg = format!("{:?}", cli);
    assert!(dbg.contains("Cli"));
    assert!(dbg.contains("brain"));
}
