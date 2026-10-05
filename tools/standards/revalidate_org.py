#!/usr/bin/env python3
"""Org-wide standards revalidation against the current ATC standards SSOT.

Read-only against product repositories. It snapshots the exact checked-out SHA,
validates repository/profile/registry bindings, and emits machine-readable
evidence. It does not fix, merge, release, or deploy anything.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ORG = "A-TownChain-Okosystems"
ROOT = Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "components/atc-standards/registry/repositories.yaml"
STANDARDS = ROOT / "components/atc-standards/registry/standards.yaml"
PROFILES = ROOT / "components/atc-standards/profiles"
OUT = ROOT / "artifacts/standards-revalidation.json"

REPO_RE = re.compile(
    r"- \{name: (?P<name>[^,]+).*?profile: (?P<profile>[^,]+).*?exempt: (?P<exempt>true|false)"
)
STD_RE = re.compile(r"id:\s*(ATC-STD-[A-Z0-9-]+).*?version:\s*['\"]?([0-9]+(?:\.[0-9]+){1,2})")
ID_RE = re.compile(r"\bATC-(?:STD|AAS)-[A-Z0-9-]+\b")
REQ_BLOCK_RE = re.compile(r"required_standards:\s*\n(?P<body>(?:\s+- .*\n?)+)", re.M)


def run(cmd: list[str], cwd: Path | None = None) -> str:
    p = subprocess.run(cmd, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if p.returncode:
        raise RuntimeError(f"{' '.join(cmd)}\n{p.stdout[-4000:]}")
    return p.stdout.strip()


def parse_registry() -> list[dict]:
    rows = []
    for line in REGISTRY.read_text(encoding="utf-8").splitlines():
        m = REPO_RE.search(line)
        if m:
            rows.append(
                {
                    "name": m.group("name").strip(),
                    "profile": m.group("profile").strip(),
                    "exempt": m.group("exempt") == "true",
                }
            )
    if not rows:
        raise RuntimeError("No repository entries found in registry/repositories.yaml")
    return rows


def parse_standard_versions() -> dict[str, str]:
    data = STANDARDS.read_text(encoding="utf-8")
    result = {}
    for m in STD_RE.finditer(data):
        result[m.group(1)] = m.group(2)
    if not result:
        raise RuntimeError("No standards found in registry/standards.yaml")
    return result


def parse_profile_required(profile: str) -> list[str]:
    path = PROFILES / f"{profile}.yaml"
    if not path.is_file():
        return []
    data = path.read_text(encoding="utf-8")
    m = REQ_BLOCK_RE.search(data)
    if not m:
        return []
    return sorted(set(ID_RE.findall(m.group("body"))))


def parse_repo_standards(path: Path) -> tuple[set[str], dict[str, str]]:
    if not path.is_file():
        return set(), {}
    data = path.read_text(encoding="utf-8")
    ids = set(ID_RE.findall(data))
    versions: dict[str, str] = {}
    for line in data.splitlines():
        ids_on_line = ID_RE.findall(line)
        vm = re.search(r"version:\s*['\"]?([0-9]+(?:\.[0-9]+){1,2})", line)
        if vm:
            for sid in ids_on_line:
                versions[sid] = vm.group(1)
    return ids, versions


def main() -> int:
    registry_rows = parse_registry()
    standard_versions = parse_standard_versions()
    evidence = {
        "schema": "ATC-STANDARDS-REVALIDATION-001",
        "version": "1.0.0",
        "standards_ssot": str(STANDARDS.relative_to(ROOT)),
        "repository_registry_ssot": str(REGISTRY.relative_to(ROOT)),
        "runner_sha": os.environ.get("GITHUB_SHA", "LOCAL"),
        "repositories": [],
    }

    work = Path(tempfile.mkdtemp(prefix="atc-standards-revalidation-"))
    failures = 0
    try:
        for row in registry_rows:
            name = row["name"]
            item = {
                "repository": name,
                "profile": row["profile"],
                "exempt": row["exempt"],
                "status": "UNANALYZED",
                "checks": {},
            }
            if row["exempt"]:
                item["status"] = "EXEMPT"
                item["checks"]["registry"] = "EXEMPT"
                evidence["repositories"].append(item)
                continue

            target = work / name.replace("/", "__")
            url = f"https://github.com/{ORG}/{name}.git"
            try:
                run(["git", "clone", "--depth", "1", url, str(target)])
                sha = run(["git", "rev-parse", "HEAD"], target)
                branch = run(["git", "branch", "--show-current"], target)
                item["exact_sha"] = sha
                item["ref"] = branch or "detached"

                required = parse_profile_required(row["profile"])
                declared, declared_versions = parse_repo_standards(target / ".atc/standards.yaml")
                item["checks"]["repository_sha"] = "PASS"
                item["checks"]["standards_manifest"] = "PASS" if declared else "FAIL"
                item["checks"]["required_profile_standards"] = (
                    "PASS" if all(x in declared for x in required) else "FAIL"
                )
                unknown = sorted(x for x in declared if x not in standard_versions)
                item["checks"]["registry_ids"] = "FAIL" if unknown else "PASS"
                item["unknown_standard_ids"] = unknown
                drift = []
                for sid, repo_ver in declared_versions.items():
                    central_ver = standard_versions.get(sid)
                    if central_ver and central_ver != repo_ver:
                        drift.append({"id": sid, "repo": repo_ver, "central": central_ver})
                item["version_drift"] = drift
                item["checks"]["version_alignment"] = "FAIL" if drift else "PASS"

                required_missing = sorted(x for x in required if x not in declared)
                item["required_missing"] = required_missing

                governance = [
                    ".atc/repository.yaml",
                    ".atc/lifecycle.yaml",
                    ".atc/ownership.yaml",
                    ".github/CODEOWNERS",
                ]
                missing_governance = [p for p in governance if not (target / p).is_file()]
                item["missing_governance_files"] = missing_governance
                item["checks"]["governance_set"] = "FAIL" if missing_governance else "PASS"

                blocking = [
                    v for k, v in item["checks"].items()
                    if k != "repository_sha" and v == "FAIL"
                ]
                if blocking:
                    item["status"] = "FAIL"
                    failures += 1
                else:
                    item["status"] = "PASS"

                # MANUAL/HYBRID checks are intentionally not converted into PASS.
                item["manual_hybrid"] = "NOT_EVALUATED_BY_THIS_GATE"
            except Exception as exc:
                item["status"] = "FAIL"
                item["error"] = str(exc)
                failures += 1

            evidence["repositories"].append(item)
    finally:
        shutil.rmtree(work, ignore_errors=True)

    evidence["summary"] = {
        "total_registry_entries": len(registry_rows),
        "exempt": sum(1 for x in evidence["repositories"] if x["status"] == "EXEMPT"),
        "pass": sum(1 for x in evidence["repositories"] if x["status"] == "PASS"),
        "fail": sum(1 for x in evidence["repositories"] if x["status"] == "FAIL"),
        "manual_hybrid_not_evaluated": sum(
            1 for x in evidence["repositories"] if x.get("manual_hybrid") == "NOT_EVALUATED_BY_THIS_GATE"
        ),
        "result": "FAIL" if failures else "PASS",
    }

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(evidence, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(evidence["summary"], indent=2))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
