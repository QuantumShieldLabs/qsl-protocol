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

## Handshake lane
- Script: `scripts/demo/qsc_remote_handshake_smoke.sh`
- Workflow: `.github/workflows/remote-handshake-tests.yml` (`workflow_dispatch` + nightly only; never `pull_request`)
- Env: `RELAY_URL` + `RELAY_TOKEN` required
- Protocol mode: real handshake/session proof only (no `QSC_ALLOW_SEED_FALLBACK`)
- Sequence:
  - `alice handshake init --peer bob`
  - `bob handshake poll --peer alice`
  - `alice handshake poll --peer bob`
  - `bob handshake poll --peer alice` (A2 confirm)
  - then bidirectional `send` + `receive`
- Required checks:
  - both peers established from handshake status and lane marker `qsp_status ACTIVE reason=handshake`
  - `qsp_pack ok=true` present for `alice->bob` and `bob->alice`
  - `qsp_unpack ok=true` present for both receive directions
  - `recv_commit count>=1` for both receive directions
  - fail closed if any `protocol_inactive` or `relay_unauthorized`
- Artifacts:
  - `alice.log`, `bob.log`, `alice_recv.log`, `bob_recv.log`
  - `summary.txt`, `normalized_subset.txt`, `normalized_counts.txt`, `markers`
- Redaction + deterministic subset:
  - redact relay URL/token from artifacts
  - exclude random channel/message identifiers from normalized subset

## workflow_dispatch inputs

- scenario: happy-path | drop-reorder
- seed: u64 (string)

Example:
- Run remote-relay-tests with scenario=drop-reorder seed=7
