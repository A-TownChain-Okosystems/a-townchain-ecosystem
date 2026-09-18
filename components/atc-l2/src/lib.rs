//! Native A-TownChain Layer-2 deterministic execution and settlement primitives.

use std::collections::BTreeMap;
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];
pub type Address = [u8; 32];
pub type TokenId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub id: TokenId,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub issuer: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    CreateToken { id: TokenId, symbol: String, decimals: u8, issuer: Address },
    Mint { token: TokenId, to: Address, amount: u128 },
    Burn { token: TokenId, from: Address, amount: u128 },
    Transfer { token: TokenId, from: Address, to: Address, amount: u128 },
    Deposit { token: TokenId, to: Address, amount: u128, l1_reference: Hash },
    Withdraw { token: TokenId, from: Address, amount: u128, l1_reference: Hash },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError {
    TokenExists, UnknownToken, InvalidAmount, InsufficientBalance,
    Overflow, InvalidDepositReference, Replay, WithdrawalAlreadyFinalized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L2State {
    pub chain_id: u64,
    pub height: u64,
    pub tokens: BTreeMap<TokenId, Token>,
    pub balances: BTreeMap<(TokenId, Address), u128>,
    pub processed_l1_references: BTreeMap<Hash, ()>,
    pub finalized_withdrawals: BTreeMap<Hash, ()>,
}

impl L2State {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id, height: 0, tokens: BTreeMap::new(), balances: BTreeMap::new(),
            processed_l1_references: BTreeMap::new(), finalized_withdrawals: BTreeMap::new(),
        }
    }

    pub fn balance_of(&self, token: TokenId, owner: Address) -> u128 {
        self.balances.get(&(token, owner)).copied().unwrap_or(0)
    }

    pub fn execute(&mut self, tx: &Transaction) -> Result<(), ExecutionError> {
        match tx {
            Transaction::CreateToken { id, symbol, decimals, issuer } => {
                if self.tokens.contains_key(id) { return Err(ExecutionError::TokenExists); }
                if symbol.is_empty() || symbol.len() > 32 || *decimals > 38 {
                    return Err(ExecutionError::InvalidAmount);
                }
                self.tokens.insert(*id, Token { id: *id, symbol: symbol.clone(), decimals: *decimals, total_supply: 0, issuer: *issuer });
            }
            Transaction::Mint { token, to, amount } => {
                if *amount == 0 { return Err(ExecutionError::InvalidAmount); }
                let current = self.balance_of(*token, *to);
                let t = self.tokens.get_mut(token).ok_or(ExecutionError::UnknownToken)?;
                t.total_supply = t.total_supply.checked_add(*amount).ok_or(ExecutionError::Overflow)?;
                self.balances.insert((*token, *to), current.checked_add(*amount).ok_or(ExecutionError::Overflow)?);
            }
            Transaction::Burn { token, from, amount } => {
                if *amount == 0 { return Err(ExecutionError::InvalidAmount); }
                let current = self.balance_of(*token, *from);
                if current < *amount { return Err(ExecutionError::InsufficientBalance); }
                let t = self.tokens.get_mut(token).ok_or(ExecutionError::UnknownToken)?;
                t.total_supply = t.total_supply.checked_sub(*amount).ok_or(ExecutionError::Overflow)?;
                self.balances.insert((*token, *from), current - *amount);
            }
            Transaction::Transfer { token, from, to, amount } => {
                if *amount == 0 { return Err(ExecutionError::InvalidAmount); }
                if !self.tokens.contains_key(token) { return Err(ExecutionError::UnknownToken); }
                let from_balance = self.balance_of(*token, *from);
                if from_balance < *amount { return Err(ExecutionError::InsufficientBalance); }
                let to_balance = self.balance_of(*token, *to);
                self.balances.insert((*token, *from), from_balance - *amount);
                self.balances.insert((*token, *to), to_balance.checked_add(*amount).ok_or(ExecutionError::Overflow)?);
            }
            Transaction::Deposit { token, to, amount, l1_reference } => {
                if *amount == 0 { return Err(ExecutionError::InvalidAmount); }
                if l1_reference.iter().all(|b| *b == 0) { return Err(ExecutionError::InvalidDepositReference); }
                if self.processed_l1_references.contains_key(l1_reference) { return Err(ExecutionError::Replay); }
                let current = self.balance_of(*token, *to);
                let t = self.tokens.get_mut(token).ok_or(ExecutionError::UnknownToken)?;
                t.total_supply = t.total_supply.checked_add(*amount).ok_or(ExecutionError::Overflow)?;
                self.balances.insert((*token, *to), current.checked_add(*amount).ok_or(ExecutionError::Overflow)?);
                self.processed_l1_references.insert(*l1_reference, ());
            }
            Transaction::Withdraw { token, from, amount, l1_reference } => {
                if *amount == 0 { return Err(ExecutionError::InvalidAmount); }
                if l1_reference.iter().all(|b| *b == 0) { return Err(ExecutionError::InvalidDepositReference); }
                if self.finalized_withdrawals.contains_key(l1_reference) { return Err(ExecutionError::WithdrawalAlreadyFinalized); }
                let current = self.balance_of(*token, *from);
                if current < *amount { return Err(ExecutionError::InsufficientBalance); }
                let t = self.tokens.get_mut(token).ok_or(ExecutionError::UnknownToken)?;
                t.total_supply = t.total_supply.checked_sub(*amount).ok_or(ExecutionError::Overflow)?;
                self.balances.insert((*token, *from), current - *amount);
                self.finalized_withdrawals.insert(*l1_reference, ());
            }
        }
        Ok(())
    }

    pub fn state_root(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(self.chain_id.to_be_bytes());
        h.update(self.height.to_be_bytes());
        for (id, token) in &self.tokens {
            h.update(id.to_be_bytes());
            h.update((token.symbol.len() as u64).to_be_bytes());
            h.update(token.symbol.as_bytes());
            h.update([token.decimals]);
            h.update(token.total_supply.to_be_bytes());
            h.update(token.issuer);
        }
        for ((token, owner), balance) in &self.balances {
            h.update(token.to_be_bytes()); h.update(owner); h.update(balance.to_be_bytes());
        }
        for (reference, _) in &self.processed_l1_references { h.update(reference); }
        for (reference, _) in &self.finalized_withdrawals { h.update(reference); }
        h.finalize().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt { pub index: u32, pub success: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L2Batch {
    pub number: u64,
    pub parent_state_root: Hash,
    pub state_root: Hash,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
    pub batch_hash: Hash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1SettlementCommitment {
    pub l2_chain_id: u64,
    pub batch_number: u64,
    pub state_root: Hash,
    pub batch_hash: Hash,
}

#[derive(Debug, Default)]
pub struct L2Executor;

impl L2Executor {
    pub fn execute_batch(state: &mut L2State, transactions: Vec<Transaction>) -> Result<L2Batch, ExecutionError> {
        let parent_state_root = state.state_root();
        let mut receipts = Vec::with_capacity(transactions.len());
        for (index, tx) in transactions.iter().enumerate() {
            state.execute(tx)?;
            receipts.push(Receipt { index: index as u32, success: true });
        }
        state.height = state.height.checked_add(1).ok_or(ExecutionError::Overflow)?;
        let state_root = state.state_root();
        let batch_hash = hash_batch(state.chain_id, state.height, parent_state_root, state_root, &transactions);
        Ok(L2Batch { number: state.height, parent_state_root, state_root, transactions, receipts, batch_hash })
    }

    pub fn commitment(l2_chain_id: u64, batch: &L2Batch) -> L1SettlementCommitment {
        L1SettlementCommitment { l2_chain_id, batch_number: batch.number, state_root: batch.state_root, batch_hash: batch.batch_hash }
    }
}

fn hash_batch(chain_id: u64, number: u64, parent: Hash, state_root: Hash, txs: &[Transaction]) -> Hash {
    let mut h = Sha256::new();
    h.update(chain_id.to_be_bytes()); h.update(number.to_be_bytes()); h.update(parent); h.update(state_root);
    for tx in txs { hash_tx(&mut h, tx); }
    h.finalize().into()
}

fn hash_tx(h: &mut Sha256, tx: &Transaction) {
    match tx {
        Transaction::CreateToken { id, symbol, decimals, issuer } => { h.update([0]); h.update(id.to_be_bytes()); h.update((symbol.len() as u64).to_be_bytes()); h.update(symbol.as_bytes()); h.update([*decimals]); h.update(issuer); }
        Transaction::Mint { token, to, amount } => { h.update([1]); h.update(token.to_be_bytes()); h.update(to); h.update(amount.to_be_bytes()); }
        Transaction::Burn { token, from, amount } => { h.update([2]); h.update(token.to_be_bytes()); h.update(from); h.update(amount.to_be_bytes()); }
        Transaction::Transfer { token, from, to, amount } => { h.update([3]); h.update(token.to_be_bytes()); h.update(from); h.update(to); h.update(amount.to_be_bytes()); }
        Transaction::Deposit { token, to, amount, l1_reference } => { h.update([4]); h.update(token.to_be_bytes()); h.update(to); h.update(amount.to_be_bytes()); h.update(l1_reference); }
        Transaction::Withdraw { token, from, amount, l1_reference } => { h.update([5]); h.update(token.to_be_bytes()); h.update(from); h.update(amount.to_be_bytes()); h.update(l1_reference); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn a(v: u8) -> Address { [v; 32] }
    fn r(v: u8) -> Hash { [v; 32] }

    #[test]
    fn token_transfer_is_deterministic() {
        let mut s = L2State::new(658467);
        s.execute(&Transaction::CreateToken { id: 1, symbol: "ATC2".into(), decimals: 18, issuer: a(1) }).unwrap();
        s.execute(&Transaction::Mint { token: 1, to: a(1), amount: 100 }).unwrap();
        s.execute(&Transaction::Transfer { token: 1, from: a(1), to: a(2), amount: 40 }).unwrap();
        assert_eq!(s.balance_of(1, a(1)), 60);
        assert_eq!(s.balance_of(1, a(2)), 40);
    }

    #[test]
    fn deposit_reference_is_replay_protected() {
        let mut s = L2State::new(658467);
        s.execute(&Transaction::CreateToken { id: 1, symbol: "ATC2".into(), decimals: 18, issuer: a(1) }).unwrap();
        assert!(s.execute(&Transaction::Deposit { token: 1, to: a(2), amount: 50, l1_reference: r(7) }).is_ok());
        assert_eq!(s.execute(&Transaction::Deposit { token: 1, to: a(2), amount: 50, l1_reference: r(7) }), Err(ExecutionError::Replay));
    }

    #[test]
    fn batch_and_commitment_are_reproducible() {
        let txs = vec![
            Transaction::CreateToken { id: 1, symbol: "ATC2".into(), decimals: 18, issuer: a(1) },
            Transaction::Mint { token: 1, to: a(1), amount: 100 },
        ];
        let mut a_state = L2State::new(658467);
        let mut b_state = L2State::new(658467);
        let a_batch = L2Executor::execute_batch(&mut a_state, txs.clone()).unwrap();
        let b_batch = L2Executor::execute_batch(&mut b_state, txs).unwrap();
        assert_eq!(a_batch.state_root, b_batch.state_root);
        assert_eq!(a_batch.batch_hash, b_batch.batch_hash);
        assert_eq!(a_batch.state_root, a_state.state_root());
    }
}
