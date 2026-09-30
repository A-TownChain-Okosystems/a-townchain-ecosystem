# Integration Matrix — A-TownChain Ecosystem

**Document ID:** `ATC-INTEGRATION-MATRIX-001`  
**Status:** `ARCHITECTURE CONTRACT`  
**Integration count:** 212

## Purpose
This matrix defines connections between canonical subsystems, APIs, policy/capability boundaries and authoritative systems. Integration is the layer between component/API contracts and the integrated system.

## Canonical full-system path
`User → Frontend/Browser → Identity/Session → API/RPC → Aurora/Wallet/OS → Capability → Policy → Approval → Orchestrator → Domain Service → A-TownChain/GlobusOS/ShivaCore → Authoritative State → Indexer/Applications`

## Integration matrix
| ID | Domain | Integration | Contract |
|---|---|---|---|
| INT-001 | Integration Core | Identity Integration | Interface/contract boundary |
| INT-002 | Integration Core | Authentication Integration | Interface/contract boundary |
| INT-003 | Integration Core | Authorization Integration | Interface/contract boundary |
| INT-004 | Integration Core | Capability Integration | Interface/contract boundary |
| INT-005 | Integration Core | Approval Integration | Interface/contract boundary |
| INT-006 | Integration Core | Event Integration | Interface/contract boundary |
| INT-007 | Integration Core | Audit Integration | Interface/contract boundary |
| INT-008 | Integration Core | Configuration Integration | Interface/contract boundary |
| INT-009 | Integration Core | Observability Integration | Interface/contract boundary |
| INT-010 | Integration Core | Recovery Integration | Interface/contract boundary |
| INT-011 | Aurora ↔ GlobusOS | Aurora OS Gateway | Interface/contract boundary |
| INT-012 | Aurora ↔ GlobusOS | AI Capability | Interface/contract boundary |
| INT-013 | Aurora ↔ GlobusOS | AI Policy | Interface/contract boundary |
| INT-014 | Aurora ↔ GlobusOS | AI Approval | Interface/contract boundary |
| INT-015 | Aurora ↔ GlobusOS | Process Integration | Interface/contract boundary |
| INT-016 | Aurora ↔ GlobusOS | Filesystem Integration | Interface/contract boundary |
| INT-017 | Aurora ↔ GlobusOS | Network Integration | Interface/contract boundary |
| INT-018 | Aurora ↔ GlobusOS | Device Integration | Interface/contract boundary |
| INT-019 | Aurora ↔ GlobusOS | Resource Integration | Interface/contract boundary |
| INT-020 | Aurora ↔ GlobusOS | System Management | Interface/contract boundary |
| INT-021 | Aurora ↔ GlobusOS | OS Events | Interface/contract boundary |
| INT-022 | Aurora ↔ GlobusOS | OS Audit | Interface/contract boundary |
| INT-023 | GlobusOS ↔ ShivaCore | Kernel API | Interface/contract boundary |
| INT-024 | GlobusOS ↔ ShivaCore | Capability Enforcement | Interface/contract boundary |
| INT-025 | GlobusOS ↔ ShivaCore | Secure IPC | Interface/contract boundary |
| INT-026 | GlobusOS ↔ ShivaCore | Process Isolation | Interface/contract boundary |
| INT-027 | GlobusOS ↔ ShivaCore | Resource Authority | Interface/contract boundary |
| INT-028 | GlobusOS ↔ ShivaCore | Memory Protection | Interface/contract boundary |
| INT-029 | GlobusOS ↔ ShivaCore | Security Monitor | Interface/contract boundary |
| INT-030 | GlobusOS ↔ ShivaCore | Kernel Audit | Interface/contract boundary |
| INT-031 | GlobusOS ↔ ShivaCore | Hardware Authority | Interface/contract boundary |
| INT-032 | GlobusOS ↔ ShivaCore | Secure Context | Interface/contract boundary |
| INT-033 | ShivaCore ↔ Hardware | CPU Integration | Interface/contract boundary |
| INT-034 | ShivaCore ↔ Hardware | Memory Integration | Interface/contract boundary |
| INT-035 | ShivaCore ↔ Hardware | Interrupt Integration | Interface/contract boundary |
| INT-036 | ShivaCore ↔ Hardware | Timer Integration | Interface/contract boundary |
| INT-037 | ShivaCore ↔ Hardware | Storage Integration | Interface/contract boundary |
| INT-038 | ShivaCore ↔ Hardware | Network Integration | Interface/contract boundary |
| INT-039 | ShivaCore ↔ Hardware | GPU Integration | Interface/contract boundary |
| INT-040 | ShivaCore ↔ Hardware | NPU Integration | Interface/contract boundary |
| INT-041 | ShivaCore ↔ Hardware | TPM/TEE Integration | Interface/contract boundary |
| INT-042 | ShivaCore ↔ Hardware | Secure Boot Integration | Interface/contract boundary |
| INT-043 | Aurora ↔ A-TownChain | Blockchain Gateway | Interface/contract boundary |
| INT-044 | Aurora ↔ A-TownChain | Blockchain Query | Interface/contract boundary |
| INT-045 | Aurora ↔ A-TownChain | Transaction Construction | Interface/contract boundary |
| INT-046 | Aurora ↔ A-TownChain | Transaction Simulation | Interface/contract boundary |
| INT-047 | Aurora ↔ A-TownChain | Signing Request | Interface/contract boundary |
| INT-048 | Aurora ↔ A-TownChain | Explicit Approval | Interface/contract boundary |
| INT-049 | Aurora ↔ A-TownChain | Wallet Integration | Interface/contract boundary |
| INT-050 | Aurora ↔ A-TownChain | Contract Integration | Interface/contract boundary |
| INT-051 | Aurora ↔ A-TownChain | Event Integration | Interface/contract boundary |
| INT-052 | Aurora ↔ A-TownChain | Governance Integration | Interface/contract boundary |
| INT-053 | Aurora ↔ A-TownChain | Staking Integration | Interface/contract boundary |
| INT-054 | Aurora ↔ A-TownChain | Marketplace Integration | Interface/contract boundary |
| INT-055 | Aurora ↔ A-TownChain | NFT Integration | Interface/contract boundary |
| INT-056 | Wallet ↔ Blockchain | Account State | Interface/contract boundary |
| INT-057 | Wallet ↔ Blockchain | Balance State | Interface/contract boundary |
| INT-058 | Wallet ↔ Blockchain | Nonce State | Interface/contract boundary |
| INT-059 | Wallet ↔ Blockchain | Transaction Encoding | Interface/contract boundary |
| INT-060 | Wallet ↔ Blockchain | Signature | Interface/contract boundary |
| INT-061 | Wallet ↔ Blockchain | Transaction Broadcast | Interface/contract boundary |
| INT-062 | Wallet ↔ Blockchain | Confirmation | Interface/contract boundary |
| INT-063 | Wallet ↔ Blockchain | Finality | Interface/contract boundary |
| INT-064 | Wallet ↔ Blockchain | Staking | Interface/contract boundary |
| INT-065 | Wallet ↔ Blockchain | NFT | Interface/contract boundary |
| INT-066 | Wallet ↔ Blockchain | Token | Interface/contract boundary |
| INT-067 | Blockchain Internal Integration | Transaction → Mempool | Interface/contract boundary |
| INT-068 | Blockchain Internal Integration | Mempool → Consensus | Interface/contract boundary |
| INT-069 | Blockchain Internal Integration | Consensus → Block | Interface/contract boundary |
| INT-070 | Blockchain Internal Integration | Block → Execution | Interface/contract boundary |
| INT-071 | Blockchain Internal Integration | Execution → ATC-VM | Interface/contract boundary |
| INT-072 | Blockchain Internal Integration | VM → State | Interface/contract boundary |
| INT-073 | Blockchain Internal Integration | State → Storage | Interface/contract boundary |
| INT-074 | Blockchain Internal Integration | Consensus → Finality | Interface/contract boundary |
| INT-075 | Blockchain Internal Integration | Node → Indexer | Interface/contract boundary |
| INT-076 | Blockchain Internal Integration | Node → RPC | Interface/contract boundary |
| INT-077 | Blockchain Internal Integration | P2P → Node | Interface/contract boundary |
| INT-078 | Blockchain Internal Integration | Sync → Storage | Interface/contract boundary |
| INT-079 | Consensus ↔ Staking | Validator Registration | Interface/contract boundary |
| INT-080 | Consensus ↔ Staking | Eligibility | Interface/contract boundary |
| INT-081 | Consensus ↔ Staking | Active Set | Interface/contract boundary |
| INT-082 | Consensus ↔ Staking | Voting Weight | Interface/contract boundary |
| INT-083 | Consensus ↔ Staking | Validator Performance | Interface/contract boundary |
| INT-084 | Consensus ↔ Staking | Slashing Evidence | Interface/contract boundary |
| INT-085 | Consensus ↔ Staking | Key Rotation | Interface/contract boundary |
| INT-086 | Consensus ↔ Staking | Validator Exit | Interface/contract boundary |
| INT-087 | Mining ↔ Blockchain | Mining Work | Interface/contract boundary |
| INT-088 | Mining ↔ Blockchain | Candidate Block | Interface/contract boundary |
| INT-089 | Mining ↔ Blockchain | PoW Validation | Interface/contract boundary |
| INT-090 | Mining ↔ Blockchain | Difficulty | Interface/contract boundary |
| INT-091 | Mining ↔ Blockchain | Reward Calculation | Interface/contract boundary |
| INT-092 | Mining ↔ Blockchain | Reward Settlement | Interface/contract boundary |
| INT-093 | Mining ↔ Blockchain | Supply Enforcement | Interface/contract boundary |
| INT-094 | Mining ↔ Blockchain | Double Claim | Interface/contract boundary |
| INT-095 | Mining ↔ Blockchain | Mining Telemetry | Interface/contract boundary |
| INT-096 | ATC-VM ↔ Blockchain | Bytecode Loading | Interface/contract boundary |
| INT-097 | ATC-VM ↔ Blockchain | Verification | Interface/contract boundary |
| INT-098 | ATC-VM ↔ Blockchain | Transaction Context | Interface/contract boundary |
| INT-099 | ATC-VM ↔ Blockchain | Block Context | Interface/contract boundary |
| INT-100 | ATC-VM ↔ Blockchain | Host API | Interface/contract boundary |
| INT-101 | ATC-VM ↔ Blockchain | Storage API | Interface/contract boundary |
| INT-102 | ATC-VM ↔ Blockchain | Gas | Interface/contract boundary |
| INT-103 | ATC-VM ↔ Blockchain | Events | Interface/contract boundary |
| INT-104 | ATC-VM ↔ Blockchain | Contract Calls | Interface/contract boundary |
| INT-105 | ATC-VM ↔ Blockchain | State Commit | Interface/contract boundary |
| INT-106 | ATCLang ↔ ATC-VM | Source → Lexer | Interface/contract boundary |
| INT-107 | ATCLang ↔ ATC-VM | AST → Semantic Analysis | Interface/contract boundary |
| INT-108 | ATCLang ↔ ATC-VM | Semantic → ATC-IR | Interface/contract boundary |
| INT-109 | ATCLang ↔ ATC-VM | ATC-IR → ABI | Interface/contract boundary |
| INT-110 | ATCLang ↔ ATC-VM | ABI → Bytecode | Interface/contract boundary |
| INT-111 | ATCLang ↔ ATC-VM | Bytecode → Verifier | Interface/contract boundary |
| INT-112 | ATCLang ↔ ATC-VM | Verifier → VM | Interface/contract boundary |
| INT-113 | ATCLang ↔ ATC-VM | Contract → Deployment | Interface/contract boundary |
| INT-114 | Frontend ↔ Backend | Frontend → API Gateway | Interface/contract boundary |
| INT-115 | Frontend ↔ Backend | Auth UI → Identity | Interface/contract boundary |
| INT-116 | Frontend ↔ Backend | UI → Authorization | Interface/contract boundary |
| INT-117 | Frontend ↔ Backend | UI → Wallet | Interface/contract boundary |
| INT-118 | Frontend ↔ Backend | UI → Blockchain | Interface/contract boundary |
| INT-119 | Frontend ↔ Backend | UI → Aurora | Interface/contract boundary |
| INT-120 | Frontend ↔ Backend | UI → Marketplace | Interface/contract boundary |
| INT-121 | Frontend ↔ Backend | UI → NFT | Interface/contract boundary |
| INT-122 | Frontend ↔ Backend | UI → Genesis | Interface/contract boundary |
| INT-123 | Frontend ↔ Backend | UI → OS | Interface/contract boundary |
| INT-124 | Frontend ↔ Backend | UI → Events | Interface/contract boundary |
| INT-125 | GateToHell ↔ Ecosystem | Browser → Identity | Interface/contract boundary |
| INT-126 | GateToHell ↔ Ecosystem | Browser → Wallet | Interface/contract boundary |
| INT-127 | GateToHell ↔ Ecosystem | Browser → Aurora | Interface/contract boundary |
| INT-128 | GateToHell ↔ Ecosystem | Browser → Marketplace | Interface/contract boundary |
| INT-129 | GateToHell ↔ Ecosystem | Browser → NFT | Interface/contract boundary |
| INT-130 | GateToHell ↔ Ecosystem | Browser → Blockchain | Interface/contract boundary |
| INT-131 | GateToHell ↔ Ecosystem | Browser → gth:// | Interface/contract boundary |
| INT-132 | GateToHell ↔ Ecosystem | Browser → GlobusOS | Interface/contract boundary |
| INT-133 | GateToHell ↔ Ecosystem | Browser → Storage | Interface/contract boundary |
| INT-134 | GateToHell ↔ Ecosystem | Browser → Extensions | Interface/contract boundary |
| INT-135 | Genesis ↔ Ecosystem | Genesis → OS | Interface/contract boundary |
| INT-136 | Genesis ↔ Ecosystem | Genesis → GPU | Interface/contract boundary |
| INT-137 | Genesis ↔ Ecosystem | Genesis → Audio | Interface/contract boundary |
| INT-138 | Genesis ↔ Ecosystem | Genesis → Input | Interface/contract boundary |
| INT-139 | Genesis ↔ Ecosystem | Genesis → Aurora | Interface/contract boundary |
| INT-140 | Genesis ↔ Ecosystem | Genesis → Blockchain | Interface/contract boundary |
| INT-141 | Genesis ↔ Ecosystem | Genesis → Wallet | Interface/contract boundary |
| INT-142 | Genesis ↔ Ecosystem | Genesis → NFT | Interface/contract boundary |
| INT-143 | Genesis ↔ Ecosystem | Genesis → Marketplace | Interface/contract boundary |
| INT-144 | Genesis ↔ Ecosystem | Genesis → Storage | Interface/contract boundary |
| INT-145 | Genesis ↔ Ecosystem | Genesis → Network | Interface/contract boundary |
| INT-146 | Marketplace ↔ Blockchain | Listing → Chain | Interface/contract boundary |
| INT-147 | Marketplace ↔ Blockchain | Purchase → Wallet | Interface/contract boundary |
| INT-148 | Marketplace ↔ Blockchain | Purchase → TX | Interface/contract boundary |
| INT-149 | Marketplace ↔ Blockchain | Escrow → State | Interface/contract boundary |
| INT-150 | Marketplace ↔ Blockchain | Payment → State | Interface/contract boundary |
| INT-151 | Marketplace ↔ Blockchain | Royalty → State | Interface/contract boundary |
| INT-152 | Marketplace ↔ Blockchain | Ownership → NFT | Interface/contract boundary |
| INT-153 | Marketplace ↔ Blockchain | License → State | Interface/contract boundary |
| INT-154 | Marketplace ↔ Blockchain | Event → Indexer | Interface/contract boundary |
| INT-155 | NFT ↔ Blockchain | Collection → Contract | Interface/contract boundary |
| INT-156 | NFT ↔ Blockchain | Mint → State | Interface/contract boundary |
| INT-157 | NFT ↔ Blockchain | Ownership → State | Interface/contract boundary |
| INT-158 | NFT ↔ Blockchain | Transfer → State | Interface/contract boundary |
| INT-159 | NFT ↔ Blockchain | Metadata → Storage | Interface/contract boundary |
| INT-160 | NFT ↔ Blockchain | Provenance → Evidence | Interface/contract boundary |
| INT-161 | NFT ↔ Blockchain | NFT → Wallet | Interface/contract boundary |
| INT-162 | NFT ↔ Blockchain | NFT → Marketplace | Interface/contract boundary |
| INT-163 | NFT ↔ Blockchain | NFT → Genesis | Interface/contract boundary |
| INT-164 | NFT ↔ Blockchain | NFT → Indexer | Interface/contract boundary |
| INT-165 | System Management Integration | Task Manager → Process Manager | Interface/contract boundary |
| INT-166 | System Management Integration | Control Center → System APIs | Interface/contract boundary |
| INT-167 | System Management Integration | Device Manager → Drivers | Interface/contract boundary |
| INT-168 | System Management Integration | Update Manager → Package Manager | Interface/contract boundary |
| INT-169 | System Management Integration | Driver Manager → Driver Framework | Interface/contract boundary |
| INT-170 | System Management Integration | Firmware Manager → Boot Layer | Interface/contract boundary |
| INT-171 | System Management Integration | Recovery Manager → Snapshot | Interface/contract boundary |
| INT-172 | System Management Integration | Backup → Storage | Interface/contract boundary |
| INT-173 | System Management Integration | Security Center → Security Services | Interface/contract boundary |
| INT-174 | System Management Integration | AI Manager → Aurora | Interface/contract boundary |
| INT-175 | System Management Integration | System Audit → Evidence | Interface/contract boundary |
| INT-176 | Cross-System Data Integration | Identity Federation | Interface/contract boundary |
| INT-177 | Cross-System Data Integration | Capability Federation | Interface/contract boundary |
| INT-178 | Cross-System Data Integration | Policy Federation | Interface/contract boundary |
| INT-179 | Cross-System Data Integration | Event Federation | Interface/contract boundary |
| INT-180 | Cross-System Data Integration | Audit Federation | Interface/contract boundary |
| INT-181 | Cross-System Data Integration | Time Contract | Interface/contract boundary |
| INT-182 | Cross-System Data Integration | Resource Contract | Interface/contract boundary |
| INT-183 | Cross-System Data Integration | Secret Boundary | Interface/contract boundary |
| INT-184 | Cross-System Data Integration | Artifact Provenance | Interface/contract boundary |
| INT-185 | Cross-System Data Integration | Recovery Contract | Interface/contract boundary |
| INT-186 | Integration Security | API Authentication | Interface/contract boundary |
| INT-187 | Integration Security | API Authorization | Interface/contract boundary |
| INT-188 | Integration Security | Capability Enforcement | Interface/contract boundary |
| INT-189 | Integration Security | Policy Enforcement | Interface/contract boundary |
| INT-190 | Integration Security | Approval Enforcement | Interface/contract boundary |
| INT-191 | Integration Security | Input Validation | Interface/contract boundary |
| INT-192 | Integration Security | Output Validation | Interface/contract boundary |
| INT-193 | Integration Security | Replay Protection | Interface/contract boundary |
| INT-194 | Integration Security | Rate Limiting | Interface/contract boundary |
| INT-195 | Integration Security | Secret Isolation | Interface/contract boundary |
| INT-196 | Integration Security | Provenance Validation | Interface/contract boundary |
| INT-197 | Integration Security | Integrity Verification | Interface/contract boundary |
| INT-198 | Integration Security | Auditability | Interface/contract boundary |
| INT-199 | Integration Testing | API Integration Tests | Interface/contract boundary |
| INT-200 | Integration Testing | Service Integration Tests | Interface/contract boundary |
| INT-201 | Integration Testing | OS Integration Tests | Interface/contract boundary |
| INT-202 | Integration Testing | AI Integration Tests | Interface/contract boundary |
| INT-203 | Integration Testing | Blockchain Integration Tests | Interface/contract boundary |
| INT-204 | Integration Testing | Wallet Integration Tests | Interface/contract boundary |
| INT-205 | Integration Testing | VM Integration Tests | Interface/contract boundary |
| INT-206 | Integration Testing | P2P Integration Tests | Interface/contract boundary |
| INT-207 | Integration Testing | Sync Integration Tests | Interface/contract boundary |
| INT-208 | Integration Testing | Marketplace Integration Tests | Interface/contract boundary |
| INT-209 | Integration Testing | NFT Integration Tests | Interface/contract boundary |
| INT-210 | Integration Testing | Genesis Integration Tests | Interface/contract boundary |
| INT-211 | Integration Testing | Browser Integration Tests | Interface/contract boundary |
| INT-212 | Integration Testing | Full E2E | Interface/contract boundary |

