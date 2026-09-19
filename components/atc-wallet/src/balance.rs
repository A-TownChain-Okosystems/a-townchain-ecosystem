//! Deterministic wallet balance model and provider boundary.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Balance {
    pub confirmed: u64,
    pub spendable: u64,
    pub locked: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    InsufficientFunds,
    Overflow,
}

impl Balance {
    pub fn available(&self) -> u64 { self.spendable.saturating_sub(self.locked) }

    pub fn can_spend(&self, amount: u64) -> bool {
        self.available() >= amount
    }

    pub fn debit(&mut self, amount: u64) -> Result<(), BalanceError> {
        if !self.can_spend(amount) {
            return Err(BalanceError::InsufficientFunds);
        }
        self.spendable -= amount;
        Ok(())
    }

    pub fn credit(&mut self, amount: u64) -> Result<(), BalanceError> {
        self.confirmed = self.confirmed.checked_add(amount).ok_or(BalanceError::Overflow)?;
        self.spendable = self.spendable.checked_add(amount).ok_or(BalanceError::Overflow)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debit_and_credit_are_checked() {
        let mut b = Balance { confirmed: 100, spendable: 100, locked: 10 };
        assert!(b.debit(90).is_ok());
        assert_eq!(b.spendable, 10);
        assert!(b.debit(1).is_ok());
        assert!(b.debit(1).is_err());
    }
}
