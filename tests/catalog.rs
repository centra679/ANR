/// Test Catalog - Machine-readable test registry
/// Implements: Master-Test-CI.md §5
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEntry {
    pub id: String,
    pub level: String, // unit, integration, e2e, fault-injection, performance, conformance, security
    pub domain: String,
    pub requirement: String, // Architecture Contract reference
    pub test_type: String,   // positive, negative, boundary, invariant, regression
    pub status: String,      // required, optional, deprecated
    pub owner: Option<String>,
    pub criticality: String, // high, medium, low
}

pub struct TestCatalog {
    entries: HashMap<String, TestEntry>,
}

impl TestCatalog {
    pub fn load() -> Self {
        let mut entries = HashMap::new();

        // Load from static definition
        Self::populate_unit_tests(&mut entries);

        Self { entries }
    }

    fn populate_unit_tests(entries: &mut HashMap<String, TestEntry>) {
        // Group A: Core Runtime & Lifecycle (10 domains × 12 tests = 120 tests)
        let domains_a = vec![
            ("core-boot", "AC §19", "Core boot lifecycle"),
            ("core-run-loop", "AC §18", "Main loop state machine"),
            ("core-shutdown", "AC §20", "Graceful shutdown"),
            ("core-emergency-shutdown", "AC §21", "Emergency shutdown"),
            ("core-degraded", "AC §21", "Degraded mode state"),
            ("config-load", "AC §37", "Configuration loading"),
            ("config-validation", "AC §37", "Config validation"),
            ("error-taxonomy", "AC §32", "Error classification"),
            ("logging-tracing", "AC §36", "Logging and tracing"),
            ("scheduler-priority", "AC §22", "Scheduler priority"),
        ];

        Self::add_domain_tests(entries, &domains_a, "unit");

        // Group B: Resource Control & Neural Cell/Column (10 domains × 12 tests = 120 tests)
        let domains_b = vec![
            ("maintenance-budget", "AC §28", "Maintenance budget"),
            ("bounded-queue", "AC §23", "Bounded queue capacity"),
            ("backpressure", "AC §23", "Backpressure handling"),
            ("cell-state", "AC §11", "Cell state basics"),
            ("cell-activation", "AC §11", "Cell activation"),
            ("cell-refractory", "AC §11", "Refractory state"),
            ("column-competition", "AC §12", "Column competition"),
            ("column-sparse", "AC §12", "Sparse activation"),
            ("column-association", "AC §12", "Association"),
            ("block-context", "AC §13", "Context binding"),
        ];

        Self::add_domain_tests(entries, &domains_b, "unit");

        // Group C: Block, Synapse, Sparse Graph (10 domains × 12 tests = 120 tests)
        let domains_c = vec![
            ("block-sequence", "AC §13", "Sequence blocks"),
            ("block-prediction", "AC §13", "Prediction state"),
            ("synapse-create", "AC §14", "Synapse creation"),
            ("synapse-validate", "AC §14", "Synapse validation"),
            ("synapse-update", "AC §14", "Weight update"),
            ("synapse-decay", "AC §14", "Decay/weakening"),
            ("synapse-prune", "AC §14", "Pruning"),
            ("sparse-traversal", "AC §15", "Graph traversal"),
            ("soa-layout", "AC §24", "SoA consistency"),
            ("scalar-kernels", "AC §25", "Scalar kernels"),
        ];

        Self::add_domain_tests(entries, &domains_c, "unit");

        // Group D: SIMD, Memory, GC (10 domains × 12 tests = 120 tests)
        let domains_d = vec![
            ("simd-neon", "AC §25", "NEON kernels"),
            ("simd-avx", "AC §25", "AVX2/AVX-512"),
            ("simd-fallback", "AC §25", "Scalar fallback"),
            ("memory-quota", "AC §26", "Quota management"),
            ("allocator", "AC §27", "Allocation/free"),
            ("memory-isolation", "AC §26", "Section isolation"),
            ("retention-scoring", "AC §29", "Retention scoring"),
            ("gc-normal", "AC §30", "Normal GC"),
            ("gc-aggressive", "AC §30", "Aggressive GC"),
            ("gc-emergency", "AC §30", "Emergency GC"),
        ];

        Self::add_domain_tests(entries, &domains_d, "unit");

        // Group E: Storage, Brain, Recovery (10 domains × 12 tests = 120 tests)
        let domains_e = vec![
            ("tiering", "AC §41", "HOT/WARM/COLD"),
            ("compression", "AC §42", "Compression"),
            ("brain-header", "AC §48", "Header parsing"),
            ("brain-offset-size", "AC §48", "Offset/size validation"),
            ("checksum", "AC §44", "Checksum integrity"),
            ("transaction", "AC §45", "Transaction handling"),
            ("recovery", "AC §43", "Recovery"),
            ("brain-seed", "AC §49", "Seed validation"),
            ("brain-build", "AC §49", "Build from seed"),
            ("brain-verify-inspect", "AC §50", "Verify/inspect"),
        ];

        Self::add_domain_tests(entries, &domains_e, "unit");

        // Group F: Provisioning, Perception, Plugin/HAL, Decision/Safety (10 domains × 12 tests = 120 tests)
        let domains_f = vec![
            ("brain-install-update", "AC §51", "Install/update"),
            ("sensor-frame", "AC §33", "SensorFrame"),
            ("camera-buffer", "AC §34", "Camera buffer"),
            ("audio-buffer", "AC §35", "Audio buffer"),
            ("perception-fusion", "AC §33", "Perception fusion"),
            ("plugin-lifecycle", "AC §52", "Plugin lifecycle"),
            ("plugin-isolation", "AC §53", "Plugin isolation"),
            ("hal-mock", "AC §54", "HAL mock"),
            ("decision-candidate", "AC §55", "Decision candidates"),
            ("safety-constraints", "AC §31", "Safety constraints"),
        ];

        Self::add_domain_tests(entries, &domains_f, "unit");

        // Group G: Learning, Feedback, CLI, Diagnostics, Security (10 domains × 12 tests = 120 tests)
        let domains_g = vec![
            ("feedback-prediction", "AC §16", "Feedback/prediction"),
            ("hebbian-learning", "AC §16", "Hebbian learning"),
            ("temporal-learning", "AC §16", "Temporal learning"),
            ("replay-selection", "AC §17", "Replay selection"),
            ("consolidation-promotion", "AC §17", "Consolidation"),
            ("contradiction-handling", "AC §17", "Contradictions"),
            ("skill-failure", "AC §8", "Skill failure"),
            ("cli-commands", "AC §56", "CLI commands"),
            ("diagnostics-telemetry", "AC §57", "Diagnostics"),
            ("security-validation", "AC §58", "Security"),
        ];

        Self::add_domain_tests(entries, &domains_g, "unit");
    }

