//! Canonical A-TownChain L1 monetary constants and supply invariants.

/// Native A-TownChain coin symbol.
pub const NATIVE_COIN_SYMBOL: &str = "ATC";

/// Maximum lifetime supply of the native L1 coin, expressed in whole ATC units.
///
/// This is a consensus invariant: state transitions must never increase
/// total supply above this value.
pub const MAX_ATC_SUPPLY: u64 = 360_000_000;

/// No protocol inflation is permitted by the default L1 state machine.
///
/// New ATC can only enter state through an explicit genesis allocation.
pub const POST_GENESIS_MINT_ALLOWED: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_supply_cap_is_360_million_atc() {
        assert_eq!(MAX_ATC_SUPPLY, 360_000_000);
    }

    #[test]
    fn native_coin_is_atc() {
        assert_eq!(NATIVE_COIN_SYMBOL, "ATC");
    }
}
