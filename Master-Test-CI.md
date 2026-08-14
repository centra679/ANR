# ANR — Tests-CI Contract

## End-to-End Strict Testing & CI Contract v1.0

**Status:** `BINDING TESTS-CI CONTRACT`  
**Turunan dari:** `ANR Architecture Contract Final Architectural Baseline v1.1`  
**Tujuan:** Mengunci validasi implementasi ANR secara ketat agar pengembangan tidak melenceng dari arsitektur, invariant, safety boundary, resource boundedness, offline-first, single-binary, single-brain, dan perilaku belajar yang diharapkan.  
**Sifat dokumen:** Kontrak pengujian dan CI. Bukan roadmap. Bukan fase implementasi. Bukan jadwal rilis.

---

## 0. Ketentuan Umum

Dokumen ini menetapkan persyaratan minimum untuk:

1. Unit tests.
2. Integration tests.
3. End-to-end tests.
4. Fault injection tests.
5. Performance/benchmark tests.
6. Conformance tests.
7. Security tests.
8. CI pipeline.
9. Coverage dan mutation gate.
10. Traceability ke Architecture Contract.
11. Flaky test policy.
12. Test reporting.
13. Release gate.

Setiap implementasi ANR WAJIB memenuhi kontrak ini sebelum dianggap layak untuk merge, release, atau deployment.

---

## 1. Prinsip Dasar Tests-CI

### 1.1 Strictness

Semua pengujian WAJIB:

1. Deterministik.
2. Dapat direproduksi.
3. Otomatis di CI.
4. Memiliki traceability ke Architecture Contract.
5. Memiliki oracle/assertion eksplisit.
6. Tidak bergantung pada cloud.
7. Tidak bergantung pada hardware fisik untuk merge gate utama.
8. Tidak boleh diloloskan dengan retry otomatis untuk menutupi kegagalan.
9. Tidak boleh menggunakan test kosong atau test tanpa assertion bermakna.
10. Tidak boleh double-count untuk satu perilaku yang sama.

### 1.2 Offline Requirement

Seluruh test execution WAJIB dapat berjalan offline.

Test runtime TIDAK BOLEH:

1. Memanggil cloud API.
2. Mengandalkan LLM server.
3. Mengandalkan database eksternal.
4. Mengandalkan network service eksternal.
5. Mengirim telemetry keluar sistem.

Dependency fetching oleh CI BOLEH menggunakan cache/artifact registry internal, tetapi runtime test itu sendiri WAJIB offline.

### 1.3 Architecture Alignment

Setiap test WAJIB memvalidasi salah satu atau lebih dari:

1. Architectural invariant.
2. Safety behavior.
3. Memory boundedness.
4. Storage integrity.
5. Learning behavior.
6. Perception behavior.
7. Decision behavior.
8. Plugin/HAL isolation.
9. CLI/diagnostics behavior.
10. Performance/resource constraint.

Jika sebuah test tidak dapat dipetakan ke requirement arsitektur, test tersebut BOLEH ada, tetapi TIDAK BOLEH dihitung sebagai test wajib untuk kontrak ini.

---

## 2. Test Levels

Kontrak ini menggunakan level berikut:

| Level | Nama | Fungsi |
|---|---|---|
| L0 | Unit Test | Validasi unit/logika terkecil. |
| L1 | Integration Test | Validasi interaksi antar modul. |
| L2 | End-to-End Test | Validasi sistem penuh menggunakan binary dan brain.anr. |
| L3 | Fault Injection Test | Validasi perilaku terhadap kegagalan. |
| L4 | Performance Test | Validasi latency, throughput, regression. |
| L5 | Conformance Test | Validasi kepatuhan terhadap Architecture Contract. |
| L6 | Security Test | Validasi terhadap input jahat/corrupt/unsafe. |
| L7 | Extended/Nightly Test | Fuzzing, soak, long-run, cross-platform lanjutan. |

---

## 3. Minimum Test Count Contract

### 3.1 Absolute Minimum

Minimum absolut unit test adalah:

```text
800 unit tests
```

Namun kontrak ini mengunci kuota yang lebih ketat:

```text
840 unit tests minimum
```

Tujuannya adalah memberi margin agar tidak turun ke bawah 800 karena refactor, penghapusan test, atau test yang dinyatakan obsolete.

### 3.2 Minimum Total Test Suite

Minimum test suite yang WAJIB dijaga:

| Level | Minimum |
|---|---:|
| Unit Tests | 840 |
| Integration Tests | 200 |
| End-to-End Tests | 120 |
| Fault Injection Tests | 80 |
| Performance/Benchmark Tests | 60 |
| Conformance Tests | 100 |
| Security Tests | 40 |
| **Total minimum** | **1440** |

CI WAJIB gagal jika salah satu minimum count tidak terpenuhi.

---

# BAGIAN I — UNIT TEST CONTRACT

---

## 4. Unit Test Requirements

### 4.1 Sifat Unit Test

Setiap unit test WAJIB:

1. Cepat.
2. Deterministik.
3. Tidak bergantung pada hardware fisik.
4. Tidak bergantung pada network.
5. Tidak bergantung pada cloud.
6. Menggunakan temporary directory jika menyentuh storage.
7. Membersihkan state setelah selesai.
8. Memiliki assertion eksplisit.
9. Memiliki test ID unik.
10. Memiliki mapping ke requirement.

### 4.2 Waktu Eksekusi

Untuk CI standar:

| Metric | Target |
|---|---:|
| Individual unit test p95 | ≤ 100 ms |
| Individual storage-related unit test p95 | ≤ 250 ms |
| Total unit suite | ≤ 5 menit pada runner standar |

Jika total unit suite melewati batas secara konsisten, test harus dioptimalkan atau dipindahkan ke integration/nightly dengan justifikasi.

### 4.3 Unit Test Composition

Setiap domain unit test WAJIB memiliki minimal 12 test dengan komposisi minimal:

