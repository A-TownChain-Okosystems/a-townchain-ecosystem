# Chapter 01 — Architecture layers

**Status:** `PROPOSED / NOT VERIFIED`

## L1–L10 — system architecture

| ID | Scope |
|---|---|
| L1 | Boot, Firmware & Hardware Abstraction (HAL) |
| L2 | ShivaCore Kernel / Trusted Computing Base |
| L3 | GlobusOS System Services & OS Architecture |
| L4 | GlobusOS Desktop, Window Manager & UI |
| L5 | A-TownChain Blockchain Core |
| L6 | ATC-VM & Smart-Contract Execution |
| L7 | Developer Platform & ATCLang |
| L8 | Aurora AI Platform |
| L9 | Applications & Ecosystem Services |
| L10 | Integration, Deployment & Operations |

L1–L10 describe system responsibilities. Exact module paths must be mapped against source; the layer label is not proof of implementation.

## X1–X36 — cross-cutting architecture

| IDs | Scope |
|---|---|
| X1–X6 | Security/trust; identity/capabilities; protocols; data/state; standards/policy; testing/evidence |
| X7–X12 | Observability/reliability; performance; portability; supply chain; privacy; resource economics |
| X13–X18 | API/service contracts; configuration/secrets; lifecycle/migration; accessibility; documentation; catalog drift |
| X19–X24 | Risk; incident response; continuity/recovery; compliance; product/configuration governance; human oversight |
| X25–X30 | Federated governance; evolution; semantic knowledge; simulation; ethics; incentives/economics |
| X31–X36 | Data governance; API security/zero trust; events/workflows; resource scheduling; deployment/release; system composition/E2E compatibility |

The complete names, descriptions, boundaries and acceptance requirements remain in the [master catalog](../ATC-ARCH-CATALOG-MASTER-X1-X36-L1-L10.md).

## Applicability

Every applicable X-layer-to-L-layer relationship must be recorded. An empty matrix cell is not evidence of non-applicability. Use an explicit disposition: applicable, not applicable with rationale, or unresolved.

## Layer acceptance

Each layer needs a defined owner, scope, boundaries, sublayers, components, function contracts, dependencies, source mappings, acceptance criteria and evidence. A layer is not `VERIFIED` merely because its documentation exists.
