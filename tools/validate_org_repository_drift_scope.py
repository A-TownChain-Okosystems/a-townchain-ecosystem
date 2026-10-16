#!/usr/bin/env python3
"""Fail-closed scope resolver for the live organization repository drift gate."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ALLOWED_DECISIONS = {"INCLUDE", "EXCLUDE", "BLOCK"}


def resolve(manifest: dict, policy: dict) -> tuple[list[str], dict]:
    errors: list[str] = []
    if manifest.get("schema_id") != "ATC-ORG-REPOSITORY-MANIFEST-001":
        errors.append("manifest schema_id must be ATC-ORG-REPOSITORY-MANIFEST-001")
    if policy.get("schema_id") != "ATC-ORG-REPOSITORY-DRIFT-SCOPE-001":
        errors.append("policy schema_id must be ATC-ORG-REPOSITORY-DRIFT-SCOPE-001")
    if policy.get("schema_version") != "1.0.0":
        errors.append("unsupported drift scope policy schema_version")
    defaults = policy.get("default_policy")
    if not isinstance(defaults, dict):
        errors.append("default_policy must be an object")
        defaults = {}
    overrides = policy.get("explicit_overrides")
    if not isinstance(overrides, list):
        errors.append("explicit_overrides must be an array")
        overrides = []
    repositories = manifest.get("repositories")
    if not isinstance(repositories, list):
        errors.append("manifest repositories must be an array")
        repositories = []

    override_by_name = {}
    for index, item in enumerate(overrides):
        if not isinstance(item, dict):
            errors.append(f"explicit_overrides[{index}] must be an object")
            continue
        name = item.get("name")
        if not isinstance(name, str) or not name.strip():
            errors.append(f"explicit_overrides[{index}] requires a non-empty name")
            continue
        if name in override_by_name:
            errors.append(f"duplicate scope override for {name}")
        override_by_name[name] = item
        if item.get("decision") not in ALLOWED_DECISIONS:
            errors.append(f"{name}: invalid override decision")
        if not isinstance(item.get("reason"), str) or not item.get("reason", "").strip():
            errors.append(f"{name}: explicit override requires a reason")
        if not isinstance(item.get("reason_code"), str) or not item.get("reason_code", "").strip():
            errors.append(f"{name}: explicit override requires a reason_code")

    names = []
    entries = []
    for index, repo in enumerate(repositories):
        if not isinstance(repo, dict):
            errors.append(f"manifest repositories[{index}] must be an object")
            continue
        name = repo.get("name")
        if not isinstance(name, str) or not name.strip():
            errors.append(f"manifest repositories[{index}] requires a name")
            continue
        names.append(name)
        archived = repo.get("archived")
        private = repo.get("private", False)
        if not isinstance(archived, bool):
            errors.append(f"{name}: archived must be boolean")
            archived = None
        if not isinstance(private, bool):
            errors.append(f"{name}: private must be boolean when present")
            private = None
        override = override_by_name.get(name)
        if override:
            for field, actual in (("expected_archived", archived), ("expected_private", private), ("expected_role", repo.get("role"))):
                if field in override and override[field] != actual:
                    errors.append(f"{name}: override {field} does not match manifest")
            decision = override.get("decision", "BLOCK")
            reason = override.get("reason", "")
            reason_code = override.get("reason_code", "")
        elif archived is True:
            decision = defaults.get("archived_repository", "BLOCK")
            reason = "Default policy for archived repository"
            reason_code = "DEFAULT_ARCHIVED_POLICY"
        elif archived is False:
            decision = defaults.get("active_repository", "BLOCK")
            reason = "Default policy for active repository"
            reason_code = "DEFAULT_ACTIVE_POLICY"
        else:
            decision, reason, reason_code = "BLOCK", "Archive state is unknown or invalid", "INVALID_ARCHIVE_STATE"
        if private is None:
            decision, reason, reason_code = "BLOCK", "Private flag is invalid", "INVALID_PRIVATE_FLAG"
        if decision not in ALLOWED_DECISIONS:
            errors.append(f"{name}: resolved decision {decision!r} is invalid")
            decision = "BLOCK"
        entries.append({
            "name": name,
            "archived": archived,
            "private": private,
            "role": repo.get("role"),
            "decision": decision,
            "reason_code": reason_code,
            "reason": reason
        })

    if len(names) != len(set(names)):
        errors.append("manifest contains duplicate repository names")
    manifest_names = set(names)
    for name in override_by_name:
        if name not in manifest_names:
            errors.append(f"scope override references unknown repository {name}")
    if defaults.get("unknown_repository") != "BLOCK" or defaults.get("unknown_scope_action") != "BLOCK":
        errors.append("unknown repositories and unknown scope must fail closed with BLOCK")
    if defaults.get("private_repository") != "NO_OVERRIDE":
        errors.append("private status must not implicitly include or exclude a repository")
    if any(item["decision"] == "BLOCK" for item in entries):
        errors.append("one or more repositories resolve to BLOCK")
    included = sum(item["decision"] == "INCLUDE" for item in entries)
    excluded = sum(item["decision"] == "EXCLUDE" for item in entries)
    report = {
        "schema_id": "ATC-ORG-REPOSITORY-DRIFT-SCOPE-REPORT-001",
        "schema_version": "1.0.0",
        "validation": "FAIL" if errors else "PASS_STRUCTURAL_ONLY",
        "repositories_total": len(entries),
        "repositories_included": included,
        "repositories_excluded": excluded,
        "repositories_blocked": sum(item["decision"] == "BLOCK" for item in entries),
        "entries": entries,
        "errors": errors,
        "warning": "Structural scope validation does not perform live GitHub HEAD/tree comparison."
    }
    return errors, report


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", default="docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json")
    parser.add_argument("--policy", default="docs/audit/ATC-ORG-REPOSITORY-DRIFT-SCOPE-001.json")
    parser.add_argument("--report", default="org-repository-drift-scope-report.json")
    args = parser.parse_args()
    try:
        manifest = json.loads(Path(args.manifest).read_text(encoding="utf-8"))
        policy = json.loads(Path(args.policy).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"INPUT_INCOMPLETE: {exc}", file=sys.stderr)
        return 2
    errors, report = resolve(manifest, policy)
    Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
