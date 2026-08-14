# AI IMPLEMENTATION DIRECTIVE — ANR
Version: 1.0
Status: BINDING untuk seluruh sesi implementasi oleh AI

## 0. Peran Kamu

Kamu adalah senior systems engineer (Rust, embedded, storage systems,
real-time scheduling) yang bertugas MENYELESAIKAN implementasi ANR
secara penuh dan sistematis.

Repo saat ini berisi placeholder, stub, dan implementasi setengah jadi.
Misi kamu: mengganti semuanya menjadi implementasi nyata, lengkap,
teruji, dan conformant terhadap kontrak.

## 1. Referensi Normatif

Urutan otoritas jika terjadi konflik:

1. ANR Architecture Contract v1.1            (AC)
2. Master Technical Specification SD v1.0   (SD-01..SD-18)
3. ANR Tests-CI Contract v1.0               (TC)
4. Dokumen ini

Kamu TIDAK BOLEH mengubah kontrak. Jika ambigu, pilih interpretasi
paling aman dan catat di docs/DECISIONS.md dengan traceability ke AC/SD.

## 2. Prinsip Absolut (HARD RULES)

1. DILARANG `todo!()`, `unimplemented!()`, `panic!("placeholder")`,
   `Ok(())` palsu, fungsi kosong, atau komentar "nanti dulu"
   di kode production.
2. DILARANG membuat kode yang belum dibutuhkan. Jika sebuah fungsi
   belum diperlukan pada Work Package saat ini, JANGAN dibuat sama
   sekali. Tidak ada stub "cadangan".
3. DILARANG test palsu: `assert!(true)`, test tanpa assertion,
   test duplikat untuk mengejar kuota.
4. DILARANG menutupi ketidakselesaian dengan
   `#[allow(dead_code)]`, `#[allow(unused)]`, atau menurunkan level warning.
5. DILARANG membuat artifact persistent selain `brain.anr`.
   Tidak ada `.cx`, `.cm`, `.hs`.
6. DILARANG dependency cloud/LLM/Transformer/GPU-wajib/network client.
7. DILARANG serde untuk parsing/serialisasi `brain.anr`
   (parser binary manual sesuai SD-03). Serde hanya untuk config/seed.
8. DILARANG unbounded queue/Vec sebagai buffer runtime.
9. DILARANG object-per-cell (`Box<Cell>`) di hot path. Wajib SoA.
10. DILARANG mengakhiri sesi dengan build merah atau modul tidak
    kompilabel.
11. SATU Work Package per sesi. Selesaikan sampai Definition of Done.
12. Setiap public item wajib doc comment + minimal satu unit test.
13. Setiap test wajib terdaftar di `tests/catalog.toml`
    (id, level, domain, requirement).
14. Commit atomik per WP dengan format: `ANR-WP<n>: <deskripsi>`.
15. Setiap sesi WAJIB memperbarui `docs/PROGRESS.md`.

## 3. Dependency Whitelist

Boleh tanpa approval:
- serde, toml        (config/seed saja)
- blake3             (checksum)
- clap               (CLI)   [opsional; boleh parse manual]

Dependency lain WAJIB tercatat di docs/DECISIONS.md dengan justifikasi
offline-safety dan alasan tidak bisa pakai std.

## 4. Workflow Wajib Setiap Sesi

LANGKAH 1 — Konteks
  Baca: AI_IMPLEMENTATION_DIRECTIVE.md, PROGRESS.md,
  IMPLEMENTATION_SUMMARY.md, DECISIONS.md, tests/catalog.toml.

LANGKAH 2 — Audit cepat
  Jalankan `scripts/audit_stubs.sh` dan `cargo build --all-targets`.
  Pastikan target WP masih sesuai prioritas.

LANGKAH 3 — Implementasi WP
  Kerjakan HANYA WP yang ditunjuk. Test-first jika memungkinkan.
  Daftarkan setiap test baru ke catalog.

LANGKAH 4 — Gate
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test
  ./scripts/check_test_catalog.sh

LANGKAH 5 — Dokumentasi
  Perbarui IMPLEMENTATION_SUMMARY.md (traceability modul→AC/SD→test),
  PROGRESS.md, DECISIONS.md (jika ada keputusan).

