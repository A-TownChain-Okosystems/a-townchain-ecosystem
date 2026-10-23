# ATC Engineering Evidence Handoff Contract

Status: implemented as a schema and fail-closed consumer validator; live evidence is accepted only when supplied and bound to the canonical census.

## Ownership

- `atc-engineering` produces build evidence and validates the local artifact bytes before upload.
- `a-townchain-ecosystem` consumes the resulting manifest as a control-plane record. It does not replace the engineering build, implementation repositories, or normative standards.
- This contract does not promote a component/function record to `VERIFIED` by itself.

## Contract

Schema: `catalog/schema/atc-build-evidence-handoff.schema.json`

Consumer: `catalog/scripts/validate_engineering_handoff.py`

The consumer requires the `atc.build-evidence/v1` manifest to contain the repository, exact source and checkout SHA, event identity, run ID/attempt, workflow ref, binary SHA-256, gate-report SHA-256, and schema identifiers. It then compares the manifest SHA to the unique `A-TownChain-Okosystems/atc-engineering` HEAD recorded in `catalog/repositories.json`.

## Invocation

Automated consumer: `.github/workflows/consume-engineering-evidence.yml`. Run it manually with the source Actions run ID and an independently inspected full SHA. It accepts only a successful `push` run for `main`, checks the GitHub artifact ZIP digest and its internal `SHA256SUMS`, verifies the binary/report digests against the manifest, then invokes the census-bound validator.

For local or offline validation after downloading the evidence artifact:

```bash
python catalog/scripts/validate_engineering_handoff.py \
  --manifest /path/to/evidence-manifest.json \
  --census catalog/repositories.json \
  --expected-sha <full-40-character-commit-sha>
```

The `--expected-sha` value must come from an independently inspected main-branch head, not from the manifest being validated. The census snapshot must also be refreshed to that exact source SHA before the handoff can pass. The workflow requires `ATC_FLEET_READ_TOKEN` with read access to `atc-engineering`; missing access fails closed.

## Fail-closed conditions

- Missing/malformed fields, unsupported schema, invalid digests, or invalid event-specific PR-head semantics: `BLOCKED`.
- Manifest SHA differs from its checkout SHA, independently expected SHA, or canonical census HEAD: `BLOCKED`.
- Missing artifact bytes, checksum mismatch, workflow logs not inspected, or run/job/step evidence not recorded: do not promote status beyond the evidence actually established.
- Green CI or schema-valid JSON alone is not `VERIFIED`; functional acceptance and governance remain separate gates.

## Current limitation

The consumer and regression tests are implemented, but this change does not silently fetch another repository's Actions artifacts or invent a manifest. A real handoff requires the evidence artifact from the actual engineering run, independent SHA inspection, and a current census snapshot. Until then, live handoff status remains `BLOCKED`.
