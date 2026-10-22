#!/usr/bin/env python3
"""Deterministically resolve a base registry and self-contained delta overlay."""
from __future__ import annotations

import argparse
import copy
import json
import sys
from pathlib import Path
from typing import Any


def canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n"


def resolve(base: dict[str, Any], overlay: dict[str, Any], template: dict[str, Any]) -> dict[str, Any]:
    if not isinstance(base.get("entries"), list):
        raise ValueError("base.entries must be an array")
    if overlay.get("mode") != "DELTA_OVERLAY":
        raise ValueError("overlay.mode must be DELTA_OVERLAY")
    if overlay.get("extends") != template.get("source_registry", {}).get("path", overlay.get("extends")):
        # A template may omit source_registry; exact extends is checked by caller metadata.
        if template.get("source_registry"):
            raise ValueError("overlay extends does not match expanded template source registry")
    entries = copy.deepcopy(base["entries"])
    index = {e.get("id"): i for i, e in enumerate(entries) if isinstance(e, dict)}
    if len(index) != len(entries):
        raise ValueError("base contains missing or duplicate stable IDs")
    seen: set[str] = set()
    for i, delta in enumerate(overlay.get("deltas", [])):
        if not isinstance(delta, dict):
            raise ValueError(f"deltas[{i}] must be an object")
        kind, ident = delta.get("type"), delta.get("id")
        if kind not in {"ADD", "UPDATE", "REMOVE"}:
            raise ValueError(f"deltas[{i}].type is invalid")
        if not isinstance(ident, str) or not ident:
            raise ValueError(f"deltas[{i}].id is required")
        if ident in seen:
            raise ValueError(f"duplicate delta ID: {ident}")
        seen.add(ident)
        if kind == "ADD":
            payload = delta.get("entry")
            if ident in index:
                raise ValueError(f"ADD target already exists: {ident}")
            if not isinstance(payload, dict) or payload.get("id") != ident:
                raise ValueError(f"ADD {ident} requires entry payload with matching id")
            index[ident] = len(entries)
            entries.append(copy.deepcopy(payload))
        elif kind == "UPDATE":
            changes = delta.get("changes")
            if ident not in index:
                raise ValueError(f"UPDATE target does not exist: {ident}")
            if not isinstance(changes, dict) or not changes:
                raise ValueError(f"UPDATE {ident} requires non-empty changes object")
            if "id" in changes and changes["id"] != ident:
                raise ValueError(f"UPDATE {ident} cannot change stable ID")
            entries[index[ident]].update(copy.deepcopy(changes))
        else:
            if ident not in index:
                raise ValueError(f"REMOVE target does not exist: {ident}")
            entries.pop(index[ident])
            index = {e["id"]: i for i, e in enumerate(entries)}
    result = copy.deepcopy(template)
    result["entries"] = entries
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", required=True)
    parser.add_argument("--overlay", required=True)
    parser.add_argument("--template", required=True, help="Expected expanded registry envelope")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    try:
        base = json.loads(Path(args.base).read_text(encoding="utf-8"))
        overlay = json.loads(Path(args.overlay).read_text(encoding="utf-8"))
        template = json.loads(Path(args.template).read_text(encoding="utf-8"))
        resolved = resolve(base, overlay, template)
        output = Path(args.output)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(resolved, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        expected = json.loads(Path(args.template).read_text(encoding="utf-8"))
        if canonical(resolved) != canonical(expected):
            print("FAIL: resolved registry differs from expanded template", file=sys.stderr)
            return 1
        print(json.dumps({
            "status": "PASS",
            "entry_count": len(resolved["entries"]),
            "delta_count": len(overlay["deltas"]),
            "canonical_match": True,
            "stable_ids_unique": len({e["id"] for e in resolved["entries"]}) == len(resolved["entries"]),
        }, sort_keys=True, separators=(",", ":")))
        return 0
    except (OSError, json.JSONDecodeError, ValueError, KeyError, TypeError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
