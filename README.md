# ANR - Autonomous Neural Runtime

**Architecture Contract:** Final Architectural Baseline v1.1

ANR is a single-binary, offline-first autonomous neural runtime implementing a persistent neural architecture with knowledge, skill, and experience memory subsystems.

## Overview

ANR enforces strict architectural contracts for:
- **Single Binary** - All core functionality in one executable
- **Single Brain** - One persistent neural memory file (`brain.anr`)
- **Offline-First** - No cloud dependencies required
- **Safety-First** - Safety layer cannot be bypassed
- **Bounded Resources** - Predictable memory and compute
- **Learning** - Hebbian, temporal, and replay-based learning with consolidation

## Architecture

### Core Components

```
anr/
├── core/              Runtime lifecycle, state machine, scheduler
├── neural/            Cell, Column, Block, Synapse, sparse graph
├── brain/             Cortex, Cerebellum, Hippocampus subsystems
├── learning/          Hebbian, temporal, replay, consolidation
├── memory/            Allocator, quotas, GC (normal/aggressive/emergency)
├── storage/           brain.anr format, IO, transaction, recovery
├── perception/        Sensor, camera, audio, fusion
├── plugins/           Plugin lifecycle, isolation
├── hardware/          HAL (Hardware Abstraction Layer)
├── action/            Decision engine, safety layer, feedback
├── simd/              SIMD abstraction (scalar/NEON/AVX)
└── interface/         CLI, diagnostics, telemetry
```

### Memory Subsystems

#### Cortex - Long-Term Knowledge
- Pattern generalization
- Semantic associations
- Validated knowledge
- Conservative garbage collection

#### Cerebellum - Procedural Skills
- Action-oriented capabilities
- Error-corrected behavior
- Prediction mappings
- Very conservative GC

#### Hippocampus - Episodic Experience
- Event recording
- Temporal episodes
- Replay source
- Feeds consolidation to Cortex/Cerebellum

## Deployment

### Minimal Deployment

```bash
/opt/anr/
├── anr              # Single executable
└── brain.anr        # Persistent neural memory
```

### Running

```bash
# Start runtime
./anr --brain /opt/anr/brain.anr

# Verify brain integrity
./anr verify /opt/anr/brain.anr

# Build brain from seed
./anr build --seed initial_knowledge.json --output brain.anr

# Diagnostics
./anr diag status
./anr diag memory
./anr diag storage
./anr diag neural
./anr diag safety
```

## Testing

### Test Requirements

Minimum test counts (total: **1440+ tests**):

| Level | Count | Domains | Tests/Domain |
|-------|-------|---------|--------------|
| Unit | 840 | 70 | 12 |
| Integration | 200 | 25 | 8 |
| E2E | 120+ | - | - |
| Fault Injection | 80 | 8 | 10 |
| Performance | 60 | 7 | 8-9 |
| Conformance | 100 | 10 | 10 |
| Security | 40 | 6 | 6-7 |

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration

# E2E tests
cargo test --test e2e

# Performance benchmarks
cargo test --test performance

# Conformance tests
cargo test --test conformance

# Security tests
cargo test --test security_extended

