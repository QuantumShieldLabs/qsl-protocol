Goals: G3, G4

# NA-0020 — Demo establish replay cache test plan

Status: DRAFT
Scope: Demo relay/CLI only. No protocol-core changes.

## Objective
- Enforce deterministic replay rejection for establish using a relay-side fingerprint.
- Ensure replay rejects do not mutate relay state (bundle remains available).
- Ensure replay detection is relay-mediated (not bypassable by client state).

## CI-gated assertions
Enforced by: `scripts/ci/metadata_conformance_smoke.sh`

- First establish succeeds and records the replay fingerprint.
- Second establish with the same fingerprint rejects deterministically and does not consume the bundle.
- Replay detection is enforced by the relay (record->consume->persist ordering).

## Evidence
- metadata-conformance-smoke CI job logs.
