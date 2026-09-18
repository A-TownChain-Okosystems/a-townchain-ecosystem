// A-TownChain Ecosystem — canonical monetary-policy integration boundary
// Consensus authority: components/atc-algorithm/src/economics.rs
// Do not define a competing issuance schedule in downstream components.

pub const MAX_SUPPLY_ATC: u64 = 360_000_000;
pub const TARGET_BLOCK_TIME_SECS: u64 = 360;
pub const HALVING_INTERVAL_BLOCKS: u64 = 360_000;
pub const HALVING_COUNT: u32 = 36;
pub const INITIAL_SUBSIDY_ATC: u64 = 500;
pub const FINAL_EMISSION_BLOCK: u64 = 12_959_999;
pub const POST_HALVING_BLOCK: u64 = 12_960_000;

pub const fn halving_epoch(height: u64) -> u32 {
    let epoch = height / HALVING_INTERVAL_BLOCKS;
    if epoch >= HALVING_COUNT as u64 { HALVING_COUNT } else { epoch as u32 }
}

pub const fn raw_subsidy_atc(height: u64) -> u128 {
    let epoch = halving_epoch(height);
    if epoch >= HALVING_COUNT { return 0; }
    (INITIAL_SUBSIDY_ATC as u128) * 1_000_000_000_000_000_000u128 >> epoch
}