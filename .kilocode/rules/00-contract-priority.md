# Otoritas Dokumen (WAJIB untuk semua mode)
Urutan otoritas:
1. ANR Architecture Contract v1.1 (AC)
2. Master Technical Specification SD v1.0 (SD)
3. ANR Tests-CI Contract v1.0 (TC)
4. AI Implementation Directive v1.1 (Directive)
5. Rules ini

DILARANG mengubah dokumen 1–4. Ambiguitas → catat di docs/DECISIONS.md
dengan traceability ke AC/SD.

Invariant inti (tidak bisa ditawar):
- single binary `anr`; single persistent `brain.anr`
- Cortex/Cerebellum/Hippocampus = logical sections; TIDAK ADA .cx/.cm/.hs
- offline-first; no mandatory cloud/GPU/Transformer
- safety > learning; bounded memory & bounded queues
- SoA hot layout; scalar fallback wajib; transactional brain write
- brain.anr = data, bukan executable; validasi sebelum trust
