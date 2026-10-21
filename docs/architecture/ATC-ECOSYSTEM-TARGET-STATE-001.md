# ATC-ECOSYSTEM-TARGET-STATE-001 — Ökosystemweiter Soll-Zustand

> **Dokument-ID:** ATC-ECOSYSTEM-TARGET-STATE-001  
> **Version:** 0.1.0  
> **Status:** DRAFT / NOT VERIFIED  
> **Geltungsbereich:** A-TownChain, GlobusOS, ShivaCore, Aurora AI, ATCLang, Genesis Engine, Wallet/SDK, Standards, Deployment und Ökosystem-Governance  
> **Autoritative Ablage:** A-TownChain-Okosystems/a-townchain-ecosystem  
> **Zweck:** zentraler Soll-Katalog und überprüfbarer Soll-/Ist-Abgleich; keine zweite Implementierungs-SSOT.

## 1. Verbindliche Lesart

Dieses Dokument legt fest, wie der vollständige Ökosystem-Soll-Zustand erfasst und nachgewiesen wird. Es ist **kein** Beleg dafür, dass die beschriebenen Funktionen bereits implementiert oder verifiziert sind.

- **Soll:** freigegebene Architektur, Standards, Funktionsverträge, Sicherheitsinvarianten und Abnahmekriterien.
- **Ist:** Zustand einer kanonischen Quelle an einem vollständigen Commit-SHA.
- **Evidence:** konkrete Tests und Prüfläufe mit Run → Job → Step → Exit-Code/Logs sowie Artefakten und Hashes.
- **Abweichung:** Differenz zwischen freigegebenem Soll und beobachtetem Ist.
- **Status:** IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED, VERIFIED, RESIDUAL, BLOCKED, UNRESOLVED oder begründet EXCLUDED.

Ein grüner CI-Run, ein vorhandenes Dokument, ein geschlossener Issue oder ein gemergter PR allein beweist keine vollständige Verifikation. Fehlende Pflichtnachweise bleiben BLOCKED oder UNRESOLVED. VERIFIED gilt nur für die konkrete Anforderung, den konkreten Commit und die tatsächlich ausgeführten Abnahmekriterien.

## 2. SSOT- und Governance-Grenzen

| Zuständigkeit | Kanonische Quelle / Regel |
|---|---|
| Ökosystem-Inventar, Soll-/Ist-Korrelation, Driftbefunde, Cross-Repo-Evidence | a-townchain-ecosystem |
| Normative Standards und technische Verträge | atc-standards; nur registrierte und freigegebene Normen sind verbindlich |
| Blockchain-Kern, ATC-VM, Algorithmus/Konsens und Wallet | jeweilige kanonische Pfade in a-townchain bzw. atc-wallet |
| ShivaCore-Kernel | globus-os/modules/atc-shivacore/kernel/ |
| GlobusOS und OS-Dienste | kanonische Module in globus-os |
| Aurora AI | aurora-ai |
| ATCLang | kanonische ATCLang-Implementierungsquelle; exakter Pfad im Inventar nachzuweisen |
| Genesis Engine | genesis-engine |
| SDKs und Tooling | jeweilige registrierte SSOT, nicht ein duplizierter Umbrella-Snapshot |

Importierte Subtrees sind Integrations-Snapshots. Sie dürfen die Quell-SSOT nicht ersetzen. Keine zweite VM, kein zweiter Konsens-/Wallet-Kern und keine zweite Kernelimplementierung im Ecosystem-Control-Plane.

## 3. Zielmodell der Architektur

Das Zielmodell soll die Hauptschichten **L0–L9** und die quer wirkenden Kontroll- und Qualitätsbereiche **X1–X36** abdecken. Die Nummerierung und Namen müssen vor normativer Freigabe gegen das Layer-Register und atc-standards reconciled werden. Bis diese Entscheidung freigegeben und die vorhandenen Register konsistent aktualisiert sind, ist die Modellkonformität BLOCKED; dieses Dokument ändert Normen nicht stillschweigend.

### Hauptschichten – Zielabdeckung

