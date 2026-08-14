# ANR — Master Technical Specification  
## Supporting Technical Documents for Architecture Contract Final Architectural Baseline v1.1

**Version:** `1.0`  
**Status:** `NORMATIVE SUPPORTING DOCUMENTS`  
**Parent document:** `ANR Architecture Contract — Final Architectural Baseline v1.1`  
**Purpose:** Memperjelas detail implementasi yang belum dikunci pada Architecture Contract tanpa mengubah invariant arsitektural.

---

## 0. Kedudukan Dokumen

Dokumen ini adalah kumpulan dokumen pendukung teknis untuk:

```text
ANR Architecture Contract — Final Architectural Baseline v1.1
```

Kedudukan normatif:

```text
Architecture Contract v1.1
        │
        ▼
Master Technical Specification — Supporting Documents v1.0
        │
        ▼
Implementation / Testing / CI / Deployment / Product Validation
```

Jika terjadi konflik:

```text
Architecture Contract > Supporting Documents > Implementation detail
```

Dokumen ini:

1. Tidak mengubah invariant Architecture Contract.
2. Tidak membuat file persistent baru selain `brain.anr`.
3. Tidak menjadikan `.cx`, `.cm`, atau `.hs` sebagai artifact persistent.
4. Tidak menjadikan Transformer, cloud, atau GPU sebagai dependency wajib.
5. Tidak berisi roadmap, sprint, milestone, atau jadwal implementasi.
6. Setiap keputusan teknis harus traceable ke Architecture Contract.

---

## 1. Global Traceability Rule

Setiap dokumen pendukung menggunakan notasi:

```text
AC §<nomor>
```

untuk merujuk ke Architecture Contract.

Contoh:

```text
AC §4   Deployment Artifact Contract
AC §5   Single Brain Contract
AC §31  Safety Layer Contract
AC §45  Transactional Write Contract
```

Setiap requirement pendukung WAJIB memiliki traceability minimal satu klausul Architecture Contract.

---

# SD-01 — Runtime State Machine & Lifecycle

## 1.1 Scope

Dokumen ini mengunci state machine runtime ANR, lifecycle boot, run, degraded, emergency stop, maintenance, dan shutdown.

Traceability:

```text
AC §18 Autonomous Loop Contract
AC §19 Boot Contract
AC §20 Shutdown Contract
AC §21 Degraded Mode Contract
AC §31 Safety Layer Contract
```

## 1.2 Terminology

| Istilah | Arti |
|---|---|
| Runtime State | Status eksekusi global ANR. |
| Safe State | Kondisi actuator aman. |
| Degraded State | Operasi terbatas yang masih aman. |
| EmergencyStopped | Kondisi penghentian darurat. |
| Maintenance Mode | Pekerjaan background non-critical. |

## 1.3 Normative Requirements

Runtime WAJIB memiliki state machine berikut:

```text
PoweredOff
   ↓
Boot
   ↓
ConfigLoad
   ↓
BrainOpen
   ↓
BrainValidate
   ↓
RecoveryIfNeeded
   ↓
CpuSimdDetect
   ↓
MemoryInit
   ↓
HalInit
   ↓
PluginInit
   ↓
NeuralInit
   ↓
SchedulerInit
   ↓
Running
   ↓
Degraded / Maintenance / EmergencyStopped / ShuttingDown
```

Ketentuan:

1. Runtime TIDAK BOLEH masuk `Running` sebelum `brain.anr` valid.
2. Runtime TIDAK BOLEH mengaktifkan actuator sebelum safety layer ready.
3. `EmergencyStopped` WAJIB dapat dimasuki dari `Running` atau `Degraded`.
4. `Shutdown` WAJIB mempertahankan integritas brain jika memungkinkan.
5. `Degraded` TIDAK BOLEH menonaktifkan safety.
6. Maintenance WAJIB berjalan hanya jika tidak melanggar control/safety budget.

## 1.4 Interface / Schema

Logical runtime state:

```rust
enum RuntimeState {
    PoweredOff,
    Boot,
    ConfigLoad,
    BrainOpen,
    BrainValidate,
    Recovery,
    CpuSimdDetect,
    MemoryInit,
    HalInit,
    PluginInit,
    NeuralInit,
    SchedulerInit,
    Running,
    Degraded,
    EmergencyStopped,
    ShuttingDown,
    Fault,
}
```

Runtime event:

```rust
enum RuntimeEvent {
    PowerOn,
    ConfigLoaded,
    BrainOpened,
    BrainValid,
    BrainInvalid,
    RecoveryComplete,
    RecoveryFailed,
    PluginsReady,
    PluginFailed,
    NeuralReady,
    SchedulerReady,
    SafetyTriggered,
    EmergencyStopRequested,
    ShutdownRequested,
    FatalError,
}
```

## 1.5 Invariants

```text
No actuator before safety ready.
No learning before brain valid.
No running without valid brain generation.
Emergency stop always reachable from Running/Degraded.
Degraded mode never disables safety.
```

## 1.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Config invalid | Gunakan default aman atau masuk Fault. |
| Brain missing | Reject run kecuali provisioning eksplisit. |
| Brain corrupt | Recovery ke generation valid atau Fault. |
| Plugin failure | Degraded, restart, atau disable plugin. |
| Safety failure | EmergencyStopped. |
| Storage failure | Degraded volatile jika diizinkan, otherwise safe stop. |

## 1.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Boot valid brain.
2. Boot invalid brain.
3. Boot missing brain.
4. Transition ke Degraded.
5. Transition ke EmergencyStopped.
6. Graceful shutdown.
7. Emergency shutdown.
8. Recovery setelah corrupt generation.
9. State machine tidak melompati safety initialization.

## 1.8 Configuration Example

```toml
[runtime]
state_trace = true
shutdown_timeout_ms = 5000
emergency_stop_timeout_ms = 100
allow_volatile_degraded_mode = false

[states]
plugin_init_timeout_ms = 3000
hal_init_timeout_ms = 2000
brain_validation_strict = true
```

---

# SD-02 — Repository / Module Boundary

## 2.1 Scope

Dokumen ini mengunci struktur repository Rust dan boundary antar modul agar implementasi tidak melanggar arsitektur.

Traceability:

```text
AC §64 Language Contract
AC §65 Repository Structure Contract
AC §66 Module Responsibility Contract
AC §67 Dependency Boundary Contract
```

## 2.2 Terminology

| Istilah | Arti |
|---|---|
| Crate | Rust package utama `anr`. |
| Module | Subunit source code. |
| Boundary | Aturan dependensi antar modul. |
| Feature flag | Opsional compile-time capability. |

## 2.3 Normative Requirements

Repository WAJIB mengikuti struktur:

```text
anr/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── src/
│   ├── main.rs
│   ├── error.rs
│   ├── core/
│   ├── neural/
│   ├── brain/
│   ├── learning/
│   ├── memory/
│   ├── storage/
│   ├── perception/
│   ├── plugins/
│   ├── hardware/
│   ├── action/
│   ├── simd/
│   └── interface/
├── tests/
├── benches/
├── examples/
└── docs/
```

Dependency boundary WAJIB:

1. `neural` TIDAK BOLEH bergantung pada `hardware`, `plugins`, atau `storage`.
2. `learning` TIDAK BOLEH mengirim perintah actuator langsung.
3. `action/safety` TIDAK BOLEH di-bypass oleh `learning` atau `decision`.
4. `storage` WAJIB menjadi satu-satunya modul yang menulis `brain.anr`.
5. `plugins` WAJIB terisolasi dari neural internal state kecuali melalui interface resmi.
6. `interface` hanya mengakses runtime melalui API publik.

## 2.4 Interface / Schema

Module responsibility:

