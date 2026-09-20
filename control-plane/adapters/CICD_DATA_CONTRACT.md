# CI/CD Data Adapter Contract

The CI/CD panel consumes GitHub Actions evidence, not inferred repository state.

## Required record

```json
{
  "workflow": "string",
  "run_id": "string",
  "status": "queued|in_progress|completed",
  "conclusion": "success|failure|cancelled|skipped|null",
  "head_sha": "string",
  "branch": "string",
  "started_at": "timestamp",
  "completed_at": "timestamp|null",
  "jobs": [],
  "artifacts": []
}
```

## Readiness mapping

- completed + success: gate may pass
- completed + failure/cancelled: gate fails
- queued/in_progress: gate is unresolved
- no run: gate is unresolved

No CI result is treated as success by default.

## Evidence chain

`workflow → run → jobs → artifacts → commit SHA → panel gate`

This contract is intended to be populated from GitHub Actions APIs/connector data.