| Layer | Soll-Verantwortung | Mindestnachweis |
|---|---|---|
| L0 | Hardware und Compute: CPU/GPU/NPU, RAM, Storage, TPM/TEE, I/O | Hardware-Inventar, Capability-Erkennung, Abwesenheits-/Fehlerfälle |
| L1 | Boot, Firmware, Secure Boot, Plattforminitialisierung, HAL | Bootkette, Firmware-Validierung, Übergabevertrag, Hardwareprofile |
| L2 | ShivaCore-Microkernel und TCB | Prozess-/Speicherisolation, Capabilities, IPC, Fault Handling, Negativtests |
| L3 | GlobusOS-Dienste und OS-Integration | Dienstverträge, Prozess-/Datei-/Netzwerk-/Gerätepfade, Recovery |
| L4 | Desktop, Window Management, UI und Accessibility | UI-Funktionskatalog, Tastatur-/Accessibility-Tests, Fehlerzustände |
| L5 | A-TownChain L1: Node, Netzwerk, Transaktionen, Konsens und Ledger | deterministische Zustandsübergänge, Multi-Node, Validierung, Finalität, Recovery |
| L6 | ATC-VM, Smart Contracts und Ausführungszustand | Bytecode-/ABI-Verträge, Gas-/Ressourcenlimits, deterministische Tests |
| L7 | ATCLang, Compiler, SDK und Developer Toolchain | Parser-/Compiler-Konformität, reproduzierbare Builds, Runtime-Integration |
| L8 | Aurora AI: Control Plane, Agenten, Policies, Tools und Model Backends | Autorisierung, Capability-Prüfung, Approval-Gates, Audit und Sandbox-Tests |
| L9 | Anwendungen und Nutzerprodukte: Wallet, Explorer, Marketplace, Genesis/Apps | End-to-End-Nutzerpfade, Berechtigungen, Fehlerbehandlung, Release-Nachweise |

### Querschnittsbereiche X1–X36

Die folgende Liste ist ein **Abdeckungsraster**, kein Implementierungsstatus. Bestehende normative Namen und IDs müssen im Register abgeglichen werden; keine IDs oder Verträge gelten allein durch ihre Nennung als angelegt.

| Bereich | Soll-Abdeckung |
|---|---|
| X1 | Sicherheit, Trust Boundaries und Threat Modeling |
| X2 | Identität, Zugriff und Capability Authorization |
| X3 | Protokolle, APIs und Interoperabilität |
| X4 | Daten, State, Persistenz, Backup und Migration |
| X5 | Standards, Governance und Policy |
| X6 | Tests, Verifikation und Evidence |
| X7 | Observability, Zuverlässigkeit und Recovery |
| X8 | Build, Release und Software Supply Chain |
| X9 | Konfiguration, Secrets und Environment Separation |
| X10 | Provenance, Signaturen und Artefaktintegrität |
| X11 | Datenschutz, Datenminimierung und Löschung |
| X12 | Performance, Skalierung und Ressourcenbudgets |
| X13 | Determinismus und Reproduzierbarkeit |
| X14 | Netzwerkbetrieb, Discovery und Ausfallverhalten |
| X15 | Kryptografie, Schlüsselverwaltung und Rotation |
| X16 | Kompatibilität, Versionierung und Migration |
| X17 | Dokumentation, Wissensverwaltung und Rückverfolgbarkeit |
| X18 | Ökosystemintegrität, Inventar und Driftkontrolle |
| X19 | Observability der Cross-Repo- und Systempfade |
| X20 | Fehlerdomänen, Resilienz und Degradation |
| X21 | Multi-Tenant-/Multi-User-Isolation |
| X22 | AI-Agent-Governance, Toolzugriff und menschliche Freigaben |
| X23 | UI-, Accessibility- und Nutzerflussabdeckung |
| X24 | API-/ABI-Kompatibilität und Contract Testing |
| X25 | Daten- und Ereignisflüsse, Messaging und Workflow |
| X26 | Performance-Benchmarks und Regression Gates |
| X27 | Security Assurance, Audits und negative Tests |
| X28 | Installation, Provisionierung und Upgrade-Kompatibilität |
| X29 | Betriebs-, Wartungs- und Incident-Prozesse |
| X30 | Lizenz-, Drittanbieter- und Abhängigkeitskonformität |
| X31 | Plattform-/Hardware-Kompatibilität und Testmatrix |
| X32 | Backup, Restore, Crash Recovery und Disaster Recovery |
| X33 | Release-Kriterien, Freigabe und Rollback |
| X34 | Systemkonfiguration, Composition und profilabhängige Features |
| X35 | Installer, Release, Deployment und Update-Kette |
| X36 | Systemkomposition und geschlossene End-to-End-Verifikation |

**Konfliktregel:** Wenn bestehende Dokumente ein anderes Layer-Modell oder andere ID-Zuordnungen führen, ist der Konflikt als Befund mit betroffenen Dateien, SHA, Normreferenz und vorgeschlagener Entscheidung zu erfassen. Bis zur autorisierten Auflösung darf kein automatischer Umnummerierungs- oder Löschlauf erfolgen.

