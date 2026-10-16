#!/usr/bin/env python3
"""Reconcile each census candidate against the checked-in runtime registry.

This is a row-level evidence tool, not a functional verifier. It deliberately
fails closed when a runtime candidate cannot be matched to the canonical
registry at the same repository/path/kind/symbol and file hash. Non-runtime
candidates receive a reproducible policy classification and reason.
"""
from __future__ import annotations

import argparse
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

from tools.build_runtime_symbol_registry import (
    ARCHIVED_REPOSITORIES,
    UMBRELLA_REPO,
    catalog_bindings,
    classify_path,
)


def key(repository: str, path: str, kind: str, symbol: str) -> tuple[str, str, str, str]:
    return repository, path, kind, symbol


def reconcile(census: dict, catalog: dict, canonical_registry: dict) -> dict:
    bound_keys, duplicate_bindings, delegated, duplicate_fn_ids = catalog_bindings(catalog)
    canonical_entries = canonical_registry.get("entries") or []
    canonical_by_key: dict[tuple[str, str, str, str], list[dict]] = defaultdict(list)
    for entry in canonical_entries:
        canonical_by_key[key(entry.get("repository", ""), entry.get("source_path", ""),
                             entry.get("kind", ""), entry.get("symbol", ""))].append(entry)

    seen_runtime_keys: set[tuple[str, str, str, str]] = set()
    rows: list[dict] = []
    expected_counts: dict[str, Counter] = defaultdict(Counter)
    current_counts: dict[str, Counter] = defaultdict(Counter)
    for repo_entry in census.get("repositories", []):
        repository = repo_entry.get("repository", "")
        name = repo_entry.get("name", repository.rsplit("/", 1)[-1])
        source_sha = repo_entry.get("source_sha")
        report = repo_entry.get("report") or {}
        if repo_entry.get("status") in {"BLOCKED", "EXCLUDED_PRIVATE_NO_CI_ACCESS"} or not source_sha:
            rows.append({
                "repository": repository, "repository_head_sha": source_sha,
                "status": "UNRESOLVED", "current_classification": None,
                "expected_classification": None,
                "policy_or_reason": "Census repository has no usable source SHA/report.",
                "evidence_ref": "ATC-ORG-SOURCE-CENSUS-001",
            })
            continue
        for file in report.get("files", []):
            path = file.get("path", "")
            path_class = classify_path(path)
            for symbol in file.get("candidate_public_symbols", []):
                symbol_name = symbol.get("name", "")
                kind = symbol.get("kind", "unknown")
                candidate_key = key(repository, path, kind, symbol_name)
                binding_key = (repository, path, symbol_name)
                if binding_key in bound_keys:
                    classification = "BOUND_TO_CATALOG_FUNCTION"
                    status = "MAPPED"
                    reason = "Exact source path and symbol are bound by the component/function catalog."
                    evidence = "docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json"
                elif name == UMBRELLA_REPO and path.startswith("components/"):
                    subrepo = path.split("/")[1] if len(path.split("/")) > 1 else ""
                    classification = ("MIRRORED_ARCHIVED_COMPONENT" if subrepo in ARCHIVED_REPOSITORIES
                                      else "MIRRORED_REFERENCE_NON_SSOT")
                    status = "EXCLUDED"
                    reason = "Umbrella subtree mirror; canonical source is the component repository."
                    evidence = "docs/build/ATC-BUILD-MANIFEST-001.json"
                elif path_class != "source":
                    classification = f"EXCLUDED_{path_class.upper()}"
                    status = "EXCLUDED"
                    reason = f"Deterministic path policy classifies this candidate as {path_class}."
                    evidence = "tools/build_runtime_symbol_registry.py:classify_path"
                else:
                    classification = "RUNTIME_SYMBOL"
                    matching = canonical_by_key.get(candidate_key, [])
                    exact = [entry for entry in matching
                             if entry.get("file_sha256") == file.get("file_sha256")
                             and entry.get("census_source_sha") == source_sha]
                    if len(exact) == 1:
                        status = "MAPPED"
                        reason = "Exact candidate key, file hash and repository source SHA match one canonical registry entry."
                    elif not matching:
                        status = "UNRESOLVED"
                        reason = "Runtime candidate has no canonical registry entry with the same repository/path/kind/symbol."
                    elif len(exact) > 1:
                        status = "UNRESOLVED"
                        reason = "Multiple canonical entries match the same exact candidate evidence."
                    else:
                        status = "UNRESOLVED"
                        reason = "Canonical entry exists for this candidate key, but file hash or repository source SHA differs."
                    evidence = "docs/audit/ATC-RUNTIME-SYMBOL-REGISTRY-001.json"
                    seen_runtime_keys.add(candidate_key)
                expected_counts[repository][classification] += 1
                rows.append({
                    "repository": repository,
                    "repository_head_sha": source_sha,
                    "source_path": path,
                    "symbol": symbol_name,
                    "kind": kind,
                    "file_sha256": file.get("file_sha256"),
                    "current_classification": (
                        "CANONICAL_ENTRY_MATCH" if status == "MAPPED" and classification == "RUNTIME_SYMBOL"
                        else classification if status == "MAPPED" else None
                    ),
                    "expected_classification": classification,
                    "status": status,
                    "policy_or_reason": reason,
                    "evidence_ref": evidence,
                })

    for entry in canonical_entries:
        repository = entry.get("repository", "")
        candidate_key = key(repository, entry.get("source_path", ""), entry.get("kind", ""), entry.get("symbol", ""))
        if candidate_key not in seen_runtime_keys:
            rows.append({
                "repository": repository,
                "repository_head_sha": entry.get("census_source_sha"),
                "source_path": entry.get("source_path"),
                "symbol": entry.get("symbol"),
                "kind": entry.get("kind"),
                "file_sha256": entry.get("file_sha256"),
                "current_classification": entry.get("status"),
                "expected_classification": None,
                "status": "UNRESOLVED",
                "policy_or_reason": "Canonical runtime registry entry has no corresponding runtime candidate in this census.",
                "evidence_ref": "docs/audit/ATC-RUNTIME-SYMBOL-REGISTRY-001.json",
            })

    for repo, classes in (canonical_registry.get("summary", {}).get("per_repository") or {}).items():
        current_counts[repo].update(classes)

    by_status = Counter(row.get("status", "UNRESOLVED") for row in rows)
    by_repo = []
    all_repos = sorted(set(expected_counts) | set(current_counts))
    for repository in all_repos:
        expected = dict(sorted(expected_counts.get(repository, {}).items()))
        current = dict(sorted(current_counts.get(repository, {}).items()))
        deltas = {
            classification: expected.get(classification, 0) - current.get(classification, 0)
            for classification in sorted(set(expected) | set(current))
            if expected.get(classification, 0) != current.get(classification, 0)
        }
        by_repo.append({
            "repository": repository,
            "expected_classification_counts": expected,
            "canonical_classification_counts": current,
            "classification_deltas": deltas,
            "status": "MAPPED" if not deltas else "UNRESOLVED",
        })

    unresolved = sum(1 for row in rows if row.get("status") == "UNRESOLVED")
    return {
        "schema_id": "ATC-RUNTIME-REGISTRY-CANDIDATE-RECONCILIATION-001",
        "schema_version": "1.0.0",
        "status": "PASS" if unresolved == 0 and not duplicate_bindings and not duplicate_fn_ids else "BLOCKED",
        "functional_verification": "NOT_PERFORMED",
        "sources": {
            "census_schema": census.get("schema_id"),
            "canonical_registry_schema": canonical_registry.get("schema_id"),
            "census_repository_heads": {
                item.get("repository"): item.get("source_sha") for item in census.get("repositories", [])
            },
        },
        "summary": {
            "candidate_rows": len(rows),
            "mapped": by_status["MAPPED"],
            "excluded": by_status["EXCLUDED"],
            "unresolved": unresolved,
            "repositories_with_classification_deltas": sum(1 for item in by_repo if item["classification_deltas"]),
            "duplicate_catalog_bindings": len(duplicate_bindings),
            "duplicate_catalog_function_ids": len(duplicate_fn_ids),
            "aggregate_classification_delta": sum(
                abs(delta) for item in by_repo for delta in item["classification_deltas"].values()
            ),
        },
        "repositories": by_repo,
        "candidates": rows,
        "limitations": [
            "MAPPED means source identity/classification evidence matches; it is not functional verification.",
            "EXCLUDED is a reproducible current-policy decision and does not prove that the prior registry stored the same per-candidate decision.",
            "Historical classifications cannot be reconstructed individually from aggregate-only canonical summary counts; such historical claims remain unresolved.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--catalog", required=True, type=Path)
    parser.add_argument("--canonical-registry", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    result = reconcile(
        json.loads(args.census.read_text(encoding="utf-8")),
        json.loads(args.catalog.read_text(encoding="utf-8")),
        json.loads(args.canonical_registry.read_text(encoding="utf-8")),
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], **result["summary"], "output": str(args.output)}, sort_keys=True))
    return 0 if result["status"] == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
