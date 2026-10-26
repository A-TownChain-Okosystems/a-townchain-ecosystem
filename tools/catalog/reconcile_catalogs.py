#!/usr/bin/env python3
"""Deterministically normalize registered JSON catalogs without merging records.

Source snapshots must be staged as <source_id>.json in --sources-dir.
The tool preserves each original record, derives stable namespaced IDs, reports
possible duplicates for human review, and fails closed on missing/invalid inputs.
It does not claim implementation or functional verification.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

COLLECTION_KINDS = {
    "components": "component",
    "functions": "function",
    "function_contracts": "function_contract",
    "function_capability_contracts": "function_capability_contract",
    "capabilities": "capability",
    "requirements": "requirement",
    "interfaces": "interface",
    "apis": "api",
    "endpoints": "endpoint",
    "opcodes": "opcode",
    "services": "service",
    "modules": "module",
    "symbols": "symbol",
    "ui_actions": "ui_action",
    "actions": "action",
    "test_functions": "test_function",
    "contracts": "contract",
    "tokens": "token",
    "keywords": "keyword",
    "productions": "grammar_production",
    "ast_nodes": "ast_node",
    "error_classes": "error_class",
    "builtins_list": "builtin",
    "semantic_rules": "semantic_rule",
    "type_definitions": "type_definition",
}
MAP_COLLECTION_KINDS = {
    "builtins": "builtin",
    "semantic_rules": "semantic_rule",
    "rules": "semantic_rule",
}
NAME_KEYS = ("name", "function", "symbol", "title", "component", "capability", "id")
ID_KEYS = ("id", "function_id", "component_id", "capability_id", "interface_id")


def canonical_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def digest(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def safe_namespace(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9._-]+", "-", value).strip("-") or "source"


def display_name(record: Any) -> str:
    if isinstance(record, dict):
        for key in NAME_KEYS:
            value = record.get(key)
            if isinstance(value, (str, int)) and str(value).strip():
                return str(value).strip()
    if isinstance(record, str):
        return record.strip()
    return ""


def original_id(record: Any) -> str | None:
    if isinstance(record, dict):
        for key in ID_KEYS:
            value = record.get(key)
            if isinstance(value, (str, int)) and str(value).strip():
                return str(value)
    return None


def collect_records(value: Any, path: str = "$") -> list[dict[str, Any]]:
    """Collect only arrays under known catalog collection keys; retain exact objects."""
    found: list[dict[str, Any]] = []
    if isinstance(value, dict):
        for key, child in value.items():
            child_path = f"{path}.{key}"
            kind = COLLECTION_KINDS.get(key.lower())
            if kind and isinstance(child, list):
                for index, item in enumerate(child):
                    if isinstance(item, (dict, str, int, float, bool)) or item is None:
                        found.append({
                            "collection": key,
                            "record_kind": kind,
                            "json_pointer": f"{child_path}[{index}]",
                            "source_record": item,
                        })
            elif key.lower() in MAP_COLLECTION_KINDS and isinstance(child, dict):
                map_kind = MAP_COLLECTION_KINDS[key.lower()]
                for entry_key, entry_value in child.items():
                    found.append({
                        "collection": key,
                        "record_kind": map_kind,
                        "json_pointer": f"{child_path}.{entry_key}",
                        "source_record": {"key": entry_key, "value": entry_value},
                    })
            else:
                found.extend(collect_records(child, child_path))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            found.extend(collect_records(child, f"{path}[{index}]"))
    return found


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as stream:
        return json.load(stream)


def collect_markdown_tables(path: Path) -> list[dict[str, Any]]:
    """Extract Markdown table rows without inferring semantics from column names."""
    lines = path.read_text(encoding="utf-8").splitlines()
    found: list[dict[str, Any]] = []
    index = 0
    while index < len(lines) - 1:
        header_line = lines[index].strip()
        separator_line = lines[index + 1].strip()
        if not (header_line.startswith("|") and separator_line.startswith("|") and
                re.fullmatch(r"\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?", separator_line)):
            index += 1
            continue
        headers = [cell.strip() for cell in header_line.strip("|").split("|")]
        index += 2
        row_number = 0
        while index < len(lines) and lines[index].strip().startswith("|"):
            cells = [cell.strip() for cell in lines[index].strip().strip("|").split("|")]
            if len(cells) == len(headers):
                record = {
                    re.sub(r"[^a-z0-9]+", "_", header.lower()).strip("_") or f"column_{col_index}": cells[col_index]
                    for col_index, header in enumerate(headers)
                }
                found.append({
                    "collection": "markdown_table",
                    "record_kind": "catalog_entry",
                    "json_pointer": f"line:{index + 1}",
                    "source_record": record,
                })
                row_number += 1
            index += 1
        if row_number == 0:
            continue
    return found


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry", required=True, type=Path)
    parser.add_argument("--sources-dir", required=True, type=Path)
    parser.add_argument("--out-dir", required=True, type=Path)
    args = parser.parse_args()

    try:
        registry = load_json(args.registry)
    except (OSError, json.JSONDecodeError) as exc:
        print(f"BLOCKED: cannot read registry: {exc}", file=sys.stderr)
        return 2

    sources = registry.get("source_catalogs") if isinstance(registry, dict) else None
    if not isinstance(sources, list) or not sources:
        print("BLOCKED: registry.source_catalogs must be a non-empty array", file=sys.stderr)
        return 2

    errors: list[str] = []
    records: list[dict[str, Any]] = []
    source_summary: list[dict[str, Any]] = []
    seen_source_ids: set[str] = set()

    for source in sources:
        if not isinstance(source, dict):
            errors.append("registry contains a non-object source entry")
            continue
        source_id = source.get("source_id")
        if not isinstance(source_id, str) or not source_id.strip():
            errors.append("source entry missing source_id")
            continue
        if source_id in seen_source_ids:
            errors.append(f"duplicate source_id in registry: {source_id}")
            continue
        seen_source_ids.add(source_id)
        snapshot_format = str(source.get("snapshot_format", "json")).lower()
        if snapshot_format not in {"json", "markdown"}:
            errors.append(f"unsupported snapshot_format for {source_id}: {snapshot_format}")
            continue
        suffix = ".md" if snapshot_format == "markdown" else ".json"
        snapshot = args.sources_dir / f"{source_id}{suffix}"
        if not snapshot.is_file():
            errors.append(f"missing snapshot: {snapshot}")
            source_summary.append({"source_id": source_id, "status": "MISSING_SNAPSHOT", "records_imported": 0})
            continue
        try:
            if snapshot_format == "json":
                document = load_json(snapshot)
                collected = collect_records(document)
            else:
                collected = collect_markdown_tables(snapshot)
        except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
            errors.append(f"invalid {snapshot_format} snapshot {snapshot}: {exc}")
            source_summary.append({"source_id": source_id, "status": "INVALID_SNAPSHOT", "records_imported": 0})
            continue

        source_sha256 = hashlib.sha256(snapshot.read_bytes()).hexdigest()
        source_summary.append({
            "source_id": source_id,
            "repository": source.get("repository"),
            "path": source.get("path"),
            "declared_head_sha": source.get("head_sha"),
            "snapshot_path": snapshot.name,
            "snapshot_sha256": source_sha256,
            "status": "IMPORTED" if collected else "NO_RECOGNIZED_COLLECTIONS",
            "records_imported": len(collected),
        })
        if not collected:
            errors.append(f"no recognized record collections in {snapshot}")
            continue

        for item in collected:
            raw = item["source_record"]
            raw_id = original_id(raw)
            record_hash = digest(raw)
            stable_fragment = safe_namespace(raw_id) if raw_id else record_hash[:16]
            normalized_id = f"{safe_namespace(source_id)}::{item['record_kind']}::{stable_fragment}"
            records.append({
                "normalized_record_id": normalized_id,
                "source_id": source_id,
                "source_repository": source.get("repository"),
                "source_catalog_path": source.get("path"),
                "source_catalog_snapshot": snapshot.name,
                "source_catalog_sha256": source_sha256,
                "declared_source_head_sha": source.get("head_sha"),
                "source_record_id": raw_id,
                "record_kind": item["record_kind"],
                "source_collection": item["collection"],
                "source_json_pointer": item["json_pointer"],
                "display_name": display_name(raw),
                "source_record_sha256": record_hash,
                "reconciliation_status": "UNRECONCILED",
                "implementation_status": "UNASSESSED",
                "verification_status": "UNVERIFIED",
                "source_record": raw,
            })

    ids: dict[str, int] = defaultdict(int)
    semantic_names: dict[str, list[str]] = defaultdict(list)
    for record in records:
        ids[record["normalized_record_id"]] += 1
        name = record["display_name"].casefold().strip()
        if name:
            semantic_names[name].append(record["normalized_record_id"])
    duplicate_ids = sorted(key for key, count in ids.items() if count > 1)
    if duplicate_ids:
        errors.append("normalized ID collision(s): " + ", ".join(duplicate_ids[:20]))

    possible_duplicates = [
        {"normalized_name": name, "record_ids": sorted(set(record_ids)), "status": "POSSIBLE_DUPLICATE_REQUIRES_REVIEW"}
        for name, record_ids in sorted(semantic_names.items())
        if len(set(record_ids)) > 1
    ]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    master = {
        "schema_id": "ATC-UNIFIED-COMPONENT-FUNCTION-CATALOG-001",
        "schema_version": "1.0.0",
        "status": "BLOCKED" if errors else "RECONCILIATION_REQUIRED",
        "normative": False,
        "purpose": "Loss-preserving union of registered source catalog records; not proof of implementation or verification.",
        "source_registry_sha256": hashlib.sha256(args.registry.read_bytes()).hexdigest(),
        "source_count": len(sources),
        "imported_source_count": sum(1 for item in source_summary if item["status"] == "IMPORTED"),
        "record_count": len(records),
        "record_kind_counts": dict(sorted(__import__("collections").Counter(r["record_kind"] for r in records).items())),
        "records": sorted(records, key=lambda r: (r["source_id"], r["normalized_record_id"], r["source_json_pointer"])),
    }
    report = {
        "schema_id": "ATC-CATALOG-RECONCILIATION-REPORT-001",
        "status": "BLOCKED" if errors else "RECONCILIATION_REQUIRED",
        "source_count": len(sources),
        "imported_source_count": master["imported_source_count"],
        "record_count": len(records),
        "duplicate_normalized_id_count": len(duplicate_ids),
        "possible_duplicate_group_count": len(possible_duplicates),
        "sources": source_summary,
        "possible_duplicates": possible_duplicates,
        "errors": errors,
        "limitations": [
            "JSON collection keys are an explicit allowlist; Markdown extraction covers pipe tables only.",
            "Possible duplicates are not automatically merged or deleted.",
            "Snapshot hashes establish snapshot integrity, not that the snapshot matches a declared commit.",
            "Function names/signatures do not establish semantic correctness.",
            "No implementation, test, CI, security-audit, or runtime verification is inferred.",
        ],
    }
    (args.out_dir / "catalog-master.json").write_text(json.dumps(master, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    (args.out_dir / "reconciliation-report.json").write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({
        "status": report["status"],
        "source_count": report["source_count"],
        "imported_source_count": report["imported_source_count"],
        "record_count": report["record_count"],
        "possible_duplicate_group_count": report["possible_duplicate_group_count"],
        "errors": errors,
        "outputs": ["catalog-master.json", "reconciliation-report.json"],
    }, indent=2))
    return 2 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
