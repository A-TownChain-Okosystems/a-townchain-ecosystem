use atc_blockchain::{network::{read_message, TcpPeerTransport, NetworkMessage}, Block};
use std::{
    net::TcpListener,
    process::Command,
    thread,
    time::Duration,
};

#[test]
fn multi_process_tcp_handshake_and_block_transfer() {
    if std::env::var("ATC_TCP_CHILD").ok().as_deref() == Some("1") {
        let addr = std::env::var("ATC_TCP_ADDR").expect("ATC_TCP_ADDR");
        let transport = TcpPeerTransport::new(658467, "node-b");
        let mut stream = transport.connect_stream(&addr, 0, [0; 32]).expect("TCP handshake");
        let message = read_message(&mut stream).expect("read block").expect("peer closed");
        match message {
            NetworkMessage::Block(block) => {
                assert_eq!(block.height, 2);
                assert_eq!(block.parent_hash, [1; 32]);
                assert_eq!(block.state_root, [2; 32]);
            }
            other => panic!("expected complete Block transfer, got {other:?}"),
        }
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let addr = listener.local_addr().expect("listener address");

    let exe = std::env::current_exe().expect("current test executable");
    let mut child = Command::new(exe)
        .arg("--exact")
        .arg("multi_process_tcp_handshake_and_block_transfer")
        .arg("--nocapture")
        .env("ATC_TCP_CHILD", "1")
        .env("ATC_TCP_ADDR", addr.to_string())
        .spawn()
        .expect("spawn second node process");

    let (mut stream, node_id, height, best) =
        TcpPeerTransport::accept(&listener, 658467, "node-a", 1, [1; 32])
            .expect("accept peer handshake");

    assert_eq!(node_id, "node-b");
    assert_eq!(height, 0);
    assert_eq!(best, [0; 32]);

    let block = Block::new(
        2,
        [1; 32],
        "node-a".into(),
        2,
        Vec::new(),
        [2; 32],
        [0; 32],
        [0; 64],
    );
    atc_blockchain::network::write_message(&mut stream, &NetworkMessage::Block(block.clone()))
        .expect("send block");

    let status = child.wait().expect("child wait");
    assert!(status.success(), "peer process exited with {status}");
}
