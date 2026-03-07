Goals: G5

# NA-0021 — Demo relay rate limiting/backoff test plan

Status: DRAFT
Scope: Demo relay/CLI only. No protocol-core changes.

## Objective
- Enforce deterministic rate limiting for /register and /poll.
- Ensure over-limit requests reject with 429 and retry_after_ms.

## CI-gated assertions
Enforced by: `scripts/ci/metadata_conformance_smoke.sh`

- /poll requests eventually return 429 within a bounded loop.
- /register requests eventually return 429 within a bounded loop.

## Evidence
- metadata-conformance-smoke CI job logs.
