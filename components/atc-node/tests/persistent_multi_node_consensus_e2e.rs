use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn rpc(addr: &str, method: &str) -> serde_json::Value {
    rpc_request(addr, serde_json::json!({"jsonrpc":"2.0","method":method,"id":1}))
}

fn rpc_request(addr: &str, request: serde_json::Value) -> serde_json::Value {
    let mut stream = TcpStream::connect(addr).expect("rpc connect");
    let body = serde_json::to_string(&request).unwrap();
    stream.write_all(body.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();
    serde_json::from_str(out.trim()).unwrap()
}

fn wait_status(addr: &str, min_height: u64) -> serde_json::Value {
    for _ in 0..120 {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let body = r#"{"jsonrpc":"2.0","method":"status","id":1}"#;
            let _ = stream.write_all(body.as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
            let mut out = String::new();
            if stream.read_to_string(&mut out).is_ok() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(out.trim()) {
                    if v.get("result")
                        .and_then(|x| x.get("height"))
                        .and_then(|x| x.as_u64())
                        .unwrap_or(0)
                        >= min_height
                    {
                        return v;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
    panic!("node did not reach height {min_height}: {addr}");
}

fn spawn_node(
    node_id: &str,
    rpc_addr: &str,
    p2p_addr: &str,
    data_dir: &PathBuf,
    validators: &str,
    peers: &str,
) -> Child {
    Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", node_id)
        .env("ATC_RPC_ADDR", rpc_addr)
        .env("ATC_P2P_ADDR", p2p_addr)
        .env("ATC_DATA_DIR", data_dir)
        .env("ATC_VALIDATORS", validators)
        .env("ATC_PEERS", peers)
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
}

#[test]
fn persistent_two_node_consensus_path() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("atc-node-network-{now}"));
    let a_dir = root.join("a");
    let b_dir = root.join("b");
    std::fs::create_dir_all(&a_dir).unwrap();
    std::fs::create_dir_all(&b_dir).unwrap();

    let a_rpc = "127.0.0.1:40171";
    let a_p2p = "127.0.0.1:40172";
    let b_rpc = "127.0.0.1:40173";
    let b_p2p = "127.0.0.1:40174";
    let validators = format!(
        "atc-node-a:1:{},atc-node-b:1:{}",
        "01".repeat(32),
        "02".repeat(32)
    );

    let mut a = spawn_node(
        "atc-node-a",
        a_rpc,
        a_p2p,
        &a_dir,
        &validators,
        "",
    );
    let mut b = spawn_node(
        "atc-node-b",
        b_rpc,
        b_p2p,
        &b_dir,
        &validators,
        a_p2p,
    );

    let a_status = wait_status(a_rpc, 2);
    let b_status = wait_status(b_rpc, 2);
    let height = a_status["result"]["height"].as_u64().unwrap();
    assert_eq!(Some(height), b_status["result"]["height"].as_u64());
    assert!(height >= 2);
    assert!(
        a_status["result"]["finalized"]["height"].as_u64().unwrap_or(0) >= 1,
        "node A did not reach weighted finality"
    );
    assert!(
        b_status["result"]["finalized"]["height"].as_u64().unwrap_or(0) >= 1,
        "node B did not reach weighted finality"
    );

    let root_a = rpc_request(
        a_rpc,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"state_root",
            "params":{"height":height},
            "id":2
        }),
    )["result"]["state_root"]
        .as_str()
        .unwrap()
        .to_owned();
    let root_b = rpc_request(
        b_rpc,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"state_root",
            "params":{"height":height},
            "id":2
        }),
    )["result"]["state_root"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(root_a, root_b, "nodes diverged at persisted state root");

    let slash = rpc_request(
        a_rpc,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"submit_slashing_evidence",
            "params":{
                "validator":"atc-node-b",
                "height":height,
                "block_a":"01".repeat(32),
                "block_b":"02".repeat(32),
                "reason":"double-sign",
                "penalty":1
            },
            "id":4
        }),
    );
    assert_eq!(slash["result"]["penalty_applied"].as_u64(), Some(1));

    for _ in 0..40 {
        let status_b = rpc(b_rpc, "status");
        if status_b["result"]["validators"]["atc-node-b"].is_null() {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    let status_a = rpc(a_rpc, "status");
    let status_b = rpc(b_rpc, "status");
    assert_eq!(status_a["result"]["validators"]["atc-node-b"], serde_json::Value::Null);
    assert_eq!(status_b["result"]["validators"]["atc-node-b"], serde_json::Value::Null);

    let _ = a.kill();
    let _ = b.kill();
    let _ = a.wait();
    let _ = b.wait();
    thread::sleep(Duration::from_millis(250));

    a = spawn_node(
        "atc-node-a",
        a_rpc,
        a_p2p,
        &a_dir,
        &validators,
        "",
    );
    b = spawn_node(
        "atc-node-b",
        b_rpc,
        b_p2p,
        &b_dir,
        &validators,
        a_p2p,
    );

    let recovered_a = wait_status(a_rpc, height);
    let recovered_b = wait_status(b_rpc, height);
    assert_eq!(
        recovered_a["result"]["height"].as_u64(),
        Some(height),
        "node A did not recover its durable height"
    );
    assert_eq!(
        recovered_b["result"]["height"].as_u64(),
        Some(height),
        "node B did not recover its durable height"
    );

    let recovered_root_a = rpc_request(
        a_rpc,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"state_root",
            "params":{"height":height},
            "id":3
        }),
    )["result"]["state_root"]
        .as_str()
        .unwrap()
        .to_owned();
    let recovered_root_b = rpc_request(
        b_rpc,
        serde_json::json!({
            "jsonrpc":"2.0",
            "method":"state_root",
            "params":{"height":height},
            "id":3
        }),
    )["result"]["state_root"]
        .as_str()
        .unwrap()
        .to_owned();

    assert_eq!(recovered_root_a, root_a, "node A changed state after restart");
    assert_eq!(recovered_root_b, root_b, "node B changed state after restart");
    assert_eq!(
        recovered_root_a, recovered_root_b,
        "restarted nodes diverged in recovered state"
    );

    let next_a = wait_status(a_rpc, height + 1);
    let next_b = wait_status(b_rpc, height + 1);
    assert_eq!(
        next_a["result"]["height"].as_u64(),
        next_b["result"]["height"].as_u64(),
        "nodes did not reconverge after restart"
    );
    assert!(
        next_a["result"]["height"].as_u64().unwrap() > height,
        "no post-restart block was produced"
    );

    let _ = a.kill();
    let _ = b.kill();
    let _ = a.wait();
    let _ = b.wait();
    std::fs::remove_dir_all(root).unwrap();
}