```text
3 positive functional tests
3 negative/invalid tests
3 boundary/edge tests
2 invariant/property tests
1 regression/fuzz-derived test
```

Total:

```text
12 tests/domain
```

### 4.4 Larangan Unit Test

Unit test TIDAK BOLEH:

1. Tidak memiliki assertion.
2. Hanya tidur/sleep tanpa validasi.
3. Bergantung pada urutan eksekusi test lain.
4. Mengubah global state tanpa cleanup.
5. Mengandalkan waktu wall-clock nyata untuk determinisme.
6. Mengakses path sistem di luar temp area.
7. Menyimpan artifact permanen.
8. Menghitung double-count untuk test yang sama.

---

## 5. Unit Test Naming and Catalog

### 5.1 Test ID

Format ID unit test:

```text
TC-U-<DOMAIN>-<SEQ>
```

Contoh:

```text
TC-U-CELL-STATE-001
TC-U-GC-EMERGENCY-007
TC-U-SAFETY-CONSTRAINTS-012
```

### 5.2 Catalog

Semua test WAJIB terdaftar dalam catalog machine-readable, misalnya:

```text
tests/catalog.toml
```

atau format setara yang disepakati.

Setiap entri WAJIB memuat minimal:

```text
id
level
domain
requirement
type
status
owner
criticality
```

Contoh:

```toml
[[test]]
id = "TC-U-CELL-STATE-001"
level = "unit"
domain = "cell-state"
requirement = "ArchitectureContract#12.1"
type = "positive"
status = "required"
criticality = "high"
```

CI WAJIB menolak test yang tidak terdaftar dalam catalog.

---

## 6. Unit Test Domain Quota

Kontrak ini menetapkan 70 domain unit test.

Setiap domain WAJIB memiliki minimal:

```text
12 unit tests
```

Total:

```text
70 domain × 12 tests/domain = 840 unit tests
```

### 6.1 Group A — Core Runtime & Lifecycle

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 1 | `core-boot` | Boot lifecycle, validasi awal. |
| 2 | `core-run-loop` | Loop state machine. |
| 3 | `core-shutdown` | Graceful shutdown. |
| 4 | `core-emergency-shutdown` | Emergency shutdown dan safe state. |
| 5 | `core-degraded` | Degraded mode state. |
| 6 | `config-load` | Load config/default. |
| 7 | `config-validation` | Validasi config invalid. |
| 8 | `error-taxonomy` | Klasifikasi error recoverable/fatal. |
| 9 | `logging-tracing` | Logging/tracing lokal. |
| 10 | `scheduler-priority` | Priority class scheduler. |

### 6.2 Group B — Resource Control & Neural Cell/Column

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 11 | `maintenance-budget` | Budget maintenance/background. |
| 12 | `bounded-queue` | Kapasitas bounded queue. |
| 13 | `backpressure` | Drop/merge/backpressure policy. |
| 14 | `cell-state` | State dasar Cell. |
| 15 | `cell-activation` | Activation/potential. |
| 16 | `cell-refractory` | Refractory state. |
| 17 | `column-competition` | Competition dalam Column. |
| 18 | `column-sparse` | Sparse activation Column. |
| 19 | `column-association` | Association antar Column. |
| 20 | `block-context` | Context binding Block. |

### 6.3 Group C — Block, Synapse, Sparse Graph

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 21 | `block-sequence` | Sequence/temporal Block. |
| 22 | `block-prediction` | Prediction state Block. |
| 23 | `synapse-create` | Pembuatan Synapse. |
| 24 | `synapse-validate` | Validasi source/target/weight. |
| 25 | `synapse-update` | Update weight/state. |
| 26 | `synapse-decay` | Decay/weakening. |
| 27 | `synapse-prune` | Pruning. |
| 28 | `sparse-traversal` | Traversal active graph. |
| 29 | `soa-layout` | Konsistensi SoA arrays. |
| 30 | `scalar-kernels` | Kernel scalar dasar. |

### 6.4 Group D — SIMD, Memory, GC

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 31 | `simd-neon` | NEON kernel. |
| 32 | `simd-avx` | AVX2/AVX-512 kernel. |
| 33 | `simd-fallback` | Scalar fallback. |
| 34 | `memory-quota` | min/target/max quota. |
| 35 | `allocator` | Allocation/free list. |
| 36 | `memory-isolation` | Isolation antar section. |
| 37 | `retention-scoring` | Retention score. |
| 38 | `gc-normal` | GC normal/monitor. |
| 39 | `gc-aggressive` | GC aggressive. |
| 40 | `gc-emergency` | GC emergency. |

### 6.5 Group E — Storage, Brain, Recovery

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 41 | `tiering` | HOT/WARM/COLD transition. |
| 42 | `compression` | Compression/decompression. |
| 43 | `brain-header` | Parsing header brain.anr. |
| 44 | `brain-offset-size` | Validasi offset/size. |
| 45 | `checksum` | Checksum/integrity. |
| 46 | `transaction` | Generation transaction. |
| 47 | `recovery` | Recovery generation valid. |
| 48 | `brain-seed` | Validasi/transform seed. |
| 49 | `brain-build` | Build brain dari seed. |
| 50 | `brain-verify-inspect` | Verify/inspect brain. |

### 6.6 Group F — Provisioning, Perception, Plugin/HAL, Decision/Safety

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 51 | `brain-install-update` | Install/update/rollback brain. |
| 52 | `sensor-frame` | Validasi SensorFrame. |
| 53 | `camera-buffer` | Camera bounded buffer. |
| 54 | `audio-buffer` | Audio bounded buffer. |
| 55 | `perception-fusion` | Fusion/preprocessing. |
| 56 | `plugin-lifecycle` | Start/stop/restart plugin. |
| 57 | `plugin-isolation` | Isolasi kegagalan plugin. |
| 58 | `hal-mock` | HAL mock behavior. |
| 59 | `decision-candidate` | Candidate action/evaluation. |
| 60 | `safety-constraints` | Allow/reject/clamp/override. |

