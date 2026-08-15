use super::decision::{ActionParams, ActionProposal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyVerdict {
    Allow,
    Reject,
    Clamp,
    Override,
    EmergencyStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyToken {
    pub proposal_id: u64,
    pub verdict: SafetyVerdict,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct SafetyDecision {
    pub proposal_id: u64,
    pub verdict: SafetyVerdict,
    pub clamped_params: Option<ActionParams>,
    pub token: Option<SafetyToken>,
}

pub struct SafetyLayer {
    max_velocity: f32,
    max_force: f32,
    emergency_stop_active: bool,
}

impl SafetyLayer {
    pub fn new(max_velocity: f32, max_force: f32) -> Self {
        Self {
            max_velocity,
            max_force,
            emergency_stop_active: false,
        }
    }

    pub fn check(&self, proposal: &ActionProposal, current_time: u64) -> SafetyDecision {
        if self.emergency_stop_active {
            return SafetyDecision {
                proposal_id: proposal.proposal_id,
                verdict: SafetyVerdict::EmergencyStop,
                clamped_params: None,
                token: Some(SafetyToken {
                    proposal_id: proposal.proposal_id,
                    verdict: SafetyVerdict::EmergencyStop,
                    timestamp: current_time,
                }),
            };
        }

        let params = &proposal.parameters;
        let needs_clamp = params.velocity > self.max_velocity || params.force > self.max_force;

        if needs_clamp {
            let clamped_velocity = params.velocity.min(self.max_velocity);
            let clamped_force = params.force.min(self.max_force);
            let clamped = ActionParams {
                velocity: clamped_velocity,
                direction: params.direction,
                force: clamped_force,
                duration_ms: params.duration_ms,
            };
            SafetyDecision {
                proposal_id: proposal.proposal_id,
                verdict: SafetyVerdict::Clamp,
                clamped_params: Some(clamped),
                token: Some(SafetyToken {
                    proposal_id: proposal.proposal_id,
                    verdict: SafetyVerdict::Clamp,
                    timestamp: current_time,
                }),
            }
        } else {
            SafetyDecision {
                proposal_id: proposal.proposal_id,
                verdict: SafetyVerdict::Allow,
                clamped_params: None,
                token: Some(SafetyToken {
                    proposal_id: proposal.proposal_id,
                    verdict: SafetyVerdict::Allow,
                    timestamp: current_time,
                }),
            }
        }
    }

    pub fn emergency_stop(&mut self) {
        self.emergency_stop_active = true;
    }

    pub fn is_emergency_stopped(&self) -> bool {
        self.emergency_stop_active
    }

    pub fn reset_emergency(&mut self) {
        self.emergency_stop_active = false;
    }
}
