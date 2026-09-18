//! Canonical A-TownChain L1 monetary constants, emission schedule and supply invariants.

pub const NATIVE_COIN_SYMBOL: &str = "ATC";
pub const EMISSION_MONTHS: u64 = 360;
pub const HALVING_INTERVAL_MONTHS: u64 = 60;
pub const HALVING_COUNT: u64 = 5;
pub const EMISSION_FIXED_DENOMINATOR: u64 = 672;
pub const MAX_ATC_SUPPLY: u64 = 360_000_000;
pub const INITIAL_MONTHLY_RATE_NUMERATOR: u64 = 2_048_000_000;
pub const DISCRETIONARY_MINT_ALLOWED: bool = false;

pub const fn halving_epoch(month: u64) -> Option<u64> {
    if month == 0 || month > EMISSION_MONTHS { None } else { Some((month - 1) / HALVING_INTERVAL_MONTHS) }
}

pub const fn emission_rate_numerator(month: u64) -> Option<u64> {
    match halving_epoch(month) {
        Some(epoch) if epoch <= HALVING_COUNT => Some(INITIAL_MONTHLY_RATE_NUMERATOR >> epoch),
        _ => None,
    }
}

pub const fn emission_rate_floor(month: u64) -> Option<u64> {
    match emission_rate_numerator(month) {
        Some(n) => Some(n / EMISSION_FIXED_DENOMINATOR),
        None => None,
    }
}

pub const fn scheduled_supply() -> u64 { MAX_ATC_SUPPLY }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn canonical_supply_cap_is_360_million_atc() { assert_eq!(MAX_ATC_SUPPLY, 360_000_000); }
    #[test] fn native_coin_is_atc() { assert_eq!(NATIVE_COIN_SYMBOL, "ATC"); }
    #[test] fn halving_epochs_are_deterministic() {
        assert_eq!(halving_epoch(1), Some(0)); assert_eq!(halving_epoch(60), Some(0));
        assert_eq!(halving_epoch(61), Some(1)); assert_eq!(halving_epoch(121), Some(2));
        assert_eq!(halving_epoch(181), Some(3)); assert_eq!(halving_epoch(241), Some(4));
        assert_eq!(halving_epoch(301), Some(5)); assert_eq!(halving_epoch(360), Some(5));
        assert_eq!(halving_epoch(361), None);
    }
    #[test] fn rates_halve_at_each_boundary() {
        assert_eq!(emission_rate_numerator(60), Some(2_048_000_000));
        assert_eq!(emission_rate_numerator(61), Some(1_024_000_000));
        assert_eq!(emission_rate_numerator(121), Some(512_000_000));
        assert_eq!(emission_rate_numerator(181), Some(256_000_000));
        assert_eq!(emission_rate_numerator(241), Some(128_000_000));
        assert_eq!(emission_rate_numerator(301), Some(64_000_000));
    }
    #[test] fn fixed_point_schedule_sums_to_exact_cap() {
        let mut total = 0u64; let mut remainder = 0u64;
        for month in 1..=EMISSION_MONTHS {
            let numerator = emission_rate_numerator(month).unwrap();
            let value = numerator + remainder; total += value / EMISSION_FIXED_DENOMINATOR;
            remainder = value % EMISSION_FIXED_DENOMINATOR;
        }
        assert_eq!(total, MAX_ATC_SUPPLY); assert_eq!(remainder, 0);
    }
}