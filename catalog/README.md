# Ecosystem file/function catalog

This catalog is an inventory and evidence control plane, not an implementation source of truth. It does not replace `atc-standards` or component repositories.

## Files

- `repositories.json`: 33-repository snapshot with exact default-branch HEAD commit SHA, tree SHA, tracked blob count, archive status and truncation status.
- `files.jsonl`: one JSON object per analyzed file; current 11 Genesis Engine records are partial semantic notes, not full repository coverage.
- `schema/file-inventory.schema.json`: JSON Schema for file records.
- `coverage.json`: counts and explicit gaps.
- `dependencies.json`: evidence-backed cross-file and cross-repository edges.
- `validation-report.json`: recorded structural validation output and limitations.

## Evidence rules

- Pin every claim to repository, exact commit SHA, path, and blob SHA.
- Keep commit SHA and tree SHA as separate fields.
- Distinguish declared dependencies from observed call/data-flow edges.
- Distinguish tests found from tests executed; CI green alone does not establish functional verification.
- Unknown or uninspected facts remain `UNKNOWN` or `BLOCKED`; never infer successful verification from missing evidence.
- Non-code files receive purpose/effect analysis rather than fabricated function lists.
- `demo-repository` is inventoried but excluded from the live drift gate.
- No implementation SSOT, standard, or protocol registry is changed by this catalog.

## Snapshot and validation state

The current manifest contains all 33 organization repositories. Each default-branch ref and recursive tree was fetched; all 33 trees returned `truncated=false`. The manifest records 23,808 tracked blob entries across the captured trees. This is a Git-tree count, not a count of semantically reviewed source files; generated, vendor and binary files may be included.

A structural validation report checks repository count and uniqueness, 40-character Git object SHA shape, tree truncation flags, file counts, demo exclusion, coverage consistency, JSON parsing and required JSONL fields. This was an in-session structural check, not a standards-compliant JSON Schema validator and not a CI run.

File-level functional analysis remains partial: only 11 Genesis Engine records exist, call-graph completeness is not established, and tests have not been verified against every function. Do not treat repository snapshot completion as functional audit completion.
