pub mod catalog;
pub mod e2e;
pub mod integration;
/// ANR Test Framework
///
/// Implements: Master-Test-CI.md
/// - 840 unit tests (70 domains × 12 tests)
/// - 200 integration tests (25 domains × 8 tests)
/// - 120+ E2E tests
/// - Offline requirement
/// - Deterministic execution
/// - Architecture alignment
pub mod unit;

pub use catalog::TestCatalog;

/// Test infrastructure
pub struct TestFramework {
    _catalog: TestCatalog,
}

impl TestFramework {
    pub fn new() -> Self {
        Self {
            _catalog: TestCatalog::load(),
        }
    }
}

impl Default for TestFramework {
    fn default() -> Self {
        Self::new()
    }
}