```text
core/       runtime lifecycle, scheduler, error orchestration
neural/     Cell, Column, Block, Synapse, sparse graph
brain/      Cortex, Cerebellum, Hippocampus logical memory
learning/   hebbian, temporal, replay, consolidation
memory/     allocator, quota, retention, GC, tiering
storage/    brain.anr IO, transaction, checksum, recovery
perception/ sensor frame, preprocessing, fusion
plugins/    plugin lifecycle, isolation
hardware/   HAL
action/     decision, safety, actuator, feedback
simd/       scalar/NEON/AVX abstraction
interface/  CLI, diagnostics, telemetry
```

## 2.5 Invariants

```text
neural core is hardware-agnostic.
storage owns brain persistence.
safety owns actuator authorization.
learning cannot command actuators directly.
plugins cannot corrupt brain directly.
```

## 2.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Compile-time boundary violation | CI gagal. |
| Module panic | Error isolation; jika critical, safe shutdown. |
| Unsafe misuse | CI static analysis gagal. |

## 2.7 Validation / Test Requirements

WAJIB ada:

1. Dependency graph lint.
2. Architecture boundary test.
3. Module isolation test.
4. Unsafe audit.
5. Clippy gate.
6. Cargo feature matrix test.

## 2.8 Configuration Example

```toml
[features]
default = ["simd-auto", "plugins-static"]
plugins-static = []
plugins-dynamic = []
simd-auto = []
simd-scalar-only = []
android-support = []
```

---

# SD-03 — brain.anr Binary Format & Schema

## 3.1 Scope

Dokumen ini mengunci format binary `brain.anr`, layout section, header, allocation table, index, dan integrity block.

Traceability:

```text
AC §5 Single Brain Contract
AC §48 Header Contract
AC §47 brain.anr Structure Contract
AC §44 Storage Validation Contract
AC §45 Transactional Write Contract
```

## 3.2 Terminology

| Istilah | Arti |
|---|---|
| Superblock | Header utama brain. |
| Section | Cortex/Cerebellum/Hippocampus region. |
| Generation | Nomor transaksi commit. |
| Tier | HOT/WARM/COLD. |
| Allocation Table | Tabel blok/region terpakai. |

## 3.3 Normative Requirements

1. `brain.anr` WAJIB little-endian.
2. Block size default WAJIB `4096` bytes.
3. Header WAJIB berada di block 0.
4. Backup header WAJIB berada di block 1.
5. Setiap section WAJIB memiliki header sendiri.
6. Setiap commit WAJIB memiliki checksum.
7. Offset WAJIB aligned minimal 4096 bytes untuk region utama.
8. File TIDAK BOLEH bergantung pada external sidecar file.

## 3.4 Interface / Schema

### 3.4.1 Primary Superblock

```text
offset  field                     type
0       magic                     [u8; 4]
4       format_version            u32
8       header_size               u32
12      flags                     u32
16      total_size                u64
24      generation                u64
32      cortex_offset             u64
40      cortex_size               u64
48      cerebellum_offset         u64
56      cerebellum_size           u64
64      hippocampus_offset        u64
72      hippocampus_size          u64
80      index_offset              u64
88      index_size                u64
96      metadata_offset           u64
104     metadata_size             u64
112     allocation_table_offset   u64
120     allocation_table_size     u64
128     section_table_offset      u64
136     section_table_count       u32
140     block_size                u32
144     checksum_algo             u8
145     checksum_scope            u8
146     reserved                  [u8; 102]
248     header_crc                u32
252     reserved                  u32
256     checksum                  [u8; 32]
288     reserved_to_header_size   [u8; ...]
```

Magic:

```text
"ANRB" = 0x41 0x4E 0x52 0x42
```

Checksum algorithm:

```text
0 = BLAKE3
1 = CRC32C
```

Default:

```text
checksum_algo = 0  # BLAKE3
```

Checksum dihitung atas:

```text
header fields excluding checksum field
section table
allocation table
global index metadata
committed generation metadata
```

Implementasi BOLEH menambah section-level checksum tambahan.

### 3.4.2 Section Header

Setiap section WAJIB memiliki header:

```text
field                 type
section_type          u8
section_version       u16
flags                 u16
hot_offset            u64
warm_offset           u64
cold_offset           u64
index_offset          u64
used_bytes            u64
min_bytes             u64
target_bytes          u64
max_bytes             u64
generation            u64
checksum              [u8; 32]
```

Section type:

```text
1 = CORTEX
2 = CEREBELLUM
3 = HIPPOCAMPUS
```

### 3.4.3 Record Container

Data di dalam section disimpan sebagai record TLV:

```text
record_magic     [u8; 4]
record_type      u16
record_flags     u16
record_length    u32
object_id        u64
tier             u8
origin           u8
created_at       u64
updated_at       u64
payload          [u8; record_length]
checksum         [u8; 32]
```

Record type minimum:

```text
0x0100 = KNOWLEDGE_PATTERN
0x0200 = SKILL_PROCEDURE
0x0300 = EPISODE
0x0400 = CELL_POOL
0x0500 = COLUMN_POOL
0x0600 = BLOCK_POOL
0x0700 = SYNAPSE_TABLE
0x0800 = INDEX_ENTRY
0x0900 = METADATA
```

## 3.5 Invariants

```text
brain.anr is self-contained.
no .cx/.cm/.hs persistent artifact.
header must be validated before mmap trust.
section offsets must stay within total_size.
generation must be monotonic on successful commit.
checksum mismatch invalidates generation.
```

## 3.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Magic invalid | Reject file. |
| Version unsupported | Reject atau explicit upgrade. |
| Offset out of bounds | Reject file. |
| Checksum mismatch | Recovery ke generation valid. |
| Section corrupt | Isolate section jika aman. |
| Truncated file | Reject/recover sesuai policy. |

## 3.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Parse valid brain.
2. Parse corrupt magic.
3. Parse corrupt version.
4. Parse invalid header size.
5. Parse offset out-of-bound.
6. Parse checksum mismatch.
7. Parse truncated file.
8. Section boundary validation.
9. Backup header fallback.
10. Golden brain regression.

## 3.8 Configuration Example

```toml
[storage]
path = "/opt/anr/brain.anr"
block_size = 4096
checksum_algo = "blake3"
validate_full_index_on_boot = true
allow_mmap = true
backup_header_enabled = true
```

---

# SD-04 — Versioning / Compatibility / Migration / Recovery

## 4.1 Scope

Dokumen ini mengatur versioning brain format, compatibility check, migration, rollback, dan recovery.

Traceability:

```text
AC §48 Header Contract
AC §49 Storage Validation Contract
AC §50 Transactional Write Contract
AC §51 Recovery Contract
AC §70 Brain Update Contract
AC §85 Backward Compatibility
```

## 4.2 Terminology

| Istilah | Arti |
|---|---|
| Format version | Versi binary brain. |
| Runtime version | Versi binary `anr`. |
| Compatible | Dapat dibuka tanpa upgrade destruktif. |
| Migration | Transformasi terkontrol ke versi baru. |
| Rollback | Kembali ke generation/brain sebelumnya. |

## 4.3 Normative Requirements

1. `format_version` WAJIB menggunakan major version.
2. Runtime WAJIB menolak brain dengan major version tidak dikenal.
3. Runtime BOLEH menerima minor/compatible revision jika forward-compatible.
4. Migration WAJIB transactional.
5. Migration TIDAK BOLEH menghapus brain lama sebelum commit baru valid.
6. Rollback WAJIB tersedia untuk update gagal.
7. Recovery WAJIB memilih latest valid generation.
8. Setiap migration WAJIB mencatat provenance di metadata.

## 4.4 Interface / Schema

Metadata versioning:

```toml
[brain.version]
format_version = 1
writer_runtime_version = "1.0.0"
created_at = 1760000000
updated_at = 1760001000
generation = 42
origin = "seed|learned|consolidated|imported|migrated"
```

Compatibility descriptor:

```text
min_reader_version
max_reader_version
migration_supported
readonly_mode_allowed
```

## 4.5 Invariants

