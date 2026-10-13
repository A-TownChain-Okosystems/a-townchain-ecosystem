#!/usr/bin/env python3
"""Report identical source-file hashes across repositories without merging symbols.

The report is a review aid only. A SHA-256 match proves byte-identical file
content; it does not establish canonical ownership, API equivalence, migration
status, or functional verification.
"""
from __future__ import annotations

import argparse
import json
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


def build_report(census: dict[str, Any]) -> dict[str, Any]:
    groups: dict[str, list[dict[str, Any]]] = defaultdict(list)
    malformed_files = 0
    repositories = census.get("repositories", [])
    for repo_entry in repositories:
        if not isinstance(repo_entry, dict):
            continue
        repo = repo_entry.get("repository")
        report = repo_entry.get("report") or {}
        source_sha = repo_entry.get("source_sha")
        for file_entry in report.get("files", []):
            if not isinstance(file_entry, dict):
                malformed_files += 1
                continue
            digest = file_entry.get("file_sha256")
            path = file_entry.get("path")
            if not isinstance(digest, str) or len(digest) != 64 or not isinstance(path, str):
                malformed_files += 1
                continue
            try:
                int(digest, 16)
            except ValueError:
                malformed_files += 1
                continue
            symbols = []
            for key in ("candidate_public_symbols", "candidate_internal_symbols", "candidate_test_symbols"):
                for symbol in file_entry.get(key, []) or []:
                    if isinstance(symbol, dict):
                        symbols.append({
                            "symbol_kind": symbol.get("kind", "unknown"),
                            "symbol": symbol.get("name", ""),
                            "line_start": symbol.get("line_start"),
                            "candidate_class": key.removeprefix("candidate_").removesuffix("_symbols"),
                        })
            groups[digest.lower()].append({
                "repository": repo,
                "source_sha": source_sha,
                "path": path,
                "language": file_entry.get("language"),
                "candidate_count": len(symbols),
                "candidates": sorted(symbols, key=lambda x: (
                    str(x.get("candidate_class")), str(x.get("symbol_kind")),
                    str(x.get("symbol")), str(x.get("line_start"))
                )),
            })

    cross_repo = []
    repeated_candidate_groups = 0
    cross_repo_candidate_lines = 0
    for digest, files in sorted(groups.items()):
        candidate_rows = sum(item["candidate_count"] for item in files)
        if candidate_rows > 1:
            repeated_candidate_groups += 1
        repo_names = sorted({str(item.get("repository")) for item in files if item.get("repository")})
        if len(repo_names) > 1:
            cross_repo_candidate_lines += candidate_rows
            cross_repo.append({
                "sha256": digest,
                "repositories": repo_names,
                "repository_count": len(repo_names),
                "file_occurrence_count": len(files),
                "candidate_count": candidate_rows,
                "files": sorted(files, key=lambda x: (
                    str(x.get("repository")), str(x.get("path")), str(x.get("source_sha"))
                )),
                "decision": "REVIEW_REQUIRED_NO_AUTOMATIC_BINDING",
            })

    return {
        "schema_id": "ATC-CROSS-REPOSITORY-FILE-HASH-REPORT-001",
        "schema_version": "1.0.0",
        "evidence_class": "TRIAGE_ONLY",
        "hash_algorithm": "SHA-256",
        "semantics": {
            "same_hash": "identical file bytes only",
            "automatic_candidate_merge": False,
            "automatic_canonical_owner_assignment": False,
            "functional_verification": False,
        },
        "summary": {
            "repositories_in_census": len([
                r for r in repositories if isinstance(r, dict) and r.get("repository")
            ]),
            "unique_file_hashes": len(groups),
            "hashes_with_multiple_candidate_rows": repeated_candidate_groups,
            "cross_repository_hash_groups": len(cross_repo),
            "cross_repository_candidate_rows": cross_repo_candidate_lines,
            "malformed_file_entries": malformed_files,
        },
        "groups": cross_repo,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", required=True, help="Exact-HEAD source census JSON")
    parser.add_argument("--output", required=True, help="Output report JSON")
    parser.add_argument("--require-cross-repo", action="store_true",
                        help="Fail if no cross-repository hash groups are found")
    args = parser.parse_args()
    try:
        census = json.loads(Path(args.input).read_text(encoding="utf-8"))
        if not isinstance(census, dict) or not isinstance(census.get("repositories"), list):
            raise ValueError("input must contain a repositories array")
        result = build_report(census)
        output = Path(args.output)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(result, indent=2, sort_keys=True), encoding="utf-8")
        print(json.dumps({"status": "REPORT_GENERATED", **result["summary"]}, sort_keys=True))
        if args.require_cross_repo and not result["groups"]:
            return 2
        if result["summary"]["malformed_file_entries"]:
            return 1
        return 0
    except (OSError, json.JSONDecodeError, ValueError) as exc:
        print(f"cross-repository hash report failed: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
