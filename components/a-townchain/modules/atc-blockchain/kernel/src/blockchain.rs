//! Canonical A-TownChain deterministic block pipeline.
pub mod chain_identity;
pub mod consensus;
pub mod crypto;
pub mod dao_state;
pub mod economics;
pub mod execution;
pub mod fork_choice;
pub mod genesis;
pub mod mempool;
pub mod network;
pub mod receipts;
pub mod rpc;
pub mod security;
pub mod storage;
use consensus::{ConsensusEngine, SlashingEvidence, Vote};
use crypto::{signing_bytes, Ed25519Verifier, SignatureVerifier};
use ed25519_dalek::{Signer, Verifier};
use execution::{AtcVmExecutor, VmExecutor};
use mempool::{MemoryPool, MempoolError, StateDb, Transaction};
use network::{NetworkMessage, PeerTransport};
use security::simple_hash;
use std::{
    collections::BTreeMap,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: [u8; 32],
    pub height: u64,
    pub parent_hash: [u8; 32],
    pub proposer: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub tx_root: [u8; 32],
    pub state_root: [u8; 32],
    pub receipt_root: [u8; 32],
    pub signature: [u8; 64],
}
impl Block {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        h: u64,
        parent: [u8; 32],
        proposer: String,
        t: u64,
        txs: Vec<Transaction>,
        state: [u8; 32],
        receipt: [u8; 32],
        sig: [u8; 64],
    ) -> Self {
        let tr = tx_root(&txs);
        let id = block_id(h, parent, &proposer, t, tr, state, receipt, sig);
        Self {
            id,
            height: h,
            parent_hash: parent,
            proposer,
            timestamp: t,
            transactions: txs,
            tx_root: tr,
            state_root: state,
            receipt_root: receipt,
            signature: sig,
        }
    }
}
fn block_signing_bytes(chain_id: u64, b: &Block) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"ATC-BLOCK-SIG-V1");
    out.extend_from_slice(&chain_id.to_be_bytes());
    out.extend_from_slice(&b.height.to_be_bytes());
    out.extend_from_slice(&b.parent_hash);
    out.extend_from_slice(&(b.proposer.len() as u32).to_be_bytes());
    out.extend_from_slice(b.proposer.as_bytes());
    out.extend_from_slice(&b.timestamp.to_be_bytes());
    out.extend_from_slice(&b.tx_root);
    out.extend_from_slice(&b.state_root);
    out.extend_from_slice(&b.receipt_root);
    out
}

fn tx_root(txs: &[Transaction]) -> [u8; 32] {
    let mut b = Vec::new();
    for x in txs {
        b.extend_from_slice(&x.id)
    }
    simple_hash(&b)
}
#[allow(clippy::too_many_arguments)]
fn block_id(
    h: u64,
    parent: [u8; 32],
    proposer: &str,
    t: u64,
    tr: [u8; 32],
    state: [u8; 32],
    receipt: [u8; 32],
    sig: [u8; 64],
) -> [u8; 32] {
    let mut b = Vec::new();
    b.extend_from_slice(b"ATC-BLOCK-V1");
    b.extend_from_slice(&h.to_be_bytes());
    b.extend_from_slice(&parent);
    b.extend_from_slice(&(proposer.len() as u32).to_be_bytes());
    b.extend_from_slice(proposer.as_bytes());
    b.extend_from_slice(&t.to_be_bytes());
    b.extend_from_slice(&tr);
    b.extend_from_slice(&state);
    b.extend_from_slice(&receipt);
    b.extend_from_slice(&sig);
    simple_hash(&b)
}
pub trait IndexerSink: Send + Sync {
    fn ingest_finalized(&self, block: &Block) -> Result<(), String>;
}
pub struct BlockChain {
    blocks: Mutex<BTreeMap<u64, Block>>,
    hashes: Mutex<BTreeMap<[u8; 32], u64>>,
    height: Mutex<u64>,
}
impl Default for BlockChain {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: Mutex::new(BTreeMap::new()),
            hashes: Mutex::new(BTreeMap::new()),
            height: Mutex::new(0),
        }
    }
    pub fn genesis(&self, b: Block) -> Result<(), String> {
        if b.height != 0 || self.blocks.lock().unwrap().contains_key(&0) {
            return Err("invalid genesis".into());
        }
        if tx_root(&b.transactions) != b.tx_root
            || block_id(
                b.height,
                b.parent_hash,
                &b.proposer,
                b.timestamp,
                b.tx_root,
                b.state_root,
                b.receipt_root,
                b.signature,
            ) != b.id
        {
            return Err("invalid genesis commitment".into());
        }
        self.hashes.lock().unwrap().insert(b.id, 0);
        self.blocks.lock().unwrap().insert(0, b);
        *self.height.lock().unwrap() = 0;
        Ok(())
    }
    pub fn validate_append(&self, b: &Block) -> Result<(), String> {
        if tx_root(&b.transactions) != b.tx_root {
            return Err("transaction root mismatch".into());
        }
        if block_id(
            b.height,
            b.parent_hash,
            &b.proposer,
            b.timestamp,
            b.tx_root,
            b.state_root,
            b.receipt_root,
            b.signature,
        ) != b.id
        {
            return Err("block id mismatch".into());
        }
        let h = *self.height.lock().unwrap();
        if b.height != h.saturating_add(1) {
            return Err("non-sequential height".into());
        }
        if self.hashes.lock().unwrap().contains_key(&b.id) {
            return Err("duplicate block".into());
        }
        if b.parent_hash
            != self
                .blocks
                .lock()
                .unwrap()
                .get(&h)
                .map(|x| x.id)
                .unwrap_or([0; 32])
        {
            return Err("parent mismatch".into());
        }
        Ok(())
    }
    pub fn append(&self, b: Block) -> Result<(), String> {
        self.validate_append(&b)?;
        self.hashes.lock().unwrap().insert(b.id, b.height);
        self.blocks.lock().unwrap().insert(b.height, b.clone());
        *self.height.lock().unwrap() = b.height;
        Ok(())
    }
    pub fn last(&self) -> Option<Block> {
        let h = *self.height.lock().unwrap();
        self.blocks.lock().unwrap().get(&h).cloned()
    }
    pub fn height(&self) -> u64 {
        *self.height.lock().unwrap()
    }
}
pub struct Node {
    pub chain_id: u64,
    pub pool: Arc<MemoryPool>,
    pub state: Arc<StateDb>,
    pub consensus: Arc<ConsensusEngine>,
    pub chain: Arc<BlockChain>,
    pub storage: Arc<storage::ChainStorage>,
    proposer: String,
    verifier: Arc<dyn SignatureVerifier>,
    indexer: Mutex<Option<Arc<dyn IndexerSink>>>,
    transport: Mutex<Option<Arc<dyn PeerTransport>>>,
    vote_signer: Mutex<Option<(String, [u8; 32])>>,
    pending_votes: Mutex<BTreeMap<[u8; 32], Vec<Vote>>>,
    network_apply_lock: Mutex<()>,
}
impl Node {
    pub fn new(chain_id: u64, proposer: String) -> Self {
        Self {
            chain_id,
            pool: Arc::new(MemoryPool::new(10000, 300)),
            state: Arc::new(StateDb::new()),
            consensus: Arc::new(ConsensusEngine::new(chain_id, proposer.clone())),
            chain: Arc::new(BlockChain::new()),
            storage: Arc::new(storage::ChainStorage::new()),
            proposer,
            verifier: Arc::new(Ed25519Verifier),
            indexer: Mutex::new(None),
            transport: Mutex::new(None),
            vote_signer: Mutex::new(None),
            pending_votes: Mutex::new(BTreeMap::new()),
            network_apply_lock: Mutex::new(()),
        }
    }
    pub fn set_indexer(&self, sink: Arc<dyn IndexerSink>) {
        *self.indexer.lock().unwrap() = Some(sink)
    }

