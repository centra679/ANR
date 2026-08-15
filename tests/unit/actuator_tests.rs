use anr::action::decision::{ActionParams, ActionType, DecisionEngine, DecisionSource};
use anr::action::feedback::{ActionResult, ActuatorCommand, ActuatorFeedback, MockActuator};
use anr::action::safety::{SafetyLayer, SafetyToken, SafetyVerdict};

fn make_token() -> SafetyToken {
    SafetyToken {
        proposal_id: 1,
        verdict: SafetyVerdict::Allow,
        timestamp: 100,
    }
}

fn make_command(token: SafetyToken) -> ActuatorCommand {
    ActuatorCommand {
        command_id: 1,
        actuator_id: 10,
        parameters: ActionParams {
            velocity: 1.0,
            direction: 0.5,
            force: 0.3,
            duration_ms: 100,
        },
        safety_token: token,
        timestamp: 100,
    }
}

#[test]
fn tc_u_actuator_001() {
    let actuator = MockActuator::new();
    assert_eq!(actuator.commands_executed(), 0);
    assert!(actuator.last_command().is_none());
}

#[test]
fn tc_u_actuator_002() {
    let mut actuator = MockActuator::new();
    let feedback = actuator.execute(make_command(make_token()));
    assert_eq!(feedback.result, ActionResult::Success);
}

#[test]
fn tc_u_actuator_003() {
    let mut actuator = MockActuator::new();
    assert_eq!(actuator.commands_executed(), 0);
    actuator.execute(make_command(make_token()));
    assert_eq!(actuator.commands_executed(), 1);
    actuator.execute(make_command(make_token()));
    assert_eq!(actuator.commands_executed(), 2);
}

#[test]
fn tc_u_actuator_004() {
    let mut actuator = MockActuator::new();
    let cmd = make_command(make_token());
    let cmd_id = cmd.command_id;
    actuator.execute(cmd);
    let last = actuator.last_command().unwrap();
    assert_eq!(last.command_id, cmd_id);
}

#[test]
fn tc_u_actuator_005() {
    let mut actuator = MockActuator::new();
    actuator.set_fail_next(true);
    let feedback = actuator.execute(make_command(make_token()));
    assert_eq!(feedback.result, ActionResult::Failure);
}

#[test]
fn tc_u_actuator_006() {
    let cmd = make_command(make_token());
    assert_eq!(cmd.command_id, 1);
    assert_eq!(cmd.actuator_id, 10);
    assert_eq!(cmd.parameters.velocity, 1.0);
    assert_eq!(cmd.timestamp, 100);
}

#[test]
fn tc_u_actuator_007() {
    let fb = ActuatorFeedback {
        command_id: 5,
        result: ActionResult::Success,
        prediction_error: 0.0,
        timestamp: 200,
    };
    assert_eq!(fb.command_id, 5);
    assert_eq!(fb.result, ActionResult::Success);
    assert_eq!(fb.prediction_error, 0.0);
    assert_eq!(fb.timestamp, 200);
}

#[test]
fn tc_u_actuator_008() {
    let token = make_token();
    let cmd = make_command(token);
    assert_eq!(cmd.safety_token.verdict, SafetyVerdict::Allow);
}

#[test]
fn tc_u_actuator_009() {
    let mut actuator = MockActuator::new();
    actuator.set_fail_next(true);
    let fb1 = actuator.execute(make_command(make_token()));
    assert_eq!(fb1.result, ActionResult::Failure);
    let fb2 = actuator.execute(make_command(make_token()));
    assert_eq!(fb2.result, ActionResult::Success);
}

#[test]
fn tc_u_actuator_010() {
    let mut actuator = MockActuator::new();
    for i in 0..5 {
        let mut cmd = make_command(make_token());
        cmd.command_id = i;
        actuator.execute(cmd);
    }
    assert_eq!(actuator.commands_executed(), 5);
}

#[test]
fn tc_u_actuator_011() {
    let s = ActionResult::Success;
    let f = ActionResult::Failure;
    let t = ActionResult::Timeout;
    let p = ActionResult::Partial;
    assert_ne!(s, f);
    assert_ne!(f, t);
    assert_ne!(t, p);
    assert_ne!(p, s);
    assert_eq!(s, ActionResult::Success);
}

#[test]
fn tc_u_actuator_012() {
    let mut engine = DecisionEngine::new();
    let safety = SafetyLayer::new(10.0, 5.0);
    let proposal = engine.propose(
        ActionType::Move,
        ActionParams {
            velocity: 2.0,
            direction: 0.0,
            force: 1.0,
            duration_ms: 50,
        },
        0.9,
        DecisionSource::Neural,
    );
    let decision = safety.check(&proposal, 100);
    assert_eq!(decision.verdict, SafetyVerdict::Allow);
    let token = decision.token.unwrap();
    let cmd = make_command(token);
    let mut actuator = MockActuator::new();
    let fb = actuator.execute(cmd);
    assert_eq!(fb.result, ActionResult::Success);
}
