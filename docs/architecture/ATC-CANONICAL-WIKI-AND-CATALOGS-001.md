# Canonical Wiki, Component Catalog and Function Catalog

**Governance state:** proposed in Draft PR #115; not yet merged.  
**Release state:** `BLOCKED` until source reconciliation and required validation have terminal evidence.

## 1. Canonical navigation / Wiki

The GitHub Wiki for `a-townchain-ecosystem` is currently disabled. Until it is explicitly enabled and governed, the version-controlled documentation index at [DELIVERY_CONTROL_PLANE.md](../DELIVERY_CONTROL_PLANE.md) is the canonical navigable Wiki substitute.

Do not maintain a second, conflicting copy of architecture facts in an unversioned Wiki. Wiki pages, README files and summaries are navigation or explanatory surfaces; when they conflict with versioned registries, approved standards, or source repositories, record the conflict and reconcile it instead of silently choosing one.

## 2. Canonical organization-wide component/function catalog

The organization-wide catalog is:

- Machine-readable catalog: [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)
- Human-readable catalog and acceptance rules: [ATC-COMPONENT-FUNCTION-CATALOG-001.md](ATC-COMPONENT-FUNCTION-CATALOG-001.md)
- Catalog source registry: [ATC-CATALOG-SOURCE-REGISTRY-001.json](ATC-CATALOG-SOURCE-REGISTRY-001.json)

The JSON catalog is the normalized cross-repository inventory for component IDs, function/capability contracts, interfaces, ownership links, source bindings, status and evidence references. The source registry records the source catalogs and their provenance. It is not proof that all source records have been imported or reconciled.

The organization-wide catalog is a **discovery, architecture and reconciliation index**, not a second implementation or normative-specification SSOT. It must link to source records and preserve their IDs and provenance.

## 3. Authority hierarchy and ownership

1. **Implementation SSOT:** the canonical source repository and exact source path designated for that component.
2. **Normative SSOT:** the approved, versioned standard/specification in its designated standards repository.
3. **Local catalog:** the component owner's repository-local inventory and contracts; these remain source records and must not be silently replaced by the organization-wide projection.
4. **Organization-wide catalog:** normalized cross-repository index that links to local catalogs and records reconciliation results.
5. **Wiki/index pages:** navigational and explanatory surfaces; they do not override items 1–4.

Canonical ownership baseline:

| Domain | Canonical owner / path |
|---|---|
| L1 Blockchain Core | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) |
| ATC-VM | [a-townchain/components/vm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/vm) |
| Consensus / algorithm implementation | [a-townchain/components/algorithm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/algorithm) |
| ShivaCore kernel | [globus-os/modules/atc-shivacore/kernel](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) |
| GlobusOS | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) |
| Normative standards registry | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) |
| Ecosystem governance / integration / evidence | This repository, [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) |

This table is an ownership baseline, not a claim that every listed component is complete or production-verified. Exact subpaths for domains without a path above must be recorded from their actual source tree before a function is marked bound.

## 4. Required function-record states

Every discovered source candidate must have exactly one current disposition:

- `BOUND`: stable function ID and exact repository, 40-character commit SHA, path, symbol kind/name, and source blob/provenance are recorded.
- `EXCLUDED`: a verifiable reason explains why the candidate is not a runtime function or catalog member (for example test-only, generated artifact, type/value rather than function, or archived reference).
- `UNRESOLVED`: source binding or classification is incomplete; this is a blocking residual and must not count as completed.

Source binding is not functional verification. Keep separate evidence states for implementation, build, tests, conformance, and runtime behavior. Do not mark a function verified merely because a symbol exists or a catalog validator passes structurally.

## 5. Reconciliation and change rules

- Preserve original source catalog IDs and records; use namespaced normalized IDs where required.
- Never deduplicate by name alone. Potential semantic duplicates remain explicit until reconciled with evidence.
- Do not count target requirements, tests, types, interfaces, or capability aspirations as implemented runtime functions.
- Compare catalogs against actual source trees at exact SHAs; record source path, symbol kind/name and blob identity where available.
- Keep conflicting owners, stale consumer workflows, API/ABI mismatches and unknown consensus semantics `BLOCKED` until a versioned decision and migration evidence resolve them.
- A schema/structural validator is not a complete source reconciliation. The strict gate must fail closed on unresolved runtime candidates and SHA mismatches.
- No implementation deletion, repository archival, release or merge authorization follows solely from catalog presence.
- Do not weaken CI gates or invent successful runs, test results or evidence.

## 6. Current decision

This page establishes the intended canonical arrangement. It does **not** certify completeness. The catalog remains `BASELINE_RESIDUAL`, source reconciliation remains `RESIDUAL` until the strict exact-HEAD gate passes, and release remains `BLOCKED`. PR #115 must remain Draft until the required catalog, validator and reconciliation evidence is reviewed.
