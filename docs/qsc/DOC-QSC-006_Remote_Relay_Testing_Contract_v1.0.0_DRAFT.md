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
- The remote relay smoke lane explicitly runs in
  `protocol_mode=unsafe_seed_fallback_diagnostic` by exporting `QSC_QSP_SEED`,
  `QSC_ALLOW_SEED_FALLBACK=1`, and `QSC_UNSAFE_TEST_SEED_FALLBACK=1`; it is a
  transport/reliability diagnostic lane and not normal runtime or
  handshake/session-proof evidence.

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
- Protocol mode: real handshake/session proof only (no `QSC_ALLOW_SEED_FALLBACK`
  and no `QSC_UNSAFE_TEST_SEED_FALLBACK`)
- Sequence (revised 2026-08-17, NA-0743 / D-1380): the committed script performs the
  round trip FIRST and re-handshakes afterwards. The previous text documented neither the
  re-handshake nor the assertion checkpoint, so it did not describe the committed script
  before this revision either.
  - `alice handshake init --peer bob`
  - `bob handshake poll --peer alice`
  - `alice handshake poll --peer bob`
  - `bob handshake poll --peer alice` (A2 confirm)
  - then the round trip, inside ONE session, with NO handshake between the halves:
    - `alice send --to bob`, then bob receive: `--mailbox <bob route token> --from alice`
    - `bob send --to alice`, then alice receive: `--mailbox <alice route token> --from bob`
    - the mailbox argument is a ROUTE TOKEN, not a peer label (ENG-0192)
  - then the ASSERTION CHECKPOINT: `handshake status` for both peers, each status
    EXTRACTED and compared BY EQUALITY to `established`
  - then the re-handshake (`bob handshake init --peer alice` and three polls), RELOCATED
    to here: it REPLACES the session, so it must FOLLOW the checkpoint and never precede
    it
  - then the X5 OBSERVATION: `handshake status` for both peers again, recorded rather
    than asserted (the re-handshake drives both peers out of `established`)
- Required checks:
  - both peers' EXTRACTED handshake status compared BY EQUALITY to `established` at the
    pre-rehandshake checkpoint (never a substring: `established` is a PREFIX of
    `established_recv_only`), published as `handshake_status_alice_at_checkpoint`,
    `handshake_status_bob_at_checkpoint` and `handshake_checkpoint`
  - the post-rehandshake pair recorded as `handshake_status_alice_after_rehandshake` and
    `handshake_status_bob_after_rehandshake`: the KEYS must be PRESENT and their values
    RECORDED. No specific post-rehandshake VALUE is a required check.
  - ⚠ those values are SCENARIO-DEPENDENT, measured 2026-08-17 by NA-0743: under
    `happy-path` the re-handshake drives both peers out of `established` (alice
    `established_recv_only`, bob `awaiting_peer_confirm`), but under `drop-reorder` it can
    complete nothing at all and both peers stay `established` — see ENG-0198
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

## Revision history

- 2026-08-17 — NA-0743 / D-1380 (ruling R348 §1). Draft revision, two corrections to the
  remote handshake lane. (a) The Required-checks line no longer names the
  `qsp_status ACTIVE reason=handshake` lane marker: the smoke script used to write that
  marker into its published output BY HAND, with no `qsc` process having emitted it, and
  NA-0743 retires it — the line now names the real observables the script publishes.
  (b) The Sequence section is rewritten to describe the reordered flow: round trip first,
  then the assertion checkpoint, then the relocated re-handshake and the post-re-handshake
  observation. Nothing else in this document is changed, `Last-Updated:` included; that
  field is left at its recorded value deliberately rather than by oversight.
