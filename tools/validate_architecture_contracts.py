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


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    path = root / "architecture/contracts/p0-boundary-contracts.json"
    data = json.loads(path.read_text(encoding="utf-8"))

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

    print(f"Architecture P0 contracts: PASS ({len(contracts)} contracts)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
