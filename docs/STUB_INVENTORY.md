# STUB INVENTORY

**Audit Date:** 2026-08-14  
**Sesi:** WP-0 (audit awal)  
**Script:** `scripts/audit_stubs.sh`  
**Build Status:** PASS (hijau)  
**Test Status:** PASS (semua 1587 test berjalan, namun mayoritas adalah test kosong/fake)  

## Ringkasan

| Kategori | Jumlah | Status |
|----------|--------|--------|
| Modul kosong / near-empty (< 30 bytes) | 15 | OPEN |
| Fungsi stub dengan `Ok(())` palsu | 26 baris | OPEN |
| Komentar placeholder di kode production | 4 | OPEN |
| Test palsu / tanpa assertion | 1.587 | OPEN |
| `serde` di modul neural/storage (risiko brain.anr) | 10 baris | OPEN |
| Vec unbounded di hot path | 3 baris | OPEN |
| Penyimpangan Architecture Contract | 10+ | OPEN |
| Ketergantungan tanpa justifikasi di DECISIONS.md | 6 | OPEN |

**Total baris inventory: 30+ temuan**  
**STUB_INVENTORY kosong = syarat Product Ready. Saat ini BUKAN kosong.**

---

## 1. Modul Kosong / Near-Empty (< 30 bytes)

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/action/decision.rs` | 1 | modul kosong | WP-8 | OPEN |
| `src/action/safety.rs` | 1 | modul kosong | WP-8 | OPEN |
| `src/action/feedback.rs` | 1 | modul kosong | WP-8 | OPEN |
| `src/hardware/mock.rs` | 1 | modul kosong | WP-7 | OPEN |
| `src/interface/cli.rs` | 1 | modul kosong | WP-12 | OPEN |
| `src/interface/telemetry.rs` | 1 | modul kosong | WP-12 | OPEN |
| `src/learning/consolidation.rs` | 2 | modul kosong | WP-10 / WP-11 | OPEN |
| `src/learning/hebbian.rs` | 2 | modul kosong | WP-10 | OPEN |
| `src/learning/replay.rs` | 2 | modul kosong | WP-10 / WP-11 | OPEN |
| `src/learning/temporal.rs` | 2 | modul kosong | WP-10 | OPEN |
| `src/memory/allocator.rs` | 2 | modul kosong | WP-4 | OPEN |
| `src/simd/scalar.rs` | 1 | modul kosong | WP-5 | OPEN |
| `src/simd/avx.rs` | 1 | modul kosong | WP-5 | OPEN |
| `src/simd/neon.rs` | 1 | modul kosong | WP-5 | OPEN |
| `src/perception/camera.rs` | 1 | modul kosong | WP-7 | OPEN |
| `src/perception/fusion.rs` | 1 | modul kosong | WP-7 | OPEN |
| `src/perception/audio.rs` | 1 | modul kosong | WP-7 | OPEN |
| `src/perception/sensor.rs` | 1 | modul kosong | WP-7 | OPEN |

---

## 2. Fungsi Stub dengan `Ok(())` Palsu

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/storage/builder.rs` | 17 | `build_from_seed` menulis header kosong, tidak parsing seed | WP-6 | OPEN |
| `src/storage/validator.rs` | 23 | `validate` delegasi ke stub `verify_file` | WP-2 | DONE |
| `src/storage/recovery.rs` | 99 | Full recovery implementation (primary/backup) | WP-3 | DONE |
| `src/storage/transaction.rs` | 33 | Full begin/commit/rollback with disk persistence | WP-3 | DONE |
| `src/interface/diagnostics.rs` | 14 | `run_diagnostic` print hardcoded, tidak ada diagnostik nyata | WP-12 | OPEN |
| `src/core/mod.rs` | 132 | `perception_cycle` stub dengan komentar placeholder | WP-9 | OPEN |
| `src/core/mod.rs` | 63, 114, 145 | `load_config`, `boot` (akhir), `shutdown` tidak memanggil subsystem nyata | WP-9 | OPEN |
| `src/core/lifecycle.rs` | 117 | `boot` mengembalikan `Ok(())` meskipun validasi brain di-skip | WP-9 | OPEN |
| `src/core/lifecycle.rs` | 189, 195, 201 | `init_plugins`, `init_neural`, `init_scheduler` stub | WP-9 | OPEN |
| `src/core/state_machine.rs` | 125, 249, 288 | Full 17-state FSM with transition table + safety invariants (95% coverage) | WP-9 | DONE |
| `src/core/scheduler.rs` | 85, 115 | Full backpressure queue (7 policies) + priority scheduler (95% coverage) | WP-9 | DONE |
| `src/storage/header.rs` | 113, 509 | `write` dan `validate` (bagian akhir) tidak menulis data sebenarnya | WP-2 | DONE |

