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


## 11. Architekturabgleich: KI × Betriebssystem × Blockchain (Audit-Snapshot 2026-10-21)

Dieser Abschnitt ergänzt die bestehende Systemarchitektur; er führt weder ein neues Layer-Modell noch eine zweite Implementierungs- oder Registry-SSOT ein. Er gleicht das Konzept „KI- und Blockchain-integriertes KAI-OS“ gegen die vorhandene Architektur, offene Issues und auffindbare Exact-SHA-/Actions-Evidenz ab.

### 11.1 Kanonische Architektur und Eigentümer

| Fähigkeit / Grenze | Kanonischer Owner / Vertrag | Architekturentscheidung |
|---|---|---|
| KAI-OS Gesamtarchitektur | atc-standards/standards/architecture/ATC-ARCH-001.md; Systemintegration in a-townchain-ecosystem/ARCHITECTURE.md | KAI-OS ist der Architekturbegriff, kein separates OS oder Repository; GlobusOS ist das Betriebssystem. |
| Kernel, TCB, Prozess-/Speicherisolation, IPC | globus-os/modules/atc-shivacore/kernel/ | Nur minimale privilegierte Sicherheitsdurchsetzung im Kernel; KI-, Chain- und Smart-Contract-Logik bleiben außerhalb der TCB. |
| HAL, Gerätedienste, Treiber, Systemlebenszyklus | globus-os | CPU/GPU/NPU/TPU-Zugriff nur über versionierte HAL-/Treiber-/Service-Verträge. Verfügbarkeit von Hardware, Treibern, DMA-Schutz, Secure Boot, TPM oder TEE muss je Plattform nachgewiesen werden; keine pauschale Hardwaregarantie. |
| KI-Agenten, Modelle, Inferenz, Memory und Tool-Planung | aurora-ai | Aurora schlägt vor und orchestriert; Modelloutput verleiht keine Autorität. Die kanonische Runtime-/Tool-Executor-Entscheidung ist durch Aurora Issue #51 noch offen. |
| Policy/Capability/Approval für KI-Aktionen | bestehende Aurora-Policy-/Capability-Verträge plus unabhängige GlobusOS/ShivaCore-Enforcement-Grenze | Keine zweite Policy Engine. Jeder Seiteneffekt muss unmittelbar vor Ausführung durch den kanonischen Autorisierungspfad laufen; fehlende/mehrdeutige Autorisierung verweigert standardmäßig. |
| Blockchain, Transaktionen und Konsens | a-townchain und zuständige kanonische Unterkomponenten | Aurora und GlobusOS werden keine Konsensautorität. Konsens-/Finalitäts- und Protokollentscheidungen bleiben nur bei versioniertem Vertrag und passender Evidenz freigabefähig. |
| Smart-Contract-Ausführung | a-townchain/components/vm | Deterministische VM bleibt eigenständige SSOT; keine zweite VM in Ecosystem, Aurora oder GlobusOS. |
| Wallet und Signatur-/Transferoperationen | kanonisches atc-wallet | Agenten dürfen Transaktionen vorschlagen; Schlüsselzugriff, Signatur und Zahlung benötigen Wallet-Policy, Capability und erforderliche explizite Freigabe. |
| Normative Verträge | atc-standards | Dieses Dokument ist Systemarchitektur, kein Ersatz für normativ versionierte Standards, ABIs, Schemas oder Protokollentscheidungen. |
| Katalog, Integrationsprüfung und Evidence-Aggregation | a-townchain-ecosystem | Keine Produktionsimplementierungen und keine Übernahme fremder SSOTs. a-townchain-os bleibt Orchestrierung/Integration, kein Ersatz für Core-Repositories. |

Referenzierte vorhandene Arbeit: Ecosystem Issue #71 (Master Architecture), Issue #190 (Frontend–Middleware–Backend-Verträge), PR #103 (diese Architekturdatei), PR #203 (Aurora Ecosystem Technical Spec) und PR #218 (Zielzustandskatalog). Diese Arbeitspakete werden konsolidiert statt als parallele Architektur- oder Funktionskataloge dupliziert. Die Registry-/Layer-Migration aus PR #218 ist nicht automatisch durch diese Ergänzung freigegeben.

### 11.2 Ergänzende Synergien: nur als spezifizierte Zielverträge

#### A. Hardwarebeschleunigte lokale KI

- Modell-/Backend-Auswahl muss anhand tatsächlich erkannter Gerätefähigkeiten, Treiber-Versionen, Speicherbudget und Sicherheitsdomäne erfolgen.
- CPU-Fallback darf die Korrektheit erhalten, aber nicht stillschweigend Sicherheits- oder Determinismusgarantien ändern.
- Modellformat, Operator-/Kernel-Support, Quantisierung, numerische Toleranzen, Speicherisolation und Ressourcengrenzen brauchen versionierte Verträge und reproduzierbare Tests.
- GPU/NPU/TPU, TPM/TEE und Secure-Boot-Integration: **NOT VERIFIED** als vollständiger, systemintegrierter Pfad. Ein HAL-Dokument oder ein vorhandener Treiber allein beweist keine sichere Ende-zu-Ende-Integration.

