# Befund: Required Check `exec-chain` und `Dependabot` in atc-vm

Owner-Entscheidung vom 16.11.2026: `exec-chain` erzeugen, `Dependabot` vorläufig erhalten.
Keine Änderung an der Branch Protection, bevor Check-Identität und Funktion geklärt sind.
Dieses Dokument beantwortet genau diese Frage und ändert die Protection **nicht**.

## 1. Gesicherte Konfiguration (vor jeder Änderung)

`GET /repos/A-TownChain-Okosystems/atc-vm/branches/main/protection`

| Feld | Wert |
|---|---|
| Required Contexts | `audit`, `cargo-tests`, `cargo-audit`, `codeql`, `exec-chain`, `Dependabot` |
| strict (up-to-date required) | `false` |
| Required approving reviews | `1` |
| enforce_admins | `true` |

Rohkopie: `atcvm_protection.json` (Annex dieses Commits).

## 2. `exec-chain` — Ursache und Reparatur

**Der Job existiert.** `.github/workflows/contract-exec-gate.yml`, Job-ID `exec-chain`,
ohne `name`-Feld, also erscheint der Check unter exakt dem Kontextnamen `exec-chain`.

**Er läuft nur nicht auf Pull Requests.** Der Trigger war:

```yaml
on:
  workflow_dispatch:
  push:
    branches: [main]
```

Belege:

| Beobachtung | Wert |
|---|---|
| Läufe des Workflows gesamt | 28 |
| Events dieser Läufe | ausschließlich `push` (main) und `workflow_dispatch` |
| Letzter Lauf | 2026-09-25, `push`, `main`, `21132a49`, `success` |
| `exec-chain` unter den Checks eines PR-Heads | nicht vorhanden (#14–#17 geprüft) |

Ein Required Context, der auf `pull_request` nie erzeugt wird, kann von keinem PR erfüllt
werden. Das erklärt `mergeable_state = blocked` bei allen vier PRs, bei gleichzeitig grünen
tatsächlich laufenden Checks.

**Lokaler Nachweis, dass der Job auf PRs grün wäre** (Reihenfolge wie im Workflow):

| Schritt | Ergebnis |
|---|---|
| `cargo build --locked --bin atc-vm-runner` | OK |
| Native ATCLang-Assembly (`e2e_adder.atc` → `.ops`) | `simulation PASS` |
| ATVM-Ausführung mit Vektor-Verifikation | `6 Ops executed, result 20 (ATVM PASS)` |
| Receipt-Verifikation | `status: PASS`, `result 20`, `expected 20` |

**Reparatur (Patch vorbereitet, nicht gelandet):** zwei Zeilen im Trigger,
keine neue Implementierung, kein Ersatz-Job:

```yaml
  pull_request:
    branches: [main]
```

Status: `VORBEREITET`. Der Push wurde von GitHub abgelehnt:
`refusing to allow an OAuth App to create or update workflow ... without 'workflow' scope`.
Der Agenten-Token hat `read:user, repo` — kein `workflow`. Kein Umgehungsweg.
Patch: `atc-vm-exec-chain-on-pr.patch`, Datei: `contract-exec-gate.PUSH.yml`.

## 3. `Dependabot` — Identität und Funktion, jetzt belegt

Der Kontext ist **kein** veralteter Name und **kein** Phantom-Check. Er wird erzeugt von
GitHubs eigenen Dependabot-Update-Läufen:

| Feld | Wert |
|---|---|
| Lauf-Pfad | `dynamic/dependabot/dependabot-updates` |
| Workflow-Name | z. B. `cargo in /. - Update #1699716075` |
| Event | `dynamic` |
| Job-Name | `Dependabot` |
| Schritte | `Run Dependabot`, `Post Run Dependabot` |
| Rhythmus | wöchentlich, laut `.github/dependabot.yml` (cargo + github-actions) |
| Branches | `main` |
| Letzte Läufe | 2026-11-13 und 2026-11-06, beide `success` |

Damit ist die fachliche Zuordnung eindeutig: `Dependabot` ist ein **geplanter
Wartungs-/Aktualisierungsjob auf `main`**, keine Prüfung eines Pull Requests. Er kann von
einem PR prinzipiell nicht erfüllt werden, weil er an `schedule`/`main` gebunden ist und
nicht an `pull_request`.

**Konsequenz, die eine Entscheidung braucht:** Solange `Dependabot` Required Context bleibt,
ist jeder PR in atc-vm dauerhaft `blocked` — auch nach der `exec-chain`-Reparatur. Zwei
dokumentierte Wege:

1. **Behalten** (Owner-Status quo): PRs bleiben blockiert; Merges nur über dokumentierte
   Admin-Ausnahme, was `enforce_admins: true` derzeit verhindert.
2. **Gezielt entfernen** mit dieser Begründung: kein eigenständiges PR-Gate, sondern ein
   schedule-gebundener Wartungsjob; die PR-seitige Dependency-Prüfung liegt nachweislich bei
   `dependency-review` und `cargo-audit`, die auf jedem PR-Head laufen.

Keine der beiden Varianten wurde umgesetzt.

## 4. Was unverändert gilt

- Branch Protection: **nicht geändert**.
- Vier PRs: weiterhin `blocked`, unabhängig von `exec-chain`:
  - #14 pinnt den alten Workflow-Ref `d04ec88`
  - #16 widerspricht ATC-ARCH-001
  - #14–#17: keine Merge-Freigabe durch eine korrigierte Protection
- `CI_VERIFIED` bleibt `0`. Kein Merge in diesem Vorgang.

## 5. Abnahmekriterien und ihr Stand

| Kriterium | Stand |
|---|---|
| `exec-chain` wird tatsächlich ausgeführt | `VORBEREITET` — Patch fertig, Push ohne `workflow`-Scope blockiert |
| Check erscheint unter dem erwarteten Kontextnamen | belegt: Job-ID `exec-chain` ohne `name`, identisch zum Required Context |
| Fehler im Job führen zu rotem Required Check | strukturell gegeben (Job-Ergebnis = Check-Ergebnis); Laufnachweis erst nach Push |
| `Dependabot` fachlich zugeordnet und nachweisbarer Status | erfüllt (Abschnitt 3) |
| Keine Required Checks ohne Begründung entfernt | erfüllt — nichts entfernt |
| Vier PRs gegen aktuellen Head erneut geprüft | erfüllt (Baseline dokumentiert); Neubewertung erst nach Reparatur sinnvoll |
