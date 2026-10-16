# Dezentrale KI-Betriebssystem-Architektur

- **Status:** ARCHITECTURE PROPOSAL — keine Implementierungs- oder Produktionsfreigabe
- **Geltungsbereich:** System-of-Systems-Architektur für ShivaCore, GlobusOS, Aurora AI und A-TownChain
- **Architektur-SSOT:** `a-townchain-ecosystem` für Systemgrenzen und Integration
- **Normative Standards-SSOT:** `atc-standards`
- **Verbindliche Leitlinie:** Standalone First, Ecosystem Second
- **Bezug:** Issue #71 — Master Architecture: canonical A-TownChain layer model L0–L7 + Cross-Layer Control Plane

## 1. Ziel und Nicht-Ziele

Ziel ist ein lokal funktionsfähiges, sicherheitsorientiertes und dezentral vernetzbares KI-Betriebssystem. Einzelne Knoten müssen grundlegende lokale Aufgaben ohne Cloud-KI, Internetzugang oder erreichbare Blockchain ausführen können. Netzwerk- und Schwarmfunktionen erweitern den Betrieb, dürfen aber keine Voraussetzung für die lokale Sicherheitsdurchsetzung sein.

Diese Architektur spezifiziert Verantwortlichkeiten und Schnittstellen. Sie behauptet nicht, dass die beschriebenen Fähigkeiten bereits implementiert, getestet, integriert oder produktionsreif sind.

Nicht-Ziele:
- Aurora AI ist kein Kernel, keine Konsensinstanz und keine autoritative ATC-VM.
- GlobusOS ersetzt weder ShivaCore noch die kanonische ATC-VM.
- Die Blockchain ist kein universeller Speicher für private Nutzerdaten, KI-Gedächtnis oder hochfrequente Telemetrie.
- Das Ecosystem-Repository dupliziert keine fachlichen Implementierungen aus den kanonischen Quell-Repositories.

## 2. Architekturkomponenten und Verantwortlichkeiten

### 2.1 ShivaCore — Security Foundation

ShivaCore ist der kleine, privilegierte Sicherheitskern und die maßgebliche Durchsetzungsgrenze für Kernelressourcen.

Verantwortlichkeiten:
- Capability-basiertes Berechtigungsmodell für Prozesse, Objekte und Ressourcen.
- Prozess-, Speicher- und IPC-Isolation sowie kontrollierte Syscalls.
- Scheduler-, Speicher- und Ressourceninvarianten.
- Kontrollierte Übergänge zwischen Kernel und Service Space.
- Einbindung der Plattform-Vertrauensbasis für Boot- und Schlüsseloperationen, soweit Hardware dies unterstützt.
- Fehlerbehandlung nach fail-closed Prinzip, wo ein sicherer Zustand sonst nicht nachweisbar ist.

Nicht verantwortlich für:
- KI-Inferenz, Modelltraining, P2P-Anwendungsprotokolle, Blockchain-Konsens oder Smart-Contract-Ausführung.

### 2.2 GlobusOS — System- und Serviceplattform

GlobusOS integriert Hardware und isolierte Dienste oberhalb des ShivaCore-Kernels.

Verantwortlichkeiten:
- CPU/GPU/NPU/Security HAL und definierte Treiberverträge.
- EventBus, IPCBus, Service APIs und Service Registry.
- Netzwerktransport, Dateisystem, Zeit-, Geräte- und Prozessdienste.
- Service Manager, Lebenszyklus, Health Checks, Diagnose und Recovery.
- Ressourcenverwaltung und isolierte Ausführungsumgebungen.
- Signierte Systemupdates, Versionsverwaltung und Rollback-Unterstützung.

Netzwerkprotokolle, KI-Engines und Vertragsausführung laufen als begrenzte Dienste im Service Space; sie werden nicht ohne begründete TCB-Entscheidung in den Kernel aufgenommen.

