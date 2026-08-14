#!/usr/bin/env python3
"""Filter fake/legacy tests from completed domains in tests/catalog.toml."""
import tomllib
from collections import defaultdict

WP1_DOMAINS = {
    "error-taxonomy",
    "config-load",
    "config-validation",
    "logging-tracing",
    "cli-commands",
}


def toml_value(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    return f'"{v}"'


def filter_catalog(catalog_path="tests/catalog.toml"):
    with open(catalog_path, "rb") as f:
        data = tomllib.load(f)

    real_counts = defaultdict(int)
    for entry in data.get("test", []):
        if entry.get("quality") == "real":
            real_counts[entry["domain"]] += 1

    filtered = []
    removed = 0
    for entry in data.get("test", []):
        domain = entry.get("domain", "")
        if domain in WP1_DOMAINS and real_counts.get(domain, 0) >= 12:
            if entry.get("quality") == "fake":
                removed += 1
                continue
        filtered.append(entry)

    data["test"] = filtered

    with open(catalog_path, "w") as f:
        for key in ["legacy", "quality"]:
            if key in data:
                f.write(f"{key} = {toml_value(data[key])}\n")
        for entry in filtered:
            f.write("\n[[test]]\n")
            for k, v in entry.items():
                f.write(f"{k} = {toml_value(v)}\n")

    print(f"Removed {removed} fake entries from WP-1 completed domains.")
    print(f"Remaining: {len(filtered)} entries.")


if __name__ == "__main__":
    filter_catalog()
