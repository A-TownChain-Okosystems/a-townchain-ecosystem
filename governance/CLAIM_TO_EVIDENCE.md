# Claim-to-Evidence Matrix

Each claim ID MUST also appear in `governance/status.json` with matching evidence IDs.
CI fails if a claim has no evidence reference.

| Claim ID | Component | Claim | Evidence IDs |
|---|---|---|---|
| CLM-IPCBUS-001 | ipcbus | IPCBus P0/P1 implementation exists on PR #37 at exact SHA `fb90684fa8bca395d29177e68b235d80f45ca7f8`. | EVD-IPCBUS-37-SHA |
| CLM-EVENTBUS-001 | eventbus | EventBus P0/P1 implementation exists on PR #37 at exact SHA `fb90684fa8bca395d29177e68b235d80f45ca7f8`. | EVD-EVENTBUS-37-SHA |
| CLM-SHIVACORE-001 | shivacore-baseline | ShivaCore kernel does not yet build cleanly under no_std x86_64 at exact SHA `fb90684fa8bca395d29177e68b235d80f45ca7f8`. | EVD-SHIVACORE-37-SHA |
| CLM-DEPREVIEW-001 | governance | Dependency Review is a documented residual because GitHub Dependency Graph support is unavailable; it is not treated as a merge blocker by this framework. | EVD-DEPREVIEW-37 |
