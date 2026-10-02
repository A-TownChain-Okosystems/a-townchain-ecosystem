#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

EXPECTED = {
    "ARCH-P0-001",
    "ARCH-P0-002",
    "ARCH-P0-003",
    "ARCH-P0-004",
    "ARCH-P0-005",
    "ARCH-P0-006",
}
STATUSES = {"UNANALYZED", "ANALYZED", "FIXED", "RERUNNING", "VERIFIED", "RESIDUAL"}


def load_json(root: Path, relative: str) -> dict:
    return json.loads((root / relative).read_text(encoding="utf-8"))


def validate_contracts(data: dict) -> None:
    assert data["schema"] == "ATC-ARCH-P0-CONTRACTS-1.0"
    assert data["architecture_source"] == "ARCHITECTURE.md"
    assert data["status"] == "ACTIVE"

    contracts = data["contracts"]
    ids = {item["id"] for item in contracts}
    assert ids == EXPECTED, f"unexpected contract IDs: {sorted(ids)}"
    assert len(contracts) == len(EXPECTED)

    for item in contracts:
        assert item["deny_by_default"] is True, item["id"]
        assert item["authority"], item["id"]
        assert item["canonical_owner"], item["id"]


def validate_conformance(data: dict) -> None:
    assert data["schema"] == "ATC-ARCH-P0-CONFORMANCE-1.0"
    assert data["source_contracts"] == "architecture/contracts/p0-boundary-contracts.json"
    assert data["status"] == "ACTIVE"
    assert data["evidence_status"] in STATUSES

    entries = data["entries"]
    ids = {item["contract_id"] for item in entries}
    assert ids == EXPECTED, f"unexpected conformance IDs: {sorted(ids)}"
    assert len(entries) == len(EXPECTED)

    for item in entries:
        assert item["implementation_surface"], item["contract_id"]
        assert item["test_requirements"], item["contract_id"]
        assert item["ci_evidence_required"], item["contract_id"]
        assert isinstance(item["integration_evidence_required"], bool), item["contract_id"]
        assert isinstance(item["e2e_evidence_required"], bool), item["contract_id"]
        assert item["status"] in STATUSES, item["contract_id"]


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    contracts = load_json(root, "architecture/contracts/p0-boundary-contracts.json")
    conformance = load_json(root, "architecture/contracts/p0-conformance-matrix.json")

    validate_contracts(contracts)
    validate_conformance(conformance)

    print(f"Architecture P0 contracts: PASS ({len(contracts['contracts'])} contracts)")
    print(f"Architecture P0 conformance: PASS ({len(conformance['entries'])} entries; evidence={conformance['evidence_status']})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
