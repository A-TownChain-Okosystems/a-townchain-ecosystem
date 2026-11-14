# Changelog

## Unreleased (2026-11-14)
- Vendorte Kernel-Kopie entfernt: `components/globus-os`, 504 Dateien. Snapshot von `globus-os@4fccb82` (11.11.2026), zuletzt 59 Commits hinter dem kanonischen HEAD `9472b4c`, 0 Dateien nur in der Kopie. Kanonisch ist allein das Repo `globus-os`. Commit `3759d78a`.
- CI entkoppelt: `ecosystem-release-readiness.yml` schliesst `components/globus-os` aus der Build-Iteration aus (Commit `effc2e93`), toter Job `globus-browser` in `test-suite.yml` entfernt (`fc81fc37`).
- Branches aufgeraeumt: 303 Branches ohne eigenen Commit in 27 Repos entfernt. 7 weitere in archivierten Repos bleiben, weil diese read-only sind.

## (2026-11-11)
- Spiegel bereinigt: `components/a-townchain/kernel/globus-os` und `.../kai-os` entfernt (563 Dateien). Im Kanon (a-townchain 7aeb08a) waren beide Kopien am 11.11.2026 entfallen; kanonisch sind die Repos `globus-os` und `a-townchain-os`, gespiegelt unter `components/globus-os` und `components/a-townchain-os`.

## 0.1.0 (2026-09-17)
- Initiale Ecosystem-Konsolidierung: 5 Komponenten-Importe per git subtree (volle Historie)
- Determinism- und Test-Gates pro Komponente; Governance-Audit gegen Org-Registry (SCR-0127)
