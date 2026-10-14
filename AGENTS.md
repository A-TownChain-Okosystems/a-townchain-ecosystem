# Repository agent instructions

## Authority and canonical catalogs

This repository is the A-TownChain ecosystem integration, architecture, compliance, and evidence control plane. It is not the implementation source of truth for blockchain runtime, VM, consensus, wallet, node, SDK, kernel, compiler, or AI runtime code.

Before changing a component or function catalog, read and reconcile these canonical artifacts:

- `docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json` — central component/function index and source bindings.
- `docs/architecture/ATC-COMPONENT-FUNCTION-SPECIFICATION-001.json` — required catalog fields and contract semantics.
- `docs/architecture/ATC-CATALOG-SOURCE-REGISTRY-001.json` — repository-local catalog routing and canonical ownership.
- `docs/architecture/ATC-PRODUCT-BOUNDARY-INTERFACE-CONTRACTS-001.json` — cross-component boundary contracts.
- `docs/audit/ATC-UNBOUND-SYMBOL-CLASSIFICATION-001.json` — triage/routing evidence only; it is not a completed classification.
- `docs/audit/ATC-CATALOG-RECONCILIATION-STATUS-001.json` — recorded reconciliation status; check its provenance against the exact HEAD being tested.

## Mandatory source-to-catalog workflow

1. Inspect the exact current source tree and record repository, full 40-character HEAD SHA, source path, symbol kind, symbol name/signature, line, and file hash where available.
2. Give every in-scope runtime function a stable catalog function ID and bind it to the canonical owning repository's local catalog; reference it centrally where required.
3. For every out-of-scope candidate, record an individual, reviewable exclusion with its exact source evidence and a concrete rationale. Repository-level assumptions, names, file-hash similarity, and lexical matches alone are not sufficient.
4. Keep unresolved candidates explicitly `RESIDUAL` or `BLOCKED`. Do not relabel them as mapped, excluded, implemented, or verified merely to make counts reconcile.
5. Do not confuse a lexical source match with behavioral verification. Functional verification remains `NOT_PERFORMED` until exact-HEAD test evidence is linked and inspected.
6. Keep unknown consensus, finality, quorum, fork-choice, transaction-domain, bytecode-format, cryptographic, and SSOT decisions `BLOCKED` until an authoritative versioned decision and evidence exist.
7. Follow `Standalone First, Ecosystem Second`: update the canonical repository-local implementation/catalog first; the ecosystem repository records routing, interface, compliance, and evidence, not a duplicate implementation or second SSOT.
8. Run the structural validator, scanner tests, exact-HEAD organization census, and strict reconciliation with `--require-complete`. Never remove or weaken a gate to obtain a green result.
9. Keep PR #115 open and Draft until the strict gate passes at an identified exact HEAD. Do not merge, enable auto-merge, force-push, or create no-op/touch commits.

## Reporting requirements

Report separately: `IMPLEMENTED`, `SOURCE_MAPPED`, `VERIFIED`, `EXCLUDED`, `RESIDUAL`, and `BLOCKED`. Include exact commit SHA, workflow run/job/step, exit code, and logs/artifact references for any claimed result. If evidence is incomplete or unavailable, fail closed and state the limitation. Do not claim that a partial triage artifact completes the 6,383 currently unbound candidates or the remaining catalog findings.
