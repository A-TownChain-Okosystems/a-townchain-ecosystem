#!/usr/bin/env python3
"""Deterministic Governance-100 assessment for the ecosystem repository.

Phase 1 implements G01-G07 as repository-data integrity checks. G08-G16 are
reported as PENDING until their real implementation/evidence exists. The
checker never converts a missing control into PASS.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:
    raise SystemExit("PyYAML is required; install components/atc-standards/requirements.txt") from exc


ROOT = Path(__file__).resolve().parents[3]
STD_ROOT = ROOT / "components" / "atc-standards"
REGISTRY = STD_ROOT / "registry" / "standards.yaml"
VERSIONS = STD_ROOT / "registry" / "versions.yaml"
FINDINGS = STD_ROOT / "registry" / "findings.yaml"
LIFECYCLE = STD_ROOT / ".atc" / "lifecycle.yaml"
EVIDENCE = STD_ROOT / ".atc" / "evidence" / "evidence.yaml"
CHANGE_STANDARD = STD_ROOT / "standards" / "governance-core" / "ATC-STD-CHANGE-001.md"
CHANGE_REQUESTS = STD_ROOT / "change-requests"


class Gate:
    def __init__(self, gate_id: str, name: str) -> None:
        self.id = gate_id
        self.name = name
        self.ok = True
        self.details: list[str] = []

    def fail(self, message: str) -> None:
        self.ok = False
        self.details.append(message)

    def note(self, message: str) -> None:
        self.details.append(message)

    @property
    def status(self) -> str:
        return "PASS" if self.ok else "FAIL"


def load_yaml(path: Path, gate: Gate) -> Any:
    if not path.is_file():
        gate.fail(f"missing file: {path.relative_to(ROOT)}")
        return None
    try:
        value = yaml.safe_load(path.read_text(encoding="utf-8"))
    except Exception as exc:
        gate.fail(f"invalid YAML in {path.relative_to(ROOT)}: {exc}")
        return None
    if value is None:
        gate.fail(f"empty YAML: {path.relative_to(ROOT)}")
    return value


def parse_frontmatter(path: Path) -> dict[str, Any] | None:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---"):
        return None
    parts = text.split("---", 2)
    if len(parts) < 3:
        return None
    try:
        data = yaml.safe_load(parts[1])
    except Exception:
        return None
    if not isinstance(data, dict):
        return None
    standard = data.get("standard")
    return standard if isinstance(standard, dict) else data


def git_head() -> str:
    try:
        return subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip()
    except Exception:
        return "unknown"


def git_commits() -> list[str]:
    try:
        out = subprocess.check_output(
            ["git", "log", "-20", "--format=%H%x09%s"],
            cwd=ROOT,
            text=True,
        )
        return [line for line in out.splitlines() if line.strip()]
    except Exception:
        return []


def approved_entry(entries: Any, current_version: str) -> bool:
    if not isinstance(entries, list):
        return False
    for item in entries:
        if not isinstance(item, dict):
            continue
        version = str(item.get("version", ""))
        change = str(item.get("change", ""))
        if version == f"{current_version}-Approval":
            return True
        if version == current_version and re.search(
            r"approve|approved|freigab|freigegeben|APPROVED",
            change,
            re.IGNORECASE,
        ):
            return True
    return False


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true", help="emit machine-readable output")
    parser.add_argument(
        "--phase1-only",
        action="store_true",
        help="evaluate G01-G07 only; useful while later gates are being implemented",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="require every G01-G16 gate to PASS",
    )
    args = parser.parse_args()

    gates = [
        Gate("G01", "Standards Registry"),
        Gate("G02", "Lifecycle"),
        Gate("G03", "Metadata"),
        Gate("G04", "Approval Chain"),
        Gate("G05", "Evidence"),
        Gate("G06", "Change Control"),
        Gate("G07", "Repository Traceability"),
        Gate("G08", "CI Enforcement"),
        Gate("G09", "Security Enforcement"),
        Gate("G10", "Runtime Governance"),
        Gate("G11", "On-Chain Governance"),
        Gate("G12", "Persistence / Restart"),
        Gate("G13", "Audit Separation"),
        Gate("G14", "Release Evidence"),
        Gate("G15", "Reproducibility"),
        Gate("G16", "Open Findings"),
    ]
    by_id = {gate.id: gate for gate in gates}

    registry = load_yaml(REGISTRY, by_id["G01"])
    versions = load_yaml(VERSIONS, by_id["G01"])
    if not isinstance(registry, dict) or not isinstance(registry.get("standards"), list):
        by_id["G01"].fail("registry/standards.yaml must contain a standards list")
        standards: list[dict[str, Any]] = []
    else:
        standards = [x for x in registry["standards"] if isinstance(x, dict)]

    version_map = versions.get("versions", {}) if isinstance(versions, dict) else {}
    if not isinstance(version_map, dict):
        by_id["G01"].fail("registry/versions.yaml must contain a versions mapping")
        version_map = {}

    ids: set[str] = set()
    allowed_status = {
        "idea", "proposed", "draft", "validated", "reviewed", "candidate",
        "approved", "effective", "maintained", "deprecated", "retired",
    }

    for entry in standards:
        sid = str(entry.get("id", "")).strip()
        if not sid:
            by_id["G01"].fail("registry entry without id")
            continue
        if sid in ids:
            by_id["G01"].fail(f"duplicate registry id: {sid}")
        ids.add(sid)

        status = str(entry.get("status", "")).lower()
        if status not in allowed_status:
            by_id["G01"].fail(f"{sid}: invalid registry status {status!r}")

        file_ref = entry.get("file")
        if not file_ref:
            by_id["G01"].fail(f"{sid}: missing file binding")
        else:
            target = STD_ROOT / str(file_ref)
            if not target.is_file():
                by_id["G01"].fail(f"{sid}: registry file missing: {file_ref}")

        version = str(entry.get("version", "")).strip()
        entries = version_map.get(sid)
        if not isinstance(entries, list):
            by_id["G01"].fail(f"{sid}: no versions.yaml history")
        elif not any(
            isinstance(v, dict) and str(v.get("version", "")) == version
            for v in entries
        ):
            by_id["G01"].fail(f"{sid}: current registry version {version!r} absent from versions.yaml")

    by_id["G01"].note(f"registry entries: {len(standards)}")
    by_id["G01"].note(f"unique ids: {len(ids)}")

    lifecycle = load_yaml(LIFECYCLE, by_id["G02"])
    if isinstance(lifecycle, dict):
        state = lifecycle.get("lifecycle")
        if not isinstance(state, dict):
            by_id["G02"].fail("lifecycle.yaml must contain a lifecycle mapping")
        else:
            stage = state.get("stage")
            history = state.get("history")
            if not stage:
                by_id["G02"].fail("lifecycle.stage missing")
            if not isinstance(history, list) or not history:
                by_id["G02"].fail("lifecycle.history must be a non-empty list")
            else:
                dates = []
                for item in history:
                    if not isinstance(item, dict) or not item.get("stage") or not item.get("date"):
                        by_id["G02"].fail("lifecycle.history contains an incomplete entry")
                    else:
                        dates.append(str(item["date"]))
                if dates != sorted(dates):
                    by_id["G02"].fail("lifecycle.history is not deterministic by date")
            by_id["G02"].note(f"repository lifecycle stage: {stage}")

    metadata_required = {
        "id", "title", "version", "status", "category", "authority", "owner",
        "created", "updated", "normative", "classification", "license",
    }
    approved_required = {"effective_date", "review_date", "approved_by"}
    metadata_missing = 0
    approved_missing = 0

    for entry in standards:
        sid = str(entry.get("id", ""))
        ref = entry.get("file")
        if not sid or not ref:
            continue
        target = STD_ROOT / str(ref)
        if not target.is_file():
            continue
        meta = parse_frontmatter(target)
        if meta is None:
            by_id["G03"].fail(f"{sid}: missing/invalid YAML frontmatter")
            metadata_missing += 1
            continue

        missing = sorted(k for k in metadata_required if not meta.get(k) and meta.get(k) is not False)
        if missing:
            metadata_missing += 1
            by_id["G03"].fail(f"{sid}: missing metadata: {', '.join(missing)}")

        status = str(entry.get("status", "")).lower()
        if status in {"approved", "effective", "maintained"}:
            missing_approved = sorted(k for k in approved_required if not meta.get(k))
            if missing_approved:
                approved_missing += 1
                by_id["G03"].fail(
                    f"{sid}: approved metadata incomplete: {', '.join(missing_approved)}"
                )

    by_id["G03"].note(f"metadata-incomplete files: {metadata_missing}")
    by_id["G03"].note(f"approved metadata-incomplete files: {approved_missing}")

    approval_missing = 0
    for entry in standards:
        sid = str(entry.get("id", ""))
        status = str(entry.get("status", "")).lower()
        if status not in {"approved", "effective", "maintained"}:
            continue
        current = str(entry.get("version", ""))
        if not approved_entry(version_map.get(sid), current):
            approval_missing += 1
            by_id["G04"].fail(
                f"{sid}: no approval evidence for current version {current}"
            )
    by_id["G04"].note(f"approval gaps: {approval_missing}")

    evidence = load_yaml(EVIDENCE, by_id["G05"])
    if isinstance(evidence, dict):
        required = {
            "schema_version", "repository", "generated", "generated_by",
            "bound_commit", "implementation", "tests", "security",
            "conformance", "release",
        }
        missing = sorted(k for k in required if k not in evidence)
        if missing:
            by_id["G05"].fail(f"evidence SSOT missing keys: {', '.join(missing)}")
        if evidence.get("repository") != "atc-standards":
            by_id["G05"].fail("evidence.repository is not atc-standards")
        bound = str(evidence.get("bound_commit", ""))
        if bound and not re.fullmatch(r"[0-9a-f]{40}", bound):
            by_id["G05"].fail("evidence.bound_commit is not a full Git SHA")
        by_id["G05"].note(f"evidence bound_commit: {bound or 'missing'}")
        by_id["G05"].note(f"assessment HEAD: {git_head()}")

    if not CHANGE_STANDARD.is_file():
        by_id["G06"].fail("ATC-STD-CHANGE-001 missing")
    if not CHANGE_REQUESTS.is_dir():
        by_id["G06"].fail("change-requests directory missing")
    else:
        scrs = sorted(CHANGE_REQUESTS.glob("SCR-*.md"))
        if not scrs:
            by_id["G06"].fail("no SCR records found")
        else:
            by_id["G06"].note(f"SCR records: {len(scrs)}")
    for sid, entries in version_map.items():
        if not isinstance(entries, list):
            by_id["G06"].fail(f"{sid}: versions history is not a list")
            continue
        for item in entries:
            if not isinstance(item, dict) or not str(item.get("change", "")).strip():
                by_id["G06"].fail(f"{sid}: version entry without change rationale")

    trace_failures = 0
    for entry in standards:
        sid = str(entry.get("id", ""))
        ref = str(entry.get("file", ""))
        target = STD_ROOT / ref if ref else None
        if not target or not target.is_file():
            continue
        meta = parse_frontmatter(target)
        if meta and meta.get("id") != sid:
            trace_failures += 1
            by_id["G07"].fail(f"{sid}: frontmatter id mismatch: {meta.get('id')!r}")
        if meta and str(meta.get("version", "")) != str(entry.get("version", "")):
            trace_failures += 1
            by_id["G07"].fail(
                f"{sid}: registry version {entry.get('version')!r} != file version {meta.get('version')!r}"
            )
        for item in version_map.get(sid, []) if isinstance(version_map.get(sid), list) else []:
            change = str(item.get("change", ""))
            for match in re.findall(r"(?:approval/)?APPROVAL-[A-Za-z0-9._-]+\.md", change):
                approval_path = STD_ROOT / "approval" / match.split("approval/")[-1]
                if not approval_path.is_file():
                    by_id["G07"].fail(f"{sid}: referenced approval artifact missing: {match}")
    by_id["G07"].note(f"traceability mismatches: {trace_failures}")

    # Phase-2/3 controls deliberately remain non-PASS until implemented.
    pending = {
        "G08": "CI enforcement is not yet represented by a strict governance-100 gate.",
        "G09": "Governance-specific security enforcement evidence is not yet implemented.",
        "G10": "Runtime governance enforcement evidence is not yet implemented.",
        "G11": "Deterministic on-chain governance execution evidence is not yet implemented.",
        "G12": "Governance persistence/restart reconstruction evidence is not yet implemented.",
        "G13": "Independent audit separation evidence is not yet implemented.",
        "G14": "Release evidence gate for G01-G16 is not yet implemented.",
        "G15": "Reproducibility comparison of identical assessment inputs is not yet implemented.",
        "G16": "Open-finding zero gate requires full exception-aware finding enforcement.",
    }
    for gate_id, reason in pending.items():
        by_id[gate_id].ok = False
        by_id[gate_id].note(reason)

    findings = load_yaml(FINDINGS, by_id["G16"])
    if isinstance(findings, dict):
        raw = findings.get("findings", [])
        if isinstance(raw, list):
            open_findings = []
            for item in raw:
                if not isinstance(item, dict):
                    continue
                status = str(item.get("status", "")).upper()
                severity = str(item.get("severity", "")).upper()
                if status not in {"RESOLVED", "VERIFIED", "CLOSED", "WONT_FIX"}:
                    open_findings.append((str(item.get("id", "?")), severity, status))
            by_id["G16"].note(f"currently unresolved findings: {len(open_findings)}")
            for fid, severity, status in open_findings[:20]:
                by_id["G16"].note(f"{fid}: {severity} {status}")
    else:
        by_id["G16"].fail("findings registry unavailable")

    active = gates[:7] if args.phase1_only else gates
    passed = sum(g.ok for g in active)
    total = len(active)
    score = round(100 * passed / total, 2) if total else 0.0
    overall_pass = all(g.ok for g in gates)

    result = {
        "schema_version": "1.0.0",
        "commit": git_head(),
        "phase": "phase1" if args.phase1_only else "full",
        "score": score,
        "overall_status": "PASS" if overall_pass else "FAIL",
        "release": "ALLOWED" if overall_pass else "BLOCKED",
        "gates": [
            {"id": g.id, "name": g.name, "status": g.status, "details": g.details}
            for g in active
        ],
        "all_gate_status": {
            g.id: g.status for g in gates
        },
        "recent_commits": git_commits(),
    }

    if args.json:
        print(json.dumps(result, indent=2, ensure_ascii=False))
    else:
        print("A-TOWNCHAIN GOVERNANCE ASSESSMENT")
        print(f"Commit: {result['commit']}")
        print()
        for item in result["gates"]:
            print(f"{item['id']} {item['name']:<30} {item['status']}")
            for detail in item["details"][:5]:
                print(f"   - {detail}")
        print()
        print("-" * 50)
        print(f"PHASE SCORE: {score:.2f}/100")
        print(f"OVERALL: {result['overall_status']}")
        print(f"RELEASE: {result['release']}")
        if not args.phase1_only:
            print("-" * 50)
            print("All G01-G16 PASS is required for 100/100.")

    if args.strict and not overall_pass:
        return 1
    if not args.phase1_only and not overall_pass:
        return 0
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
