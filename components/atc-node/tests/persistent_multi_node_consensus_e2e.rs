use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn rpc(addr: &str, method: &str) -> serde_json::Value {
    let mut stream = TcpStream::connect(addr).expect("rpc connect");
    let body = format!(r#"{{"jsonrpc":"2.0","method":"{method}","id":1}}"#);
    stream.write_all(body.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();
    serde_json::from_str(out.trim()).unwrap()
}

fn wait_status(addr: &str, min_height: u64) -> serde_json::Value {
    for _ in 0..80 {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            let body = r#"{"jsonrpc":"2.0","method":"status","id":1}"#;
            let _ = stream.write_all(body.as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));
            let mut out = String::new();
            if stream.read_to_string(&mut out).is_ok() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(out.trim()) {
                    if v.get("result").and_then(|x| x.get("height")).and_then(|x| x.as_u64()).unwrap_or(0) >= min_height {
                        return v;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(250));
    }
    panic!("node did not reach height {min_height}: {addr}");
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
        "atc-node-a:1:{}{},atc-node-b:1:{}{}",
        "01".repeat(32),
        "",
        "02".repeat(32),
        ""
    );

    let mut a = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-a")
        .env("ATC_RPC_ADDR", a_rpc)
        .env("ATC_P2P_ADDR", a_p2p)
        .env("ATC_DATA_DIR", &a_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", "")
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let mut b = Command::new(env!("CARGO_BIN_EXE_atc-node"))
        .env("ATC_NODE_ID", "atc-node-b")
        .env("ATC_RPC_ADDR", b_rpc)
        .env("ATC_P2P_ADDR", b_p2p)
        .env("ATC_DATA_DIR", &b_dir)
        .env("ATC_VALIDATORS", &validators)
        .env("ATC_PEERS", a_p2p)
        .env("ATC_BLOCK_INTERVAL_SECS", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let a_status = wait_status(a_rpc, 2);
    let b_status = wait_status(b_rpc, 2);

    assert_eq!(
        a_status["result"]["height"].as_u64(),
        b_status["result"]["height"].as_u64()
    );
    assert!(
        a_status["result"]["finalized"]["height"].as_u64().unwrap_or(0) >= 1,
        "node A did not reach weighted finality"
    );
    assert!(
        b_status["result"]["finalized"]["height"].as_u64().unwrap_or(0) >= 1,
        "node B did not reach weighted finality"
    );

    let _ = rpc(a_rpc, "ping");
    let _ = a.kill();
    let _ = b.kill();
    let _ = a.wait();
    let _ = b.wait();
    std::fs::remove_dir_all(root).unwrap();
}