## 4. Pflichtschema für jede Soll-Anforderung

Jede Anforderung wird als maschinenlesbarer Datensatz geführt und mindestens mit folgenden Feldern verbunden:

- Identität: requirement_id, Titel, präzise Sollbeschreibung und Version.
- Normquelle: Repository, exakter Pfad sowie freigegebene Version oder Commit-SHA.
- Architektur: Layer, stabile Komponenten-ID und Funktions-ID; ungeklärte IDs bleiben UNRESOLVED.
- Ownership: kanonisches Repository, exakter Pfad und Symbol.
- Abnahmekriterien: stabile Test-ID, erwartetes beobachtbares Ergebnis und negativer Fehler-/Missbrauchsfall.
- Abhängigkeiten und Sicherheitsinvarianten.
- Evidence-Pflichten: exakter Commit-SHA, Workflow-Run/Job/Step, Exit-Code/Logs und Artefakt-Hashes.
- Beobachteter Zustand: geprüfter Commit, Resultat, offene Lücken und Status.
- Ausnahme: genehmigter Verweis, Verantwortlicher und Prüfzeitpunkt.

Das Schema muss maschinenlesbar als versioniertes JSON- oder YAML-Schema registriert und validiert werden. Dieser Abschnitt allein ist noch kein implementiertes oder validiertes Schema.

## 5. Ökosystem-Inventar und Soll-/Ist-Abgleich

Das Inventar muss alle aktiven, archivierten, externen und ausdrücklich ausgeschlossenen Repositories erfassen. Pro Repository sind mindestens festzuhalten:

- Repository-Name, Status, Default-Branch, verantwortliche Domäne und SSOT-Rolle.
- Vollständiger geprüfter Commit-SHA und Root-Tree-SHA.
- kanonische Komponenten/Funktionen sowie zugehörige Katalog-IDs.
- Snapshot-Commit und Differenz zum kanonischen Quell-Commit, sofern importiert.
- Workflows, verpflichtende Checks, letzte relevante Runs und deren SHA-Bindung.
- offene Architektur-, Sicherheits-, Katalog- und Evidence-Befunde.
- genehmigte Ausnahmen mit Begründung, Verantwortlichem und Review-Datum.

Ein unvollständiger Repository-Census darf nicht als vollständig gemeldet werden. Das demo-repository bleibt vom Live-Drift-Gate ausgeschlossen, sofern die bestehende Governance dies festlegt; die Ausnahme ist explizit und nachvollziehbar zu führen.

## 6. Verifikations- und Release-Gates

| Gate | Abnahmekriterium | Fail-closed-Verhalten |
|---|---|---|
| G0 – Inventory | alle relevanten Quellen erfasst oder als offen/excluded begründet | fehlende Quellen → BLOCKED |
| G1 – Catalog | stabile IDs, keine ungeklärten Dubletten, alle Lücken sichtbar | fehlende/mehrdeutige Zuordnung → BLOCKED |
| G2 – SSOT | jede Implementierung einer autoritativen Quelle zugeordnet | konkurrierende Kernimplementierung → Merge-Block nach Policy |
| G3 – Contract | Inputs, Outputs, Fehler, Invarianten und Rechte festgelegt | fehlender Pflichtvertrag → UNRESOLVED |
| G4 – Component | unit-, negative-, security- und determinism tests nach Vertrag | fehlender oder fehlgeschlagener Test → nicht bestanden |
| G5 – Integration | Cross-Repo-Schnittstellen mit exakten SHAs geprüft | nur Einzel-CI grün → keine E2E-Aussage |
| G6 – System/E2E | vollständige, dokumentierte Systempfade auf unterstützten Profilen | ungetesteter Pflichtpfad → BLOCKED |
| G7 – Release/Deployment | signierte Artefakte, Herkunft, Upgrade/Rollback, Installer und Recovery belegt | fehlender Supply-Chain-/Rollback-Nachweis → Release blockiert |
| G8 – Evidence | Claim ↔ Commit ↔ Run/Job/Step ↔ Logs ↔ Artefakt-Hash nachvollziehbar | fehlende Bindung → BLOCKED |
| G9 – Governance | Ausnahmen, Reviews und Freigaben gültig; keine offenen kritischen Befunde | kritischer offener Befund → Merge/Release blockiert |

Ein Gate kann nur die Kriterien bestätigen, die es tatsächlich ausgeführt hat. CI-Erfolg und Produktionsreife bleiben getrennte Aussagen.

