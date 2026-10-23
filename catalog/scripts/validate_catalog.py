#!/usr/bin/env python3
"""Validate the machine-readable ecosystem file/function catalog."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[2]
CATALOG = ROOT / "catalog"
SHA40 = re.compile(r"^[0-9a-f]{40}$")


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"{path.relative_to(ROOT)} is not valid JSON: {exc}")


def main() -> None:
    manifest = read_json(CATALOG / "repositories.json")
    coverage = read_json(CATALOG / "coverage.json")
    schema = read_json(CATALOG / "schema" / "file-inventory.schema.json")
    if manifest.get("repository_count") != 33:
        fail("repository_count must be 33")
    repositories = manifest.get("repositories")
    if not isinstance(repositories, list) or len(repositories) != 33:
        fail("repositories must contain exactly 33 entries")
    ids = [item.get("repository") for item in repositories]
    if len(set(ids)) != len(ids):
        fail("repository identifiers are not unique")
    for item in repositories:
        for field in ("head_commit_sha", "tree_sha"):
            if not isinstance(item.get(field), str) or not SHA40.fullmatch(item[field]):
                fail(f"{item.get('repository')}: invalid {field}")
        if item.get("tree_truncated") is not False:
            fail(f"{item.get('repository')}: tree_truncated must be false")
        if not isinstance(item.get("tracked_file_count"), int) or item["tracked_file_count"] < 0:
            fail(f"{item.get('repository')}: invalid tracked_file_count")
    demo = next((item for item in repositories if item["repository"].endswith("/demo-repository")), None)
    if not demo or demo.get("exclusion_from_live_drift_gate") is not True:
        fail("demo-repository must be excluded from the live drift gate")
    if coverage.get("repository_count") != len(repositories):
        fail("coverage repository_count does not match manifest")
    if coverage.get("repository_snapshots_recorded") != len(repositories):
        fail("coverage snapshot count does not match manifest")
    total = sum(item["tracked_file_count"] for item in repositories)
    if coverage.get("totals", {}).get("tracked_files_all_repositories") != total:
        fail("coverage tracked file total does not match manifest")

    validator = Draft202012Validator(schema)
    jsonl_path = CATALOG / "files.jsonl"
    records = []
    for line_number, line in enumerate(jsonl_path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except Exception as exc:
            fail(f"files.jsonl:{line_number} is not valid JSON: {exc}")
        errors = sorted(validator.iter_errors(record), key=lambda err: list(err.path))
        if errors:
            fail(f"files.jsonl:{line_number}: {errors[0].message}")
        if not SHA40.fullmatch(record["commit_sha"] or ""):
            fail(f"files.jsonl:{line_number}: commit_sha must be a 40-character Git SHA")
        if not SHA40.fullmatch(record["blob_sha"] or ""):
            fail(f"files.jsonl:{line_number}: blob_sha must be a 40-character Git SHA")
        records.append(record)
    if coverage.get("file_level_records") != len(records):
        fail("coverage file_level_records does not match files.jsonl")
    print(json.dumps({
        "status": "PASS",
        "repository_count": len(repositories),
        "trees_untruncated": len(repositories),
        "tracked_file_count": total,
        "file_records_schema_validated": len(records),
        "schema_draft": "2020-12"
    }, indent=2))


if __name__ == "__main__":
    main()
