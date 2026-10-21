#!/usr/bin/env python3
"""Plan one safe build step from the canonical ecosystem catalog.

This tool does not edit implementation repositories, infer verification, or run
commands taken from catalog data. It selects one canonical repository with an
explicitly allowlisted toolchain/build profile. GitHub Actions performs the
actual build in an isolated job and stores the exact-SHA evidence as an artifact.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG_PATH = ROOT / "docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json"
MANIFEST_PATH = ROOT / "docs/build/ATC-BUILD-MANIFEST-001.json"
GRAPH_PATH = ROOT / "docs/architecture/ATC-DEPENDENCY-GRAPH-001.json"

# These are profile identifiers, not shell commands. The workflow maps each
# profile to fixed commands; arbitrary commands from the manifest are never run.
ALLOWED_PROFILES = {
    "cargo-workspace": {"toolchain": "cargo", "build": "cargo build --workspace", "test": "cargo test --workspace"},
    "python-pytest": {"toolchain": "python", "build": "python -m compileall -q .", "test": "python -m pytest -q"},
    "npm-build": {"toolchain": "npm", "build": "npm run build", "test": "npm test"},
}
EXCLUDED_NAMES = {".github", "demo-repository"}
HEX40 = re.compile(r"^[0-9a-f]{40}$")


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()

    catalog = load_json(CATALOG_PATH)
    manifest = load_json(MANIFEST_PATH)
    graph = load_json(GRAPH_PATH)

    if catalog.get("schema_id") != "ATC-COMPONENT-FUNCTION-CATALOG-001":
        raise ValueError("unexpected catalog schema_id")
    if manifest.get("schema_id") != "ATC-BUILD-MANIFEST-001":
        raise ValueError("unexpected build manifest schema_id")
    if not isinstance(manifest.get("repos"), list):
        raise ValueError("build manifest repos must be a list")
    if not isinstance(graph.get("nodes"), list):
        raise ValueError("dependency graph nodes must be a list")

    entries = {}
    for item in manifest["repos"]:
        if not isinstance(item, dict) or not isinstance(item.get("repo"), str):
            raise ValueError("every build manifest entry must have a repo name")
        name = item["repo"]
        if name in entries:
            raise ValueError(f"duplicate build manifest repo: {name}")
        entries[name] = item

    nodes = {node.get("id"): node for node in graph["nodes"] if isinstance(node, dict)}
    order = graph.get("topological_order", [])
    if not isinstance(order, list):
        raise ValueError("topological_order must be a list")

    report = {
        "schema_id": "ATC-ECOSYSTEM-BUILD-LOOP-REPORT-001",
        "schema_version": "1.0.0",
        "status": "PLANNED",
        "catalog_schema": catalog["schema_id"],
        "catalog_status": catalog.get("status", "UNKNOWN"),
        "manifest_status": manifest.get("status", "UNKNOWN"),
        "selected_repo": None,
        "profile": None,
        "reason": "validate-only" if args.validate_only else "no eligible build target",
        "selection_policy": "one canonical target per run; dependency order; no arbitrary manifest commands; no auto-merge",
    }

    if not args.validate_only:
        for name in order:
            if not isinstance(name, str) or name in EXCLUDED_NAMES:
                continue
            item = entries.get(name)
            node = nodes.get(name)
            if not item or not node:
                continue
            status = str(item.get("status", ""))
            if status in {"EXEMPT_ORG_CONFIG", "EXEMPT_FIXTURE", "CANONICAL_ELSEWHERE", "BLOCKED_NO_ENTRY"}:
                continue
            # Do not build a reference/quell repository as a second SSOT.
            if item.get("quellrepo_role") or item.get("canonical_note") and "quellrepo" in str(item.get("canonical_note")).lower():
                continue
            toolchain = item.get("toolchain")
            command = item.get("build_command", "")
            profile = None
            if toolchain == "cargo" and command in {"cargo build --workspace", "cargo build --release --workspace"}:
                profile = "cargo-workspace"
            elif toolchain == "python" and command in {"python -m pytest tools/ -q", "python -m pytest tests/ -q", "pip install -e ."}:
                profile = "python-pytest"
            elif toolchain == "npm" and command in {"npm ci && npm run build", "npm run build"}:
                profile = "npm-build"
            if profile:
                report.update({
                    "status": "READY",
                    "selected_repo": name,
                    "profile": profile,
                    "reason": "first eligible canonical target in dependency order",
                })
                break

    print(json.dumps(report, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, json.JSONDecodeError, ValueError) as exc:
        print(json.dumps({"status": "BLOCKED", "error": str(exc)}, indent=2), file=sys.stderr)
        raise SystemExit(2)
