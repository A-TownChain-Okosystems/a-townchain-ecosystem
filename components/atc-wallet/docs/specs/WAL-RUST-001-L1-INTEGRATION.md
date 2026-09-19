---
spec_id: WAL-RUST-001
title: "Rust Wallet / A-TownChain L1 Integration"
version: 0.2.0
status: IMPLEMENTATION
---

# WAL-RUST-001 — Rust Wallet / A-TownChain L1 Integration

## Implemented

- OS-CSPRNG Ed25519 wallet keys.
- Private key encapsulation and zeroization of generated seed material.
- WAL-ADDR-001 address primitives.
- ATC-TX-DOMAIN-V2 signing preimage.
- Ed25519 transaction signing and verification.
- Checked balance accounting.
- Pending/confirmed/rejected history model.
- UI-independent view model that does not expose secret key material.
- Typed NodeClient boundary for transaction submission and balance queries.

## Protocol boundaries

The wallet does not use Ethereum RLP, EIP-155, Keccak addresses or an
Ethereum RPC abstraction.

The executable L1 kernel currently verifies Ed25519. WAL-SIGN-001 remains
in conflict if it is still defined as secp256k1/RFC6979/Low-S. This must be
resolved as one governed protocol change across kernel, wallet, SDK and
conformance vectors.

Network/version bytes remain configuration-driven until the corresponding
network specification freezes them.

## Evidence gates

1. cargo fmt --all -- --check
2. cargo check --all-targets
3. cargo test --all-targets
4. Cross-component signing vector against the L1 kernel
5. Wallet -> node transaction submission
6. Multi-process transaction -> block -> state E2E
7. Restart/recovery balance and history verification

The wallet is not called production-ready until these gates have actual CI
evidence.
