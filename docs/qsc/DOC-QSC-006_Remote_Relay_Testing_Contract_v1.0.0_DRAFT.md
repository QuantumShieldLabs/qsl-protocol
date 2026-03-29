Goals: G4, G5

Status: Supporting
Owner: QSC maintainers
Last-Updated: 2026-03-29

# DOC-QSC-006 — Remote Relay Testing Contract

## Role in the current product surface

- This document captures compatibility-only remote evidence for `qsc`.
- It is not the validated qbuild/local front door for `qsc`.
- Use `qsl/qsl-client/qsc/README.md` and `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
  first when you need the current truthful operator baseline.

## Purpose

Enable realistic remote relay testing without destabilizing the current qbuild-first,
AWS-free validation baseline. This lane remains secondary proof for remote transport
health and remote handshake exercise; it is never the operator front door.

## Threat model

- Relay is hostile/unreliable (drop/reorder/duplicate/delay).
- Network conditions are nondeterministic (timing variance).

## Configuration

- `RELAY_URL` required.
- `RELAY_TOKEN` optional secret when the remote lane requires auth.
- Timeout and region notes are evidence inputs for a given remote run; record the
  actual values in the evidence bundle instead of relying on placeholders here.

## Safety and redaction

- Logs must be marker-only and safe to share.
- No secrets, keys, payloads, or secret-bearing URLs may appear in artifacts.

## Determinism

- Define a normalized marker subset for comparison across runs.
- Same scenario inputs should yield identical normalized subsets even if timings differ.
- The remote relay smoke lane explicitly runs in `protocol_mode=seed_fallback_test` by exporting
  `QSC_QSP_SEED` and `QSC_ALLOW_SEED_FALLBACK=1`; it is a transport/reliability lane and
  not handshake/session-proof evidence.

## Execution policy

- Remote relay and remote handshake lanes are compatibility-only proof, not the qbuild/local baseline.
- They may be wrapped by manual or scheduled automation, but any such automation is non-authoritative here.
- They must never become required merge gates for ordinary PR validation.

## Remote relay smoke lane

- Script: `scripts/demo/qsc_remote_relay_smoke.sh`
- Env: `RELAY_URL` required; `RELAY_TOKEN` optional secret
- Artifacts: `remote.markers`, `normalized_subset.txt`, `summary.txt`

## Remote handshake lane

- Script: `scripts/demo/qsc_remote_handshake_smoke.sh`
- Env: `RELAY_URL` and `RELAY_TOKEN` required
- Protocol mode: real handshake/session proof only (no `QSC_ALLOW_SEED_FALLBACK`)
- Sequence:
  - `alice handshake init --peer bob`
  - `bob handshake poll --peer alice`
  - `alice handshake poll --peer bob`
  - `bob handshake poll --peer alice` (A2 confirm)
  - then bidirectional `send` + `receive` using explicit mailbox/peer split:
    - bob receive: `--mailbox bob --from alice`
    - alice receive: `--mailbox alice --from bob`
- Required checks:
  - both peers established from handshake status and lane marker `qsp_status ACTIVE reason=handshake`
  - `qsp_pack ok=true` present for `alice->bob` and `bob->alice`
  - `qsp_unpack ok=true` present for both receive directions
  - `recv_commit count>=1` for both receive directions
  - fail closed if any `protocol_inactive` or `relay_unauthorized`
- Artifacts:
  - `alice.log`, `bob.log`, `alice_recv.log`, `bob_recv.log`
  - `summary.txt`, `normalized_subset.txt`, `normalized_counts.txt`, `markers`
- Redaction plus deterministic subset:
  - redact relay URL/token from artifacts
  - exclude random channel/message identifiers from normalized subset

## Scenario inputs

- `scenario`: `happy-path` or `drop-reorder`
- `seed`: `u64` string

Example:

- Run the remote relay smoke lane with `scenario=drop-reorder` and `seed=7`.