```text
no silent destructive migration.
no commit without checksum validation.
no rollback loss if previous generation valid.
incompatible brain cannot enter Running without explicit upgrade.
```

## 4.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Unsupported version | Reject dengan error eksplisit. |
| Migration interrupted | Rollback ke state valid. |
| Partial migration | Reject brain baru. |
| No valid generation | Fault atau explicit provisioning. |

## 4.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Open same version.
2. Open compatible revision.
3. Reject incompatible version.
4. Migration success.
5. Migration power-loss rollback.
6. Update success.
7. Update corrupt rollback.
8. Recovery latest valid generation.
9. Metadata provenance tercatat.

## 4.8 Configuration Example

```toml
[storage.versioning]
allow_compatible_minor = true
require_explicit_upgrade = true
backup_previous_generation = true
max_rollback_generations = 3
```

---

# SD-05 — Brain Seed & Provisioning Cortex/Cerebellum/Hippocampus

## 5.1 Scope

Dokumen ini mengatur Brain Seed, Brain Builder, dan provisioning awal Cortex, Cerebellum, Hippocampus.

Traceability:

```text
AC §16 Initial Brain Provisioning
AC §17 Brain Seed
AC §18 Brain Builder
AC §19 Initial Cortex
AC §20 Initial Cerebellum
AC §21 Initial Hippocampus
AC §22 Initial Brain vs Learned Brain
```

## 5.2 Terminology

| Istilah | Arti |
|---|---|
| Brain Seed | Input provisioning. |
| Knowledge seed | Sumber Cortex awal. |
| Skill seed | Sumber Cerebellum awal. |
| Episode seed | Sumber Hippocampus awal. |
| Provenance | Asal-usul data. |

## 5.3 Normative Requirements

1. Brain Seed TIDAK BOLEH menjadi persistent runtime format.
2. Seed WAJIB divalidasi sebelum build.
3. Seed TIDAK BOLEH berisi executable payload.
4. Provisioning WAJIB dapat mengisi:
   - Cortex,
   - Cerebellum,
   - Hippocampus opsional.
5. Initial Hippocampus WAJIB tetap tunduk pada GC.
6. Setiap objek seed WAJIB mencatat origin `seed`.
7. Brain Builder WAJIB menghasilkan satu `brain.anr`.

## 5.4 Interface / Schema

Seed format normatif menggunakan TOML atau JSON. Contoh TOML:

```toml
[meta]
name = "robot-arm-basic"
version = "1.0.0"
origin = "seed"

[cortex.knowledge]
items = [
  { id = "know_obstacle_front", pattern = "obstacle.front", confidence = 0.9 },
  { id = "know_battery_low", pattern = "battery.low", confidence = 0.8 },
]

[cerebellum.skills]
items = [
  { id = "skill_move_forward", action = "move_forward", validated = true },
  { id = "skill_stop", action = "stop", validated = true },
]

[hippocampus.episodes]
items = [
  { id = "ep_demo_001", context = "lab", action = "move_forward", reward = 0.8 },
]
```

## 5.5 Invariants

```text
seed is input only.
seed cannot override safety constraints.
initial hippocampus is not permanent by default.
all provisioned objects carry provenance.
brain builder outputs single brain.anr.
```

## 5.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Seed schema invalid | Build gagal. |
| Skill tidak validated | Reject atau masukkan sebagai candidate. |
| Knowledge confidence rendah | Masukkan sebagai candidate, bukan Cortex final. |
| Episode terlalu besar | Compress/reject sesuai quota. |

## 5.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Build dari seed valid.
2. Reject seed invalid.
3. Cortex seed provisioning.
4. Cerebellum seed provisioning.
5. Hippocampus seed provisioning.
6. Initial hippocampus dapat GC.
7. Provenance metadata tercatat.
8. Brain verify setelah build.
9. Brain install atomic.

## 5.8 Configuration Example

```toml
[brain.seed]
path = "seed/basic_robot.toml"
strict_validation = true
allow_unvalidated_skills = false
allow_initial_hippocampus = true
initial_hippocampus_retention = "gc_eligible"
```

---

# SD-06 — Neural Core Data Structures & SoA Layout

## 6.1 Scope

Dokumen ini mengunci struktur data Neural Core: Cell, Column, Block, Synapse, dan SoA layout.

Traceability:

```text
AC §11 Neural Core Hierarchy
AC §12 Cell Contract
AC §13 Column Contract
AC §14 Block Contract
AC §15 Synapse Contract
AC §16 Sparse Computation
AC §17 Data Layout Contract
```

## 6.2 Terminology

| Istilah | Arti |
|---|---|
| CellId | Identifier Cell. |
| ColumnId | Identifier Column. |
| BlockId | Identifier Block. |
| SynapseId | Identifier Synapse. |
| ActiveSet | Himpunan entitas aktif pada cycle. |

## 6.3 Normative Requirements

1. Hot neural data WAJIB menggunakan SoA.
2. Hot path TIDAK BOLEH menggunakan `Box<Cell>` per cell.
3. ID WAJIB menggunakan `u64` atau index `u32` dengan versioning jika perlu.
4. Active set WAJIB sparse.
5. Synapse WAJIB mendukung source/target Cell atau Column.
6. Weight WAJIB menggunakan float 32-bit sebagai default.
7. State enum WAJIB compact, maksimal 1 byte untuk hot state.

## 6.4 Interface / Schema

### 6.4.1 Cell Pool

```rust
struct CellPool {
    activation: Vec<f32>,
    potential: Vec<f32>,
    threshold: Vec<f32>,
    state: Vec<u8>,
    refractory_until: Vec<u64>,
    last_active: Vec<u64>,
    usage: Vec<u32>,
}
```

### 6.4.2 Column Pool

```rust
struct ColumnPool {
    cell_start: Vec<u32>,
    cell_len: Vec<u32>,
    activation: Vec<f32>,
    competition_state: Vec<u8>,
    last_active: Vec<u64>,
    usage: Vec<u32>,
}
```

### 6.4.3 Block Pool

```rust
struct BlockPool {
    block_id: Vec<u64>,
    context_tag: Vec<u64>,
    column_set_offset: Vec<u32>,
    column_set_len: Vec<u32>,
    temporal_depth: Vec<u16>,
    prediction_score: Vec<f32>,
    state: Vec<u8>,
}
```

### 6.4.4 Synapse Table

```rust
struct SynapseTable {
    source_kind: Vec<u8>,
    source_id: Vec<u64>,
    target_kind: Vec<u8>,
    target_id: Vec<u64>,
    weight: Vec<f32>,
    strength: Vec<f32>,
    state: Vec<u8>,
    last_active: Vec<u64>,
    age: Vec<u32>,
    plasticity: Vec<f32>,
}
```

### 6.4.5 Active Set

```rust
struct ActiveSet {
    active_columns: Vec<u32>,
    active_cells: Vec<u32>,
    active_synapses: Vec<u32>,
    active_blocks: Vec<u32>,
}
```

## 6.5 Invariants

```text
SoA hot arrays must be contiguous.
no per-cell heap object in hot path.
active graph must be sparse.
synapse source/target must be valid.
block cannot reference non-existent column.
```

## 6.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Invalid CellId | Error validation. |
| Invalid Synapse target | Reject atau prune. |
| Active set overflow | Backpressure/drop sesuai policy. |
| NaN weight | Clamp/reset dan catat error. |

## 6.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Cell activation.
2. Column competition.
3. Block context binding.
4. Synapse update.
5. Sparse traversal.
6. SoA consistency.
7. Invalid ID rejection.
8. NaN/Inf handling.
9. SIMD kernel correctness.
10. Scalar fallback.

## 6.8 Configuration Example

```toml
[neural]
cell_capacity = 65536
column_capacity = 4096
block_capacity = 1024
synapse_capacity = 262144
default_weight_type = "f32"
active_set_max = 8192
nan_guard = true
```

---

# SD-07 — Memory Ownership / Isolation / Allocator

