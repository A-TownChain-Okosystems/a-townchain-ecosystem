// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0

use atc_wallet::{
    address,
    keys::WalletKey,
    tx::{Transaction, TxType},
};

fn main() {
    let key = WalletKey::generate().expect("OS CSPRNG unavailable");
    let public_key = key.public_key();

    println!("atc-wallet — A-TownChain Rust core");
    println!("public_key={}", hex(&public_key));

    // Example only: concrete network version bytes remain configuration-driven
    // until ATC-NETWORK-ID-001 is frozen.
    let address = address::encode(&public_key, 42);
    println!("example_address={address}");

    let tx = Transaction {
        chain_id: 1,
        tx_type: TxType::Transfer,
        sender_did: address.clone(),
        recipient_did: Some("ATC-example-recipient".into()),
        amount: 1,
        gas_price: 1,
        gas_limit: 1000,
        nonce: 0,
        timestamp: 0,
        payload: Vec::new(),
        poh_hash: [0u8; 32],
    };

    let signature = tx.sign(&key).expect("transaction signing failed");
    tx.verify(&public_key, &signature)
        .expect("self verification failed");

    println!("signature={}", hex(&signature));
    println!("address={address}");
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}
