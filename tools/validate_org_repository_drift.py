#!/usr/bin/env python3
"""Compare the manifest's historical main HEAD/tree pins with live GitHub refs.

The validator is fail-closed: any API/shape error or any SHA drift makes the
overall result BLOCKED. It never rewrites the historical pins automatically.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

SHA_RE = re.compile(r"^[0-9a-f]{40}$")
API_ROOT = "https://api.github.com"


def fetch_live_commit(owner: str, name: str, token: str | None) -> dict:
    url = f"{API_ROOT}/repos/{owner}/{name}/commits/main"
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "ATC-org-repository-drift-validator",
    }
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = urllib.request.Request(url, headers=headers, method="GET")
    with urllib.request.urlopen(request, timeout=20) as response:
        if response.status != 200:
            raise RuntimeError(f"GitHub API returned HTTP {response.status}")
        payload = json.loads(response.read().decode("utf-8"))
    commit_sha = payload.get("sha")
    tree_sha = (payload.get("commit") or {}).get("tree", {}).get("sha")
    if not isinstance(commit_sha, str) or not SHA_RE.fullmatch(commit_sha):
        raise ValueError("API response missing a valid full commit SHA")
    if not isinstance(tree_sha, str) or not SHA_RE.fullmatch(tree_sha):
        raise ValueError("API response missing a valid full root-tree SHA")
    return {"head_sha": commit_sha, "root_tree_sha": tree_sha}


def validate(manifest: dict, token: str | None, fetcher=fetch_live_commit) -> dict:
    owner = manifest.get("owner")
    repositories = manifest.get("repositories")
    if not isinstance(owner, str) or not owner or not isinstance(repositories, list) or not repositories:
        raise ValueError("manifest must contain owner and a non-empty repositories array")
    names = [item.get("name") for item in repositories if isinstance(item, dict)]
    if len(names) != len(repositories) or len(set(names)) != len(names):
        raise ValueError("manifest repository entries must be objects with unique names")

    entries = []
    for item in sorted(repositories, key=lambda row: row["name"].lower()):
        name = item["name"]
        entry = {
            "repository": f"{owner}/{name}",
            "ref": "main",
            "archived": bool(item.get("archived", False)),
            "expected_head_sha": item.get("observed_head_sha"),
            "expected_root_tree_sha": item.get("observed_root_tree_sha"),
            "live_head_sha": None,
            "live_root_tree_sha": None,
            "status": "BLOCKED",
            "error": None,
        }
        if item.get("drift_check_excluded") is True:
            reason = item.get("drift_check_exclusion_reason")
            if not isinstance(reason, str) or not reason.strip():
                entry["error"] = "EXCLUSION_REQUIRES_NONEMPTY_REASON"
            else:
                entry["status"] = "EXCLUDED"
                entry["exclusion_reason"] = reason.strip()
            entries.append(entry)
            continue
        # Archived repositories are historical references, not active live sources.
        # Empty pins are therefore valid only for archived entries and never fetched.
        if entry["archived"]:
            entry["status"] = "EXCLUDED"
            entry["exclusion_reason"] = "Archived repository is reference-only; live drift validation is out of scope."
            entries.append(entry)
            continue
        expected_head = entry["expected_head_sha"]
        expected_tree = entry["expected_root_tree_sha"]
        if not isinstance(expected_head, str) or not SHA_RE.fullmatch(expected_head):
            entry["error"] = "INVALID_EXPECTED_HEAD_SHA"
            entries.append(entry)
            continue
        if not isinstance(expected_tree, str) or not SHA_RE.fullmatch(expected_tree):
            entry["error"] = "INVALID_EXPECTED_ROOT_TREE_SHA"
            entries.append(entry)
            continue
        try:
            live = fetcher(owner, name, token)
            entry["live_head_sha"] = live["head_sha"]
            entry["live_root_tree_sha"] = live["root_tree_sha"]
            if live["head_sha"] != expected_head or live["root_tree_sha"] != expected_tree:
                entry["status"] = "DRIFT"
                entry["error"] = "LIVE_MAIN_DIFFERS_FROM_HISTORICAL_SNAPSHOT"
            else:
                entry["status"] = "MATCH"
        except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, ValueError, RuntimeError, json.JSONDecodeError) as exc:
            # Error strings are bounded and never include request headers/tokens.
            entry["error"] = f"API_OR_RESPONSE_ERROR: {type(exc).__name__}: {str(exc)[:240]}"
        entries.append(entry)

    counts = {key: sum(entry["status"] == key for entry in entries) for key in ("MATCH", "DRIFT", "BLOCKED", "EXCLUDED")}
    in_scope_count = len(entries) - counts["EXCLUDED"]
    overall = "PASS_NO_DRIFT" if counts["MATCH"] == in_scope_count and counts["DRIFT"] == 0 and counts["BLOCKED"] == 0 else "BLOCKED"
    return {
        "schema_id": "ATC-ORG-REPOSITORY-DRIFT-REPORT-001",
        "schema_version": "1.0.0",
        "status": overall,
        "snapshot_status": "HISTORICAL_SNAPSHOT_COMPARED_TO_LIVE_MAIN",
        "owner": owner,
        "repository_count": len(entries),
        "in_scope_repository_count": in_scope_count,
        "summary": counts,
        "fail_closed": True,
        "pins_rewritten": False,
        "repositories": entries,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    try:
        manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
        token = os.environ.get("ATC_CENSUS_TOKEN") or os.environ.get("GITHUB_TOKEN")
        report = validate(manifest, token)
    except Exception as exc:
        report = {
            "schema_id": "ATC-ORG-REPOSITORY-DRIFT-REPORT-001",
            "schema_version": "1.0.0",
            "status": "BLOCKED",
            "fail_closed": True,
            "error": f"{type(exc).__name__}: {str(exc)[:240]}",
            "repositories": [],
        }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({key: report.get(key) for key in ("status", "repository_count", "in_scope_repository_count", "summary", "error", "fail_closed", "pins_rewritten")}, sort_keys=True))
    return 0 if report.get("status") == "PASS_NO_DRIFT" else 1


if __name__ == "__main__":
    sys.exit(main())
