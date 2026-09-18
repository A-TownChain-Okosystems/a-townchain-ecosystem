#!/usr/bin/env python3
"""Generiert registry/protocol-registry.yaml — die ATC-Protocol-Registry (ATC-STD-PROTOCOL-001 §20/§21).
SSOT der ATC-PROTO-*-Familien; Regenerierung nur via SCR (REQ-PROTO-020)."""

import yaml

# (Domain, Name, Priorität, Status, Layer, Notiz) — 26 Familien gem. Owner-Matrix
PROTOCOLS = [
    (
        "P2P",
        "ATC Peer-to-Peer Protocol",
        "P1",
        "draft",
        "L1/L3",
        "v1.0.0 IMPLEMENTIERT (08.09. 01:35, K14-Upgrade p2p_secure.rs, SCR-0028): Envelope 9+1, 6-Phasen-Handshake, Replay-Schutz, Rate-Limiting, v0.9-Kompatibilitaetsmodus; Kernel 423/423 Tests gruen; Status draft bis Testnet-Verifikation/Activation (PROTOCOL-001 Par.19)",
    ),
    (
        "NODE",
        "ATC Node Communication",
        "P1",
        "planned",
        "L1",
        "Node-Discovery/Management auf P2P aufbauend",
    ),
    (
        "CONSENSUS",
        "ATC Consensus Protocol",
        "P1",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K16 (DAG+PoH+Validator+Voting+Finality)",
    ),
    ("BLOCK", "ATC Block Propagation", "P1", "draft", "L3", "Impl.-Spuren: a-townchain Blockchain"),
    (
        "TX",
        "ATC Transaction Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: a-townchain + ShivaCore Tx-Pipeline",
    ),
    (
        "MEMPOOL",
        "ATC Mempool Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: mempool.rs (Referenzimplementierung)",
    ),
    (
        "VALIDATOR",
        "ATC Validator Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: ShivaCore K16 Validator",
    ),
    ("STAKING", "ATC Staking Protocol", "P1", "planned", "L3", ""),
    (
        "MINING",
        "ATC Mining Protocol",
        "P1",
        "draft",
        "L3/L5",
        "Repo atc-mining vorhanden (L5-Dev-Plattform)",
    ),
    (
        "WALLET",
        "ATC Wallet Protocol",
        "P1",
        "draft",
        "L3/L5",
        "Repo atc-wallet vorhanden (L5-Dev-Plattform)",
    ),
    (
        "IDENTITY",
        "ATC Identity Protocol",
        "P1",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K6 (DID+RCT) + K6b (Ed25519)",
    ),
    (
        "REPUTATION",
        "ATC Reputation Protocol",
        "P2",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K15 (Reputation)",
    ),
    ("GOVERNANCE", "ATC Governance Protocol", "P2", "planned", "L3", ""),
    ("ORACLE", "ATC Oracle Protocol", "P1", "planned", "L5", "Repo atc-oracle vorhanden"),
    ("ZKP", "ATC Zero Knowledge Protocol", "P1", "planned", "L5", "Repo atc-zkp vorhanden"),
    ("BRIDGE", "ATC Cross-Chain Bridge", "P1", "planned", "L5", "Repo atc-interop vorhanden"),
    ("IBC", "ATC Interoperability (IBC)", "P1", "planned", "L5", "Repo atc-interop vorhanden"),
    ("DATA", "ATC Data Layer Protocol", "P2", "planned", "L3/L5", ""),
    ("STORAGE", "ATC Decentralized Storage", "P2", "planned", "L5", "Repo atc-storage vorhanden"),
    (
        "AI",
        "ATC AI Agent Protocol",
        "P2",
        "planned",
        "L2",
        "aurora-ai (L2); Kopplung Kernel-Event-Bridge (ATC-M-003)",
    ),
    (
        "AGENT",
        "ATC Agent-to-Agent Protocol",
        "P2",
        "planned",
        "L2",
        "AAS-Kopplung (ATC-AAS-001..025)",
    ),
    ("API", "ATC API Protocol", "P2", "planned", "L5", "Kopplung ATC-STD-202 (API-Konventionen)"),
    (
        "EVENT",
        "ATC Event Protocol",
        "P2",
        "planned",
        "L2/L4",
        "Kopplung Kernel-Event-Bridge (ATC-M-003)",
    ),
    ("AUDIT", "ATC Audit Protocol", "P2", "planned", "L3/L7", "Kopplung AUDIT-001/LogChain"),
    (
        "UPGRADE",
        "ATC Protocol Upgrade",
        "P1",
        "planned",
        "L0-L7",
        "Prozessnorm in ATC-STD-PROTOCOL-001 §19 verankert",
    ),
    (
        "KERNEL",
        "ATC Kernel Interface Protocol",
        "P2",
        "planned",
        "L1",
        "Syscall-Interface ATC-96 (ShivaCore K9)",
    ),
]
# Formale Spezifikationen (SCR-dokumentiert) — Override des Standard-Spezifikations-Texts
SPEC_OVERRIDES = {
    "P2P": "protocols/p2p/ATC-PROTO-P2P-001.md — Spezifikation v1.0.0 APPROVED (SCR-0027) und IMPLEMENTIERT in ShivaCore K14-Upgrade (SCR-0028, p2p_secure.rs, 29 Unit-Tests, Kernel 423/423): Envelope 9+1, 13 Message-Types, 6-Phasen-Handshake, Replay-Schutz, Rate-Limiting, v0.9-Kompatibilitaetsmodus; Status draft bis Testnet-Verifikation/Activation",
}

