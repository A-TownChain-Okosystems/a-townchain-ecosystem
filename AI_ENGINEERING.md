# AI-Assisted / Vibe Engineering

This repository permits AI-assisted implementation, but generated code is treated exactly like human-authored code.

## Engineering contract

1. Specification first — Every non-trivial change must state the intended behavior and affected interfaces.
2. Repository reality first — Inspect the existing implementation before creating parallel abstractions.
3. Canonical source only — Do not reintroduce duplicate consensus, signing-domain, tokenomics, or protocol constants.
4. Rust-first — Prefer the existing Rust workspace for chain, node, VM, SDK, wallet, storage, mining, and integration logic.
5. No silent architecture changes — AI-generated refactors must preserve public behavior unless the change explicitly requests a migration.
6. Tests are mandatory — New behavior requires regression coverage where practical.
7. Compiler is authoritative — cargo check --workspace --all-targets must pass.
8. Clippy is mandatory — cargo clippy --workspace --all-targets --all-features -- -D warnings must pass where the workspace supports it.
9. Tests are authoritative — cargo test --workspace --all-targets must pass.
10. Integration is authoritative — Changes crossing crate boundaries must exercise the relevant integration tests.
11. No “looks correct” claims — A change is not considered verified until CI produces the corresponding evidence.
12. Human review remains required — AI may implement and repair code; it does not approve protocol/security/governance changes.

## Standard AI coding loop

Requirement
  -> inspect repository
  -> identify canonical implementation
  -> implement smallest coherent change
  -> compile
  -> test
  -> clippy
  -> integration test
  -> inspect CI failures
  -> fix
  -> rerun
  -> review

## Definition of done

A change is complete only when:

- the implementation is connected to the existing production path;
- obsolete or duplicate paths are removed when the migration requires it;
- regression tests cover the changed behavior;
- CI is green for the relevant gates;
- protocol/security/governance changes have explicit human review.

## AI-generated change declaration

Pull requests containing substantial AI-generated code should state:

- what was generated or modified;
- which existing implementation was treated as canonical;
- tests and CI jobs executed;
- known limitations or unverified assumptions.