#### B. Verifizierbares / Zero-Knowledge Machine Learning

- Die vorhandene ATC-ZKP-/Proof-Verifikationsarbeit ist nicht gleichbedeutend mit einem verifizierten ML-Proof-System.
- Vor einer Implementierung müssen Beweisziel und Vertrauensmodell festgelegt werden: Inferenzkorrektheit, Daten-/Modellcommitment, Training/Federation oder nur Attestation. Prover, Verifier, Circuit/Constraint-Set, öffentliche/private Inputs, Soundness-Annahmen, Versionierung und Ressourcen-/DoS-Grenzen sind getrennt zu spezifizieren.
- Ein ZKP-Nachweis beweist nur die Aussagen des konkret spezifizierten Circuits/Statements; er beweist weder die Qualität des Modells noch die Richtigkeit nicht erfasster Vorverarbeitung.
- Status für integriertes ZK-ML über Aurora → kanonische ATC-ZKP-/Chain-Verträge: **UNRESOLVED / NOT VERIFIED**. Kein Nachweis gefunden, der diesen gesamten Pfad an einem exakten Commit mit positiven und negativen Tests belegt.

#### C. Föderiertes Lernen / dezentrales Modelltraining

- Rohdaten bleiben standardmäßig lokal; Teilnahme, Gradienten-/Update-Freigabe, Aggregation, Teilnehmeridentität, Modellversion und Abbruch/Widerruf brauchen explizite Verträge.
- Datenschutz (einschließlich möglicher Differential Privacy), Poisoning-/Sybil-Abwehr, Update-Validierung, Reproduzierbarkeit und Leckage-Risiko sind eigenständige Abnahmekriterien.
- On-chain-Verankerung ist optional und beschränkt sich auf explizit vereinbarte Commitments, Teilnehmer-/Rundenmetadaten oder Abrechnung; private Trainingsdaten und rohe Gradienten werden nicht automatisch on-chain publiziert.
- Föderiertes Lernen als vollständiger sicherer, integrierter und CI-verifizierter Dienst: **NOT VERIFIED**. Historische/archivierte Architekturtexte oder Katalogeinträge zählen nicht als Implementierungs- oder CI-Nachweis.

#### D. Autonome Micropayments / agentische Zahlungen

- Ein Agent erstellt zunächst einen typisierten Zahlungsauftrag mit Empfänger, Asset, Betrag, Zweck, Ablauf, Nonce und Budget.
- Danach: Policy → Capability → erforderliche menschliche Freigabe → kanonische Wallet-Signatur → Transaktionsvalidierung → autoritative Chain-Ausführung/Finalität → korrelierte Evidence.
- Budget-/Rate-Limits, Allowlist, Ablauf, Replay-Schutz, Gebührenobergrenze, Rücknahme-/Fehlersemantik und Kill-Switch sind zwingend. Modelloutput darf niemals unmittelbar Schlüsselmaterial verwenden oder eine Zahlung selbst autorisieren.
- Vollständige autonome Zahlung mit realem Wallet-/Chain-Pfad und negativen Autorisierungstests: **BLOCKED / NOT VERIFIED**, bis die kanonischen Wallet- und Agent-Gateway-Verträge sowie Exact-SHA-Integrationstests nachgewiesen sind.

### 11.3 Implementierungs- und CI-Evidenzmatrix

**Auditregel:** Die folgenden Einträge sind gezielte, zum Prüfzeitpunkt auffindbare Evidenzanker, kein vollständiges Repository-Census. Ein vorhandener Quellpfad, ein offener PR oder ein erfolgreicher Teil-Workflow beweist nicht, dass die gesamte Architektur implementiert ist. Fehlende oder nicht abgeschlossene Nachweise bleiben nicht freigabefähig.

