# R-2 — Org-weiter Audit der Branch Protection

**Datum:** 17.11.2026 · **Methode:** GitHub API, read-only · **Umfang:** alle 33 Repositories
**Frage:** Wird ein Merge durch Required Checks und Review tatsächlich erzwungen?

## 1. Ergebnis

Nur **6 von 33** Repositories haben überhaupt eine Branch Protection auf `main`.
Davon sind **2 archiviert** (SCR-0127). Durchgesetzt wird CI also in **4 von 26 aktiven Repositories** — 15 %.

### Geschützt (6)

| Repo | Kontexte | Reviews | enforce_admins | strict |
|---|---|---|---|---|
| atc-standards | 14 | 1 | true | **true** |
| atc-vm | 6 | 1 | true | false |
| atc-engineering | 3 | 1 | true | false |
| atc-node | 3 | 1 | true | false |
| atc-interop (archiviert) | 3 | 1 | true | false |
| atc-oracle (archiviert) | 2 | 1 | true | false |

### Ungeschützt (27)

`.github`, `a-townchain`, `a-townchain-ecosystem`, `a-townchain-os`, `a-townchain-os-docs`,
`atc-algorithm`, `atc-compute`, `atc-contracts`, `atc-explorer`*, `atc-ide`, `atc-indexer`*,
`atc-launchpad`, `atc-marketplace`, `atc-mining`*, `atc-sdk`, `atc-shivacore`, `atc-storage`*,
`atc-toolchain`, `atc-wallet`*, `atc-zkp`, `atclang`, `aurora-ai`, `demo-repository`,
`genesis-chronicles`, `genesis-engine`, `genesis-franchise-factory`, `globus-os`

`*` = archiviert (7 Repos, deckungsgleich mit SCR-0127). Aktive ungeschützte Repositories: **23**.

## 2. Was das konkret bedeutet

In den 23 aktiven ungeschützten Repositories gilt:

- Required Checks sind nicht wirksam. Ein roter Lauf verhindert den Merge nicht.
- Es gibt keine erzwungene Review-Freigabe. `APPROVED` ist Konvention, nicht Bedingung.
- `enforce_admins` existiert dort nicht, weil es keine Protection gibt, die greifen könnte.

Damit ist die Aussage „Gate rot" in diesen Repositories nur ein Signal, keine Sperre. Von den
**238 offenen PRs** liegen 214 in ungeschützten Repositories; dort ist der Zustand `mergeable`
die einzige technische Schranke.

Für die bereits dokumentierten Vorgänge heißt das rückblickend: Die fünf Merges vom 16.11.
(`a-townchain-os-docs` #50/#51, `atc-marketplace` #15, `globus-os` #118, `atc-standards` #124)
waren bei `atc-standards` durchsetzbar geschützt, bei den drei anderen Repositories nicht —
dort war die Prüfung freiwillig und ist als solche dokumentiert.

## 3. Dangling Contexts

`atc-interop` und `atc-oracle` sind archiviert, tragen aber weiterhin Required Contexts
(`cargo-audit`, `codeql`, `Dependabot`), die dort nie mehr erzeugt werden können. Auf archivierten
Repositories ohne PR-Verkehr ist das wirkungslos, aber irreführend. Hausmeisterschaft, keine
Sicherheitslücke.

## 4. Nicht betroffen

Die drei geschützten aktiven Repositories `atc-standards`, `atc-engineering` und `atc-node`
erfüllen alle Required Contexts auf ihren PR-Heads. `atc-vm` ist der einzige reale Ausreißer
(`exec-chain`, Befund EXEC_CHAIN_GATE_FINDING_2026-11-16.md).

## 5. Empfehlung (Owner-Entscheidung, nichts geändert)

Mindest-Schutz für die kanonischen Repositories, Reihenfolge nach Schadenswirkung:

1. `a-townchain` (Konsens, State, Rust kanonisch) — 1 Review + `cargo-tests`, `audit`, `codeql`, `dependency-review`
2. `atc-contracts` (Verträge) — 1 Review + Vertrags-/Exec-Tests
3. `atc-shivacore` (Kernel-TCB) — 1 Review + `cargo-tests`, `codeql`, `dependency-review`
4. `atclang` (Sprache/Toolchain) — 1 Review + Compiler-Tests
5. `globus-os` (OS) — 1 Review + `cargo-tests`, `codeql`
6. `a-townchain-ecosystem` (Integration) — 1 Review; Gates sind hier importierte Snapshots, deshalb erst nach D-1/D-2
7. `aurora-ai` (AI-Plattform) — 1 Review + Tests

`enforce_admins: true` konsequent wie in den bestehenden vier. Kein `strict` erforderlich,
solange Rebase-Disziplin freiwillig bleibt.

**Hinweis:** Schutz setzt voraus, dass die jeweiligen Gates auf PRs tatsächlich erzeugt werden
(Lehre aus atc-vm). Ein Gate, das nur auf `push` oder `schedule` läuft, macht ein Repository
dauerhaft merge-blockiert — deshalb ist die Reihenfolge: erst Gate-Erzeugung nachweisen, dann
Protection aktivieren.
