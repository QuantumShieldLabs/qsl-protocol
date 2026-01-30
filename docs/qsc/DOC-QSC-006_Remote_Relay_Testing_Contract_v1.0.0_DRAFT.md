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

## CI policy
- Workflow must be workflow_dispatch + nightly only.
- Must never be required for PR merge.

## Implementation
- Script: scripts/demo/qsc_remote_relay_smoke.sh
- Workflow: .github/workflows/remote-relay-tests.yml (workflow_dispatch + nightly)
- Env: RELAY_URL (required), RELAY_TOKEN (optional secret)
- Artifacts: remote.markers, normalized_subset.txt, summary.txt
