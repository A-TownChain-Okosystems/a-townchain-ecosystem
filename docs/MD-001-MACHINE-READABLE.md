# ATC-MD-001 — Machine-Readable Markdown

## Contract

Every tracked Markdown document in this repository is represented exactly once in
`evidence/markdown-registry.json`.

The registry is the machine-readable projection of the Markdown corpus. It does
not replace the human-readable Markdown body and it never upgrades implementation
or release status.

## Required fields

| Field | Meaning |
|---|---|
| `path` | Repository-relative Markdown path |
| `blob_sha` | Exact Git blob SHA at registry generation |
| `type` | Deterministic document class |
| `lifecycle` | `active` or `archived` |
| `filename` | Markdown filename |

## Classification

`README.md`, `STATUS.md`, `ROADMAP.md`, `CHANGELOG.md`, `SECURITY.md`,
`ARCHITECTURE.md`, `CONTRIBUTING.md` and `AGENTS.md` receive stable types.
Documents below an archive path receive `archive`; all other Markdown is
`document`.

## Evidence rule

The registry is metadata/evidence infrastructure only. A registry entry does not
mean that the referenced document is correct, current, implemented, tested,
CI-verified, E2E-verified, or release-ready.

## Update rule

When Markdown is added, removed, renamed, or changed, the registry MUST be
regenerated so its path set and blob SHAs correspond to the repository state.
