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
        identity = "|".join(str(row.get(k, "")) for k in ("repository", "source_sha", "source_path", "symbol_kind", "symbol_or_entrypoint", "line_start"))
        candidate_id = "ATC-CAND-" + hashlib.sha256(identity.encode()).hexdigest()[:20].upper()
        category = {
            "owner_domain": owner_category,
            "source_layer": layer,
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
            "category": category,
        })
    summary = {
        "candidate_count": len(candidates),
        "unique_candidate_ids": len({x["candidate_id"] for x in candidates}),
        "owner_domain_counts": dict(sorted(Counter(x["category"]["owner_domain"] for x in candidates).items())),
        "source_layer_counts": dict(sorted(Counter(x["category"]["source_layer"] for x in candidates).items())),
        "category_pair_counts": [
            {"owner_domain": owner, "source_layer": layer, "count": count}
            for (owner, layer), count in sorted(counts.items())
        ],
        "automatically_bound": 0,
        "automatically_excluded": 0,
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
            "completion_rule": "Every candidate still requires review and must be bound to a canonical catalog function or receive an individually evidenced exclusion. Strict reconciliation remains blocked until then."
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
