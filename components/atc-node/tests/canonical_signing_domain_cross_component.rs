use atc_blockchain::{
    crypto::{signing_bytes, Ed25519Verifier, SignatureVerifier},
    Node,
};
use atc_sdk::TransactionBuilder;
use atc_wallet::{
    keys::WalletKey,
    tx::{Transaction as WalletTransaction, TxType as WalletTxType},
};
use ed25519_dalek::SigningKey;

const CHAIN_ID: u64 = 658467;

fn wallet_tx() -> WalletTransaction {
    WalletTransaction {
        chain_id: CHAIN_ID,
        tx_type: WalletTxType::Transfer,
        sender_did: "alice".into(),
        recipient_did: Some("bob".into()),
        amount: 100,
        gas_price: 1,
        gas_limit: 2_000,
        nonce: 0,
        timestamp: 1,
        payload: b"cross-component".to_vec(),
        poh_hash: [7u8; 32],
    }
}

#[test]
fn wallet_sdk_and_blockchain_use_identical_l1_signing_bytes() {
    let wallet_key = WalletKey::from_seed([9u8; 32]);
    let sdk_key = SigningKey::from_bytes(&[9u8; 32]);

    let wallet = wallet_tx();
    let wallet_bytes = wallet.signing_bytes().expect("wallet signing bytes");

    let sdk = TransactionBuilder::transfer(
        CHAIN_ID,
        "alice",
        "bob",
        100,
        1,
        2_000,
        0,
        1,
    )
    .payload(b"cross-component".to_vec())
    .poh_hash([7u8; 32])
    .sign(&sdk_key);
    let sdk_bytes = signing_bytes(&sdk);

    assert_eq!(wallet_bytes, sdk_bytes);
    assert_eq!(wallet.sign(&wallet_key).unwrap(), sdk.signature);

    let verifier = Ed25519Verifier;
    assert!(verifier.verify(
        &wallet_bytes,
        &sdk.signature,
        &sdk.public_key,
    ));

    let node = Node::new(CHAIN_ID, "validator-1".into());
    node.create_genesis(0).expect("genesis");
    let wallet_signature = wallet.sign(&wallet_key).expect("wallet signature");
    let blockchain_tx = atc_blockchain::mempool::Transaction::new_with_chain_id(
        CHAIN_ID,
        atc_blockchain::mempool::TxType::Transfer,
        "alice".into(),
        Some("bob".into()),
        100,
        1,
        2_000,
        0,
        1,
        b"cross-component".to_vec(),
        wallet_signature,
        wallet_key.public_key(),
        [7u8; 32],
    );

    node.submit(blockchain_tx, 0)
        .expect("blockchain must accept wallet signature");
    let block = node.produce(2, 100).expect("block production");
    assert_eq!(block.transactions.len(), 1);
}
