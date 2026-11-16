# Evidence Model — vier getrennte Evidenzobjekte

**Status:** ACTIVE (Owner-Entscheidung 2026-11-16)
**Scope:** gesamtes Ökosystem, alle 33 Repositories
**Grundsatz:** No Evidence. No Trust. Kein Objekt darf ein anderes ersetzen.

## 1. Die vier Objekte

| Objekt | Aufgabe | Änderbarkeit | Ort |
|---|---|---|---|
| **RCA** | historische Ursachenanalyse eines abgeschlossenen Incidents | **unveränderlich** nach `FINAL`; Korrekturen nur über Nachfolgedokument | `PHASE0_CI_ROOT_CAUSE.md` |
| **Recovery** | operativer Fortschritt, Reparaturmaßnahmen, aktuelle Gate-Entscheidung | fortlaufend, mit Datum und Evidenz | `PHASE0_CI_RECOVERY_VERIFICATION.md` |
| **Blocker-Register** | aktueller Status, Abhängigkeiten, Verantwortlichkeit | fortlaufend, Status nur mit Nachweis | `BLOCKER_REGISTER.yaml` |
| **Evidence-Registry** | maschinenprüfbare Nachweise (Hashes, Run-IDs, SHAs) | append-only | `EVIDENCE_REGISTRY.yaml` |

## 2. Regeln

1. **Historische Aussage ist unantastbar.** Eine abgeschlossene RCA wird nicht rückwirkend an den Reparaturerfolg angepasst. Sonst ist später nicht mehr prüfbar, was zum Incident-Zeitpunkt bekannt war.
2. **Operativer Fortschritt gehört ausschließlich ins Recovery-Dokument.** Befunde und Maßnahmen werden nicht vermischt.
3. **Statuswechsel nur mit Evidenz.** Ein Blocker wechselt von `BLOCKED` nach `VERIFIED`, wenn die Registry einen vollständigen, SHA-gebundenen Nachweis enthält (`evidence_complete: true`).
4. **Fehlende Nachweise sind kein Erfolg.** Ein fehlender Joblog, ein HTTP-403 oder ein Lauf ohne Jobs ist `MISSING`, niemals `success`.
5. **Ein Objekt referenziert das andere, ersetzt es aber nicht.** RCA verweist auf den Nachfolger, Recovery auf die RCA, das Register auf beide, die Registry auf alle.

## 3. Lebenszyklus

```
DRAFT → ASSESSED → FINAL (historisch, eingefroren)
                      └── Nachfolger: Recovery (laufend) → VERIFIED / REJECTED
```

Ein `FINAL`-Dokument erhält keine inhaltlichen Änderungen mehr. Neue Erkenntnisse erzeugen ein neues Dokument mit Verweis auf das alte.

## 4. Bindung an Artefakte

Jede Evidenzzeile der Registry nennt: `artifact`, `sha256`, `commit`, `run_id` (falls anwendbar), `collected_at`, `collector` und `evidence_complete`. Der Hash wird über die Bytes gebildet, die der Collector tatsächlich erhalten hat; Normalisierung ist unzulässig, weil sie den Nachweis verändert.

## 5. Anwendung auf die übrigen Blocker

Das Modell gilt für alle Register-Einträge: `BLK-P0-01`, `BLK-P0-02` (ATC-BC-001), `BLK-P1-01` (E-1 bis E-7) usw. Jeder Blocker erhält bei Abarbeitung ein Recovery-Objekt; die RCA entsteht nur dort, wo ein abgeschlossenes Ereignis zu analysieren ist.
