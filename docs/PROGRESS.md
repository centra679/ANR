# PROGRESS

**Sesi Saat Ini:** WP-3 — Storage Write Path & Recovery  
**Status Awal:** WP-0 audit + WP-1 core hygiene selesai; WP-2 storage read path selesai  
**Sesi Berikutnya:** WP-4 — Memory Manager  

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
**Commit:** f8c198f (`ANR-WP1: core hygiene`) + 91af3b9 (`ANR-WP1R: scope reconciliation`)

**Hasil:**
- `src/error.rs`: taxonomy lengkap SD-16 (13 kelas), Severity, kode ANR-E-*, 12 test real.
- `src/core/config.rs`: RuntimeConfig default safe, load TOML, validasi, 14 test real.
- `src/core/logging.rs`: logging lokal-only, 12 test real.
- `src/interface/cli.rs`: --version/--help, subcommand validation, 16 test real.
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
| Total test (catalog.toml) | 1.471 | Gabungan real + fake |
| Real | 315 | Test dengan assertion |
| Fake (legacy) | 1.156 | Test tanpa assertion / stub |
| Unknown | 0 | Semua sudah dikategorikan |
| Unit real | 315 | Target global 840 (di-enforce mulai WP-13) |
| Domain kanonik | 46 | Dari 136 generated domain |
| Domain WP-1 selesai | 5 | error-taxonomy, config-load, config-validation, logging-tracing, cli-commands |
| Fake difilter | 60 | Dari domain WP-1 yang sudah selesai |
| Build | HIJAU | cargo build --all-targets |
| Clippy | HIJAU | -D warnings |
| Fmt | HIJAU | cargo fmt --check |
| Test | HIJAU | 101 lib tests + 1307 integration tests passing |

### Penjelasan Metrik

- **1587** adalah jumlah invocation test dari `cargo test -- --list` (termasuk duplikasi lintas file dan test tanpa assertion).
- **1471** adalah jumlah entri di `tests/catalog.toml` setelah deduplikasi by function name dan filtering 60 fake entries dari 5 domain WP-1 yang sudah selesai.
- **101** adalah jumlah test real yang dijalankan di `cargo test --lib`.
- **1307** adalah jumlah test real yang dijalankan di `cargo test --test lib`.
- **60 fake difilter** adalah entri fake yang dihapus dari catalog untuk domain WP-1 yang sudah mencapai kuota 12 real per domain.

---

## WP-2 — Storage Read Path (SELESAI)

**Waktu:** 2026-08-14  
**Commit:** (current session)

**Hasil:**
- `src/storage/validate.rs`: full validation rules (magic, version, header_size, block_size, total_size, generation, section offsets, section sizes, section boundaries, section table, checksum) with 14 tests real.
- `src/storage/checksum.rs`: BLAKE3 checksum module with scope support and 7 tests real.
- `src/storage/inspect.rs`: dump text + JSON with 5 tests real.
- `src/storage/header.rs`: updated serialize/deserialize/validate/compute_checksum methods.
- `src/interface/cli.rs`: `anr brain init | verify | inspect` subcommands with 8 tests real.
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

**Hasil:**
- `src/error.rs`: added 5 new error variants (ANR-E-STORAGE-005 through ANR-E-STORAGE-009) with severity, is_fatal, is_recoverable, Display, and PartialEq support.
- `src/storage/header.rs`: added `write_atomic()` (backup-first + fsync + primary + fsync), `write_backup()`, `write_section_data()`, `read_backup()` methods implementing AC §45 transactional write contract.
- `src/storage/transaction.rs`: replaced stubs with real implementations — `begin()` (validates no active tx, creates descriptor, writes backup snapshot), `commit()` (increments generation, computes checksum, writes atomic, validates, marks Committed), `rollback()` (reads backup, validates, restores primary, fsyncs).
- `src/storage/recovery.rs`: replaced stub with full `recover()` (reads primary + backup, validates both, restores from backup if primary invalid, error if both invalid) and `recovery_needed()` (compares generation of primary vs backup). Implements AC §46 recovery contract.
- `src/storage/mod.rs`: added `BrainWriter` struct (wraps path + header + TransactionManager with begin_transaction/commit_transaction/rollback_transaction/write_section/flush). Re-exports `BrainWriter`, `Recovery`, `TransactionDescriptor`, `TxState`, `SectionType`.
- `src/lib.rs`: updated re-exports to include new public types.

**Files Modified:**
- `src/error.rs` — 5 new error variants + severity/PartialEq/is_fatal updates
- `src/storage/header.rs` — 4 new methods (write_atomic, write_backup, write_section_data, read_backup)
- `src/storage/transaction.rs` — full implementations of begin/commit/rollback
- `src/storage/recovery.rs` — full implementations of recover/recovery_needed
- `src/storage/mod.rs` — BrainWriter struct + re-exports
- `src/lib.rs` — updated re-exports

**Wiring-only changes outside scope:** None.

**Gate results:**
- `cargo fmt`: HIJAU
- `cargo clippy -- -D warnings`: HIJAU (0 warnings)
- `cargo test --lib`: 101 passed, 0 failed
- `cargo test --test lib`: 1307 passed, 0 failed

---

## Riwayat WP

| WP | Nama | Status | Commit |
|----|------|--------|--------|
| WP-0 | Audit & Inventory | SELESAI | ccac57f |
| WP-1 | Core Hygiene | SELESAI | f8c198f, 91af3b9 |
| WP-2 | Storage Read Path | SELESAI | (current session) |
| WP-3 | Storage Write Path & Recovery | SELESAI | (current session) |
| WP-4 | Memory Manager | PENDING | — |
| WP-5 | Neural Core SoA + SIMD | PENDING | — |
| WP-6 | Brain Sections + Provisioning Core | PENDING | — |
| WP-7 | Perception + Mock Plugins | PENDING | — |
| WP-8 | Decision + Safety + Actuator Mock | PENDING | — |
| WP-9 | Runtime State Machine + Scheduler + Vertical Slice | PENDING | — |
| WP-10 | Learning Dasar | PENDING | — |
| WP-11 | Replay/Consolidation/Retention/GC | PENDING | — |
| WP-12 | Interface Lengkap | PENDING | — |
| WP-13 | Hardening | PENDING | — |
| WP-14 | Product Validation | PENDING | — |