# Fault injection tests
cargo test --test fault_injection
```

### Test Naming Convention

Tests follow ID format: `TC-{LEVEL}-{DOMAIN}-{SEQ}`

Examples:
- `TC-U-CORE-BOOT-001` - Unit test, core-boot domain, test 001
- `TC-I-NEURAL-GRAPH-003` - Integration test, neural-graph domain, test 003
- `TC-E2E-BOOT-VALID-BRAIN-005` - E2E test, boot valid brain scenario, test 005

## Unit Test Domains (70 Total)

### Group A: Core Runtime & Lifecycle (10 domains)
- core-boot, core-run-loop, core-shutdown
- core-emergency-shutdown, core-degraded
- config-load, config-validation
- error-taxonomy, logging-tracing, scheduler-priority

### Group B: Resource Control & Neural (10 domains)
- maintenance-budget, bounded-queue, backpressure
- cell-state, cell-activation, cell-refractory
- column-competition, column-sparse, column-association
- block-context

### Group C: Block, Synapse, Graph (10 domains)
- block-sequence, block-prediction
- synapse-create, synapse-validate, synapse-update
- synapse-decay, synapse-prune
- sparse-traversal, soa-layout, scalar-kernels

### Group D: SIMD, Memory, GC (10 domains)
- simd-neon, simd-avx, simd-fallback
- memory-quota, allocator, memory-isolation
- retention-scoring, gc-normal, gc-aggressive, gc-emergency

### Group E: Storage, Brain, Recovery (10 domains)
- tiering, compression
- brain-header, brain-offset-size, checksum
- transaction, recovery
- brain-seed, brain-build, brain-verify-inspect

### Group F: Provisioning, Perception, Plugin, Safety (10 domains)
- brain-install-update
- sensor-frame, camera-buffer, audio-buffer, perception-fusion
- plugin-lifecycle, plugin-isolation, hal-mock
- decision-candidate, safety-constraints

### Group G: Learning, CLI, Diagnostics, Security (10 domains)
- feedback-prediction, hebbian-learning, temporal-learning
- replay-selection, consolidation-promotion
- contradiction-handling, skill-failure
- cli-commands, diagnostics-telemetry, security-validation

## Integration Test Domains (25 Total)

- sensor-to-perception, camera-to-perception, audio-to-perception
- perception-to-neural, neural-active-graph
- cortex-interface, cerebellum-interface, hippocampus-episode
- replay-to-learning, learning-to-synapse
- consolidation-to-memory, retention-to-gc
- allocation-to-tiering, storage-read-validation, storage-write-transaction
- recovery-to-boot, brain-build-install
- cli-to-runtime, diagnostics-to-telemetry
- decision-to-safety, safety-to-actuator, actuator-to-feedback
- plugin-to-hal, plugin-failure-degradation
- simd-to-neural-update

## CI/CD Pipeline

The project includes a comprehensive CI pipeline (`.github/workflows/ci.yml`) that enforces:

1. **Unit Testing** - All 840+ unit tests must pass
2. **Integration Testing** - All 200+ integration tests must pass
3. **E2E Testing** - All 120+ E2E tests must pass
4. **Linting** - Clippy with strict warnings
5. **Formatting** - rustfmt compliance
6. **Security Audit** - Dependency vulnerability scanning
7. **Architecture Conformance** - Dedicated conformance tests
8. **Code Coverage** - Optional coverage reporting
9. **Test Count Gate** - Enforces minimum test counts
10. **Performance Benchmarks** - Monitors performance regressions

### CI Requirements

```yaml
Minimum Tests:
  - Unit Tests: 840
  - Integration Tests: 200
  - E2E Tests: 120
  - Total: 1440

Execution Time:
  - Single Unit Test: ≤ 100ms (storage: ≤ 250ms)
  - Full Unit Suite: ≤ 5 minutes

Constraints:
  - All tests offline (no cloud/network)
  - Deterministic execution
  - No retry masking of failures
  - No empty tests
  - Traceable to Architecture Contract
```

## Build & Release

### Development Build

```bash
cargo build
./target/debug/anr --help
```

### Release Build

```bash
cargo build --release
./target/release/anr --help
```

### Documentation

```bash
cargo doc --no-deps --open
```

## Architecture Contract References

This implementation follows the strict requirements of:

1. **Master-Arsitektur.md** - Architectural baseline v1.1
   - Deployment contract (single binary, single brain)
   - Persistent brain structure
   - Three memory subsystems
   - Safety and isolation requirements

2. **Master-Technical.md** - Supporting technical specifications
   - Runtime state machine and lifecycle
   - Repository module structure
   - brain.anr binary format specification
   - Detailed field layouts and checksums

3. **Master-Test-CI.md** - Tests and CI contract
   - Strict test requirements (1440+ tests)
   - Test level definitions (unit/integration/E2E/fault/performance/conformance/security)
   - Test domain quotas and compositions
   - CI pipeline enforcement gates
   - Offline-first and determinism requirements

## Key Invariants

```text
✓ No actuator activation before safety layer ready
✓ No learning before brain validation
✓ No running without valid brain generation
✓ Emergency stop always reachable from Running/Degraded
✓ Degraded mode never disables safety
✓ Single binary contains all core functionality
✓ Cortex, Cerebellum, Hippocampus are logical sections in brain.anr
✓ No external files required for core operation
✓ Offline operation possible without cloud
✓ Deterministic perception → decide → act cycle
```

## Configuration

Example configuration file (TOML):

```toml
[runtime]
state_trace = false
shutdown_timeout_ms = 5000
emergency_stop_timeout_ms = 100
allow_volatile_degraded_mode = false

[memory]
min_bytes = 26214400        # 25 MB
target_bytes = 52428800     # 50 MB
max_bytes = 104857600       # 100 MB

[storage]
block_size = 4096
compression_enabled = true
checksum_algo = "blake3"    # or "crc32c"

[scheduler]
maintenance_budget_percent = 10
priority_classes = 4

[perception]
frame_buffer_size = 100
fusion_enabled = true

[safety]
constraints_strict = true
override_allowed = false
```

## License

ANR - Autonomous Neural Runtime
© 2026 - Licensed under MIT/Apache-2.0

## Documentation Links

- Architecture Contract: [Master-Arsitektur.md](Master-Arsitektur.md)
- Technical Specification: [Master-Technical.md](Master-Technical.md)
- Tests & CI Contract: [Master-Test-CI.md](Master-Test-CI.md)

---

**Status:** Architecture Contract v1.1 - Implementation Phase
**Test Coverage:** 1590+ tests (unit + integration + E2E + performance + conformance + security + fault-injection)
**CI Status:** Full pipeline configured and enforced