## 7.1 Scope

Dokumen ini mengatur ownership memori, isolasi Cortex/Cerebellum/Hippocampus, quota, dan allocator.

Traceability:

```text
AC §10 Memory Isolation Contract
AC §45 Memory Allocation Contract
AC §59 Resource Isolation
AC §60 RAM Strategy
```

## 7.2 Terminology

| Istilah | Arti |
|---|---|
| Quota | Batas min/target/max per section. |
| Arena | Region alokasi contiguous. |
| Free list | Daftar blok bebas. |
| Pressure | Tingkat pemakaian memori. |

## 7.3 Normative Requirements

1. Setiap section WAJIB memiliki allocator tersendiri.
2. Allocator WAJIB menghormati min/target/max.
3. Section TIDAK BOLEH mengambil hard minimum section lain.
4. Allocation WAJIB bounded.
5. GC section hanya boleh mempengaruhi section sendiri.
6. Allocator WAJIB mendukung defragmentasi/compaction pada maintenance window.
7. OOM pada satu section TIDAK BOLEH menyebabkan OOM global jika masih ada quota section lain.

## 7.4 Interface / Schema

```rust
struct MemoryQuota {
    min_bytes: u64,
    target_bytes: u64,
    max_bytes: u64,
}

struct SectionMemoryState {
    used_bytes: u64,
    reserved_bytes: u64,
    pressure: f32,
    tier_hot_bytes: u64,
    tier_warm_bytes: u64,
    tier_cold_bytes: u64,
}
```

Allocation API:

```rust
trait SectionAllocator {
    fn allocate(&mut self, size: u64, priority: Priority) -> Result<AllocId>;
    fn free(&mut self, id: AllocId) -> Result<()>;
    fn compress(&mut self, id: AllocId) -> Result<()>;
    fn compact(&mut self) -> Result<()>;
}
```

## 7.5 Invariants

```text
hippocampus cannot exhaust cortex reserved memory.
cerebellum cannot exceed max.
no section exceeds max quota.
gc respects ownership.
allocation must be reversible/freeable.
```

## 7.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Section near max | Trigger retention/GC. |
| Allocation low-priority rejected | Catat telemetry. |
| Fragmentation tinggi | Compact saat maintenance. |
| Global RAM pressure | Compress COLD, lazy load. |

## 7.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Quota enforcement.
2. Isolation antar section.
3. Allocate/free.
4. Fragmentation/compaction.
5. Pressure transition.
6. Low-priority allocation rejection.
7. Emergency memory behavior.
8. Memory leak check.

## 7.8 Configuration Example

```toml
[memory.cortex]
min_bytes = 16777216
target_bytes = 67108864
max_bytes = 134217728

[memory.cerebellum]
min_bytes = 16777216
target_bytes = 67108864
max_bytes = 134217728

[memory.hippocampus]
min_bytes = 33554432
target_bytes = 134217728
max_bytes = 268435456
```

---

# SD-08 — Learning Authority, Replay, Consolidation, Retention, GC

## 8.1 Scope

Dokumen ini mengunci otoritas learning, replay selection, consolidation, retention scoring, dan GC behavior.

Traceability:

```text
AC §33 Core Learning Contract
AC §34 Experience Replay Contract
AC §35 Adaptive Consolidation Contract
AC §36 Knowledge Promotion
AC §37 Skill Promotion
AC §40 Retention Engine
AC §41 Hippocampus GC
AC §42 GC Pipeline
```

## 8.2 Terminology

| Istilah | Arti |
|---|---|
| Learning Authority | Hak mengubah state neural. |
| Replay Candidate | Episode terpilih untuk replay. |
| Promotion Candidate | Kandidat knowledge/skill. |
| Retention Score | Nilai kelayakan simpan. |

## 8.3 Normative Requirements

1. Learning WAJIB hanya mengubah synapse/state melalui learning engine resmi.
2. Learning TIDAK BOLEH mengubah safety constraint.
3. Learning TIDAK BOLEH menulis langsung ke Cortex/Cerebellum tanpa consolidation/provisioning valid.
4. Replay WAJIB background/low priority.
5. Consolidation WAJIB adaptive, bukan satu threshold statis.
6. Satu episode TIDAK BOLEH otomatis menjadi knowledge/skill permanen.
7. GC Hippocampus WAJIB mengikuti pressure state AC §41.
8. GC TIDAK BOLEH menghapus episode high-value sebelum consolidation attempt jika memungkinkan.

## 8.4 Interface / Schema

Replay score default:

```text
replay_score =
    0.25 * prediction_error
  + 0.20 * novelty
  + 0.15 * importance
  + 0.15 * reward
  + 0.15 * failure_signal
  + 0.10 * recurrence
```

Retention score default:

```text
retention_score =
    0.15 * recency
  + 0.15 * frequency
  + 0.10 * access_count
  + 0.10 * novelty
  + 0.10 * importance
  + 0.10 * reward
  + 0.10 * success
  + 0.10 * prediction_error
  + 0.05 * context_diversity
  + 0.05 * consolidation_state
```

Semua nilai dinormalisasi ke `[0.0, 1.0]`.

Promotion threshold default:

```text
knowledge_promotion:
  recurrence_min = 3
  context_diversity_min = 2
  confidence_min = 0.75
  contradiction_max = 0.20

skill_promotion:
  execution_min = 5
  success_rate_min = 0.90
  avg_prediction_error_max = 0.15
  stability_min = 0.80
```

## 8.5 Invariants

```text
learning cannot override safety.
one episode cannot become permanent knowledge/skill alone.
replay must not block control.
gc must respect retention value.
consolidation decisions must be auditable.
```

## 8.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Replay queue penuh | Drop low-priority. |
| Consolidation gagal | Episode tetap Hippocampus atau compress. |
| Contradiction | Contextualize/revise, bukan delete langsung. |
| GC emergency | Hapus low-value, pertahankan high-value. |

## 8.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Hebbian update.
2. Temporal association.
3. Replay selection.
4. Replay priority.
5. Knowledge promotion.
6. Skill promotion.
7. Single episode no promotion.
8. Contradiction handling.
9. Retention score.
10. GC pressure states.

## 8.8 Configuration Example

```toml
[learning]
mode = "adaptive"          # adaptive | deterministic
background_priority = "low"

[replay]
queue_capacity = 1024
drop_policy = "drop_low_priority"

[consolidation]
knowledge_confidence_min = 0.75
skill_success_rate_min = 0.90

[retention]
keep_threshold = 0.70
compress_threshold = 0.40
delete_threshold = 0.20
```

---

# SD-09 — Perception Pipeline & Sensor/Camera/Audio Interface

## 9.1 Scope

Dokumen ini mengatur pipeline perception dan interface sensor, camera, audio.

Traceability:

```text
AC §22 Perception Pipeline
AC §23 Sensor Architecture
AC §24 Sensor Frame
AC §25 Camera Architecture
AC §26 Camera Buffer
AC §27 Audio Contract
```

## 9.2 Terminology

| Istilah | Arti |
|---|---|
| SensorFrame | Unit data sensor. |
| Encoder | Pengubah data mentah ke representasi neural. |
| Fusion | Penggabungan multi-sensor. |
| Drop Policy | Kebijakan buffer penuh. |

## 9.3 Normative Requirements

1. Semua sensor WAJIB melalui plugin/HAL.
2. Frame WAJIB memiliki timestamp dan sequence.
3. Buffer camera/audio WAJIB bounded.
4. Perception TIDAK BOLEH bergantung langsung pada hardware spesifik.
5. Drop policy WAJIB eksplisit.
6. Frame invalid WAJIB ditolak atau ditandai quality rendah.
7. Fusion WAJIB menghasilkan representation yang dapat dikonsumsi Neural Core.

## 9.4 Interface / Schema

SensorFrame:

