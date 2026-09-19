// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Canonical ATC node JSON-RPC boundary.
//!
//! The RPC is deliberately thin: transaction validation, mempool admission,
//! block execution, rewards, persistence and state roots remain in the
//! canonical atc-blockchain::Node.

use crate::bootstrap::Genesis;
use crate::peers::PeerTable;
use crate::runtime::Runtime;
use atc_blockchain::mempool::{Transaction, TxType};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub struct DevnetRpc {
    pub chain_id: String,
    pub boot_hash: u64,
    pub peer_count: usize,
    runtime: Option<Arc<Runtime>>,
}

impl DevnetRpc {
    pub fn from_state(genesis: &Genesis, peers: &PeerTable) -> Self {
        Self {
            chain_id: genesis.chain_id.clone(),
            boot_hash: genesis.boot_hash(),
            peer_count: peers.len(),
            runtime: None,
        }
    }

    pub fn from_runtime(
        runtime: Arc<Runtime>,
        genesis: &Genesis,
        peers: &PeerTable,
    ) -> Self {
        Self {
            chain_id: runtime.node.chain_id.to_string(),
            boot_hash: genesis.boot_hash(),
            peer_count: peers.len(),
            runtime: Some(runtime),
        }
    }

    pub fn answer(&self, req: &str) -> String {
        match req.trim() {
            "CHAIN_ID" => self.chain_id.clone(),
            "BOOT_HASH" => self.boot_hash.to_string(),
            "PEERS" => self.peer_count.to_string(),
            "PING" => "PONG".to_string(),
            other => format!("ERR: unbekannter Befehl {}", other),
        }
    }

    pub fn answer_json(&self, req: &str) -> String {
        let parsed: Value = match serde_json::from_str(req) {
            Ok(v) => v,
            Err(_) => return rpc_error(Value::Null, -32700, "parse error"),
        };
        let id = parsed.get("id").cloned().unwrap_or(Value::Null);
        let method = parsed.get("method").and_then(Value::as_str).unwrap_or("");

        let result = match method {
            "chain_id" => Ok(json!(self.chain_id)),
            "boot_hash" => Ok(json!(self.boot_hash)),
            "peers" => Ok(json!(self.peer_count)),
            "ping" => Ok(json!("pong")),
            "submit_transaction" => self.submit_transaction(&parsed),
            "submit_slashing_evidence" => self.submit_slashing_evidence(&parsed),
            "produce_block" => self.produce_block(&parsed),
            "balance" => self.balance(&parsed),
            "block" => self.block(&parsed),
            "state_root" => self.state_root(&parsed),
            "status" => self.status(),
            other => return rpc_error(id, -32601, &format!("method not found: {other}")),
        };

        match result {
            Ok(value) => json!({"jsonrpc":"2.0","id":id,"result":value}).to_string(),
            Err((code, message)) => rpc_error(id, code, &message),
        }
    }

    fn runtime(&self) -> Result<&Arc<Runtime>, (i64, String)> {
        self.runtime
            .as_ref()
            .ok_or((-32000, "runtime unavailable".into()))
    }

    fn params<'a>(&self, req: &'a Value) -> &'a Value {
        req.get("params").unwrap_or(&Value::Null)
    }

    fn submit_transaction(&self, req: &Value) -> Result<Value, (i64, String)> {
        let p = self.params(req);
        let tx = decode_transaction(p)?;
        let now = tx.timestamp;
        let id = self.runtime()?.node.submit_and_broadcast(tx, now).map_err(|e| (-32001, format!("transaction rejected: {e:?}")))?;
        Ok(json!({"tx_id": hex::encode(id), "status":"accepted"}))
    }

    fn submit_slashing_evidence(&self, req: &Value) -> Result<Value, (i64, String)> {
        let p = self.params(req);
        let evidence = atc_blockchain::consensus::SlashingEvidence {
            validator: string_param(p, "validator")?,
            height: u64_param(p, "height")?,
            block_a: array_param::<32>(p, "block_a")?,
            block_b: array_param::<32>(p, "block_b")?,
            reason: string_param(p, "reason")?,
        };
        let penalty = u64_param(p, "penalty")?;
        let applied = self.runtime()?.node.submit_slashing_evidence(evidence.clone(), penalty)
            .map_err(|e| (-32003, format!("slashing evidence rejected: {e}")))?;
        Ok(json!({"evidence_id": hex::encode(evidence.id()), "validator": evidence.validator, "penalty_applied": applied}))
    }

    fn produce_block(&self, req: &Value) -> Result<Value, (i64, String)> {
        let p = self.params(req);
        let timestamp = p.get("timestamp").and_then(Value::as_u64).ok_or((-32602, "missing params.timestamp".into()))?;
        let max = p.get("max").and_then(Value::as_u64).unwrap_or(100) as usize;
        let block = self.runtime()?.produce(timestamp, max).map_err(|e| (-32002, e))?;
        Ok(json!({
            "height": block.height,
            "block_id": hex::encode(block.id),
            "state_root": hex::encode(block.state_root),
            "tx_count": block.transactions.len()
        }))
    }

    fn balance(&self, req: &Value) -> Result<Value, (i64, String)> {
        let address = self.params(req).get("address").and_then(Value::as_str).ok_or((-32602, "missing params.address".into()))?;
        Ok(json!({"address": address, "balance": self.runtime()?.node.state.balance(address)}))
    }

    fn block(&self, req: &Value) -> Result<Value, (i64, String)> {
        let height = self.params(req).get("height").and_then(Value::as_u64).ok_or((-32602, "missing params.height".into()))?;
        let block = self.runtime()?.node.storage.block(height).ok_or((-32004, "block not found".into()))?;
        Ok(json!({
            "height": block.height,
            "block_id": hex::encode(block.id),
            "parent_hash": hex::encode(block.parent_hash),
            "state_root": hex::encode(block.state_root),
            "tx_root": hex::encode(block.tx_root),
            "tx_count": block.transactions.len()
        }))
    }

    fn status(&self) -> Result<Value, (i64, String)> {
        let runtime = self.runtime()?;
        let node = &runtime.node;
        Ok(json!({
            "chain_id": node.chain_id,
            "height": node.chain.height(),
            "finalized": node.consensus.finalized().map(|(h, id)| json!({
                "height": h,
                "block_id": hex::encode(id)
            })),
            "validators": node.consensus.validators_snapshot(),
            "transport_attached": node.transport_attached()
        }))
    }

    fn state_root(&self, req: &Value) -> Result<Value, (i64, String)> {
        let height = self.params(req).get("height").and_then(Value::as_u64).ok_or((-32602, "missing params.height".into()))?;
        let root = self.runtime()?.node.storage.state_root(height).ok_or((-32004, "state root not found".into()))?;
        Ok(json!({"height": height, "state_root": hex::encode(root)}))
    }
}

