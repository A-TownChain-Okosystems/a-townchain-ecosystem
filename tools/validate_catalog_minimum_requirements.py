#!/usr/bin/env python3
"""Fail-closed minimum-requirements checks for the canonical ATC catalog.

Structural success is not functional, security, or release verification.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

SHA_RE = re.compile(r"^[0-9a-f]{40}$")
IMPLEMENTED_STATES = {"IMPLEMENTED", "TESTED", "AUDITED", "PRODUCTION_READY", "VERIFIED"}
MAPPED_STATES = {"MAPPED", "SOURCE_MAPPED", "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED"}


def validate(doc: dict) -> list[str]:
    errors: list[str] = []
    components = doc.get("components")
    if not isinstance(components, list) or not components:
        return ["components must be a non-empty array"]

    component_ids, function_ids = [], []
    component_id_set = set()
    all_functions = []
    for ci, comp in enumerate(components):
        if not isinstance(comp, dict):
            errors.append(f"components[{ci}] must be an object")
            continue
        cid = comp.get("id")
        if not isinstance(cid, str) or not cid.strip():
            errors.append(f"components[{ci}] missing id")
            continue
        component_ids.append(cid)
        component_id_set.add(cid)
        if not comp.get("canonical_owner"):
            errors.append(f"{cid}: canonical_owner is required")
        funcs = comp.get("functions")
        if not isinstance(funcs, list) or not funcs:
            errors.append(f"{cid}: functions must be a non-empty array")
            continue
        for fi, fn in enumerate(funcs):
            if not isinstance(fn, dict):
                errors.append(f"{cid}.functions[{fi}] must be an object")
                continue
            fid = fn.get("id")
            if not isinstance(fid, str) or not fid.strip():
                errors.append(f"{cid}.functions[{fi}] missing stable id")
                continue
            function_ids.append(fid)
            all_functions.append((cid, fn))
            for field in ("name", "purpose", "inputs", "outputs", "failure_modes", "acceptance", "test_classes", "readiness"):
                value = fn.get(field)
                if value is None or value == "" or value == []:
                    errors.append(f"{fid}: required field {field} is missing/empty")
            readiness = str(fn.get("readiness", "")).upper()
            binding = fn.get("source_binding")
            if binding is None:
                analysis = fn.get("contract_analysis")
                evidence = analysis.get("source_evidence") if isinstance(analysis, dict) else None
                if not isinstance(evidence, dict):
                    errors.append(f"{fid}: source_binding or explicit contract_analysis.source_evidence required")
                    evidence = {}
                status = str(evidence.get("status", "")).upper()
                if not status:
                    errors.append(f"{fid}: source evidence must state mapping status")
            else:
                if not isinstance(binding, dict):
                    errors.append(f"{fid}: source_binding must be an object")
                    binding = {}
                status = str(binding.get("status", "")).upper()
                if not status:
                    errors.append(f"{fid}: source_binding.status is required")
                mapped = status in MAPPED_STATES or "MAPPED" in status
                implemented = readiness in IMPLEMENTED_STATES
                if mapped or implemented:
                    for field in ("source_path", "symbol_or_entrypoint", "exact_source_sha", "file_blob_sha"):
                        if not binding.get(field):
                            errors.append(f"{fid}: mapped/implemented binding missing {field}")
                    for field in ("exact_source_sha", "file_blob_sha"):
                        value = binding.get(field)
                        if value and not SHA_RE.fullmatch(str(value)):
                            errors.append(f"{fid}: {field} must be a full 40-character lowercase Git SHA")
                if readiness in {"TESTED", "AUDITED", "PRODUCTION_READY", "VERIFIED"}:
                    if not binding.get("evidence"):
                        errors.append(f"{fid}: verified/readiness claim requires evidence reference")
                    if readiness == "PRODUCTION_READY" and not fn.get("production_evidence"):
                        errors.append(f"{fid}: PRODUCTION_READY requires production_evidence")
                if readiness in {"RESIDUAL", "BLOCKED", "UNRESOLVED"} and not (binding.get("note") or binding.get("reason") or fn.get("residuals") or fn.get("contract_analysis")):
                    errors.append(f"{fid}: unresolved/residual state requires explanation")
            for field in ("inputs", "outputs", "failure_modes", "acceptance", "test_classes"):
                value = fn.get(field)
                if not isinstance(value, list) or not value or any(not isinstance(x, str) or not x.strip() for x in value):
                    errors.append(f"{fid}: {field} must be a non-empty array of non-empty strings")

    if len(component_ids) != len(set(component_ids)):
        errors.append("duplicate component IDs")
    if len(function_ids) != len(set(function_ids)):
        errors.append("duplicate function IDs")

    for cid, comp in ((c.get("id"), c) for c in components if isinstance(c, dict)):
        deps = comp.get("depends_on", [])
        if not isinstance(deps, list):
            errors.append(f"{cid}: depends_on must be an array")
            continue
        for dep in deps:
            if dep not in component_id_set:
                errors.append(f"{cid}: unresolved component dependency {dep!r}")

    interfaces = doc.get("cross_component_interfaces", [])
    if not isinstance(interfaces, list):
        errors.append("cross_component_interfaces must be an array")
        interfaces = []
    interface_ids = []
    for i, interface in enumerate(interfaces):
        if not isinstance(interface, dict):
            errors.append(f"cross_component_interfaces[{i}] must be an object")
            continue
        iid = interface.get("id")
        interface_ids.append(iid)
        for field in ("producer", "consumer", "contract", "status"):
            if not interface.get(field):
                errors.append(f"{iid}: interface missing {field}")
        if interface.get("producer") not in component_id_set:
            errors.append(f"{iid}: unknown producer")
        if interface.get("consumer") not in component_id_set:
            errors.append(f"{iid}: unknown consumer")
    if len(interface_ids) != len(set(interface_ids)):
        errors.append("duplicate interface IDs")

    for gi, gate in enumerate(doc.get("production_gates", [])):
        if not isinstance(gate, dict) or not gate.get("id") or not isinstance(gate.get("acceptance"), list) or not gate.get("acceptance"):
            errors.append(f"production_gates[{gi}] requires id and non-empty acceptance")
    release = doc.get("release_decision", {})
    if isinstance(release, dict) and release.get("status") in {"PRODUCTION_READY", "VERIFIED"}:
        if errors:
            errors.append("release decision cannot be VERIFIED/PRODUCTION_READY while catalog errors exist")
        if any(str(g.get("status", "")).upper() != "VERIFIED" for g in doc.get("production_gates", []) if isinstance(g, dict)):
            errors.append("release decision requires every production gate VERIFIED")
    return errors


def main() -> int:
    path = Path(sys.argv[1] if len(sys.argv) > 1 else "docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json")
    try:
        doc = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"CATALOG_MINIMUM_REQUIREMENTS_FAIL: {exc}", file=sys.stderr)
        return 2
    errors = validate(doc)
    result = {"validator": "ATC-CATALOG-MINIMUM-REQUIREMENTS-001", "status": "FAIL" if errors else "PASS_STRUCTURAL_ONLY", "error_count": len(errors), "errors": errors, "functional_or_security_verification": False, "release_readiness": "BLOCKED" if errors else "NOT_ESTABLISHED"}
    print(json.dumps(result, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
