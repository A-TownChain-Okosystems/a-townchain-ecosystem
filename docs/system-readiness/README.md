# System Readiness — Einstieg

Dieses Verzeichnis enthält die Evidenz zur Betriebsbereitschaft des Ökosystems:
Befunde, Register, Prüfungen und die daraus abgeleiteten Blocker. Es ist der
Einstiegspunkt; einzelne Dokumente sind ohne diesen Index nicht auffindbar.

**Regel:** Neue Dokumente werden hier eingetragen, mit Zweck und Stand. Ein
Dokument ohne Eintrag gilt als nicht vorhanden.

## Reihenfolge für den Einstieg

1. [system-readiness.yaml](system-readiness.yaml) — aggregierter Zustand
2. [BLOCKER_REGISTER.md](BLOCKER_REGISTER.md) — was blockiert, mit Evidenzverweis
3. [EVIDENCE_MODEL.md](EVIDENCE_MODEL.md) — welche Nachweise zulässig sind
4. [EVIDENCE_REGISTRY.yaml](EVIDENCE_REGISTRY.yaml) — SHA-gebundene Nachweise

## Befunde

| Dokument | Gegenstand | Stand |
|---|---|---|
| [PHASE0_CI_ROOT_CAUSE.md](PHASE0_CI_ROOT_CAUSE.md) | Ursachenanalyse des CI-Ausfalls im November | eingefroren |
| [PHASE0_CI_RECOVERY_VERIFICATION.md](PHASE0_CI_RECOVERY_VERIFICATION.md) | Wiederherstellungsnachweis R-1 bis R-6 | laufend |
| [EXEC_CHAIN_GATE_FINDING_2026-11-16.md](EXEC_CHAIN_GATE_FINDING_2026-11-16.md) | Required Check `exec-chain`, `Dependabot`-Kontext | Fix vorbereitet, Push offen |
| [ATC_TEST_SUITE_RCA_2026-11-16.md](ATC_TEST_SUITE_RCA_2026-11-16.md) | Fehlerkomplex ATC Test Suite / `cargo-tests` | 4 von 6 Defekten geschlossen, 2 brauchen `workflow`-Scope |
| [ORG_PROTECTION_AUDIT_2026-11-17.md](ORG_PROTECTION_AUDIT_2026-11-17.md) | Branch Protection über alle 33 Repositories | erhoben, Änderung offen |
| [R5_ROTE_PRS_2026-11-17.md](R5_ROTE_PRS_2026-11-17.md) | Klassifikation der 66 roten PRs | erhoben |
| [MERGE_LOG_2026-11-16.md](MERGE_LOG_2026-11-16.md) | Einzelmerges mit SHA-Nachweis | abgeschlossen |
| [ORG_SNAPSHOT.md](ORG_SNAPSHOT.md) | Zustand der 33 Repositories | abgeschlossen |

## Zielbild Desktop

| Dokument | Gegenstand | Stand |
|---|---|---|
| [DESKTOP_LAYER_GAP_2026-11-17.md](DESKTOP_LAYER_GAP_2026-11-17.md) | Ist-Bestand gegen ATC-ARCH-002 | erhoben |
| [DESKTOP_LAYER_REQ_MATRIX.yaml](DESKTOP_LAYER_REQ_MATRIX.yaml) | Anforderung je Status, maschinenlesbar | erhoben |
| [END_TO_END_RUNTIME_ROADMAP.md](END_TO_END_RUNTIME_ROADMAP.md) | Weg zur Ende-zu-Ende-Lauffähigkeit | geplant |

## Werkzeuge und Rohbelege

| Datei | Inhalt |
|---|---|
| [tools/org_snapshot.sh](tools/org_snapshot.sh) | Repo-Snapshot, fetch-only |
| [atcvm_protection.json](atcvm_protection.json) | Rohantwort der Protection-API (Annex zum exec-chain-Befund) |
| [patches/atc-vm-exec-chain-on-pr.patch](patches/atc-vm-exec-chain-on-pr.patch) | Der `pull_request`-Trigger als anwendbarer Patch |
| [patches/contract-exec-gate.PUSH.yml](patches/contract-exec-gate.PUSH.yml) | Korrigierte Workflow-Datei im Klartext |

## Was hier nicht hingehört

Normative Standards liegen in `atc-standards`, nicht hier. Befunde dürfen dort
nur referenziert, nicht dupliziert werden. Verweise auf andere Repositories
erfolgen als Pfad plus Repositoryname, nie als kopierter Inhalt.

## Altlasten in Nachbar-Repositories

| Item | Repository | Stand |
|---|---|---|
| FILE_REGISTER ohne Bezug zum Bestand (828 von 2160 Dateien) | atc-standards | Fix als PR #128 vorgeschlagen |
