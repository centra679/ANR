use anr::action::decision::{ActionParams, ActionType, DecisionEngine, DecisionSource};
use anr::action::safety::{SafetyLayer, SafetyToken, SafetyVerdict};

fn make_proposal(
    engine: &mut DecisionEngine,
    velocity: f32,
    force: f32,
) -> anr::action::decision::ActionProposal {
    engine.propose(
        ActionType::Move,
        ActionParams {
            velocity,
            direction: 0.0,
            force,
            duration_ms: 100,
        },
        0.8,
        DecisionSource::Neural,
    )
}

#[test]
fn tc_u_safety_001() {
    let layer = SafetyLayer::new(10.0, 5.0);
    assert!(!layer.is_emergency_stopped());
}

#[test]
fn tc_u_safety_002() {
    let layer = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 5.0, 3.0);
    let decision = layer.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::Allow);
    assert!(decision.clamped_params.is_none());
}

#[test]
fn tc_u_safety_003() {
    let layer = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 15.0, 1.0);
    let decision = layer.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::Clamp);
    let clamped = decision.clamped_params.unwrap();
    assert_eq!(clamped.velocity, 10.0);
}

#[test]
fn tc_u_safety_004() {
    let layer = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 1.0, 8.0);
    let decision = layer.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::Clamp);
    let clamped = decision.clamped_params.unwrap();
    assert_eq!(clamped.force, 5.0);
}

#[test]
fn tc_u_safety_005() {
    let mut layer = SafetyLayer::new(10.0, 5.0);
    layer.emergency_stop();
    assert!(layer.is_emergency_stopped());
}

#[test]
fn tc_u_safety_006() {
    let mut layer = SafetyLayer::new(10.0, 5.0);
    layer.emergency_stop();
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 0.1, 0.1);
    let decision = layer.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::EmergencyStop);
    assert!(decision.clamped_params.is_none());
}

#[test]
fn tc_u_safety_007() {
    let mut layer = SafetyLayer::new(10.0, 5.0);
    layer.emergency_stop();
    assert!(layer.is_emergency_stopped());
    layer.reset_emergency();
    assert!(!layer.is_emergency_stopped());
}

#[test]
fn tc_u_safety_008() {
    let token = SafetyToken {
        proposal_id: 42,
        verdict: SafetyVerdict::Allow,
        timestamp: 1000,
    };
    assert_eq!(token.proposal_id, 42);
    assert_eq!(token.verdict, SafetyVerdict::Allow);
    assert_eq!(token.timestamp, 1000);
}

#[test]
fn tc_u_safety_009() {
    let layer = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 5.0, 3.0);
    let decision = layer.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::Allow);
    assert!(decision.token.is_some());
}

#[test]
fn tc_u_safety_010() {
    let mut layer = SafetyLayer::new(10.0, 5.0);
    layer.emergency_stop();
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 1.0, 1.0);
    let decision = layer.check(&proposal, 200);
    assert_eq!(decision.verdict, SafetyVerdict::EmergencyStop);
}

#[test]
fn tc_u_safety_011() {
    let mut layer = SafetyLayer::new(10.0, 5.0);
    layer.emergency_stop();
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 0.5, 0.5);
    let decision = layer.check(&proposal, 500);
    assert_eq!(decision.verdict, SafetyVerdict::EmergencyStop);
    assert!(decision.clamped_params.is_none());
    let token = decision.token.unwrap();
    assert_eq!(token.verdict, SafetyVerdict::EmergencyStop);
}

#[test]
fn tc_u_safety_012() {
    let layer = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    let proposal = make_proposal(&mut engine, 5.0, 3.0);
    let decision = layer.check(&proposal, 777);
    let token = decision.token.unwrap();
    assert_eq!(token.proposal_id, proposal.proposal_id);
    assert_eq!(token.verdict, SafetyVerdict::Allow);
    assert_eq!(token.timestamp, 777);
}