---

## 3. Komentar Placeholder di Kode Production

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/core/mod.rs` | 131 | `// Placeholder for sense/perceive/decide/act cycle` | WP-9 | OPEN |
| `src/core/lifecycle.rs` | 187 | `// Placeholder: actual plugin loading happens in plugins module` | WP-9 | OPEN |
| `src/core/lifecycle.rs` | 193 | `// Placeholder: actual neural core init happens in neural module` | WP-9 | OPEN |
| `src/core/lifecycle.rs` | 199 | `// Placeholder: actual scheduler init happens in core/scheduler module` | WP-9 | OPEN |

---

## 4. Test Palsu / Tanpa Assertion

| file | jumlah test | jenis | WP target | status |
|------|-------------|-------|-----------|--------|
| `tests/unit/group_a_core_tests.rs` | 120 | test kosong `{}` | WP-1 / WP-9 | OPEN |
| `tests/unit/group_b_resource_tests.rs` | 120 | test kosong `{}` | WP-4 / WP-5 | OPEN |
| `tests/unit/group_c_block_tests.rs` | 120 | test kosong `{}` | WP-5 | OPEN |
| `tests/unit/group_d_simd_tests.rs` | 120 | test kosong `{}` | WP-5 | OPEN |
| `tests/unit/group_e_storage_tests.rs` | 120 | test kosong `{}` | WP-2 / WP-3 | OPEN |
| `tests/unit/group_f_provision_tests.rs` | 120 | test kosong `{}` | WP-6 / WP-7 / WP-8 | OPEN |
| `tests/unit/group_g_learning_tests.rs` | 120 | test kosong `{}` | WP-10 / WP-11 / WP-12 | OPEN |
| `tests/integration/all_domains.rs` | 200 | test kosong `{}` | WP-13 | OPEN |
| `tests/e2e/all_scenarios.rs` | 130 | test kosong `{}` | WP-13 | OPEN |
| `tests/fault_injection.rs` | 80 | test kosong `{}` | WP-13 | OPEN |
| `tests/security_extended.rs` | 40 | test kosong `{}` | WP-13 | OPEN |
| `tests/performance.rs` | 60 | test kosong `{}` | WP-13 | OPEN |
| `tests/conformance.rs` | 100 | test kosong `{}` | WP-14 | OPEN |
| `tests/unit/action_tests.rs` | 12 | test kosong `{}` | WP-8 | OPEN |
| `tests/unit/learning_tests.rs` | 12 | test kosong `{}` | WP-10 | OPEN |
| `tests/unit/perception_tests.rs` | 12 | test kosong `{}` | WP-7 | OPEN |
| `tests/unit/plugin_tests.rs` | 12 | test kosong `{}` | WP-7 | OPEN |
| `tests/unit/security_tests.rs` | 12 | test kosong `{}` | WP-13 | OPEN |
| `tests/unit/simd_tests.rs` | 12 | test kosong `{}` | WP-5 | OPEN |
| `tests/unit/core_tests.rs` | 24 | 12 test `Ok(())` tanpa assertion (modul `core_run_loop`) | WP-9 | OPEN |
| `tests/unit/memory_tests.rs` | 12 | 12 test konstruksi objek tanpa assertion | WP-4 | OPEN |
| `tests/unit/storage_tests.rs` | 12 | 12 test dengan assertion (SATU-SATUNYA test group dengan logika) | WP-2 | OPEN |
| `tests/unit/neural_tests.rs` | 12 | 12 test dengan assertion (SATU-SATUNYA test group lain dengan logika) | WP-5 | OPEN |
| `tests/unit/group_a_core_tests.rs` | 2 | `fn placeholder()` di `mod.rs` test (placeholder test) | WP-13 | OPEN |
| `tests/integration/mod.rs` | 1 | `fn placeholder()` di `mod.rs` test | WP-13 | OPEN |
| `tests/e2e/mod.rs` | 1 | `fn placeholder()` di `mod.rs` test | WP-13 | OPEN |

