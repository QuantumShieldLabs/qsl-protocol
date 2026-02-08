# DOC-QSC-006 — Remote Relay Testing Contract (DRAFT)

## Purpose
Enable realistic, remote relay testing without destabilizing PR CI. This lane is nightly/manual only and never required for merges.

## Threat model
- Relay is hostile/unreliable (drop/reorder/duplicate/delay).
- Network conditions are nondeterministic (timing variance).

## Configuration
- RELAY_URL (required)
- RELAY_TOKEN (optional, secret)
- Timeouts and region notes (TBD if needed)

## Safety & redaction
- Logs must be marker-only and safe to share.
- No secrets, keys, or payloads may appear in artifacts.

## Determinism
- Define a normalized marker subset for comparison across runs.
- Same scenario inputs should yield identical normalized subsets even if timings differ.
- This lane explicitly runs in `protocol_mode=seed_fallback_test` by exporting
  `QSC_QSP_SEED` and `QSC_ALLOW_SEED_FALLBACK=1`; it is a transport/reliability
  lane and not handshake/session-proof evidence.

## CI policy
- Workflow must be workflow_dispatch + nightly only.
- Must never be required for PR merge.

## Implementation
- Script: scripts/demo/qsc_remote_relay_smoke.sh
- Workflow: .github/workflows/remote-relay-tests.yml (workflow_dispatch + nightly)
- Env: RELAY_URL (required), RELAY_TOKEN (optional secret)
- Artifacts: remote.markers, normalized_subset.txt, summary.txt

## workflow_dispatch inputs

- scenario: happy-path | drop-reorder
- seed: u64 (string)

Example:
- Run remote-relay-tests with scenario=drop-reorder seed=7