    pub fn proposer_id(&self) -> String {
        self.proposer.clone()
    }

    /// Attach the network transport to the canonical Node/consensus boundary.
    pub fn set_transport(&self, transport: Arc<dyn PeerTransport>) {
        *self.transport.lock().unwrap() = Some(transport);
    }

    pub fn transport_attached(&self) -> bool {
        self.transport.lock().map(|t| t.is_some()).unwrap_or(false)
    }

    /// Run the canonical Node message loop on an already authenticated TCP peer.
    pub fn serve_tcp_stream(
        self: Arc<Self>,
        mut stream: TcpStream,
    ) -> thread::JoinHandle<Result<(), String>> {
        thread::spawn(move || loop {
            match network::read_message(&mut stream)? {
                Some(message) => self.handle_network_message(message)?,
                None => return Ok(()),
            }
        })
    }

    /// Connect a Node to a peer and start its receive loop. Outbound writes use
    /// the same authenticated stream through the shared transport.
    pub fn connect_tcp_peer(
        self: &Arc<Self>,
        transport: Arc<network::TcpPeerTransport>,
        addr: &str,
    ) -> Result<thread::JoinHandle<Result<(), String>>, String> {
        let last = self.chain.last().ok_or("genesis required")?;
        let stream = transport.connect_stream(addr, last.height, last.id)?;
        let reader = stream.try_clone().map_err(|e| e.to_string())?;
        transport.register_stream(stream)?;
        self.set_transport(transport.clone());
        // Ask the peer for any height we do not have yet. The peer answers
        // from durable storage; requesting beyond its tip is harmless.
        transport.broadcast(NetworkMessage::BlockRequest {
            from_height: last.height.saturating_add(1),
        })?;
        Ok(self.clone().serve_tcp_stream(reader))
    }

    /// Configure the validator identity used by the long-running node consensus loop.
    /// The seed is supplied by the operator and is never generated implicitly.
    pub fn set_vote_signer(&self, validator: impl Into<String>, seed: [u8; 32]) {
        *self.vote_signer.lock().unwrap() = Some((validator.into(), seed));
    }

    fn sign_block(&self, mut block: Block) -> Result<Block, String> {
        let Some((validator, seed)) = self.vote_signer.lock().map_err(|_| "vote signer lock poisoned")?.clone() else {
            return Err("proposer signing key is not configured".into());
        };
        if validator != block.proposer {
            return Err("configured validator is not the block proposer".into());
        }
        let signing = ed25519_dalek::SigningKey::from_bytes(&seed);
        let public_key = signing.verifying_key().to_bytes();
        let (_, keys) = self
            .consensus
            .validator_snapshot_for_height(block.height)
            .ok_or("validator snapshot is unavailable for block height")?;
        let registered = keys
            .get(&validator)
            .copied()
            .ok_or("block proposer is not registered at block height")?;
        if registered != public_key {
            return Err("proposer signing key does not match validator identity at block height".into());
        }
        block.signature = signing.sign(&block_signing_bytes(self.chain_id, &block)).to_bytes();
        block.id = block_id(block.height, block.parent_hash, &block.proposer, block.timestamp,
            block.tx_root, block.state_root, block.receipt_root, block.signature);
        Ok(block)
    }

    fn vote_for_block(&self, block: &Block) -> Result<(), String> {
        let Some((validator, seed)) = self.vote_signer.lock().unwrap().clone() else {
            return Ok(());
        };
        let signing = ed25519_dalek::SigningKey::from_bytes(&seed);
        let mut vote = Vote {
            block: block.id,
            voter: validator,
            approve: true,
            signature: [0; 64],
            public_key: signing.verifying_key().to_bytes(),
        };
        vote.signature = signing
            .sign(&consensus::vote_signing_bytes(self.chain_id, &vote))
            .to_bytes();
        self.submit_vote_and_broadcast(vote)
    }

    fn queue_pending_vote(&self, vote: Vote) -> Result<(), String> {
        self.consensus.verify_vote_signature(&vote)?;
        let mut pending = self.pending_votes.lock().map_err(|_| "pending vote lock poisoned")?;
        let total: usize = pending.values().map(Vec::len).sum();
        if total >= 4096 {
            return Err("pending vote buffer is full".into());
        }
        let list = pending.entry(vote.block).or_default();
        if list.iter().any(|existing| {
            existing.voter == vote.voter
                && existing.approve == vote.approve
                && existing.signature == vote.signature
                && existing.public_key == vote.public_key
        }) {
            return Ok(());
        }
        if list.len() >= 128 {
            return Err("pending vote buffer for block is full".into());
        }
        list.push(vote);
        Ok(())
    }

