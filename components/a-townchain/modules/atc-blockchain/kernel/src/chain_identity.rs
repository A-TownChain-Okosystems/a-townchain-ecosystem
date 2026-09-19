// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical numeric identity for the A-TownChain L1 development network.
//!
//! ATC-STD-600 / AD-004 freezes the numeric Chain-ID at 658467. Keeping the
//! value in the canonical blockchain crate prevents wallet/node drift.

/// Canonical numeric Chain-ID used by the current A-TownChain network.
pub const NUMERIC_CHAIN_ID: u64 = 658_467;
