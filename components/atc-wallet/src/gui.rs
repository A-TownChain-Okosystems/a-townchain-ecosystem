//! UI-independent wallet view model.
//! The GUI must consume this model; cryptographic key material is never exposed.

use crate::{balance::Balance, history::History};

#[derive(Debug)]
pub struct WalletView {
    pub address: String,
    pub balance: Balance,
    pub history_count: usize,
}

pub fn snapshot(address: impl Into<String>, balance: Balance, history: &History) -> WalletView {
    WalletView {
        address: address.into(),
        balance,
        history_count: history.entries().len(),
    }
}