### 6.7 Group G — Learning, Feedback, CLI, Diagnostics, Security

Minimal 10 domain × 12 = 120 unit tests.

| No | Domain Code | Deskripsi |
|---:|---|---|
| 61 | `feedback-prediction` | Feedback/prediction error. |
| 62 | `hebbian-learning` | Hebbian reinforcement. |
| 63 | `temporal-learning` | Temporal association. |
| 64 | `replay-selection` | Replay selection. |
| 65 | `consolidation-promotion` | Consolidation/promotion. |
| 66 | `contradiction-handling` | Contradiction/context analysis. |
| 67 | `skill-failure` | Skill failure adjustment. |
| 68 | `cli-commands` | CLI command behavior. |
| 69 | `diagnostics-telemetry` | Status/telemetry lokal. |
| 70 | `security-validation` | Validasi input/brain untrusted. |

### 6.8 Total Unit Test Quota

```text
7 group × 10 domain × 12 tests = 840 unit tests
```

CI WAJIB:

1. Menghitung jumlah test per domain.
2. Menggagalkan pipeline jika ada domain < 12 test.
3. Menggagalkan pipeline jika total unit test < 840.
4. Menolak double-counting antar domain.

---

# BAGIAN II — INTEGRATION TEST CONTRACT

---

## 7. Integration Test Requirements

Integration test WAJIB memvalidasi interaksi antar modul nyata atau mock yang merepresentasikan interface nyata.

Integration test TIDAK BOLEH hanya menguji unit secara terisolasi.

### 7.1 Minimum Integration Tests

Minimum:

```text
200 integration tests
```

### 7.2 Integration Domain Quota

Ditetapkan 25 integration domain.

Setiap domain minimal:

```text
8 tests
```

Total:

```text
25 × 8 = 200 integration tests
```

| No | Domain Code | Integrasi yang Diuji |
|---:|---|---|
| 1 | `sensor-to-perception` | Sensor plugin → perception. |
| 2 | `camera-to-perception` | Camera plugin → perception. |
| 3 | `audio-to-perception` | Audio plugin → perception. |
| 4 | `perception-to-neural` | Perception → Neural Core representation. |
| 5 | `neural-active-graph` | Cell/Column/Block/Synapse active flow. |
| 6 | `cortex-interface` | Cortex read/write/association. |
| 7 | `cerebellum-interface` | Cerebellum skill invocation/update. |
| 8 | `hippocampus-episode` | Episode append/query/replay. |
| 9 | `replay-to-learning` | Replay → learning update. |
| 10 | `learning-to-synapse` | Learning → synapse weight/state. |
| 11 | `consolidation-to-memory` | Consolidation → Cortex/Cerebellum. |
| 12 | `retention-to-gc` | Retention → GC decision. |
| 13 | `allocation-to-tiering` | Allocation → HOT/WARM/COLD. |
| 14 | `storage-read-validation` | Read brain → validation. |
| 15 | `storage-write-transaction` | Write → flush → checksum → commit. |
| 16 | `recovery-to-boot` | Recovery → boot runtime. |
| 17 | `brain-build-install` | Seed → build → install. |
| 18 | `cli-to-runtime` | CLI → runtime status/control. |
| 19 | `diagnostics-to-telemetry` | Diagnostics → telemetry output. |
| 20 | `decision-to-safety` | Decision → safety validation. |
| 21 | `safety-to-actuator` | Safety → actuator command. |
| 22 | `actuator-to-feedback` | Actuator → feedback → Hippocampus. |
| 23 | `plugin-to-hal` | Plugin lifecycle → HAL. |
| 24 | `plugin-failure-degradation` | Plugin failure → degraded runtime. |
| 25 | `simd-to-neural-update` | SIMD kernel → neural update. |

---

# BAGIAN III — END-TO-END TEST CONTRACT

---

## 8. End-to-End Test Requirements

End-to-end test WAJIB menjalankan sistem sebagai satu kesatuan:

```text
anr + brain.anr
```

E2E test WAJIB menggunakan binary nyata, bukan unit-level mock penuh.

### 8.1 E2E Environment

E2E test WAJIB:

1. Menggunakan binary `anr` hasil build.
2. Menggunakan `brain.anr` nyata di temporary directory.
3. Menggunakan simulated sensor/actuator/HAL.
4. Tidak memerlukan hardware fisik.
5. Tidak memerlukan network.
6. Memverifikasi output melalui:
   - exit code,
   - log,
   - `anr status`,
   - `anr inspect`,
   - brain metadata,
   - actuator log,
   - telemetry lokal.

### 8.2 Minimum E2E Tests

Minimum:

```text
120 E2E tests
```

### 8.3 E2E Family Quota

| No | E2E Family | Minimum |
|---:|---|---:|
| 1 | Boot & Provisioning | 18 |
| 2 | Autonomous Loop | 18 |
| 3 | Learning & Consolidation | 18 |
| 4 | Storage, Recovery & Update | 18 |
| 5 | Memory Pressure & GC | 12 |
| 6 | Safety & Actuator | 14 |
| 7 | Plugin/HAL Failure & Degraded Mode | 10 |
| 8 | CLI, Diagnostics & Telemetry | 12 |
| **Total** |  | **120** |

---

## 9. Mandatory E2E Scenarios

Berikut adalah scenario minimum yang WAJIB ada. Setiap scenario dapat diturunkan menjadi beberapa test case.

### 9.1 Boot & Provisioning

WAJIB ada E2E untuk:

1. Boot dengan brain valid.
2. Boot dengan brain missing.
3. Boot dengan brain corrupt header.
4. Boot dengan brain incompatible version.
5. `anr brain init`.
6. `anr brain build` dari seed valid.
7. `anr brain build` dari seed invalid.
8. `anr brain verify` pada brain valid.
9. `anr brain verify` pada brain corrupt.
10. `anr brain inspect` output minimal.
11. `anr brain install` sukses.
12. `anr brain install` gagal dan rollback.
13. Provisioning initial Cortex.
14. Provisioning initial Cerebellum.
15. Provisioning optional Hippocampus.
16. Factory clone master brain.
17. Device-specific brain setelah learning.
18. Deployment artifact hanya `anr` dan `brain.anr`.