### 2.3 Aurora AI — Intelligenz- und Agentenplattform

Aurora AI interpretiert Ziele, erzeugt Vorschläge, plant Arbeit und koordiniert KI-Agenten. Aurora ist außerhalb der ShivaCore TCB und besitzt keine pauschale Kernel-Autorität.

Vorgesehene Subsysteme:
- **Aurora Runtime:** Ausführung von Agenten, Workflows und Aufgaben.
- **ATC Model ABI:** versionierte Schnittstelle zwischen Modellformaten, Inferenz-Engines und Hardware-Backends.
- **ATC AI Runtime:** standardisierte Laufzeitintegration, Ressourcenlimits und Backend-Auswahl.
- **Aurora Authority Plane:** Policy-Anwendung, Freigabeanforderungen und kontrollierte Autorisierung; keine Umgehung der tatsächlichen Capability-Durchsetzung.
- Modell- und Adapterverwaltung, Tokenisierung, Kontextverwaltung und Inferenz.
- Gedächtnis, semantische Suche, RAG, Quellenherkunft, Aufbewahrung und Löschung.
- Tool-Adapter, Ergebnisvalidierung, Agentenkoordination und optionale Schwarmfunktionen.
- Optionales föderiertes Lernen mit expliziten Datenschutz- und Modellintegritätskontrollen.

Ein Modelloutput ist immer eine nicht autoritative Empfehlung. Er wird erst nach Policy- und Capability-Prüfung zu einem zulässigen Auftrag.

### 2.4 Dezentrale Identitäts- und Netzwerkschicht

Verantwortlichkeiten:
- Kryptografische Identitäten für Geräte, Benutzer, Workloads, Dienste und Peers.
- Peer Discovery, sichere Sitzungen, verschlüsselter Transport und Routing.
- Schlüsselrotation, Widerruf, Vertrauensdomänen und Recovery.
- Schutz gegen Replay, Spoofing, Sybil-Angriffe, manipulierte Nachrichten und bösartige Peers.
- Synchronisierung, Replikation, Konfliktauflösung und Wiederverbindung.
- Offline-fähige lokale Warteschlangen und definierte Verhalten bei Netzwerkpartitionen.

Eine bestehende Verbindung oder Netzwerkposition allein begründet kein Vertrauen. Jede sensible Operation muss am zuständigen Dienst anhand der Identität, des konkreten Auftrags und der Berechtigung geprüft werden.

### 2.5 A-TownChain — verifizierbarer gemeinsamer Zustand

A-TownChain stellt Blockchain-Protokollfunktionen bereit, wenn mehrere Teilnehmer einen gemeinsam verifizierbaren Zustand benötigen.

Verantwortlichkeiten:
- Transaktions- und Wallet-Schnittstellen sowie kryptografische Signierung.
- Kanonische ATC-VM und kanonische Algorithmus-/Konsensimplementierung in den zuständigen SSOT-Repositories.
- Validierung von Transaktionen und deterministischen Zustandsübergängen.
- Protokollgemäße Finalität, Governance und ausgewählte gemeinsame Nachweise.
- Sichere Einreichung zuvor lokal gepufferter Transaktionen, sobald Konnektivität besteht.

Lokale Berechtigungen, private Daten, Modellgewichte und internes KI-Gedächtnis werden nicht automatisch on-chain gespeichert. Blockchain-Zustand ist nur dort autoritativ, wo das Protokoll dies ausdrücklich definiert.

## 3. Durchgängige Vertrauens- und Sicherheitsdienste

### 3.1 Identitäts- und Schlüsselverwaltung

Das System muss unterscheiden zwischen:
- Geräte- und Hardwareidentität;
- Benutzeridentität;
- Dienst- und Workload-Identität;
- Agentenidentität;
- Signier-, Verschlüsselungs- und Vertrauensankerschlüsseln.

