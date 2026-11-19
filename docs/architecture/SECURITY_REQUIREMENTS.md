---
document_id: ATC-KAI-OS-SECURITY-001
title: Security Requirements
version: 0.1.0
status: DRAFT
type: Anlage zu ATC-KAI-OS-ROADMAP-001
---

# Security Requirements

Externe Orientierung: NIST SP 800-218 (SSDF). Interne Autoritaet bleibt bei
`atc-standards`.

## Konsenskritisch

- Kein `unwrap()` in Konsens- oder Sicherheitscode.
- Fail-Closed bei Autorisierungs-, Signatur- und Identitaetsfehlern.
- Determinismus: keine Systemzeit, keine Zufallswerte, keine ungeordnete Iteration in Ausfuehrungspfaden.
- Eine Authority-SSOT im Kernel, keine parallele Rechteverwaltung.

## Kernel und Betriebssystem

- Capability-basierte Zugriffskontrolle mit Owner-only Create, Mint, Delegate, Revoke.
- Vollstaendige Ressourcenfreigabe, nachweisbar getestet (HandleClose).
- DMA-Autorisierung vor Geraetezugriff.
- Vertrauenskette von Secure Boot bis zum Paketdienst, mit Fail-Closed-Hoechstalter fuer den Trust-Store.

## KI-Domaene

- Modellisolierung und Ressourcenbegrenzung.
- Tool-Zugriffe ausschliesslich capability-vermittelt.
- Lokale Verarbeitung als Standard, externe Adapter kontrolliert.

## Lieferkette

- Reproduzierbare Builds, SBOM, signierte Artefakte, Dependency-Pruefung mit OSV-Intervallen.
- Kein Gate darf durch Ausnahmen, uebersprungene Jobs oder geaenderte Schwellenwerte gruen werden.

Jede Anforderung gilt erst als erfuellt, wenn ein Nachweis nach der
Qualitaetsleiter vorliegt. Stand 17.11.2026 ist keine dieser Anforderungen
domaenenuebergreifend nachgewiesen.
