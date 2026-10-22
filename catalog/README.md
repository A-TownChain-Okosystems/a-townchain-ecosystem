# Ecosystem file/function catalog

This catalog is an inventory and evidence control plane, not an implementation source of truth. It does not replace `atc-standards` or component repositories.

## Files

- `repositories.json`: organization repository inventory and snapshot state.
- `files.jsonl`: one JSON object per tracked file; records are appended only after the relevant file and commit have been inspected.
- `schema/file-inventory.schema.json`: JSON Schema for file records.
- `coverage.json`: counts and explicit gaps per repository.
- `dependencies.json`: evidence-backed cross-file and cross-repository edges.

## Evidence rules

- Pin every claim to repository, exact commit SHA, path, and blob SHA.
- Distinguish declared dependencies from observed call/data-flow edges.
- Distinguish tests found from tests executed; CI green alone does not establish functional verification.
- Unknown or uninspected facts remain `UNKNOWN` or `BLOCKED`; never infer successful verification from missing evidence.
- Non-code files receive purpose/effect analysis rather than fabricated function lists.
- `demo-repository` is inventoried but excluded from the live drift gate.
- No implementation SSOT, standard, or protocol registry is changed by this catalog.

## Bootstrap state

The repository manifest currently records a first snapshot for eight repositories. The other listed repositories remain pending exact ref/tree retrieval. No claim of complete file-level analysis is made. Counts in the manifest are file-tree counts, not semantic-review counts.