Erforderliche Funktionen sind geschützte Schlüsselspeicherung, Schlüsselrotation, Widerruf, Ablauf, Recovery und Audit. Hardwaregestützte Garantien sind nur dann zu behaupten, wenn die jeweilige Plattform sie tatsächlich bereitstellt und die Integration geprüft wurde.

### 3.2 Policy Engine und Capability Enforcement

Jede sensible Aktion wird mindestens anhand folgender Informationen bewertet:
- Akteur und kryptografisch belegte Identität;
- Zielressource und konkrete Operation;
- vorhandene Capability und deren Gültigkeitsbereich;
- Policy-Version und Kontext;
- Ressourcenbudget, Risiko und erforderliche Freigabe;
- Ablaufzeit, Replay-Schutz und Korrelations-ID.

Die Policy Engine entscheidet über Zulässigkeit und Freigabebedarf. ShivaCore beziehungsweise der zuständige isolierte Dienst setzt die tatsächlichen Rechte durch. Eine positive Policy-Antwort allein ersetzt keine Capability.

### 3.3 Software- und Modell-Lieferkette

Für Kernel, Treiber, Dienste, Agenten, Modelle und Pakete sind abhängig von der Vertrauensklasse festzulegen:
- unveränderliche Versionskennung und kryptografischer Integritätsnachweis;
- Herkunft, Lizenz, Abhängigkeiten und Freigabestatus;
- signiertes Manifest und nachvollziehbare Build-Provenienz;
- Kompatibilitäts- und Konformitätsprüfung;
- Rollback- und Wiederherstellungsstrategie;
- Widerruf kompromittierter Artefakte und Schlüssel.

Modellgewichte sind als nicht vertrauenswürdige Eingaben zu behandeln, bis Format, Herkunft, Integrität und Laufzeitbeschränkungen geprüft wurden. Ein gültiger Hash beweist Integrität, nicht Sicherheit oder Qualität.

## 4. Autorisierter KI-Aktionspfad

Für jede sicherheitsrelevante Aktion gilt folgender logischer Pfad:

1. **Intent / Proposal:** Aurora erstellt einen typisierten Vorschlag mit Ziel, Parametern und Begründung.
2. **Policy Evaluation:** Der zuständige Policy-Dienst prüft Regeln, Kontext, Risiko und Freigabebedarf.
3. **Capability Check:** Die ausführende Identität weist die erforderlichen, begrenzten Rechte nach.
4. **Approval Gate:** Menschliche Freigabe, falls die Policy sie verlangt.
5. **Bounded Tool Invocation:** Nur der freigegebene Auftrag mit festgelegtem Parameter- und Ressourcenumfang wird übergeben.
6. **Service Boundary:** Der Zielservice validiert Eingaben erneut und begrenzt Nebenwirkungen.
7. **Authoritative Runtime:** Der zuständige Dienst beziehungsweise die deterministische Runtime führt die Operation aus.
8. **Evidence and Result:** Ergebnis, Status, Versionen und relevante Nachweise werden erfasst.

Ablehnung, Zeitüberschreitung, fehlende Berechtigung und nicht verfügbare Dienste müssen definierte Fehlerzustände erzeugen. Es darf keinen impliziten Fallback geben, der Rechte erweitert oder eine abgelehnte Aktion dennoch ausführt.

## 5. Register und Speicherbereiche