    fn add_domain_tests(
        entries: &mut HashMap<String, TestEntry>,
        domains: &[(&str, &str, &str)],
        level: &str,
    ) {
        for (domain, requirement, _desc) in domains {
            for i in 1..=12 {
                let test_id = format!("TC-U-{}-{:03}", domain.to_uppercase(), i);
                let test_type = match i {
                    1..=3 => "positive",
                    4..=6 => "negative",
                    7..=9 => "boundary",
                    10..=11 => "invariant",
                    _ => "regression",
                };

                let entry = TestEntry {
                    id: test_id.clone(),
                    level: level.to_string(),
                    domain: domain.to_string(),
                    requirement: requirement.to_string(),
                    test_type: test_type.to_string(),
                    status: "required".to_string(),
                    owner: None,
                    criticality: if i <= 3 {
                        "high".to_string()
                    } else if i <= 9 {
                        "medium".to_string()
                    } else {
                        "low".to_string()
                    },
                };

                entries.insert(test_id, entry);
            }
        }
    }

    pub fn get(&self, id: &str) -> Option<&TestEntry> {
        self.entries.get(id)
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn count_by_level(&self, level: &str) -> usize {
        self.entries.values().filter(|e| e.level == level).count()
    }

    pub fn count_by_domain(&self, domain: &str) -> usize {
        self.entries.values().filter(|e| e.domain == domain).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_count() {
        let catalog = TestCatalog::load();
        assert_eq!(
            catalog.count_by_level("unit"),
            840,
            "Should have 840 unit tests"
        );
    }

    #[test]
    fn test_domain_count() {
        let catalog = TestCatalog::load();
        // Each domain should have 12 tests
        let domains = vec![
            "core-boot",
            "core-run-loop",
            "cell-state",
            "simd-neon",
            "brain-header",
            "sensor-frame",
            "feedback-prediction",
        ];

        for domain in domains {
            assert_eq!(
                catalog.count_by_domain(domain),
                12,
                "Domain {} should have 12 tests",
                domain
            );
        }
    }

    #[test]
    fn test_catalog_entry_structure() {
        let catalog = TestCatalog::load();
        if let Some(entry) = catalog.get("TC-U-CORE-BOOT-001") {
            assert_eq!(entry.level, "unit");
            assert_eq!(entry.domain, "core-boot");
            assert_eq!(entry.status, "required");
            assert!(entry.requirement.starts_with("AC"));
        } else {
            panic!("Test entry not found");
        }
    }
}