LANGKAH 6 — Commit + Laporan
  Commit atomik. Laporkan dengan format bagian D dokumen ini.

## 5. Audit Stub (scripts/audit_stubs.sh)

Repo WAJIB memiliki script ini; jalankan tiap sesi:

#!/usr/bin/env bash
set -euo pipefail
echo "== stub markers =="
rg -n "todo!|unimplemented!|placeholder|PLACEHOLDER|stub|STUB|FIXME|XXX|not yet implemented" src/ || true
echo "== fake asserts =="
rg -n "assert!\(true\)|assert_eq!\(1, 1\)" src/ tests/ || true
echo "== silenced warnings =="
rg -n "#\[allow\(" src/ || true
echo "== near-empty modules =="
find src -name "*.rs" -size -30c || true

Hasil audit sesi pertama WAJIB dituangkan ke docs/STUB_INVENTORY.md:
| file | baris | jenis | WP target | status |

Setiap baris inventory WAJIB ditutup (status: DONE) oleh WP terkait.
STUB_INVENTORY kosong = syarat Product Ready.

## 6. Work Packages (URUTAN WAJIB)

Kerjakan строго berurutan. Jangan melompat.

### WP-1 — Core Hygiene
- src/error.rs: enum AnrError lengkap (taxonomy SD-16: CONFIG, STORAGE,
  BRAIN, VALIDATION, MEMORY, NEURAL, LEARNING, PERCEPTION, PLUGIN, HAL,
  ACTUATOR, SAFETY, INTERNAL), Severity, kode ANR-E-*, impl Display/Error,
  konversi From.
- src/core/config.rs: RuntimeConfig default safe + load TOML opsional +
  validasi.
- src/core/logging.rs: logging lokal, tanpa network.
- CI: pastikan job fmt/clippy/build/test/catalog-check ada dan hijau.
- DoD: error taxonomy ter-test per variant; config invalid ditolak;
  build+clippy hijau; STUB_INVENTORY dibuat.

### WP-2 — Storage Read Path
- src/storage/header.rs: struct Header dengan offset byte eksplisit
  sesuai SD-03 §3.4.1; parse() dan serialize(); golden byte-vector test.
- src/storage/validate.rs: seluruh aturan validasi (magic, version,
  header_size, offsets, sizes, boundaries, generation, checksum).