| Register / Speicher | Mindestinhalt | Sicherheitsgrenze |
|---|---|---|
| Systeminventar | Hardware, Fähigkeiten, Treiber, Versionen | Systemadministration |
| Identitäts-/Vertrauensregister | Identitäten, Vertrauensanker, Widerruf | Identitätsdienst |
| Capability-/Policy-Register | Rechte, Regeln, Versionen, Freigaben | Policy- und Autorisierungsdienst |
| Service Registry | Dienst-ID, API/ABI, Version, Status, benötigte Rechte | Service Manager |
| Modellregister | Modell-ID, Format, Hash, Herkunft, Lizenz, Ressourcenprofil | Modellverwaltung |
| Gedächtnis-/Wissensspeicher | Daten, Embeddings, Quellen, Zugriff, Aufbewahrung, Löschung | Nutzer- und Datendomäne |
| Software-/Release-Register | Manifeste, Signaturen, Abhängigkeiten, Provenienz, Rollback | Update-/Release-Dienst |
| Evidence Registry | Run-ID, exakter SHA, Workflow-/Job-/Step-Resultate, Logs, Residuals | Governance-/Assurance-Dienst |
| Betriebsregister | Health, Fehler, Ressourcen, Recovery-Status | Betriebs- und Diagnosezugriff |

Diese Register bleiben logisch und zugriffsseitig getrennt. Ein zentraler Verzeichnisdienst darf nicht automatisch Lesezugriff auf private Gedächtnisdaten oder Schlüssel erhalten. Aufbewahrung und Löschung sind für jede Datenklasse festzulegen.

## 6. Kanonische Repository-Zuordnung

| Domäne | Kanonischer Verantwortungsbereich |
|---|---|
| ATCLang | Sprachdefinition, Syntax, Semantik und Sprachwerkzeuge |
| atc-standards | Normative Standards, Protokolle, Formate, Konformitätsregeln |
| ShivaCore-Kernel | `globus-os/modules/atc-shivacore/kernel/` gemäß bestätigter Kernel-SSOT |
| GlobusOS | HAL, Treiberintegration, Systemdienste und Betriebssystemintegration |
| aurora-ai | Agenten, KI-Planung, Modellintegration und KI-spezifische Runtime-Funktionen |
| ATC-VM | `a-townchain/components/vm` als kanonische Implementierung |
| ATC-Algorithm | `a-townchain/components/algorithm` als kanonische Implementierung |
| atc-toolchain | Build, Tests, Paketierung und Entwicklungswerkzeuge |
| a-townchain-ecosystem | Systemgrenzen, Architektur, Integrationskontrolle, Governance und Evidence-Aggregation |

Diese Tabelle setzt die bestehenden SSOT-Verträge nicht eigenständig um. Wenn ein vorhandener Registry-Eintrag oder ein bestätigter Migrationsstand abweicht, muss der Konflikt zuerst in der zuständigen SSOT geklärt werden. Es dürfen keine parallelen produktiven Implementierungen im Ecosystem angelegt werden.

## 7. Umsetzungsphasen

### Phase 0 — Architekturverträge
- TCB und Service-Space Grenzen dokumentieren.
- Threat Model, Identitätsmodell, Capability-Modell und Policy-Verträge definieren.
- APIs/ABIs, Fehlerzustände, Versionsregeln und Abnahmekriterien festlegen.
- Widersprüche gegen `ARCHITECTURE.md`, Registry und `atc-standards` auflösen.

### Phase 1 — Basis-OS
- Bootpfad, Kernel, Speicher-/Prozessverwaltung und IPC stabilisieren.
- HAL und minimale Systemdienste anbinden.
- Tests, Build-Provenienz, Sicherheitsinvarianten und Recovery prüfen.

### Phase 2 — Lokale Aurora Runtime
- Modell-ABI und Inferenzdienst integrieren.
- Gedächtniszugriff und Tools isolieren.
- Policy-/Capability-geprüften Aktionspfad testen.
- Lokalen Betrieb ohne Cloud-Abhängigkeit demonstrieren.

### Phase 3 — Sicherer Knotenverbund
- Geräte-/Dienstidentitäten, verschlüsselte P2P-Sitzungen und sichere Synchronisierung implementieren.
- Netzwerkpartition, Widerruf, Replay und Wiederverbindung testen.
- Offline-Verhalten und Konfliktregeln nachweisen.