### 9.2 Autonomous Loop

WAJIB ada E2E untuk:

1. Sense → perceive → decide → act penuh.
2. Loop dengan sensor deterministik.
3. Loop dengan camera simulated.
4. Loop dengan audio simulated.
5. Loop dengan sparse activation.
6. Loop dengan multiple sensors.
7. Loop dengan sensor dropout.
8. Loop dengan high input rate.
9. Loop dengan bounded queue drop policy.
10. Loop dengan prediction.
11. Loop dengan prediction error.
12. Loop dengan feedback to Hippocampus.
13. Loop dengan decision confidence.
14. Loop dengan goal.
15. Loop dengan safe idle state.
16. Loop deterministic mode.
17. Loop replay tidak mengganggu control.
18. Loop maintenance budget tidak melanggar latency.

### 9.3 Learning & Consolidation

WAJIB ada E2E untuk:

1. Episode baru masuk Hippocampus.
2. Satu episode tidak otomatis promotion.
3. Repeated pattern → Cortex candidate.
4. Repeated skill success → Cerebellum candidate.
5. Prediction error tinggi → replay priority.
6. Novelty tinggi → replay priority.
7. Reward tinggi → replay priority.
8. Failure episode → replay priority.
9. Replay → synapse strengthening.
10. Replay → skill adjustment.
11. Consolidation KEEP.
12. Consolidation COMPRESS.
13. Consolidation DELETE.
14. Consolidation → Cortex.
15. Consolidation → Cerebellum.
16. Contradiction → contextualize.
17. Contradiction repeated → revise.
18. Skill failure tidak langsung menghapus skill.

### 9.4 Storage, Recovery & Update

WAJIB ada E2E untuk:

1. Write brain transactional sukses.
2. Power loss simulation saat write.
3. Fallback ke generation sebelumnya.
4. Corrupt header recovery.
5. Corrupt section recovery.
6. Corrupt index recovery.
7. Checksum mismatch rejection.
8. Offset out-of-bound rejection.
9. Truncated brain rejection/recovery.
10. Brain update sukses.
11. Brain update incompatible rejection.
12. Brain update corrupt rollback.
13. Atomic install.
14. Generation increment.
15. Brain inspect setelah recovery.
16. Brain verify setelah update.
17. Degraded operation jika region tertentu corrupt.
18. No brain trust before validation.

### 9.5 Memory Pressure & GC

WAJIB ada E2E untuk:

1. Hippocampus pressure NORMAL.
2. Hippocampus pressure MONITOR.
3. Hippocampus pressure CONSOLIDATE.
4. Hippocampus pressure AGGRESSIVE GC.
5. Hippocampus pressure EMERGENCY GC.
6. GC tidak menghapus episode bernilai tinggi.
7. GC mengompres episode medium.
8. GC menghapus episode low-value.
9. GC tidak mengganggu control loop.
10. Memory isolation Cortex terjaga.
11. Memory isolation Cerebellum terjaga.
12. Hippocampus tidak mengambil reserved memory subsystem lain.

### 9.6 Safety & Actuator

WAJIB ada E2E untuk:

1. Decision aman diizinkan.
2. Decision berbahaya ditolak.
3. Actuator command clamp.
4. Actuator command override.
5. Emergency stop non-droppable.
6. Emergency stop menghasilkan safe state.
7. Safety tetap aktif saat plugin failure.
8. Safety tidak bisa dibypass learning.
9. Safety tidak bisa dinonaktifkan oleh config production.
10. Safety priority lebih tinggi dari learning.
11. Actuator failure menghasilkan safe state.
12. Feedback failure tercatat.
13. Prediction error setelah action failure.
14. Safety log/telemetry tercatat.

### 9.7 Plugin/HAL Failure & Degraded Mode

WAJIB ada E2E untuk:

1. Sensor plugin failure.
2. Camera plugin failure.
3. Audio plugin failure.
4. Robotics plugin failure.
5. Plugin hang/timeout.
6. Plugin restart.
7. Plugin disable.
8. Runtime continue setelah plugin failure.
9. Degraded vision mode.
10. Degraded sensor pathway mode.

### 9.8 CLI, Diagnostics & Telemetry

WAJIB ada E2E untuk:

1. `anr run`.
2. `anr status`.
3. `anr memory`.
4. `anr inspect`.
5. `anr learn`.
6. `anr consolidate`.
7. `anr status --json` valid.
8. Telemetry lokal tersedia.
9. Dropped frames tercatat.
10. GC metrics tercatat.
11. Promotion metrics tercatat.
12. Brain generation tercatat.

---

# BAGIAN IV — FAULT INJECTION CONTRACT

---

## 10. Fault Injection Requirements

Fault injection WAJIB menguji perilaku sistem dalam kondisi abnormal.

Minimum:

```text
80 fault injection tests
```

### 10.1 Fault Family Quota

| No | Fault Family | Minimum |
|---:|---|---:|
| 1 | Storage corruption | 20 |
| 2 | Process kill / power loss simulation | 10 |
| 3 | Queue overflow | 10 |
| 4 | Sensor/camera/audio failure | 10 |
| 5 | Plugin panic/hang | 10 |
| 6 | Memory pressure / near-OOM | 10 |
| 7 | Clock/timestamp anomaly | 5 |
| 8 | Config corruption | 5 |
| **Total** |  | **80** |

### 10.2 Mandatory Fault Behaviors

Pada fault injection, sistem WAJIB:

1. Tidak corrupt `brain.anr` secara permanen.
2. Tidak memasuki infinite loop.
3. Tidak menghabiskan memory tanpa batas.
4. Tidak mematikan safety layer.
5. Tidak mengeksekusi data sebagai code.
6. Dapat masuk degraded mode jika aman.
7. Dapat recover ke generation valid.
8. Mencatat error/telemetry lokal.
9. Menolak melanjutkan operasi jika state tidak aman.

---

# BAGIAN V — PERFORMANCE TEST CONTRACT

---

## 11. Performance Requirements

Performance test WAJIB menjaga invariant:

```text
Safety > Control latency > Memory boundedness > Cache locality > SIMD > Parallelism > Storage efficiency > Learning throughput
```

Minimum:

```text
60 performance/benchmark tests
```

### 11.1 Performance Family Quota

| No | Performance Family | Minimum |
|---:|---|---:|
| 1 | Control loop latency | 10 |
| 2 | Neural kernel throughput | 10 |
| 3 | Storage read/write throughput | 10 |
| 4 | GC/replay/consolidation overhead | 10 |
| 5 | Perception/camera/audio throughput | 10 |
| 6 | Memory/CPU boundedness | 10 |
| **Total** |  | **60** |

### 11.2 Performance Gate

CI WAJIB gagal jika:

1. Control path p95 latency regresi > 5% tanpa approval.
2. Control path p99 latency regresi > 10% tanpa approval.
3. Memory usage melewati max quota.
4. GC pause mengganggu safety/control melebihi batas.
5. Throughput neural kernel turun > 10% pada target platform utama tanpa approval.
6. Terjadi memory leak pada soak test.

### 11.3 Embedded Profile Test

WAJIB ada performance/smoke test dengan constrained profile:

```text
CPU limit: 2 cores
RAM limit: 512 MB
Storage: temp limited
SIMD: scalar-only dan NEON/AVX jika tersedia
```

Tujuan:

1. Membuktikan ANR tetap berjalan pada minimum target.
2. Membuktikan bounded memory.
3. Membuktikan tidak ada OOM pada operasi normal ringan.

---

# BAGIAN VI — CONFORMANCE TEST CONTRACT

---

## 12. Conformance Requirements

Conformance test WAJIB memvalidasi kepatuhan terhadap Architecture Contract.

Minimum:

```text
100 conformance tests/checks
```

### 12.1 Conformance Family Quota

| No | Conformance Family | Minimum |
|---:|---|---:|
| 1 | Architectural invariant checks | 40 |
| 2 | Deployment artifact checks | 10 |
| 3 | CLI command checks | 10 |
| 4 | Offline/no-cloud checks | 10 |
| 5 | Hardware target checks | 10 |
| 6 | Security boundary checks | 20 |
| **Total** |  | **100** |

### 12.2 Mandatory Conformance Checks

Conformance test WAJIB memvalidasi minimal:

1. Binary tunggal `anr`.
2. Persistent artifact tunggal `brain.anr`.
3. Tidak ada file neural terpisah.
4. Cortex/Cerebellum/Hippocampus logical sections.
5. Non-Transformer core invariant.
6. Cell → Column → Block hierarchy.
7. Synapse sebagai koneksi.
8. SoA hot layout.
9. Sparse computation.
10. Bounded queues.
11. Safety layer mandatory.
12. GC Hippocampus otomatis.
13. Memory isolation.
14. Transactional brain write.
15. Recovery generation.
16. Scalar fallback.
17. Offline operation.
18. No mandatory GPU.
19. No mandatory cloud.
20. Brain data not executable.

---

# BAGIAN VII — SECURITY TEST CONTRACT

---

## 13. Security Requirements

Minimum:

```text
40 security tests
```

### 13.1 Security Family Quota

| No | Security Family | Minimum |
|---:|---|---:|
| 1 | Malformed brain.anr | 15 |
| 2 | Offset/length/index attacks | 10 |
| 3 | Plugin isolation abuse | 5 |
| 4 | Unsafe memory/serialization behavior | 5 |
| 5 | Dependency/advisory checks | 5 |
| **Total** |  | **40** |

### 13.2 Mandatory Security Tests

WAJIB ada test untuk:

1. Magic invalid.
2. Version invalid.
3. Header size invalid.
4. Offset out-of-bounds.
5. Size overflow.
6. Section boundary violation.
7. Checksum mismatch.
8. Truncated file.
9. Excessively large metadata.
10. Invalid allocation table.
11. Invalid global index.
12. Brain containing executable-like payload.
13. Plugin input malicious.
14. Sensor frame malicious payload.
15. Config malicious value.
16. Dependency advisory critical/high.

### 13.3 Security Gate

CI WAJIB gagal jika:

1. Ada celah yang memungkinkan brain data dieksekusi.
2. Ada crash karena input malformed yang tidak ditangani.
3. Ada dependency critical advisory yang belum mitigasi.
4. Ada unsafe block tanpa justifikasi/safety comment.
5. Ada memory safety violation pada safe boundary.

---

# BAGIAN VIII — CI PIPELINE CONTRACT

---

## 14. CI Stages

CI WAJIB memiliki stage berikut:

```text
1. Contract & Catalog Check
2. Format Check
3. Static Analysis
4. Build
5. Unit Tests
6. Integration Tests
7. E2E Tests
8. Fault Injection Tests
9. Coverage & Mutation
10. Performance/Benchmark
11. Security/Dependency Audit
12. Artifact Packaging
13. Conformance Report
```

Semua stage WAJIB memiliki status blocking kecuali dinyatakan eksplisit sebagai informational.

---

## 15. Stage Requirements

### 15.1 Contract & Catalog Check

WAJIB memeriksa:

1. `tests/catalog.toml` valid.
2. Semua test memiliki ID unik.
3. Semua test memiliki mapping requirement.
4. Unit test count ≥ 840.
5. Integration count ≥ 200.
6. E2E count ≥ 120.
7. Fault count ≥ 80.
8. Performance count ≥ 60.
9. Conformance count ≥ 100.
10. Security count ≥ 40.

Jika salah satu gagal, pipeline WAJIB gagal.