assert len(PROTOCOLS) == 26, f"Erwartet 26 Protokollfamilien, gefunden {len(PROTOCOLS)}"
STATUSES = {"planned", "draft", "active", "experimental", "deprecated"}
assert all(s in STATUSES for _, _, _, s, _, _ in PROTOCOLS)
assert all(p in {"P0", "P1", "P2"} for _, _, p, *_ in PROTOCOLS)

data = {
    "protocol-registry": {
        "standard": "ATC-STD-PROTOCOL-001",
        "version": "1.0.0",
        "generated": "2026-09-08",
        "note": (
            "ATC Protocol Registry gem. ATC-STD-PROTOCOL-001 §20/§21 — SSOT der ATC-PROTO-*-Familien. "
            "Eintrag = Autorisierung: Ein Protokoll OHNE Registry-Eintrag ist kein ATC-Protokoll. "
            "Status-Ehrlichkeitsregel (REQ-PROTO-021): planned = Baseline ohne Implementierung, "
            "draft = Teilimplementierung existiert (Impl.-Spuren), active/experimental nur mit "
            "verifizierter Implementierung, deprecated = aktiv abgelöst. Änderungen nur via SCR; "
            "Validator S-23 prüft Integrität je CI-Lauf."
        ),
        "domains": [d for d, *_ in PROTOCOLS],
        "protocols": [
            {
                "id": f"ATC-PROTO-{d}-001",
                "name": n,
                "domain": d,
                "version": "1.0.0",
                "status": s,
                "priority": p,
                "layer": lay,
                "specification": SPEC_OVERRIDES.get(
                    d,
                    "Zu spezifizieren gem. ATC-STD-PROTOCOL-001 (SCR erforderlich)"
                    if s == "planned"
                    else "Teilweise implementiert; formale Spezifikation gem. ATC-STD-PROTOCOL-001 nachzuziehen",
                ),
                "note": note,
            }
            for d, n, p, s, lay, note in PROTOCOLS
        ],
    }
}
with open("registry/protocol-registry.yaml", "w", encoding="utf-8") as fh:
    fh.write("# ATC Protocol Registry — ATC-STD-PROTOCOL-001 §20/§21 (SSOT)\n")
    fh.write("# Generiert von tools/protocol/gen_protocol_registry.py — Änderungen nur via SCR.\n")
    yaml.dump(data, fh, allow_unicode=True, sort_keys=False, width=200)

counts = {}
for _, _, _, s, *_ in PROTOCOLS:
    counts[s] = counts.get(s, 0) + 1
print(
    f"protocol-registry.yaml: {len(PROTOCOLS)} Protokollfamilien, Status: {counts}, "
    f"P0-Fundament = Dachstandard ATC-STD-PROTOCOL-001 selbst"
)