### Phase 4 — Schwarm und verteiltes Lernen
- Agentenaufgaben, Vertrauensgrenzen und Ergebnisvalidierung definieren.
- Ressourcenabrechnung und Fehler-/Byzantine-Verhalten testen.
- Föderiertes Lernen nur mit eigener Datenschutz- und Integritätsbewertung aktivieren.

### Phase 5 — Ökosystemintegration
- Blockchain- und App-Integrationen über versionierte Schnittstellen anbinden.
- Signierte Releases, Update-/Rollback-Verhalten und Konformität testen.
- Systemübergreifende E2E-Prüfung und Release-Freigabe anhand exakter SHA-Evidence durchführen.

Die Phasen sind eine Roadmap, keine Aussage über den aktuellen Status eines Repositories.

## 8. Abnahmekriterien

Ein Meilenstein gilt erst als VERIFIED, wenn seine Kriterien am exakten Quell-SHA mit nachvollziehbarer Evidence geprüft wurden.

- **Boot/Kernel:** reproduzierbarer Build beziehungsweise dokumentierte Reproduzierbarkeitsgrenzen, Kernel-Tests und geprüfte Sicherheitsinvarianten.
- **Isolation:** ein kompromittierter KI-Dienst kann keine nicht freigegebenen Kernel- oder Dienstrechte übernehmen.
- **Lokale Autonomie:** definierte Kernaufgaben funktionieren ohne Internet, Cloud-KI oder erreichbare Blockchain.
- **Autorisierung:** jeder sensible Tool-Aufruf wird vor Ausführung geprüft; Ablehnungen bleiben wirksam.
- **P2P-Sicherheit:** Identität, Replay-Schutz, Widerruf, Partition und Wiederverbindung sind getestet.
- **Artefaktvertrauen:** Software- und Modellmanifest, Integrität, Version und Rollback werden geprüft.
- **Datenintegrität:** Replikation und Konfliktauflösung liefern definierte, nachvollziehbare Resultate.
- **Evidence:** Run-ID, Commit-SHA, Workflow-Run, Job/Step-Resultate, Logs und verbleibende Risiken sind verknüpft.
- **E2E:** Intent → Policy → Capability → Service Boundary → autoritative Ausführung → Evidence ist getestet.
- **Keine falschen Statusclaims:** Architekturtext, Implementierung, Tests, CI, Integration und E2E bleiben getrennte Statusdimensionen.

## 9. Evidence-Statusmodell

Für jedes Kriterium sind mindestens folgende Felder zu erfassen:
- `status`: `MISSING`, `IMPLEMENTED`, `BLOCKED`, `VERIFIED` oder `RESIDUAL`;
- `source_sha`: exakter getesteter Commit;
- `run_id`: unveränderliche CI-/Test-Run-ID;
- `evidence`: konkrete Workflow-, Job-, Step- und Logreferenzen;
- `residuals`: offene Risiken, Einschränkungen oder nicht geprüfte Pfade;
- `owner_ssot`: zuständiges Quell-Repository.

`IMPLEMENTED` bedeutet nicht `VERIFIED`. Fehlende Evidence darf nicht als Erfolg interpretiert werden. Rote Gates werden durch Ursachenbehebung und erneute Prüfung korrigiert; Gates dürfen nicht abgeschwächt werden, um einen grünen Status zu erzeugen.

## 10. Normative Abgrenzung

Dieses Dokument definiert Systemarchitektur und vorgeschlagene Abnahmekriterien. Es ersetzt keine normativen Protokoll-/ABI-Spezifikation in `atc-standards`, keine Implementierung in einem Komponenten-SSOT und keine CI-, Sicherheits- oder E2E-Evidence.

Konflikte mit der kanonischen L0–L7+X-Architektur werden in `ARCHITECTURE.md` und dem zugehörigen Master-Architecture-Issue konsolidiert; es darf kein konkurrierendes Layer-Modell entstehen.
