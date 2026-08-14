#!/usr/bin/env python3
"""Check tests/catalog.toml for legacy/fake/unfinished domain issues."""
import re
import sys
import tomllib
from collections import defaultdict


def read_progress_completed_domains(path="docs/PROGRESS.md"):
    domains = []
    try:
        with open(path, "r") as f:
            for line in f:
                m = re.search(r"completed_domains.*?\[([^\]]*)\]", line)
                if m:
                    text = m.group(1)
                    for part in text.split(","):
                        part = part.strip().strip("\"'")
                        if part:
                            domains.append(part)
    except FileNotFoundError:
        pass
    return domains


def check(catalog_path="tests/catalog.toml", progress_path="docs/PROGRESS.md", wp="0", enforce_global=False):
    with open(catalog_path, "rb") as f:
        data = tomllib.load(f)

    legacy = data.get("legacy", False)
    quality = data.get("quality", "unknown")

    completed = read_progress_completed_domains(progress_path)

    domains = defaultdict(list)
    for entry in data.get("test", []):
        dom = entry.get("domain", "")
        q = entry.get("quality", quality)
        domains[dom].append((entry.get("id", ""), q))

    errors = []
    for dom in completed:
        entries = domains.get(dom, [])
        real_count = sum(1 for _, q in entries if q == "real")
        fake_count = sum(1 for _, q in entries if q == "fake")
        if fake_count > 0:
            errors.append(f"DOMAIN {dom}: {fake_count} fake test(s) in completed domain")
        if real_count < 12:
            errors.append(f"DOMAIN {dom}: {real_count} real tests, minimum 12 required")

    if enforce_global or int(wp) >= 13:
        unit_real = 0
        total_real = 0
        for entry in data.get("test", []):
            q = entry.get("quality", quality)
            if q == "real":
                total_real += 1
                if entry.get("level") == "unit":
                    unit_real += 1
        if unit_real < 840:
            errors.append(f"GLOBAL: unit real tests = {unit_real}, minimum 840")
        if total_real < 1440:
            errors.append(f"GLOBAL: total real tests = {total_real}, minimum 1440")

    if errors:
        print("CHECK_TEST_CATALOG FAILED:")
        for e in errors:
            print(f"  - {e}")
        sys.exit(1)
    print(f"CHECK_TEST_CATALOG PASSED: {len(domains)} domains, {sum(len(v) for v in domains.values())} tests")
    if completed:
        print(f"  completed_domains = {completed}")
    else:
        print("  completed_domains = [] (no enforcement active)")


if __name__ == "__main__":
    wp = sys.argv[1] if len(sys.argv) > 1 else "0"
    enforce = "--enforce-global" in sys.argv
    check(wp=wp, enforce_global=enforce)
