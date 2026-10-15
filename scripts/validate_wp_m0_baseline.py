#!/usr/bin/env python3
"""Fail-closed validation of the revision-pinned WP-M0 repository baseline.

Requires Python 3.10+ and an optional GITHUB_TOKEN with organization repository
read access. The validator never treats unavailable API data as a pass.
"""
from __future__ import annotations

import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path
from datetime import datetime, timezone

OWNER = "A-TownChain-Okosystems"
MANIFEST = Path(__file__).resolve().parents[1] / "docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json"
SHA40 = re.compile(r"^[0-9a-f]{40}$")
EXCLUDED = {"demo-repository"}
API = "https://api.github.com"


class ValidationError(RuntimeError):
    pass


def api_get(path: str):
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "ATC-WP-M0-baseline-validator",
    }
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("ATC_GITHUB_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = urllib.request.Request(API + path, headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            if response.status != 200:
                raise ValidationError(f"HTTP {response.status}: {path}")
            return json.loads(response.read().decode("utf-8"))
    except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, json.JSONDecodeError) as exc:
        raise ValidationError(f"API fetch failed for {path}: {exc}") from exc


def main() -> int:
    errors: list[str] = []
    try:
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"BLOCKED: cannot read/parse manifest: {exc}", file=sys.stderr)
        return 2

    if manifest.get("owner") != OWNER:
        errors.append(f"manifest owner must be {OWNER}")
    entries = manifest.get("repositories")
    if not isinstance(entries, list) or not entries:
        errors.append("manifest repositories must be a non-empty array")
        entries = []
    names = [entry.get("name") for entry in entries if isinstance(entry, dict)]
    if len(names) != len(entries):
        errors.append("every repository entry must be an object")
    if len(names) != len(set(names)):
        errors.append("duplicate repository names in manifest")
    if len(entries) != 33:
        errors.append(f"expected 33 inventory entries, got {len(entries)}")

    manifest_by_name = {entry.get("name"): entry for entry in entries if isinstance(entry, dict)}
    if EXCLUDED - set(manifest_by_name):
        errors.append("required explicit exclusion missing from manifest: demo-repository")
    for name, entry in manifest_by_name.items():
        if not name or not isinstance(name, str):
            errors.append("empty/invalid repository name")
            continue
        if not isinstance(entry.get("default_branch"), str) or not entry["default_branch"].strip():
            errors.append(f"{name}: default_branch missing or invalid")
        for field in ("head_sha", "root_tree_sha"):
            if not isinstance(entry.get(field), str) or not SHA40.fullmatch(entry[field]):
                errors.append(f"{name}: invalid/missing {field}")
        if entry.get("drift_gate_excluded") != (name in EXCLUDED):
            errors.append(f"{name}: drift_gate_excluded must be true only for explicit exclusions")

    # Enumerate the organization to detect additions/removals; inability to
    # enumerate the full inventory is a hard failure, never a silent partial pass.
    live_repos = []
    page = 1
    while True:
        batch = api_get(f"/orgs/{OWNER}/repos?type=all&per_page=100&page={page}")
        if not isinstance(batch, list):
            raise ValidationError("organization repository listing returned a non-array response")
        live_repos.extend(batch)
        if len(batch) < 100:
            break
        page += 1
        if page > 20:
            raise ValidationError("repository pagination safety limit exceeded")
    live_by_name = {r.get("name"): r for r in live_repos if isinstance(r, dict)}
    if len(live_by_name) != len(live_repos):
        errors.append("live organization listing contains duplicate/invalid repository entries")
    expected_names = set(manifest_by_name)
    live_names = set(live_by_name)
    for name in sorted((expected_names - live_names) - EXCLUDED):
        errors.append(f"manifest repository missing from live organization listing: {name}")
    for name in sorted(live_names - expected_names):
        errors.append(f"live organization repository missing from manifest: {name}")

    checked = 0
    for name in sorted(expected_names - EXCLUDED):
        entry = manifest_by_name[name]
        live_meta = live_by_name.get(name)
        if not live_meta:
            continue
        try:
            repo = api_get(f"/repos/{OWNER}/{name}")
            default_branch = repo.get("default_branch")
            archived = repo.get("archived")
            if default_branch != entry.get("default_branch"):
                errors.append(f"{name}: default branch drift: manifest={entry.get('default_branch')} live={default_branch}")
                continue
            if archived != entry.get("archived"):
                errors.append(f"{name}: archived status drift: manifest={entry.get('archived')} live={archived}")
            ref = api_get(f"/repos/{OWNER}/{name}/git/ref/heads/{default_branch}")
            head_sha = (ref.get("object") or {}).get("sha")
            if (ref.get("object") or {}).get("type") != "commit" or not isinstance(head_sha, str) or not SHA40.fullmatch(head_sha):
                errors.append(f"{name}: default-branch ref did not resolve to a valid commit SHA")
                continue
            commit = api_get(f"/repos/{OWNER}/{name}/git/commits/{head_sha}")
            tree_sha = (commit.get("tree") or {}).get("sha")
            if not isinstance(tree_sha, str) or not SHA40.fullmatch(tree_sha):
                errors.append(f"{name}: commit object has no valid root-tree SHA")
                continue
            if head_sha != entry.get("head_sha"):
                errors.append(f"{name}: HEAD drift: manifest={entry.get('head_sha')} live={head_sha}")
            if tree_sha != entry.get("root_tree_sha"):
                errors.append(f"{name}: root-tree drift: manifest={entry.get('root_tree_sha')} live={tree_sha}")
            checked += 1
        except ValidationError as exc:
            errors.append(f"{name}: {exc}")

    report = {
        "schema_id": "ATC-WP-M0-VALIDATION-REPORT-001",
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "manifest_path": str(MANIFEST.relative_to(Path(__file__).resolve().parents[1])),
        "manifest_schema_version": manifest.get("schema_version"),
        "inventory_count": len(entries),
        "live_inventory_count": len(live_by_name),
        "live_drift_gate_exclusions": sorted(EXCLUDED),
        "revision_pins_checked": checked,
        "revision_pins_expected_in_gate": len(expected_names - EXCLUDED),
        "status": "PASS" if not errors and checked == len(expected_names - EXCLUDED) else "BLOCKED",
        "errors": errors,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["status"] == "PASS" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(json.dumps({"status": "BLOCKED", "fatal_error": str(exc)}, indent=2), file=sys.stderr)
        raise SystemExit(2)
