use anr::action::decision::{ActionParams, ActionType, DecisionEngine, DecisionSource};
use anr::action::feedback::{FeedbackLoop, MockActuator};
use anr::action::safety::SafetyLayer;

#[test]
fn tc_u_feedback_001() {
    let fb = FeedbackLoop::new();
    assert_eq!(fb.prediction_error(), 0.0);
}

#[test]
fn tc_u_feedback_002() {
    let mut fb = FeedbackLoop::new();
    fb.record(5.0, 5.0);
    assert_eq!(fb.prediction_error(), 0.0);
}

#[test]
fn tc_u_feedback_003() {
    let mut fb = FeedbackLoop::new();
    fb.record(5.0, 5.0);
    assert_eq!(fb.prediction_error(), 0.0);
}

#[test]
fn tc_u_feedback_004() {
    let mut fb = FeedbackLoop::new();
    fb.record(3.0, 5.0);
    let error = fb.prediction_error();
    assert!((error - 2.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_feedback_005() {
    let mut fb = FeedbackLoop::new();
    fb.record(4.0, 4.0);
    let error = fb.prediction_error();
    assert!((error - 0.0).abs() < f32::EPSILON);
}

#[test]
fn tc_u_feedback_006() {
    let mut fb = FeedbackLoop::new();
    fb.record(2.0, 5.0);
    let error = fb.prediction_error();
    assert!(error > 0.0);
}

#[test]
fn tc_u_feedback_007() {
    let mut fb = FeedbackLoop::new();
    fb.record(8.0, 3.0);
    let error = fb.prediction_error();
    assert!(error < 0.0);
}

#[test]
fn tc_u_feedback_008() {
    let mut fb = FeedbackLoop::new();
    fb.record(1.0, 2.0);
    assert!((fb.prediction_error() - 1.0).abs() < f32::EPSILON);
    fb.record(5.0, 3.0);
    assert!((fb.prediction_error() - (-2.0)).abs() < f32::EPSILON);
    fb.record(0.0, 0.0);
    assert!((fb.prediction_error()).abs() < f32::EPSILON);
}

#[test]
fn tc_u_feedback_009() {
    let mut engine = DecisionEngine::new();
    let safety = SafetyLayer::new(10.0, 5.0);
    let proposal = engine.propose(
        ActionType::Move,
        ActionParams {
            velocity: 2.0,
            direction: 0.0,
            force: 1.0,
            duration_ms: 100,
        },
        0.8,
        DecisionSource::Neural,
    );
    let decision = safety.check(&proposal, 100);
    assert!(decision.token.is_some());
    assert_eq!(decision.verdict, anr::action::safety::SafetyVerdict::Allow);
}

#[test]
fn tc_u_feedback_010() {
    let mut engine = DecisionEngine::new();
    let safety = SafetyLayer::new(10.0, 5.0);
    let mut actuator = MockActuator::new();
    let mut feedback_loop = FeedbackLoop::new();

    let proposal = engine.propose(
        ActionType::Move,
        ActionParams {
            velocity: 2.0,
            direction: 0.0,
            force: 1.0,
            duration_ms: 100,
        },
        0.9,
        DecisionSource::Neural,
    );
    let safety_decision = safety.check(&proposal, 100);
    let token = safety_decision.token.unwrap();
    let cmd = anr::action::feedback::ActuatorCommand {
        command_id: proposal.proposal_id,
        actuator_id: 0,
        parameters: proposal.parameters.clone(),
        safety_token: token,
        timestamp: 100,
    };
    let exec_feedback = actuator.execute(cmd);
    assert_eq!(
        exec_feedback.result,
        anr::action::feedback::ActionResult::Success
    );
    feedback_loop.record(proposal.confidence, 1.0 - exec_feedback.prediction_error);
    let error = feedback_loop.prediction_error();
    assert!(
        (error - (1.0 - exec_feedback.prediction_error - proposal.confidence)).abs() < f32::EPSILON
    );
}

#[test]
fn tc_u_feedback_011() {
    let mut safety = SafetyLayer::new(10.0, 5.0);
    let mut engine = DecisionEngine::new();
    safety.emergency_stop();
    let proposal = engine.propose(
        ActionType::Move,
        ActionParams {
            velocity: 0.5,
            direction: 0.0,
            force: 0.1,
            duration_ms: 50,
        },
        0.9,
        DecisionSource::Neural,
    );
    let decision = safety.check(&proposal, 100);
    assert_eq!(
        decision.verdict,
        anr::action::safety::SafetyVerdict::EmergencyStop
    );
    assert!(decision.token.is_some());
    let mut actuator = MockActuator::new();
    let token = decision.token.unwrap();
    let cmd = anr::action::feedback::ActuatorCommand {
        command_id: 0,
        actuator_id: 0,
        parameters: proposal.parameters.clone(),
        safety_token: token,
        timestamp: 100,
    };
    let fb = actuator.execute(cmd);
    assert_eq!(fb.result, anr::action::feedback::ActionResult::Success);
}

#[test]
fn tc_u_feedback_012() {
    let mut actuator = MockActuator::new();
    let mut feedback_loop = FeedbackLoop::new();
    actuator.set_fail_next(true);
    let cmd = anr::action::feedback::ActuatorCommand {
        command_id: 1,
        actuator_id: 0,
        parameters: ActionParams {
            velocity: 1.0,
            direction: 0.0,
            force: 0.5,
            duration_ms: 50,
        },
        safety_token: anr::action::safety::SafetyToken {
            proposal_id: 1,
            verdict: anr::action::safety::SafetyVerdict::Allow,
            timestamp: 100,
        },
        timestamp: 100,
    };
    let fb = actuator.execute(cmd);
    assert_eq!(fb.result, anr::action::feedback::ActionResult::Failure);
    feedback_loop.record(0.8, fb.prediction_error);
    let error = feedback_loop.prediction_error();
    assert!((error - (fb.prediction_error - 0.8)).abs() < f32::EPSILON);
}
