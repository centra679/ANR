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
        Some(Commands::Verify { brain }) => {
            let valid = anr::storage::BrainValidator::verify_file(&brain)?;
            println!(
                "Brain verification: {}",
                if valid { "OK" } else { "FAILED" }
            );
        }
        Some(Commands::Build { seed, output }) => {
            anr::storage::BrainBuilder::build_from_seed(&seed, &output)?;
            println!("Brain built successfully: {}", output.display());
        }
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
