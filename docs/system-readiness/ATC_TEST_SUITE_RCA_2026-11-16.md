# ATC-TEST-SUITE-RCA-001 — Befund und Klassifikation

**Datum:** 16.11.2026 · **Gegenstand:** Workflow `ATC Test Suite`, Jobs mit `cargo`-/`pytest`-Bezug
**Abgrenzung:** nicht Gegenstand sind Rust CI und System CI (laut Owner grün) sowie der Phase-0-Plattform-Incident.
**Status:** `BLOCKED` — kein Defekt dieses Komplexes ist vollständig behoben, `CI_VERIFIED = 0`.

## 1. Erfasste fehlgeschlagene Läufe

| Repo | Run | Event | Head | Zeit (UTC) | Ergebnis |
|---|---|---|---|---|---|
| a-townchain-ecosystem | 43941035963 | pull_request | `ea75ce9255` | 19:36 | 4 von 5 Jobs rot |
| atc-marketplace | 43935715993 | push (main) | `2e2bc090c9` | 19:05 | 1 von 2 Jobs rot |
| globus-os | 43935737075 | push (main) | `a38738dfcd` | 19:05 | 1 von 3 Jobs rot |
| a-townchain | 43893673930 / 43893381145 | pull_request (dependabot) | `acce944f9c` / `5ded648978` | 15:11 / 15:09 | rot, separates Paket |

`a-townchain` und `atc-oracle`/`atc-explorer`-Läufe vom 07.10. sind historisch und nicht Teil dieses Komplexes.
Der `startup_failure` in `atc-node` (43850950645) gehört zum Phase-0-Incident, nicht hierher.

## 2. Befunde je Job (Run → Job → Step → Logzeile)

| # | Repo / Job | Fehlgeschlagener Step | Kernzeile aus dem Log |
|---|---|---|---|
| D-1 | a-townchain-ecosystem / `shivacore-tests` | `shivacore cargo test ueber alle Manifeste` | `error: current package believes it's in a workspace when it's not` — `components/atc-shivacore/modules/atc-security`, `components/atc-shivacore/sdk` |
| D-2 | a-townchain-ecosystem / `atclang-quality` | `ATCLang fmt + clippy + cargo test` | identische Ursache — `components/atclang/crates/atc-core` |
| D-3 | a-townchain-ecosystem / `aurora-tests` | `aurora-ai npm test/build ueber alle Manifeste` | `[vitest-pool]: Failed to start forks worker` · `Caused by: TypeError: Cannot read properties of undefined (reading 'bind')` in `node_modules/undici/lib/interceptor/decompress.js:78` — 7 Fehler, 0 Tests geladen |
| D-4 | a-townchain-ecosystem / `a-townchain-python` | `a-townchain ruff (kern-scoped)` | `F401 [*] 'pytest' imported but unused` — `tools/atc-sc-validator/tests/test_contract_registry_gate.py:15` — `Found 1 error` |
| D-5 | atc-marketplace / `pytest` | `Evidence-Bindung (SCR-0086 ... hartes Push-Gate)` | `remote: error: GH006: Protected branch update failed for refs/heads/main` · `Changes must be made through a pull request` |
| D-6 | globus-os / `cargo-tests` | `cargo test ueber alle Manifeste` | `error[E0463]: can't find crate for 'core'` — `modules/atc-shivacore/init_program`, `modules/atc-shivacore/greeter`; Hinweis `rustup target add x86_64-unknown-none` |

**Zentraler Befund: In keinem der sechs Fälle schlägt eine Test-Assertion fehl.**
Rot sind ausschließlich Workspace-Konfiguration, Werkzeug-/Versionsmatrix, ein Lint-Befund,
ein Push auf einen geschützten Branch und ein fehlendes Cross-Target.

## 3. Reproduzierbarkeit (lokal, gleiche Befehle wie im CI)

