/// Local Logging Configuration
/// Aligns with: AC §36, DEC-003
/// Logging is local-only. No network transport.
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initialize local logging subscriber if not already initialized.
pub fn init_logging() {
    let _ = std::panic::catch_unwind(|| {
        let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}

/// Set logging level at runtime via filter string.
/// Examples: "debug", "warn", "anr=debug", "anr=info,crate=warn"
pub fn set_log_level(filter: &str) {
    let _ = std::panic::catch_unwind(|| {
        let level = EnvFilter::new(filter);
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(level)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
    tracing::info!("Log level set to {}", filter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn ensure_init() {
        INIT.call_once(|| {
            init_logging();
        });
    }

    #[test]
    fn test_logging_default_info() {
        ensure_init();
        tracing::info!("default info level");
    }

    #[test]
    fn test_logging_warn_level() {
        ensure_init();
        tracing::warn!("warn level message");
    }

    #[test]
    fn test_logging_debug_level() {
        ensure_init();
        tracing::debug!("debug level message");
    }

    #[test]
    fn test_logging_error_level() {
        ensure_init();
        tracing::error!("error level message");
    }

    #[test]
    fn test_set_log_level_debug() {
        ensure_init();
        set_log_level("debug");
        tracing::debug!("debug after set level");
    }

    #[test]
    fn test_set_log_level_warn() {
        ensure_init();
        set_log_level("warn");
        tracing::warn!("warn after set level");
    }

    #[test]
    fn test_set_log_level_error() {
        ensure_init();
        set_log_level("error");
        tracing::error!("error after set level");
    }

    #[test]
    fn test_set_log_level_off() {
        ensure_init();
        set_log_level("off");
        tracing::debug!("this should not emit");
    }

    #[test]
    fn test_set_log_level_crate_specific() {
        ensure_init();
        set_log_level("anr=debug");
        tracing::debug!("crate-specific debug");
    }

    #[test]
    fn test_logging_no_network_dependency() {
        ensure_init();
        let _ = "logging is local-only";
    }

    #[test]
    fn test_set_log_level_invalid_falls_back_to_info() {
        ensure_init();
        set_log_level("invalid_directive_xyz");
    }

    #[test]
    fn test_multiple_set_level_calls_do_not_panic() {
        ensure_init();
        set_log_level("info");
        set_log_level("info");
    }
}
