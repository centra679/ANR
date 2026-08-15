use anr::core::{Runtime, RuntimeState};

#[test]
fn tc_e2e_vertical_slice_001() {
    let temp_dir = std::env::temp_dir().join("anr_e2e_vertical");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let brain_path = temp_dir.join("brain.anr");

    let mut runtime = Runtime::new(&brain_path, None).unwrap();
    let result = runtime.run_e2e(5).unwrap();

    assert_eq!(result.cycles_completed, 5);
    assert!(result.episodes_stored > 0);
    assert!(result.commands_executed > 0);
    assert_eq!(result.final_state, RuntimeState::PoweredOff);

    let _ = std::fs::remove_dir_all(&temp_dir);
}
