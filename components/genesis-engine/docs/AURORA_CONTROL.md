# Aurora ↔ Genesis Engine Control Protocol

## Purpose

Provide a concrete userspace execution boundary through which Aurora can inspect and mutate the Genesis Engine MVP without importing Genesis internals or granting kernel authority.

## Data flow

Aurora model
  -> ChatTool / AuthorizedTool
  -> GenesisEngineControl
  -> persistent control.py process
  -> Genesis Engine World/ECS

## Security boundary

Only the documented command verbs are accepted. Arguments are validated by the engine process and invalid/unknown commands fail closed. The control process is not part of ShivaCore's trusted computing base.

## Production evolution

The protocol is intentionally small. Future engine backends can implement the same control contract over a local socket or authenticated transport without changing Aurora's tool-level contract.