---

## 5. `serde` di Modul Neural/Storage (Risiko brain.anr)

**Konteks:** Direktive Hard Rule #7: "DILARANG serde untuk parsing/serialisasi `brain.anr` (parser binary manual sesuai SD-03). Serde hanya untuk config/seed."

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/neural/cell.rs` | 4, 11, 19 | `use serde` + derive Serialize/Deserialize pada Cell dan CellState | WP-5 | OPEN |
| `src/neural/column.rs` | 4, 9, 16 | `use serde` + derive Serialize/Deserialize pada ColumnState dan Column | WP-5 | OPEN |
| `src/neural/synapse.rs` | 3, 5 | `use serde` + derive Serialize/Deserialize pada Synapse | WP-5 | OPEN |
| `src/neural/block.rs` | 3, 5 | `use serde` + derive Serialize/Deserialize pada BlockState dan Block | WP-5 | OPEN |

**Catatan:** `src/core/mod.rs` juga menggunakan `serde` untuk `RuntimeConfig`, namun ini **BOLEH** karena RuntimeConfig adalah config/seed, bukan brain.anr.

---

## 6. Vec Unbounded di Hot Path

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/neural/cell.rs` | 169 | `Vec::new()` dalam `CellPool::update_all()` — hot neural cycle | WP-5 | OPEN |
| `src/neural/column.rs` | 30 | `Vec::new()` dalam `Column::new()` — unbounded per-column cell index list | WP-5 | OPEN |
| `src/neural/column.rs` | 132 | `Vec::new()` dalam `ColumnPool::winner_take_all_all()` — hot neural cycle | WP-5 | OPEN |

**Catatan:** `src/storage/header.rs:119` menggunakan `Vec::with_capacity(BRAIN_HEADER_SIZE)` — ini **BOUNDED** dan sesuai kontrak.

---

