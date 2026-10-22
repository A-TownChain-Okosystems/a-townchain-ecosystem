# Registry correction — 21 October 2026

**Authoritative snapshot:** [Registry v0.2.2](../registry/2026-10-21-v0.2.2.json)

This correction supersedes the initial [v0.2.1 snapshot](../registry/2026-10-21.json) for current machine-readable use. It does not change the report's market conclusions. It corrects three registry semantics:

1. Removes a placeholder `REMOVE` row with no provider ID; there are **14 actual ADD/UPDATE delta records**.
2. Marks Ethereum Glamsterdam and Windows 11 26H2 as `BASELINE`, because no reliable prior product snapshot is persisted for those entries.
3. Records Bubble's Mobile and Web & Mobile price effective date as 20 October 2026 based on Bubble's official pricing article.

The original v0.2.1 file is retained as an audit artifact. The correction remains on the Draft PR branch and is not yet merged to `main`.