| Defekt | Reproduktion | Ergebnis |
|---|---|---|
| D-1 | `cd components/atc-shivacore/modules/atc-security && cargo test --quiet` | exakt derselbe Fehlertext wie im CI |
| D-2 | `cargo test` in `components/atclang/crates/atc-core` | exakt derselbe Fehlertext |
| D-4 | `ruff check tools/atc-sc-validator/` auf `a-townchain@main` | `Found 1 error` (F401) |
| D-5 | Ursache strukturell: Step committet `.atc/evidence/evidence.yaml` und ruft `git push` auf `main` | nachvollziehbar aus Log; Schutzregel lehnt ab |
| D-6 | Ziel `x86_64-unknown-none` nicht installiert | Log nennt fehlendes Target; `boot` wird im selben Loop per SKIP ausgenommen |

### Experiment zur Workspace-Behebung

| Variante | Ergebnis |
|---|---|
| `exclude` mit Glob `components/atc-shivacore/**` | **wirkungslos** — Fehler bleibt |
| `exclude` mit einstufigen Globs `components/atc-shivacore/*`, `.../modules/*` | **wirkungslos** — Fehler bleibt |
| `exclude` mit exaktem Pfad je Paket | **wirkt** — `cargo test` läuft, Exit 0 |

Konsequenz: Eine Behebung über `exclude` verlangt einen Eintrag je Paket — im Umbrella sind
**141 Manifeste** weder Mitglied noch ausgeschlossen. Das ist keine Formatierungsfrage, sondern
eine Aussage darüber, was der Umbrella überhaupt testen soll.

## 4. Klassifikation

| Defekt | Klasse | Begründung |
|---|---|---|
| D-1, D-2 | **Build-/Toolchain-Problem** (Workspace-Manifest) | Pakete liegen im Baum des Root-Workspaces, sind aber weder Mitglied noch ausgeschlossen |
| D-3 | **Umgebungsabhängigkeit** (Version-/Laufzeitmatrix) | vitest-Worker scheitert an `undici`-API, die die Node-Version nicht bereitstellt |
| D-4 | **Lint-Befund im Quellcode** | echter, reproduzierbarer F401; keine Testregression |
| D-5 | **Infrastruktur-/Governance-Fehler** | Evidence-Schreiber verletzt die Branch-Protection, nachdem die Tests durch sind |
| D-6 | **Umgebungsabhängigkeit** (Cross-Target) | `#![no_std]`-Programme werden mit Host-`cargo test` geprüft |
| — | **Testregression** | in keinem Fall nachgewiesen |

## 5. Korrekturplan (kleinste native Korrektur je Defekt)

