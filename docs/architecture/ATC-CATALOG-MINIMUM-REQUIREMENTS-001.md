# ATC Catalog Minimum Requirements 001

Version: 1.0.0  
Status: NORMATIVE / release gate BLOCKED until validated  
Scope: all A-TownChain and GlobusOS repositories and interfaces represented by the ecosystem catalog.

## Normative requirements

The existing `ATC-COMPONENT-FUNCTION-CATALOG-001.json` remains the single catalog. This specification does not authorize a second catalog or move implementation ownership into the ecosystem repository.

- MUST-001 — Inventory all relevant repositories, modules, components, system services, public APIs, external interfaces and platform-specific capabilities. Inventory gaps must be explicit.
- MUST-002 — Component, function, interface, gate and dependency identifiers MUST be stable and unique.
- MUST-003 — A source-mapped or implemented function MUST identify repository, exact 40-character source commit SHA, source path, symbol/entry point and Git blob SHA. Unknown bindings MUST be marked unresolved/residual, never guessed.
- MUST-004 — Implementation, source mapping, integration and verification MUST be separate states. Source mapping alone MUST NOT imply implementation; implementation alone MUST NOT imply verification.
- MUST-005 — Every implementation capability MUST have one canonical SSOT owner. The ecosystem repository is the integration/evidence control plane only.
- MUST-006 — Dependencies and cross-component interfaces MUST resolve to known IDs and document producer, consumer, contract and status.
- MUST-007 — Security-relevant contracts MUST document permissions/capabilities, trust boundaries, input validation, failure semantics and denied/negative paths. If not yet known, the gap MUST be explicit and release-blocking where applicable.
- MUST-008 — VERIFIED/TESTED/AUDITED/PRODUCTION_READY claims MUST include reproducible evidence tied to the exact source SHA. CI green alone is not sufficient.
- MUST-009 — Missing evidence, incomplete implementation, ambiguous ownership and excluded census candidates MUST be recorded explicitly. Exclusions require a reason.
- MUST-010 — Release MUST fail closed when required contracts or evidence are missing, contradictory, stale or unresolved, unless a formally documented exception identifies scope, rationale, owner, expiry and approver. No silent exceptions.

## Minimum function contract

Every function record MUST provide the catalog's required identity and behavioral fields: `id`, `name`, `purpose`, `inputs`, `outputs`, `failure_modes`, `acceptance`, `test_classes` and `readiness`. The catalog's contract-analysis object MUST distinguish purpose, inputs, outputs, preconditions, behavior, architecture placement, consumers, reuse, failure semantics and source evidence.

Source-mapped/implemented functions additionally require repository identity, exact source SHA, source path, symbol/entry point and file blob SHA. Functions that do not yet meet this requirement MUST retain an explicit residual/blocked/unresolved state and MUST NOT be represented as verified.

## UI and system-control coverage

Catalog each UI element's component, event/action, responsible service/function, required authority/capability, resulting state, success/failure behavior and tests. Coverage includes window open/close/move/resize/minimize/maximize/restore/focus; desktop shell, launcher, taskbar, menus, notifications and sessions; settings read/change/validate/save/reset; buttons, toggles, sliders, dropdowns and fields; mouse/keyboard/touch, focus, shortcuts and accessibility; service calls, event routing and IPC; authorization, denied actions and audit events; and recovery, invalid input, timeout, cancellation and service failure.

A visual element or source symbol is not evidence of working integration.

## Required CI gate

The validator MUST check JSON structure, required fields, unique IDs, reference integrity, full SHA syntax, source-binding completeness for mapped/implemented records, explicit residuals for gaps, status/evidence consistency, canonical ownership declarations and exclusion reasons. It MUST exit non-zero for violations. A structural pass MUST explicitly state that it is not functional or security verification.

## Status semantics

- `MAPPED`: source binding evidenced.
- `IMPLEMENTED`: implementation evidenced at a specific source commit.
- `VERIFIED`: required tests/checks passed with exact-commit evidence.
- `RESIDUAL`: known incomplete work or missing evidence.
- `BLOCKED`: concrete impediment prevents completion or verification.
- `UNRESOLVED`: ownership, mapping or behavior is not yet established.
- `EXCLUDED`: intentionally excluded with a reproducible reason.

Existing catalog readiness values remain authoritative for backward compatibility; these semantics MUST be mapped explicitly rather than silently conflated.

## Acceptance

The catalog, its validator, and CI must be updated together. Existing component/function/interface counts may change only through auditable catalog changes. The release decision remains BLOCKED until all mandatory release gates have real evidence for the exact commit.