| Prüffeld | Exact-SHA / Referenz | Beobachtete Evidence | Status für die hier beschriebene Gesamtfähigkeit |
|---|---|---|---|
| Systemarchitektur / Ecosystem-PR #103 | PR-Head 35ca8d96cb9dc6881c8d2a09b4bba7c546125d41; zugehöriger Merge-Ref 3a53502ea68f970f5fdd28cc2f82a1196e9fcd4d | Actions Run [#37852262191](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/actions/runs/37852262191): Python-, Rust-Manifest- und Repository-Integrity-Jobs bestanden; javascript-projects fehlgeschlagen. Step „Validate every package.json project“ meldet Dependency-Auflösung für components/a-townchain/components/explorer: TypeScript 7.0.2 kollidiert mit ts-jest@29.4.14 (Peer-Bereich >=4.3 <7); zusätzliche npm-CLI-/Installationsprobleme wurden für SDK-Pakete protokolliert. | **BLOCKED** für Release-/Gesamtintegration; dieser Run ist außerdem historisch und prüfte den Merge-Ref, nicht nur den isolierten PR-Head. |
| ShivaCore Kernel / PR #98 | 4efd7be7460816e229bdb88efd6e83fcea0febbf | [Kernel-Boot-Run #39222390403](https://github.com/A-TownChain-Okosystems/globus-os/actions/runs/39222390403): Rust CI, GlobusOS System CI, ATC Test Suite, Governance, SDK Build/Test und Security Audit meldeten Erfolg. BIOS-Smoke-Test für Intel Skylake war beim Audit noch in_progress; UEFI-Smoke-Test und SHA-gebundenes Bootmanifest standen noch aus. AMD EPYC Job war cancelled. PR #98 bleibt Draft und beschreibt ausdrücklich nur fail-closed Kernel-Stopp, keine sichere Ring-3-Prozesswiederherstellung. | **NOT VERIFIED** für vollständige Boot-/Isolation-/AI-Systemintegration; Teil-CI erfolgreich, Boot-Evidence unvollständig. |
| Ring-3 Fault-Recovery / PR #99 | 9c129b295415dc592736c1f57c50644e33065408 | PR #99 ist Draft; für genau diesen SHA wurden in der abgefragten PR-Workflow-Runs-Antwort keine Runs geliefert. Der PR-Text sagt Build, Tests, QEMU und negative Cross-Process-Isolationstests seien noch nicht verifiziert. | **BLOCKED / NOT VERIFIED**. |
| Aurora autorisierter Tool-Executor / PR #48, Issue #52 | 0ed3520dcc7838373916fa76cb7180c166f0d508; Basis-Audit von Issue #52: 032519b5bac505b25651214ec717f448264c8948 | PR #48 ist Draft. Er enthält einen lokalen authorize_action()-Gate und beschriebene Regressionstests, sagt aber, dass diese Tests in dieser Änderung nicht lokal ausgeführt wurden und der exakte Head CI benötigt. Issue #52 hält fest, dass AgentPool::execute() direkt dispatcht und ein vollständiger Gateway-Pfad nachzuweisen ist. Für den PR-Head wurden in der abgefragten Workflow-Runs-Antwort keine Runs geliefert. | **BLOCKED / NOT VERIFIED** für „jeder Side-Effect ist autorisiert“. |
| Aurora Memory / PR #45, Issue #44 | 70adba7ec7c0a9b35cdac9cf5fdd14e35b546262 | Draft; PR beschreibt persistentes Long-Term-Memory-MVP, aber Payloads sind nicht at-rest verschlüsselt, Multi-Process-Locking und authentifizierte Integrität fehlen, abgeleitete Index-/Embedding-/Cache-Löschung und OS-Capability-Enforcement sind offen. Exact-Head Build/Test/Security/Governance-Evidence war im PR-Text ausstehend; keine Runs wurden für den Head in der abgefragten Antwort geliefert. | **RESIDUAL / NOT VERIFIED**; nicht als sichere vollständige Memory-Schicht ausgeben. |
| Blockchain / ZKP-Basis | a-townchain Architektur-/ZKP-Dateireferenz bei Commit-Snapshot a5e689671b1870699c7a1750f8a166e83590a41c | Quell- und Architekturdateien für ZKP/Verifier sind auffindbar. Der abgefragte Commit-Status lieferte keine Statuschecks. Das beweist weder erfolgreiches CI noch ein ML-spezifisches Circuit-/Prover-/Verifier-System. Zusätzlich hält Ecosystem-Katalogarbeit offene Protokoll-/Konsens- und VM-Entscheidungen fest. | ZKP-Primitiven: **IMPLEMENTATION STATUS NOT ASSESSED HERE**; integriertes ZK-ML und Gesamt-Chain-Pfad: **UNRESOLVED / NOT VERIFIED**. |
| Normativer Architekturstandard | atc-standards ATC-ARCH-001.md, Commit-Snapshot 68448117334b49729f423f1648f9a960f4411220 | Standard ATC-ARCH-001 und Registry-Eintrag sind als APPROVED auffindbar; die abgefragte Commit-Status-API lieferte keine Statuschecks. Normative Freigabe des Architekturbegriffs ist kein Nachweis technischer Implementierung oder CI/E2E-Konformität. | Standardstatus: **APPROVED**; Implementierungs-/E2E-Konformität: **NOT VERIFIED**. |
| Föderiertes Lernen, ZK-ML, Hardware-TPM/TEE-Bindung, agentische Micropayments | Kein zusammenhängender exakter Source-SHA + Run/Job/Step/Exit/Artefakt-Nachweis im geprüften Ausschnitt gefunden | Architekturbeschreibungen und benachbarte Komponenten sind kein vollständiger End-to-End-Nachweis. | **BLOCKED / UNRESOLVED / NOT VERIFIED**, jeweils bis zum geforderten Nachweis. |

