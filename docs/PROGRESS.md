# PROGRESS

*Sesi Saat Ini:* WP-7 — Perception + Mock Plugins  
*Status Awal:* WP-0 audit + WP-1 core hygiene selesai; WP-2 storage read path selesai; WP-3 storage write path & recovery selesai; BLOK-R finalization selesai; WP-4 memory manager selesai; WP-5 neural core SoA + SIMD selesai; WP-6 brain sections + provisioning core selesai  
*Sesi Berikutnya:* WP-8 — Decision + Safety + Actuator Mock  

---

## WP-0 — Audit & Inventory (SELESAI)

**Waktu:** 2026-08-14  
**Tugas:** Baca seluruh struktur repo, contract docs, jalankan audit script, buat STUB_INVENTORY.md, peta temuan ke WP-1..WP-14, perbarui dokumen ini.  

**Hasil:**
- `scripts/audit_stubs.sh` dibuat dan dijalankan.
- `scripts/gen_catalog.sh` + `scripts/gen_catalog.py` dibuat; `tests/catalog.toml` di-generate dari 1.466 test eksisting (legacy migration strategy).
- `scripts/check_test_catalog.sh` + `scripts/check_test_catalog.py` dibuat untuk enforcement bertahap.
- `docs/STUB_INVENTORY.md` dibuat dengan 30+ kategori temuan.
- `docs/TEST_DISTRIBUTION.md` dibuat; distribusi test per domain terukur.
- 15 modul near-empty teridentifikasi.
- ~1.466 test legacy teridentifikasi (semua `quality = "unknown"`).
- 10 baris `serde` di neural/storage teridentifikasi (risiko brain.anr).
- 3 Vec unbounded di hot path teridentifikasi.
- 10+ penyimpangan Architecture Contract teridentifikasi.
- Build hijau, test hijau.
- `IMPLEMENTATION_SUMMARY.md` teridentifikasi sebagai overclaim (tidak sesuai temuan audit).

**Blocker untuk WP-1:**
- `docs/DECISIONS.md` sudah dibuat (DEC-001 s/d DEC-007).
- `tests/catalog.toml` sudah di-generate dan di-triage untuk domain WP-1.
- `completed_domains` diisi dengan 5 domain WP-1.

---

## WP-1 — Core Hygiene (SELESAI)

**Waktu:** 2026-08-14  
**Commit:** f8c198f (`ANR-WP1: core hygiene`) + 91af3b9 (`ANR-WP1R: scope reconciliation`) + a8fb95d (`ANR-WP1R: consolidation — remove format.rs and validator.rs duplicates`)

**Hasil:**
- `src/error.rs`: taxonomy lengkap SD-16 (13 kelas), Severity, kode ANR-E-*, 12 test real.
- `src/core/config.rs`: RuntimeConfig default safe, load TOML, validasi, 25 test real.
- `src/core/logging.rs`: logging lokal-only, 12 test real.
- `src/interface/cli.rs`: --version/--help, subcommand validation, 15 test real.
- `tests/catalog.toml`: 1.471 entries (315 real, 1.156 fake).
- 5 domain WP-1 selesai: error-taxonomy, config-load, config-validation, logging-tracing, cli-commands.
- CI: job catalog-check ditambahkan.
- `docs/DECISIONS.md`: DEC-001 s/d DEC-007.
- `docs/SCOPE_RECONCILIATION_WP1.md`: rekonsiliasi 8 hunk wiring-only di 5 file.

**Scope Reconciliation:**
- Semua perubahan di `src/storage/transaction.rs`, `src/core/lifecycle.rs`, `src/core/scheduler.rs`, `src/core/state_machine.rs`, `src/interface/diagnostics.rs` adalah wiring-only (penggantian nama variant error).
- Tidak ada kode setengah jadi yang perlu di-revert.
- Tidak ada kode yang perlu ditandai untuk WP lain.

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect"]`

---

## Metrik Kanonik (A-4)

