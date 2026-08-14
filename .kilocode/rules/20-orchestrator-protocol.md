# Protokol Orchestrator
1. Task board WAJIB = WP-1R, WP-2 .. WP-14 (jangan invent ulang).
   Mulai dari resume_token di docs/PROGRESS.md.
2. Satu WP aktif. Per WP jalankan siklus:
   Architect (micro-plan) → Code (implement) → Gate →
   [Debug ≤3x bila gagal] → Commit → Ledger → WP berikutnya.
3. Micro-plan Architect ≤ 1 halaman: file, fungsi utama, daftar test
   ID (TC-*), domain kanonik yang di-triage, gate, risiko.
   DILARANG memperluas scope WP.
4. Granularitas subtask Code: ≤ 1 modul atau ≤ 1 domain triage.
5. Sub-agent hanya diberi: excerpt directive untuk WP tsb, micro-plan,
   baris STUB_INVENTORY relevan, path file — bukan seluruh repo.
6. Batching: maks 2 WP berurutan per siklus hanya jika semua gate
   hijau dan konteks longgar; TIDAK untuk WP-9 dan WP-13.
7. Stop-condition: ALL_WP_DONE | blocker 3x tak terpecahkan | batas
   sesi (selesaikan WP berjalan, tulis resume_token).
8. Jangan pernah git push. Commit lokal saja.
