# TEST DISTRIBUTION

**Updated:** 2026-08-14 (WP-2 post-triage)  
**Sumber:** `scripts/gen_catalog.sh` → `tests/catalog.toml`  
**Konteks:** Strategi legacy migration untuk 1.531 test eksisting  

---

## Ringkasan Global

| Metric | Nilai |
|--------|-------|
| Total test (legacy + real) | 1.471 |
| Legacy (fake) | 1.156 |
| Real | 315 |
| Unknown | 0 |
| Unit real | 315 |
| Integration real | 0 |
| E2E real | 0 |
| Fault-injection real | 0 |
| Performance real | 0 |
| Conformance real | 0 |
| Security real | 0 |

---

## Distribusi per Domain vs Kuota

### Unit Tests (kuota 12/domain) — WP-1 Completed Domains

| Domain | Real | Fake | Kuota | Status |
|--------|------|------|-------|--------|
| error-taxonomy | 12 | 0 | 12 | DONE |
| config-load | 13 | 0 | 12 | DONE (over-quota, sisa dipakai untuk validation) |
| config-validation | 12 | 0 | 12 | DONE |
| logging-tracing | 12 | 0 | 12 | DONE |
| cli-commands | 16 | 0 | 12 | DONE (over-quota, sisa dipakai untuk edge cases) |

### Unit Tests — Sisa Domain (masih fake)

| Domain | Real | Fake | Kuota | Status |
|--------|------|------|-------|--------|
| core-boot | 0 | 24 | 12 | PENDING WP-9 |
| core-degraded | 0 | 12 | 12 | PENDING WP-9 |
| core-emergency-shutdown | 0 | 12 | 12 | PENDING WP-9 |
| core-run-loop | 0 | 12 | 12 | PENDING WP-9 |
| core-shutdown | 0 | 12 | 12 | PENDING WP-9 |
| maintenance-budget | 0 | 12 | 12 | PENDING WP-1 (scheduler) |
| bounded-queue | 0 | 12 | 12 | PENDING WP-1 (scheduler) |
| backpressure | 0 | 12 | 12 | PENDING WP-1 (scheduler) |
| cell-state | 0 | 12 | 12 | PENDING WP-5 |
| cell-activation | 0 | 12 | 12 | PENDING WP-5 |
| cell-refractory | 0 | 12 | 12 | PENDING WP-5 |
| column-competition | 0 | 12 | 12 | PENDING WP-5 |
| column-sparse | 0 | 12 | 12 | PENDING WP-5 |
| column-association | 0 | 12 | 12 | PENDING WP-5 |
| block-context | 0 | 12 | 12 | PENDING WP-5 |
| block-prediction | 0 | 12 | 12 | PENDING WP-5 |
| block-sequence | 0 | 12 | 12 | PENDING WP-5 |
| synapse-create | 0 | 12 | 12 | PENDING WP-5 |
| synapse-validate | 0 | 12 | 12 | PENDING WP-5 |
| synapse-update | 0 | 12 | 12 | PENDING WP-5 |
| synapse-decay | 0 | 12 | 12 | PENDING WP-5 |
| synapse-prune | 0 | 12 | 12 | PENDING WP-5 |
| sparse-traversal | 0 | 12 | 12 | PENDING WP-5 |
| soa-layout | 0 | 12 | 12 | PENDING WP-5 |
| scalar-kernels | 0 | 12 | 12 | PENDING WP-5 |
| simd-neon | 0 | 12 | 12 | PENDING WP-5 |
| simd-avx | 0 | 12 | 12 | PENDING WP-5 |
| simd-fallback | 0 | 12 | 12 | PENDING WP-5 |
| memory-quota | 0 | 12 | 12 | PENDING WP-4 |
| allocator | 0 | 12 | 12 | PENDING WP-4 |
| memory-isolation | 0 | 12 | 12 | PENDING WP-4 |
| retention-scoring | 0 | 12 | 12 | PENDING WP-11 |
| gc-normal | 0 | 12 | 12 | PENDING WP-11 |
| gc-aggressive | 0 | 12 | 12 | PENDING WP-11 |
| gc-emergency | 0 | 12 | 12 | PENDING WP-11 |
| tiering | 0 | 12 | 12 | PENDING WP-11 |
| compression | 0 | 12 | 12 | PENDING WP-11 |
| brain-header | 14 | 0 | 12 | DONE (real: validate rules) |
| brain-offset-size | 7 | 0 | 12 | DONE (real: BLAKE3 checksum) |
| checksum | 7 | 0 | 12 | DONE (real: checksum module) |
| transaction | 0 | 12 | 12 | PENDING WP-2 / WP-3 |
| recovery | 0 | 12 | 12 | PENDING WP-2 / WP-3 |
| brain-seed | 0 | 12 | 12 | PENDING WP-6 |
| brain-build | 0 | 12 | 12 | PENDING WP-6 |
| brain-verify-inspect | 13 | 0 | 12 | DONE (real: inspect + CLI) |
| brain-install-update | 0 | 12 | 12 | PENDING WP-6 |
| sensor-frame | 0 | 12 | 12 | PENDING WP-7 |
| camera-buffer | 0 | 12 | 12 | PENDING WP-7 |
| audio-buffer | 0 | 12 | 12 | PENDING WP-7 |
| perception-fusion | 0 | 12 | 12 | PENDING WP-7 |
| plugin-lifecycle | 0 | 12 | 12 | PENDING WP-7 |
| plugin-isolation | 0 | 12 | 12 | PENDING WP-7 |
| hal-mock | 0 | 12 | 12 | PENDING WP-7 |
| decision-candidate | 0 | 12 | 12 | PENDING WP-8 |
| safety-constraints | 0 | 12 | 12 | PENDING WP-8 |
| feedback-prediction | 0 | 12 | 12 | PENDING WP-10 |
| hebbian-learning | 0 | 12 | 12 | PENDING WP-10 |
| temporal-learning | 0 | 12 | 12 | PENDING WP-10 |
| replay-selection | 0 | 12 | 12 | PENDING WP-10 / WP-11 |
| consolidation-promotion | 0 | 12 | 12 | PENDING WP-10 / WP-11 |
| contradiction-handling | 0 | 12 | 12 | PENDING WP-10 |
| skill-failure | 0 | 12 | 12 | PENDING WP-10 |
| security-validation | 0 | 12 | 12 | PENDING WP-12 / WP-13 |

