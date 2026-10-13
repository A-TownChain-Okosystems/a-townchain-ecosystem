#!/usr/bin/env python3
"""Reconcile lexical source candidates with catalog function bindings.

This report is deliberately conservative: lexical symbol hits are not contracts,
and matching names do not constitute functional verification.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


def reconcile(catalog: dict, census: dict) -> dict:
    census_repos = {
        entry.get("repository"): entry
        for entry in census.get("repositories", [])
        if isinstance(entry, dict) and entry.get("repository")
    }
    symbols_by_repo: dict[str, set[tuple[str, str, str, int]]] = {}
    symbol_signatures_by_repo: dict[str, dict[tuple[str, str, str, int], str]] = {}
    test_symbols_by_repo: dict[str, set[tuple[str, str, str]]] = {}
    file_metadata_by_repo: dict[str, dict[str, dict]] = {}
    for repository, entry in census_repos.items():
        report = entry.get("report") or {}
        found: set[tuple[str, str, str, int]] = set()
        signatures: dict[tuple[str, str, str, int], str] = {}
        test_found: set[tuple[str, str, str]] = set()
        file_metadata: dict[str, dict] = {}
        for file in report.get("files", []):
            path = file.get("path", "")
            file_metadata[path] = {
                "file_sha256": file.get("file_sha256"),
                "language": file.get("language"),
            }
            for symbol in file.get("candidate_public_symbols", []) + file.get("candidate_internal_symbols", []):
                key = (path, symbol.get("kind", "unknown"), symbol.get("name", ""), int(symbol.get("line_start", 0)))
                found.add(key)
                signatures[key] = symbol.get("signature", "")
            for symbol in file.get("candidate_test_symbols", []):
                test_found.add((path, symbol.get("kind", "function"), symbol.get("name", "")))
        symbols_by_repo[repository] = found
        symbol_signatures_by_repo[repository] = signatures
        test_symbols_by_repo[repository] = test_found
        file_metadata_by_repo[repository] = file_metadata

    bound: dict[str, set[tuple[str, str, str, int]]] = {};
    function_findings = []
    component_findings = []
    self_reference_observations = []
    self_repository = (catalog.get("authority") or {}).get("repository")
    total_functions = 0
    for component in catalog.get("components", []):
        owner = component.get("canonical_owner")
        source_inventory = component.get("source_inventory") or {}
        inventory_repo = source_inventory.get("repository")
        repo_entry = census_repos.get(inventory_repo or owner)
        if not repo_entry:
            component_findings.append({
                "component_id": component.get("id"),
                "component": component.get("name"),
                "finding": "SOURCE_REPOSITORY_NOT_IN_CENSUS",
                "repository": inventory_repo or owner,
                "status": "BLOCKED",
            })
        else:
            observed = source_inventory.get("observed_head_sha")
            actual = repo_entry.get("source_sha")
            if observed and actual and observed != actual:
                if inventory_repo == self_repository:
                    # A self-hosted catalog PR is scanned at its exact PR HEAD,
                    # which necessarily differs from the default-branch SHA recorded
                    # inside the catalog. Preserve this as an explicit observation,
                    # not a false component source-drift finding.
                    self_reference_observations.append({
                        "component_id": component.get("id"),
                        "component": component.get("name"),
                        "finding": "SELF_REFERENCE_PR_HEAD_DIFFERS_FROM_DEFAULT_BRANCH",
                        "catalog_default_branch_sha": observed,
                        "census_pr_head_sha": actual,
                        "status": "RESIDUAL",
                    })
                else:
                    component_findings.append({
                        "component_id": component.get("id"),
                        "component": component.get("name"),
                        "finding": "CATALOG_CENSUS_SHA_MISMATCH",
                        "catalog_sha": observed,
                        "census_sha": actual,
                        "status": "RESIDUAL",
                    })
        for function in component.get("functions", []):
            total_functions += 1
            binding = function.get("source_binding") or {}
            repo = inventory_repo or owner
            path = binding.get("source_path")
            symbol = binding.get("symbol_or_entrypoint")
            symbol_kind = binding.get("symbol_kind") or "function"
            key = (path or "", symbol_kind, symbol or "")
            item = {
                "component_id": component.get("id"),
                "component": component.get("name"),
                "function_id": function.get("id"),
                "function": function.get("name"),
                "repository": repo,
                "source_path": path,
                "symbol_or_entrypoint": symbol,
                "symbol_kind": symbol_kind,
                "line_start": binding.get("line_start"),
                "catalog_source_sha": binding.get("exact_source_sha"),
                "census_source_sha": (census_repos.get(repo) or {}).get("source_sha"),
                "status": "RESIDUAL",
                "finding": None,
            }
            binding_status = binding.get("status")
            item["binding_status"] = binding_status
            item["binding_note"] = binding.get("note")
            reasoned_statuses = {
                "SOURCE_SURFACE_NOT_IDENTIFIED": "SOURCE_SURFACE_NOT_IDENTIFIED",
                "NORMATIVE_DECISION_BLOCKED": "NORMATIVE_DECISION_BLOCKED",
                "BLOCKED_ARCHIVED_REFERENCE_ONLY": "BLOCKED_ARCHIVED_REFERENCE_ONLY",
                "BLOCKED_PRIVATE_REPOSITORY_EXCLUDED": "BLOCKED_PRIVATE_REPOSITORY_EXCLUDED",
                "NON_FUNCTION_SOURCE_ARTIFACT": "NON_FUNCTION_SOURCE_ARTIFACT_EXCLUDED_FROM_SYMBOL_SCAN",
            }
            binding_line = binding.get("line_start")
            if binding_status in reasoned_statuses:
                item["finding"] = reasoned_statuses[binding_status]
            elif symbol_kind != "function":
                item["finding"] = "FUNCTION_BINDING_KIND_NOT_EXECUTABLE"
            elif binding_status == "TEST_SURFACE_ONLY":
                candidates_test = test_symbols_by_repo.get(repo, set())
                if key not in candidates_test:
                    item["finding"] = "BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN"
                elif binding.get("exact_source_sha") != census_repos.get(repo, {}).get("source_sha"):
                    item["finding"] = "FUNCTION_BINDING_SHA_MISMATCH"
                else:
                    item["finding"] = "TEST_SURFACE_ONLY_NOT_RUNTIME_IMPLEMENTATION"
            elif binding_status == "RELATED_SOURCE_SURFACE_ONLY":
                candidates_related = [candidate for candidate in symbols_by_repo.get(repo, set()) if candidate[:3] == key and (binding_line is None or candidate[3] == int(binding_line))]
                if not candidates_related:
                    item["finding"] = "BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN"
                elif binding_line is None and len(candidates_related) > 1:
                    item["finding"] = "AMBIGUOUS_SOURCE_SYMBOL_BINDING"
                    item["candidate_line_starts"] = sorted(candidate[3] for candidate in candidates_related)
                elif binding.get("exact_source_sha") != census_repos.get(repo, {}).get("source_sha"):
                    item["finding"] = "FUNCTION_BINDING_SHA_MISMATCH"
                else:
                    matched = candidates_related[0]
                    bound.setdefault(repo, set()).add(matched)
                    item["matched_line_start"] = matched[3]
                    item["matched_signature"] = symbol_signatures_by_repo.get(repo, {}).get(matched, "")
                    item["finding"] = "RELATED_SOURCE_SURFACE_ONLY_CONTRACT_RESIDUAL"
            elif not path or not symbol:
                item["finding"] = "FUNCTION_BINDING_MISSING"
            elif repo not in census_repos or not census_repos[repo].get("report"):
                item["finding"] = "SOURCE_CENSUS_UNAVAILABLE"
                item["status"] = "BLOCKED"
            else:
                identity_candidates = [
                    candidate for candidate in symbols_by_repo.get(repo, set())
                    if candidate[:3] == key
                ]
                candidates = [
                    candidate for candidate in identity_candidates
                    if binding_line is None or candidate[3] == int(binding_line)
                ]
                if not identity_candidates:
                    item["finding"] = "BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN"
                elif binding_line is not None and not candidates and len(identity_candidates) == 1:
                    # The exact source SHA and unique path/kind/name match prove
                    # that the symbol exists, but a stale line locator must remain
                    # an explicit catalog defect rather than being misreported as
                    # a missing symbol or silently accepted as a clean binding.
                    item["finding"] = "SOURCE_BINDING_LINE_MISMATCH"
                    item["catalog_line_start"] = int(binding_line)
                    item["observed_line_start"] = identity_candidates[0][3]
                elif binding_line is None and len(identity_candidates) > 1:
                    item["finding"] = "AMBIGUOUS_SOURCE_SYMBOL_BINDING"
                    item["candidate_line_starts"] = sorted(candidate[3] for candidate in identity_candidates)
                elif binding.get("exact_source_sha") != census_repos[repo].get("source_sha"):
                    item["finding"] = "FUNCTION_BINDING_SHA_MISMATCH"
                else:
                    matched = candidates[0]
                    bound.setdefault(repo, set()).add(matched)
                    item["matched_line_start"] = matched[3]
                    item["matched_signature"] = symbol_signatures_by_repo.get(repo, {}).get(matched, "")
                    item["finding"] = "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING"
            function_findings.append(item)

    unmatched_candidates = []
    excluded_candidates = []
    # Inline Rust test functions are scanned separately from runtime candidates.
    # Preserve exact source/file identity and an explicit reason for excluding them
    # from the runtime function catalog.
    for repo, found in sorted(test_symbols_by_repo.items()):
        for path, symbol_kind, symbol in sorted(found):
            file_meta = file_metadata_by_repo.get(repo, {}).get(path, {})
            excluded_candidates.append({
                "repository": repo,
                "source_sha": (census_repos.get(repo) or {}).get("source_sha"),
                "source_path": path,
                "source_file_sha256": file_meta.get("file_sha256"),
                "language": file_meta.get("language"),
                "symbol_kind": symbol_kind,
                "symbol_or_entrypoint": symbol,
                "status": "EXCLUDED_BY_POLICY",
                "finding": "INLINE_TEST_FUNCTION_EXCLUDED_FROM_RUNTIME_CATALOG",
                "reason": "Function is inside an inline Rust cfg(test) module or has a test-only attribute. It is retained as a verification surface, not a runtime function contract.",
            })
    for repo, found in sorted(symbols_by_repo.items()):
        for path, symbol_kind, symbol, line_start in sorted(found - bound.get(repo, set())):
            file_meta = file_metadata_by_repo.get(repo, {}).get(path, {})
            source_sha = (census_repos.get(repo) or {}).get("source_sha")
            symbol_key = (path, symbol_kind, symbol, line_start)
            common = {
                "repository": repo,
                "source_sha": source_sha,
                "source_path": path,
                "source_file_sha256": file_meta.get("file_sha256"),
                "language": file_meta.get("language"),
                "symbol_kind": symbol_kind,
                "symbol_or_entrypoint": symbol,
                "line_start": line_start,
                "signature": symbol_signatures_by_repo.get(repo, {}).get(symbol_key, ""),
            }
            if symbol_kind != "function":
                excluded_candidates.append({
                    **common,
                    "status": "EXCLUDED_BY_POLICY",
                    "finding": "NON_FUNCTION_SOURCE_ARTIFACT_EXCLUDED_FROM_SYMBOL_SCAN",
                    "reason": "The source symbol is a type, value, or macro rather than an executable function. It remains in the exact source inventory and is excluded from runtime function coverage by policy; add a separate typed contract if it carries protocol/security semantics.",
                })
                continue
            parts = [part.lower() for part in path.replace("\\\\", "/").split("/")]
            filename = parts[-1] if parts else ""
            test_surface = any(part in {"tests", "test", "__tests__", "benches", "bench", "fuzz", "fuzzing", "fixtures", "fixture"} for part in parts[:-1]) or filename.startswith("test_") or filename.endswith("_test.py") or filename.endswith("_test.rs") or filename.endswith("_tests.py")
            example_surface = any(part in {"examples", "example", "samples", "sample"} for part in parts[:-1])
            if test_surface:
                excluded_candidates.append({
                    **common,
                    "status": "EXCLUDED_BY_POLICY",
                    "finding": "TEST_OR_FIXTURE_FUNCTION_EXCLUDED_FROM_RUNTIME_CATALOG",
                    "reason": "Test, fixture, benchmark, or fuzz-harness functions are inventoried as verification surfaces, not runtime function contracts. They must be linked to test evidence rather than counted as production API functions.",
                })
            elif example_surface:
                excluded_candidates.append({
                    **common,
                    "status": "EXCLUDED_BY_POLICY",
                    "finding": "EXAMPLE_OR_SAMPLE_FUNCTION_EXCLUDED_FROM_RUNTIME_CATALOG",
                    "reason": "Example/sample code is not a canonical runtime entrypoint. Keep the exact repository, source SHA, path, file hash and symbol for traceability; validate conformance separately if it is normative.",
                })
            else:
                unmatched_candidates.append({
                    **common,
                    "status": "RESIDUAL",
                    "finding": "CANDIDATE_NOT_BOUND_TO_CATALOG_FUNCTION",
                })

    blocking = census.get("status") == "BLOCKED" or any(
        item.get("status") == "BLOCKED" for item in component_findings + function_findings
    )
    return {
        "schema_id": "ATC-COMPONENT-FUNCTION-RECONCILIATION-001",
        "schema_version": "1.0.0",
        "status": "BLOCKED" if blocking else "RESIDUAL",
        "catalog_schema": catalog.get("schema_id"),
        "census_schema": census.get("schema_id"),
        "census_status": census.get("status"),
        "summary": {
            "components": len(catalog.get("components", [])),
            "functions": total_functions,
            "function_bindings_residual_or_blocked": sum(
                item["finding"] != "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING"
                for item in function_findings
            ),
            "component_findings": len(component_findings),
            "candidate_symbols_total": sum(len(found) for found in symbols_by_repo.values()) + sum(len(found) for found in test_symbols_by_repo.values()),
            "candidate_runtime_symbols_total": sum(1 for found in symbols_by_repo.values() for _, kind, _, _ in found if kind == "function"),
            "candidate_non_function_symbols_total": sum(1 for found in symbols_by_repo.values() for _, kind, _, _ in found if kind != "function"),
            "candidate_test_symbols_total": sum(len(found) for found in test_symbols_by_repo.values()),
            "candidate_symbols_bound": sum(len(bound.get(repo, set())) for repo in symbols_by_repo),
            "candidate_symbols_unbound": len(unmatched_candidates),
            "candidate_symbols_excluded_by_policy": len(excluded_candidates),
            "functional_verification": "NOT_PERFORMED",
        },
        "component_findings": component_findings,
        "self_reference_observations": self_reference_observations,
        "function_findings": function_findings,
        "unmatched_candidates": unmatched_candidates,
        "excluded_candidates": excluded_candidates,
        "limitations": [
            "A lexical name/path match is discovery evidence only, not a function contract or proof of behavior.",
            "Dynamic, generated, macro-expanded, conditional, indirect, and non-public entrypoints may be absent from the scan.",
            "A repository census SHA mismatch must be reconciled before any source binding can be considered current.",
            "Tests and workflow evidence must be separately linked and verified at the exact source SHA.",
        ],
    }


def completion_gaps(report: dict) -> list[str]:
    """Return blockers for the strict source-to-catalog completeness gate.

    A PASS means every scanned candidate is bound or has a documented policy
    exclusion and every catalog entry has a resolved source disposition. It
    does not mean functional verification or production readiness.
    """
    gaps: list[str] = []
    summary = report.get("summary") or {}
    if summary.get("candidate_symbols_unbound", 0):
        gaps.append(f'{summary["candidate_symbols_unbound"]} source symbol candidates remain unbound and without a reasoned exclusion')
    if summary.get("component_findings", 0):
        gaps.append(f'{summary["component_findings"]} component source inventory findings remain unresolved')
    if report.get("census_status") == "BLOCKED":
        gaps.append("organization source census is BLOCKED")
    allowed_findings = {
        "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING",
        "TEST_SURFACE_ONLY_NOT_RUNTIME_IMPLEMENTATION",
        "NON_FUNCTION_SOURCE_ARTIFACT_EXCLUDED_FROM_SYMBOL_SCAN",
        "SOURCE_SURFACE_NOT_IDENTIFIED",
        "NORMATIVE_DECISION_BLOCKED",
        "BLOCKED_ARCHIVED_REFERENCE_ONLY",
        "BLOCKED_PRIVATE_REPOSITORY_EXCLUDED",
    }
    unresolved = []
    for item in report.get("function_findings", []):
        finding = item.get("finding")
        # A source-surface gap is an explicit implementation backlog item, not an
        # unbound existing symbol. It is allowed for catalog/source-inventory
        # completion only when the catalog records a concrete reason. It does not
        # imply that the required function exists or is verified.
        explicit_gap = (
            finding == "FUNCTION_BINDING_MISSING"
            and item.get("binding_status") == "SOURCE_SURFACE_NOT_IDENTIFIED"
            and bool(item.get("binding_note"))
        )
        reasoned_finding = finding in allowed_findings and bool(item.get("binding_note"))
        if not (explicit_gap or reasoned_finding):
            unresolved.append(item)
    if unresolved:
        counts: dict[str, int] = {}
        for item in unresolved:
            finding = str(item.get("finding") or "UNCLASSIFIED")
            counts[finding] = counts.get(finding, 0) + 1
        gaps.append("unresolved catalog source bindings: " + ", ".join(f"{key}={value}" for key, value in sorted(counts.items())))
    return gaps


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", required=True, type=Path)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--require-complete", action="store_true", help="Fail unless all scanned candidates and catalog bindings are reconciled; functional verification remains separate.")
    args = parser.parse_args()
    catalog = json.loads(args.catalog.read_text(encoding="utf-8"))
    census = json.loads(args.census.read_text(encoding="utf-8"))
    if catalog.get("schema_id") != "ATC-COMPONENT-FUNCTION-CATALOG-001":
        parser.error("unexpected component/function catalog schema")
    if census.get("schema_id") != "ATC-ORG-SOURCE-CENSUS-001":
        parser.error("unexpected organization census schema")
    report = reconcile(catalog, census)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    gaps = completion_gaps(report)
    report["completion_gate"] = "PASS" if not gaps else "BLOCKED"
    report["completion_gaps"] = gaps
    args.output.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": report["status"], "completion_gate": report["completion_gate"], "completion_gaps": gaps, **report["summary"], "output": str(args.output)}, sort_keys=True))
    if args.require_complete and gaps:
        return 2
    return 0 if report["status"] == "RESIDUAL" else 2


if __name__ == "__main__":
    sys.exit(main())