```rust
struct SensorFrame {
    sensor_id: u32,
    timestamp: u64,
    sequence: u64,
    payload: Vec<u8>,
    dimensions: [u32; 3],
    format: SensorFormat,
    quality: f32,
    flags: u32,
}
```

Camera buffer:

```rust
struct CameraBufferConfig {
    max_frames: u32,
    max_frame_bytes: u32,
    drop_policy: DropPolicy,
}
```

Audio buffer:

```rust
struct AudioBufferConfig {
    max_chunks: u32,
    max_chunk_bytes: u32,
    drop_policy: DropPolicy,
}
```

## 9.5 Invariants

```text
no unbounded sensor queue.
no unbounded camera queue.
no unbounded audio queue.
invalid frame cannot corrupt neural state.
timestamp must be monotonic or flagged.
```

## 9.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Sensor timeout | Drop/flag dan degraded pathway. |
| Camera buffer penuh | Drop policy. |
| Audio overrun | Drop/sample. |
| Frame corrupt | Reject dan telemetry. |

## 9.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Sensor frame valid.
2. Sensor frame invalid.
3. Camera drop oldest/newest.
4. Audio drop policy.
5. Timestamp anomaly.
6. Fusion multi-sensor.
7. Preprocessing boundedness.
8. Perception to neural representation.

## 9.8 Configuration Example

```toml
[perception.camera]
max_frames = 8
max_frame_bytes = 2097152
drop_policy = "drop_oldest"

[perception.audio]
max_chunks = 16
max_chunk_bytes = 262144
drop_policy = "drop_newest"

[perception.sensor]
require_timestamp = true
require_sequence = true
invalid_frame_policy = "reject"
```

---

# SD-10 — Plugin Lifecycle & HAL

## 10.1 Scope

Dokumen ini mengatur plugin lifecycle, isolasi failure, dan Hardware Abstraction Layer.

Traceability:

```text
AC §28 Plugin Architecture
AC §29 HAL Contract
AC §30 Plugin Failure Isolation
AC §4 Deployment Single Binary
```

## 10.2 Terminology

| Istilah | Arti |
|---|---|
| Plugin Manifest | Metadata capability plugin. |
| HAL | Hardware abstraction layer. |
| Plugin Isolation | Isolasi kegagalan plugin. |
| Static plugin | Plugin dikompilasi ke binary. |

## 10.3 Normative Requirements

1. Deployment wajib TIDAK BOLEH membutuhkan plugin file eksternal.
2. Plugin BOLEH static atau embedded.
3. Dynamic plugin BOLEH opsional, tetapi tidak boleh menjadi dependensi wajib.
4. Plugin WAJIB memiliki lifecycle:
   - discovered,
   - loaded,
   - initialized,
   - running,
   - degraded,
   - stopped,
   - failed.
5. Plugin failure WAJIB isolated.
6. Plugin TIDAK BOLEH menulis brain.anr langsung.
7. HAL WAJIB menyediakan interface device yang stabil.

## 10.4 Interface / Schema

Plugin manifest:

```toml
[plugin]
name = "camera_v4l2"
kind = "camera"
version = "1.0.0"
capabilities = ["stream", "timestamp", "format_rgb8"]
entry = "static"
```

Plugin trait:

```rust
trait Plugin {
    fn manifest(&self) -> PluginManifest;
    fn init(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn health(&self) -> PluginHealth;
}
```

HAL trait minimal:

```rust
trait Hal {
    fn gpio(&self) -> Result<&dyn Gpio>;
    fn i2c(&self) -> Result<&dyn I2c>;
    fn spi(&self) -> Result<&dyn Spi>;
    fn uart(&self) -> Result<&dyn Uart>;
    fn pwm(&self) -> Result<&dyn Pwm>;
    fn adc(&self) -> Result<&dyn Adc>;
}
```

## 10.5 Invariants

```text
plugin failure cannot crash core.
plugin cannot bypass safety.
plugin cannot persist neural state.
single-binary deployment remains valid.
```

## 10.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Plugin init gagal | Disable plugin. |
| Plugin hang | Watchdog timeout, restart/disable. |
| Plugin panic | Isolate dan degraded. |
| HAL error | Sensor/actuator pathway disabled. |

## 10.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Plugin init/start/stop.
2. Plugin failure isolation.
3. Plugin restart.
4. Plugin disable.
5. HAL mock behavior.
6. Dynamic plugin optional gate.
7. Static plugin conformance.
8. Plugin cannot write brain.

## 10.8 Configuration Example

```toml
[plugins]
mode = "static"
watchdog_timeout_ms = 1000
restart_on_failure = true
max_restarts = 3

[plugins.camera_v4l2]
enabled = true

[plugins.audio_alsa]
enabled = false
```

---

# SD-11 — Decision / Action / Actuator Protocol & Safety Boundary

## 11.1 Scope

Dokumen ini mengatur decision engine, action proposal, actuator command, feedback, safety boundary, dan emergency handling.

Traceability:

```text
AC §30 Decision Engine
AC §31 Safety Layer
AC §32 Feedback Contract
AC §77 Performance Priority
```

## 11.2 Terminology

| Istilah | Arti |
|---|---|
| ActionProposal | Kandidat aksi dari decision engine. |
| SafetyToken | Bukti lolos safety check. |
| ActuatorCommand | Perintah final ke actuator. |
| EmergencyStop | Perintah henti darurat. |

## 11.3 Normative Requirements

1. Setiap aksi WAJIB melalui safety layer.
2. Actuator TIDAK BOLEH menerima command tanpa SafetyToken.
3. Emergency stop WAJIB non-droppable.
4. Safety WAJIB dapat allow, reject, clamp, override, emergency stop.
5. Learning TIDAK BOLEH mengubah safety limit.
6. Feedback WAJIB kembali ke Hippocampus.
7. Actuator failure WAJIB memicu safe state.

## 11.4 Interface / Schema

```rust
struct ActionProposal {
    proposal_id: u64,
    action_type: ActionType,
    parameters: ActionParams,
    confidence: f32,
    source: DecisionSource,
}

struct SafetyDecision {
    proposal_id: u64,
    decision: SafetyVerdict,
    clamped_parameters: Option<ActionParams>,
    reason: SafetyReason,
    token: Option<SafetyToken>,
}

struct ActuatorCommand {
    command_id: u64,
    actuator_id: u32,
    parameters: ActionParams,
    safety_token: SafetyToken,
    timestamp: u64,
}

struct ActuatorFeedback {
    command_id: u64,
    result: ActionResult,
    observed_state: ObservedState,
    prediction_error: f32,
    timestamp: u64,
}
```

Safety verdict:

```text
ALLOW
REJECT
CLAMP
OVERRIDE
EMERGENCY_STOP
```

## 11.5 Invariants

```text
no actuator command without safety token.
emergency stop cannot be dropped.
safety overrides learning.
feedback must be recorded when available.
```

## 11.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Safety timeout | Reject dan safe state. |
| Actuator timeout | Safe stop/retry terbatas. |
| Feedback missing | Prediction error ditandai invalid. |
| Emergency stop requested | Semua actuator masuk safe state. |

## 11.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Proposal allowed.
2. Proposal rejected.
3. Command clamp.
4. Command override.
5. Emergency stop priority.
6. No token no actuator.
7. Feedback to Hippocampus.
8. Actuator failure safe state.

## 11.8 Configuration Example

```toml
[safety]
enable_clamp = true
enable_override = true
emergency_stop_timeout_ms = 50
safe_state_on_timeout = true

[actuators.motor_left]
max_velocity = 1.5
max_torque = 0.8
timeout_ms = 200
```

---

# SD-12 — Scheduler / Concurrency / Priority / Bounded Queue / Backpressure

## 12.1 Scope

Dokumen ini mengatur scheduler, concurrency, priority class, bounded queue, dan backpressure.

Traceability:

```text
AC §55 Scheduler Contract
AC §56 Maintenance Budget
AC §57 Queue Contract
AC §58 Backpressure
AC §60 RAM Strategy
```

