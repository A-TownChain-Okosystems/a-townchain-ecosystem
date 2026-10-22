#!/usr/bin/env python3
"""Deterministic validation for the A-TownChain competitive-monitoring registry."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

ENTRY_STATUSES = {
    "BASELINE", "UNCHANGED", "CHANGED", "STATUS_CHANGED", "NEW",
    "WITHDRAWN", "UNVERIFIED", "ANNOUNCED_PENDING_EFFECTIVE_DATE",
}
SOURCE_STATUSES = {
    "OFFICIAL_DOCS_AND_PRICING_VERIFIED", "PARTIAL_OFFICIAL_VERIFIED",
    "PARTIAL_OFFICIAL_DOCS_VERIFIED", "PARTIAL_OFFICIAL_ROADMAP_VERIFIED",
    "PARTIAL_OFFICIAL_RELEASE_HEALTH_VERIFIED",
    "OFFICIAL_RELEASE_ARCHIVES_VERIFIED",
    "OFFICIAL_PRODUCT_AND_PRICING_ANNOUNCEMENT_VERIFIED",
    "PARTIAL_OFFICIAL_PRODUCT_AND_PRICING_VERIFIED", "UNVERIFIED",
}
DELTA_TYPES = {"ADD", "UPDATE", "REMOVE"}
ID_RE = re.compile(r"^[A-Z0-9]+(?:-[A-Z0-9]+)*-[0-9]{3}$")


def _has_unverified_marker(entry: dict[str, Any], *terms: str) -> bool:
    values = [str(x).lower() for x in entry.get("unverified", [])]
    return any(term.lower() in value for term in terms for value in values)


def validate_registry(registry: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if not isinstance(registry, dict):
        return ["registry must be a JSON object"]
    if not isinstance(registry.get("entries"), list):
        return ["entries must be an array"]
    seen: set[str] = set()
    for i, entry in enumerate(registry["entries"]):
        where = f"entries[{i}]"
        if not isinstance(entry, dict):
            errors.append(f"{where} must be an object")
            continue
        ident = entry.get("id")
        if not isinstance(ident, str) or not ID_RE.fullmatch(ident):
            errors.append(f"{where}.id is missing or invalid")
        elif ident in seen:
            errors.append(f"duplicate stable id: {ident}")
        else:
            seen.add(ident)
        for field in ("name", "areas", "products", "urls", "checked", "source_status", "snapshot"):
            if field not in entry or entry[field] in (None, "", []):
                errors.append(f"{where}.{field} is required")
        if entry.get("source_status") not in SOURCE_STATUSES:
            errors.append(f"{where}.source_status has unsupported value")
        urls = entry.get("urls", {})
        if not isinstance(urls, dict):
            errors.append(f"{where}.urls must be an object")
            urls = {}
        valid_url = False
        for key, value in urls.items():
            if value is None or value == "":
                continue
            if not isinstance(value, str) or not value.startswith("https://"):
                errors.append(f"{where}.urls.{key} must be https or null")
            else:
                valid_url = True
        if not valid_url and entry.get("source_status") != "UNVERIFIED" and not _has_unverified_marker(entry, "official", "source", "url"):
            errors.append(f"{where} has no usable source URL and is not explicitly UNVERIFIED")
        snapshot = entry.get("snapshot", {})
        if not isinstance(snapshot, dict):
            errors.append(f"{where}.snapshot must be an object")
            continue
        status = snapshot.get("status")
        if status is not None and status not in ENTRY_STATUSES:
            errors.append(f"{where}.snapshot.status has unsupported value: {status}")
        if "prices" in snapshot and snapshot["prices"] is not None:
            if not isinstance(snapshot["prices"], list):
                errors.append(f"{where}.snapshot.prices must be an array or null")
            else:
                for j, price in enumerate(snapshot["prices"]):
                    pwhere = f"{where}.snapshot.prices[{j}]"
                    if not isinstance(price, dict):
                        errors.append(f"{pwhere} must be an object")
                        continue
                    currency = price.get("currency")
                    billing = price.get("billing")
                    effective = any(price.get(k) for k in ("effective_date", "effective_from", "effective_until"))
                    if not currency or currency == "UNVERIFIED":
                        if not _has_unverified_marker(entry, "currency", "price"):
                            errors.append(f"{pwhere} currency missing/unverified without explicit UNVERIFIED marker")
                    if not billing and not _has_unverified_marker(entry, "billing", "price"):
                        errors.append(f"{pwhere} billing model missing without explicit UNVERIFIED marker")
                    if not effective and not _has_unverified_marker(entry, "effective date", "price date", "price"):
                        errors.append(f"{pwhere} effective date missing without explicit UNVERIFIED marker")
    return errors


def validate_overlay(base: dict[str, Any], overlay: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    entries = base.get("entries")
    deltas = overlay.get("deltas")
    if not isinstance(entries, list):
        return ["base.entries must be an array"]
    if not isinstance(deltas, list):
        return ["overlay.deltas must be an array"]
    ids = [e.get("id") for e in entries if isinstance(e, dict)]
    if len(ids) != len(set(ids)):
        errors.append("base contains duplicate stable IDs")
    known = set(ids)
    seen_delta_ids: set[str] = set()
    for i, delta in enumerate(deltas):
        where = f"deltas[{i}]"
        if not isinstance(delta, dict):
            errors.append(f"{where} must be an object")
            continue
        kind, ident = delta.get("type"), delta.get("id")
        if kind not in DELTA_TYPES:
            errors.append(f"{where}.type is invalid")
            continue
        if not isinstance(ident, str) or not ID_RE.fullmatch(ident):
            errors.append(f"{where}.id is missing or invalid")
            continue
        if ident in seen_delta_ids:
            errors.append(f"duplicate delta ID: {ident}")
        seen_delta_ids.add(ident)
        if kind == "ADD" and ident in known:
            errors.append(f"ADD target already exists: {ident}")
        elif kind in {"UPDATE", "REMOVE"} and ident not in known:
            errors.append(f"{kind} target does not exist: {ident}")
        if kind == "ADD":
            known.add(ident)
        elif kind == "REMOVE":
            known.discard(ident)
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--registry", required=True, help="Fully expanded registry JSON")
    parser.add_argument("--base", help="Base registry JSON, required with --overlay")
    parser.add_argument("--overlay", help="Delta overlay JSON, required with --base")
    args = parser.parse_args()
    try:
        registry = json.loads(Path(args.registry).read_text(encoding="utf-8"))
        errors = validate_registry(registry)
        if bool(args.base) != bool(args.overlay):
            errors.append("--base and --overlay must be provided together")
        if args.base and args.overlay:
            base = json.loads(Path(args.base).read_text(encoding="utf-8"))
            overlay = json.loads(Path(args.overlay).read_text(encoding="utf-8"))
            errors.extend(validate_overlay(base, overlay))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2
    if errors:
        print("\n".join(f"ERROR: {item}" for item in errors))
        return 1
    print(json.dumps({
        "status": "PASS",
        "registry_id": registry.get("registry_id"),
        "entry_count": len(registry["entries"]),
        "stable_ids_unique": True,
        "source_urls_checked": True,
        "price_metadata_checked": True,
        "overlay_operations_checked": bool(args.base),
    }, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
