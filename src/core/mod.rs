use crate::action::decision::{
    ActionParams, ActionProposal, ActionType, DecisionEngine, DecisionSource,
};
use crate::action::feedback::{ActuatorCommand, ActuatorFeedback, FeedbackLoop, MockActuator};
use crate::action::safety::{SafetyDecision, SafetyLayer};
use crate::brain::cortex::DataOrigin;
use crate::brain::hippocampus::{Episode, Hippocampus};
use crate::core::scheduler::Task;
use crate::neural::NeuralCore;
use crate::perception::camera::{CameraConfig, MockCamera};
use crate::perception::fusion::PerceptionOutput;
use crate::perception::sensor::DropPolicy;
use crate::perception::Perception;
use crate::Result;
use std::path::{Path, PathBuf};

pub mod config;
pub mod lifecycle;
pub mod logging;
pub mod scheduler;
pub mod state_machine;

pub use config::RuntimeConfig;
pub use scheduler::{PriorityClass, Scheduler};
pub use state_machine::{RuntimeEvent, RuntimeState};

pub struct Runtime {
    brain_path: PathBuf,
    state: RuntimeState,
    camera: MockCamera,
    perception: Perception,
    neural: NeuralCore,
    decision: DecisionEngine,
    safety: SafetyLayer,
    actuator: MockActuator,
    feedback: FeedbackLoop,
    hippocampus: Hippocampus,
    scheduler: Scheduler,
    cycle_count: u64,
    max_cycles: u64,
    running: bool,
}

impl Runtime {
    pub fn new(brain_path: &Path, _config_path: Option<PathBuf>) -> Result<Self> {
        Ok(Self {
            brain_path: brain_path.to_path_buf(),
            state: RuntimeState::PoweredOff,
            camera: MockCamera::new(CameraConfig {
                max_frames: 8,
                max_frame_bytes: 2_097_152,
                drop_policy: DropPolicy::DropOldest,
            }),
            perception: Perception::new(16),
            neural: NeuralCore::new(1024, 128, 64, 4096),
            decision: DecisionEngine::new(),
            safety: SafetyLayer::new(1.5, 0.8),
            actuator: MockActuator::new(),
            feedback: FeedbackLoop::new(),
            hippocampus: Hippocampus::new(1024),
            scheduler: Scheduler::default(),
            cycle_count: 0,
            max_cycles: 100,
            running: false,
        })
    }

    pub fn run_e2e(&mut self, max_cycles: u64) -> Result<E2EResult> {
        self.max_cycles = max_cycles;
        self.boot()?;
        while self.running && self.cycle_count < self.max_cycles {
            self.cycle()?;
        }
        self.shutdown()?;
        Ok(E2EResult {
            cycles_completed: self.cycle_count,
            episodes_stored: self.hippocampus.episode_count(),
            commands_executed: self.actuator.commands_executed(),
            final_state: self.state,
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        self.state = RuntimeState::Boot;
        let _ = self.brain_path.exists();
        self.camera.start()?;
        self.state = RuntimeState::Running;
        self.running = true;
        self.cycle_count = 0;
        Ok(())
    }

    pub fn cycle(&mut self) -> Result<()> {
        let frame = self.camera.capture_frame(64, 64)?;
        let _output: PerceptionOutput = self.perception.process_frames(&[frame])?;

        let task = Task {
            id: self.cycle_count,
            priority: PriorityClass::Normal,
            payload: format!("cycle_{}", self.cycle_count),
        };
        let _ = self.scheduler.submit(task);

        let _ = self.neural.cycle(self.cycle_count, &[]);

        let proposal: ActionProposal = self.decision.propose(
            ActionType::Move,
            ActionParams {
                velocity: 0.5,
                direction: 0.0,
                force: 0.3,
                duration_ms: 100,
            },
            0.8,
            DecisionSource::Neural,
        );

        let safety_decision: SafetyDecision = self.safety.check(&proposal, self.cycle_count);

        if let Some(token) = safety_decision.token {
            let command = ActuatorCommand {
                command_id: self.cycle_count,
                actuator_id: 0,
                parameters: proposal.parameters.clone(),
                safety_token: token,
                timestamp: self.cycle_count,
            };
            let feedback_result: ActuatorFeedback = self.actuator.execute(command);

            let outcome =
                if feedback_result.result == crate::action::feedback::ActionResult::Success {
                    1.0
                } else {
                    0.0
                };
            self.feedback.record(0.8, outcome);

            let episode = Episode {
                id: format!("ep_{}", self.cycle_count),
                context: "vertical_slice".to_string(),
                action: "move".to_string(),
                reward: if feedback_result.result == crate::action::feedback::ActionResult::Success
                {
                    0.8
                } else {
                    0.2
                },
                origin: DataOrigin::Learned,
                created_at: self.cycle_count,
            };
            let _ = self.hippocampus.add_episode(episode);
        }

        self.cycle_count += 1;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        self.running = false;
        self.state = RuntimeState::PoweredOff;
        Ok(())
    }

    pub async fn run_loop(&mut self, _maintenance_mode: bool) -> Result<()> {
        self.boot()?;
        self.shutdown()?;
        Ok(())
    }

    pub fn state(&self) -> RuntimeState {
        self.state
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    pub fn episode_count(&self) -> usize {
        self.hippocampus.episode_count()
    }

    pub fn commands_executed(&self) -> u64 {
        self.actuator.commands_executed()
    }

    pub fn prediction_error(&self) -> f32 {
        self.feedback.prediction_error()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[derive(Debug)]
pub struct E2EResult {
    pub cycles_completed: u64,
    pub episodes_stored: usize,
    pub commands_executed: u64,
    pub final_state: RuntimeState,
}