| Defekt | Zuständig | Kleinste native Korrektur | Blockiert durch |
|---|---|---|---|
| D-4 | `a-townchain` (Quelle) | Import entfernen (**umgesetzt**, PR #109) | — |
| D-1, D-2 | `a-townchain-ecosystem` | Testumfang auf lebende Komponenten begrenzen (SKIP für Snapshots wie beim `boot`-Crate) **oder** 141 exakte `exclude`-Einträge | `workflow`-Scope (Variante Job) |
| D-3 | `aurora-ai` (Quelle) | `undici`-Version bzw. Node-Matrix im Paket `atc-aistudio` festziehen | Nachweis der kompatiblen Version |
| D-5 | `atc-marketplace` (Workflow) | Evidence nicht auf `main` pushen: Bot-Branch + PR oder Artefakt statt Repo-Commit | `workflow`-Scope |
| D-6 | `globus-os` (Workflow) | SKIP für `#![no_std]`-Programme wie beim `boot`-Crate, oder Target im Job installieren | `workflow`-Scope |

Bewusst **nicht** umgesetzt: das Setzen von 141 `exclude`-Einträgen. Das wäre ein pauschaler
Eingriff in die Struktur importierter Snapshots und widerspricht der Regel, Änderungen an
Snapshots erst nach Herkunfts- und Aktualitätsprüfung vorzunehmen.

## 6. Umgesetzt in diesem Schritt

`a-townchain` PR #109 — `fix(tools): unbenutzten pytest-Import entfernen (ruff F401)`

| Prüfung | vorher (`main`) | nachher (Branch) |
|---|---|---|
| `ruff check tools/atc-sc-validator/` | `Found 1 error` (F401) | `All checks passed` |
| `pytest tools/atc-sc-validator/tests` | 1 failed, 46 passed | 1 failed, 46 passed |
| Tests der betroffenen Datei | 21 passed | 21 passed |

Der verbleibende Fehlschlag `test_links.py::test_repository_is_clean` ist vorbestehend und
unabhängig: er meldet tote relative Markdown-Links, nicht diese Änderung. Er ist ein eigener
Befund und **nicht** Teil des Ruff-Gates.

## 7. Zusatzbefund: Scheingrün ohne Tests

`cargo test` in `components/atclang/crates/atc-core` meldet `running 0 tests` und Exit 0.
Ein grüner Lauf ohne Testfälle ist keine Evidenz. Für ATC-95/ATC-98 sollte ein Gate
Testfälle zählen, nicht nur den Exit-Code lesen.

## 8. Abnahmestatus

| Kriterium | Stand |
|---|---|
| Fehlerlokalisierung | erfüllt (Abschnitt 1–2) |
| Reproduzierbarkeit | erfüllt für D-1, D-2, D-4; strukturell für D-5; Log-belegt für D-6 |
| Ursachenklassifikation | erfüllt (Abschnitt 4) |
| Architekturprüfung | offen — betroffen sind ATC-ARCH-001 (Umbrella ist Integrationsebene), SCR-0129 (Standalone First), ATC-95/ATC-98 (Testevidenz) |
| Korrekturplan | erfüllt (Abschnitt 5) |
| Evidenz mit HEAD, Run→Job→Step, Logs, Hashes | Abschnitt 9 |

`cargo-tests` bleibt **BLOCKED**. Ein grüner Rust- oder System-CI-Lauf wird nicht als Ersatz gewertet.

## 9. Artefakt-Hashes der ausgewerteten Joblogs (SHA-256)

| Log | SHA-256 |
|---|---|
| a-townchain-ecosystem_shivacore-tests | `cdba3fac817b1794d750850430ec0c13ba2b6734fa3deab938dfb311d750b838` |
| a-townchain-ecosystem_atclang-quality | `1e89f6c1dc4e0e92f73eb6059e6ebc7349c2cc816773840012035bcfed813bb9` |
| a-townchain-ecosystem_aurora-tests | `74675dc5348c08e93097b45e52996d77a935231614f718419812dad8de2d7ae3` |
| a-townchain-ecosystem_a-townchain-python | `a4b34fae51e1e2d878306413eb735a6ca6a55b31765e2cf7807d5953f83c176d` |
| atc-marketplace_pytest | `9fe77020c4ea5d84e7ea4bd4967bf4df1b53d55b3dac5b76b380dbf96ed54077` |
| globus-os_cargo-tests | `c43047d78eaf06fc69274fcb81c90ba98443cb370d29ba8b95d9f12cb60624ca` |

---

## 8. Umsetzung D-3 (17.11.2026) — Node-20-Vertraeglichkeit

**Fix:** `aurora-ai` PR #65 (Branch `fix/aistudio-node20-compat-2026-11-17`, Commit `da94528`).

| | vorher | nachher |
|---|---|---|
| `vitest run` (Node 20.20.2) | 7 Errors, 0 Tests | 7 Dateien, 31 Tests gruen |
| `npm run build` | ok | ok |
| `npm ls undici` | `undici@8.12.2` (engines `node >=22.19.0`) | `undici@7.30.2` via `jsdom@29.1.1` |

Ursache im Detail: `jsdom@30` zieht `undici@8`; `undici@8.12.2` deklariert `engines.node >=22.19.0`,
beide Workflows fahren `node-version: "20"`. Zusaetzlich war `undici` eine ungenutzte
Direktabhaengigkeit. Korrektur: `jsdom` auf `^29.1.1`, `undici` entfernt. Keine Testaenderung.

### 8.1 Folgefund: Dependabot-Majors in diesem Bereich sind Fallen

`a-townchain-os-docs` PR #49 (`jsdom 29.1.1 -> 30.1.3`) traegt exakt dieselbe Inkompatibilitaet.
Der PR wurde am 16.11. 19:19 ohne Merge geschlossen; `main` steht dort weiterhin auf `^29.1.1`.
Bewertung bleibt: Major-Bumps von `jsdom` (>=30) und `undici` (>=8) sind auf Node 20 nicht lauffaehig
und duerfen nicht als reine Patch-/Minor-Pflege behandelt werden. Der Ausbau der CI auf Node 22 ist
eine Workflow-Aenderung und bleibt separat.

---

## 9. R-4 und atc-standards (17.11.2026)

### 9.1 R-4 Logzugriff: Luecke geschlossen

Der in Phase 0 dokumentierte 403 auf `a-townchain`-Joblogs ist nicht reproduzierbar. Stichprobe
ueber acht Repositories, jeweils der erste fehlgeschlagene Job:

| Repo | Run | Job | HTTP | Bytes |
|---|---|---|---|---|
| a-townchain | 43941833383 | pytest | 200 | 28.291 |
| a-townchain | 43941833344 | determinism | 200 | 24.850 |
| a-townchain-ecosystem | 43981505156 | rust-components (atc-indexer) | 200 | 29.250 |
| a-townchain-ecosystem | 43981505261 | a-townchain-rust | 200 | 12.576 |
| globus-os | 43935737075 | cargo-tests | 200 | 61.521 |
| globus-os | 43892871011 | rust | 200 | 377.102 |
| atc-marketplace | 43935715993 | pytest | 200 | 16.195 |
| atc-standards | 43935757996 | regenerate | 200 | 27.447 |
| atc-standards | 43935758423 | matrix | 200 | 19.919 |
| aurora-ai | 43981455878 | markdownlint | 200 | 33.690 |
| atc-node | 43891181845 | markdownlint | 200 | 55.478 |

**14/14 HTTP 200.** Der 403 war zeitgebunden (Incident-Fenster), kein Dauerzustand. R-4 wechselt
von BLOCKED auf IN_PROGRESS: Zugriff steht, die vollstaendige Erhebung aller roten Laeufe ist die
offene Restarbeit.

### 9.2 atc-standards: zwei getrennte Fehlschlaege

**Regenerate Standards Views (run 43935757996)** — die Regeneration selbst ist erfolgreich
(533 Standards, 506 APPROVED/27 CANDIDATE, 76 Familien, SHA `ca5c3b44`), der Commit `7fcccfd` und der
Bot-Branch `bot/regenerate-standards-views-43935757996` entstehen. Der Job scheitert erst an:

```
pull request create failed: GraphQL: GitHub Actions is not permitted to create or approve pull requests
```

Klasse: **Governance-Einstellung** (Repo/Org-Option "Allow GitHub Actions to create and approve pull
requests" ist aus), nicht Code. Zwei Wege: Einstellung freigeben (Owner) oder Workflow so umbauen, dass
kein PR erzeugt wird.

**Code Quality Matrix Live (run 43935758423)** — echtes Inhaltsfinding `a-townchain — language=MISMATCH`.
Ursache: Byte-Mehrheits-Sprache TypeScript (39,8 %) gegen deklariert `[rust, python]` bei Rust 30,4 %
und Python 28,3 %. Behoben in `atc-standards` PR #126 (Praesenz >= 5 % statt Byte-Mehrheit, Anteile im
Bericht sichtbar). Lokal 26/26 PASS, 0 FINDINGS; genau ein Verdict aendert sich.