### Integration Tests (kuota 8/domain)

| Domain | Real | Fake | Kuota | Status |
|--------|------|------|-------|--------|
| 25 domain | 0 | 200 | 8 | PENDING WP-13 |

### E2E Tests

| Domain | Real | Fake | Status |
|--------|------|------|--------|
| 11 domain | 0 | 130 | PENDING WP-9 / WP-13 |

### Fault Injection Tests

| Domain | Real | Fake | Status |
|--------|------|------|--------|
| 8 domain | 0 | 80 | PENDING WP-13 |

### Performance Tests

| Domain | Real | Fake | Status |
|--------|------|------|--------|
| 8 domain | 0 | 60 | PENDING WP-13 |

### Conformance Tests

| Domain | Real | Fake | Status |
|--------|------|------|--------|
| 10 domain | 0 | 100 | PENDING WP-14 |

### Security Tests

| Domain | Real | Fake | Status |
|--------|------|------|--------|
| 7 domain | 0 | 40 | PENDING WP-13 |

---

## Insight

1. **WP-1 berhasil menutupi 5 domain unit dengan test real:** error-taxonomy, config-load, config-validation, logging-tracing, cli-commands. **WP-2 berhasil menutupi 4 domain storage:** brain-header, brain-offset-size, checksum, brain-verify-inspect.
2. **Total real test saat ini: 349** (315 dari WP-1 + 34 dari WP-2 domains). Target global (unit real ≥ 840) akan di-enforce mulai WP-13.
3. **Sisa 1.122 test adalah fake/legacy** yang akan di-triage oleh WP berikutnya.
4. **Duplikasi lintas file sudah tereliminasi** untuk WP-1 domains (fake entries difilter dari catalog).

---

## Rencana Triage Berikutnya

| WP | Domain yang ditangani | Target real |
|----|----------------------|-------------|
| WP-1 | error-taxonomy, config-load, config-validation, logging-tracing, cli-commands | 65+ |
| WP-2 | brain-header, brain-offset-size, checksum, transaction, recovery, brain-seed, brain-build, brain-verify-inspect | 96+ |
| WP-3 | transaction, recovery | 24+ |
| WP-4 | memory-quota, allocator, memory-isolation, retention-scoring, gc-normal, gc-aggressive, gc-emergency | 84+ |
| WP-5 | cell-state, cell-activation, cell-refractory, column-competition, column-sparse, column-association, block-context, block-sequence, block-prediction, synapse-*, sparse-traversal, soa-layout, scalar-kernels, simd-neon, simd-avx, simd-fallback | 180+ |
| WP-6 | brain-seed, brain-build, brain-verify-inspect, brain-install-update | 48+ |
| WP-7 | sensor-frame, camera-buffer, audio-buffer, perception-fusion, plugin-lifecycle, plugin-isolation, hal-mock | 84+ |
| WP-8 | decision-candidate, safety-constraints, feedback-prediction | 36+ |
| WP-9 | core-boot, core-run-loop, core-shutdown, core-emergency-shutdown, core-degraded | 60+ |
| WP-10 | feedback-prediction, hebbian-learning, temporal-learning, replay-selection, consolidation-promotion, contradiction-handling, skill-failure | 84+ |
| WP-11 | replay-selection, consolidation-promotion, retention-scoring, gc-normal, gc-aggressive, gc-emergency, tiering, compression | 96+ |
| WP-12 | cli-commands, diagnostics-telemetry | 24+ |
| WP-13 | Semua domain integration, e2e, fault-injection, performance, security | 480+ |
| WP-14 | Semua domain conformance | 100+ |
