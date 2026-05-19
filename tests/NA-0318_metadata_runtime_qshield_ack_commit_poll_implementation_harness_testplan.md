Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0318 Metadata Runtime qshield Ack/Commit Poll Implementation Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qshield embedded-relay ack/commit-after-local-verify
implementation selected by NA-0317, proving candidate fetch preserves the
remote queue until explicit ack and invalid local receive-side reject does not
delete the queued candidate.

## Protected Invariants

- Invalid local receive verification must not delete the remote queued message
  in the proven qshield embedded-relay boundary.
- Invalid local receive verification must not create accepted local state.
- Invalid local receive verification must not create accepted output.
- Invalid local receive verification must not leak relay token, candidate id,
  plaintext sentinel, padding sentinel, panic, or backtrace text.
- Valid local receive verification may commit/delete exactly one queued
  message.
- Legacy destructive `/poll` remains visible and must not be used as metadata
  no-mutation proof.
- No qsl-server production relay support is claimed.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`
- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`
- `tests/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation changes.
- qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Cargo manifest or lockfile changes.
- Workflow, branch-protection, public-safety, README, START_HERE, website, or
  external repo changes.
- NA-0319 implementation.

## Prior Authorization Review Requirements

Review and preserve:

- NA-0315 metadata runtime harness plan.
- NA-0316 `PROVEN_REMOTE_MUTATION` / `NEEDS_RUNTIME_CHANGE` classification for
  legacy qshield `/poll`.
- NA-0317 ack/commit authorization and future marker set.
- Live NA-0318 allowed and forbidden scope.

## Implementation Requirements

- Add queue-preserving candidate fetch.
- Add explicit ack/commit delete.
- Keep route-token checks intact.
- Preserve legacy `/poll` as visible destructive behavior or document any
  compatibility change.
- Update qshield receive paths to use candidate fetch and ack after local
  verification.
- Keep errors coarse and secret-safe.

## Harness Marker Requirements

The executable harness must declare and emit when run with `--nocapture`:

- `NA0318_QSHIELD_PEEK_OR_POLL_CANDIDATE_OK`
- `NA0318_QSHIELD_LOCAL_VERIFY_BEFORE_COMMIT_OK`
- `NA0318_QSHIELD_ACK_COMMIT_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_REMOTE_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_ACCEPTED_STATE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_OUTPUT_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_SECRET_LEAK_OK`
- `NA0318_QSHIELD_STALE_ACK_FAIL_CLOSED_OK`
- `NA0318_QSHIELD_REPEATED_INVALID_RECV_BOUNDED_OK`
- `NA0318_METADATA_RUNTIME_ACK_COMMIT_OK`

## Valid Path Requirements

- Queue at least two relay messages.
- Fetch one candidate without deleting it.
- Fetch again and observe the same candidate.
- Ack the first candidate.
- Prove exactly one queued message remains.
- Prove stale/duplicate ack for the first candidate fails closed and does not
  delete the remaining message.

## Invalid Path Requirements

- Queue a representable invalid local receive candidate.
- Run qshield `recv`.
- Prove the command rejects.
- Prove the same candidate remains remotely queued.
- Repeat invalid receive and prove deterministic bounded behavior.

## No-Remote-Delete Requirements

After invalid receive reject, a fresh `/poll-candidate` call must return the
same candidate id. No `/ack` must be accepted for that invalid candidate.

## No-Local-Output / State Requirements

The harness must byte-compare local qshield state before and after invalid
receive and assert no accepted `from <peer>:` output appears.

## No-Secret-Leak Requirements

The invalid receive output must be scanned for:

- relay token;
- candidate id;
- plaintext sentinel;
- padding sentinel;
- panic/backtrace markers.

Any match is a failure.

## Successor-Selection Requirements

If the implementation and harness prove qshield embedded relay queue-
preserving ack/commit behavior, selected successor is:

`NA-0319 -- Metadata Runtime Identifier and Default Padding Executable Harness`

If implementation blocks on qsl-server support, select a cross-repo
authorization successor instead and do not emit implementation markers.

## Claim-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay boundary only;
- no qsl-server production relay proof;
- no metadata-free/anonymity/untraceable claim;
- no production/public-internet/external-review completion claim;
- metadata runtime gaps remain visible.

## Backup-Impact Requirements

Record whether changes create durable artifacts outside the qsl-protocol
worktree or existing Codex response path. Expected result: no backup-plan
update required.

## Required Local Checks

Run or record exact blocker:

- `cargo fmt --package qshield-cli -- --check`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted NA-0310 refimpl oracle test
- qsc NA-0313 harness if directly runnable
- queue/decision/scope/link/leak/goal-lint checks

## CI Expectations

The PR must keep required checks green, including `public-safety`. Runtime
critical qshield changes should run qshield tests/build and demo smoke/stress
checks. Do not merge with required red or missing checks.

## Successor Handoff

After implementation PR merge and post-merge `public-safety` success, a
separate closeout may mark NA-0318 DONE and restore:

`NA-0319 -- Metadata Runtime Identifier and Default Padding Executable Harness`

The closeout must not implement NA-0319.
