use anr::plugins::lifecycle::PluginHealth;
use anr::plugins::lifecycle::{PluginEvent, PluginLifecycle, PluginState};
use anr::plugins::{Plugin, PluginRegistry};

struct TestPlugin {
    name: String,
    health: PluginHealth,
}

impl TestPlugin {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: PluginHealth::Healthy,
        }
    }
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn init(&mut self) -> anr::Result<()> {
        Ok(())
    }
    fn start(&mut self) -> anr::Result<()> {
        Ok(())
    }
    fn stop(&mut self) -> anr::Result<()> {
        Ok(())
    }
    fn health(&self) -> PluginHealth {
        self.health
    }
}

#[test]
fn tc_u_plugin_lifecycle_001() {
    let lc = PluginLifecycle::new(3);
    assert_eq!(lc.state(), PluginState::Discovered);
}

#[test]
fn tc_u_plugin_lifecycle_002() {
    let mut lc = PluginLifecycle::new(3);
    let state = lc.transition(PluginEvent::Load).unwrap();
    assert_eq!(state, PluginState::Loaded);
}

#[test]
fn tc_u_plugin_lifecycle_003() {
    let mut lc = PluginLifecycle::new(3);
    lc.transition(PluginEvent::Load).unwrap();
    let state = lc.transition(PluginEvent::Init).unwrap();
    assert_eq!(state, PluginState::Initialized);
}

#[test]
fn tc_u_plugin_lifecycle_004() {
    let mut lc = PluginLifecycle::new(3);
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    let state = lc.transition(PluginEvent::Start).unwrap();
    assert_eq!(state, PluginState::Running);
}

#[test]
fn tc_u_plugin_lifecycle_005() {
    let mut lc = PluginLifecycle::new(3);
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    let state = lc.transition(PluginEvent::Stop).unwrap();
    assert_eq!(state, PluginState::Stopped);
}

#[test]
fn tc_u_plugin_lifecycle_006() {
    let mut lc = PluginLifecycle::new(3);
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    let state = lc.transition(PluginEvent::Error).unwrap();
    assert_eq!(state, PluginState::Failed);
}

#[test]
fn tc_u_plugin_lifecycle_007() {
    let mut lc = PluginLifecycle::new(3);
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    lc.transition(PluginEvent::Error).unwrap();
    let state = lc.transition(PluginEvent::Restart).unwrap();
    assert_eq!(state, PluginState::Discovered);
}

#[test]
fn tc_u_plugin_lifecycle_008() {
    let mut lc = PluginLifecycle::new(1);
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    lc.transition(PluginEvent::Error).unwrap();
    lc.transition(PluginEvent::Restart).unwrap();
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    lc.transition(PluginEvent::Error).unwrap();
    let result = lc.transition(PluginEvent::Restart);
    assert!(result.is_err());
}

#[test]
fn tc_u_plugin_lifecycle_009() {
    let mut lc = PluginLifecycle::new(3);
    assert!(!lc.is_running());
    lc.transition(PluginEvent::Load).unwrap();
    lc.transition(PluginEvent::Init).unwrap();
    lc.transition(PluginEvent::Start).unwrap();
    assert!(lc.is_running());
    lc.transition(PluginEvent::Stop).unwrap();
    assert!(!lc.is_running());
}

#[test]
fn tc_u_plugin_lifecycle_010() {
    let mut reg = PluginRegistry::new();
    assert_eq!(reg.count(), 0);
    reg.register(Box::new(TestPlugin::new("p1")));
    reg.register(Box::new(TestPlugin::new("p2")));
    assert_eq!(reg.count(), 2);
}

#[test]
fn tc_u_plugin_lifecycle_011() {
    let mut reg = PluginRegistry::new();
    reg.register(Box::new(TestPlugin::new("p1")));
    reg.register(Box::new(TestPlugin::new("p2")));
    assert!(reg.init_all().is_ok());
}

#[test]
fn tc_u_plugin_lifecycle_012() {
    let mut reg = PluginRegistry::new();
    reg.register(Box::new(TestPlugin::new("p1")));
    reg.register(Box::new(TestPlugin::new("p2")));
    assert!(reg.start_all().is_ok());
}