## 7. Penyimpangan Architecture Contract

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/core/state_machine.rs` | 145-149 | Full event-driven transitions implemented; proper initialization events | WP-9 | DONE |
| `src/core/lifecycle.rs` | 68 | `set_brain_valid(true)` dipanggil sebelum validasi brain nyata; melanggar AC §19.1 ("TIDAK BOLEH masuk Running sebelum brain.anr valid") | WP-9 | OPEN |
| `src/core/mod.rs` | 117-128 | `main_loop` adalah infinite sleep loop tanpa sense/perceive/decide/act; melanggar AC §18 Autonomous Loop Contract | WP-9 | OPEN |
| `src/storage/header.rs` | 454-510 | `validate()` tidak memeriksa `total_size`, `generation`, `checksum` match, `allocation_table_*`, `metadata_*`, `section_table_offset`, atau cross-section boundary overlap — AC §44 tidak terpenuhi penuh | WP-2 | DONE |
| `src/storage/builder.rs` | 8-17 | `build_from_seed` hanya menulis header kosong; tidak ada TLV record construction, tidak ada seed parsing, tidak ada section allocation — AC §49 tidak terpenuhi | WP-6 | OPEN |
| `src/storage/transaction.rs` | 82-85 | Full begin/commit/rollback with disk persistence (95% coverage) | WP-3 | DONE |
| `src/storage/recovery.rs` | 8-11 | Full primary/backup recovery with file I/O (95% coverage) | WP-3 | DONE |
| `src/neural/graph.rs` | 7-8 | `SparseGraph` menggunakan `HashMap<u32, Vec<u32>>` untuk adjacency — bukan layout SoA, tidak mendukung SIMD, melanggar AC §16 | WP-5 | OPEN |
| `src/error.rs` | 1-167 | Error taxonomy tidak sesuai SD-16: hilang variant CONFIG, STORAGE, BRAIN, VALIDATION, MEMORY, NEURAL, LEARNING, PERCEPTION, PLUGIN, HAL, ACTUATOR, SAFETY, INTERNAL; tidak ada kode ANR-E-* | WP-1 | OPEN |
| `Cargo.toml` | 29, 34-36 | `tokio` dengan feature `full` (termasuk networking), `anyhow`, `rand`, `parking_lot`, `crossbeam` tanpa justifikasi di DECISIONS.md; `lazy_static`, `once_cell` juga tanpa justifikasi | WP-1 | OPEN |
| `tests/catalog.rs` | 1-251 | `TestCatalog::populate_unit_tests` menghasilkan 840 entri, tetapi implementasi test aktual adalah stub kosong; catalog tidak sesuai dengan test nyata | WP-13 | OPEN |

---

## 8. Ketergantungan Tanpa Justifikasi di DECISIONS.md

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `Cargo.toml` | 34 | `anyhow = "1.0"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |
| `Cargo.toml` | 36 | `rand = "0.8"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |
| `Cargo.toml` | 30 | `parking_lot = "0.12"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |
| `Cargo.toml` | 31 | `crossbeam = "0.8"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |
| `Cargo.toml` | 37 | `lazy_static = "1.4"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |
| `Cargo.toml` | 38 | `once_cell = "1.20"` — tidak ada di whitelist, tidak ada di DECISIONS.md | WP-1 | OPEN |

---

## 9. Katalog Test (tests/catalog.toml) Tidak Ada

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `tests/` | — | `catalog.toml` tidak ada; directive mewajibkan `tests/catalog.toml` (id, level, domain, requirement) | WP-13 | OPEN |

**Catatan:** Saat ini hanya ada `tests/catalog.rs` yang menghasilkan entri statis, bukan file TOML yang diminta kontrak.

---

## 10. Modul Brain Kosong

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/brain/cortex.rs` | 1 | `pub struct Cortex;` dengan `new()` kosong | WP-6 | OPEN |
| `src/brain/cerebellum.rs` | 1 | `pub struct Cerebellum;` dengan `new()` kosong | WP-6 | OPEN |
| `src/brain/hippocampus.rs` | 1 | `pub struct Hippocampus;` dengan `new()` kosong | WP-6 | OPEN |

---

## 11. Modul Learning Kosong

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/learning/mod.rs` | 8 | `pub struct Learning;` (8 baris) | WP-10 | OPEN |
| `src/learning/hebbian.rs` | 2 | `pub struct Hebbian;` | WP-10 | OPEN |
| `src/learning/temporal.rs` | 2 | `pub struct Temporal;` | WP-10 | OPEN |
| `src/learning/replay.rs` | 2 | `pub struct Replay;` | WP-10 / WP-11 | OPEN |
| `src/learning/consolidation.rs` | 2 | `pub struct Consolidation;` | WP-10 / WP-11 | OPEN |

---

