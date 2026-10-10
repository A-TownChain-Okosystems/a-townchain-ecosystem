# Delivery Control Plane — Roadmap, Wiki Index und Sprintsteuerung

**Status:** initialer maschinenlesbarer Entwurf; nicht als vollständige Verifikation aller Repositories zu verstehen.  
**SSOT:** [control-plane/DELIVERY_REGISTRY.json](../control-plane/DELIVERY_REGISTRY.json)  
**Architektur:** [ARCHITECTURE.md](../ARCHITECTURE.md) · [Control Matrix](../control-plane/CONTROL_MATRIX.md) · [Control Plane](../control-plane/README.md)

## Wiki-/Dokumentationsindex

Das GitHub-Wiki ist für dieses Repository derzeit deaktiviert. Bis es explizit aktiviert wird, ist dieser versionierte Dokumentationsindex die navigierbare Wiki-Ersatzstruktur. Inhalte werden nicht in einem zweiten, widersprüchlichen Wiki-SSOT dupliziert.

| Bereich | Kanonische Quelle |
|---|---|
| Architektur, Schichten, Systemgrenzen | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| Zuständigkeiten / Konfliktauflösung | [CONTROL_MATRIX.md](../control-plane/CONTROL_MATRIX.md) |
| Statusmodell und Evidence-Regeln | [STATUS_SCHEMA.yaml](../control-plane/STATUS_SCHEMA.yaml) |
| Komponenten und Delivery-Registry | [DELIVERY_REGISTRY.json](../control-plane/DELIVERY_REGISTRY.json) |
| Repository-Standard | [REPOSITORY_STANDARD.md](REPOSITORY_STANDARD.md) |
| Kernel-Implementierung und Boot-Evidence | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) |
| Blockchain-/VM-/Algorithmusimplementierung | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) |
| ATC-Standards und Konformität | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) |
| Genesis Engine | [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) |

## Roadmap und Meilensteine

Die Registry ist die maschinenlesbare Quelle für Roadmap-Einträge, Meilensteine, Checkpoints, Komponenten, Funktionen, Sprints und TODOs. Ein Status wird nur aus nachprüfbaren Repository-/CI-Daten aktualisiert. Nicht belegte Annahmen bleiben `PLANNED`, `BLOCKED` oder `RESIDUAL`.

1. **MS-01 — Evidence-driven Governance:** stabile IDs, exakter SHA, Run → Job → Step → Logs, Root Cause, Fix und Retest.
2. **MS-02 — ShivaCore Kernel Safety:** VMM-Mutation/Rollback, Mapping-Lifetime, Rust- und Boot-/UEFI-/ACPI-Evidence.
3. **MS-03 — A-TownChain Protocol:** kanonische Economics, TX-V2-Vektoren und explizite Konsensentscheidung.
4. **MS-04 — ATCLang/ATC-VM:** deterministisches Bytecodeformat, Limits, Fehlerbehandlung und Konformitätstests.
5. **MS-05 — Aurora/Genesis Integration:** Service-Grenzen, Berechtigungen und keine Verlagerung von KI-Autorität in den Kernel-TCB oder Konsens.

## Checkpoint-Regeln

- **IMPLEMENTED** bedeutet, dass eine Änderung vorhanden ist; es bedeutet nicht, dass sie verifiziert ist.
- **VERIFIED** verlangt die exakte getestete Commit-SHA, erfolgreiche erforderliche Gates und verlinkte Evidence.
- Ein laufender, übersprungener oder nicht erreichbarer Required Check ist kein grüner Check.
- Artifact-Upload-Erfolg allein belegt nicht, dass die Governance-Semantik des Artefakts erfüllt ist.
- Keine Gates schwächen, keine Evidence fingieren und keinen PR automatisch mergen.

## Sprint- und TODO-Steuerung

Der aktuelle Sprint ist bewusst nicht kalendarisch terminiert: zuerst rote Gates und Evidence-Integrität, danach SSOT-/Komponentenabgleich, dann Kernel- und Protokoll-Checkpoints. Termine werden erst gesetzt, wenn echte Issue-/Milestone-Daten geprüft sind. Jede Änderung an der Registry muss aus GitHub-/CI-Evidence abgeleitet werden; fehlende Evidenz bleibt explizit offen.

## Aktualisierungsvertrag für den Gate-Fix-Loop

Bei jedem Durchlauf:
1. Repositories und offene PRs aus der aktiven Registry neu inventarisieren.
2. Fehlgeschlagene und neu fehlgeschlagene Actions-Läufe erfassen; Run-ID, Job-ID, Step, Exit-Code und Log sichern.
3. Root Cause isolieren und minimal korrigieren, ohne Gates abzuschwächen.
4. Neue Gates auf dem exakten Korrektur-SHA erneut prüfen.
5. Registry-Einträge, Roadmap, Checkpoints, Sprint-Ziele und TODO-Status synchronisieren.
6. Komponenten-/Funktionslisten mit kanonischer SSOT-Zuständigkeit abgleichen.
7. Fehlende Daten als `BLOCKED`/`RESIDUAL` dokumentieren. Kein automatischer Merge.
