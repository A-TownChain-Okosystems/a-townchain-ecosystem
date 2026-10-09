// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Cross-Repo-Integrationstest (SCR-0111, F-140): der SDK-Client (atc_cli)
//! spricht ueber echtes TCP mit dem ECHTEN atc-node-Code (git-Dependency,
//! rev-gepinnt) — kein Mock. Ehrlichkeit: Devnet-only, localhost, kein TLS.

use atc_cli::rpc_client::RpcClient;
use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::rpc::{serve, DevnetRpc};
use std::time::Duration;

#[test]
fn sdk_spricht_mit_echtem_node() {
    let g = Genesis::devnet();
    let (peers, boot_hash) = devnet_boot(&g, &[(1, "addr1".to_string()), (2, "addr2".to_string())])
        .expect("devnet_boot fehlgeschlagen");
    let state = DevnetRpc::from_state(&g, &peers);

    // Freien Port suchen, dann Node-Dienst darauf starten
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe fehlgeschlagen");
    let port = probe.local_addr().expect("keine Adresse").port();
    drop(probe);
    std::thread::spawn(move || {
        let _ = serve(&format!("127.0.0.1:{}", port), state);
    });

    // KEIN separater TCP-Bereitschafts-Probe: ein connect-and-drop wird von
    // serve() als leere Anfrage gelesen, die Antwort schlaegt auf die
    // geschlossene Verbindung fehl (EPIPE) und beendet den Serve-Thread —
    // danach schlagen alle RPCs fehl (CI-Failure 'RPC nach 5s nicht bereit').
    // Bereitschaft deckt stattdessen mit_retry ab (ConnectionRefused bis der
    // Thread gebunden hat).
    let addr = format!("127.0.0.1:{}", port);

    // Der Serve-Thread ist unmittelbar nach dem TCP-Handshake eventuell
    // noch nicht anfragebereit (unter CI-Last beobachtet: ECONNRESET auf
    // dem ersten RPC-Call). Jeden RPC-Aufruf begrenzt wiederholen.
    fn mit_retry<T>(addr: &str, f: impl Fn(&mut RpcClient) -> Result<T, String>) -> T {
        for _ in 0..50 {
            let mut c = RpcClient::new(addr.to_string());
            match f(&mut c) {
                Ok(v) => return v,
                Err(_) => std::thread::sleep(Duration::from_millis(100)),
            }
        }
        panic!("RPC nach 5s nicht bereit");
    }

    assert_eq!(
        mit_retry(&addr, |c| c.chain_id()),
        658467,
        "SDK-Client muss Chain-ID des echten Nodes lesen"
    );
    assert_eq!(mit_retry(&addr, |c| c.peers()), 2);
    // Kerninvariante: Boot-Hash aus der Node-Genesis == was der Client ueber RPC erhaelt
    assert_eq!(
        mit_retry(&addr, |c| c.boot_hash()),
        boot_hash,
        "Boot-Hash muss ueber RPC identisch sein"
    );
}
