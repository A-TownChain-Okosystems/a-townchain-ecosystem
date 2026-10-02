# Residuals Register

Open deviations and known limitations. A residual MUST NOT be marked `RESOLVED`
unless the referenced evidence exists in `governance/status.json`.

| ID | Component | Description | Status | Evidence | Owner | Target |
|---|---|---|---|---|---|---|
| RES-SHIVACORE-BASELINE | shivacore-baseline | ShivaCore kernel baseline does not compile under no_std x86_64 at exact SHA `fb90684fa8bca395d29177e68b235d80f45ca7f8`: missing Rights::INSPECT, x86_64 availability, Vec/String imports, u64 dereference, ToString under no_std, PageSize/Size4KiB::SIZE, UserContext move, and ELF/kernel test failures. | OPEN | EVD-SHIVACORE-37-SHA | @maintainer | 2026-10-31 |
| RES-DEPREVIEW-GATE | governance | GitHub Dependency Review is currently unavailable because Dependency Graph support is unavailable for the repository. It is documented as a residual and is not treated as a merge blocker by this framework; it is not silently bypassed. | OPEN | EVD-DEPREVIEW-37 | @maintainer | 2026-11-15 |
