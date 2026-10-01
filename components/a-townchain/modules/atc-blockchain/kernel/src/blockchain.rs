        let votes = self.take_pending_votes(&block.id);
        for vote in votes {
            let _ = self.submit_vote(vote);
        }
        if self.consensus.weighted_finality_at_height(&block.id, block.height) {
            let _ = self.finalize_weighted(block)?;
        }
        Ok(())
    }

    fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
        if let Some(transport) = self
            .transport
            .lock()
            .map_err(|_| "transport lock poisoned")?
            .clone()
        {
            transport.broadcast(message)?;
        }
        Ok(())
    }

    /// Apply a network block through the same deterministic state transition
    /// rules used by local block production, then persist it.
    pub fn import_block(&self, b: Block) -> Result<(), String> {
        self.import_block_internal(b, None)
    }

    fn import_block_internal(
        &self,
        b: Block,
        sync_snapshot: Option<(u64, BTreeMap<String, u128>, BTreeMap<String, [u8; 32]>)>,
    ) -> Result<(), String> {
        if self.chain_id
            != b.transactions
                .first()
                .map(|t| t.chain_id)
                .unwrap_or(self.chain_id)
        {
            return Err("block transaction chain-id mismatch".into());
        }
        if let Some((finalized_height, finalized_id)) = self.consensus.finalized() {
            if b.height <= finalized_height {
                return Err("cannot replace finalized block".into());
            }
            if b.height == finalized_height.saturating_add(1) && b.parent_hash != finalized_id {
                return Err("candidate would reorg finalized prefix".into());
            }
        }
        let parent = self.chain.last().ok_or("genesis required")?;
        let finalized_height = self.consensus.finalized().map(|(height, _)| height);
        let selected = fork_choice::choose(&parent, &b, finalized_height)
            .map_err(|_| "fork-choice finality violation")?;
        if selected.id != b.id {
            return Err("candidate rejected by deterministic fork-choice".into());
        }
        self.chain.validate_append(&b)?;

        if let Some((activation_height, validators, keys)) = sync_snapshot.as_ref() {
            if *activation_height > b.height || *activation_height > self.chain.height().saturating_add(1) {
                return Err("validator snapshot activation is outside the synchronization boundary".into());
            }
            if self.consensus.has_validator_snapshot(*activation_height) {
                let existing = self.consensus.validator_snapshot_for_height(*activation_height)
                    .ok_or("existing validator snapshot is unavailable")?;
                if existing.0 != *validators || existing.1 != *keys {
                    return Err("conflicting validator snapshot at activation height".into());
                }
            }
            if let Some(existing_for_block) = self.consensus.validator_snapshot_for_height(b.height) {
                if existing_for_block.0 != *validators || existing_for_block.1 != *keys {
                    return Err("synchronized block conflicts with local validator history".into());
                }
            }
            consensus::ConsensusEngine::validator_snapshot_commitment_from(validators, keys)
                .ok_or("invalid validator snapshot identity set")?;
        }

        if b.height > 0 {
            let snapshot_keys;
            let keys = if let Some((_, _, keys)) = sync_snapshot.as_ref() {
                keys
            } else {
                snapshot_keys = self.consensus
                    .validator_snapshot_for_height(b.height)
                    .ok_or("validator snapshot is unavailable for block height")?;
                &snapshot_keys.1
            };
            let public_key = keys
                .get(&b.proposer)
                .copied()
                .ok_or("block proposer has no signing key at block height")?;
            let key = ed25519_dalek::VerifyingKey::from_bytes(&public_key)
                .map_err(|_| "invalid proposer public key")?;
            key.verify(
                &block_signing_bytes(self.chain_id, &b),
                &ed25519_dalek::Signature::from_bytes(&b.signature),
            ).map_err(|_| "invalid block proposer signature")?;
        }

        let parent = self.chain.last().ok_or("genesis required")?;
        if b.parent_hash != parent.id || b.height != parent.height.saturating_add(1) {
            return Err("block is not the next canonical height".into());
        }

        let state_snapshot = self.state.snapshot();
        let dao_snapshot = self.state.dao_snapshot();
        let issued_snapshot = self.state.issued_base_units();

        let exec = AtcVmExecutor {
            protocol: "1.0.0".into(),
            vm_version: "1.0.0".into(),
            genesis_id: hex::encode(parent.id),
        };
        let mut receipts = Vec::new();
        for tx in &b.transactions {
            if tx.chain_id != self.chain_id
                || !self
                    .verifier
                    .verify(&signing_bytes(tx), &tx.signature, &tx.public_key)
            {
                self.state.restore(state_snapshot);
                let _ = self.state.restore_dao(&dao_snapshot);
                let _ = self.state.restore_issued_base_units(issued_snapshot);
                return Err("invalid transaction signature or chain".into());
            }
            receipts.push(
                exec.execute(tx, self.state.root())
                    .map_err(|e| e.to_string())?,
            );
        }

        self.state
            .apply_batch(&b.transactions)
            .map_err(|e| format!("state transition: {e:?}"))?;

        for tx in &b.transactions {
            if !tx.payload.is_empty() {
                if let Err(e) = self
                    .state
                    .apply_dao_payload(&tx.payload, b.height, &tx.sender_did)
                {
                    self.state.restore(state_snapshot);
                    let _ = self.state.restore_dao(&dao_snapshot);
                    let _ = self.state.restore_issued_base_units(issued_snapshot);
                    return Err(format!("DAO transition: {e}"));
                }
            }
        }

        if let Err(e) = self.state.apply_block_reward(b.height, &b.proposer) {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(format!("block reward: {e}"));
        }

        let root = self.state.root();
        let expected_state_root = committed_state_root(
            root,
            b.height,
            if let Some((_, validators, keys)) = sync_snapshot.as_ref() {
                consensus::ConsensusEngine::validator_snapshot_commitment_from(validators, keys)
            } else {
                self.consensus.validator_snapshot_commitment(b.height)
            },
        )?;
        if expected_state_root != b.state_root || receipts::root(&receipts) != b.receipt_root {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err("network block state/receipt root mismatch".into());
        }

        let storage_result = if let Some((activation_height, validators, keys)) = sync_snapshot.as_ref() {
            self.storage.commit_block_state_issuance_with_validator_snapshot(
                b.clone(),
                &self.state.snapshot(),
                &self.state.dao_snapshot(),
                self.state.issued_base_units(),
                *activation_height,
                validators,
                keys,
            )
        } else {
            self.storage.commit_block_state_issuance(
                b.clone(),
                &self.state.snapshot(),
                &self.state.dao_snapshot(),
                self.state.issued_base_units(),
            )
        };
        if let Err(e) = storage_result {
            self.state.restore(state_snapshot);
            let _ = self.state.restore_dao(&dao_snapshot);
            let _ = self.state.restore_issued_base_units(issued_snapshot);
            return Err(e);
        }
        self.chain.append(b.clone())?;
        if let Some((activation_height, validators, keys)) = sync_snapshot {
            self.consensus.restore_validator_snapshot(activation_height, validators, keys)?;
        }
        for tx in &b.transactions {
            self.pool.mark_in_block(&tx.id);
        }
        self.consensus.set_height(b.height);
        self.replay_pending_votes(&b)?;
        self.vote_for_block(&b)?;
        Ok(())
    }

    /// Feed one decoded network message into the canonical Node.
    pub fn handle_network_message(&self, message: NetworkMessage) -> Result<(), String> {
        self.handle_network_message_from_peer(message, "")
    }

    fn handle_network_message_from_peer(&self, message: NetworkMessage, peer_id: &str) -> Result<(), String> {
        let _apply_guard = self.network_apply_lock.lock().map_err(|_| "network apply lock poisoned")?;
        match message {
            NetworkMessage::Block(b) => self.import_block(b),
            NetworkMessage::BlockWithValidatorSnapshot { block, activation_height, validators, validator_keys } => {