### 15.2 Format Check

WAJIB menjalankan:

```text
cargo fmt --check
```

Pipeline gagal jika ada format tidak sesuai.

### 15.3 Static Analysis

WAJIB menjalankan minimal:

```text
cargo clippy --all-targets --all-features -- -D warnings
```

Disarankan/boleh ditambah:

```text
cargo deny check
cargo audit
cargo udeps
cargo geiger
```

Gate:

1. Clippy warnings = 0.
2. Tidak ada dependency terlarang.
3. Tidak ada advisory critical/high tanpa mitigasi.
4. Unsafe usage harus memiliki safety comment.

### 15.4 Build

WAJIB build:

```text
debug
release
```

Untuk target utama:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
```

Build release WAJIB menghasilkan binary `anr`.

Binary WAJIB:

1. Dapat dijalankan.
2. Memiliki CLI minimum.
3. Tidak memerlukan file eksternal wajib selain config opsional.
4. Dapat menjalankan `anr brain verify`.

### 15.5 Unit Tests

WAJIB menjalankan seluruh unit test.

Gate:

```text
pass rate = 100%
minimum unit count = 840
no ignored required tests
```

### 15.6 Integration Tests

WAJIB menjalankan seluruh integration test.

Gate:

```text
pass rate = 100%
minimum integration count = 200
```

### 15.7 E2E Tests

WAJIB menjalankan seluruh E2E test.

Gate:

```text
pass rate = 100%
minimum E2E count = 120
```

E2E WAJIB menggunakan binary hasil build, bukan mock runtime penuh.

### 15.8 Fault Injection Tests

WAJIB menjalankan seluruh fault injection test.

Gate:

```text
pass rate = 100%
minimum fault count = 80
```

### 15.9 Coverage & Mutation

Coverage WAJIB diukur dengan tool yang mendukung Rust, misalnya llvm-cov atau setara.

Minimum gate:

| Metric | Minimum |
|---|---:|
| Overall line coverage | ≥ 85% |
| Critical module line coverage | ≥ 95% |
| Critical module branch coverage | ≥ 85% |
| Critical module mutation score | ≥ 65% |

Critical modules WAJIB mencakup minimal:

```text
storage/
memory/
brain/
neural/
action/safety
core/
```

Jika coverage turun dari baseline pada critical module, pipeline WAJIB gagal.

### 15.10 Performance/Benchmark

WAJIB menjalankan benchmark suite.

Gate:

1. Tidak ada regression kontrol latency melebihi threshold.
2. Tidak ada memory quota violation.
3. Tidak ada GC pause berbahaya.
4. Benchmark artifact tersimpan.
5. Perbandingan dengan baseline tersedia.

### 15.11 Security/Dependency Audit

WAJIB menjalankan:

```text
cargo audit
cargo deny check advisories bans licenses
```

Gate:

```text
critical = 0
high = 0 tanpa waiver
license policy = pass
banned dependencies = 0
```

### 15.12 Artifact Packaging

CI WAJIB menghasilkan artifact:

```text
anr binary
brain.anr golden sample
test reports
coverage report
benchmark report
SBOM
checksum manifest
conformance report
```

Artifact WAJIB diberi hash dan version.

### 15.13 Conformance Report

CI WAJIB menghasilkan laporan yang memuat:

1. Jumlah test per level.
2. Test yang gagal.
3. Test yang di-ignore.
4. Coverage.
5. Mutation score.
6. Benchmark delta.
7. Dependency advisory.
8. Mapping requirement → test.
9. Status conformance terhadap Architecture Contract.

---

# BAGIAN IX — CI ENVIRONMENT CONTRACT

---

## 16. Runner Environment

CI WAJIB menyediakan:

1. Linux x86_64 runner.
2. Linux ARM64 runner atau emulasi QEMU.
3. Rust stable pinned.
4. Cache dependency.
5. Temporary storage.
6. Resource limit untuk embedded profile.
7. Network restricted untuk runtime test.

### 16.1 Network Policy

Test runtime WAJIB dijalankan tanpa akses network keluar.

Jika CI memerlukan fetch dependency:

```text
fetch hanya pada tahap setup
bukan pada tahap runtime test
```

### 16.2 Deterministic Environment

CI WAJIB mendukung:

1. Fixed random seed.
2. Mocked clock.
3. Deterministic temp directory.
4. Reproducible fixture.
5. Stable environment variable.

---

## 17. Platform Matrix

Minimum platform matrix:

| Platform | Purpose | Required |
|---|---|---|
| x86_64 Linux | Primary CI | YES |
| aarch64 Linux | Embedded target | YES |
| scalar-only build | Fallback | YES |
| NEON build | ARM SIMD | YES |
| AVX2 build | x86 SIMD | YES |
| no-GPU build | Offline embedded | YES |
| GPU build | Optional | NO |

GPU tidak boleh menjadi required gate.

---

# BAGIAN X — TEST DATA, MOCKS, AND SIMULATION CONTRACT

---

## 18. Test Data Contract

Test data WAJIB versioned dan checksummed.

Minimum fixture:

1. Valid brain golden.
2. Empty brain golden.
3. Seed knowledge golden.
4. Seed skill golden.
5. Seed episode golden.
6. Corrupt brain corpus.
7. Truncated brain corpus.
8. Checksum-mismatch brain corpus.
9. Sensor frame fixture.
10. Camera frame fixture.
11. Audio frame fixture.
12. Actuator expected log fixture.

### 18.1 Golden Brain

Golden brain WAJIB:

1. Memiliki checksum.
2. Dapat diverifikasi oleh `anr brain verify`.
3. Digunakan untuk regression.
4. Tidak boleh berubah tanpa perubahan version/contract.

---

## 19. Mock and Simulation Contract

### 19.1 Allowed Mocks

Mock BOLEH digunakan untuk:

1. Sensor.
2. Camera.
3. Audio.
4. Actuator.
5. HAL.
6. Clock.
7. Resource limiter.
8. Network failure.

### 19.2 Forbidden Mocks

Mock TIDAK BOLEH digunakan untuk menggantikan:

1. Safety layer.
2. Brain validation.
3. Storage transaction.
4. Recovery logic.
5. Memory isolation.
6. GC logic.
7. Checksum validation.

Safety dan storage integrity WAJIB diuji dengan implementasi nyata atau test-double yang sangat ketat dan telah diverifikasi equivalensinya.

---

## 20. Simulation Harness

CI WAJIB memiliki simulation harness yang menyediakan:

```text
virtual sensor
virtual camera
virtual audio
virtual actuator
virtual HAL
virtual clock
fault injector
load generator
brain inspector
safety oracle
telemetry oracle
```

Simulation harness WAJIB deterministik untuk test yang diberi label deterministic.

---

# BAGIAN XI — FLAKY TEST AND IGNORE POLICY

---

## 21. Flaky Test Policy

### 21.1 Zero Known Flaky on Main

Branch utama WAJIB tidak memiliki known flaky test.

Jika test flaky terdeteksi:

1. Test WAJIB dikarantina.
2. Issue dibuat.
3. Root cause dianalisis.
4. Test tidak boleh dihitung sebagai required gate selama quarantine.
5. Quarantine harus memiliki batas waktu dan owner.

### 21.2 No Retry to Pass

CI TIDAK BOLEH menggunakan retry otomatis untuk meloloskan test yang gagal.

Retry hanya BOLEH digunakan untuk diagnostik, bukan untuk mengubah status gate.

### 21.3 Ignore Policy

Test dengan status `ignore`, `skip`, atau setara TIDAK BOLEH ada pada required suite di branch utama tanpa waiver.

Setiap ignore WAJIB memiliki:

1. Issue ID.
2. Reason.
3. Owner.
4. Expiry.
5. Replacement test jika diperlukan.

---

# BAGIAN XII — TRACEABILITY CONTRACT

---

## 22. Requirement Traceability

Setiap test WAJIB memiliki mapping ke minimal satu requirement.

Format mapping:

```text
ArchitectureContract#<clause>
TestsCIContract#<clause>
```

Contoh:

```text
ArchitectureContract#31.2
ArchitectureContract#45.1
TestsCIContract#9.6
```

### 22.1 Mandatory Traceability Coverage

WAJIB ada test untuk setiap kategori Architecture Contract berikut:

| Architecture Area | Required Test Levels |
|---|---|
| Single binary deployment | Conformance, E2E |
| Single brain.anr | Conformance, Storage, E2E |
| Cell/Column/Block | Unit, Integration |
| Synapse | Unit, Integration |
| Sparse computation | Unit, Performance |
| SoA | Unit, Performance |
| Cortex | Unit, Integration, E2E |
| Cerebellum | Unit, Integration, E2E |
| Hippocampus | Unit, Integration, E2E |
| Learning | Unit, Integration, E2E |
| Replay | Unit, Integration, E2E |
| Consolidation | Unit, Integration, E2E |
| Retention/GC | Unit, Integration, E2E, Fault |
| Memory allocation | Unit, Integration, Fault |
| HOT/WARM/COLD | Unit, Integration |
| Storage validation | Unit, Security, Fault |
| Transactional write | Unit, Integration, Fault |
| Recovery | Unit, Integration, E2E, Fault |
| Perception | Unit, Integration, E2E |
| Camera | Unit, Integration, E2E |
| Plugin/HAL | Unit, Integration, E2E, Fault |
| Decision | Unit, Integration, E2E |
| Safety | Unit, Integration, E2E, Fault, Conformance |
| CLI | Unit, E2E, Conformance |
| Diagnostics/Telemetry | Unit, Integration, E2E |
| SIMD | Unit, Integration, Performance |
| Scalar fallback | Unit, Integration, Performance |
| Security | Security, Fault, Conformance |

---

## 23. Traceability Report

CI WAJIB menghasilkan traceability report yang memuat:

```text
requirement_id
test_ids
coverage_status
criticality
last_run_status
```

Jika ada requirement kritis tanpa test, pipeline WAJIB gagal.

---

# BAGIAN XIII — RELEASE GATE CONTRACT

---

## 24. Release Gate

Sebuah build ANR BOLEH dinyatakan release-ready hanya jika:

1. Semua CI stage required pass.
2. Unit test count ≥ 840.
3. Integration test count ≥ 200.
4. E2E test count ≥ 120.
5. Fault test count ≥ 80.
6. Performance test count ≥ 60.
7. Conformance test count ≥ 100.
8. Security test count ≥ 40.
9. Coverage threshold terpenuhi.
10. Mutation threshold critical terpenuhi.
11. Tidak ada flaky required test aktif.
12. Tidak ada dependency critical/high advisory tanpa waiver.
13. Benchmark regression dalam batas.
14. Binary artifact checksum valid.
15. Golden brain verification pass.
16. Conformance report pass.

Jika salah satu gagal, release WAJIB diblokir.

---

## 25. Waiver Process

Waiver hanya BOLEH diberikan untuk kondisi terbatas.

Waiver WAJIB:

1. Tertulis.
2. Memiliki owner.
3. Memiliki expiry.
4. Memiliki risk assessment.
5. Tidak berlaku untuk safety-critical failure.
6. Tidak berlaku untuk storage integrity failure.
7. Tidak berlaku untuk single-binary/single-brain violation.
8. Tidak berlaku untuk offline-first violation.

Waiver TIDAK BOLEH digunakan untuk menurunkan minimum test count secara permanen.

---

# BAGIAN XIV — NIGHTLY / EXTENDED TEST CONTRACT

---

## 26. Extended CI

Selain CI utama, WAJIB ada extended CI minimal nightly.

Extended CI WAJIB mencakup:

1. Fuzzing brain parser.
2. Fuzzing sensor frame parser.
3. Fuzzing seed input.
4. Long-running soak test.
5. Memory leak test.
6. Full mutation testing.
7. Cross-platform matrix lebih lengkap.
8. Benchmark full suite.
9. Recovery chaos test.
10. Plugin failure chaos test.

### 26.1 Soak Test

Soak test WAJIB menjalankan runtime simulasi dalam durasi panjang.

Minimum:

```text
4 jam untuk nightly
8 jam untuk release candidate
```

Soak test WAJIB memverifikasi:

1. Tidak ada memory leak.
2. Tidak ada file descriptor leak.
3. Tidak ada queue growth tanpa batas.
4. Tidak ada degradation latency progresif.
5. GC tetap bounded.
6. Brain generation tidak corrupt.
7. Telemetry tetap konsisten.

### 26.2 Fuzzing

Fuzzing WAJIB menargetkan minimal:

```text
brain.anr parser
header parser
index parser
seed parser
sensor frame parser
camera frame parser
plugin capability parser
```

Jika fuzzing menemukan crash atau memory safety issue, issue WAJIB diperlakukan sebagai blocking.

---

# BAGIAN XV — ENFORCEMENT AND NON-CONFORMANCE

---

## 27. Enforcement

Kontrak ini WAJIB ditegakkan melalui:

1. CI gate.
2. Branch protection.
3. Test catalog validation.
4. Coverage gate.
5. Benchmark gate.
6. Security gate.
7. Conformance report.

Tidak ada merge ke branch utama jika required gate gagal.

---

## 28. Non-Conformance Conditions

Implementasi dinyatakan non-conformance jika:

1. Unit test < 840.
2. Total required test < 1440.
3. Ada required test gagal.
4. Ada required test di-ignore tanpa waiver.
5. Coverage critical di bawah threshold.
6. Safety test gagal.
7. Storage integrity test gagal.
8. Offline test gagal.
9. Single-binary deployment test gagal.
10. Single-brain deployment test gagal.
11. Flaky test dibiarkan tanpa quarantine.
12. Benchmark control latency regression melewati batas.
13. Security critical issue terbuka.
14. Traceability requirement kritis kosong.

---

## 29. Definition of Done for Testing

Sebuah perubahan kode dianggap selesai dari sisi Tests-CI jika:

1. Semua test baru/perubahan terdaftar di catalog.
2. Semua required CI pass.
3. Coverage tidak turun pada critical module.
4. Tidak ada clippy warning.
5. Tidak ada fmt issue.
6. Tidak ada regression benchmark kritis.
7. Tidak ada flaky baru.
8. Traceability updated.
9. Conformance report pass.
10. Artifact build valid.

---

# BAGIAN XVI — FINAL TESTS-CI INVARIANTS

---

## 30. Tests-CI Invariants

Kontrak ini mengunci invariant berikut:

```text
Minimum 840 unit tests.
Minimum 200 integration tests.
Minimum 120 end-to-end tests.
Minimum 80 fault injection tests.
Minimum 60 performance tests.
Minimum 100 conformance tests.
Minimum 40 security tests.

