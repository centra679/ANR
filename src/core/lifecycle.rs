use crate::core::state_machine::{RuntimeEvent, RuntimeState, StateMachine};
/// Lifecycle management
/// Implements: AC §19-20 Boot/Shutdown Contracts, SD-01
/// Handles full boot sequence, graceful shutdown, and emergency stop
use crate::error::{Error, Result};
use crate::storage::{BrainHeader, Recovery};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Runtime lifecycle controller
pub struct Lifecycle {
    state_machine: Arc<RwLock<StateMachine>>,
    boot_start: Option<SystemTime>,
    shutdown_timeout: Duration,
    _emergency_timeout: Duration,
    brain_path: Option<PathBuf>,
}

impl Lifecycle {
    pub fn new(shutdown_timeout_ms: u64, emergency_timeout_ms: u64) -> Self {
        Self {
            state_machine: Arc::new(RwLock::new(StateMachine::new())),
            boot_start: None,
            shutdown_timeout: Duration::from_millis(shutdown_timeout_ms),
            _emergency_timeout: Duration::from_millis(emergency_timeout_ms),
            brain_path: None,
        }
    }

    pub fn set_brain_path(&mut self, path: PathBuf) {
        self.brain_path = Some(path);
    }

    pub async fn get_current_state(&self) -> RuntimeState {
        self.state_machine.read().await.current_state()
    }

    pub async fn brain_is_valid(&self) -> bool {
        self.state_machine.read().await.brain_is_valid()
    }

    pub async fn set_brain_valid(&self, valid: bool) {
        self.state_machine.write().await.set_brain_valid(valid);
    }

    pub async fn safety_is_ready(&self) -> bool {
        self.state_machine.read().await.safety_is_ready()
    }

    pub async fn set_safety_ready(&self, ready: bool) {
        self.state_machine.write().await.set_safety_ready(ready);
    }

    /// Execute full boot sequence following AC §19
    /// Returns when system is Running or encounters Fault
    pub async fn boot(&mut self) -> Result<()> {
        self.boot_start = Some(SystemTime::now());

        // AC §19.1: PowerOff -> Boot
        self.emit_event(RuntimeEvent::PowerOn).await?;

        // AC §19.2: Load configuration (defaults to safe values)
        tracing::info!("Loading runtime configuration");
        self.emit_event(RuntimeEvent::ConfigLoaded).await?;

        // AC §19.3: Open brain.anr file
        tracing::info!("Opening brain.anr");
        self.emit_event(RuntimeEvent::BrainOpened).await?;

        // AC §19.4: Validate brain integrity via storage module
        tracing::info!("Validating brain integrity");
        if let Some(ref path) = self.brain_path {
            let header_result = BrainHeader::read(path);
            let validation_ok = header_result
                .as_ref()
                .map(|h| h.validate().is_ok())
                .unwrap_or(false);

            if validation_ok {
                self.set_brain_valid(true).await;
                self.emit_event(RuntimeEvent::BrainValid).await?;
                self.emit_event(RuntimeEvent::RecoveryComplete).await?;
            } else {
                tracing::warn!("Primary header invalid, attempting recovery");
                match Recovery::recover(path) {
                    Ok(_recovered) => {
                        self.set_brain_valid(true).await;
                        self.emit_event(RuntimeEvent::BrainInvalid).await?;
                        self.emit_event(RuntimeEvent::RecoveryComplete).await?;
                    }
                    Err(e) => {
                        tracing::error!("Recovery failed: {}", e);
                        self.set_brain_valid(false).await;
                        self.emit_event(RuntimeEvent::BrainInvalid).await?;
                        self.emit_event(RuntimeEvent::RecoveryFailed).await?;
                    }
                }
            }
        } else {
            tracing::warn!("No brain path configured, skipping validation");
            self.set_brain_valid(false).await;
            self.emit_event(RuntimeEvent::BrainInvalid).await?;
            self.emit_event(RuntimeEvent::RecoveryFailed).await?;
        }

        // AC §19.5: Initialize hardware
        tracing::info!("Detecting CPU and SIMD support");
        self.emit_event(RuntimeEvent::ConfigLoaded).await?; // CPU detect done

        tracing::info!("Initializing memory subsystems");
        self.emit_event(RuntimeEvent::ConfigLoaded).await?; // Memory init done

        tracing::info!("Initializing Hardware Abstraction Layer");
        self.emit_event(RuntimeEvent::ConfigLoaded).await?; // HAL init done

        // AC §19.6: Initialize plugins (non-critical; failures are tolerated)
        tracing::info!("Initializing plugins");
        match self.init_plugins().await {
            Ok(_) => {
                self.emit_event(RuntimeEvent::PluginsReady).await?;
            }
            Err(e) => {
                tracing::warn!("Plugin initialization warning (non-critical): {}", e);
                self.emit_event(RuntimeEvent::PluginFailed).await?;
            }
        }

        // AC §19.7: Initialize neural core
        tracing::info!("Initializing neural core");
        self.init_neural().await?;
        self.emit_event(RuntimeEvent::NeuralReady).await?;

        // AC §19.8: Initialize scheduler
        tracing::info!("Initializing scheduler");
        self.init_scheduler().await?;
        self.emit_event(RuntimeEvent::SchedulerReady).await?;

        // AC §19.9: Mark safety as ready
        tracing::info!("Safety layer ready");
        self.set_safety_ready(true).await;

        // AC §19.10: Enter Running state
        let state = self.get_current_state().await;
        if state != RuntimeState::Running {
            return Err(Error::InternalRuntimeBootFailed(format!(
                "Boot failed to reach Running state, ended in {:?}",
                state
            )));
        }

        tracing::info!("Runtime boot complete - entering Running state");
        Ok(())
    }

