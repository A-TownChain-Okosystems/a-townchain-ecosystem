// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Wallet key boundary.
//!
//! Key generation and signing remain owned by the canonical `atc-wallet` crate.
//! This module only re-exports that implementation for the legacy L1 wallet
//! integration path; it does not maintain a second key implementation.

pub use atc_wallet::WalletKey;
