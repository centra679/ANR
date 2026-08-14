# PROGRESS

**Sesi Saat Ini:** WP-0 — Audit & Inventory  
**Status Awal:** Audit selesai, STUB_INVENTORY dibuat, catalog strategi legacy migration disiapkan  
**Sesi Berikutnya:** WP-2 — Storage Read Path  

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

**Prioritas:** TINGGI  
**Target:**
- `src/error.rs`: enum `AnrError` lengkap dengan taxonomy SD-16 (CONFIG, STORAGE, BRAIN, VALIDATION, MEMORY, NEURAL, LEARNING, PERCEPTION, PLUGIN, HAL, ACTUATOR, SAFETY, INTERNAL), Severity, kode `ANR-E-*`, impl Display/Error, konversi From.
- `src/core/config.rs`: RuntimeConfig default safe + load TOML opsional + validasi.
- `src/core/logging.rs`: logging lokal, tanpa network.
- CI: pastikan job fmt/clippy/build/test/catalog-check ada dan hijau.
- `Cargo.toml`: kurangi dependency ke whitelist directive + justifikasi di DECISIONS.md.
- Hapus `#[allow(dead_code)]` / `#[allow(unused)]` jika ada.

**DoD:**
- Error taxonomy ter-test per variant.
- Config invalid ditolak.
- Build + clippy hijau.
- Domain WP-1 di-catalog: `error-taxonomy`, `config-load`, `config-validation`, `logging-tracing`, `cli-commands` → `quality = "real"`, >= 12 test real per domain. [DONE]
- STUB_INVENTORY ditutup untuk item WP-1. [DONE]

**completed_domains:** `["error-taxonomy", "config-load", "config-validation", "logging-tracing", "cli-commands"]`

---

## Riwayat WP

| WP | Nama | Status | Commit |
|----|------|--------|--------|
| WP-0 | Audit & Inventory | SELESAI | — |
| WP-1 | Core Hygiene | SELESAI | — |
| WP-2 | Storage Read Path | PENDING | — |
| WP-3 | Storage Write Path & Recovery | PENDING | — |
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
