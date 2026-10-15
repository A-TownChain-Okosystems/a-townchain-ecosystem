# A-TownChain Architecture Catalog Wiki — Home

**Wiki ID:** `ATC-ARCH-CATALOG-WIKI-X1-X36-L1-L10`  
**Status:** `PROPOSED / NOT VERIFIED`

## Purpose

This chapter set provides a navigable explanation of the A-TownChain / GlobusOS / ShivaCore / ATC-VM / ATCLang / Aurora AI architecture catalog. It separates architecture scope and contracts from implementation ownership and verification evidence.

## Governing principles

- **Catalog First, Implementation Second.**
- **Standalone First, Ecosystem Second.**
- **Evidence before Acceptance.**
- One canonical implementation SSOT per production capability.
- Unknown facts remain `UNRESOLVED`; missing mandatory evidence fails closed.
- `demo-repository` is excluded from the live drift gate.
- A green CI result alone does not establish verification.

## Chapters

1. [Architecture layers](ATC-ARCH-WIKI-01-Layers.md) — L1–L10 and X1–X36.
2. [Sublayers, components and contracts](ATC-ARCH-WIKI-02-Contracts.md) — IDs, records and contract requirements.
3. [SSOT and ownership](ATC-ARCH-WIKI-03-SSOT.md) — canonical implementation boundaries.
4. [Roadmap and delivery](ATC-ARCH-WIKI-04-Roadmap.md) — milestones, checkpoints, sprints and TODOs.
5. [Verification and evidence](ATC-ARCH-WIKI-05-Evidence.md) — gates, statuses and proof requirements.
6. [Governance and change control](ATC-ARCH-WIKI-06-Governance.md) — approvals and normative authority.

## Canonical catalog

[Open the master catalog](../ATC-ARCH-CATALOG-MASTER-X1-X36-L1-L10.md)

The master catalog is the cross-layer registry. These chapters are explanatory navigation pages and must not silently diverge from it.

## Contribution workflow

1. Identify the stable layer, sublayer, component or contract ID.
2. Inspect existing registry records and canonical source before adding a new record.
3. Record evidence-backed ownership and source revisions; do not infer paths or semantics.
4. Update the master catalog/index and relevant chapter together.
5. Keep unverified claims `PROPOSED`, `UNRESOLVED`, `RESIDUAL` or `BLOCKED` as appropriate.
6. Link the pull request, exact commit and evidence packet.

## Current disposition

The Wiki chapter structure is proposed. Full source census, chapter-to-registry consistency validation and technical verification remain outstanding.
