//! Durable append-only block storage with crash recovery.
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::{
    mempool::{Account, Transaction, TxType},
    Block,
};

const MAGIC: &[u8] = b"ATCB1";

fn put(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u32).to_be_bytes());
    out.extend_from_slice(b);
}

fn get<'a>(b: &'a [u8], p: &mut usize) -> Result<&'a [u8], String> {
    if *p + 4 > b.len() {
        return Err("truncated length".into());
    }
    let n = u32::from_be_bytes(b[*p..*p + 4].try_into().unwrap()) as usize;
    *p += 4;
    if *p + n > b.len() {
        return Err("truncated field".into());
    }
    let r = &b[*p..*p + n];
    *p += n;
    Ok(r)
}

fn fixed<const N: usize>(b: &[u8], p: &mut usize) -> Result<[u8; N], String> {
    if *p + N > b.len() {
        return Err("truncated fixed field".into());
    }
    let r = b[*p..*p + N].try_into().unwrap();
    *p += N;
    Ok(r)
}

fn tx_encode(t: &Transaction, o: &mut Vec<u8>) {
    o.extend_from_slice(&t.chain_id.to_be_bytes());
    o.push(t.tx_type as u8);
    put(o, t.sender_did.as_bytes());
    match &t.recipient_did {
        Some(v) => {
            o.push(1);
            put(o, v.as_bytes());
        }
        None => o.push(0),
    }
    o.extend_from_slice(&t.amount.to_be_bytes());
    o.extend_from_slice(&t.gas_price.to_be_bytes());
    o.extend_from_slice(&t.gas_limit.to_be_bytes());
    o.extend_from_slice(&t.nonce.to_be_bytes());
    o.extend_from_slice(&t.timestamp.to_be_bytes());
    put(o, &t.payload);
    o.extend_from_slice(&t.signature);
    o.extend_from_slice(&t.public_key);
    o.extend_from_slice(&t.poh_hash);
}

fn tx_decode(b: &[u8], p: &mut usize) -> Result<Transaction, String> {
    let chain_id = u64::from_be_bytes(fixed::<8>(b, p)?);
    let ty = {
        let v = fixed::<1>(b, p)?[0];
        match v {
            0 => TxType::Transfer,
            1 => TxType::Stake,
            2 => TxType::Unstake,
            3 => TxType::Contract,
            _ => return Err("invalid tx type".into()),
        }
    };
    let sender = String::from_utf8(get(b, p)?.to_vec()).map_err(|_| "invalid sender")?;
    let has = fixed::<1>(b, p)?[0];
    let recipient = if has == 1 {
        Some(String::from_utf8(get(b, p)?.to_vec()).map_err(|_| "invalid recipient")?)
    } else if has == 0 {
        None
    } else {
        return Err("invalid recipient flag".into());
    };
    let amount = u64::from_be_bytes(fixed::<8>(b, p)?);
    let gas_price = u64::from_be_bytes(fixed::<8>(b, p)?);
    let gas_limit = u64::from_be_bytes(fixed::<8>(b, p)?);
    let nonce = u64::from_be_bytes(fixed::<8>(b, p)?);
    let timestamp = u64::from_be_bytes(fixed::<8>(b, p)?);
    let payload = get(b, p)?.to_vec();
    let signature = fixed::<64>(b, p)?;
    let public_key = fixed::<32>(b, p)?;
    let poh = fixed::<32>(b, p)?;
    Ok(Transaction::new_with_chain_id(
        chain_id, ty, sender, recipient, amount, gas_price, gas_limit, nonce, timestamp, payload,
        signature, public_key, poh,
    ))
}

fn block_encode(b: &Block) -> Vec<u8> {
    let mut o = Vec::from(MAGIC);
    o.extend_from_slice(&b.height.to_be_bytes());
    o.extend_from_slice(&b.parent_hash);
    put(&mut o, b.proposer.as_bytes());
    o.extend_from_slice(&b.timestamp.to_be_bytes());
    o.extend_from_slice(&(b.transactions.len() as u32).to_be_bytes());
    for t in &b.transactions {
        let mut x = Vec::new();
        tx_encode(t, &mut x);
        put(&mut o, &x);
    }
    o.extend_from_slice(&b.tx_root);
    o.extend_from_slice(&b.state_root);
    o.extend_from_slice(&b.receipt_root);
    o.extend_from_slice(&b.signature);
    o.extend_from_slice(&b.id);
    o
}

fn block_decode(b: &[u8]) -> Result<Block, String> {
    if !b.starts_with(MAGIC) {
        return Err("invalid storage magic".into());
    }
    let mut p = MAGIC.len();
    let h = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    let parent = fixed::<32>(b, &mut p)?;
    let proposer = String::from_utf8(get(b, &mut p)?.to_vec()).map_err(|_| "invalid proposer")?;
    let ts = u64::from_be_bytes(fixed::<8>(b, &mut p)?);
    if p + 4 > b.len() {
        return Err("truncated tx count".into());
    }
    let n = u32::from_be_bytes(b[p..p + 4].try_into().unwrap()) as usize;
    p += 4;
    let mut txs = Vec::with_capacity(n);
    for _ in 0..n {
        let raw = get(b, &mut p)?;
        let mut q = 0usize;
        txs.push(tx_decode(raw, &mut q)?);
        if q != raw.len() {
            return Err("trailing tx bytes".into());
        }
    }
    let state = fixed::<32>(b, &mut p)?;
    let receipt = fixed::<32>(b, &mut p)?;
    let sig = fixed::<64>(b, &mut p)?;
    let stored_id = fixed::<32>(b, &mut p)?;
    if p != b.len() {
        return Err("trailing storage bytes".into());
    }
    let block = Block::new(h, parent, proposer, ts, txs, state, receipt, sig);
    if block.id != stored_id {
        return Err("block commitment mismatch".into());
    }
    Ok(block)
}