## 12. Modul Memory Kosong

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/memory/quota.rs` | — | `MemoryQuota` tanpa enforcement logic (hanya constructor + getter) | WP-4 | OPEN |
| `src/memory/allocator.rs` | 2 | `pub struct Allocator;` | WP-4 | OPEN |
| `src/memory/gc.rs` | — | `GarbageCollector` dengan 3 fungsi `collect_*()` kosong | WP-4 / WP-11 | OPEN |

---

## 13. Modul Perception, Plugins, Hardware Kosong

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `src/perception/sensor.rs` | 1 | `pub struct Sensor;` | WP-7 | OPEN |
| `src/perception/camera.rs` | 1 | `pub struct Camera;` | WP-7 | OPEN |
| `src/perception/audio.rs` | 1 | `pub struct Audio;` | WP-7 | OPEN |
| `src/perception/fusion.rs` | 1 | `pub struct Fusion;` | WP-7 | OPEN |
| `src/plugins/lifecycle.rs` | 1 | `pub struct Lifecycle;` | WP-7 | OPEN |
| `src/plugins/isolation.rs` | 1 | `pub struct Isolation;` | WP-7 | OPEN |
| `src/hardware/mock.rs` | 1 | `pub struct MockHAL;` | WP-7 | OPEN |

---

## 14. Test Nama "network" (Fault Injection Palsu)

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `tests/fault_injection.rs` | 66-84 | 10 test bernama `tc_fault_network_timeout_*` — test kosong `{}`; arsitektur adalah offline-first, tidak ada network dependency | WP-13 | OPEN |

---

## 15. Dokumentasi Overclaim (IMPLEMENTATION_SUMMARY.md)

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `IMPLEMENTATION_SUMMARY.md` | 1-322 | Dokumen mengklaim 1590 test passing dan "COMPLETE", padahal 1557+ test adalah stub kosong tanpa assertion; mengklaim implementasi 12 modul padahal sebagian besar adalah `struct Nama;` | WP-13 / WP-14 | OPEN |

---

## 16. CI Kurang `catalog-check`

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `.github/workflows/ci.yml` | 1-211 | Tidak ada job `catalog-check` sebagaimana diwajibkan oleh directive (Langkah 4: `./scripts/check_test_catalog.sh`) | WP-13 | OPEN |

---

---

## 17. Katalog Test Legacy (tests/catalog.toml)

| file | baris | jenis | WP target | status |
|------|-------|-------|-----------|--------|
| `tests/catalog.toml` | 1-1471 | 315 real, 1.156 fake; 5 domain WP-1 sudah di-triage (error-taxonomy, config-load, config-validation, logging-tracing, cli-commands) | ALL (WP-1 s/d WP-14) | IN PROGRESS |

**Catatan:** `tests/catalog.toml` di-generate otomatis oleh `scripts/gen_catalog.sh` dari `cargo test -- --list`. Setiap WP wajib men-triage domain miliknya: menandai `quality`, mengganti test fake/lemah, menambah test hingga kuota 12 real per domain, lalu menormalkan ID menjadi `TC-U-<DOMAIN>-<SEQ>` dengan `legacy = false`.

---

## 18. Test-Quality Triage (WP-1)

| domain | real | fake | WP target | status |
|--------|------|------|-----------|--------|
| error-taxonomy | 12 | 0 | WP-1 | DONE |
| config-load | 13 | 0 | WP-1 | DONE |
| config-validation | 12 | 0 | WP-1 | DONE |
| logging-tracing | 12 | 0 | WP-1 | DONE |
| cli-commands | 16 | 0 | WP-1 | DONE |

---

## Catatan untuk WP Berikutnya

- **WP-1** harus menutupi: error taxonomy lengkap, dependency whitelist compliance, config default safe, CI fmt/clippy/build/test/catalog-check hijau.
- **WP-2** harus menutupi: header validation lengkap, validator real, checksum BLAKE3, fixtures golden/corrupt, CLI `anr brain init|verify|inspect`.
- **WP-3** harus menutupi: transaction write nyata, generation monotonic, recovery logic, rollback isolation.
- **WP-4** harus menutupi: quota enforcement, slab allocator, section isolation, GC modes.
- **WP-5** harus menutupi: SoA layout, SIMD dispatch, unbounded Vec diganti bounded/arena, serde di neural dihapus/dipindah ke config/seed.
- **WP-6** harus menutupi: Cortex/Cerebellum/Hippocampus dengan fungsi nyata, builder parse seed → neural → brain.anr (TLV SD-03).
- **WP-7** harus menutupi: perception bounded buffers, mock deterministik, plugin lifecycle nyata.
- **WP-8** harus menutupi: decision/safety/actuator dengan SafetyToken, verdict ALLOW/REJECT/CLAMP/OVERRIDE/EMERGENCY_STOP.
- **WP-9** harus menutupi: state machine guard transisi lengkap, scheduler nyata, main_loop sense→perceive→decide→act, boot tanpa `set_brain_valid(true)` palsu.
- **WP-10** harus menutupi: Hebbian, temporal, episode, replay selection.
- **WP-11** harus menutupi: consolidation, retention scoring, GC pressure states.
- **WP-12** harus menutupi: CLI lengkap `status|memory|inspect|brain install`, telemetry metrics lokal.
- **WP-13** harus menutupi: 80 fault injection test nyata, 40 security test nyata, 60 performance test nyata, catalog.toml terdaftar, semua test di CI.
- **WP-14** harus menutupi: conformance report, golden brain release, endurance script, STUB_INVENTORY = 0 open.
---

## 19. Half-Done Files (Coverage <10%)

| file | baris | fungsi real / stub | liputan | WP target | status |
|------|-------|--------------------|---------|-----------|--------|
| `src/brain/cortex.rs` | 15 | 0 / 1 | ~0% | WP-6 | OPEN |
| `src/brain/cerebellum.rs` | 15 | 0 / 1 | ~0% | WP-6 | OPEN |
| `src/brain/hippocampus.rs` | 15 | 0 / 1 | ~0% | WP-6 | OPEN |
| `src/brain/mod.rs` | 31 | 0 / 1 | ~5% | WP-6 | OPEN |
| `src/memory/gc.rs` | 9 | 0 / 3 | ~0% | WP-4 / WP-11 | OPEN |
| `src/interface/diagnostics.rs` | 19 | 0 / 1 | ~5% | WP-12 | OPEN |
| `src/neural/mod.rs` | 18 | 0 / 0 | ~5% | WP-5 | OPEN |

**Catatan:** File ini hanya berisi struct kosong tanpa metode, fungsi `Ok(())` tanpa logika, atau modul deklarasi tanpa implementasi. Semua perlu implementasi nyata sebelum Product Ready.

---

## 20. Revert/Absorb Decisions (WP-1R Protocol)

| File | Keputusan | Alasan | WP |
|------|-----------|--------|-----|
| `src/core/lifecycle.rs` | **Absorb** | Boot validation + recovery integration real (55-60% coverage). 3 init_* stubs are WP-9 scope. | WP-9 |
| `src/core/scheduler.rs` | **Absorb** | Full backpressure queue (7 policies) + priority scheduler (95% coverage). Not a stub. | WP-9 |
| `src/core/state_machine.rs` | **Absorb** | Full 17-state FSM with transition table + safety invariants (95% coverage). Not a stub. | WP-9 |
| `src/storage/transaction.rs` | **Absorb** | Full begin/commit/rollback with disk persistence (95% coverage). Not a stub. | WP-3 |
| `src/storage/recovery.rs` | **Absorb** | Full primary/backup recovery with file I/O (95% coverage). Not a stub. | WP-3 |
| `src/interface/diagnostics.rs` | **Revert scope** | Pure stub (5% coverage). WP-12 will implement from scratch. Current code is just println. | WP-12 |
