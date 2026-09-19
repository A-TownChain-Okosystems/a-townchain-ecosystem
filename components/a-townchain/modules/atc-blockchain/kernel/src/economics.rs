//! Canonical A-TownChain L1 monetary and timing invariants.

/// Native A-TownChain coin symbol.
pub const NATIVE_COIN_SYMBOL: &str = "ATC";

/// Maximum lifetime supply, in whole ATC units.
pub const MAX_ATC_SUPPLY: u64 = 360_000_000;

/// Consensus block interval.
pub const BLOCK_TIME_SECONDS: u64 = 360;

/// Number of blocks between halving events.
pub const HALVING_INTERVAL_BLOCKS: u64 = 360_000;

/// Number of halving events in the configured emission schedule.
pub const HALVING_EVENTS: u64 = 36;

/// Post-genesis minting is only valid through the canonical block-reward path.
pub const POST_GENESIS_MINT_ALLOWED: bool = true;

/// Initial block reward in whole ATC units.
///
/// NOTE: integer ATC accounting cannot distribute the mathematical 360,000,000 ATC
/// exactly over 36 integer halvings without a terminal supply-cap rule. The reward
/// path therefore MUST cap issuance at MAX_ATC_SUPPLY.
pub const INITIAL_BLOCK_REWARD_ATC: u64 = 500;

/// Return the integer block reward for a given height.
///
/// Height zero is genesis and receives no reward. After HALVING_EVENTS the reward
/// is zero. The state transition must additionally cap total supply at
/// MAX_ATC_SUPPLY.
pub const fn block_reward_at_height(height: u64) -> u64 {
    if height == 0 {
        return 0;
    }
    let era = (height - 1) / HALVING_INTERVAL_BLOCKS;
    if era >= HALVING_EVENTS {
        return 0;
    }
    INITIAL_BLOCK_REWARD_ATC >> era
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_supply_and_timing() {
        assert_eq!(MAX_ATC_SUPPLY, 360_000_000);
        assert_eq!(BLOCK_TIME_SECONDS, 360);
        assert_eq!(HALVING_INTERVAL_BLOCKS, 360_000);
        assert_eq!(HALVING_EVENTS, 36);
    }

    #[test]
    fn reward_halves_at_boundaries() {
        assert_eq!(block_reward_at_height(0), 0);
        assert_eq!(block_reward_at_height(1), 500);
        assert_eq!(block_reward_at_height(360_000), 500);
        assert_eq!(block_reward_at_height(360_001), 250);
        assert_eq!(block_reward_at_height(720_000), 250);
        assert_eq!(block_reward_at_height(720_001), 125);
        assert_eq!(block_reward_at_height(360_000 * 36 + 1), 0);
    }
}
