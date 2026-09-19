// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Canonical ATC node process.
//!
//! The process now exposes the same Runtime/Node instance that owns the
//! mempool, block production, state transition and durable storage.

use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::rpc::{serve, DevnetRpc};
use atc_node::runtime::Runtime;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:39471".to_string());

    let genesis = Genesis::devnet();
    let (peers, boot_hash) = match devnet_boot(
        &genesis,
        &[(1, "atc-node-1".to_string()), (2, "atc-node-2".to_string())],
    ) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("devnet_boot fehlgeschlagen: {}", e);
            std::process::exit(1);
        }
    };

    let runtime = match Runtime::devnet("atc-node-1") {
        Ok(v) => Arc::new(v),
        Err(e) => {
            eprintln!("canonical runtime startup failed: {e}");
            std::process::exit(1);
        }
    };

    eprintln!(
        "ATC-Node gestartet | Chain-ID {} | Boot-Hash {} | Peers {} | Runtime-Hoehe {} | RPC {}",
        runtime.node.chain_id,
        boot_hash,
        peers.len(),
        runtime.node.chain.height(),
        addr
    );

    serve(&addr, DevnetRpc::from_runtime(runtime, &genesis, &peers))
}
