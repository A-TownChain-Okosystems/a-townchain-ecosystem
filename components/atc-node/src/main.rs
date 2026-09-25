// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Canonical ATC node process.
//!
//! A normal node process owns the canonical L1 runtime and keeps the full
//! network/consensus path alive:
//! RPC -> mempool -> proposer -> block broadcast -> block validation/state
//! transition -> validator vote -> weighted finality -> durable storage.

use atc_blockchain::{chain_identity::NUMERIC_CHAIN_ID, network::TcpPeerTransport, Node};
use atc_node::bootstrap::Genesis;
use atc_node::rpc::{serve, DevnetRpc};
use atc_node::runtime::Runtime;
use std::{env, net::TcpListener, path::PathBuf, sync::Arc, thread, time::Duration};

const DEFAULT_GENESIS_PROPOSER: &str = "atc-genesis";

#[derive(Clone)]
struct ValidatorConfig {
    id: String,
    stake: u64,
    seed: [u8; 32],
}

fn parse_seed(value: &str) -> Result<[u8; 32], String> {
    let raw = value.trim();
    if raw.len() != 64 {
        return Err("validator seed must contain exactly 64 hex characters".into());
    }
    let bytes = hex_to_bytes(raw)?;
    bytes
        .try_into()
        .map_err(|_| "validator seed must be 32 bytes".into())
}

fn hex_to_bytes(value: &str) -> Result<Vec<u8>, String> {
    if value.len() % 2 != 0 {
        return Err("hex value has odd length".into());
    }
    (0..value.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&value[i..i + 2], 16).map_err(|_| "invalid hex value".to_string())
        })
        .collect()
}

fn validators_from_env() -> Result<Vec<ValidatorConfig>, String> {
    let raw = env::var("ATC_VALIDATORS").unwrap_or_else(|_| {
        let id = env::var("ATC_NODE_ID").unwrap_or_else(|_| "atc-node-1".into());
        format!("{id}:1:{}", "01".repeat(32))
    });
    let mut out = Vec::new();
    for item in raw.split(',').filter(|x| !x.trim().is_empty()) {
        let mut parts = item.split(':');
        let id = parts
            .next()
            .ok_or("validator id missing")?
            .trim()
            .to_string();
        let stake = parts
            .next()
            .ok_or("validator stake missing")?
            .parse::<u64>()
            .map_err(|_| "invalid validator stake")?;
        let seed = parse_seed(parts.next().ok_or("validator seed missing")?)?;
        if parts.next().is_some() || id.is_empty() || stake == 0 {
            return Err("validator format is id:stake:64hexseed".into());
        }
        out.push(ValidatorConfig { id, stake, seed });
    }
    if out.is_empty() {
        return Err("ATC_VALIDATORS must contain at least one validator".into());
    }
    Ok(out)
}

fn is_local_proposer(node: &Node) -> bool {
    let height = node.chain.height().saturating_add(1);
    let mut validators: Vec<_> = node.consensus.validators_snapshot().into_iter().collect();
    validators.sort_by(|a, b| a.0.cmp(&b.0));
    !validators.is_empty()
        && validators[(height as usize - 1) % validators.len()].0 == node.proposer_id()
}

fn load_or_create_node(node_id: &str, data_dir: &PathBuf) -> Result<Arc<Node>, String> {
    std::fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
    let journal_path = data_dir.join("chain.journal");
    let node = Node::open_storage(NUMERIC_CHAIN_ID, node_id.to_string(), journal_path)?;
    if node.chain.last().is_none() {
        node.state.genesis_credit("alice", 1_000_000)?;
        node.create_genesis_with_proposer(0, DEFAULT_GENESIS_PROPOSER)?;
    }
    Ok(Arc::new(node))
}