**Wichtig zur Aktualität:** Die oben angegebenen SHAs sind konkrete Auditanker aus den abgefragten PRs/Dateien; sie dürfen nicht stillschweigend als aktueller Default-Branch-HEAD jedes Repositories ausgegeben werden. Vor Freigabe muss die Matrix auf den dann aktuellen kanonischen HEADs erneut erhoben werden. Die API-Antwort „keine Statuschecks/keine Workflow-Runs“ ist kein Beweis, dass nie CI existierte; sie bedeutet nur, dass mit der hier abgefragten Evidenz kein positiver Exact-SHA-Nachweis vorliegt.

### 11.4 Verbindliche E2E-Abnahmekette

Für jede integrierte Funktion ist die folgende Kette einzeln und negativ zu testen:

**Intent → Agent Identity → Policy Decision → Capability Check → Approval (if required) → Versioned Tool/IPC Contract → Isolated Service → Canonical Wallet/ATC-VM/OS Owner → Result/Finality → Audit Evidence**

Mindest-Negativfälle:
- unbekannte, widerrufene oder abgelaufene Identität/Capability;
- fehlende, mehrdeutige oder abgelehnte Policy-/Approval-Entscheidung;
- Replay, falsche Nonce, abgelaufener Auftrag und geänderte Parameter nach Freigabe;
- Cross-Process-/Cross-Tenant-Zugriff auf Memory, Schlüssel oder Geräte;
- ungültige Transaktion, fehlerhafte VM-Ausführung, fehlende Finalität oder Netzwerkpartition;
- nicht unterstützter GPU/NPU/TPU-Backendpfad, Ressourcenerschöpfung, Modell-/Treiber-Mismatch;
- ungültiger oder nicht zum Statement passender ZK-Beweis, manipuliertes Modell-/Update-Artefakt;
- Federated-Learning-Teilnehmer mit Poisoning-/Sybil-Signalen oder widerrufener Identität;
- fehlende Evidence, SHA-Mismatch, fehlender Exit-Code, fehlgeschlagener oder übersprungener CI-Step.

Für jeden Test sind Requirement-ID, Owner-Repository, vollständiger Quell-SHA, Pfad/Symbol/Blob-SHA, Testname, Actions Run → Job → Step → Exit-Code, Logs und Artefakt-Hashes zu erfassen. Ein grüner Teilworkflow, ein Dokument oder ein Mapping allein erfüllt diese Kette nicht.

### 11.5 Offene Entscheidungen und Freigabesperren

1. **Aurora Runtime/Executor SSOT:** Issue [aurora-ai #51](https://github.com/A-TownChain-Okosystems/aurora-ai/issues/51) zuerst entscheiden; keinen dritten Executor und keine parallele Policy-/Agent-/Memory-/Model-Registry einführen.
2. **Vollständiger Autorisierungspfad:** [aurora-ai #52](https://github.com/A-TownChain-Okosystems/aurora-ai/issues/52) schließen und negative Tests nachweisen; PR #48 allein belegt keine Vollabdeckung.
3. **Kernel-Isolation:** [globus-os PR #98](https://github.com/A-TownChain-Okosystems/globus-os/pull/98) exakt abschließen; [PR #99](https://github.com/A-TownChain-Okosystems/globus-os/pull/99) erst nach Build-, Boot-, Ring-3-Recovery- und Cross-Process-Negativtests als vollständig bewerten.
4. **Normative Contracts:** neue API-/ABI-/Schema-Anforderungen in atc-standards versionieren und freigeben; nicht allein durch Ecosystem-Dokumentation normativ machen.
5. **Zielmodell/Registry:** PR #218 und die bereits offene L0–L7+X-Masterarchitektur koordinieren. Diese Ergänzung genehmigt weder eine Migration von L0–L10/X1–X8 auf L0–L9/X0–X36 noch eine Umdeutung vorhandener Layer.
6. **Gesamtfreigabe:** erst nach erneuter Exact-HEAD-Erhebung, erfolgreicher Doku-/Schema-Prüfung, geklärtem Ecosystem Release-Readiness-Fehler und vollständiger E2E-Evidence. Bis dahin: **ARCHITECTURE PROPOSAL / INTEGRATION NOT VERIFIED / RELEASE BLOCKED**.
