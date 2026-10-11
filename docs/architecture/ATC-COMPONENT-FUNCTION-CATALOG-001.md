# ATC Component and Function Catalog 001

**Catalog state:** `BASELINE_RESIDUAL`  
**Release decision:** `BLOCKED`  
**Machine-readable SSOT:** [ATC-COMPONENT-FUNCTION-CATALOG-001.json](ATC-COMPONENT-FUNCTION-CATALOG-001.json)

This catalog is the architecture/control-plane inventory. It does not replace component source code, normative standards, or repository-local API documentation.

## Rules

- Each capability has a stable component ID and function ID.
- Each component names one canonical owner; specifications and support repositories are not treated as duplicate implementation owners.
- Each function records purpose, inputs, outputs, failure modes, measurable acceptance criteria, test classes, and readiness.
- Cross-component edges are explicit and remain `RESIDUAL` until their exact API/ABI and compatibility evidence are verified.
- `IMPLEMENTED` does not mean `VERIFIED`; `PRODUCTION_READY` requires full exact-SHA evidence, required gates, logs/artifacts and review.
- A skipped or pending required check is not a successful check. No automatic merge, repository archive, or source deletion is authorized by this catalog.

## Canonical domains in this baseline

| Component | Canonical implementation owner | Principal responsibilities |
|---|---|---|
| atc-engineering | [atc-engineering](https://github.com/A-TownChain-Okosystems/atc-engineering) | Fleet governance and exact-SHA assurance |
| ATC Standards | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | Normative specifications and conformance contracts |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) | Language syntax, parsing and frontend |
| ShivaCore Kernel | [globus-os/modules/atc-shivacore/kernel](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) | no_std capability microkernel and TCB |
| A-TownChain | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | Canonical L1 transaction, block and state implementation |
| ATC-Algorithm | [a-townchain/components/algorithm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/algorithm) | Canonical consensus/algorithm implementation |
| ATC-VM | [a-townchain/components/vm](https://github.com/A-TownChain-Okosystems/a-townchain/tree/main/components/vm) | Deterministic bytecode execution |
| GlobusOS | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | Userspace services and OS integration above ShivaCore |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | AI runtime, agent and swarm coordination; outside kernel TCB and consensus authority |
| ATC Toolchain | [atc-toolchain](https://github.com/A-TownChain-Okosystems/atc-toolchain) | Compiler/build/test/package tooling |
| Genesis Engine | [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) | Genesis configuration and runtime/bootstrap orchestration |
| Ecosystem control plane | [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | Architecture, integration, compliance and evidence only |

## Production gates

1. **Ownership:** exactly one canonical implementation per capability.
2. **Interface:** versioned inputs, outputs, error semantics and compatibility rules.
3. **Correctness:** positive, negative, boundary, regression and deterministic-vector tests.
4. **Security:** threat model, trust boundary, invalid-input and resource-exhaustion behavior.
5. **Build integrity:** required formatting, lint, build, test and security checks have terminal conclusions.
6. **Operations:** diagnostics, compatibility, recovery/rollback and artifact provenance where applicable.
7. **Evidence:** exact 40-character source SHA, workflow run, job, step, exit code 0, logs/artifacts and review reference.

## Validate locally or in CI

```sh
python3 -m json.tool docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json >/dev/null
python3 tools/validate_component_function_catalog.py \
  --input docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json \
  --report component-function-catalog-report.json
```

The validator's `PASS_STRUCTURAL_ONLY` means the registry is structurally consistent; it is not a certification of production readiness. A component or function cannot be marked `PRODUCTION_READY` without complete evidence fields. The release decision remains `BLOCKED` until implementation and integration gates are independently verified.

## Known protocol-sensitive acceptance criteria

- A-TownChain: chain ID `658467`, supply `360000000 ATC`, 18 decimals, u128 amounts as 16-byte big-endian and u64 counters as 8-byte big-endian.
- TX-V2: secp256k1 ECDSA, RFC6979/SHA-256, low-S, 33-byte compressed SEC1 public key, 64-byte compact signature and `ATC-TX-DOMAIN-V2`; legacy signing domains must be rejected.
- ATC-VM: normative `ATCB` magic, u16 big-endian version, u32 big-endian instruction count; resolve `ATCB` versus `ATC1` drift against the normative specification. Enforce the documented operation, stack and storage limits and gas behavior.
- ShivaCore/GlobusOS: kernel authority remains capability-bounded; AI, networking and contract execution remain outside the kernel TCB unless a formally approved architecture change says otherwise.
- Consensus: do not claim production readiness until the consensus algorithm and parameters are formally frozen and deterministic/adversarial evidence passes.

## Organization-wide coverage

The machine-readable catalog currently defines **30 components**, **63 functions**, **7 cross-component interfaces** and **7 production gates**. It also includes a `repository_coverage` entry for all **33 observed repositories** from the organization integration registry: **26 active** and **7 archived**. The validator checks unique full repository names, archive-state totals and the required fail-closed status of each entry.

This is deliberately not equivalent to 33 complete function inventories. The current catalog defines 30 component records and 63 function contracts across core, developer tooling, node/service and application domains. Every remaining active repository is still `RESIDUAL` / `AUDIT_REQUIRED` until its actual source tree, README/specs, exported APIs, tests, dependencies and exact-SHA CI evidence are inspected. Archived repositories are `REFERENCE_ONLY`, not active production targets. The audit must not invent functions from repository names.

Next audit passes must add repository-specific function IDs and interface contracts for the remaining active domains, including SDK, node, contracts, ZKP, storage/compute services, oracle/interop, applications, organization governance and repository orchestration. Migration candidates require source-to-canonical path/history evidence before KEEP/SYNC/ARCHIVE decisions.

## Updating the catalog

Update this catalog only from reviewed source/interface evidence. Every change should include a reason, exact source refs and test coverage. Unknown or inaccessible facts stay `RESIDUAL`/ `BLOCKED`. Never infer migration completeness from root/subtree tree-SHA equality alone, and never archive or delete a source repository based on this catalog alone.


### Current coverage counts

- Component records: **30**
- Function contracts: **63**
- Cross-component interface records: **7**
- Production gates: **7**
- Repository coverage: **33 total / 26 active / 7 archived**

The catalog includes support/governance records for the standalone ATC-VM and ATC-Algorithm repositories, but their canonical production implementations remain in `a-townchain/components/vm` and `a-townchain/components/algorithm`. The standalone `atc-shivacore` repository is specification/support only; the canonical kernel remains under GlobusOS. Archived repositories are retained as reference records and are not silently reactivated.