pub struct ChainStorage {
    blocks: RwLock<BTreeMap<u64, Block>>,
    state_roots: RwLock<BTreeMap<u64, [u8; 32]>>,
    journal: Option<PathBuf>,
    state_journal: Option<PathBuf>,
}

impl ChainStorage {
    pub fn new() -> Self {
        Self {
            blocks: RwLock::new(BTreeMap::new()),
            state_roots: RwLock::new(BTreeMap::new()),
            journal: None,
            state_journal: None,
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let p = path.as_ref().to_path_buf();
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let state_p = p.with_extension("state");
        let s = Self {
            blocks: RwLock::new(BTreeMap::new()),
            state_roots: RwLock::new(BTreeMap::new()),
            journal: Some(p),
            state_journal: Some(state_p),
        };
        s.recover()?;
        Ok(s)
    }

    fn recover(&self) -> Result<(), String> {
        let Some(p) = &self.journal else {
            return Ok(());
        };
        if !p.exists() {
            return Ok(());
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        for (line_no, line) in BufReader::new(f).lines().enumerate() {
            let l = line.map_err(|e| e.to_string())?;
            if l.trim().is_empty() {
                continue;
            }
            let bytes = hex::decode(l.trim())
                .map_err(|e| format!("journal line {}: invalid hex: {e}", line_no + 1))?;
            let block =
                block_decode(&bytes).map_err(|e| format!("journal line {}: {e}", line_no + 1))?;
            self.state_roots
                .write()
                .unwrap()
                .insert(block.height, block.state_root);
            self.blocks.write().unwrap().insert(block.height, block);
        }
        Ok(())
    }

    pub fn commit(&self, b: Block) -> Result<(), String> {
        if let Some(p) = &self.journal {
            let line = format!("{}\n", hex::encode(block_encode(&b)));
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)
                .map_err(|e| e.to_string())?;
            f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            f.sync_data().map_err(|e| e.to_string())?;
        }
        self.state_roots
            .write()
            .unwrap()
            .insert(b.height, b.state_root);
        self.blocks.write().unwrap().insert(b.height, b);
        Ok(())
    }

    pub fn commit_state(
        &self,
        height: u64,
        state: &BTreeMap<String, Account>,
    ) -> Result<(), String> {
        self.commit_state_with_dao(height, state, &[])
    }

    pub fn commit_state_with_dao(
        &self,
        height: u64,
        state: &BTreeMap<String, Account>,
        dao: &[u8],
    ) -> Result<(), String> {
        if let Some(p) = &self.state_journal {
            let mut o = Vec::new();
            o.extend_from_slice(&height.to_be_bytes());
            o.extend_from_slice(&(state.len() as u32).to_be_bytes());
            for (k, v) in state {
                put(&mut o, k.as_bytes());
                o.extend_from_slice(&v.balance.to_be_bytes());
                o.extend_from_slice(&v.staked.to_be_bytes());
                o.extend_from_slice(&v.nonce.to_be_bytes());
            }
            put(&mut o, dao);
            let line = format!("{}\n", hex::encode(o));
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)
                .map_err(|e| e.to_string())?;
            f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            f.sync_data().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn recover_state(&self) -> Result<Option<BTreeMap<String, Account>>, String> {
        Ok(self.recover_state_with_dao()?.map(|x| x.0))
    }

    pub fn recover_state_with_dao(
        &self,
    ) -> Result<Option<(BTreeMap<String, Account>, Vec<u8>)>, String> {
        let Some(p) = &self.state_journal else {
            return Ok(None);
        };
        if !p.exists() {
            return Ok(None);
        }
        let f = File::open(p).map_err(|e| e.to_string())?;
        let mut latest = None;
        for line in BufReader::new(f).lines() {
            let raw = line.map_err(|e| e.to_string())?;
            if raw.trim().is_empty() {
                continue;
            }
            let b = hex::decode(raw.trim()).map_err(|e| e.to_string())?;
            let mut q = 0usize;
            let h = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
            let n = u32::from_be_bytes(fixed::<4>(&b, &mut q)?) as usize;
            let mut map = BTreeMap::new();
            for _ in 0..n {
                let k = String::from_utf8(get(&b, &mut q)?.to_vec())
                    .map_err(|_| "invalid state key")?;
                let balance = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                let staked = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                let nonce = u64::from_be_bytes(fixed::<8>(&b, &mut q)?);
                map.insert(
                    k,
                    Account {
                        balance,
                        staked,
                        nonce,
                    },
                );
            }
            let dao = if q < b.len() {
                get(&b, &mut q)?.to_vec()
            } else {
                Vec::new()
            };
            if q != b.len() {
                return Err("trailing state bytes".into());
            }
            latest = Some((h, (map, dao)));
        }
        Ok(latest.map(|(_, m)| m))
    }

    pub fn block(&self, h: u64) -> Option<Block> {
        self.blocks.read().unwrap().get(&h).cloned()
    }

    pub fn state_root(&self, h: u64) -> Option<[u8; 32]> {
        self.state_roots.read().unwrap().get(&h).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let s = ChainStorage::new();
        let b = Block::new(
            0,
            [0; 32],
            "v".into(),
            1,
            Vec::new(),
            [1; 32],
            [2; 32],
            [0; 64],
        );
        s.commit(b.clone()).unwrap();
        assert_eq!(s.block(0), Some(b));
    }
}
