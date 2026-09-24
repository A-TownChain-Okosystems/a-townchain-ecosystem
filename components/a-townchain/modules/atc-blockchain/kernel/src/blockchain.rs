            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let key = ed25519_dalek::SigningKey::from_bytes(&[72u8; 32]);

        // Construct the orphan validator journal directly. There is no
        // canonical block journal, so the snapshot has no activation anchor.
        let mut record = Vec::from(b"ATCV2".as_slice());
        record.extend_from_slice(&1u64.to_be_bytes());
        record.extend_from_slice(&1u32.to_be_bytes());
        record.extend_from_slice(&(11u32).to_be_bytes());
        record.extend_from_slice(b"validator-a");
        record.extend_from_slice(&100u64.to_be_bytes());
        record.extend_from_slice(&key.verifying_key().to_bytes());
        std::fs::write(
            path.with_extension("validators"),
            format!("{}\n", hex::encode(record)),
        )
        .unwrap();

        let err = match Node::open_storage(658467, "validator-a".into(), &path) {
            Ok(_) => panic!("orphan validator snapshot must be rejected"),
            Err(err) => err,
        };
        assert!(err.contains("validator snapshot exists without a canonical chain"));

        for suffix in ["", ".state", ".validators", ".finality", ".slashing", ".issuance"] {
            let target = if suffix.is_empty() {
                path.clone()
            } else {
                std::path::PathBuf::from(format!("{}{}", path.display(), suffix))
            };
            let _ = std::fs::remove_file(target);
        }
    }

    #[test]
    fn pending_validator_snapshot_does_not_bypass_wrong_parent_rejection() {
        let node = Node::new(658467, "validator-a".into());
        node.create_genesis_with_proposer(1, "genesis").unwrap();
        node.register_validator("validator-a".into(), 100).unwrap();
        node.register_validator_key(
            "validator-a",
            ed25519_dalek::SigningKey::from_bytes(&[73u8; 32]).verifying_key().to_bytes(),
        ).unwrap();

        let key = ed25519_dalek::SigningKey::from_bytes(&[73u8; 32]);
        let state = node.state.snapshot();
        let wrong_parent = [0xabu8; 32];
        let mut block = Block::new(
            1,
            wrong_parent,
            "validator-a".into(),
            2,