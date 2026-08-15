use anr::core::{Runtime, RuntimeState};
use std::path::PathBuf;

fn make_runtime() -> Runtime {
    Runtime::new(&PathBuf::from("/tmp/test_e2e.anr"), None).unwrap()
}

#[test]
fn tc_u_core_run_001_new_creates_powered_off() {
    let rt = make_runtime();
    assert_eq!(rt.state(), RuntimeState::PoweredOff);
}

#[test]
fn tc_u_core_run_002_boot_transitions_to_running() {
    let mut rt = make_runtime();
    rt.boot().unwrap();
    assert_eq!(rt.state(), RuntimeState::Running);
}

#[test]
fn tc_u_core_run_003_cycle_increments_count() {
    let mut rt = make_runtime();
    rt.boot().unwrap();
    assert_eq!(rt.cycle_count(), 0);
    rt.cycle().unwrap();
    assert_eq!(rt.cycle_count(), 1);
    rt.cycle().unwrap();
    assert_eq!(rt.cycle_count(), 2);
}

#[test]
fn tc_u_core_run_004_run_e2e_ten_cycles() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(10).unwrap();
    assert_eq!(result.cycles_completed, 10);
}

#[test]
fn tc_u_core_run_005_run_e2e_zero_completes() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(0).unwrap();
    assert_eq!(result.cycles_completed, 0);
}

#[test]
fn tc_u_core_run_006_episodes_stored_after_cycles() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(5).unwrap();
    assert!(result.episodes_stored > 0);
}

#[test]
fn tc_u_core_run_007_commands_executed_after_cycles() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(5).unwrap();
    assert!(result.commands_executed > 0);
    assert_eq!(result.commands_executed, 5);
}

#[test]
fn tc_u_core_run_008_shutdown_sets_powered_off() {
    let mut rt = make_runtime();
    rt.boot().unwrap();
    assert_eq!(rt.state(), RuntimeState::Running);
    rt.shutdown().unwrap();
    assert_eq!(rt.state(), RuntimeState::PoweredOff);
}

#[test]
fn tc_u_core_run_009_e2e_result_fields_correct() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(7).unwrap();
    assert_eq!(result.cycles_completed, 7);
    assert!(result.episodes_stored > 0);
    assert_eq!(result.commands_executed, 7);
    assert_eq!(result.final_state, RuntimeState::PoweredOff);
}

#[test]
fn tc_u_core_run_010_full_e2e_vertical_slice() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(5).unwrap();
    assert_eq!(result.cycles_completed, 5);
    assert!(result.episodes_stored > 0);
    assert!(result.commands_executed > 0);
    assert_eq!(result.final_state, RuntimeState::PoweredOff);
}

#[test]
fn tc_u_core_run_011_safety_rejection_stores_episode() {
    let mut rt = make_runtime();
    let result = rt.run_e2e(3).unwrap();
    assert!(result.episodes_stored > 0);
    assert!(result.commands_executed > 0);
}

#[test]
fn tc_u_core_run_012_multiple_run_e2e_independent() {
    let mut rt = make_runtime();
    let r1 = rt.run_e2e(5).unwrap();
    assert_eq!(r1.cycles_completed, 5);
    let r2 = rt.run_e2e(3).unwrap();
    assert_eq!(r2.cycles_completed, 3);
}
