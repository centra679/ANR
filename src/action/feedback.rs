use super::decision::ActionParams;
use super::safety::SafetyToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionResult {
    Success,
    Failure,
    Timeout,
    Partial,
}

#[derive(Debug, Clone)]
pub struct ActuatorCommand {
    pub command_id: u64,
    pub actuator_id: u32,
    pub parameters: ActionParams,
    pub safety_token: SafetyToken,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ActuatorFeedback {
    pub command_id: u64,
    pub result: ActionResult,
    pub prediction_error: f32,
    pub timestamp: u64,
}

pub struct FeedbackLoop {
    last_prediction: f32,
    last_outcome: f32,
}

impl Default for FeedbackLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedbackLoop {
    pub fn new() -> Self {
        Self {
            last_prediction: 0.0,
            last_outcome: 0.0,
        }
    }

    pub fn record(&mut self, prediction: f32, outcome: f32) {
        self.last_prediction = prediction;
        self.last_outcome = outcome;
    }

    pub fn prediction_error(&self) -> f32 {
        self.last_outcome - self.last_prediction
    }
}

pub struct MockActuator {
    commands_executed: u64,
    last_command: Option<ActuatorCommand>,
    fail_next: bool,
}

impl Default for MockActuator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockActuator {
    pub fn new() -> Self {
        Self {
            commands_executed: 0,
            last_command: None,
            fail_next: false,
        }
    }

    pub fn execute(&mut self, command: ActuatorCommand) -> ActuatorFeedback {
        let result = if self.fail_next {
            self.fail_next = false;
            ActionResult::Failure
        } else {
            ActionResult::Success
        };

        let error = match result {
            ActionResult::Success => 0.0,
            ActionResult::Failure => 1.0,
            ActionResult::Timeout => 1.0,
            ActionResult::Partial => 0.5,
        };

        self.commands_executed += 1;
        let ts = command.timestamp;
        let cid = command.command_id;
        self.last_command = Some(command);

        ActuatorFeedback {
            command_id: cid,
            result,
            prediction_error: error,
            timestamp: ts,
        }
    }

    pub fn set_fail_next(&mut self, fail: bool) {
        self.fail_next = fail;
    }

    pub fn commands_executed(&self) -> u64 {
        self.commands_executed
    }

    pub fn last_command(&self) -> Option<&ActuatorCommand> {
        self.last_command.as_ref()
    }
}
