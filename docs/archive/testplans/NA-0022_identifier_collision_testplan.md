Goals: G5

# NA-0022 — Demo relay identifier collision handling test plan

Status: DRAFT
Scope: Demo relay/CLI only. No protocol-core changes.

## Objective
- Enforce deterministic duplicate registration rejection.
- Enforce relay identifier format bounds and invalid-id rejection.

## CI-gated assertions
Enforced by: `scripts/ci/metadata_conformance_smoke.sh`

- Registering the same id twice returns 409 on the second attempt.
- Registering an id outside the allowed format returns 400.

## Evidence
- metadata-conformance-smoke CI job logs.