fn attach_network(node: Arc<Node>, listen_addr: String, peers: Vec<String>) -> Result<(), String> {
    let listener =
        TcpListener::bind(&listen_addr).map_err(|e| format!("network bind {listen_addr}: {e}"))?;
    let transport = Arc::new(TcpPeerTransport::new(node.chain_id, node.proposer_id()));
    node.set_transport(transport.clone());

    {
        let node = node.clone();
        let transport = transport.clone();
        thread::spawn(move || {
            for accepted in listener.incoming() {
                match accepted {
                    Ok(stream) => match TcpPeerTransport::accept_stream(
                        stream,
                        node.chain_id,
                        node.proposer_id(),
                        node.chain.height(),
                        node.chain.last().map(|b| b.id).unwrap_or([0; 32]),
                    ) {
                        Ok((stream, peer, _height, _best)) => {
                            let _ = transport.register_stream_with_peer_id(stream.try_clone().unwrap(), peer.clone());
                            let _ = node.clone().serve_tcp_stream_with_peer(stream, peer);
                        }
                        Err(e) => eprintln!("peer handshake failed: {e}"),
                    },
                    Err(e) => eprintln!("peer accept failed: {e}"),
                }
            }
        });
    }

    for peer in peers {
        let node = node.clone();
        let transport = transport.clone();
        thread::spawn(move || loop {
            match node.connect_tcp_peer(transport.clone(), &peer) {
                Ok(handle) => {
                    let _ = handle.join();
                }
                Err(_) => thread::sleep(Duration::from_millis(250)),
            }
        });
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let node_id = env::var("ATC_NODE_ID").unwrap_or_else(|_| "atc-node-1".into());
    let rpc_addr = env::var("ATC_RPC_ADDR").unwrap_or_else(|_| "127.0.0.1:39471".into());
    let listen_addr = env::var("ATC_P2P_ADDR").unwrap_or_else(|_| "127.0.0.1:39472".into());
    let data_dir =
        PathBuf::from(env::var("ATC_DATA_DIR").unwrap_or_else(|_| format!("./data/{node_id}")));
    let peers = env::var("ATC_PEERS")
        .unwrap_or_default()
        .split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();
    let block_interval = env::var("ATC_BLOCK_INTERVAL_SECS")
        .ok()
        .and_then(|x| x.parse::<u64>().ok())
        .filter(|x| *x > 0)
        .unwrap_or(360);

    let validators = match validators_from_env() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("validator configuration invalid: {e}");
            std::process::exit(1);
        }
    };

    let runtime = match load_or_create_node(&node_id, &data_dir) {
        Ok(node) => Arc::new(Runtime { node }),
        Err(e) => {
            eprintln!("canonical runtime startup failed: {e}");
            std::process::exit(1);
        }
    };

    for validator in &validators {
        if let Err(e) = runtime.node.register_validator(validator.id.clone(), u128::from(validator.stake)) {
            eprintln!("validator registration failed for {}: {e}", validator.id);
            std::process::exit(1);
        }
        let signing = ed25519_dalek::SigningKey::from_bytes(&validator.seed);
        if let Err(e) = runtime.node.register_validator_key(
            &validator.id,
            signing.verifying_key().to_bytes(),
        ) {
            eprintln!("validator key registration failed for {}: {e}", validator.id);
            std::process::exit(1);
        }
    }

    if let Err(e) = runtime.node.finalize_validator_snapshot() {
        eprintln!("validator snapshot finalization failed: {e}");
        std::process::exit(1);
    }

    let local_validator = validators.iter().find(|v| v.id == node_id);
    if let Some(v) = local_validator {
        runtime.node.set_vote_signer(v.id.clone(), v.seed);
    }

    if let Err(e) = attach_network(runtime.node.clone(), listen_addr.clone(), peers) {
        eprintln!("network startup failed: {e}");
        std::process::exit(1);
    }

    let genesis = Genesis::devnet();
    let rpc_runtime = runtime.clone();
    let rpc_addr_clone = rpc_addr.clone();
    thread::spawn(move || {
        if let Err(e) = serve(
            &rpc_addr_clone,
            DevnetRpc::from_runtime(rpc_runtime, &genesis, &atc_node::peers::PeerTable::new()),
        ) {
            eprintln!("RPC server stopped: {e}");
        }
    });

    let producer = runtime.node.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(block_interval));
        if !is_local_proposer(&producer) {
            continue;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let result = if producer.pool.get_pending_batch(100).is_empty() {
            producer.produce_reward_block(now)
        } else {
            producer.produce(now, 100)
        };
        if let Err(e) = result {
            eprintln!("block production skipped: {e}");
        }
    });

    eprintln!(
        "ATC-Node started | chain={} node={} rpc={} p2p={} data={}",
        NUMERIC_CHAIN_ID,
        node_id,
        rpc_addr,
        listen_addr,
        data_dir.display()
    );

    loop {
        thread::park_timeout(Duration::from_secs(60));
    }
}