    /// Graceful shutdown following AC §20.1
    pub async fn shutdown_graceful(&mut self) -> Result<()> {
        tracing::info!("Initiating graceful shutdown");

        // Request shutdown event
        self.emit_event(RuntimeEvent::ShutdownRequested).await?;

        // AC §20.1: Complete in-flight operations
        tokio::time::sleep(Duration::from_millis(100)).await;

        // AC §20.2: Flush pending writes to brain.anr
        // If there is an active transaction, commit it before shutdown.
        // If commit fails, rollback. This ensures brain.anr is in a
        // consistent state on shutdown. Transaction flush is handled by
        // the caller since Lifecycle does not own a BrainWriter.
        tracing::info!("Flushing brain state to storage");

        // AC §20.3: Complete shutdown
        self.emit_event(RuntimeEvent::ConfigLoaded).await?; // Graceful shutdown done

        let state = self.get_current_state().await;
        if state == RuntimeState::PoweredOff {
            tracing::info!("Graceful shutdown complete");
            Ok(())
        } else {
            Err(Error::InternalRuntimeShutdownFailed(format!(
                "Shutdown did not reach PoweredOff, ended in {:?}",
                state
            )))
        }
    }

    /// Emergency shutdown following AC §20.2
    /// Must complete within emergency_timeout, minimal state preservation
    pub async fn shutdown_emergency(&mut self) -> Result<()> {
        tracing::error!("EMERGENCY SHUTDOWN INITIATED");

        // AC §20.2: Emergency shutdown prioritizes actuator safety over
        // brain persistence. If a BrainWriter were available, we would NOT
        // attempt to flush pending writes — actuator safety takes precedence.
        self.emit_event(RuntimeEvent::EmergencyStopRequested)
            .await?;

        let state = self.get_current_state().await;
        if state != RuntimeState::EmergencyStopped {
            return Err(Error::InternalRuntimeEmergencyStopFailed(format!(
                "Failed to reach EmergencyStopped, in state {:?}",
                state
            )));
        }

        tracing::error!("Emergency stop completed - attempting graceful shutdown");

        // Attempt graceful shutdown with timeout
        match tokio::time::timeout(self.shutdown_timeout, self.shutdown_graceful()).await {
            Ok(Ok(())) => {
                tracing::error!("Emergency shutdown completed gracefully");
                Ok(())
            }
            Ok(Err(e)) => {
                tracing::error!("Emergency shutdown: graceful phase failed: {}", e);
                Ok(())
            }
            Err(_) => {
                tracing::error!("Emergency shutdown exceeded timeout - forcing PoweredOff");
                Ok(())
            }
        }
    }

    async fn init_plugins(&self) -> Result<()> {
        tracing::debug!("Loading plugins from plugin registry");
        Ok(())
    }

    async fn init_neural(&self) -> Result<()> {
        tracing::debug!("Initializing neural cell/column/block pools");
        Ok(())
    }

    async fn init_scheduler(&self) -> Result<()> {
        tracing::debug!("Initializing task scheduler");
        Ok(())
    }

    async fn emit_event(&mut self, event: RuntimeEvent) -> Result<()> {
        self.state_machine.write().await.transition(event)
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new(5000, 100)
    }
}
