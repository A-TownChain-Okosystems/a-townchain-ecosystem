#!/usr/bin/env python3
"""Validate the committed function catalog against its JSON Schema.

This helper intentionally does not claim to regenerate the Rust inventory: use the
source-census workflow/tooling to refresh source entries, then validate this artifact.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CATALOG = ROOT / "docs" / "function-catalog.json"
SCHEMA = ROOT / "schemas" / "function-catalog.schema.json"

def main() -> int:
    try:
        catalog = json.loads(CATALOG.read_text(encoding="utf-8"))
        schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"FAIL: cannot load catalog/schema: {exc}")
        return 1
    try:
        import jsonschema
    except ImportError:
        print("BLOCKED: install jsonschema to validate schema: python3 -m pip install jsonschema")
        return 2
    errors = sorted(jsonschema.Draft202012Validator(schema).iter_errors(catalog), key=lambda e: list(map(str, e.path)))
    if errors:
        for error in errors:
            print(f"FAIL: {list(error.path)}: {error.message}")
        return 1
    print(f"PASS: schema-valid catalog; {catalog['summary']['functions_discovered']} discovered functions")
    if catalog["catalog_status"] != "SOURCE_INVENTORY_BASELINE_NOT_COMPLETE":
        print("BLOCKED: catalog status requires review")
        return 2
    print("NOTE: semantic contracts are not verified by schema validation.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