## 12.2 Terminology

| Istilah | Arti |
|---|---|
| Priority Class | Kelas prioritas task. |
| Bounded Queue | Antrean berkapasitas tetap. |
| Backpressure | Tekanan saat queue penuh. |
| Maintenance Budget | Budget pekerjaan background. |

## 12.3 Normative Requirements

1. Scheduler WAJIB memiliki kelas:
   - REALTIME,
   - HIGH,
   - NORMAL,
   - LOW,
   - BACKGROUND.
2. Control/safety WAJIB REALTIME atau priority tertinggi yang tersedia.
3. Learning/replay/consolidation/GC WAJIB LOW/BACKGROUND.
4. Semua queue kritis WAJIB bounded.
5. Backpressure policy WAJIB eksplisit.
6. Emergency stop TIDAK BOLEH dropped oleh backpressure normal.
7. Maintenance WAJIB yield terhadap control loop.

## 12.4 Interface / Schema

```rust
enum PriorityClass {
    Realtime,
    High,
    Normal,
    Low,
    Background,
}

struct QueueConfig {
    capacity: usize,
    policy: BackpressurePolicy,
}

enum BackpressurePolicy {
    DropOldest,
    DropNewest,
    Sample,
    Merge,
    Compress,
    Block,
    Reject,
}
```

Queue mapping:

```text
sensor_queue       HIGH     bounded
camera_queue       HIGH     bounded
perception_queue   HIGH     bounded
action_queue       REALTIME bounded, no-drop for e-stop
experience_queue   NORMAL   bounded
learning_queue     LOW      bounded
maintenance_queue  BACKGROUND bounded
```

## 12.5 Invariants

```text
no unbounded queue.
no background task starves safety.
no e-stop dropped.
control latency has priority over learning.
```

## 12.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Queue penuh | Terapkan policy. |
| Realtime overrun | Shed low-priority work. |
| Thread panic | Isolate dan recover jika mungkin. |
| Deadlock risk | Timeout dan safe state. |

## 12.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Priority ordering.
2. Bounded queue capacity.
3. Drop policy.
4. Backpressure behavior.
5. E-stop non-droppable.
6. Maintenance budget yield.
7. Load generator.
8. Starvation prevention.

## 12.8 Configuration Example

```toml
[scheduler]
realtime_policy = "best_effort"
maintenance_budget_percent = 10

[queues.sensor]
capacity = 64
policy = "drop_oldest"

[queues.action]
capacity = 32
policy = "reject"

[queues.experience]
capacity = 512
policy = "drop_low_priority"
```

---

# SD-13 — SIMD & Platform Abstraction

## 13.1 Scope

Dokumen ini mengatur abstraction SIMD, feature detection, scalar fallback, dan platform Linux/ARM64, Android/ARM64, x86-64.

Traceability:

```text
AC §53 SIMD Contract
AC §54 SIMD Workloads
AC §63 GPU Contract
AC §75 Hardware Target
```

## 13.2 Terminology

| Istilah | Arti |
|---|---|
| SIMD backend | Implementasi vector. |
| Scalar fallback | Jalur CPU biasa. |
| Feature detection | Deteksi CPU capability. |
| Platform profile | Target OS/arch. |

## 13.3 Normative Requirements

1. ANR WAJIB berjalan tanpa GPU.
2. Scalar fallback WAJIB selalu tersedia.
3. SIMD backend BOLEH NEON, AVX2, AVX-512.
4. Feature detection WAJIB saat startup.
5. SIMD TIDAK BOLEH mengubah hasil semantic di luar toleransi numerik.
6. Platform minimum WAJIB ARM64 dan x86-64 Linux.
7. Android/ARM64 BOLEH didukung sebagai platform embedded Linux-like, tanpa dependency Google/cloud.

## 13.4 Interface / Schema

```rust
enum SimdBackend {
    Scalar,
    Neon,
    Avx2,
    Avx512,
}

trait SimdKernel {
    fn backend(&self) -> SimdBackend;
    fn activate(&self, input: &[f32], output: &mut [f32]);
    fn dot(&self, a: &[f32], b: &[f32]) -> f32;
    fn weighted_accumulate(&self, weights: &[f32], activations: &[f32], out: &mut [f32]);
}
```

Platform profiles:

```text
linux-arm64
android-arm64
linux-x86_64
```

Android note:

```text
Android support harus tetap offline-first.
Tidak boleh bergantung pada Google Play Services.
Deployment root dapat direalisasikan melalui private app directory atau bind mount, tetapi artifact neural tetap hanya brain.anr.
```

## 13.5 Invariants

```text
scalar fallback always works.
gpu optional never mandatory.
simd selection cannot break determinism mode.
```

## 13.6 Failure Behavior

| Failure | Behavior |
|---|---|
| SIMD unsupported | Fallback scalar. |
| SIMD kernel mismatch | Disable backend tersebut. |
| Platform unsupported | Reject build/target dengan error. |

## 13.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Scalar kernel.
2. NEON kernel.
3. AVX2 kernel.
4. AVX-512 jika tersedia.
5. Feature detection.
6. Backend fallback.
7. Numerical tolerance.
8. Platform cross-build.

## 13.8 Configuration Example

```toml
[simd]
backend = "auto"      # auto | scalar | neon | avx2 | avx512
tolerance = 1e-5
allow_runtime_switch = true

[platform]
target_profile = "linux-arm64"
android_deployment_root = "/data/local/opt/anr"
```

---

# SD-14 — Storage Transaction, Checksum/Integrity, Security/Trust Boundary

## 14.1 Scope

Dokumen ini mengatur transactional write, checksum, integrity validation, security boundary, dan trust model.

Traceability:

```text
AC §44 Storage Validation
AC §45 Transactional Write
AC §46 Recovery
AC §64 Security Boundary
AC §72 Brain Data Security
```

## 14.2 Terminology

| Istilah | Arti |
|---|---|
| Commit | Pengesahan generation baru. |
| Checksum | Nilai integritas. |
| Trust boundary | Batas data terpercaya. |
| Untrusted input | Input yang harus divalidasi. |

## 14.3 Normative Requirements

1. Setiap write generation WAJIB transactional.
2. Commit hanya setelah checksum valid.
3. Power loss WAJIB fallback ke generation valid.
4. `brain.anr` adalah data, bukan executable.
5. Runtime TIDAK BOLEH mengeksekusi isi brain.
6. Plugin dan sensor input WAJIB untrusted.
7. File permission default untuk brain SEBAIKNYA `0600`.
8. Secure boot/OS-level hardening BOLEH digunakan, tetapi tidak wajib.

## 14.4 Interface / Schema

Transaction descriptor:

```rust
struct TransactionDescriptor {
    generation: u64,
    parent_generation: u64,
    started_at: u64,
    committed_at: Option<u64>,
    checksum: [u8; 32],
    state: TxState,
}
```

Trust boundary:

```text
untrusted:
  brain.anr dari sumber luar
  plugin input
  sensor frames
  config file

trusted after validation:
  parsed neural structures
  validated runtime config
  internal scheduler messages
```

## 14.5 Invariants

```text
no commit without checksum.
no execution from brain data.
no trust before validation.
no silent corruption acceptance.
```

## 14.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Checksum mismatch | Reject generation. |
| Torn write | Recovery generation lama. |
| Untrusted payload | Reject/sanitize. |
| Permission error | Fail safe dengan error. |

## 14.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Transaction commit.
2. Power loss rollback.
3. Checksum mismatch.
4. Corrupt brain corpus.
5. Untrusted seed.
6. Plugin malicious input.
7. File permission check.
8. No-exec brain policy.

## 14.8 Configuration Example

```toml
[storage.security]
file_mode = "0600"
reject_world_writable = true
validate_plugin_payloads = true
zeroize_temp_buffers = true
```

---

# SD-15 — Diagnostics / Telemetry / Configuration / CLI

## 15.1 Scope