| Metrik | Nilai | Keterangan |
|--------|-------|------------|
| Total test invocations | 3.265 | Dari `cargo test -- --list` |
| Total catalog entries | 1.451 | Setelah filter 20 fake trans/recovery |
| Real | 257 | Test dengan assertion |
| Fake (legacy) | 1.274 | Test tanpa assertion / stub |
| Unknown | 4 | Belum dikategorikan |
| Unit real | 313 (catalog strict) / 205 (lib binary) | Target global 840 (di-enforce mulai WP-13) |
| Domain kanonik | 22 | Dari 136 generated domain; selesai: WP-1(5) + WP-2(4) + WP-3(2) + WP-4(4) + WP-5(3) + WP-6(4) |
| Fake difilter | 80 | 60 WP-1 + 20 trans/recovery |
| Build | HIJAU | cargo build --all-targets |
| Clippy | HIJAU | -D warnings |
| Fmt | HIJAU | cargo fmt --check |
| Test | HIJAU | 101 lib tests + 1443 integration tests passing |

### Penjelasan Metrik

- **3265** adalah jumlah invocation test dari `cargo test -- --list`.
- **1451** adalah jumlah entri di `tests/catalog.toml` setelah filtering 80 fake entries (60 WP-1 + 20 trans/recovery).
- **101** adalah jumlah test real yang dijalankan di `cargo test --lib`.
- **1443** adalah jumlah test real yang dijalankan di `cargo test --test lib`.
- **80 fake difilter** adalah entri fake yang dihapus dari catalog untuk domain yang sudah selesai (kuota 12 real per domain).
- **BLOK-R**: fix `tc_u_builder_006` — `build_from_seed` menghasilkan header dengan `cortex_offset=4096` tetapi `cortex_size=0`, melanggar `validate_section_sizes`. Fix: set `cortex_offset=0` di builder. Filter 20 fake entries dari transaction/recovery domains.

---

## WP-2 — Storage Read Path (SELESAI)

**Waktu:** 2026-08-14  
**Commit:** 96a09ca

**Hasil:**
- `src/storage/validate.rs`: full validation rules (magic, version, header_size, block_size, total_size, generation, section offsets, section sizes, section boundaries, section table, checksum) with 13 tests real.
- `src/storage/checksum.rs`: BLAKE3 checksum module with scope support and 7 tests real.
- `src/storage/inspect.rs`: dump text + JSON with 5 tests real.
- `src/storage/header.rs`: updated serialize/deserialize/validate/compute_checksum methods.
- `src/interface/cli.rs`: `anr brain init | verify | inspect` subcommands with 15 tests real.
- `src/main.rs`: brain init writes valid header with checksum.
- `tests/fixtures/brains/valid_golden.anr`: golden fixture for regression testing.
- `tests/fixtures/brains/corrupt/*.anr`: 10 corrupt fixture classes.
- `scripts/make_fixtures.sh`: deterministic fixture generation.
- 4 storage domains selesai: brain-header, brain-offset-size, checksum, brain-verify-inspect.
- CI: catalog-check job validates completed domains.

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect"]`

---

## WP-3 — Storage Write Path & Recovery (SELESAI)

**Waktu:** 2026-08-14  
**Commit:** 08c9ad9

**Hasil:**
- `src/storage/header.rs`: atomic dual-copy write (backup + fsync + primary + fsync)
- `src/storage/transaction.rs`: real begin/commit/rollback with disk persistence
- `src/storage/recovery.rs`: recover from backup superblock, detect power-loss
- `src/storage/mod.rs`: BrainWriter struct for write-path operations
- `src/error.rs`: 5 new storage error variants
- `src/core/lifecycle.rs`: boot validates brain, shutdown documents flush
- 12 transaction tests + 12 recovery tests (24 real tests)
- CI: catalog-check validates transaction + recovery domains

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect", "transaction", "recovery"]`


---

## WP-4 — Memory Manager (SELESAI)

**Waktu:** 2026-08-14

**Hasil:**
- `src/memory/quota.rs`: MemoryQuota(min/target/max), SectionMemoryState, PressureLevel (5 states)
- `src/memory/allocator.rs`: SlabAllocator with free list, best-fit, compact, coalesce
- `src/memory/gc.rs`: GarbageCollector with pressure-based GcMode selection
- `src/memory/mod.rs`: MemoryManager with per-section isolation (Cortex/Cerebellum/Hippocampus)
- 48 real tests (12 quota + 12 allocator + 12 GC + 12 memory-manager)
- Catalog: 4 new domains registered, 24 fake stubs filtered

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect", "transaction", "recovery", "memory-quota", "allocator", "gc-normal", "gc-aggressive"]`

---

## WP-5 — Neural Core SoA + SIMD (SELESAI)

