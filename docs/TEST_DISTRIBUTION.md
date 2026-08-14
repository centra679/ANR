# TEST DISTRIBUTION

**Audit Date:** 2026-08-14  
**Sesi:** WP-0 (audit awal)  
**Sumber:** `scripts/gen_catalog.sh` → `tests/catalog.toml`  
**Konteks:** Strategi legacy migration untuk 1.466 test eksisting  

---

## Ringkasan Global

| Metric | Nilai |
|--------|-------|
| Total test (legacy) | 1.466 |
| Unit | 852 |
| Integration | 200 |
| E2E | 130 |
| Fault-injection | 80 |
| Performance | 60 |
| Conformance | 100 |
| Security | 40 |
| Admin / catalog-admin | 4 |
| **Real (quality = "real")** | **0** |
| **Fake (quality = "fake")** | **0** |
| **Unknown (quality = "unknown")** | **1.466** |

---

## Distribusi per Domain vs Kuota

### Unit Tests (kuota 12/domain)

| Domain | Jumlah | Kuota | Status |
|--------|--------|-------|--------|
| core_boot | 24 | 12 | OVER (duplikasi lintas file) |
| plugin_isolation | 22 | 12 | OVER (duplikasi lintas file) |
| recovery | 22 | 12 | OVER (duplikasi lintas file) |
| transaction | 22 | 12 | OVER (duplikasi lintas file) |
| 71 domain lainnya | 12 each | 12 | ok |
| **Total unit** | **852** | — | — |

**Catatan:** 4 domain memiliki jumlah ganda karena definisi test yang sama muncul di lebih dari satu file (mis. `core_boot` ada di `group_a_core_tests` dan `core_tests`). Ini menandakan duplikasi yang harus di-triage oleh WP-1.

### Integration Tests (kuota 8/domain)

| Domain | Jumlah | Kuota | Status |
|--------|--------|-------|--------|
| 25 domain | 8 each | 8 | ok |
| **Total integration** | **200** | — | — |

### E2E Tests (kuota tidak diatur di directive)

| Domain | Jumlah |
|--------|--------|
| boot_invalid_brain | 12 |
| boot_missing_brain | 12 |
| boot_valid_brain | 12 |
| emergency_shutdown | 12 |
| graceful_shutdown | 12 |
| perception_cycle | 12 |
| recovery_corrupt_generation | 12 |
| state_safety_initialization | 12 |
| transition_degraded | 12 |
| transition_emergency_stop | 12 |
| learning_cycle | 10 |
| **Total E2E** | **130** |

### Fault Injection Tests

| Domain | Jumlah |
|--------|--------|
| corruption | 10 |
| io_failure | 10 |
| memory_exhaustion | 10 |
| network_timeout | 10 |
| partial_failure | 10 |
| plugin_crash | 10 |
| resource_depletion | 10 |
| safety_violation | 10 |
| **Total** | **80** |

### Performance Tests

| Domain | Jumlah |
|--------|--------|
| decision_time | 8 |
| general | 4 |
| learning_speed | 8 |
| memory_allocation | 8 |
| neural_throughput | 8 |
| perception_latency | 8 |
| safety_overhead | 8 |
| storage_io | 8 |
| **Total** | **60** |

### Conformance Tests

| Domain | Jumlah |
|--------|--------|
| bounded_memory | 10 |
| determinism | 10 |
| graceful_degradation | 10 |
| offline_first | 10 |
| plugin_isolation | 10 |
| recovery | 10 |
| safety_first | 10 |
| single_binary | 10 |
| single_brain | 10 |
| transaction | 10 |
| **Total** | **100** |

### Security Tests

| Domain | Jumlah |
|--------|--------|
| buffer_overflow | 6 |
| general | 4 |
| input_validation | 6 |
| isolation | 6 |
| memory_safety | 6 |
| permission_checks | 6 |
| unsafe_audit | 6 |
| **Total** | **40** |

---

## Insight

1. **Angka 1.466 adalah gabungan test nyata + test duplikasi lintas file + test palsu.** Saat ini seluruh test ditandai `quality = "unknown"` karena belum di-triage.
2. **Duplikasi teridentifikasi pada 4 domain unit** (`core_boot`, `plugin_isolation`, `recovery`, `transaction`) — setiap domain muncul di 2 file berbeda dengan nama test yang sedikit berbeda (mis. `tc_u_core_boot_001` vs `tc_u_core_boot_001_boot_sequence_starts`).
3. **Tidak ada test dengan assertion** di sebagian besar file (lihat `docs/STUB_INVENTORY.md`). Jumlah 1.466 adalah kapasitas test yang ada, bukan kualitas test yang terverifikasi.
4. **Kuota unit 840 terpenuhi secara jumlah**, tetapi seluruhnya berstatus `unknown`/`fake` sampai WP-1 dan WP-13 menyelesaikan triage.

---

## Rencana Triage per WP

| WP | Domain yang ditangani | Tujuan |
|----|----------------------|--------|
| WP-1 | core-boot, config-load, error-taxonomy, logging-tracing, scheduler-priority | Ganti fake test → real; tandai `quality = "real"` |
| WP-2 | brain-header, brain-offset-size, checksum, transaction, recovery, brain-seed, brain-build, brain-verify-inspect | Triage storage read path |
| WP-3 | transaction, recovery | Triage storage write path |
| WP-4 | memory-quota, allocator, memory-isolation, retention-scoring, gc-normal, gc-aggressive, gc-emergency | Triage memory manager |
| WP-5 | cell-state, cell-activation, cell-refractory, column-competition, column-sparse, column-association, block-context, block-sequence, block-prediction, synapse-*, sparse-traversal, soa-layout, scalar-kernels, simd-neon, simd-avx, simd-fallback | Triage neural core + SIMD |
| WP-6 | brain-seed, brain-build, brain-verify-inspect, brain-install-update | Triage provisioning |
| WP-7 | sensor-frame, camera-buffer, audio-buffer, perception-fusion, plugin-lifecycle, plugin-isolation, hal-mock | Triage perception + plugins |
| WP-8 | decision-candidate, safety-constraints, feedback-prediction | Triage decision/safety |
| WP-9 | core-boot, core-run-loop, core-shutdown, core-emergency-shutdown, core-degraded | Triage runtime state machine |
| WP-10 | feedback-prediction, hebbian-learning, temporal-learning, replay-selection, consolidation-promotion, contradiction-handling, skill-failure | Triage learning |
| WP-11 | replay-selection, consolidation-promotion, retention-scoring, gc-normal, gc-aggressive, gc-emergency, tiering, compression | Triage replay/consolidation/retention/GC |
| WP-12 | cli-commands, diagnostics-telemetry | Triage interface |
| WP-13 | Semua domain integration, e2e, fault-injection, performance, security | Ganti seluruh fake test → real; kuota 12 real/domain |
| WP-14 | Semua domain conformance | Validasi akhir; tandai selesai |

---

##catatan untuk Enforcement

- `scripts/check_test_catalog.sh` akan **gagal** jika ada domain di `completed_domains` yang masih memiliki `quality = "fake"` atau < 12 `quality = "real"`.
- Enforcement **global** (unit real ≥ 840, total real ≥ 1440) hanya aktif mulai **WP-13**.
- Daftar `completed_domains` dicatat di `docs/PROGRESS.md`.