Dokumen ini mengatur CLI, diagnostics, telemetry lokal, dan configuration.

Traceability:

```text
AC §67 CLI Contract
AC §68 Brain Provisioning Workflow
AC §70 Diagnostics Contract
AC §71 Telemetry Contract
```

## 15.2 Terminology

| Istilah | Arti |
|---|---|
| CLI | Command line interface. |
| Telemetry | Metrics lokal. |
| Config | Konfigurasi runtime opsional. |
| Diagnostics | Status/inspeksi sistem. |

## 15.3 Normative Requirements

1. Semua CLI wajib berada dalam binary `anr`.
2. Telemetry WAJIB lokal, tidak cloud.
3. Config bersifat opsional; jika tidak ada, runtime menggunakan default aman.
4. `anr status` WAJIB menyediakan informasi minimal sesuai AC §70.
5. Output JSON WAJIB didukung untuk diagnostics.
6. Config TIDAK BOLEH menyimpan neural state.
7. Config TIDAK BOLEH menjadi artifact wajib deployment.

## 15.4 Interface / Schema

CLI minimum:

```text
anr run
anr status
anr memory
anr inspect
anr learn
anr consolidate

anr brain init
anr brain build
anr brain verify
anr brain inspect
anr brain install
```

Status JSON minimum:

```json
{
  "runtime_state": "running",
  "cpu": "aarch64",
  "simd_backend": "neon",
  "ram_used_bytes": 123456789,
  "storage_used_bytes": 987654321,
  "brain_generation": 42,
  "cortex_usage_bytes": 1000000,
  "cerebellum_usage_bytes": 900000,
  "hippocampus_usage_bytes": 5000000,
  "episode_rate": 12.5,
  "gc_rate": 0.2,
  "sensor_status": "ok",
  "camera_status": "degraded",
  "plugin_status": "ok"
}
```

## 15.5 Invariants

```text
single binary cli.
local telemetry only.
config is optional.
no neural state in config.
```

## 15.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Config invalid | Gunakan default aman atau fail explicit. |
| CLI command gagal | Exit code non-zero dan pesan error. |
| Telemetry penuh | Rotate/drop oldest. |

## 15.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Semua CLI command.
2. `anr status --json`.
3. Config default.
4. Config invalid.
5. Telemetry lokal.
6. Brain inspect/verify.
7. Exit codes.
8. No network telemetry.

## 15.8 Configuration Example

```toml
[cli]
json_output = false
verbose = false

[telemetry]
enabled = true
local_path = "/opt/anr/var/telemetry"
max_size_bytes = 10485760
rotate_policy = "rotate_oldest"

[config]
path = "/opt/anr/anr.toml"
optional = true
```

---

# SD-16 — Error / Failure Taxonomy & Deterministic Mode

## 16.1 Scope

Dokumen ini mengatur klasifikasi error/failure dan deterministic mode.

Traceability:

```text
AC §21 Degraded Mode
AC §74 Determinism
AC §20 Shutdown
AC §31 Safety
```

## 16.2 Terminology

| Istilah | Arti |
|---|---|
| Error class | Kategori error. |
| Severity | Tingkat dampak. |
| Deterministic mode | Mode reproducible. |
| Failure domain | Area kegagalan. |

## 16.3 Normative Requirements

1. Error WAJIB diklasifikasikan.
2. Severity WAJIB memiliki:
   - info,
   - recoverable,
   - degraded,
   - fatal,
   - emergency.
3. Deterministic mode WAJIB tersedia.
4. Deterministic mode WAJIB menggunakan seed tetap dan clock mock.
5. Error TIDAK BOLEH menyembunyikan safety failure.
6. Panic pada safety path harus menghasilkan safe state jika memungkinkan.

## 16.4 Interface / Schema

Error taxonomy:

```text
ANR-E-CONFIG
ANR-E-STORAGE
ANR-E-BRAIN
ANR-E-VALIDATION
ANR-E-MEMORY
ANR-E-NEURAL
ANR-E-LEARNING
ANR-E-PERCEPTION
ANR-E-PLUGIN
ANR-E-HAL
ANR-E-ACTUATOR
ANR-E-SAFETY
ANR-E-INTERNAL
```

Severity:

```rust
enum Severity {
    Info,
    Recoverable,
    Degraded,
    Fatal,
    Emergency,
}
```

Deterministic config:

```rust
struct DeterministicConfig {
    seed: u64,
    fixed_clock: bool,
    replay_input_log: bool,
    disable_adaptive_random: bool,
    floating_point_policy: FpPolicy,
}
```

## 16.5 Invariants

```text
safety errors cannot be downgraded.
deterministic mode reproducible.
fatal storage errors cannot enter Running.
```

## 16.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Recoverable error | Retry/fallback terbatas. |
| Degraded error | Masuk degraded mode. |
| Fatal error | Stop aman. |
| Emergency error | Emergency stop. |

## 16.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Error classification.
2. Severity mapping.
3. Deterministic replay.
4. Fixed seed reproducibility.
5. Panic handling.
6. Error logging.
7. Safety error priority.

## 16.8 Configuration Example

```toml
[error]
panic_safe_state = true
log_level = "info"

[deterministic]
enabled = false
seed = 0xANR0001
fixed_clock = true
replay_input_log = true
```

---

# SD-17 — Performance / Resource Budget

## 17.1 Scope

Dokumen ini menetapkan budget performance dan resource agar runtime tetap sesuai embedded/edge target.

Traceability:

```text
AC §77 Performance Invariants
AC §60 RAM Strategy
AC §56 Maintenance Budget
AC §75 Hardware Target
```

## 17.2 Terminology

| Istilah | Arti |
|---|---|
| Control latency | Latensi decision/safety/actuator. |
| Budget | Batas resource/waktu. |
| Soak | Uji durasi panjang. |
| Quota | Batas memori. |

## 17.3 Normative Requirements

1. Safety dan control latency WAJIB lebih prioritas daripada learning.
2. Maintenance WAJIB memiliki budget.
3. Memory usage WAJIB bounded.
4. Control loop TIDAK BOLEH menunggu GC/learning tanpa batas.
5. Performance regression pada control path WAJIB menjadi CI gate.
6. Budget default harus sesuai minimum hardware target.

## 17.4 Interface / Schema

Default budget profile:

```text
control_tick_hz = 100
control_budget_ms = 8
safety_eval_budget_ms = 1
perception_budget_ms = 10
experience_record_budget_ms = 2
learning_budget_percent = 10
maintenance_budget_percent = 10
gc_pause_budget_ms = 5
```

Minimum hardware profile:

```text
CPU: 2 cores ARM64
RAM: 512 MB
Storage: 1 GB
SIMD: NEON/scalar
GPU: none
```

## 17.5 Invariants

```text
control path latency > learning throughput.
memory boundedness > storage efficiency.
safety response > convenience.
```

## 17.6 Failure Behavior

| Failure | Behavior |
|---|---|
| Control overrun | Shed background work. |
| Memory over quota | GC/pressure policy. |
| GC overrun | Incremental GC, emergency GC jika perlu. |
| Performance regression | CI gagal. |

## 17.7 Validation / Test Requirements

WAJIB ada test untuk:

1. Control latency p95/p99.
2. GC pause.
3. Replay overhead.
4. Memory quota.
5. Perception throughput.
6. Storage throughput.
7. Soak test.
8. Resource limit profile.

## 17.8 Configuration Example

```toml
[performance]
control_tick_hz = 100
control_budget_ms = 8
safety_eval_budget_ms = 1
maintenance_budget_percent = 10
gc_pause_budget_ms = 5

[resources]
ram_target_profile = "embedded-512mb"
storage_cache_mode = "hot_only"
```

---

# SD-18 — Testing / CI / GitHub Actions / Hardware CI / Fault Injection / Endurance / Conformance Levels / Product Ready Criteria

## 18.1 Scope

