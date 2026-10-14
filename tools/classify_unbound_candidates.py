#!/usr/bin/env python3
"""Assign conservative triage categories to every unbound source candidate.

Categories route review; they do not create contracts, source bindings, exclusions,
or functional verification. Canonical ownership remains governed by the repo map.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path

OWNER_CATEGORIES = {
    "globus-os": ("GLOBUSOS_OS_KERNEL_RUNTIME", "globus-os/modules/atc-shivacore/kernel/"),
    "a-townchain": ("BLOCKCHAIN_CORE", "a-townchain/components/{vm,algorithm}"),
    "genesis-engine": ("GENESIS_ENGINE", "repository-local registered catalog; ownership review required"),
    "aurora-ai": ("AURORA_AI", "repository-local Aurora catalog; ownership review required"),
    "a-townchain-os": ("OS_INTEGRATION_ORCHESTRATION", "route by subsystem to canonical implementation"),
    "a-townchain-os-docs": ("OS_DOCUMENTATION", "documentation/support only; not implementation SSOT"),
    "atc-engineering": ("ENGINEERING_GOVERNANCE", "engineering governance and evidence"),
    "atc-ide": ("DEVELOPER_IDE", "developer tooling"),
    "atc-standards": ("NORMATIVE_STANDARDS", "normative registry; no duplicate implementation"),
    "atclang": ("ATCLANG_COMPILER_LANGUAGE", "language/compiler ownership review"),
    "genesis-chronicles": ("GENESIS_CHRONICLES", "application/service ownership review"),
    "atc-shivacore": ("SHIVACORE_SPEC_GOVERNANCE", "specification/governance/support; kernel SSOT is globus-os"),
    "atc-marketplace": ("MARKETPLACE_APPLICATION", "application/service"),
    ".github": ("ORG_GOVERNANCE_AUTOMATION", "organization governance automation"),
    "atc-vm": ("VM_SUPPORT_NONCANONICAL", "support/specification only; canonical VM is a-townchain/components/vm"),
    "a-townchain-ecosystem": ("ECOSYSTEM_CONTROL_PLANE", "integration/compliance/evidence only"),
    "atc-node": ("BLOCKCHAIN_NODE", "node implementation; check TX-V2 and canonical boundaries"),
    "atc-launchpad": ("LAUNCHPAD_APPLICATION", "application/service"),
    "genesis-franchise-factory": ("FRANCHISE_FACTORY", "application/tooling"),
    "atc-zkp": ("ZERO_KNOWLEDGE_PROOFS", "ZK component ownership review"),
    "atc-sdk": ("SDK", "SDK API and cross-language compatibility review"),
    "atc-algorithm": ("ALGORITHM_SUPPORT_NONCANONICAL", "support/specification only; canonical algorithm is a-townchain/components/algorithm"),
    "atc-compute": ("AI_COMPUTE", "compute/scheduler service"),
}

def source_layer(path: str, language: str) -> tuple[str, str]:
    p = path.replace("\\", "/").lower()
    parts = [x for x in p.split("/") if x]
    filename = parts[-1] if parts else ""
    dirs = set(parts[:-1])
    if any(x in dirs for x in ("tests", "test", "__tests__", "benches", "bench", "fixtures", "fixture", "fuzz", "fuzzing")) or filename.startswith("test_") or filename.endswith(("_test.py", "_test.rs", "_tests.py")):
        return "TEST_OR_VERIFICATION_SUPPORT", "Path/name matches a test-like surface; candidate remains unbound and is not auto-excluded."
    if any(x in dirs for x in ("examples", "example", "samples", "sample")):
        return "EXAMPLE_OR_SAMPLE", "Path is under an example/sample directory; review whether normative behavior is involved."
    if any(x in dirs for x in ("docs", "doc", "documentation")) or filename.endswith((".md", ".rst", ".adoc")):
        return "DOCUMENTATION_SUPPORT", "Documentation path; not presumed to be runtime implementation."
    if any(x in p for x in ("kernel", "microkernel", "shivacore")):
        return "KERNEL_TCB", "Path contains an explicit kernel/TCB marker; canonical ownership still needs confirmation."
    if any(x in p for x in ("consensus", "finality", "fork_choice", "fork-choice", "quorum")):
        return "CONSENSUS_ALGORITHM", "Path contains consensus/finality markers; unresolved normative rules remain BLOCKED."
    if any(x in p for x in ("bytecode", "interpreter", "/vm/", "virtual_machine")):
        return "VM_OR_BYTECODE_RUNTIME", "Path contains VM/bytecode markers; canonical VM ownership must be respected."
    if any(x in p for x in ("wallet", "transaction", "signing", "signature", "cryptograph", "/crypto", "secp256k1")):
        return "CRYPTO_TRANSACTION", "Path contains transaction/cryptography markers; check protocol contracts and canonical owner."
    if any(x in p for x in ("network", "p2p", "peer", "transport", "/rpc", "http_server", "router", "endpoint")):
        return "NETWORKING_OR_API", "Path contains network/API markers; service boundary review required."
    if any(x in p for x in ("agent", "llm", "inference", "model", "aurora", "memory")):
        return "AI_AGENT_INTELLIGENCE", "Path contains AI/agent/model markers; AI must not gain kernel/consensus/wallet authority."
    if any(x in p for x in ("storage", "store", "database", "persistence", "persist")):
        return "STORAGE_PERSISTENCE", "Path contains storage/persistence markers; verify cryptographic and durability requirements."
    if any(x in p for x in ("workflow", "scripts/", "tools/", "build.rs", "ci/")):
        return "BUILD_CI_TOOLING", "Path appears to contain build, CI, or developer tooling."
    if language in ("typescript", "javascript") and any(x in p for x in ("frontend", "ui", "web", "aistudio", "tsx", "components/")):
        return "USER_INTERFACE_OR_WEB", "Web-language/path heuristic; inspect exact symbol before deciding ownership."
    if any(x in p for x in ("compiler", "parser", "lexer", "codegen", "atclang")):
        return "LANGUAGE_COMPILER_TOOLCHAIN", "Path contains compiler/toolchain markers."
    return "GENERAL_RUNTIME_REVIEW", "No sufficiently specific path category; manual review required."

def execution_model(path: str, language: str, symbol: str) -> tuple[str, str]:
    """Classify how the candidate is executed: library surface vs runtime entrypoint.

    Heuristic is conservative and machine-checkable; it never upgrades a binding
    or exclusion decision and always records its basis.
    """
    p = path.replace("\\", "/").lower()
    parts = [x for x in p.split("/") if x]
    filename = parts[-1] if parts else ""
    dirs = set(parts[:-1])
    sym = (symbol or "").lower()
    runtime_path = (
        filename in ("main.rs", "main.py", "main.ts", "main.tsx", "main.jsx", "main.go", "__main__.py")
        or "bin" in dirs or "cmd" in dirs
        or "src/main" in p
    )
    runtime_symbol = sym in {"main", "serve", "run", "start_server", "run_server", "start", "boot", "entrypoint"}
    runtime_dir = any(x in dirs for x in ("bin", "cmd", "apps", "app", "services", "service", "daemon", "server"))
    if runtime_path or runtime_symbol or (runtime_dir and sym in {"main", "serve", "run", "start", "boot"}):
        return "RUNTIME", "Path/symbol matches a standalone process or service entrypoint pattern (main/bin/cmd/service marker)."
    if any(x in dirs for x in ("examples", "example", "samples", "sample")) or filename.startswith("test_") or filename.endswith(("_test.py", "_test.rs")):
        return "NOT_EXECUTABLE_INVENTORY", "Test/example surface; execution model is not a product runtime or library API."
    return "LIBRARY", "No entrypoint marker; candidate is a library/internal surface consumed by other code (default, conservative)."


def inspect_source_location(row: dict, repos: dict) -> dict:
    """Verify a candidate against the exact-SHA census symbol inventory.

    This confirms a lexical source location only. It deliberately does not decide
    that a symbol is a catalog function or that it is eligible for exclusion.
    """
    repository = row.get("repository", "")
    repo = repos.get(repository) or {}
    expected_sha = row.get("source_sha")
    report = repo.get("report") or {}
    repo_sha = repo.get("source_sha")
    path = row.get("source_path", "")
    file_record = next((item for item in report.get("files", []) if item.get("path") == path), None)
    if not repo or not report:
        return {"status": "SOURCE_EVIDENCE_UNAVAILABLE", "reason": "Repository or lexical census report is unavailable.", "repository_sha": repo_sha}
    if not expected_sha or expected_sha != repo_sha:
        return {"status": "SOURCE_SHA_MISMATCH", "reason": "Candidate SHA does not equal the exact census checkout SHA.", "repository_sha": repo_sha}
    if file_record is None:
        return {"status": "SOURCE_PATH_NOT_FOUND_IN_CENSUS", "reason": "Exact source path is absent from the census file inventory.", "repository_sha": repo_sha}
    candidate_file_hash = row.get("source_file_sha256")
    census_file_hash = file_record.get("file_sha256")
    if not isinstance(candidate_file_hash, str) or not candidate_file_hash:
        return {"status": "SOURCE_FILE_HASH_MISSING", "reason": "Candidate does not carry a file hash; exact source evidence cannot be confirmed.", "repository_sha": repo_sha, "source_path": path, "census_file_sha256": census_file_hash}
    if not isinstance(census_file_hash, str) or not census_file_hash:
        return {"status": "CENSUS_FILE_HASH_MISSING", "reason": "The exact-SHA census file inventory does not carry a file hash.", "repository_sha": repo_sha, "source_path": path}
    if census_file_hash != candidate_file_hash:
        return {"status": "SOURCE_FILE_HASH_MISMATCH", "reason": "Candidate file hash differs from the file hash in the exact-SHA census.", "repository_sha": repo_sha, "source_path": path, "census_file_sha256": census_file_hash}
    line_start = row.get("line_start")
    if not isinstance(line_start, int) or isinstance(line_start, bool) or line_start < 1:
        return {"status": "INVALID_SYMBOL_POSITION", "reason": "Candidate line_start must be a positive integer.", "repository_sha": repo_sha, "source_path": path, "source_file_sha256": census_file_hash}
    symbol_lists = ("candidate_public_symbols", "candidate_internal_symbols", "candidate_test_symbols")
    observed = []
    for field in symbol_lists:
        for symbol in file_record.get(field, []):
            symbol_line = symbol.get("line_start")
            if (symbol.get("kind", "function") == row.get("symbol_kind", "unknown")
                and symbol.get("name", "") == row.get("symbol_or_entrypoint", "")
                and isinstance(symbol_line, int) and not isinstance(symbol_line, bool)
                and symbol_line == line_start):
                observed.append((field, symbol))
    if not observed:
        return {"status": "SYMBOL_NOT_FOUND_AT_RECORDED_LOCATION", "reason": "No exact kind/name/line match exists in the census inventory.", "repository_sha": repo_sha, "source_path": path, "source_file_sha256": census_file_hash}
    if len(observed) != 1:
        return {"status": "AMBIGUOUS_SYMBOL_MATCH", "reason": "More than one census symbol matches the exact kind/name/line key.", "repository_sha": repo_sha, "source_path": path, "source_file_sha256": census_file_hash, "match_count": len(observed)}
    field, symbol = observed[0]
    return {
        "status": "SOURCE_LOCATION_CONFIRMED",
        "reason": "Exact repository SHA, path, file hash, symbol kind/name, and line match the lexical census. This is not semantic review or functional verification.",
        "repository_sha": repo_sha,
        "source_path": path,
        "source_file_sha256": file_record.get("file_sha256"),
        "symbol_kind": symbol.get("kind", "unknown"),
        "symbol_or_entrypoint": symbol.get("name", ""),
        "line_start": int(symbol.get("line_start", 0)),
        "signature": symbol.get("signature", ""),
        "census_symbol_surface": field,
    }

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--reconciliation", required=True, type=Path)
    ap.add_argument("--census", required=True, type=Path)
    ap.add_argument("--output", required=True, type=Path)
    args = ap.parse_args()
    reconciliation = json.loads(args.reconciliation.read_text(encoding="utf-8"))
    census = json.loads(args.census.read_text(encoding="utf-8"))
    repos = {x.get("repository"): x for x in census.get("repositories", []) if x.get("repository")}
    candidates = []
    counts = Counter()
    for row in reconciliation.get("unmatched_candidates", []):
        repository = row.get("repository", "")
        repo_name = repository.rsplit("/", 1)[-1]
        owner_category, canonical_route = OWNER_CATEGORIES.get(repo_name, ("REPOSITORY_SPECIFIC_REVIEW", "No registered routing rule; manual owner assignment required"))
        layer, rationale = source_layer(row.get("source_path", ""), row.get("language", ""))
        model, model_rationale = execution_model(row.get("source_path", ""), row.get("language", ""), row.get("symbol_or_entrypoint", ""))
        source_evidence = inspect_source_location(row, repos)
        identity = "|".join(str(row.get(k, "")) for k in ("repository", "source_sha", "source_path", "symbol_kind", "symbol_or_entrypoint", "line_start"))
        candidate_id = "ATC-CAND-" + hashlib.sha256(identity.encode()).hexdigest()[:20].upper()
        category = {
            "owner_domain": owner_category,
            "source_layer": layer,
            "execution_model": model,
            "execution_model_basis": model_rationale,
            "classification_status": "PROVISIONAL_TRIAGE",
            "classification_basis": "repository ownership map + conservative source-path/language heuristic",
            "canonical_route": canonical_route,
            "rationale": rationale,
            "requires_human_review": True,
            "binding_created": False,
            "exclusion_created": False,
        }
        counts[(owner_category, layer)] += 1
        candidates.append({
            "candidate_id": candidate_id,
            "repository": repository,
            "repository_role": (repos.get(repository) or {}).get("expected_role"),
            "source_sha": row.get("source_sha"),
            "source_path": row.get("source_path"),
            "source_file_sha256": row.get("source_file_sha256"),
            "language": row.get("language"),
            "symbol_kind": row.get("symbol_kind"),
            "symbol_or_entrypoint": row.get("symbol_or_entrypoint"),
            "line_start": row.get("line_start"),
            "signature": row.get("signature", ""),
            "original_status": "RESIDUAL",
            "original_finding": "CANDIDATE_NOT_BOUND_TO_CATALOG_FUNCTION",
            "decision": "UNRESOLVED",
            "decision_reason": "No canonical function binding or individually evidenced exclusion has been established. This candidate remains a blocking residual even when its lexical source location is confirmed.",
            "source_evidence": source_evidence,
            "category": category,
        })
    summary = {
        "candidate_count": len(candidates),
        "unique_candidate_ids": len({x["candidate_id"] for x in candidates}),
        "owner_domain_counts": dict(sorted(Counter(x["category"]["owner_domain"] for x in candidates).items())),
        "source_layer_counts": dict(sorted(Counter(x["category"]["source_layer"] for x in candidates).items())),
        "execution_model_counts": dict(sorted(Counter(x["category"]["execution_model"] for x in candidates).items())),
        "category_pair_counts": [
            {"owner_domain": owner, "source_layer": layer, "count": count}
            for (owner, layer), count in sorted(counts.items())
        ],
        "automatically_bound": 0,
        "automatically_excluded": 0,
        "decision_counts": dict(sorted(Counter(x["decision"] for x in candidates).items())),
        "source_location_confirmed": sum(1 for x in candidates if x["source_evidence"]["status"] == "SOURCE_LOCATION_CONFIRMED"),
        "source_location_unconfirmed": sum(1 for x in candidates if x["source_evidence"]["status"] != "SOURCE_LOCATION_CONFIRMED"),
        "functional_verification": "NOT_PERFORMED",
        "completion_gate": "BLOCKED",
    }
    output = {
        "schema_id": "ATC-UNBOUND-CANDIDATE-CATEGORIZATION-001",
        "schema_version": "1.0.0",
        "status": "TRIAGE_ONLY",
        "purpose": "Assign each unbound census candidate a review-routing category; categories do not constitute canonical function contracts, bindings, exclusions, or functional verification.",
        "provenance": {
            "reconciliation_input": args.reconciliation.name,
            "census_input": args.census.name,
            "census_repository_count": len(repos),
            "census_status": census.get("status"),
            "catalog_reconciliation_status": reconciliation.get("status"),
            "catalog_completion_gate": reconciliation.get("completion_gate"),
        },
        "category_semantics": {
            "owner_domain": "Provisional domain inferred from repository's registered role; not an implementation ownership decision.",
            "source_layer": "Review queue inferred from source path/language; not proof of runtime behavior or exclusion eligibility.",
            "candidate_id": "Stable SHA-256-derived identifier over repository, source SHA, path, symbol kind/name and line start.",
            "decision": "Every candidate is explicitly BOUND, EXCLUDED, or UNRESOLVED. This classifier may only emit UNRESOLVED; a confirmed lexical location is not a binding or exclusion.",
            "source_evidence": "SOURCE_LOCATION_CONFIRMED means exact repo SHA, path, file hash, symbol kind/name and line match the census inventory. Missing/mismatched evidence is recorded as a residual blocker.",
            "completion_rule": "Every candidate must be bound to a canonical catalog function or receive an individually evidenced exclusion. UNRESOLVED candidates block completion; functional verification remains a separate gate."
        },
        "summary": summary,
        "candidates": candidates,
    }
    if summary["candidate_count"] != reconciliation.get("summary", {}).get("candidate_symbols_unbound"):
        raise SystemExit("candidate count mismatch against reconciliation summary")
    if summary["unique_candidate_ids"] != summary["candidate_count"]:
        raise SystemExit("candidate ID collision detected")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": output["status"], **summary, "output": str(args.output)}, sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
