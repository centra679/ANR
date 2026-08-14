[KILO ORCHESTRATOR — MASTER TASK: ANR END-TO-END → PRODUCT READY]

Kamu adalah orchestrator implementasi ANR. Hukum tertinggi:
.kilocode/rules/* dan dokumen otoritas (AC v1.1, SD v1.0, TC v1.0,
Directive v1.1). Baca semuanya sekarang, plus docs/PROGRESS.md,
docs/STUB_INVENTORY.md, docs/DOMAIN_MAP.toml, tests/catalog.toml.

TARGET: selesaikan seluruh Work Package dari resume_token hingga
WP-14, lalu validasi Product Ready. Tidak ada WP yang dilewati,
tidak ada scope tambahan.

DEKOMPOSISI WAJIB (task board):
  WP-1R  Scope reconciliation WP-1 (hunk luar-scope di
         storage/transaction.rs, core/lifecycle.rs, core/scheduler.rs,
         core/state_machine.rs, interface/diagnostics.rs)
  WP-2   Storage Read Path
  WP-3   Storage Write Path & Recovery
  WP-4   Memory Manager
  WP-5   Neural Core SoA + SIMD
  WP-6   Brain Sections + Provisioning Core
  WP-7   Perception + Mock Plugins
  WP-8   Decision + Safety + Actuator Mock
  WP-9   Runtime State Machine + Scheduler + Vertical Slice
  WP-10  Learning Dasar
  WP-11  Replay/Consolidation/Retention/GC
  WP-12  Interface Lengkap (CLI/diagnostics/telemetry)
  WP-13  Hardening (fault/security/perf/CI matrix)
  WP-14  Product Validation

SIKLUS PER WP (tanpa pengecualian):
  1. Architect: micro-plan (≤1 halaman) sesuai protocol rules.
  2. Code: implementasi + triage domain kanonik + ledger.
  3. Gate: fmt; clippy -D warnings; cargo test; check_test_catalog;
     fixtures script bila relevan. Gagal → Debug (≤3x).
  4. Commit atomik "ANR-WP<n>: ...".
  5. Update PROGRESS.md (completed_domains, metrik kanonik,
     resume_token), STUB_INVENTORY.md, IMPLEMENTATION_SUMMARY.md.
  6. Append laporan WP ke docs/SESSION_LOG.md (format standar +
     metrik kanonik).

ATURAN KHUSUS:
- WP-2 wajib dimulai dengan konfirmasi hasil WP-1R ter-commit.
- WP-9 wajib menghasilkan E2E vertical slice hijau
  (boot→sense(sim)→decide→safety→act(mock)→feedback→episode→shutdown).
- WP-13 wajib fake global = 0 dan STUB_INVENTORY 0 OPEN sebelum
  masuk WP-14.
- Endurance/hardware panjang TIDAK dijalankan blocking di sesi:
  wujudkan sebagai script + CI job (nightly), buktikan wiring-nya.

EXIT — PRODUCT READY (WAJIB semua):
  [ ] WP-1R..WP-14 completed di PROGRESS.md
  [ ] unit real ≥ 840; total ≥ 1440; fake = 0; 70 domain kanonik done
  [ ] STUB_INVENTORY 0 OPEN
  [ ] gate penuh hijau + CI matrix (x86_64, aarch64/QEMU) hijau
  [ ] security audit: critical/high = 0
  [ ] benchmark dalam budget (SD-17); tidak ada regresi control path
  [ ] endurance script (4h/8h) + CI nightly wired
  [ ] golden brain + checksum manifest + conformance report
  [ ] docs/PRODUCT_READY.md terisi checklist SD-18 §18.4
  [ ] PROGRESS.md: status: ALL_WP_DONE

Bila exit terpenuhi, akhiri dengan LAPORAN PRODUCT READY:
ringkasan commit WP-0..14, metrik final, daftar artifact release,
sisa risiko (harus kosong atau ber-waiver tercatat).

MULAI SEKARANG: bangun task board dari resume_token, lalu jalankan
siklus WP pertama.
