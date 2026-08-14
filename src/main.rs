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
use anr::interface::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anr::Result<()> {
    let args = Cli::parse();

    if args.verbose {
        anr::core::logging::set_log_level("debug");
    } else {
        anr::core::logging::init_logging();
    }

    args.validate()?;

    match args.command {
        Some(Commands::Run { maintenance }) => {
            let mut runtime = anr::core::Runtime::new(&args.brain, args.config)?;
            runtime.run(maintenance).await?;
        }
        Some(Commands::Brain { action }) => match action {
            anr::interface::cli::BrainAction::Init { output } => {
                let mut header = anr::storage::BrainHeader::new();
                header.cortex_offset = 0;
                header.compute_checksum();
                header.write(&output)?;
                println!("Brain initialized: {}", output.display());
            }
            anr::interface::cli::BrainAction::Verify { brain } => {
                let valid = anr::storage::BrainValidator::verify_file(&brain)?;
                println!(
                    "Brain verification: {}",
                    if valid { "OK" } else { "FAILED" }
                );
            }
            anr::interface::cli::BrainAction::Inspect { brain, format } => {
                let fmt = match format.as_str() {
                    "json" => anr::storage::InspectFormat::Json,
                    _ => anr::storage::InspectFormat::Text,
                };
                let output = anr::storage::inspect_brain(&brain, fmt)?;
                println!("{}", output);
            }
        },
        Some(Commands::Diag { action }) => {
            let runtime = anr::core::Runtime::new(&args.brain, args.config)?;
            anr::interface::diagnostics::run_diagnostic(&runtime, &action).await?;
        }
        None => {
            let mut runtime = anr::core::Runtime::new(&args.brain, args.config)?;
            runtime.run(false).await?;
        }
    }

    Ok(())
}
