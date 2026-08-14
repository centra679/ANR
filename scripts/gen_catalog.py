#!/usr/bin/env python3
"""Generate tests/catalog.toml from `cargo test -- --list` output."""
import subprocess
import re
import sys
from collections import defaultdict

LEVEL_MAP = {
    "tc_u": "unit",
    "tc_i": "integration",
    "tc_e2e": "e2e",
    "tc_fault": "fault-injection",
    "tc_perf": "performance",
    "tc_conform": "conformance",
    "tc_sec": "security",
}

PREFIX_RE = re.compile(
    r"^(tc_u|tc_i|tc_e2e|tc_fault|tc_perf|tc_conform|tc_sec)_(.+?)_(\d+)(?:_|$)"
)


def parse_tests():
    proc = subprocess.run(
        ["cargo", "test", "--", "--list"],
        capture_output=True,
        text=True,
        check=True,
    )
    output = proc.stdout + "\n" + proc.stderr
    tests = []
    seen = set()
    for line in output.splitlines():
        line = line.strip()
        if not line or line.startswith("Running ") or line.endswith(" tests"):
            continue
        if ": test" not in line:
            continue
        full_name = line.rsplit(": ", 1)[0].strip()
        # Deduplicate by actual test function name (last path component)
        test_name = full_name.split("::")[-1]
        if test_name in seen:
            continue
        seen.add(test_name)
        tests.append(full_name)
    return tests


def classify(name):
    # Strip module path: keep only the last component after ::
    test_name = name.split("::")[-1]
    m = PREFIX_RE.match(test_name)
    if not m:
        return None, None, None
    prefix, domain, seq = m.groups()
    level = LEVEL_MAP.get(prefix, "unknown")
    return level, domain, int(seq)


def generate(tests, out_path):
    domains = defaultdict(list)
    special = []
    for t in tests:
        level, domain, seq = classify(t)
        if level is None:
            special.append(t)
            continue
        domains[(level, domain)].append((seq, t))

    lines = [
        'legacy = true',
        'quality = "unknown"',
        "",
    ]

    for t in special:
        lines.extend([
            '[[test]]',
            f'id = "{t.upper()}"',
            'level = "admin"',
            'domain = "catalog-admin"',
            'requirement = "legacy"',
            'test_type = "unknown"',
            'status = "legacy"',
            'owner = "WP-13"',
            'criticality = "unknown"',
            "",
        ])

    prefix_map = {
        "unit": "U",
        "integration": "I",
        "e2e": "E",
        "fault-injection": "F",
        "performance": "P",
        "conformance": "C",
        "security": "S",
    }

    items = []
    for (level, domain), seqs in sorted(domains.items()):
        seqs.sort()
        for i, (seq, _) in enumerate(seqs, 1):
            lvl_prefix = prefix_map.get(level, "X")
            norm_id = f"TC-{lvl_prefix}-{domain.upper()}-{i:03d}"
            items.append((norm_id, level, domain, seq))

    for norm_id, level, domain, seq in items:
        lines.extend([
            '[[test]]',
            f'id = "{norm_id}"',
            f'level = "{level}"',
            f'domain = "{domain}"',
            'requirement = "legacy"',
            'test_type = "unknown"',
            'status = "legacy"',
            f'owner = "WP-{guess_wp(level, domain)}"',
            'criticality = "unknown"',
            "",
        ])

    with open(out_path, "w") as f:
        f.write("\n".join(lines).rstrip() + "\n")
    print(f"Wrote {len(items) + len(special)} entries to {out_path}")


def guess_wp(level, domain):
    d = domain.lower()
    if level == "unit":
        if any(k in d for k in ["core", "config", "error", "logging", "scheduler"]):
            return "1"
        if any(k in d for k in ["brain", "header", "offset", "checksum", "transaction", "recovery", "seed", "build", "verify"]):
            return "2"
        if any(k in d for k in ["memory", "allocator", "gc", "retention", "tier", "compression"]):
            return "4"
        if any(k in d for k in ["cell", "column", "block", "synapse", "sparse", "soa", "scalar", "simd", "neon", "avx", "fallback"]):
            return "5"
        if any(k in d for k in ["sensor", "camera", "audio", "perception", "fusion"]):
            return "7"
        if any(k in d for k in ["plugin", "hal", "mock"]):
            return "7"
        if any(k in d for k in ["decision", "safety", "actuator", "feedback"]):
            return "8"
        if any(k in d for k in ["hebbian", "temporal", "replay", "consolidation", "contradiction", "skill"]):
            return "10"
        if any(k in d for k in ["cli", "diagnostics", "telemetry", "security"]):
            return "12"
        if any(k in d for k in ["cortex", "cerebellum", "hippocampus", "install", "update"]):
            return "6"
        return "1"
    if level == "integration":
        return "13"
    if level == "e2e":
        return "9"
    if level == "fault-injection":
        return "13"
    if level == "performance":
        return "13"
    if level == "conformance":
        return "14"
    if level == "security":
        return "13"
    return "1"


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "tests/catalog.toml"
    tests = parse_tests()
    generate(tests, out)
