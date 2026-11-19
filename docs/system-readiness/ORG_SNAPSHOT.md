# Org-Snapshot: Arbeitskopie und Nachweisführung

**Zweck:** lokale Arbeitskopie aller 33 Repositories aktuell halten, ohne sie zur Wahrheit zu erklären.
**Werkzeug:** [`tools/org_snapshot.sh`](tools/org_snapshot.sh) (versioniert, keine Schreiboperation auf Remotes)

## Disziplin

| Regel | Umsetzung |
|---|---|
| GitHub bleibt die Wahrheit | Die Kopie ist Arbeitsbestand; jede Aussage wird gegen `origin` geprüft |
| Kein ungeprüfter Push | Skript fetcht nur; es committet und pusht nie |
| Jeder Lauf dokumentiert | `SNAPSHOTS.jsonl`, append-only: Zeitstempel (UTC), alle 33 HEAD-SHAs, `drift`, `unpushed`, `dirty` |
| Drift sofort sichtbar | `--check` endet mit Exit 1, sobald ein Remote voraus ist |
| Personenlesbar | `ORG_MANIFEST.md` wird bei jedem Lauf neu erzeugt |

## Ablage

- Arbeitskopie: persistenter Pfad (nicht `/tmp`), z. B. `/app/atc-org`
- Nachweise: `SNAPSHOTS.jsonl` (Maschine), `ORG_MANIFEST.md` (Mensch)
- Bewusst **nicht** im Repository: die geklonten Repos selbst

## Beziehung zur Evidence-Registry

Der Snapshot ist ein Arbeitsnachweis, kein CI-Nachweis. Er belegt, welcher HEAD zum
Erhebungszeitpunkt gelesen wurde, und ersetzt keine `evidence_complete`-Bewertung
(siehe [`EVIDENCE_MODEL.md`](EVIDENCE_MODEL.md)).