    fn take_pending_votes(&self, block_id: &[u8; 32]) -> Vec<Vote> {
        self.pending_votes
            .lock()
            .map(|mut pending| pending.remove(block_id).unwrap_or_default())
            .unwrap_or_default()
    }

    fn replay_pending_votes(&self, block: &Block) -> Result<(), String> {
        let votes = self.take_pending_votes(&block.id);
        for vote in votes {
            let _ = self.submit_vote(vote);
        }
        if self.consensus.weighted_finality_at_height(&block.id, block.height) {
            let _ = self.finalize_weighted(block)?;
        }
        Ok(())
    }

    fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
        if let Some(transport) = self
            .transport
            .lock()
            .map_err(|_| "transport lock poisoned")?
            .clone()
        {
            transport.broadcast(message)?;
        }
        Ok(())
    }

    /// Apply a network block through the same deterministic state transition
    /// rules used by local block production, then persist it.
    pub fn import_block(&self, b: Block) -> Result<(), String> {
        if self.chain_id
            != b.transactions
                .first()
                .map(|t| t.chain_id)
                .unwrap_or(self.chain_id)
        {
            return Err("block transaction chain-id mismatch".into());
        }
        if let Some((finalized_height, finalized_id)) = self.consensus.finalized() {
            if b.height <= finalized_height {
                return Err("cannot replace finalized block".into());
            }
            if b.height == finalized_height.saturating_add(1) && b.parent_hash != finalized_id {
                return Err("candidate would reorg finalized prefix".into());
            }
        }
        self.chain.validate_append(&b)?;

        if b.height > 0 {
            let (_, keys) = self
                .consensus
                .validator_snapshot_for_height(b.height)
                .ok_or("validator snapshot is unavailable for block height")?;
            let public_key = keys
                .get(&b.proposer)
                .copied()
                .ok_or("block proposer has no signing key at block height")?;
            let key = ed25519_dalek::VerifyingKey::from_bytes(&public_key)
                .map_err(|_| "invalid proposer public key")?;
            key.verify(
                &block_signing_bytes(self.chain_id, &b),
                &ed25519_dalek::Signature::from_bytes(&b.signature),
            ).map_err(|_| "invalid block proposer signature")?;
        }

        let parent = self.chain.last().ok_or("genesis required")?;
        if b.parent_hash != parent.id || b.height != parent.height.saturating_add(1) {
            return Err("block is not the next canonical height".into());
        }

        let state_snapshot = self.state.snapshot();
        let dao_snapshot = self.state.dao_snapshot();
        let issued_snapshot = self.state.issued_base_units();

        let exec = AtcVmExecutor {
            protocol: "1.0.0".into(),
            vm_version: "1.0.0".into(),
            genesis_id: hex::encode(parent.id),
        };
        let mut receipts = Vec::new();
        for tx in &b.transactions {
            if tx.chain_id != self.chain_id
                || !self
                    .verifier
                    .verify(&signing_bytes(tx), &tx.signature, &tx.public_key)
            {
                self.state.restore(state_snapshot);
                let _ = self.state.restore_dao(&dao_snapshot);
                let _ = self.state.restore_issued_base_units(issued_snapshot);
                return Err("invalid transaction signature or chain".into());
            }
            receipts.push(
                exec.execute(tx, self.state.root())
                    .map_err(|e| e.to_string())?,
            );
        }

        self.state
            .apply_batch(&b.transactions)
            .map_err(|e| format!("state transition: {e:?}"))?;

        for tx in &b.transactions {
            if !tx.payload.is_empty() {
                if let Err(e) = self
                    .state
                    .apply_dao_payload(&tx.payload, b.height, &tx.sender_did)
                {
                    self.state.restore(state_snapshot);
                    let _ = self.state.restore_dao(&dao_snapshot);
                    let _ = self.state.restore_issued_base_units(issued_snapshot);
                    return Err(format!("DAO transition: {e}"));
                }
            }
        }

        if let Err(e) = self.state.apply_block_reward(b.height, &b.proposer) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(format!("block reward: {e}"));
        }

        let root = self.state.root();
        if root != b.state_root || receipts::root(&receipts) != b.receipt_root {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err("network block state/receipt root mismatch".into());
        }

        if let Err(e) = self.storage.commit_block_state_issuance(
            b.clone(),
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
            self.state.issued_base_units(),
        ) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        self.chain.append(b.clone())?;
        for tx in &b.transactions {
            self.pool.mark_in_block(&tx.id);
        }
        self.consensus.set_height(b.height);
        self.replay_pending_votes(&b)?;
        self.vote_for_block(&b)?;
        Ok(())
    }

    /// Feed one decoded network message into the canonical Node.
    pub fn handle_network_message(&self, message: NetworkMessage) -> Result<(), String> {
        let _apply_guard = self.network_apply_lock.lock().map_err(|_| "network apply lock poisoned")?;
        match message {
            NetworkMessage::Block(b) => self.import_block(b),
            NetworkMessage::Vote(v) => {
                let block_id = v.block;
                let Some(block) = self.storage.find_block_by_id(block_id) else {
                    return self.queue_pending_vote(v);
                };
                self.submit_vote(v)?;
                if self.consensus.weighted_finality_at_height(&block_id, block.height) {
                    let _ = self.finalize_weighted(&block)?;
                }
                Ok(())
            }
            NetworkMessage::Transaction(tx) => {
                // Remote transactions are admitted locally but are not re-broadcast
                // here, preventing gossip loops. Local submission uses the broadcast path.
                self.submit(tx, 0).map_err(|e| e.to_string())?;
                Ok(())
            }
            NetworkMessage::BlockRequest { from_height } => {
                for h in from_height..=self.chain.height() {
                    if let Some(b) = self.storage.block(h) {
                        self.broadcast(NetworkMessage::Block(b))?;
                    }
                }
                Ok(())
            }
            NetworkMessage::StatusRequest => {
                let last = self.chain.last().ok_or("genesis required")?;
                self.broadcast(NetworkMessage::Status {
                    height: last.height,
                    best_block: last.id,
                    finalized: self.consensus.finalized(),
                })
            }
            NetworkMessage::Status { .. } | NetworkMessage::Hello { .. } => Ok(()),
        }
    }

    pub fn submit_vote_and_broadcast(&self, vote: Vote) -> Result<(), String> {
        self.submit_vote(vote.clone())?;
        self.broadcast(NetworkMessage::Vote(vote))
    }
    pub fn open_storage<P: AsRef<std::path::Path>>(
        chain_id: u64,
        proposer: String,
        path: P,
    ) -> Result<Self, String> {
        let mut n = Self::new(chain_id, proposer);
        n.storage = Arc::new(storage::ChainStorage::open(path)?);
        let recovered_validator_snapshots = n.storage.recover_validator_snapshots()?;
        for (height, (validators, keys)) in &recovered_validator_snapshots {
            for (address, stake) in validators {
                n.consensus.register_validator(address.clone(), *stake)?;
            }
            for (address, public_key) in keys {
                n.consensus.register_validator_key(address, *public_key)?;
            }
            n.consensus.restore_validator_snapshot(*height, validators.clone(), keys.clone())?;
        }
        let recovered_state = n.storage.recover_state_with_dao_at_height()?;
        if let Some((_, (snapshot, dao))) = &recovered_state {
            n.state.restore(snapshot.clone());
            if !dao.is_empty() {
                n.state.restore_dao(dao)?
            }
        }
        let recovered_issuance = n.storage.recover_issuance()?;
        if let Some((_, issued)) = recovered_issuance {
            n.state.restore_issued_base_units(issued)?;
        }
        if let Some(g) = n.storage.block(0) {
            n.state.seal_genesis();
            n.chain.genesis(g)?;
            let mut h = 1u64;
            while let Some(b) = n.storage.block(h) {
                n.chain.append(b)?;
                h = h.saturating_add(1)
            }
        }
        if let Some(last) = n.chain.last() {
            let (state_height, _) = recovered_state
                .as_ref()
                .ok_or("canonical chain has no durable state snapshot")?;
            let (issuance_height, _) = recovered_issuance
                .as_ref()
                .ok_or("canonical chain has no durable issuance record")?;
            if *state_height != last.height {
                return Err(format!(
                    "recovered state height {} does not match canonical tip {}",
                    state_height, last.height
                ));
            }
            if *issuance_height != last.height {
                return Err(format!(
                    "recovered issuance height {} does not match canonical tip {}",
                    issuance_height, last.height
                ));
            }
            if n.state.root() != last.state_root {
                return Err("recovered state root mismatch".into());
            }
            n.consensus.set_height(last.height);
            // A next-height validator snapshot is a valid pending activation
            // and may legitimately exist before the corresponding block is
            // committed. Anything farther in the future is corrupt.
            if recovered_validator_snapshots.keys().any(|h| *h > last.height.saturating_add(1)) {
                return Err("validator snapshot is beyond the next activation height".into());
            }
        }
        // Slashing records are durable evidence/audit records. The active
        // validator snapshot is the canonical recovered voting weight, so
        // evidence is not replayed as a second penalty during restart.
        let _ = n.storage.recover_slashing()?;
        if let Some((height, id)) = n.storage.recover_finalized()? {
            let block = n.storage.block(height).ok_or("finalized block missing")?;
            if block.id != id || height > n.chain.height() {
                return Err("recovered finality marker is inconsistent".into());
            }
            n.consensus.mark_finalized(height, id)?;
        }
        Ok(n)
    }
    pub fn create_genesis(&self, t: u64) -> Result<Block, String> {
        self.create_genesis_with_proposer(t, self.proposer.clone())
    }

    /// Create the canonical shared genesis. All nodes in one network must use
    /// the same genesis proposer so the genesis commitment is identical.
    pub fn create_genesis_with_proposer(
        &self,
        t: u64,
        genesis_proposer: impl Into<String>,
    ) -> Result<Block, String> {
        let b = Block::new(
            0,
            [0; 32],
            genesis_proposer.into(),
            t,
            Vec::new(),
            self.state.root(),
            [0; 32],
            [0; 64],
        );
        self.storage.commit_block_state_issuance(
            b.clone(),
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
            self.state.issued_base_units(),
        )?;
        self.chain.genesis(b.clone())?;
        self.state.seal_genesis();
        self.consensus.set_height(b.height);
        Ok(b)
    }
    pub fn submit_and_broadcast(
        &self,
        tx: Transaction,
        now: u64,
    ) -> Result<[u8; 32], MempoolError> {
        let id = self.submit(tx.clone(), now)?;
        self.broadcast(NetworkMessage::Transaction(tx))
            .map_err(|_| MempoolError::TxNotFound)?;
        Ok(id)
    }

    pub fn submit(&self, tx: Transaction, now: u64) -> Result<[u8; 32], MempoolError> {
        if tx.chain_id != self.chain_id {
            return Err(MempoolError::WrongChain);
        }
        let id = tx.id;
        if !self
            .verifier
            .verify(&signing_bytes(&tx), &tx.signature, &tx.public_key)
        {
            return Err(MempoolError::InvalidSignature);
        }
        self.pool.add(tx, now)?;
        if let Err(e) = self.pool.validate_tx(&id, now) {
            self.pool.remove(&id);
            return Err(e);
        }
        Ok(id)
    }
    /// Produce a deterministic reward-only block for process-level network E2E.
    pub fn produce_reward_block(&self, t: u64) -> Result<Block, String> {
        let parent = self.chain.last().ok_or("genesis required")?;
        let height = parent.height.saturating_add(1);
        let snapshot = self.state.snapshot();
        let dao_snapshot = self.state.dao_snapshot();
        let issued_snapshot = self.state.issued_base_units();

        self.state
            .apply_block_reward(height, &self.proposer)
            .map_err(|e| format!("block reward: {e}"))?;
        let b = self.sign_block(Block::new(
            height, parent.id, self.proposer.clone(), t, Vec::new(),
            self.state.root(), receipts::root(&[]), [0; 64],
        ))?;
        self.chain.validate_append(&b)?;
        if let Err(e) = self.storage.commit_block_state_issuance(
            b.clone(),
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
            self.state.issued_base_units(),
        ) {
            self.state.restore(snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        self.chain.append(b.clone())?;
        self.consensus.set_height(height);
        self.broadcast(NetworkMessage::Block(b.clone()))?;
        self.vote_for_block(&b)?;
        Ok(b)
    }

    pub fn produce(&self, t: u64, max: usize) -> Result<Block, String> {
        let txs = self.pool.get_pending_batch(max);
        if txs.is_empty() {
            return Err("no validated transactions".into());
        }
        for tx in &txs {
            if tx.chain_id != self.chain_id
                || !self
                    .verifier
                    .verify(&signing_bytes(tx), &tx.signature, &tx.public_key)
            {
                return Err("invalid transaction signature or chain".into());
            }
        }
        let parent = self.chain.last().ok_or("genesis required")?;
        let exec = AtcVmExecutor {
            protocol: "1.0.0".into(),
            vm_version: "1.0.0".into(),
            genesis_id: hex::encode(parent.id),
        };
        let mut receipts = Vec::new();
        for tx in &txs {
            receipts.push(
                exec.execute(tx, self.state.root())
                    .map_err(|e| e.to_string())?,
            )
        }
        let mut expected_nonces = BTreeMap::<String, u64>::new();
        for tx in &txs {
            let expected = *expected_nonces
                .entry(tx.sender_did.clone())
                .or_insert_with(|| self.state.nonce(&tx.sender_did));
            if tx.nonce != expected {
                return Err(format!(
                    "invalid nonce for {}: expected {}, got {}",
                    tx.sender_did, expected, tx.nonce
                ));
            }
            *expected_nonces.get_mut(&tx.sender_did).unwrap() = expected.saturating_add(1)
        }
        let state_snapshot = self.state.snapshot();
        let dao_snapshot = self.state.dao_snapshot();
        let issued_snapshot = self.state.issued_base_units();
        self.state
            .apply_batch(&txs)
            .map_err(|e| format!("state transition: {e:?}"))?;
        for tx in &txs {
            if !tx.payload.is_empty() {
                if let Err(e) = self.state.apply_dao_payload(
                    &tx.payload,
                    parent.height.saturating_add(1),
                    &tx.sender_did,
                ) {
                    self.state.restore(state_snapshot);
                    let _ = self.state.restore_dao(&dao_snapshot);
                    let _ = self.state.restore_issued_base_units(issued_snapshot);
                    return Err(format!("DAO transition: {e}"));
                }
            }
        }
        let block_height = parent.height.saturating_add(1);
        self.state
            .apply_block_reward(block_height, &self.proposer)
            .map_err(|e| format!("block reward: {e}"))?;
        let new_root = self.state.root();
        let receipt_root = receipts::root(&receipts);
        let b = self.sign_block(Block::new(
            block_height, parent.id, self.proposer.clone(), t, txs,
            new_root, receipt_root, [0; 64],
        ))?;
        self.chain.validate_append(&b)?;
        if let Err(e) = self.storage.commit_block_state_issuance(
            b.clone(),
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
            self.state.issued_base_units(),
        ) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        self.chain.append(b.clone())?;
        for tx in &b.transactions {
            self.pool.mark_in_block(&tx.id)
        }
        self.consensus.set_height(b.height);
        self.broadcast(NetworkMessage::Block(b.clone()))?;
        self.vote_for_block(&b)?;
        Ok(b)
    }
    fn persist_validator_snapshot(&self, activation_height: u64) -> Result<(), String> {
        let (validators, keys) = self.consensus.validator_snapshot_with_keys()?;
        self.storage.commit_validators(activation_height, &validators, &keys)
    }

    pub fn register_validator(&self, address: String, stake: u64) -> Result<(), String> {
        let activation_height = if self.consensus.has_validator_snapshot(self.consensus.height()) {
            self.consensus.height().saturating_add(1)
        } else {
            self.consensus.height()
        };
        self.consensus.register_validator(address, stake)?;
        // Registration and key binding are intentionally separate API operations.
        // Do not persist an incomplete identity snapshot; register_validator_key()
        // commits the complete registry once the Ed25519 key is known.
        if self.consensus.validator_snapshot_with_keys().is_ok() {
            self.persist_validator_snapshot(activation_height)?;
        }
        Ok(())
    }

    /// Atomically finalize the bootstrap validator set at the current
    /// height after all configured validator keys have been registered.
    pub fn finalize_validator_snapshot(&self) -> Result<(), String> {
        let height = self.consensus.height();
        let (validators, keys) = self.consensus.validator_snapshot_with_keys()?;
        self.consensus.restore_validator_snapshot(height, validators.clone(), keys.clone())?;
        self.storage.commit_validators(height, &validators, &keys)
    }

    pub fn register_validator_key(&self, address: &str, public_key: [u8; 32]) -> Result<(), String> {
        let activation_height = if self.consensus.has_validator_snapshot(self.consensus.height()) {
            self.consensus.height().saturating_add(1)
        } else {
            self.consensus.height()
        };
        self.consensus.register_validator_key(address, public_key)?;
        self.persist_validator_snapshot(activation_height)
    }

    pub fn slash_validator(&self, evidence: SlashingEvidence, penalty: u64) -> Result<u64, String> {
        if evidence.height > self.chain.height() {
            return Err("slashing evidence is above current chain height".into());
        }
        let applied = self.consensus.slash(evidence.clone(), penalty)?;
        if applied == 0 {
            return Err("slashing penalty is zero".into());
        }
        self.storage.commit_slashing(
            evidence.height,
            &evidence.validator,
            evidence.id(),
            applied,
        )?;
        let activation_height = self.consensus.height().saturating_add(1);
        self.persist_validator_snapshot(activation_height)?;
        Ok(applied)
    }

    pub fn unregister_validator(&self, address: &str) -> Result<(), String> {
        self.consensus.unregister_validator(address);
        let (validators, keys) = self.consensus.validator_snapshot_with_keys()?;
        self.storage.commit_validators(self.consensus.height().saturating_add(1), &validators, &keys)
    }

    pub fn submit_vote(&self, vote: Vote) -> Result<(), String> {
        let block = self.storage.find_block_by_id(vote.block).ok_or("vote references unknown block")?;
        self.consensus.vote_at_height(vote, block.height)
    }

    pub fn finalize_weighted(&self, b: &Block) -> Result<bool, String> {
        if !self.consensus.weighted_finality_at_height(&b.id, b.height) {
            return Ok(false);
        }
        if b.height > self.chain.height() || self.chain.last().map(|x| x.id) != Some(b.id) {
            return Err("can only finalize the current canonical tip".into());
        }
        // Persist the canonical finality marker before exposing it in memory.
        // A storage failure must never leave the live node claiming finality
        // that will disappear after restart.
        self.storage.commit_finalized(b.height, b.id)?;
        self.consensus.mark_finalized(b.height, b.id)?;
        if let Some(sink) = self.indexer.lock().unwrap().clone() {
            sink.ingest_finalized(b)?
        }
        Ok(true)
    }

    pub fn finalize(&self, b: &Block, quorum: usize) -> Result<bool, String> {
        if quorum == 0 {
            return Err("quorum must be non-zero".into());
        }
        if !self.consensus.finality_at_height(&b.id, b.height, quorum) {
            return Ok(false);
        }
        if b.height > self.chain.height() || self.chain.last().map(|x| x.id) != Some(b.id) {
            return Err("can only finalize the current canonical tip".into());
        }
        // Storage is the durability boundary: never expose finality in memory
        // before its canonical marker is durably synced.
        self.storage.commit_finalized(b.height, b.id)?;
        self.consensus.mark_finalized(b.height, b.id)?;
        if let Some(sink) = self.indexer.lock().unwrap().clone() {
            sink.ingest_finalized(b)?
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validator_public_key_survives_node_restart() {
        let path = std::env::temp_dir().join(format!(
            "atc-node-validator-restart-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let key = ed25519_dalek::SigningKey::from_bytes(&[19u8; 32]);
        let public_key = key.verifying_key().to_bytes();

        {
            let node = Node::open_storage(658467, "validator-a".into(), &path).unwrap();
            node.create_genesis_with_proposer(0, "atc-genesis").unwrap();
            node.register_validator("validator-a".into(), 100).unwrap();
            node.register_validator_key("validator-a", public_key).unwrap();
            assert_eq!(node.consensus.validator_public_key("validator-a"), Some(public_key));
        }

        {
            let node = Node::open_storage(658467, "validator-a".into(), &path).unwrap();
            assert_eq!(node.consensus.validator_stake("validator-a"), 100);
            assert_eq!(node.consensus.validator_public_key("validator-a"), Some(public_key));
        }

        for suffix in ["", ".state", ".validators", ".finality", ".slashing", ".issuance"] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[derive(Default)]
    struct CaptureTransport {
        messages: Mutex<Vec<NetworkMessage>>,
    }

    impl PeerTransport for CaptureTransport {
        fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
            self.messages.lock().unwrap().push(message);
            Ok(())
        }
    }

    fn configure_two_validator_node(node: &Node, proposer: &str) {
        node.register_validator("validator-a".into(), 2).unwrap();
        node.register_validator("validator-b".into(), 1).unwrap();
        node.register_validator_key("validator-a", ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]).verifying_key().to_bytes()).unwrap();
        node.register_validator_key("validator-b", ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]).verifying_key().to_bytes()).unwrap();
        node.set_vote_signer(proposer, if proposer == "validator-a" { [1u8; 32] } else { [2u8; 32] });
    }

    #[test]
    fn network_vote_before_block_is_buffered_and_replayed() {
        let producer = Node::new(658467, "validator-a".into());
        producer.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer, "validator-a");
        let block = producer.produce_reward_block(2).unwrap();

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        // Do not let receiver produce its own vote; the delayed vote is from validator-a.
        let signing = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut vote = Vote {
            block: block.id,
            voter: "validator-a".into(),
            approve: true,
            signature: [0; 64],
            public_key: signing.verifying_key().to_bytes(),
        };
        vote.signature = signing
            .sign(&consensus::vote_signing_bytes(658467, &vote))
            .to_bytes();

        // Network reordering: vote arrives before the corresponding block.
        receiver.handle_network_message(NetworkMessage::Vote(vote.clone())).unwrap();
        assert_eq!(receiver.consensus.finalized(), None);

        // The later block arrival resolves the canonical height and replays the buffered vote.
        receiver.handle_network_message(NetworkMessage::Block(block.clone())).unwrap();
        assert_eq!(receiver.chain.height(), 1);
        assert_eq!(receiver.consensus.finalized(), Some((1, block.id)));
    }

    #[test]
    fn future_block_is_rejected_without_state_corruption_then_accepts_after_parent() {
        let producer = Node::new(658467, "validator-a".into());
        producer.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer, "validator-a");
        let block1 = producer.produce_reward_block(2).unwrap();
        let block2 = producer.produce_reward_block(3).unwrap();

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");

        let before = receiver.state.root();
        assert_eq!(
            receiver.handle_network_message(NetworkMessage::Block(block2.clone())).unwrap_err(),
            "non-sequential height"
        );
        assert_eq!(receiver.chain.height(), 0);
        assert_eq!(receiver.state.root(), before);

        receiver.handle_network_message(NetworkMessage::Block(block1)).unwrap();
        receiver.handle_network_message(NetworkMessage::Block(block2.clone())).unwrap();
        assert_eq!(receiver.chain.height(), 2);
        assert_eq!(receiver.chain.last().unwrap().id, block2.id);
    }

    #[test]
    fn duplicate_network_vote_is_rejected_after_first_acceptance() {
        let producer = Node::new(658467, "validator-a".into());
        producer.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer, "validator-a");
        let block = producer.produce_reward_block(2).unwrap();

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        let signing = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut vote = Vote {
            block: block.id,
            voter: "validator-a".into(),
            approve: true,
            signature: [0; 64],
            public_key: signing.verifying_key().to_bytes(),
        };
        vote.signature = signing
            .sign(&consensus::vote_signing_bytes(658467, &vote))
            .to_bytes();

        receiver.handle_network_message(NetworkMessage::Block(block.clone())).unwrap();
        receiver.handle_network_message(NetworkMessage::Vote(vote.clone())).unwrap();
        assert_eq!(
            receiver.handle_network_message(NetworkMessage::Vote(vote)).unwrap_err(),
            "duplicate voter"
        );
    }

    #[test]
    fn restart_resync_does_not_reconstruct_transient_votes_and_late_vote_restores_finality() {
        let path = std::env::temp_dir().join(format!(
            "atc-restart-resync-votes-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));

        let producer = Node::open_storage(658467, "validator-a".into(), &path).unwrap();
        producer.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer, "validator-a");
        let block = producer.produce_reward_block(2).unwrap();

        // The block is durable, but the in-memory vote is intentionally not.
        assert_eq!(producer.storage.block(block.height), Some(block.clone()));

        drop(producer);

        let receiver = Node::open_storage(658467, "validator-b".into(), &path).unwrap();
        assert_eq!(receiver.chain.height(), block.height);
        assert_eq!(receiver.consensus.finalized(), None);

        // Exercise the actual BlockRequest -> BlockResponse/broadcast path.
        let source = Node::open_storage(658467, "validator-a".into(), &path).unwrap();
        let capture = Arc::new(CaptureTransport::default());
        source.set_transport(capture.clone());
        source.handle_network_message(NetworkMessage::BlockRequest { from_height: block.height }).unwrap();
        let messages = capture.messages.lock().unwrap().clone();
        assert_eq!(messages, vec![NetworkMessage::Block(block.clone())]);
        for message in messages {
            receiver.handle_network_message(message).unwrap();
        }
        assert_eq!(receiver.chain.height(), block.height);
        assert_eq!(receiver.consensus.finalized(), None);

        let key_a = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut late_vote = Vote {
            block: block.id,
            voter: "validator-a".into(),
            approve: true,
            signature: [0; 64],
            public_key: key_a.verifying_key().to_bytes(),
        };
        late_vote.signature = key_a
            .sign(&consensus::vote_signing_bytes(658467, &late_vote))
            .to_bytes();

        // One delayed vote after restart is still insufficient: the node did
        // not reconstruct a phantom vote from pre-restart memory.
        receiver.handle_network_message(NetworkMessage::Vote(late_vote)).unwrap();
        assert_eq!(receiver.consensus.finalized(), None);

        let key_b = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
        let mut second_vote = Vote {
            block: block.id,
            voter: "validator-b".into(),
            approve: true,
            signature: [0; 64],
            public_key: key_b.verifying_key().to_bytes(),
        };
        second_vote.signature = key_b
            .sign(&consensus::vote_signing_bytes(658467, &second_vote))
            .to_bytes();

        receiver.handle_network_message(NetworkMessage::Vote(second_vote)).unwrap();
        assert_eq!(receiver.consensus.finalized(), Some((block.height, block.id)));

        drop(receiver);
        let reopened = Node::open_storage(658467, "validator-b".into(), &path).unwrap();
        assert_eq!(reopened.consensus.finalized(), Some((block.height, block.id)));
        assert_eq!(reopened.storage.recover_finalized().unwrap(), Some((block.height, block.id)));

        for suffix in ["", ".state", ".validators", ".finality", ".slashing", ".issuance"] {
            let target = if suffix.is_empty() { path.clone() } else { std::path::PathBuf::from(format!("{}{}", path.display(), suffix)) };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn conflicting_same_height_block_is_rejected_after_finality() {
        let producer_a = Node::new(658467, "validator-a".into());
        producer_a.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer_a, "validator-a");
        let canonical = producer_a.produce_reward_block(2).unwrap();

        let producer_b = Node::new(658467, "validator-a".into());
        producer_b.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&producer_b, "validator-a");
        let conflicting = producer_b.produce_reward_block(3).unwrap();
        assert_ne!(canonical.id, conflicting.id);
        assert_eq!(canonical.height, conflicting.height);
        assert_eq!(canonical.parent_hash, conflicting.parent_hash);

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        receiver.handle_network_message(NetworkMessage::Block(canonical.clone())).unwrap();

        let key_a = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut va = Vote { block: canonical.id, voter: "validator-a".into(), approve: true, signature: [0;64], public_key: key_a.verifying_key().to_bytes() };
        va.signature = key_a.sign(&consensus::vote_signing_bytes(658467, &va)).to_bytes();
        let key_b = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
        let mut vb = Vote { block: canonical.id, voter: "validator-b".into(), approve: true, signature: [0;64], public_key: key_b.verifying_key().to_bytes() };
        vb.signature = key_b.sign(&consensus::vote_signing_bytes(658467, &vb)).to_bytes();
        receiver.handle_network_message(NetworkMessage::Vote(va)).unwrap();
        receiver.handle_network_message(NetworkMessage::Vote(vb)).unwrap();
        assert_eq!(receiver.consensus.finalized(), Some((canonical.height, canonical.id)));

        let before_root = receiver.state.root();
        assert!(receiver.handle_network_message(NetworkMessage::Block(conflicting)).is_err());
        assert_eq!(receiver.chain.height(), canonical.height);
        assert_eq!(receiver.chain.last().unwrap().id, canonical.id);
        assert_eq!(receiver.state.root(), before_root);
        assert_eq!(receiver.consensus.finalized(), Some((canonical.height, canonical.id)));
        assert_eq!(receiver.storage.block(canonical.height).unwrap().id, canonical.id);
    }

    #[test]
    fn longer_divergent_chain_cannot_reorg_finalized_prefix() {
        let canonical = Node::new(658467, "validator-a".into());
        canonical.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&canonical, "validator-a");
        let canonical_h1 = canonical.produce_reward_block(2).unwrap();
        let canonical_h2 = canonical.produce_reward_block(3).unwrap();

        let fork = Node::new(658467, "validator-a".into());
        fork.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&fork, "validator-a");
        let fork_h1 = fork.produce_reward_block(99).unwrap();
        let fork_h2 = fork.produce_reward_block(100).unwrap();
        assert_ne!(canonical_h1.id, fork_h1.id);
        assert_ne!(canonical_h2.id, fork_h2.id);
        assert_eq!(fork_h2.parent_hash, fork_h1.id);

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        receiver.handle_network_message(NetworkMessage::Block(canonical_h1.clone())).unwrap();

        let key_a = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut vote_a = Vote { block: canonical_h1.id, voter: "validator-a".into(), approve: true, signature: [0;64], public_key: key_a.verifying_key().to_bytes() };
        vote_a.signature = key_a.sign(&consensus::vote_signing_bytes(658467, &vote_a)).to_bytes();
        let key_b = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
        let mut vote_b = Vote { block: canonical_h1.id, voter: "validator-b".into(), approve: true, signature: [0;64], public_key: key_b.verifying_key().to_bytes() };
        vote_b.signature = key_b.sign(&consensus::vote_signing_bytes(658467, &vote_b)).to_bytes();
        receiver.handle_network_message(NetworkMessage::Vote(vote_a)).unwrap();
        receiver.handle_network_message(NetworkMessage::Vote(vote_b)).unwrap();
        assert_eq!(receiver.consensus.finalized(), Some((canonical_h1.height, canonical_h1.id)));

        assert!(receiver.handle_network_message(NetworkMessage::Block(fork_h1)).is_err());
        assert!(receiver.handle_network_message(NetworkMessage::Block(fork_h2)).is_err());
        assert_eq!(receiver.chain.height(), canonical_h1.height);
        assert_eq!(receiver.chain.last().unwrap().id, canonical_h1.id);
        assert_eq!(receiver.consensus.finalized(), Some((canonical_h1.height, canonical_h1.id)));

        // The longer canonical continuation remains valid and cannot be
        // displaced by the rejected fork.
        receiver.handle_network_message(NetworkMessage::Block(canonical_h2.clone())).unwrap();
        assert_eq!(receiver.chain.last().unwrap().id, canonical_h2.id);
    }

    #[test]
    fn late_conflicting_block_after_finality_cannot_replace_canonical_state() {
        let source = Node::new(658467, "validator-a".into());
        source.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&source, "validator-a");
        let canonical = source.produce_reward_block(2).unwrap();

        let fork_source = Node::new(658467, "validator-a".into());
        fork_source.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&fork_source, "validator-a");
        let fork = fork_source.produce_reward_block(3).unwrap();
        assert_ne!(canonical.id, fork.id);

        let node = Node::new(658467, "validator-b".into());
        node.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&node, "validator-b");
        node.handle_network_message(NetworkMessage::Block(canonical.clone())).unwrap();

        let key_a = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut vote_a = Vote { block: canonical.id, voter: "validator-a".into(), approve: true, signature: [0;64], public_key: key_a.verifying_key().to_bytes() };
        vote_a.signature = key_a.sign(&consensus::vote_signing_bytes(658467, &vote_a)).to_bytes();
        let key_b = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
        let mut vote_b = Vote { block: canonical.id, voter: "validator-b".into(), approve: true, signature: [0;64], public_key: key_b.verifying_key().to_bytes() };
        vote_b.signature = key_b.sign(&consensus::vote_signing_bytes(658467, &vote_b)).to_bytes();
        node.handle_network_message(NetworkMessage::Vote(vote_a)).unwrap();
        node.handle_network_message(NetworkMessage::Vote(vote_b)).unwrap();
        assert_eq!(node.consensus.finalized(), Some((canonical.height, canonical.id)));

        let state_root = node.state.root();
        let finalized = node.consensus.finalized();
        assert!(node.import_block(fork).is_err());
        assert_eq!(node.chain.last().unwrap().id, canonical.id);
        assert_eq!(node.state.root(), state_root);
        assert_eq!(node.consensus.finalized(), finalized);
    }

    #[test]
    fn duplicate_canonical_block_after_resync_is_idempotent_without_finality_change() {
        let source = Node::new(658467, "validator-a".into());
        source.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&source, "validator-a");
        let block = source.produce_reward_block(2).unwrap();

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        receiver.handle_network_message(NetworkMessage::Block(block.clone())).unwrap();
        let root = receiver.state.root();

        assert!(receiver.handle_network_message(NetworkMessage::Block(block.clone())).is_err());
        assert_eq!(receiver.chain.height(), block.height);
        assert_eq!(receiver.chain.last().unwrap().id, block.id);
        assert_eq!(receiver.state.root(), root);
        assert_eq!(receiver.consensus.finalized(), None);
    }

    #[test]
    fn block_request_from_divergent_peer_cannot_replace_finalized_block() {
        let canonical = Node::new(658467, "validator-a".into());
        canonical.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&canonical, "validator-a");
        let block = canonical.produce_reward_block(2).unwrap();

        let divergent = Node::new(658467, "validator-a".into());
        divergent.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&divergent, "validator-a");
        let other = divergent.produce_reward_block(99).unwrap();
        assert_ne!(block.id, other.id);

        let receiver = Node::new(658467, "validator-b".into());
        receiver.create_genesis_with_proposer(1, "genesis").unwrap();
        configure_two_validator_node(&receiver, "validator-b");
        receiver.handle_network_message(NetworkMessage::Block(block.clone())).unwrap();

        let key_a = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
        let mut va = Vote { block: block.id, voter: "validator-a".into(), approve: true, signature: [0;64], public_key: key_a.verifying_key().to_bytes() };
        va.signature = key_a.sign(&consensus::vote_signing_bytes(658467, &va)).to_bytes();
        let key_b = ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]);
        let mut vb = Vote { block: block.id, voter: "validator-b".into(), approve: true, signature: [0;64], public_key: key_b.verifying_key().to_bytes() };
        vb.signature = key_b.sign(&consensus::vote_signing_bytes(658467, &vb)).to_bytes();
        receiver.handle_network_message(NetworkMessage::Vote(va)).unwrap();
        receiver.handle_network_message(NetworkMessage::Vote(vb)).unwrap();
        assert_eq!(receiver.consensus.finalized(), Some((block.height, block.id)));

        let transport = Arc::new(CaptureTransport::default());
        divergent.set_transport(transport.clone());
        divergent.handle_network_message(NetworkMessage::BlockRequest { from_height: 1 }).unwrap();
        let messages = transport.messages.lock().unwrap().clone();
        assert!(messages.iter().any(|m| matches!(m, NetworkMessage::Block(b) if b.id == other.id)));

        for message in messages {
            let _ = receiver.handle_network_message(message);
        }
        assert_eq!(receiver.chain.height(), block.height);
        assert_eq!(receiver.chain.last().unwrap().id, block.id);
        assert_eq!(receiver.consensus.finalized(), Some((block.height, block.id)));
    }

    #[test]
    fn genesis_sets_chain_height_and_allows_first_append() {
        let chain = BlockChain::new();
        let genesis = Block::new(
            0,
            [0; 32],
            "validator-0".into(),
            1,
            Vec::new(),
            [1; 32],
            [0; 32],
            [0; 64],
        );

        chain.genesis(genesis.clone()).unwrap();

        assert_eq!(chain.height(), 0);
        assert_eq!(chain.last(), Some(genesis.clone()));

        let block1 = Block::new(
            1,
            genesis.id,
            "validator-0".into(),
            361,
            Vec::new(),
            [2; 32],
            [0; 32],
            [0; 64],
        );

        chain.append(block1.clone()).unwrap();

        assert_eq!(chain.height(), 1);
        assert_eq!(chain.last(), Some(block1));
    }
}
