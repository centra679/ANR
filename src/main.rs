/// ANR - Autonomous Neural Runtime
///
/// Architecture Contract: Final Architectural Baseline v1.1
///
/// Single binary executable that includes:
/// - Runtime and scheduler
/// - Neural Core (Cell, Column, Block, Synapse)
/// - Memory subsystems (Cortex, Cerebellum, Hippocampus)
/// - Learning and consolidation
/// - Storage and recovery
/// - Perception system
/// - Plugin system and HAL
/// - Decision engine and safety layer
/// - SIMD abstraction
/// - CLI and diagnostics
mod error;
pub use error::{Error, Result};

pub mod action;
pub mod brain;
pub mod core;
pub mod hardware;
pub mod interface;
pub mod learning;
pub mod memory;
pub mod neural;
pub mod perception;
pub mod plugins;
pub mod simd;
pub mod storage;

use clap::Parser;
use std::path::PathBuf;

/// ANR - Autonomous Neural Runtime
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to brain.anr file
    #[arg(short, long, default_value = "/opt/anr/brain.anr")]
    brain: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    if args.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    // Initialize runtime
    let mut runtime = core::Runtime::new(&args.brain, args.config)?;

    // Execute command or default run
    match args.command {
        Some(Commands::Run { maintenance }) => {
            runtime.run(maintenance).await?;
        }
        Some(Commands::Verify { brain }) => {
            let valid = storage::BrainValidator::verify_file(&brain)?;
            println!(
                "Brain verification: {}",
                if valid { "OK" } else { "FAILED" }
            );
        }
        Some(Commands::Build { seed, output }) => {
            storage::BrainBuilder::build_from_seed(&seed, &output)?;
            println!("Brain built successfully: {}", output.display());
        }
        Some(Commands::Diag { action }) => {
            interface::diagnostics::run_diagnostic(&runtime, &action).await?;
        }
        None => {
            runtime.run(false).await?;
        }
    }

    Ok(())
}