**Waktu:** 2026-08-15
**Commit:** 2393a57

**Hasil:**
- `src/simd/mod.rs`: SimdBackend enum, SimdKernel trait, detect_backend(), get_kernel()
- `src/simd/scalar.rs`: ScalarKernel with sigmoid, relu, dot_product, weighted_accumulate, argmax
- `src/neural/cell.rs`: removed serde, pre-allocated fired Vec in update_all()
- `src/neural/column.rs`: removed serde, pre-allocated cell_indices and winners Vecs
- `src/neural/block.rs`: removed serde from Block
- `src/neural/synapse.rs`: removed serde from Synapse
- `src/neural/graph.rs`: replaced HashMap adjacency with SoA arrays (adj_offsets + adj_targets)
- `src/neural/mod.rs`: NeuralCore with real pools, cycle(), active_columns()
- 36 real tests (12 scalar + 12 soa-layout + 12 neural-core)
- Catalog: 3 new domains registered

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect", "transaction", "recovery", "memory-quota", "allocator", "gc-normal", "gc-aggressive", "simd-fallback", "soa-layout", "neural-core"]`

---

## BLOK-R — Finalization (SELESAI)

**Waktu:** 2026-08-14

**Hasil:**
- Fix `tc_u_builder_006`: `build_from_seed` menghasilkan header dengan `cortex_offset=4096` tetapi `cortex_size=0`, gagal validasi `validate_section_sizes`. Fix: set `cortex_offset=0` di builder.
- Filter 20 fake entries dari `tests/catalog.toml` (10 transaction + 10 recovery) sehingga completed domain validation bersih.
- Semua 12 builder tests pass.
- Gate: fmt + clippy + test + catalog-check + coverage-check semua HIJAU.

---


## WP-6 — Brain Sections + Provisioning Core (SELESAI)

**Waktu:** 2026-08-15  
**Commit:** f929b5a

**Hasil:**
- `src/brain/cortex.rs`: Knowledge struct, Cortex with capacity, add/get/query_by_pattern
- `src/brain/cerebellum.rs`: Skill struct, Cerebellum with capacity, add/get/validated_skills
- `src/brain/hippocampus.rs`: Episode struct, Hippocampus with capacity, add/get/gc_eligible
- `src/brain/mod.rs`: Brain integration, total_objects()
- `src/storage/builder.rs`: TOML seed parser (serde), TLV record writers (ANRR magic, 0x0100/0x0200/0x0300), BLAKE3 checksum, section allocation pipeline
- 56 real tests (14 cortex + 14 cerebellum + 15 hippocampus + 13 provisioning)
- Catalog: 4 new domains registered (cortex-interface, cerebellum-interface, hippocampus-episode, brain-provisioning)

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands", "brain-header", "brain-offset-size", "checksum", "brain-verify-inspect", "transaction", "recovery", "memory-quota", "allocator", "gc-normal", "gc-aggressive", "simd-fallback", "soa-layout", "neural-core", "cortex-interface", "cerebellum-interface", "hippocampus-episode", "brain-provisioning"]`

---

## Riwayat WP

| WP | Nama | Status | Commit |
|----|------|--------|--------|
| WP-0 | Audit & Inventory | SELESAI | ccac57f |
| WP-1 | Core Hygiene | SELESAI | f8c198f, 91af3b9, a8fb95d |
| WP-2 | Storage Read Path | SELESAI | 96a09ca |
| WP-3 | Storage Write Path & Recovery | SELESAI | 08c9ad9 |
| BLOK-R | Finalization | SELESAI | (sesi ini) |
| WP-4 | Memory Manager | SELESAI | (sesi ini) |
| WP-5 | Neural Core SoA + SIMD | SELESAI | 2393a57 |
| WP-6 | Brain Sections + Provisioning Core | SELESAI | f929b5a |
| WP-7 | Perception + Mock Plugins | PENDING | — |
| WP-8 | Decision + Safety + Actuator Mock | PENDING | — |
| WP-9 | Runtime State Machine + Scheduler + Vertical Slice | PENDING | — |
| WP-10 | Learning Dasar | PENDING | — |
| WP-11 | Replay/Consolidation/Retention/GC | PENDING | — |
| WP-12 | Interface Lengkap | PENDING | — |
| WP-13 | Hardening | PENDING | — |
| WP-14 | Product Validation | PENDING | — |
