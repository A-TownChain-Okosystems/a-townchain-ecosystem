# Unified component and function catalog reconciliation

## Purpose

`reconcile_catalogs.py` creates a loss-preserving normalized union from JSON and Markdown catalog snapshots registered in `docs/architecture/ATC-CATALOG-SOURCE-REGISTRY-001.json`. It is an inventory/reconciliation tool, not an implementation verifier and not a replacement for the owning repository's catalog.

## Snapshot staging

For each registry entry, fetch the exact catalog path at a reviewed source ref and save JSON bytes as `<source_id>.json` or Markdown bytes as `<source_id>.md`, according to `snapshot_format` in the registry:

```text
<snapshot-dir>/<source_id>.json
```

Use the exact source commit SHA where available. Before accepting a snapshot, independently verify that its bytes correspond to the declared repository/path/ref. The tool records a SHA-256 of each staged file; it does not itself fetch GitHub or prove that the snapshot corresponds to a declared Git commit.

Do not stage a file under a source ID until its source repository, catalog path, source ref/commit, and content have been reviewed. If a registered source is missing or invalid, the run fails closed and the report remains BLOCKED.

## Run

```bash
python3 tools/catalog/reconcile_catalogs.py \
  --registry docs/architecture/ATC-CATALOG-SOURCE-REGISTRY-001.json \
  --sources-dir /tmp/atc-catalog-snapshots \
  --out-dir build/catalog-reconciliation
```

Outputs:

- `catalog-master.json`: normalized records, retaining each complete original `source_record`, original ID, source pointer, source hash, and declared source provenance.
- `reconciliation-report.json`: per-source import counts, errors, ID collisions, and possible duplicate groups for review.

A non-zero exit indicates invalid/missing inputs or ID collisions. Do not publish generated output as complete unless every registered source is imported and independently reconciled.

## Identity and duplicate policy

- Namespaced IDs are deterministic for a given source ID, record kind, and original ID (or source-record content hash when no original ID exists).
- Same-name records remain separate and are reported as `POSSIBLE_DUPLICATE_REQUIRES_REVIEW`. Names alone are not sufficient to merge records.
- Original records are never overwritten by normalized display fields.
- Requirements, UI actions, test functions, opcodes, components, APIs and function contracts remain distinct `record_kind` values.
- Unknown collection shapes are not guessed. Review the source schema and extend `COLLECTION_KINDS` deliberately; a source with no recognized collections fails closed.

## Status semantics

`UNRECONCILED`, `UNASSESSED` and `UNVERIFIED` are intentionally conservative. Catalog membership does not establish that a symbol is implemented, tested, deterministic, secure, or functionally verified. Unresolved consensus semantics remain BLOCKED until a versioned normative decision is approved. The ecosystem repository remains an architecture/integration/evidence control plane; canonical implementation ownership stays with the designated source repository.

## Current limitations

- The source registry is not yet a complete census of all active repositories and catalog-like lists.
- Source snapshots must be staged separately; this script performs no network calls.
- Markdown extraction covers pipe tables only. YAML, source-code ASTs, shell entrypoints and workflow declarations need format-specific extractors and explicit scope accounting before they can be claimed as inventoried.
- The generated master catalog is non-normative until all sources, exclusions, contracts, and validation gates are reconciled.
