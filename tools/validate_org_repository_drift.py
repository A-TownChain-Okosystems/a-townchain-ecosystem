#!/usr/bin/env python3
"""Compare manifest-scoped repository HEAD and root-tree SHAs with live GitHub."""
from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from validate_org_repository_drift_scope import resolve as resolve_scope  # noqa: E402

API_ROOT = "https://api.github.com"
SHA_LENGTH = 40


def github_json(url: str, token: str | None = None) -> dict:
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "ATC-Repository-Drift-Validator",
    }
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=20) as response:
        payload = json.loads(response.read().decode("utf-8"))
    if not isinstance(payload, dict):
        raise ValueError("GitHub API response must be a JSON object")
    return payload


def valid_sha(value: object) -> bool:
    return isinstance(value, str) and len(value) == SHA_LENGTH and all(c in "0123456789abcdef" for c in value)


def compare(manifest: dict, policy: dict, integration: dict, fetch=github_json, token: str | None = None) -> dict:
    errors, scope_report = resolve_scope(manifest, policy)
    if errors:
        return {
            "schema_id": "ATC-ORG-REPOSITORY-DRIFT-REPORT-001",
            "schema_version": "1.0.0",
            "validation": "BLOCKED",
            "reason_code": "INVALID_SCOPE_POLICY",
            "errors": errors,
            "entries": [],
        }

    inventory = integration.get("repositories")
    if not isinstance(inventory, list):
        return {
            "schema_id": "ATC-ORG-REPOSITORY-DRIFT-REPORT-001",
            "schema_version": "1.0.0",
            "validation": "BLOCKED",
            "reason_code": "INVALID_INTEGRATION_INVENTORY",
            "errors": ["integration repositories must be an array"],
            "entries": [],
        }
    by_name = {item.get("full_name", "").split("/", 1)[-1]: item for item in inventory if isinstance(item, dict)}
    entries = []
    for scope in scope_report["entries"]:
        name = scope["name"]
        decision = scope["decision"]
        entry = {
            "repository": f"{manifest.get('owner', '')}/{name}",
            "scope_decision": decision,
            "scope_reason_code": scope["reason_code"],
            "expected_default_branch": None,
            "observed_default_branch": None,
            "expected_head_sha": None,
            "observed_head_sha": None,
            "expected_root_tree_sha": None,
            "observed_root_tree_sha": None,
            "status": decision if decision == "EXCLUDE" else "BLOCKED",
            "reason_code": scope["reason_code"] if decision == "EXCLUDE" else "LIVE_CHECK_NOT_COMPLETED",
            "error": None,
        }
        if decision == "EXCLUDE":
            entry["status"] = "EXCLUDED"
            entry["reason_code"] = scope["reason_code"]
            entries.append(entry)
            continue
        if decision != "INCLUDE":
            entry["status"] = "BLOCKED"
            entry["reason_code"] = "SCOPE_BLOCKED"
            entry["error"] = "Scope resolver did not authorize inclusion"
            entries.append(entry)
            continue

        baseline = by_name.get(name)
        if baseline is None:
            entry["reason_code"] = "MISSING_BASELINE_REPOSITORY"
            entry["error"] = "No matching integration-inventory record"
            entries.append(entry)
            continue
        branch = baseline.get("default_branch")
        expected_head = baseline.get("observed_head_sha")
        expected_tree = baseline.get("observed_tree_sha")
        entry["expected_default_branch"] = branch
        entry["expected_head_sha"] = expected_head
        entry["expected_root_tree_sha"] = expected_tree
        if not isinstance(branch, str) or not branch.strip() or not valid_sha(expected_head) or not valid_sha(expected_tree):
            entry["reason_code"] = "INVALID_BASELINE_IDENTITY"
            entry["error"] = "Baseline default branch, full HEAD SHA, or root-tree SHA is missing/invalid"
            entries.append(entry)
            continue

        full_name = entry["repository"]
        encoded_branch = urllib.parse.quote(branch, safe="")
        try:
            repo_data = fetch(f"{API_ROOT}/repos/{full_name}", token)
            observed_branch = repo_data.get("default_branch")
            entry["observed_default_branch"] = observed_branch
            if not isinstance(observed_branch, str) or not observed_branch:
                raise ValueError("repository response has no default_branch")
            if repo_data.get("archived") is not scope.get("archived"):
                entry["status"] = "BLOCKED"
                entry["reason_code"] = "REPOSITORY_METADATA_MISMATCH"
                entry["error"] = "Live archived flag does not match manifest scope metadata"
                entries.append(entry)
                continue
            if isinstance(scope.get("private"), bool) and repo_data.get("private") is not scope.get("private"):
                entry["status"] = "BLOCKED"
                entry["reason_code"] = "REPOSITORY_METADATA_MISMATCH"
                entry["error"] = "Live private flag does not match explicit manifest metadata"
                entries.append(entry)
                continue
            commit_data = fetch(f"{API_ROOT}/repos/{full_name}/commits/{urllib.parse.quote(observed_branch, safe='')}", token)
            observed_head = commit_data.get("sha")
            observed_tree = (commit_data.get("commit") or {}).get("tree", {}).get("sha")
            entry["observed_head_sha"] = observed_head
            entry["observed_root_tree_sha"] = observed_tree
            if not valid_sha(observed_head) or not valid_sha(observed_tree):
                raise ValueError("GitHub response lacks valid full HEAD/root-tree SHA")
            if observed_branch != branch:
                entry["status"] = "DRIFT"
                entry["reason_code"] = "DEFAULT_BRANCH_CHANGED"
            elif observed_head != expected_head or observed_tree != expected_tree:
                entry["status"] = "DRIFT"
                entry["reason_code"] = "HEAD_OR_ROOT_TREE_CHANGED"
            else:
                entry["status"] = "MATCH"
                entry["reason_code"] = "EXACT_HEAD_AND_ROOT_TREE_MATCH"
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError, OSError, ValueError, json.JSONDecodeError) as exc:
            entry["status"] = "BLOCKED"
            entry["reason_code"] = "LIVE_GITHUB_QUERY_FAILED"
            entry["error"] = f"{type(exc).__name__}: {exc}"
        entries.append(entry)

    included = [e for e in entries if e["scope_decision"] == "INCLUDE"]
    blocked = sum(e["status"] == "BLOCKED" for e in included)
    drift = sum(e["status"] == "DRIFT" for e in included)
    matched = sum(e["status"] == "MATCH" for e in included)
    excluded = sum(e["status"] == "EXCLUDED" for e in entries)
    if blocked:
        validation = "BLOCKED"
    elif drift:
        validation = "DRIFT_DETECTED"
    elif matched == len(included):
        validation = "NO_DRIFT_OBSERVED"
    else:
        validation = "BLOCKED"
    return {
        "schema_id": "ATC-ORG-REPOSITORY-DRIFT-REPORT-001",
        "schema_version": "1.0.0",
        "validation": validation,
        "baseline_snapshot_date": integration.get("snapshot_date"),
        "repositories_total": len(entries),
        "repositories_included": len(included),
        "repositories_matched": matched,
        "repositories_drifted": drift,
        "repositories_blocked": blocked,
        "repositories_excluded": excluded,
        "entries": entries,
        "errors": [],
        "warning": "A matching snapshot is not CI/conformance verification. This report compares identity only; it never emits VERIFIED.",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", default="docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json")
    parser.add_argument("--policy", default="docs/audit/ATC-ORG-REPOSITORY-DRIFT-SCOPE-001.json")
    parser.add_argument("--integration", default="docs/integration/ATC-ORG-REPOSITORY-INTEGRATION-001.json")
    parser.add_argument("--report", default="org-repository-drift-report.json")
    args = parser.parse_args()
    try:
        manifest = json.loads(Path(args.manifest).read_text(encoding="utf-8"))
        policy = json.loads(Path(args.policy).read_text(encoding="utf-8"))
        integration = json.loads(Path(args.integration).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"INPUT_INCOMPLETE: {exc}", file=sys.stderr)
        return 2
    report = compare(manifest, policy, integration, token=os.environ.get("GITHUB_TOKEN"))
    Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 1 if report["validation"] in {"BLOCKED", "DRIFT_DETECTED"} else 0


if __name__ == "__main__":
    raise SystemExit(main())
