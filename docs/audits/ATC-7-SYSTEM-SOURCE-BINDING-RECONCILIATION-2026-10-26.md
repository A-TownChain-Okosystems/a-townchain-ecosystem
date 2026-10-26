# ATC-7-System Source-Binding Reconciliation — Initial Symbol Review

**Status:** `RESIDUAL / BLOCKED`  
**Review snapshot:** 2026-10-26  
**Scope:** initial source-level review of the ten `BLOB_DRIFT` rows recorded in `ATC-7-SYSTEM-LIVE-CENSUS-2026-10-26.md`.  
**Important:** this is not a complete call-graph review, not a full symbol census, and not test/security verification. No catalog binding is changed by this report.

## Method

For each drift row, the canonical repository was read at the exact source commit recorded by the live census. The current file blob SHA and the catalogued symbol/signature were inspected. A matching symbol name and compatible signature are evidence of candidate identity, but not by themselves proof of all call relationships or full behavioral equivalence. Consequently, source-binding refresh remains pending where call sites and semantic deltas have not yet been fully reviewed.

## Initial findings

| Contract | Canonical source at pinned commit | Current blob SHA | Direct source observation | Reconciliation decision |
|---|---|---|---|---|
| `FN-LANG-001` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:crates/atc-core/src/parser.rs` | `50e6ff7f1c9db8de214b9bf5155368b516c920bc` | `pub fn parse_program(src: &str) -> Result<Program, ParseError>` exists; implementation tokenizes source and invokes the parser. Responsibility matches parsing/diagnostics. | Candidate identity confirmed; caller/semantic-delta review remains open. Do not refresh binding yet. |
| `FN-LANG-002` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:crates/atc-core/src/native.rs` | `b3d42b9d2df2fceab055adf70c52cbb71dadd737` | `pub fn parse_native_contract(src: &str) -> Result<NativeContract, NativeParseError>` exists. File describes native contract syntax/AST recognition; lowering remains the existing ATCLang path. | Candidate identity confirmed; validate downstream use and contract semantics before binding refresh. |
| `FN-LANG-003` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:crates/atc-core/src/lower.rs` | `686239c12529f01c25f8042a3ca0afb4174d6131` | `pub fn lower_program(prog: &Program) -> Result<CompiledProgram, LowerError>` exists. The module documents AST-to-bytecode lowering and verification before returning compiled output. | Candidate identity confirmed; caller/semantic-delta review remains open. |
| `FN-LANG-004` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:crates/atc-core/src/bytecode.rs` | `8c4419edbaff6ea5fad5cf486a70e990c7a8fb9f` | Bytecode verifier method `verify(&self, local_count: u16, function_count: u16) -> Result<(), VerifyError>` exists. File documents canonical bytecode encoding and path-sensitive validation. | Candidate identity confirmed at file/method level; call sites and verifier behavior changes still need review. |
| `FN-LANG-005` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:crates/atc-core/src/artifact.rs` | `8da375daf60ba8317a1558824f3c5fcb7f41d351` | `pub fn decode_and_verify(bytes: &[u8]) -> Result<VerifiedArtifact, ArtifactError>` exists. It rejects oversized input and checks the ATCA envelope before constructing the verified wrapper. | Candidate identity confirmed; complete compatibility/error-path and caller review remains open. |
| `FN-LANG-010` | `atclang@d4753fa298aa52e0130af0ae5eb046cde7bbad64:tools/determinism_check.py` | `47cc23f72ba91c9095febcad064df17dd7c81f8a` | `def main():` exists. The current module documents token-aware scanning, allowlist handling, self-exclusion, and fail-closed checks for nondeterministic constructs and repeatable output. | Entrypoint identity confirmed; invocation paths, gate wiring and behavioral delta require review. |
| `FN-SHIVA-001` | `globus-os@e6ab916e1af766b0a15c6b997f7dfa9eb2cbe1b7:modules/atc-shivacore/kernel/src/capability.rs` | `a29b963a1a7ba6cb4535f63b8e4ca3657d573568` | `check(&self, pid: Pid, resource_type: ResourceType, resource_id: u64, required: Rights) -> bool` exists. The documented responsibility is checking process capability rights for a resource. | Candidate identity confirmed; inspect callers and authorization/fail-closed semantics before binding refresh. |
| `FN-AURORA-001` | `aurora-ai@17e4e718ca66858457ed9861681a35d7ed6296e6:modules/atc-aurora-core/src/aurora_core.rs` | `1c555b51283e49541f69ec11564673cd5f5df776` | `process_request(&self, agent_id: &str, prompt: &str) -> Result<String, String>` exists. The implementation checks active state and retrieves an agent from the registry. | Candidate identity confirmed; trace the full policy/authority path and callers before binding refresh. |
| `FN-AURORA-002` | `aurora-ai@17e4e718ca66858457ed9861681a35d7ed6296e6:modules/atc-aurora-agents/src/agent_pool.rs` | `d2100fb930774af7b63c51705a801f22de4ca010` | `execute(&self, agent_id: &str, ctx: AgentContext) -> Result<AgentResponse, String>` exists. The function resolves an agent ID and delegates to that agent's `execute`; the inspected method itself does not show a policy gate. | Symbol identity confirmed, but the catalog's policy-gated responsibility is not proven by this method alone. Inspect wrappers/callers and retain RESIDUAL until authorization is evidenced. |
| `FN-AURORA-003` | `aurora-ai@17e4e718ca66858457ed9861681a35d7ed6296e6:modules/atc-aurora-core/src/agent_registry.rs` | `3274f77ec7be74e249e90e122091a4621d0f18e6` | `register(&mut self, id: &str, name: &str, role: &str, caps: Vec<String>) -> Result<(), RegistryError>` exists. The module describes stable IDs and duplicate protection. | Candidate identity confirmed; swarm coordination/result validation is broader than this registration method. Inspect callers and separate contract scope before binding refresh. |

## Unbound contracts

### `FN-GLOBUS-003` — OS service boundary and platform integration

The contract is explicitly recorded as `PROPOSED_CONTRACT_ONLY` with source path `UNMAPPED`. The canonical GlobusOS documentation describes user-space service responsibilities and platform integration, but a broad architectural boundary is not equivalent to one function-level implementation. No single function is asserted here. Keep `RESIDUAL / UNBOUND_RESIDUAL` until the contract is decomposed into source-backed interface(s), or an owner explicitly defers/excludes it with rationale.

### `FN-AURORA-004` — Policy-gated Neural AI orchestration

The contract is explicitly recorded as `PROPOSED_CONTRACT_ONLY` with source path `UNMAPPED`. Existing source contains candidate building blocks such as `AuroraCore::process_request`, `AgentPool::execute`, and agent-registry operations, but the reviewed evidence does not yet establish one complete policy-gated orchestration path. In particular, `AgentPool::execute` delegates directly to the agent and does not itself establish a policy gate. Do not bind this contract to a guessed path or single symbol. Keep `RESIDUAL / UNBOUND_RESIDUAL` until the authority/policy chain and callers are traced, or the contract is explicitly decomposed/deferred by its owner.

## Remaining gates

1. Inspect diffs and history for each changed blob; trace direct and indirect callers and relevant tests.
2. Record signatures, visibility, cfg/feature/target conditions and behavioral changes for each contract.
3. Resolve both unbound contracts through source-backed decomposition or explicit owner-approved deferral/exclusion.
4. Run a complete Rust/Python/TypeScript symbol census and resolve Cargo workspace membership from manifests.
5. Bind test/security evidence to exact source SHA and record Actions Run → Job → Step, terminal conclusion, logs/artifacts and artifact SHA-256.
6. Keep release status `RESIDUAL / BLOCKED`. Do not mark the catalog `VERIFIED` based on symbol presence or blob identity alone.
