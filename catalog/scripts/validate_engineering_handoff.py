#!/usr/bin/env python3
"""Validate an atc-engineering evidence manifest against the canonical ecosystem census."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CATALOG = ROOT / "catalog"
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
ENGINEERING_REPO = "A-TownChain-Okosystems/atc-engineering"
SCHEMA_VERSION = "atc.build-evidence/v1"
NA_PUSH = "N/A (push event: no PR-head context)"


def fail(message: str) -> None:
    raise ValueError(f"BLOCKED: {message}")


def read_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        fail(f"cannot read {path}: {exc}")


def validate_manifest(manifest: dict, census_path: Path, expected_sha: str | None = None) -> dict:
    if not isinstance(manifest, dict):
        fail("evidence manifest must be a JSON object")
    required = {
        "schema_version", "repository", "source_sha", "checked_out_sha",
        "event_name", "event_ref", "pr_head_sha", "release_binary_path",
        "release_binary_sha256", "workflow_ref", "run_id", "run_attempt",
        "gate_report_path", "gate_report_sha256", "gate_report_schema",
    }
    missing = sorted(required - manifest.keys())
    if missing:
        fail(f"evidence manifest is missing required fields: {', '.join(missing)}")
    if manifest["schema_version"] != SCHEMA_VERSION:
        fail("unsupported evidence manifest schema")
    if manifest["gate_report_schema"] != "atc.gate-report/v1":
        fail("unsupported gate report schema")
    if manifest["repository"] != ENGINEERING_REPO:
        fail("manifest repository is not the canonical atc-engineering repository")
    for field in ("source_sha", "checked_out_sha"):
        if not isinstance(manifest[field], str) or not SHA40.fullmatch(manifest[field]):
            fail(f"{field} must be a full 40-character lowercase Git SHA")
    if manifest["source_sha"] != manifest["checked_out_sha"]:
        fail("source_sha and checked_out_sha differ")
    if expected_sha is not None:
        if not SHA40.fullmatch(expected_sha) or manifest["source_sha"] != expected_sha:
            fail("manifest SHA does not match the explicitly expected head SHA")
    for field in ("release_binary_sha256", "gate_report_sha256"):
        if not isinstance(manifest[field], str) or not SHA256.fullmatch(manifest[field]):
            fail(f"{field} must be a full 64-character SHA-256 digest")
    for field in ("run_id", "run_attempt"):
        if isinstance(manifest[field], bool) or not isinstance(manifest[field], int) or manifest[field] < 1:
            fail(f"{field} must be a positive integer")
    for field in ("event_name", "event_ref", "release_binary_path", "workflow_ref", "gate_report_path"):
        if not isinstance(manifest[field], str) or not manifest[field]:
            fail(f"{field} must be a non-empty string")
    if manifest["event_name"] == "push":
        if not manifest["event_ref"].startswith("refs/heads/") or manifest["event_ref"] == "refs/heads/":
            fail("push event must identify a branch ref")
        if manifest["pr_head_sha"] != NA_PUSH:
            fail("push event requires explicit N/A PR-head context")
    elif manifest["event_name"] == "pull_request":
        if not SHA40.fullmatch(manifest["pr_head_sha"]):
            fail("pull_request event requires a full PR-head SHA")
    elif not manifest["pr_head_sha"].startswith("N/A ("):
        fail("event without PR context requires explicit N/A PR-head context")

    census = read_json(census_path)
    repos = census.get("repositories")
    if not isinstance(repos, list):
        fail("canonical repository census has no repositories array")
    matches = [r for r in repos if r.get("repository") == ENGINEERING_REPO]
    if len(matches) != 1:
        fail("canonical census must contain exactly one atc-engineering record")
    census_sha = matches[0].get("head_commit_sha")
    if not isinstance(census_sha, str) or not SHA40.fullmatch(census_sha):
        fail("canonical census atc-engineering head SHA is missing or malformed")
    if manifest["source_sha"] != census_sha:
        fail(f"stale evidence: manifest SHA {manifest['source_sha']} != census head {census_sha}")
    return {
        "status": "PASS_SCHEMA_AND_CENSUS_BINDING",
        "repository": ENGINEERING_REPO,
        "source_sha": manifest["source_sha"],
        "run_id": manifest["run_id"],
        "run_attempt": manifest["run_attempt"],
        "functional_verification": "NOT_INFERRED",
        "note": "Manifest structure and census binding validated; artifact bytes and Actions logs must be verified separately."
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--census", type=Path, default=CATALOG / "repositories.json")
    parser.add_argument("--expected-sha")
    args = parser.parse_args()
    try:
        result = validate_manifest(read_json(args.manifest), args.census, args.expected_sha)
    except ValueError as exc:
        print(str(exc), file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
