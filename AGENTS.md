# Repository AI Engineering Rules

Use AI_ENGINEERING.md as the repository-level contract for AI-assisted development.

Before changing code:

- inspect the existing implementation and call graph;
- search for duplicate implementations before adding a new abstraction;
- preserve canonical protocol constants and domain definitions;
- prefer Rust within the existing workspace;
- keep changes minimal and integration-oriented.

After changing code:

- run formatting/check/test/clippy as applicable;
- add regression tests for behavior changes;
- verify cross-crate integration paths;
- do not mark work complete based only on static inspection.

AI is an implementation tool, not a release authority. Protocol, cryptography, consensus, tokenomics, governance, and security-sensitive changes require human review.
