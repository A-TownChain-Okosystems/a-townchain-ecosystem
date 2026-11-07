<!-- Diesen Text als PR-Beschreibung verwenden (Branch audit/security-pentest-p0-2026-11-05 -> main) -->

# security(p0): Pentest-Report Transaction Authority & Kryptografie (05.11.2026)

## Zweck

P0-Penetrationstest **Transaction Authority & Kryptografie** (Audit vom 05.11.2026) mit reproduzierbaren Befunden, maschinenlesbarem Register und ausführbaren Reproduktionen.

## Inhalt

- `docs/audit/security/SEC-PENTEST-P0-2026-11-05.md` — Bericht: Scope, Methodik, Evidenzbasis (exakte Commit-SHAs), 9 Findings mit Remediation- und Re-Test-Kriterien
- `docs/audit/security/registry/p0-findings.yaml` — Findings-Register (maschinenlesbar, 9 Einträge)
- `docs/audit/security/repro/*.rs` — drei ausführbare Reproduktionen (Wallet, Oracle, Validator)

## Ergebnis in einem Satz

Die Transaktionsautorität ist in **keinem** Live-Konsenspfad kryptografisch verankert: die kanonische secp256k1-Verifikation existiert, ist aber nicht verdrahtet; die live genutzten Autoritätspfade nutzen entweder Ed25519 (kai-os) oder nicht-kryptografische Konstruktionen (Validator-Hash mit Klartext-Passwort, Mock-Verifier mit VALID-Präfix).

## Severity-Übersicht

| ID | Severity | Kurzfassung |
|---|---|---|
| SEC-P0-001 | kritisch | secp256k1-Tx-Verifikation nirgends im Node verdrahtet, kein Mempool, Blöcke ohne Signaturen |
| SEC-P0-002 | kritisch | Mock-Verifier akzeptiert jede Signatur mit `VALID`-Präfix |
| SEC-P0-003 | kritisch | Validator-„Signatur" = SHA-256 aus Klartext-Passwort; verify verlangt Signierer-Passwort |
| SEC-P0-004 | kritisch | keine DID↔Pubkey-Bindung: beliebiger Key beansprucht jede DID (reproduziert) |
| SEC-P0-005 | hoch | keine Adress-/DID-Ableitung im Live-Wallet |
| SEC-P0-006 | hoch | 33-Byte-secp256k1 vs 32-Byte-Ed25519, typgebundene Inkompatibilität |
| SEC-P0-007 | hoch | Blockhash = 64-Bit-Truncation, Kettenintegrität ohne Signatur |
| SEC-P0-008 | hoch | Domain-/Replay-/Nonce-Lücken in Validator- und kai-os-Handshake |
| SEC-P0-009 | mittel | Oracle-Tests zementieren den VALID-Bypass als „gültig" |

## Normative Richtung

`atc-standards/registry/findings.yaml:529` (SPEC-DRAFT **WAL-SIGN-001**): kanonisch = secp256k1/RFC 6979/Low-S für TX-Signaturen, **Ed25519 ausschließlich P2P/DID-Layer**. Die Spec ist referenziert, aber noch nicht als Dokument materialisiert; Umsetzung steht aus.

## Reproduktion

Die drei Testdateien sind lauffähig und wurden am 05.11.2026 grün ausgeführt (Beweise: `NACHWEIS SEC-P0-…`-Ausgaben). Kein `VERIFIED` wird beansprucht; jeder Befund ist am exakten SHA belegt.

Agent-ID: aurora-base44-superagent-6a1d37209d7f46ac065d39c5
Task-ID: ECOSYSTEM-SEC-PENTEST-P0
