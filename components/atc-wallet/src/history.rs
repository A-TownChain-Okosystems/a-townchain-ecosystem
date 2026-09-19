//! Wallet transaction history with explicit confirmation state.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Pending,
    Confirmed { height: u64 },
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub tx_id: [u8; 32],
    pub nonce: u64,
    pub amount: u64,
    pub status: TxStatus,
}

#[derive(Debug, Default)]
pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn push(&mut self, entry: HistoryEntry) { self.entries.push(entry); }
    pub fn entries(&self) -> &[HistoryEntry] { &self.entries }

    pub fn confirm(&mut self, tx_id: &[u8; 32], height: u64) -> bool {
        if let Some(e) = self.entries.iter_mut().find(|e| &e.tx_id == tx_id) {
            e.status = TxStatus::Confirmed { height };
            true
        } else {
            false
        }
    }
}
