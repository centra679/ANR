# Hard Rules Directive v1.1 (semua mode)
1. Tidak ada todo!/unimplemented!/placeholder/Ok(()) palsu.
2. Tidak ada kode yang belum dibutuhkan WP berjalan.
3. Tidak ada test palsu (assert!(true) dll). Fake diketahui = wajib
   dihapus/dikonversi oleh WP pemilik domain; WP-13 fake global = 0.
4. Tidak ada #[allow(dead_code/unused)] untuk menutupi masalah.
5. Tidak ada dependency di luar whitelist (serde, toml, blake3, clap)
   tanpa DEC-xxx.
6. Serde TIDAK untuk brain.anr (parser binary manual, SD-03).
7. Satu WP per siklus; perubahan luar scope = wiring-only tercatat,
   atau rekonsiliasi (revert/absorb).
8. Setiap test terdaftar di tests/catalog.toml (domain kanonik via
   docs/DOMAIN_MAP.toml); kuota 12 real/domain.
9. Commit atomik "ANR-WP<n>: ..."; ledger PROGRESS.md wajib update
   (completed_domains, metrik kanonik, resume_token).
10. Gate sebelum handoff: fmt, clippy -D warnings, cargo test,
    check_test_catalog.
11. Coverage gate: setiap file dalam docs/coverage_gate.toml WAJIB mencapai
    minimum_coverage sebelum WP dinyatakan selesai. CI coverage job FAIL
    bila file di bawah threshold. Critical file = true menghalangi merge.