## Authority boundaries
- Aurora has no direct kernel authority.
- UI/browser/model output is never authorization.
- Orchestrator coordinates but does not become the authority source.
- Wallet owns explicit signing/key boundary.
- Consensus owns proposal/voting/finality.
- Staking supplies economic and eligibility inputs.
- Mining does not define independent monetary rules.
- ATC-VM executes deterministic state transitions but does not replace chain authority.
- ShivaCore remains the trusted kernel/TCB boundary.
- Indexers and UIs are derived views.

## Critical transaction contract
`Aurora → Capability → Policy → Approval → Wallet → Canonical Transaction → Explicit Signature → SDK → RPC → Node → Mempool → Consensus → Execution → State`

Rust, TypeScript SDK and Wallet must be byte-identical for canonical TX wire encoding.

## Integration contract
Every integration must identify:
`Source, Target, Interface, Protocol, Data Contract, Request, Response, State Transition, Authentication, Authorization, Capability, Approval, Security Boundary, Error Handling, Timeout, Retry, Idempotency, Audit, Test, Workflow, Exact-SHA, Run, Job, Step, Exit Code, Log, Verification, Residual`

## Verification
`Architecture → Interface → Contract → Source → Integration Test → E2E Test → Exact-SHA CI → Run → Job → Step → Exit Code → Log → Verification → Residual`

**Integration present ≠ integration correct.**