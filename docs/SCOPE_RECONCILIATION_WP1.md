# SCOPE_RECONCILIATION_WP1.md

**Tujuan:** Rekonsiliasi scope commit `f8c198f` (ANR-WP1: core hygiene) terhadap directive WP-1.  
**Metodologi:** Per-file, per-hunk review → kategori + disposisi.  
**Kepatuhan Directive:** A-1 (scope), A-2 (reconciliation), A-3 (disposition).  

---

## 0. Konfirmasi Commit WP-0

| Commit | Hash | Status |
|--------|------|--------|
| ANR-WP0: audit & stub inventory | `ccac57f` | ✅ Ada |

**Kesimpulan:** Artifact audit (STUB_INVENTORY.md, PROGRESS.md, audit_stubs.sh, catalog generator, TEST_DISTRIBUTION.md) termuat di commit WP-0 dan tidak perlu di-replay.

---

## 1. File Review: Wiring-Only Changes

Semua hunk di 5 file berikut adalah **penggantian nama variant error** agar kompatibel dengan taxonomy WP-1 yang baru. Tidak ada implementasi baru, tidak ada stub, tidak ada perubahan logika.

### 1.1 `src/storage/transaction.rs`

| Hunk | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| 1 | `Error::TransactionError(...)` → `Error::InternalTransactionError(...)` | wiring-only | KEEP — sudah absorbed oleh WP-1 |

### 1.2 `src/core/lifecycle.rs`

| Hunk | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| 1 | `Error::RuntimeBootFailed(...)` → `Error::InternalRuntimeBootFailed(...)` | wiring-only | KEEP — sudah absorbed oleh WP-1 |
| 2 | `Error::RuntimeShutdownFailed(...)` → `Error::InternalRuntimeShutdownFailed(...)` | wiring-only | KEEP — sudah absorbed oleh WP-1 |
| 3 | `Error::RuntimeEmergencyStopFailed(...)` → `Error::InternalRuntimeEmergencyStopFailed(...)` | wiring-only | KEEP — sudah absorbed oleh WP-1 |

### 1.3 `src/core/scheduler.rs`

| Hunk | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| 1 | `Error::Other("Queue full, rejected")` → `Error::InternalOther("Queue full, rejected")` | wiring-only | KEEP — sudah absorbed oleh WP-1 |
| 2 | `Error::Other("Queue full, block")` → `Error::InternalOther("Queue full, block")` | wiring-only | KEEP — sudah absorbed oleh WP-1 |
| 3 | `Error::Other("Unknown priority class")` → `Error::InternalOther("Unknown priority class")` | wiring-only | KEEP — sudah absorbed oleh WP-1 |

### 1.4 `src/core/state_machine.rs`

| Hunk | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| 1 | `Error::RuntimeStateTransitionInvalid(...)` → `Error::InternalRuntimeStateTransitionInvalid(...)` | wiring-only | KEEP — sudah absorbed oleh WP-1 |

### 1.5 `src/interface/diagnostics.rs`

| Hunk | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| 1 | `Error::Other("Unknown diagnostic")` → `Error::InternalOther("Unknown diagnostic")` | wiring-only | KEEP — sudah absorbed oleh WP-1 |

---

## 2. File Review: Real Implementations (WP-1 Scope)

| File | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| `src/error.rs` | Enum Error lengkap dengan 13 kelas SD-16, kode ANR-E-*, Severity, Display/Error, PartialEq manual | real-implementation | KEEP — inti WP-1 |
| `src/core/config.rs` | RuntimeConfig default safe, load_from_toml, validate, 14 test real | real-implementation | KEEP — inti WP-1 |
| `src/core/logging.rs` | init_logging, set_log_level, 12 test real | real-implementation | KEEP — inti WP-1 |
| `src/interface/cli.rs` | Cli struct, Commands enum, validate(), 16 test real | real-implementation | KEEP — inti WP-1 |

---

## 3. File Review: Test Infrastructure

| File | Perubahan | Kategori | Disposisi |
|------|-----------|----------|-----------|
| `tests/catalog.toml` | 1.471 entries (315 real, 1.156 fake), 5 domain WP-1 selesai | test-infrastructure | KEEP — WP-1 deliverable |
| `scripts/gen_catalog.py` | Generator dengan module-domain mapping + quality triage | test-infrastructure | KEEP — WP-1 deliverable |
| `scripts/check_test_catalog.py` | Enforcement bertahap + completed_domains parsing | test-infrastructure | KEEP — WP-1 deliverable |
| `scripts/filter_fake_tests.py` | Filter fake entries dari completed domains | test-infrastructure | KEEP — WP-1 deliverable |
| `docs/TEST_DISTRIBUTION.md` | Distribusi test per domain | test-infrastructure | KEEP — WP-1 deliverable |

---

## 4. Rekonsiliasi: Tidak Ada Kode Setengah Jadi

| Kriteria | Temuan |
|----------|--------|
| `todo!()` / `unimplemented!()` | 0 di commit WP-1 |
| `Ok(())` palsu | 0 baru di WP-1 |
| Komentar placeholder | 0 baru di WP-1 |
| Test tanpa assertion | 0 baru di WP-1 (semua test WP-1 memiliki assertion) |
| Wiring-only changes | 5 file, 8 hunk — semua legitimate error taxonomy migration |
| Half-done implementation | TIDAK ADA |

**Kesimpulan:** Commit `f8c198f` bersih. Tidak perlu revert. Tidak ada kode setengah jadi yang dibiarkan.

---

## 5. Disposisi Eksekusi

| Aksi | File | Status |
|------|------|--------|
| KEEP | Semua file di commit WP-1 | DIPATUHI |
| REVERT | — | TIDAK DIPERLUKAN |
| Tandai untuk WP<n> | — | TIDAK DIPERLUKAN |

---

## 6. Konfirmasi Blocker

| Blocker | Status |
|---------|--------|
| WP-0 commit ada | ✅ `ccac57f` |
| DECISIONS.md | ✅ Dibuat di WP-1 |
| tests/catalog.toml | ✅ Di-generate dan di-triage |
| completed_domains | ✅ 5 domain WP-1 |

---

## 7. Siap untuk WP-2

Berdasarkan rekonsiliasi ini, repo dalam keadaan bersih untuk melanjutkan ke WP-2 (Storage Read Path). Tidak ada kode setengah jadi yang perlu dibersihkan terlebih dahulu.
