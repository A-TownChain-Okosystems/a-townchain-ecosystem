#!/usr/bin/env python3
"""Fail-closed reconciliation gate for the canonical organization census.

This gate validates provenance and status claims; it does not infer functional
correctness from source presence, symbol discovery, or a green workflow.
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CATALOG = ROOT / "catalog"
SHA40 = re.compile(r"^[0-9a-f]{40}$")
ELEVATED = {"IMPLEMENTED", "TESTED", "CI_VERIFIED", "E2E_VERIFIED", "VERIFIED"}


def fail(message: str) -> None:
    print(f"BLOCKED: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")


def main() -> int:
    manifest = load_json(CATALOG / "repositories.json")
    repos = manifest.get("repositories")
    if not isinstance(repos, list) or not repos:
        fail("repository manifest must contain a non-empty repositories array")

    by_repo = {}
    for repo in repos:
        name = repo.get("repository")
        if not isinstance(name, str) or name in by_repo:
            fail(f"missing or duplicate repository identifier: {name!r}")
        for field in ("head_commit_sha", "tree_sha"):
            value = repo.get(field)
            if not isinstance(value, str) or not SHA40.fullmatch(value):
                fail(f"{name}: invalid {field}")
        if repo.get("tree_truncated") is not False:
            fail(f"{name}: tree snapshot is truncated or unconfirmed")
        if name.endswith("/demo-repository") and repo.get("exclusion_from_live_drift_gate") is not True:
            fail("demo-repository must be explicitly excluded from the live drift gate")
        if repo.get("archived") is True and repo.get("inventory_status") not in {
            "TREE_VERIFIED", "ARCHIVED_REFERENCE_ONLY", "EXCLUDED"
        }:
            fail(f"{name}: archived repository lacks an explicit historical classification")
        by_repo[name] = repo

    seen = set()
    counts = Counter()
    records_path = CATALOG / "files.jsonl"
    try:
        lines = records_path.read_text(encoding="utf-8").splitlines()
    except OSError as exc:
        fail(f"cannot read catalog/files.jsonl: {exc}")

    for line_no, line in enumerate(lines, 1):
        if not line.strip():
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError as exc:
            fail(f"files.jsonl:{line_no}: invalid JSON: {exc}")
        repo_name = rec.get("repository")
        if repo_name not in by_repo:
            fail(f"files.jsonl:{line_no}: repository is absent from manifest: {repo_name!r}")
        path, commit_sha, blob_sha = rec.get("path"), rec.get("commit_sha"), rec.get("blob_sha")
        if not isinstance(path, str) or not path or path.startswith("/") or ".." in Path(path).parts:
            fail(f"files.jsonl:{line_no}: invalid repository-relative path")
        for field, value in (("commit_sha", commit_sha), ("blob_sha", blob_sha)):
            if not isinstance(value, str) or not SHA40.fullmatch(value):
                fail(f"files.jsonl:{line_no}: invalid {field}")
        key = (repo_name, commit_sha, path)
        if key in seen:
            fail(f"files.jsonl:{line_no}: duplicate record key {key}")
        seen.add(key)
        counts[repo_name] += 1

        expected = by_repo[repo_name]["head_commit_sha"]
        if commit_sha != expected:
            fail(
                f"files.jsonl:{line_no}: stale source binding for {repo_name}:{path}; "
                f"record={commit_sha}, manifest_head={expected}"
            )

        status = rec.get("status", "UNKNOWN")
        if status in ELEVATED:
            verification = rec.get("verification") or {}
            tests = rec.get("tests") or []
            if status in {"TESTED", "CI_VERIFIED", "E2E_VERIFIED", "VERIFIED"}:
                if not tests or not all(t.get("status") == "PASS" for t in tests):
                    fail(f"files.jsonl:{line_no}: {status} requires explicit passing test records")
            if status in {"CI_VERIFIED", "E2E_VERIFIED", "VERIFIED"}:
                if not any(
                    t.get("status") == "PASS"
                    and t.get("run_url")
                    and t.get("commit_sha") == commit_sha
                    for t in tests
                ):
                    fail(f"files.jsonl:{line_no}: {status} requires run URL and exact tested commit SHA")
            if status in {"E2E_VERIFIED", "VERIFIED"} and verification.get("call_graph_complete") is not True:
                fail(f"files.jsonl:{line_no}: {status} requires complete call-graph evidence")
            if status == "VERIFIED" and verification.get("tests_verified") is not True:
                fail(f"files.jsonl:{line_no}: VERIFIED requires tests_verified=true")

    coverage = load_json(CATALOG / "coverage.json")
    declared = coverage.get("file_level_records")
    if declared != len(seen):
        fail(f"coverage file_level_records={declared!r} but actual records={len(seen)}")

    print(json.dumps({
        "status": "PASS_STRUCTURAL_AND_PROVENANCE",
        "repositories": len(by_repo),
        "file_records": len(seen),
        "repositories_with_file_records": len(counts),
        "functional_verification": "NOT_INFERRED",
        "limitations": [
            "Passing this gate establishes structural and provenance consistency only.",
            "Source mappings, passing tests, CI evidence, E2E evidence, and functional verification remain distinct claims."
        ]
    }, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