## 7. Mindest-Evidence-Vertrag

Jeder Verifikationsbefund enthält:

- claim_id, requirement_id, component_id und test_id.
- kanonisches Repository, vollständigen Commit-SHA, Pfad, Symbol und Blob-SHA soweit anwendbar.
- Referenz-SHA für den Vergleich, Zeitpunkt und Umgebung/Hardwareprofil.
- Workflow-Run-ID, Job-ID/-Name, Step, Exit-Code und relevante Logs.
- Artefaktname, unveränderliche Referenz und SHA-256-Hash.
- erwartetes Ergebnis, beobachtetes Ergebnis, Einschränkungen und Status.
- Verantwortlichen und Verweis auf einen offenen Befund bei Fehlschlag.

Keine SHAs, Testresultate, Artefakte, Freigaben oder Verantwortlichen dürfen erfunden oder aus benachbarten Commits übernommen werden. Wenn ein Nachweis nicht abrufbar ist, bleibt der Status offen.

## 8. Vollständigkeitskriterien

Der Soll-Zustand darf erst als katalogseitig vollständig bezeichnet werden, wenn:

- [ ] Layer-/Sublayer-Modell, ID-Namensräume und normative Konflikte autorisiert aufgelöst sind.
- [ ] Repository-Census vollständig ist; Ausschlüsse und archivierte Quellen sind begründet.
- [ ] jede bekannte Komponente und Funktion eine stabile ID und eine Zuständigkeit besitzt.
- [ ] jede Anforderung messbare Abnahmekriterien einschließlich negativer Fälle hat.
- [ ] jede bekannte Abhängigkeit/Schnittstelle als Vertrag oder als offene Lücke geführt wird.
- [ ] alle SSOT-Zuordnungen belegt oder explizit UNRESOLVED sind.
- [ ] Katalog, Layer-Register, Dependency Graph und Standards-Registry konsistent sind.
- [ ] Drift-, Dubletten-, Schema- und Evidence-Validierung automatisiert und getestet ist.
- [ ] die Abdeckung je Layer, Komponente, Funktion und Test offen ausgewiesen wird; unbekannte Werte werden nicht als 100 % gezählt.
- [ ] End-to-End- und Release-Nachweise auf exakten SHAs vorliegen, bevor das Gesamtsystem als verifiziert bezeichnet wird.

## 9. Umsetzungsreihenfolge

1. **Registry reconciliation:** aktuelles Layer-Register, Architekturvertrag, Standards-Registry und bestehende Kataloge auf Konflikte prüfen; keine stille Umnummerierung.
2. **Repository census:** aktive/archivierte/ausgeschlossene Repositories, Branches, Commit-SHAs und SSOT-Rollen festhalten.
3. **Catalog gap analysis:** fehlende Komponenten, Funktionen, Schnittstellen und Abnahmekriterien identifizieren.
4. **Schema + validator:** Soll-Anforderungen und Evidence maschinenlesbar validieren; negative Tests für fehlende Felder, falsche SHA-Bindung und doppelte IDs hinzufügen.
5. **Cross-repo reconciliation:** kanonische SHAs und importierte Snapshots vergleichen; Drift nicht automatisch als Implementierungsfehler klassifizieren.
6. **Verification matrix:** Pflicht-Workflows, Profile, Tests, E2E-Pfade und Release-Gates pro Anforderung zuordnen.
7. **Evidence-backed closure:** Befunde einzeln beheben und mit frischer, exakter Evidenz erneut prüfen.

## 10. Aktueller Dokumentstatus

**DRAFT / NOT VERIFIED.** Dieses Dokument strukturiert den vollständigen Soll-Zustand und die erforderliche Beweiskette. Es belegt weder die Vollständigkeit des aktuellen Katalogs noch die Implementierung oder Verifikation des Ökosystems. Die Layer-ID-Zuordnung, Katalogintegration, Validatoren, tatsächliche Repository-Abdeckung und alle zugehörigen CI-Nachweise müssen separat geprüft und freigegeben werden.

### Referenzen

- [Ökosystem-README](../../README.md)
- [Layer-Architektur](ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001.md)
- [Komponenten-/Funktionskatalog (JSON)](ATC-COMPONENT-FUNCTION-CATALOG-001.json)
- [Abhängigkeitsgraph (JSON)](ATC-DEPENDENCY-GRAPH-001.json)
- [Layer-Register und Mapping](layers/)
- [X18 – Ecosystem Integrity Entwurf](../knowledge-system/records/generated/ATC-KMS-ISSUE-163.json)
