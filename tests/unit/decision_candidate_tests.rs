use anr::action::decision::{
    ActionParams, ActionProposal, ActionType, DecisionEngine, DecisionSource,
};

#[test]
fn tc_u_decision_001() {
    let mut engine = DecisionEngine::new();
    let p = engine.propose(
        ActionType::Move,
        ActionParams::default(),
        0.5,
        DecisionSource::Neural,
    );
    assert_eq!(p.proposal_id, 0);
}

#[test]
fn tc_u_decision_002() {
    let mut engine = DecisionEngine::new();
    let params = ActionParams {
        velocity: 1.0,
        direction: 0.5,
        force: 0.3,
        duration_ms: 100,
    };
    let p = engine.propose(ActionType::Move, params.clone(), 0.8, DecisionSource::Skill);
    assert_eq!(p.action_type, ActionType::Move);
    assert_eq!(p.parameters.velocity, 1.0);
    assert_eq!(p.parameters.direction, 0.5);
    assert_eq!(p.parameters.force, 0.3);
    assert_eq!(p.parameters.duration_ms, 100);
    assert_eq!(p.confidence, 0.8);
    assert_eq!(p.source, DecisionSource::Skill);
}

#[test]
fn tc_u_decision_003() {
    let mut engine = DecisionEngine::new();
    let p1 = engine.propose(
        ActionType::Move,
        ActionParams::default(),
        0.5,
        DecisionSource::Neural,
    );
    let p2 = engine.propose(
        ActionType::Stop,
        ActionParams::default(),
        0.5,
        DecisionSource::Neural,
    );
    assert_eq!(p1.proposal_id, 0);
    assert_eq!(p2.proposal_id, 1);
}

#[test]
fn tc_u_decision_004() {
    let engine = DecisionEngine::new();
    let proposal = ActionProposal {
        proposal_id: 0,
        action_type: ActionType::Move,
        parameters: ActionParams {
            velocity: 0.5,
            direction: 0.0,
            force: 0.0,
            duration_ms: 100,
        },
        confidence: 0.7,
        source: DecisionSource::Neural,
    };
    let score = engine.evaluate(&proposal);
    assert!(score > proposal.confidence);
}

#[test]
fn tc_u_decision_005() {
    let mut engine = DecisionEngine::new();
    let p1 = engine.propose(
        ActionType::Move,
        ActionParams::default(),
        0.3,
        DecisionSource::Fallback,
    );
    let p2 = engine.propose(
        ActionType::Stop,
        ActionParams::default(),
        0.9,
        DecisionSource::Neural,
    );
    let p3 = engine.propose(
        ActionType::Turn,
        ActionParams::default(),
        0.5,
        DecisionSource::Skill,
    );
    let proposals = [p1.clone(), p2.clone(), p3.clone()];
    let best = engine.select(&proposals).unwrap();
    assert_eq!(best.proposal_id, p2.proposal_id);
}

#[test]
fn tc_u_decision_006() {
    let engine = DecisionEngine::new();
    let proposals: Vec<ActionProposal> = vec![];
    assert!(engine.select(&proposals).is_none());
}

#[test]
fn tc_u_decision_007() {
    let mut engine = DecisionEngine::new();
    let p1 = engine.propose(
        ActionType::Move,
        ActionParams::default(),
        0.9,
        DecisionSource::Neural,
    );
    let p2 = engine.propose(
        ActionType::Move,
        ActionParams::default(),
        0.5,
        DecisionSource::Neural,
    );
    let proposals = [p2.clone(), p1.clone()];
    let best = engine.select(&proposals).unwrap();
    assert_eq!(best.proposal_id, p1.proposal_id);
}

#[test]
fn tc_u_decision_008() {
    let p = ActionParams::default();
    assert_eq!(p.velocity, 0.0);
    assert_eq!(p.direction, 0.0);
    assert_eq!(p.force, 0.0);
    assert_eq!(p.duration_ms, 0);
}

#[test]
fn tc_u_decision_009() {
    assert_eq!(ActionType::Move, ActionType::Move);
    assert_ne!(ActionType::Move, ActionType::Stop);
    assert_ne!(ActionType::Grasp, ActionType::Release);
    assert_eq!(ActionType::Custom(42), ActionType::Custom(42));
    assert_ne!(ActionType::Custom(1), ActionType::Custom(2));
}

#[test]
fn tc_u_decision_010() {
    assert_eq!(DecisionSource::Neural, DecisionSource::Neural);
    assert_ne!(DecisionSource::Neural, DecisionSource::Skill);
    assert_ne!(DecisionSource::Knowledge, DecisionSource::Fallback);
}

#[test]
fn tc_u_decision_011() {
    let mut engine = DecisionEngine::new();
    let params = ActionParams::default();
    let p1 = engine.propose(
        ActionType::Move,
        params.clone(),
        0.5,
        DecisionSource::Neural,
    );
    let p2 = engine.propose(ActionType::Move, params, 0.5, DecisionSource::Skill);
    assert_eq!(p1.source, DecisionSource::Neural);
    assert_eq!(p2.source, DecisionSource::Skill);
}

#[test]
fn tc_u_decision_012() {
    let engine = DecisionEngine::new();
    let proposal = ActionProposal {
        proposal_id: 0,
        action_type: ActionType::Custom(99),
        parameters: ActionParams::default(),
        confidence: 0.01,
        source: DecisionSource::Fallback,
    };
    let score = engine.evaluate(&proposal);
    assert!(score < 0.1);
}
