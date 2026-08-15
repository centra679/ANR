use anr::hardware::mock::MockHAL;
use anr::hardware::Hal;
use anr::perception::audio::{AudioConfig, MockAudio};
use anr::perception::camera::{CameraConfig, MockCamera};
use anr::perception::sensor::DropPolicy;
use anr::plugins::isolation::PluginIsolation;
use anr::plugins::lifecycle::PluginHealth;

#[test]
fn tc_u_isolation_001() {
    let iso = PluginIsolation::new(3);
    assert_eq!(iso.health(), PluginHealth::Healthy);
    assert!(!iso.is_isolated());
}

#[test]
fn tc_u_isolation_002() {
    let mut iso = PluginIsolation::new(3);
    iso.record_failure();
    iso.record_success();
    assert_eq!(iso.health(), PluginHealth::Healthy);
    assert!(!iso.is_isolated());
}

#[test]
fn tc_u_isolation_003() {
    let mut iso = PluginIsolation::new(3);
    iso.record_failure();
    assert_eq!(iso.health(), PluginHealth::Degraded);
    assert!(!iso.is_isolated());
}

#[test]
fn tc_u_isolation_004() {
    let mut iso = PluginIsolation::new(2);
    iso.record_failure();
    iso.record_failure();
    assert_eq!(iso.health(), PluginHealth::Failed);
    assert!(iso.is_isolated());
}

#[test]
fn tc_u_isolation_005() {
    let mut iso = PluginIsolation::new(3);
    assert_eq!(iso.health(), PluginHealth::Healthy);
    iso.record_failure();
    assert_eq!(iso.health(), PluginHealth::Degraded);
    iso.record_failure();
    iso.record_failure();
    assert_eq!(iso.health(), PluginHealth::Failed);
}

#[test]
fn tc_u_isolation_006() {
    let mut iso = PluginIsolation::new(1);
    iso.record_failure();
    assert!(iso.is_isolated());
}

#[test]
fn tc_u_isolation_007() {
    let mut iso = PluginIsolation::new(5);
    iso.record_failure();
    iso.record_failure();
    iso.record_failure();
    assert_eq!(iso.health(), PluginHealth::Degraded);
    iso.record_success();
    assert_eq!(iso.health(), PluginHealth::Healthy);
}

#[test]
fn tc_u_isolation_008() {
    let hal = MockHAL::new();
    assert!(!hal.is_initialized());
}

#[test]
fn tc_u_isolation_009() {
    let mut hal = MockHAL::new();
    hal.initialize().unwrap();
    assert!(hal.is_initialized());
    assert!(Hal::is_ready(&hal));
}

#[test]
fn tc_u_isolation_010() {
    let mut hal = MockHAL::new();
    hal.initialize().unwrap();
    assert!(hal.is_initialized());
    hal.shutdown();
    assert!(!hal.is_initialized());
}

#[test]
fn tc_u_isolation_011() {
    let config = CameraConfig {
        max_frames: 10,
        max_frame_bytes: 4096,
        drop_policy: DropPolicy::DropOldest,
    };
    let cam = MockCamera::new(config);
    assert_eq!(cam.buffer_len(), 0);
    assert!(!cam.is_running());
    assert_eq!(cam.dropped_count(), 0);
}

#[test]
fn tc_u_isolation_012() {
    let config = AudioConfig {
        max_chunks: 10,
        max_chunk_bytes: 2048,
        drop_policy: DropPolicy::DropNewest,
    };
    let audio = MockAudio::new(config);
    assert_eq!(audio.buffer_len(), 0);
    assert!(!audio.is_running());
    assert_eq!(audio.dropped_count(), 0);
}
