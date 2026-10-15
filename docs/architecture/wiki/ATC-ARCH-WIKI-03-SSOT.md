# Chapter 03 — Canonical ownership and SSOT

**Status:** `PROPOSED / NOT VERIFIED`

## Ownership registry

| Capability | Canonical owner rule |
|---|---|
| Blockchain core | `A-TownChain-Okosystems/a-townchain` |
| ATC-VM | `a-townchain/components/vm` |
| Consensus/algorithm | `a-townchain/components/algorithm` |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` |
| GlobusOS services/UI/install | `A-TownChain-Okosystems/globus-os`; exact module paths must be evidenced |
| ATCLang/toolchain | Repository and paths must be source-verified before assignment |
| Aurora AI | Repository and paths must be source-verified before assignment |
| Normative standards | `A-TownChain-Okosystems/atc-standards` |
| Architecture catalog/integration governance | `A-TownChain-Okosystems/a-townchain-ecosystem` |

These are ownership rules, not proof that every function has been found or implemented.

## Ecosystem repository boundary

The ecosystem repository owns architecture cataloging, cross-repository mapping, integration/compliance governance, evidence aggregation and drift controls. It must not become the production SSOT for a second VM, kernel, consensus algorithm, compiler, installer, scheduler or AI runtime.

## Source-mapping requirements

For a mapped implementation, record repository, exact path, symbol/entry point, full 40-character commit SHA, blob SHA when available, and relevant tests/evidence. If any key relationship is unknown, preserve it as unresolved rather than inventing a path.

## Drift and duplication controls

- Detect stale source SHAs and missing paths.
- Detect multiple claimed canonical owners for the same production capability.
- Keep the repository manifest authoritative for live drift checks.
- Exclude `demo-repository` from the live drift gate.
- Do not use documentation-only assertions as implementation evidence.
