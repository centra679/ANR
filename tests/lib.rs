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
pub mod integration;
pub mod e2e;
pub mod catalog;

pub use catalog::TestCatalog;

/// Test infrastructure
pub struct TestFramework {
    catalog: TestCatalog,
}

impl TestFramework {
    pub fn new() -> Self {
        Self {
            catalog: TestCatalog::load(),
        }
    }
}

impl Default for TestFramework {
    fn default() -> Self {
        Self::new()
    }
}
