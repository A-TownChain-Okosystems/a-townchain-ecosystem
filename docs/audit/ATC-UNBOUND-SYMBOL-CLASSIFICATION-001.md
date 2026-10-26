# ATC Unbound Symbol Classification 001

**Status:** `TRIAGE_ONLY`  
**Source PR:** [a-townchain-ecosystem#115](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/115)  
**Exact source SHA:** `6ad06513754c4a8b0c93e362935f4403a799fbe0`  
**Evidence run:** [38394483388](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38394483388) · [artifact 11810359144](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/38394483388/artifacts/11810359144)

## Current evidence

- **6,387** source-function candidates remain unbound and without a reasoned exclusion.
- **164** candidate symbols are bound to catalog records; this is lexical/source mapping only.
- **6,429** candidates are excluded by current scanner policy; exclusions remain subject to policy review.
- **12,980** candidates total: 6,555 runtime candidates, 3,998 test candidates and 2,427 non-function candidates.
- **33** repositories are represented in the census; **25/26 active** repositories were scanned, one private repository was explicitly excluded, and seven archived repositories are reference-only.
- **0** functions are functionally verified. The strict completion gate is **BLOCKED**.
- Two source-binding findings remain: `AMBIGUOUS_SOURCE_SYMBOL_BINDING=1` and `BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN=1`.

## Repository and catalog routing

| Repository | Unbound candidates | Initial catalog/owner routing | State |
|---|---:|---|---|
| [.github](https://github.com/A-TownChain-Okosystems/.github) | 38 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — organization governance tools | ROUTE_REVIEW_REQUIRED |
| [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | 507 | `components/vm/docs/VM_FUNCTION_CATALOG.json` (VM scope only) — route by component path | ROUTE_REVIEW_REQUIRED |
| [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | 391 | `docs/AURORA_DECENTRALIZED_MEMORY_FUNCTION_CATALOG.json` (memory scope only) — other modules need catalog discovery | ROUTE_REVIEW_REQUIRED |
| [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | 33 | `docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json` — control plane only | ROUTE_REVIEW_REQUIRED |
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | 296 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — integration/orchestration; route by subsystem ownership | ROUTE_REVIEW_REQUIRED |
| [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) | 278 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — documentation/reference, not runtime implementation | ROUTE_REVIEW_REQUIRED |
| [atc-algorithm](https://github.com/A-TownChain-Okosystems/atc-algorithm) | 13 | `docs/function-catalog.json` — support/specification only; canonical algorithm is in `a-townchain/components/algorithm` | ROUTE_REVIEW_REQUIRED |
| [atc-compute](https://github.com/A-TownChain-Okosystems/atc-compute) | 6 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [atc-engineering](https://github.com/A-TownChain-Okosystems/atc-engineering) | 198 | `schemas/org-function-catalog.schema.json` and `docs/architecture/ORG_FUNCTION_CATALOG_SPEC.md` — governance schema/spec, not runtime catalog | ROUTE_REVIEW_REQUIRED |
| [atc-ide](https://github.com/A-TownChain-Okosystems/atc-ide) | 141 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [atc-launchpad](https://github.com/A-TownChain-Okosystems/atc-launchpad) | 27 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [atc-marketplace](https://github.com/A-TownChain-Okosystems/atc-marketplace) | 41 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [atc-node](https://github.com/A-TownChain-Okosystems/atc-node) | 28 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — review against canonical node paths in `a-townchain` | ROUTE_REVIEW_REQUIRED |
| [atc-sdk](https://github.com/A-TownChain-Okosystems/atc-sdk) | 15 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — review against canonical SDK/client contracts | ROUTE_REVIEW_REQUIRED |
| [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | 122 | `docs/function-catalog.json` — validation pending | ROUTE_REVIEW_REQUIRED |
| [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore) | 77 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — support/specification only; canonical kernel is in `globus-os` | ROUTE_REVIEW_REQUIRED |
| [atc-vm](https://github.com/A-TownChain-Okosystems/atc-vm) | 35 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — noncanonical support/snapshot; canonical VM is in `a-townchain/components/vm` | ROUTE_REVIEW_REQUIRED |
| [atc-zkp](https://github.com/A-TownChain-Okosystems/atc-zkp) | 21 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | 106 | `specs/language/registry.json` and `specs/semantics/registry.json` — normative registries; function inventory separate | ROUTE_REVIEW_REQUIRED |
| [genesis-chronicles](https://github.com/A-TownChain-Okosystems/genesis-chronicles) | 78 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) | 484 | `docs/architecture/genesis-unified-capability-catalog.json` — registered catalog, record-level reconciliation required | ROUTE_REVIEW_REQUIRED |
| [genesis-franchise-factory](https://github.com/A-TownChain-Okosystems/genesis-franchise-factory) | 24 | `NOT_REGISTERED_IN_SOURCE_CATALOG_REGISTRY` — repository-local catalog required | ROUTE_REVIEW_REQUIRED |
| [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | 3,428 | `docs/OS_FUNCTION_CATALOG.json` and `docs/GLOBUSOS_FRONTEND_FUNCTION_CATALOG.json` — route by source path and subsystem | ROUTE_REVIEW_REQUIRED |

## Rules for applying this classification

1. The central ecosystem catalog is the architecture/ownership/evidence control plane; it does not take over implementation ownership.
2. Each candidate must be reviewed against its exact repository HEAD, source path, symbol kind/name, canonical owner and relevant tests/contracts.
3. Add a real function/interface contract to the owning repository's local catalog only when source evidence supports that contract. Then register/reference that catalog in the central catalog source registry.
4. If a candidate is not a function contract, record a reviewed, reasoned exclusion; never silently discard it.
5. Do not auto-bind based on matching names, identical hashes or repository names. Identical-file hash groups are triage hints only.
6. Keep unresolved consensus decisions and ambiguous bindings BLOCKED. Do not change runtime implementation as part of this catalog-first pass.

The full candidate-level source evidence is retained in the linked exact-SHA census/reconciliation artifact. This document routes review work; it does **not** assert that any of the 6,387 candidates has been bound, excluded, implemented or verified.