All required tests must pass.
All required tests must be cataloged.
All required tests must be traceable.
No cloud dependency in tests.
No mandatory hardware in merge gate.
No ignored required tests without waiver.
No retry-to-pass for failing tests.
No release with safety/storage failure.
No coverage regression on critical modules.
No benchmark regression on control path without approval.
No flaky required tests on main.
No brain trust before validation.
No actuator command without safety check.
No learning override safety.
No unbounded queue/memory behavior.
No single-binary/single-brain violation.
```

---

## 31. Acceptance Checklist

Sebelum implementasi dinyatakan memenuhi Tests-CI Contract, checklist berikut WAJIB bernilai benar:

```text
[ ] Unit tests ≥ 840.
[ ] Integration tests ≥ 200.
[ ] E2E tests ≥ 120.
[ ] Fault tests ≥ 80.
[ ] Performance tests ≥ 60.
[ ] Conformance tests ≥ 100.
[ ] Security tests ≥ 40.
[ ] Semua test memiliki ID unik.
[ ] Semua test memiliki requirement mapping.
[ ] Semua test deterministic atau memiliki controlled nondeterminism.
[ ] Unit suite pass 100%.
[ ] Integration suite pass 100%.
[ ] E2E suite pass 100%.
[ ] Fault suite pass 100%.
[ ] Security suite pass 100%.
[ ] Coverage threshold terpenuhi.
[ ] Mutation threshold critical terpenuhi.
[ ] Clippy zero warnings.
[ ] Fmt pass.
[ ] Dependency audit pass.
[ ] Benchmark gate pass.
[ ] Offline runtime test pass.
[ ] Single-binary deployment test pass.
[ ] Single-brain deployment test pass.
[ ] Safety layer test pass.
[ ] Storage recovery test pass.
[ ] GC boundedness test pass.
[ ] Flaky policy enforced.
[ ] Traceability report pass.
[ ] Conformance report pass.
```

---

## 32. Penutup

ANR — Tests-CI Contract ini adalah kontrak pengujian end-to-end yang ketat dan terstruktur.

Kontrak ini memastikan bahwa implementasi ANR:

1. Tidak melenceng dari Architecture Contract.
2. Tidak mengorbankan safety.
3. Tidak merusak integritas `brain.anr`.
4. Tidak melanggar single-binary dan single-brain deployment.
5. Tidak memperkenalkan ketergantungan cloud.
6. Tidak mengabaikan bounded memory dan bounded queue.
7. Tidak mengurangi kualitas learning, consolidation, dan GC.
8. Tidak menurunkan determinisme control path.
9. Tidak melewati validasi security dan fault tolerance.
10. Tidak dapat dinyatakan release-ready tanpa bukti pengujian lengkap.

Dokumen ini mengikat seluruh pengembangan ANR. Setiap perubahan yang menurunkan strictness kontrak ini WAJIB diperlakukan sebagai perubahan kontrak eksplisit, bukan perubahan implementasi biasa.
