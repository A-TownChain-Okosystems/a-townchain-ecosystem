---
document_id: ATC-KAI-OS-GAPS-001
title: Missing Modules Register
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Missing Modules Register

Erhoben am 17.11.2026. Vollstaendigkeit ist nicht behauptet; die Erhebung ist
AP-001 und noch nicht abgeschlossen.

Belegt fehlend oder unvollstaendig:

| Bereich | Befund | Beleg |
|---|---|---|
| Desktop-Schicht | 12 von 32 Anforderungen ohne Umsetzung, 13 teilweise | Gap-Analyse gegen ATC-ARCH-002, Commit b339a9d |
| Barrierefreiheit, Portale, Locale | im Bestand nicht vorhanden | Gap-Analyse |
| Browser | kein kanonisches Projekt bestaetigt | offen, AP-001 |
| GPU-/NPU-Backend fuer Aurora | keine Implementierung nachgewiesen | offen, AP-001 |
| Paket- und Update-Vertrauenskette | geplant, nicht implementiert | ATC-ARCH-002 §4.6.1 |
| Secure Boot, TPM-Kette | geplant | ATC-ARCH-002 |
| Recovery | geplant | ATC-ARCH-002 |
| SBOM und signierte Release-Artefakte | nicht nachgewiesen | Phase 9 |

Nicht in diesem Register: fehlende Tests je Modul. Diese gehoeren in die
IMPLEMENTATION_MATRIX, sobald AP-001 abgeschlossen ist.