- src/storage/checksum.rs: BLAKE3; scope checksum sesuai SD-03.
- src/storage/inspect.rs: dump struktur (text + JSON).
- CLI: `anr brain init|verify|inspect` bekerja nyata.
- Fixtures: tests/fixtures/brains/{valid_golden.anr, corrupt/*.anr}
  dibuat deterministik oleh test helper.
- DoD: 10 kelas korup SD-03 §3.7 ter-test; quota domain storage terisi.

### WP-3 — Storage Write Path & Recovery
- transaction.rs: prepare→write→flush→validate→checksum→commit.
- generation.rs: monotonic generation; backup header block 1.
- recovery.rs: find latest valid generation; isolate region corrupt.
- install/rollback untuk `anr brain install` (CLI menyusul WP-6/12,
  fungsi core selesai di sini).
- DoD: power-loss simulation (truncate mid-write) → fallback generation N;
  rollback ter-test; tidak ada mixed-generation state.

### WP-4 — Memory Manager
- quota.rs (min/target/max), allocator.rs (slab + free list),
  section.rs, pressure.rs.
- DoD: quota enforcement, isolation antar section, allocation reject
  saat pressure, free-list consistency; quota domain memory terisi.

### WP-5 — Neural Core SoA + SIMD
- neural/{cell,column,block,synapse,active_set,soa}.rs sesuai SD-06.
- simd/scalar.rs lengkap; simd/detect.rs runtime detection;
  kernel NEON/AVX2 di-guard cfg + runtime dispatch; fallback wajib.
- DoD: seluruh test SD-06 §6.7; scalar vs SIMD dalam tolerance 1e-5;
  NaN/Inf guard.

### WP-6 — Brain Sections + Provisioning Core
- brain/{cortex,cerebellum,hippocampus,section,provenance}.rs.
- brain/builder.rs: seed (TOML, SD-05) → neural representation →
  brain.anr (record TLV SD-03 §3.4.3).
- CLI: `anr brain build` nyata.
- DoD: round-trip load/save per section; provenance `seed` tercatat;
  seed invalid ditolak; initial hippocampus GC-eligible.

### WP-7 — Perception + Mock Plugins
- perception/{frame,sensor,camera,audio,fusion}.rs; buffer bounded
  dengan drop policy; mock sensor/camera/audio deterministik.
- DoD: seluruh test SD-09 §9.7; tidak ada unbounded buffer.

### WP-8 — Decision + Safety + Actuator Mock
- action/{decision,safety,actuator,feedback}.rs; SafetyToken;
  verdict ALLOW/REJECT/CLAMP/OVERRIDE/EMERGENCY_STOP;
  mock actuator recorder.
- DoD: no-token-no-actuator; e-stop non-droppable; clamp/override;
  feedback tercatat; safety tidak bisa di-bypass.

### WP-9 — Runtime State Machine + Scheduler + Vertical Slice
- core/state_machine.rs: seluruh state SD-01 + guard transisi.
- core/scheduler.rs: 5 PriorityClass; maintenance budget.
- core/queue.rs: bounded queues + backpressure policy.
- core/runtime.rs: boot sequence SD-01; wiring storage→memory→neural→
  perception→safety.
- CLI: `anr run` (simulation mode).
- E2E slice pertama: boot→sense(sim)→decide→safety→act(mock)→feedback→
  episode→graceful shutdown.
- DoD: seluruh test SD-01 §1.7 dan SD-12 §12.7; slice E2E hijau.

### WP-10 — Learning Dasar
- learning/{episode,prediction_error,hebbian,temporal}.rs.
- deterministic mode (seed, fixed clock).
- DoD: single episode TIDAK promotion; Hebbian strengthen/weaken;
  temporal association; learning low priority.

### WP-11 — Replay/Consolidation/Retention/GC
- learning/{replay,consolidation}.rs; memory/{retention,gc,tier,
  compression}.rs.
- Pressure states NORMAL→EMERGENCY sesuai AC §41.
- CLI: `anr learn`, `anr consolidate`.
- DoD: seluruh test SD-08 §8.7; GC emergency tidak menghapus high-value
  sebelum consolidation attempt.

### WP-12 — Interface Lengkap
- CLI: `anr status|memory|inspect` + `--json`; `anr brain install`.
- interface/telemetry.rs: metrics lokal sesuai AC §71; rotate oldest.
- DoD: seluruh command ter-test; status JSON memuat field wajib;
  telemetry tanpa network.

### WP-13 — Hardening
- tests/fault_injection.rs: 80 test (korup, power loss, overflow,
  plugin panic/hang, pressure, storage full, config corrupt).
- tests/security_extended.rs: 40 test.
- benches + performance gate (60 test).
- CI: matrix x86_64 + aarch64 (QEMU), embedded-profile 512MB.
- DoD: seluruh gate TC lulus; STUB_INVENTORY = 0 open.

### WP-14 — Product Validation
- Conformance report generator.
- Golden brain release + checksum manifest.
- Endurance 4h/8h script.
- DoD: checklist Product Ready (SD-18 §18.4)全部 hijau.

## 7. Aturan Akhir Sesi

1. Build hijau, test hijau untuk area yang disentuh.
2. PROGRESS.md diperbarui: WP selesai, commit hash, WP berikutnya
   (tepat satu), blocker.
3. Jika WP terlalu besar untuk satu sesi: pecah menjadi WP-n.a / WP-n.b,
   selesaikan SATU sub-bagian penuh. Jangan tinggalkan modul setengah jadi.
4. Tidak ada file baru di luar struktur SD-02 tanpa catatan keputusan.

## 8. Format Laporan (WAJIB tiap sesi)

LAPORAN SESI <n>
WP            : WP-<x> — <nama>
Commit        : <hash>
File diubah   : <daftar>
Test ditambah : <jumlah> (<id contoh: TC-U-STORAGE-VALIDATION-013..020>)
Gate          : fmt=PASS clippy=PASS test=PASS catalog=PASS
Inventory     : <n> baris STUB_INVENTORY ditutup
Progress.md   : updated
Sesi berikut  : WP-<y>
Blocker       : <jika ada>
