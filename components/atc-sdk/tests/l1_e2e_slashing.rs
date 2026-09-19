use atc_blockchain::{consensus::SlashingEvidence, Node};
use atc_sdk::TransactionBuilder;
use ed25519_dalek::{Signer, SigningKey};

#[test]
fn l1_e2e_slashing_finality_restart_and_continue() {
    let chain_id = 658467;
    let path = std::env::temp_dir().join(format!(
        "atc-l1-e2e-{}-{}.journal",
        std::process::id(),
        chain_id
    ));
    for suffix in ["", "state", "validators", "finality", "slashing", "issuance"] {
        let p = if suffix.is_empty() {
            path.clone()
        } else {
            path.with_extension(suffix)
        };
        let _ = std::fs::remove_file(p);
    }

    let node = Node::open_storage(chain_id, "proposer".into(), &path).unwrap();
    node.state.genesis_credit("alice", 1_000_000).unwrap();
    node.create_genesis(1).unwrap();
    node.register_validator("validator-1".into(), 100).unwrap();

    let key = SigningKey::from_bytes(&[9u8; 32]);
    let tx = TransactionBuilder::transfer(
        chain_id, "alice", "bob", 25, 1, 1000, 0, 2,
    ).sign(&key);
    node.submit(tx, 2).unwrap();
    let block = node.produce(3, 10).unwrap();
    assert_eq!(block.height, 1);
    assert_eq!(node.state.balance("bob"), 25);
    assert_eq!(node.consensus.epoch(block.height), 0);

    let evidence = SlashingEvidence {
        validator: "validator-1".into(),
        height: block.height,
        block_a: [1u8; 32],
        block_b: [2u8; 32],
        reason: "double-sign".into(),
    };
    assert_eq!(node.slash_validator(evidence, 40).unwrap(), 40);
    assert_eq!(node.consensus.validator_stake("validator-1"), 60);

    let mut vote = atc_blockchain::consensus::Vote {
        block: block.id,
        voter: "validator-1".into(),
        approve: true,
        signature: [0; 64],
        public_key: key.verifying_key().to_bytes(),
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"ATC-VOTE-V1");
    bytes.extend_from_slice(&chain_id.to_be_bytes());
    bytes.extend_from_slice(&vote.block);
    bytes.push(1);
    bytes.extend_from_slice(&(vote.voter.len() as u32).to_be_bytes());
    bytes.extend_from_slice(vote.voter.as_bytes());
    vote.signature = key.sign(&bytes).to_bytes();
    node.submit_vote(vote).unwrap();
    assert!(node.finalize_weighted(&block).unwrap());

    drop(node);

    let reopened = Node::open_storage(chain_id, "proposer".into(), &path).unwrap();
    assert_eq!(reopened.chain.height(), 1);
    assert_eq!(reopened.state.balance("bob"), 25);
    assert_eq!(reopened.consensus.validator_stake("validator-1"), 60);
    assert_eq!(reopened.consensus.finalized(), Some((1, block.id)));

    let tx2 = TransactionBuilder::transfer(
        chain_id, "alice", "carol", 10, 1, 1000, 1, 4,
    ).sign(&key);
    reopened.submit(tx2, 4).unwrap();
    let block2 = reopened.produce(5, 10).unwrap();
    assert_eq!(block2.height, 2);
    assert_eq!(reopened.state.balance("carol"), 10);

    for suffix in ["", "state", "validators", "finality", "slashing", "issuance"] {
        let p = if suffix.is_empty() {
            path.clone()
        } else {
            path.with_extension(suffix)
        };
        let _ = std::fs::remove_file(p);
    }
}
