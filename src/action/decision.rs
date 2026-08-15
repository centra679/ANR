#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Move,
    Stop,
    Turn,
    Grasp,
    Release,
    Custom(u32),
}

#[derive(Debug, Clone)]
pub struct ActionParams {
    pub velocity: f32,
    pub direction: f32,
    pub force: f32,
    pub duration_ms: u32,
}

impl Default for ActionParams {
    fn default() -> Self {
        Self {
            velocity: 0.0,
            direction: 0.0,
            force: 0.0,
            duration_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionSource {
    Neural,
    Skill,
    Knowledge,
    Fallback,
}

#[derive(Debug, Clone)]
pub struct ActionProposal {
    pub proposal_id: u64,
    pub action_type: ActionType,
    pub parameters: ActionParams,
    pub confidence: f32,
    pub source: DecisionSource,
}

pub struct DecisionEngine {
    next_proposal_id: u64,
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DecisionEngine {
    pub fn new() -> Self {
        Self {
            next_proposal_id: 0,
        }
    }

    pub fn propose(
        &mut self,
        action: ActionType,
        params: ActionParams,
        confidence: f32,
        source: DecisionSource,
    ) -> ActionProposal {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;
        ActionProposal {
            proposal_id: id,
            action_type: action,
            parameters: params,
            confidence,
            source,
        }
    }

    pub fn evaluate(&self, proposal: &ActionProposal) -> f32 {
        let mut score = proposal.confidence;
        let param = &proposal.parameters;
        let magnitude = (param.velocity * param.velocity
            + param.direction * param.direction
            + param.force * param.force)
            .sqrt();
        let complexity_bonus = match proposal.action_type {
            ActionType::Stop => 0.1,
            ActionType::Move | ActionType::Turn => 0.05,
            ActionType::Grasp | ActionType::Release => 0.03,
            ActionType::Custom(_) => 0.0,
        };
        let source_bonus = match proposal.source {
            DecisionSource::Neural => 0.05,
            DecisionSource::Skill => 0.04,
            DecisionSource::Knowledge => 0.03,
            DecisionSource::Fallback => 0.0,
        };
        score += complexity_bonus + source_bonus;
        if magnitude > 0.0 {
            score += (1.0 / (1.0 + magnitude)) * 0.02;
        }
        score
    }

    pub fn select<'a>(&self, proposals: &'a [ActionProposal]) -> Option<&'a ActionProposal> {
        if proposals.is_empty() {
            return None;
        }
        proposals.iter().max_by(|a, b| {
            self.evaluate(a)
                .partial_cmp(&self.evaluate(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
