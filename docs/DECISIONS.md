# DECISIONS.md — ANR

**Tanggal:** 2026-08-14  
**Status:** LIVE  
**Proses:** Setiap keputusan baru ditambahkan sebagai baris baru; tidak menghapus atau mengubah baris lama tanpa catatan perubahan.

---

| ID | Tanggal | Pertanyaan | Keputusan | Traceability | Alternatif ditolak | Status |
|---|---|---|---|---|---|---|
| DEC-001 | 2026-08-14 | Dependency apa yang diizinkan tanpa justifikasi? | Whitelist: `serde`, `toml`, `blake3`, `clap`. Semua dependency lain wajib dicatat di DECISIONS.md dengan justifikasi offline-safety dan alasan tidak bisa pakai std. | AC §4.5, SD-02 | `reqwest`/`tokio` networking, LLM/transformer crates, GPU crates | ACCEPTED |
| DEC-002 | 2026-08-14 | Bagaimana strategi migrasi 1.466 test legacy yang sudah ada? | Legacy migration: generate otomatis `tests/catalog.toml` dari `cargo test -- --list` dengan `legacy = true`, `quality = "unknown"`. Setiap WP men-triage domain miliknya: tandai `quality`, ganti fake test, tambah test hingga kuota 12 real/domain, normalisasi ID ke `TC-U-<DOMAIN>-<SEQ>` dengan `legacy = false`. Enforcement bertahap via `scripts/check_test_catalog.sh`. | Master-Test-CI.md §3, directive §5 | Manual registration 1.466 test (lumpuh); menghapus test lama (rugi coverage) | ACCEPTED |
| DEC-003 | 2026-08-14 | Bagaimana representasi error dan logging? | Error: enum `Error` dengan 13 kelas taxonomy SD-16, kode `ANR-E-*`, `Severity`, impl `Display`/`Error`, konversi `From`. Logging: `tracing` + `tracing-subscriber` lokal tanpa network; `env-filter` opsional. | AC §32, SD-16 §16.4 | `anyhow` untuk production error (trojan-horse), `log` crate (kurang metadata), network logging | ACCEPTED |
| DEC-004 | 2026-08-14 | Format config dan kebijakan validasi? | Config format: TOML (`toml` crate, di-whitelist). `RuntimeConfig` default safe: `allow_volatile_degraded_mode = false`, `emergency_stop_timeout_ms = 100`, `shutdown_timeout_ms = 5000`. Validasi: reject jika field di luar range atau missing required field. | AC §37, SD-02 §2.3 | JSON config (verbose), binary config (tidak human-readable), env-var only (tidak ter-versioning) | ACCEPTED |
| DEC-005 | 2026-08-14 | Bagaimana CLI unavailable command behavior? | Jika user memanggil subcommand yang belum diimplementasi di WP saat ini, CLI mengembalikan error terstruktur (bukan panic/todo!) yang menyebutkan daftar command yang tersedia dan menyatakan fitur dalam pengembangan. | AC §56, directive §2.1 | Silent ignore, panic, stub yang mengembalikan Ok(()), atau crash | ACCEPTED |
| DEC-006 | 2026-08-14 | Bagaimana Error enum di-organisasi? | 13 kelas taxonomy SD-16 dengan kode `ANR-E-*`, enum `Severity` (Low/Medium/High/Critical), impl `Display`/`Error`, konversi `From`. `StorageIo` menggunakan `#[from] std::io::Error)`. Manual `PartialEq` karena `std::io::Error` tidak mengimplementasikan `PartialEq`. | AC §32, SD-16 §16.4 | Flat error list tanpa taxonomy, `anyhow` untuk production, `Box<dyn Error>` | ACCEPTED |
| DEC-007 | 2026-08-14 | Bagaimana logging di-inisialisasi di test? | Gunakan `std::sync::Once` + `std::panic::catch_unwind` untuk mencegah panic saat re-init global subscriber. `set_log_level` menggunakan `tracing::subscriber::set_global_default` dengan catch_unwind. | AC §36, directive §2.2 | `#[allow(unused)]`, global mutable state tanpa guard | ACCEPTED |