fn decode_transaction(params: &Value) -> Result<Transaction, (i64, String)> {
    let tx_type = match params.get("tx_type").and_then(Value::as_u64).ok_or((-32602, "missing params.tx_type".into()))? {
        0 => TxType::Transfer,
        1 => TxType::Stake,
        2 => TxType::Unstake,
        3 => TxType::Contract,
        _ => return Err((-32602, "invalid params.tx_type".into())),
    };
    let chain_id = params.get("chain_id").and_then(Value::as_u64).ok_or((-32602, "missing params.chain_id".into()))?;
    let sender_did = string_param(params, "sender_did")?;
    let recipient_did = params.get("recipient_did").and_then(Value::as_str).map(str::to_owned);
    let amount = u64_param(params, "amount")?;
    let gas_price = u64_param(params, "gas_price")?;
    let gas_limit = u64_param(params, "gas_limit")?;
    let nonce = u64_param(params, "nonce")?;
    let timestamp = u64_param(params, "timestamp")?;
    let payload = hex_param(params, "payload")?;
    let signature = array_param::<64>(params, "signature")?;
    let public_key = array_param::<32>(params, "public_key")?;
    let poh_hash = array_param::<32>(params, "poh_hash")?;

    Ok(Transaction::new_with_chain_id(
        chain_id,
        tx_type,
        sender_did,
        recipient_did,
        amount,
        gas_price,
        gas_limit,
        nonce,
        timestamp,
        payload,
        signature,
        public_key,
        poh_hash,
    ))
}

fn string_param(params: &Value, key: &str) -> Result<String, (i64, String)> {
    params.get(key).and_then(Value::as_str).map(str::to_owned).ok_or((-32602, format!("missing params.{key}")))
}

fn u64_param(params: &Value, key: &str) -> Result<u64, (i64, String)> {
    params.get(key).and_then(Value::as_u64).ok_or((-32602, format!("missing params.{key}")))
}

fn hex_param(params: &Value, key: &str) -> Result<Vec<u8>, (i64, String)> {
    let value = string_param(params, key)?;
    hex::decode(value).map_err(|e| (-32602, format!("invalid hex params.{key}: {e}")))
}

fn array_param<const N: usize>(params: &Value, key: &str) -> Result<[u8; N], (i64, String)> {
    let value = hex_param(params, key)?;
    value.try_into().map_err(|_| (-32602, format!("params.{key} must contain exactly {N} bytes")))
}

fn rpc_error(id: Value, code: i64, message: &str) -> String {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":message}}).to_string()
}

pub fn serve(addr: &str, state: DevnetRpc) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming().flatten() {
        handle(stream, &state)?;
    }
    Ok(())
}

fn handle(stream: TcpStream, state: &DevnetRpc) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let resp = if line.trim().starts_with('{') {
        state.answer_json(&line)
    } else {
        state.answer(&line)
    };
    let mut w = stream;
    w.write_all(resp.as_bytes())?;
    w.write_all(b"\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::{devnet_boot, Genesis};

    fn test_state() -> DevnetRpc {
        let g = Genesis::devnet();
        let (peers, _) =
            devnet_boot(&g, &[(1, "addr1".to_string()), (2, "addr2".to_string())]).unwrap();
        DevnetRpc::from_state(&g, &peers)
    }

    #[test]
    fn rpc_antworten_deterministisch() {
        let rpc = test_state();
        assert_eq!(rpc.answer("CHAIN_ID"), "atc");
        assert_eq!(rpc.answer("PEERS"), "2");
        assert_eq!(rpc.answer("PING"), "PONG");
        assert!(!rpc.answer("BOOT_HASH").is_empty());
    }

    #[test]
    fn jsonrpc_antworten() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"chain_id\",\"id\":7}");
        assert!(r.contains("\"id\":7"));
        assert!(r.contains("\"result\":\"atc\""));
    }

    #[test]
    fn jsonrpc_unbekannte_methode() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"gib_nichts\",\"id\":3}");
        assert!(r.contains("-32601"));
    }
}
