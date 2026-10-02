#!/usr/bin/env python3
"""Validate the A-TownChain project evidence/status framework.

JSON Schema handles structural constraints. This validator adds repository-local
cross-field and human/machine consistency rules without generating PASS/VERIFIED
values.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parent.parent
STATUS = ROOT / "governance" / "status.json"
SCHEMA = ROOT / "schemas" / "governance" / "project-status.schema.json"
PROJECT_STATUS = ROOT / "PROJECT_STATUS.md"
CLAIMS = ROOT / "governance" / "CLAIM_TO_EVIDENCE.md"
RESIDUALS = ROOT / "governance" / "RESIDUALS.md"
ATTRIBUTION = ROOT / "governance" / "ATTRIBUTION.md"
SECURITY = ROOT / "governance" / "SECURITY.md"

EVD_ID_RE = re.compile(r"^EVD-[A-Z0-9][A-Z0-9-]*$")
RES_ID_RE = re.compile(r"^RES-[A-Z0-9][A-Z0-9-]*$")
SHA_RE = re.compile(r"^[0-9a-f]{40}$")

errors: list[str] = []


def fail(message: str) -> None:
    errors.append(message)


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def parse_md_table(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        return []

    rows: list[list[str]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        rows.append(cells)

    if len(rows) < 2:
        return []

    header = rows[0]
    data = [row for row in rows[1:] if not all(set(cell) <= {"-", ":"} for cell in row)]
    return [dict(zip(header, row)) for row in data if len(row) == len(header)]


def main() -> int:
    schema = load_json(SCHEMA)
    data = load_json(STATUS)

    try:
        jsonschema.Draft7Validator(schema, format_checker=jsonschema.FormatChecker()).validate(data)
    except jsonschema.ValidationError as exc:
        path = "/".join(map(str, exc.absolute_path))
        fail(f"schema: {exc.message}" + (f" at {path}" if path else ""))

    if errors:
        return report()

    residual_rows = parse_md_table(RESIDUALS)
    claim_rows = parse_md_table(CLAIMS)
    attribution_rows = parse_md_table(ATTRIBUTION)
    security_rows = parse_md_table(SECURITY)
    project_rows = parse_md_table(PROJECT_STATUS)

    known_residuals = {row.get("ID", "") for row in residual_rows if row.get("ID")}
    resolved_residuals = {
        row["ID"]: row
        for row in residual_rows
        if row.get("Status", "").upper() == "RESOLVED"
    }

    all_evidence: dict[str, dict] = {}
    all_claims: dict[str, dict] = {}

    for component in data["components"]:
        cid = component["id"]
        local_evidence = {entry["id"]: entry for entry in component["evidence"]}

        for entry in component["evidence"]:
            eid = entry["id"]
            if eid in all_evidence:
                fail(f"duplicate evidence id: {eid}")
            all_evidence[eid] = entry

            if not SHA_RE.fullmatch(entry["sha"]):
                fail(f"{cid}/{eid}: invalid exact source SHA")

        for residual_id in component["residuals"]:
            if not RES_ID_RE.fullmatch(residual_id):
                fail(f"{cid}: invalid residual id {residual_id!r}")
            if residual_id not in known_residuals:
                fail(f"{cid}: residual {residual_id} not found in RESIDUALS.md")

        for claim in component["claims"]:
            claim_id = claim["id"]
            if claim_id in all_claims:
                fail(f"duplicate claim id: {claim_id}")
            all_claims[claim_id] = claim

            for evidence_id in claim["evidenceIds"]:
                if not EVD_ID_RE.fullmatch(evidence_id):
                    fail(f"{cid}/{claim_id}: invalid evidence id {evidence_id!r}")
                if evidence_id not in local_evidence:
                    fail(
                        f"{cid}/{claim_id}: evidence {evidence_id} "
                        "is not defined in this component"
                    )

        if component["status"] in {"IMPLEMENTED", "TESTED", "VERIFIED", "BLOCKED", "RESIDUAL"}:
            if not component["evidence"]:
                fail(f"{cid}: {component['status']} without evidence")

        if component["status"] in {"TESTED", "VERIFIED"}:
            if not any(entry["type"] == "ci-run" for entry in component["evidence"]):
                fail(f"{cid}: {component['status']} without CI-run evidence")

        if component["status"] == "VERIFIED":
            if component["residuals"]:
                fail(f"{cid}: VERIFIED while residuals are still referenced")
            independent = [
                entry
                for entry in component["evidence"]
                if entry.get("independent") is True
                and entry.get("type") in {"audit", "reproduction"}
            ]
            if not independent:
                fail(f"{cid}: VERIFIED without independent audit/reproduction evidence")

    # Markdown claim matrix must match the machine matrix.
    markdown_claims = {
        row.get("Claim ID", ""): row.get("Evidence IDs", "")
        for row in claim_rows
        if row.get("Claim ID")
    }
    if set(markdown_claims) != set(all_claims):
        fail(
            "CLAIM_TO_EVIDENCE.md claim IDs do not match status.json: "
            f"markdown={sorted(markdown_claims)} machine={sorted(all_claims)}"
        )
    for claim_id, claim in all_claims.items():
        expected = ", ".join(claim["evidenceIds"])
        actual = markdown_claims.get(claim_id, "")
        actual_ids = [item.strip() for item in actual.split(",") if item.strip()]
        if actual_ids != claim["evidenceIds"]:
            fail(
                f"{claim_id}: markdown evidence IDs {actual_ids!r} "
                f"do not match status.json {claim['evidenceIds']!r}"
            )

    # Human-readable snapshot must contain the same component IDs and statuses.
    markdown_components = {
        row.get("ID", ""): row.get("Status", "")
        for row in project_rows
        if row.get("ID")
    }
    machine_components = {
        component["id"]: component["status"] for component in data["components"]
    }
    if markdown_components != machine_components:
        fail(
            "PROJECT_STATUS.md snapshot does not match status.json: "
            f"markdown={markdown_components!r} machine={machine_components!r}"
        )

    # RESOLVED residuals need evidence pointing to a real machine evidence ID.
    for residual_id, row in resolved_residuals.items():
        evidence_id = row.get("Evidence", "")
        if not evidence_id or evidence_id not in all_evidence:
            fail(
                f"residual {residual_id} marked RESOLVED but evidence "
                f"{evidence_id!r} is missing in status.json"
            )

    # Attribution rows, when present, require all mandatory fields.
    required_attribution = {"Project", "Version/Commit", "License", "Usage", "URL"}
    for row in attribution_rows:
        missing = [key for key in required_attribution if not row.get(key)]
        if missing:
            fail(f"attribution {row.get('Project', '<unknown>')}: missing {missing}")

    # Security rows, when present, require all mandatory fields.
    required_security = {"ID", "Finding", "Severity", "Status", "Evidence", "Owner"}
    for row in security_rows:
        missing = [key for key in required_security if not row.get(key)]
        if missing:
            fail(f"security {row.get('ID', '<unknown>')}: missing {missing}")

    return report()


def report() -> int:
    if errors:
        print("Governance validation FAILED:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("Governance validation PASSED.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
