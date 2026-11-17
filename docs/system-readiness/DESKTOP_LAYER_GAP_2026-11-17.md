# Desktop-Schicht: Ist-Bestand gegen ATC-ARCH-002

**Datum:** 17.11.2026 · **Grundlage:** ATC-ARCH-002 (DRAFT, atc-standards PR #127), SCR-0142
**Methode:** Datei- und Testinventur in `globus-os`, `aurora-ai`, `a-townchain`; keine Schätzung
**Zweck:** Zeigen, was für einen bedienbaren Desktop fehlt, bevor Implementierung beginnt

## Bilanz

| Schicht | Zustand | Was vorhanden ist | Was fehlt |
|---|---|---|---|
| D0 Display | teilweise | Modell in `system/graphics` (190 Composer, 377 WM, 449 Desktop, 252 Display, 288 GPU), 23 Tests, konsumiert von `system/runtime` | Scanout-Pfad, Mode-Setting, Present-Loop, echter Treiber |
| D1 Session | teilweise | `input.rs` (173), Kernel `input_queue.rs`, `greeter/src/main.rs` (459) | Seat-Manager, Sitzungswechsel, Capability-Tokens pro Sitzung |
| D2 Shell | fehlt als GUI | Desktop-, WM- und Shell-Modell (1028 Zeilen zusammen) | Startmenue, Taskleiste, Benachrichtigungen, Einstellungen-UI |
| D3 Dateien | fehlt als UI | VFS im Kernel | Dateimanager, Verschluesselungs-UI, Netzlaufwerke |
| D4 Runtime/Sandbox | teilweise | Capability-Kern (`cap_authority.rs`, `system/security`), `system/runtime` | App-Sandbox-Profile, Portal-Modell, Capability-Deklaration je App |
| D5 Pakete/Update | teilweise | `system/package` (532), `system/update` (435: boot_state, health, transition) | Marktplatz-UI, Signaturkette, belegter Rollback |
| D6 Kompatibilitaet | Platzhalter | `modules/atc-linux-edition`, `modules/atc-windows-edition` | jegliche Laufzeit; GateToHell ist in globus-os nicht vorhanden |
| D7 Assistenz | extern | `system/aurora-core`, Repo `aurora-ai` | Desktop-Schnittstelle, Portal-Bindung, Audit je KI-Aktion |

## Drei belegte Nullbefunde

Diese drei Begriffe kommen im gesamten `globus-os` nicht vor: **portal** (0 Treffer),
**accessibility** (0), **locale/i18n** (0). Damit fehlen genau die drei Systemdienste,
die aus einem Fenstermanager einen benutzbaren Desktop machen: vermittelte Zugriffe,
Barrierefreiheit, uebersetzte Oberflaeche. ATC-ARCH-002 verlangt sie in REQ-ARCH-213/214
(Portale, Default Deny) und REQ-ARCH-224/225 (Barrierefreiheit, Lokalisierung).

## Einordnung

Der Bestand ist besser als erwartet, aber anders verteilt als erwartet: Es gibt
ueberzeugende **Modelle** fuer Fenster, Fokus, Platzierung und GPU-Klassifikation,
und es gibt **keine** Produktoberflaeche. `modules/atc-globus-desktop` ist leer.
`modules/atc-globus-shell` ist eine CLI-Shell (293 Zeilen), kein Desktop-Shell.

Die Grafik ist angebunden, aber nicht dort, wo man sie vermuten wuerde: nicht in
`service_space`, sondern `system/runtime` haengt per Pfad auf `globus-graphics`.

## Abgleich gegen die 32 Anforderungen

| Status | Anzahl | Anforderungsgruppen |
|---|---|---|
| gedeckt | 7 | Profil-Einmaligkeit, Repo-Zuordnung, keine Konsenssemantik, Default Deny, ShivaCore als Rechtequelle, ACTIVE-Gate |
| teilweise | 13 | Display, Eingabe, Anmeldung, IPC-Schema, Sandbox, Keyring, Audit, Pakete, Update, Reproduzierbarkeit, KI, Abhaengigkeiten |
| nicht vorhanden | 12 | Sitzungsdienst, Schema-Majorierung, Portale, Signaturpruefung, Anwendungskategorien, Capability-Mapping, Browser, Wiederherstellung, Bedienbarkeit, Barrierefreiheit, Lokalisierung, Telemetrie |

Die Zuordnung ist im Detail in `DESKTOP_LAYER_REQ_MATRIX.yaml` daneben abgelegt, damit
sie maschinell geprueft werden kann.

## Empfohlene Reihenfolge

1. **D1 Sitzung.** Ohne Sitzungsidentitaet ist jede Capability-Zuordnung unbelegt. Kleinster
   Schritt mit groesster Wirkung, weil `greeter` bereits laeuft und `input_queue` existiert.
2. **D0 Scanout und Present-Loop.** Das Modell ist da, der Weg auf den Bildschirm fehlt.
3. **D2 Shell-Minimum.** Taskleiste, Startmenue, Fensterrahmen, Benachrichtigungen. Erst danach
   hat der Desktop eine pruefbare Oberflaeche.
4. **Portale vor Anwendungen.** Wer zuerst Anwendungen baut, erfindet Rechtewege, die
   ATC-ARCH-002 spaeter verbietet.
5. **D5 mit Signaturkette und belegtem Rollback**, nicht nur mit Paketaufloesung.

## Grenzen dieser Erhebung

Gezaehlt wurden Dateien, Zeilen, Tests und Abhaengigkeiten. Nicht geprueft wurde, ob die
vorhandenen Modelle unter QEMU tatsaechlich Bilder erzeugen; der Boot-Pfad ist ein eigener
Nachweis (P0 #45, Hardware-Kette). Die Aussage dieses Dokuments ist daher: **was fehlt**,
nicht: **was laeuft**.
