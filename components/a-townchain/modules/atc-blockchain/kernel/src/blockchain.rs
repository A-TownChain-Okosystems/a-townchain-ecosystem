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
use ed25519_dalek::Signer;
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
        self.chain.validate_append(&b)?;

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
            match exec.execute(tx, self.state.root()) {
                Ok(receipt) => receipts.push(receipt),
                Err(e) => {
                    self.state.restore(state_snapshot);
                    let _ = self.state.restore_dao(&dao_snapshot);
                    let _ = self.state.restore_issued_base_units(issued_snapshot);
                    return Err(e.to_string());
                }
            }
        }

        if let Err(e) = self.state.apply_batch(&b.transactions) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(format!("state transition: {e:?}"));
        }

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

        if let Err(e) = self.storage.commit(b.clone()) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self.storage.commit_state_with_dao(
            b.height,
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
        ) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self
            .storage
            .commit_issuance(b.height, self.state.issued_base_units())
        {
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
        self.vote_for_block(&b)?;
        Ok(())
    }

    /// Feed one decoded network message into the canonical Node.
    pub fn handle_network_message(&self, message: NetworkMessage) -> Result<(), String> {
        match message {
            NetworkMessage::Block(b) => self.import_block(b),
            NetworkMessage::Vote(v) => {
                let block_id = v.block;
                self.submit_vote(v)?;
                if self.consensus.weighted_finality(&block_id) {
                    if let Some(block) = self.chain.last() {
                        if block.id == block_id {
                            let _ = self.finalize_weighted(&block)?;
                        }
                    }
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
        if let Some((height, validators)) = n.storage.recover_validators()? {
            if n.storage.block(height).is_none() {
                return Err("validator snapshot references missing block".into());
            }
            n.consensus.restore_validators_with_keys(validators)?;
            
        }
        if let Some((snapshot, dao)) = n.storage.recover_state_with_dao()? {
            n.state.restore(snapshot);
            if !dao.is_empty() {
                n.state.restore_dao(&dao)?
            }
        }
        if let Some((_, issued)) = n.storage.recover_issuance()? {
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
            if n.state.root() != last.state_root {
                return Err("recovered state root mismatch".into());
            }
            n.consensus.set_height(last.height);
        }
        // Slashing records are durable evidence/audit records. The active
        // validator snapshot is the canonical recovered voting weight, so
        // evidence is not replayed as a second penalty during restart.
        let slashing_records = n.storage.recover_slashing()?;
        n.consensus
            .restore_slashing_evidence(slashing_records.into_iter().map(|(_, _, evidence_id, _)| evidence_id));
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
        self.chain.genesis(b.clone())?;
        self.storage.commit(b.clone())?;
        self.storage.commit_state_with_dao(
            b.height,
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
        )?;
        self.storage
            .commit_issuance(b.height, self.state.issued_base_units())?;
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
        let b = Block::new(
            height,
            parent.id,
            self.proposer.clone(),
            t,
            Vec::new(),
            self.state.root(),
            receipts::root(&[]),
            [0; 64],
        );
        self.chain.validate_append(&b)?;
        if let Err(e) = self.storage.commit(b.clone()) {
            self.state.restore(snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self.storage.commit_state_with_dao(
            height,
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
        ) {
            self.state.restore(snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self
            .storage
            .commit_issuance(height, self.state.issued_base_units())
        {
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
        let b = Block::new(
            block_height,
            parent.id,
            self.proposer.clone(),
            t,
            txs,
            new_root,
            receipt_root,
            [0; 64],
        );
        self.chain.validate_append(&b)?;
        if let Err(e) = self.storage.commit(b.clone()) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self.storage.commit_state_with_dao(
            b.height,
            &self.state.snapshot(),
            &self.state.dao_snapshot(),
        ) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        if let Err(e) = self
            .storage
            .commit_issuance(b.height, self.state.issued_base_units())
        {
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
    pub fn register_validator(&self, address: String, stake: u64) -> Result<(), String> {
        self.consensus.register_validator(address, stake)?;
        self.persist_validator_snapshot()
    }

    pub fn register_validator_with_key(
        &self,
        address: String,
        stake: u64,
        public_key: [u8; 32],
    ) -> Result<(), String> {
        self.consensus
            .register_validator_with_key(address, stake, public_key)?;
        self.persist_validator_snapshot()
    }

    fn persist_validator_snapshot(&self) -> Result<(), String> {
        self.storage.commit_validators(
            self.consensus.height(),
            &self.consensus.validators_with_keys_snapshot(),
        )
    }

    pub fn slash_validator(&self, evidence: SlashingEvidence, penalty: u64) -> Result<u64, String> {
        if evidence.height > self.chain.height() {
            return Err("slashing evidence is above current chain height".into());
        }
        let evidence_id = evidence.id();
        let applied = self.consensus.slash(evidence.clone(), penalty)?;
        if applied == 0 {
            return Err("slashing penalty is zero".into());
        }
        self.storage.commit_slashing(
            evidence.height,
            &evidence.validator,
            evidence_id,
            applied,
        )?;
        self.persist_validator_snapshot()?;
        Ok(applied)
    }

    pub fn unregister_validator(&self, address: &str) -> Result<(), String> {
        self.consensus.unregister_validator(address);
        self.storage.commit_validators(
            self.consensus.height(),
            &self.consensus.validators_snapshot(),
        )
    }

    pub fn submit_vote(&self, vote: Vote) -> Result<(), String> {
        self.consensus.vote(vote)
    }

    pub fn finalize_weighted(&self, b: &Block) -> Result<bool, String> {
        if !self.consensus.weighted_finality(&b.id) {
            return Ok(false);
        }
        if b.height > self.chain.height() || self.chain.last().map(|x| x.id) != Some(b.id) {
            return Err("can only finalize the current canonical tip".into());
        }
        self.consensus.mark_finalized(b.height, b.id)?;
        self.storage.commit_finalized(b.height, b.id)?;
        if let Some(sink) = self.indexer.lock().unwrap().clone() {
            sink.ingest_finalized(b)?
        }
        Ok(true)
    }

    pub fn finalize(&self, b: &Block, quorum: usize) -> Result<bool, String> {
        if quorum == 0 {
            return Err("quorum must be non-zero".into());
        }
        if !self.consensus.finality(&b.id, quorum) {
            return Ok(false);
        }
        if b.height > self.chain.height() || self.chain.last().map(|x| x.id) != Some(b.id) {
            return Err("can only finalize the current canonical tip".into());
        }
        self.consensus.mark_finalized(b.height, b.id)?;
        self.storage.commit_finalized(b.height, b.id)?;
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