Dokumen ini mengatur testing, CI, GitHub Actions, hardware CI, fault injection, endurance testing, conformance levels, dan Product Ready criteria.

Traceability:

```text
AC §72 Testing
AC §73 Benchmark
AC §74 Determinism
AC §75 Diagnostics
AC §80 Verification Contract
ANR Tests-CI Contract v1.0
```

## 18.2 Terminology

| Istilah | Arti |
|---|---|
| CI gate | Gerbang otomatis kualitas. |
| Hardware CI | Pengujian pada target nyata/emulasi. |
| Endurance | Uji durasi panjang. |
| Conformance Level | Tingkat kepatuhan produk. |
| Product Ready | Kesiapan release. |

## 18.3 Normative Requirements

1. CI WAJIB menjalankan:
   - fmt,
   - clippy,
   - build,
   - unit tests,
   - integration tests,
   - E2E tests,
   - fault injection,
   - coverage,
   - benchmark,
   - security audit,
   - conformance report.
2. Minimum test count WAJIB mengikuti Tests-CI Contract:
   - 840 unit tests,
   - 200 integration tests,
   - 120 E2E tests,
   - 80 fault tests,
   - 60 performance tests,
   - 100 conformance tests,
   - 40 security tests.
3. CI WAJIB offline pada tahap runtime test.
4. Hardware CI BOLEH menggunakan device nyata atau emulator/QEMU.
5. Endurance test WAJIB ada untuk release validation.
6. Product Ready TIDAK BOLEH dinyatakan jika safety/storage test gagal.

## 18.4 Interface / Schema

GitHub Actions job minimum:

```yaml
name: anr-ci

on:
  push:
  pull_request:

jobs:
  contract:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check test catalog
        run: ./scripts/check_test_catalog.sh

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo fmt --check

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo clippy --all-targets --all-features -- -D warnings

  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo build --release

  unit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --lib -- --test-threads=4

  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --test integration

  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/run_e2e.sh

  fault:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/run_fault_injection.sh

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/run_coverage.sh

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/run_benchmark_gate.sh

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo audit
      - run: cargo deny check advisories bans licenses

  conformance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/generate_conformance_report.sh
```

Hardware CI matrix minimum:

```text
linux-x86_64
linux-arm64
android-arm64 via emulator/device
qemu-aarch64 fallback
```

Endurance minimum:

```text
4 jam nightly
8 jam release candidate
```

Conformance levels:

```text
Level 0: Prototype
Level 1: Architecture Conforming
Level 2: Product Ready
```

Level 1 WAJIB:

```text
semua mandatory test pass
coverage threshold pass
single binary pass
single brain pass
offline pass
safety pass
storage recovery pass
```

Level 2 WAJIB:

```text
Level 1 terpenuhi
endurance pass
hardware target pass
performance target pass
security audit pass
fault injection pass
documentation pass
release artifact signed/verifiable
```

## 18.5 Invariants

```text
no release without passing required CI.
no retry-to-pass for failing required tests.
no product ready with safety failure.
no product ready with storage integrity failure.
no product ready with cloud dependency.
```

## 18.6 Failure Behavior

| Failure | Behavior |
|---|---|
| CI gate gagal | Merge/release diblokir. |
| Flaky test | Quarantine dan issue. |
| Hardware unavailable | Gunakan emulator/QEMU fallback. |
| Endurance gagal | Release tidak Product Ready. |

## 18.7 Validation / Test Requirements

WAJIB ada:

1. CI workflow test.
2. Test catalog count check.
3. Coverage gate.
4. Benchmark gate.
5. Security audit.
6. E2E binary smoke.
7. Fault injection suite.
8. Endurance suite.
9. Hardware/emulator matrix.
10. Conformance report generator.

## 18.8 Configuration Example

```toml
[ci]
required_unit_tests = 840
required_integration_tests = 200
required_e2e_tests = 120
required_fault_tests = 80
required_performance_tests = 60
required_conformance_tests = 100
required_security_tests = 40
fail_on_flaky = true
offline_runtime_tests = true

[hardware_ci]
use_qemu_arm64 = true
use_android_emulator = true
physical_devices = []

[endurance]
nightly_hours = 4
release_candidate_hours = 8
```

---

# 2. Global Traceability Matrix

| Supporting Document | Architecture Contract Reference |
|---|---|
| SD-01 Runtime State Machine | AC §18–21, §31 |
| SD-02 Repository/Module Boundary | AC §64–67 |
| SD-03 brain.anr Binary Format | AC §5, §44–48 |
| SD-04 Versioning/Recovery | AC §48–51, §70, §85 |
| SD-05 Brain Seed/Provisioning | AC §16–22 |
| SD-06 Neural Core/SoA | AC §11–17 |
| SD-07 Memory/Allocator | AC §10, §45, §59–60 |
| SD-08 Learning/Replay/GC | AC §33–42 |
| SD-09 Perception/Sensor/Camera/Audio | AC §22–27 |
| SD-10 Plugin/HAL | AC §28–29 |
| SD-11 Decision/Action/Safety | AC §30–32, §77 |
| SD-12 Scheduler/Queue | AC §55–58 |
| SD-13 SIMD/Platform | AC §53–54, §63, §75 |
| SD-14 Storage/Security | AC §44–46, §64, §72 |
| SD-15 CLI/Diagnostics/Config | AC §67–71 |
| SD-16 Error/Deterministic | AC §20–21, §74 |
| SD-17 Performance/Resource | AC §56, §60, §75, §77 |
| SD-18 Testing/CI/Product Ready | AC §72–75, §80 |

---

# 3. Global Invariants

Seluruh dokumen pendukung WAJIB mempertahankan invariant berikut:

```text
Rust implementation.
Single binary anr.
Single persistent memory brain.anr.
Cortex/Cerebellum/Hippocampus adalah logical sections.
Tidak ada .cx/.cm/.hs sebagai artifact persistent.
Offline-first.
No mandatory cloud.
No mandatory GPU.
Non-Transformer core.
Cell → Column → Block.
Synapse sebagai koneksi.
Safety di atas learning.
Bounded memory.
Bounded queues.
Transactional brain write.
Checksum/integrity validation.
Crash recovery.
SIMD + scalar fallback.
SoA hot layout.
Plugin/HAL isolation.
Deterministic control path.
Traceable conformance.
```

---

# 4. Product Ready Criteria

Sebuah implementasi ANR dinyatakan Product Ready hanya jika:

```text
1. Architecture Contract v1.1 terpenuhi.
2. Seluruh Supporting Documents ini terpenuhi.
3. Tests-CI Contract terpenuhi.
4. Minimum 840 unit tests pass.
5. Minimum 200 integration tests pass.
6. Minimum 120 E2E tests pass.
7. Minimum 80 fault injection tests pass.
8. Minimum 60 performance tests pass.
9. Minimum 100 conformance tests pass.
10. Minimum 40 security tests pass.
11. Coverage dan mutation threshold terpenuhi.
12. Safety layer tidak dapat di-bypass.
13. brain.anr transactional/recovery terbukti.
14. Offline-first terbukti.
15. Single-binary deployment terbukti.
16. No mandatory GPU/cloud/Transformer terbukti.
17. Endurance test lulus.
18. Hardware target/emulation matrix lulus.
19. Benchmark regression gate lulus.
20. Release artifact memiliki checksum dan conformance report.
```

---

# 5. Penutup

Dokumen pendukung ini membentuk satu kesatuan Master Technical Specification dengan Architecture Contract v1.1.

Dokumen ini:

1. Tidak mengubah invariant arsitektur.
2. Tidak menambah artifact persistent baru.
3. Tidak memasukkan roadmap atau jadwal.
4. Menyediakan detail teknis yang cukup untuk implementasi, testing, CI, deployment, dan product validation.
5. Menjaga ANR tetap:

```text
single binary
single brain.anr
offline-first
non-Transformer core
safety-first
bounded-resource
embedded-ready
```
