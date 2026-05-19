Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0319 Metadata Runtime Identifier and Default Padding Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qshield embedded-relay metadata runtime harness for
identifier/handle and default-padding behavior after NA-0318 delivered
queue-preserving candidate fetch and explicit ack/commit.

## Protected Invariants

- Invalid identifier or padding rejects must not delete a queued candidate in
  the qshield embedded relay candidate/ack boundary.
- Invalid identifier or padding rejects must not create accepted local state.
- Invalid identifier or padding rejects must not create accepted output.
- Invalid identifier or padding rejects must not leak relay token, candidate
  ack id, raw peer handle, plaintext sentinel, padding sentinel, panic, or
  backtrace text.
- Valid receive may ack/delete exactly one queued candidate after identifier
  and padding verification.
- qsl-server production relay semantics remain explicitly unproven.
- No anonymity, metadata-free, untraceable, public-internet readiness,
  production readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`
- `tests/NA-0319_metadata_runtime_identifier_default_padding_harness_testplan.md`
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
- NA-0320 implementation.

## Prior Ack/Commit Review Requirements

Review and preserve NA-0318 qshield embedded relay proof:

- `/poll-candidate` preserves queued candidates.
- `/ack` deletes exactly one candidate only after verification.
- legacy `/poll` remains destructive and is not no-mutation evidence.
- qsl-server production relay support remains unproven.

## Implementation Requirements

- Use the qshield embedded relay candidate/ack path for remote no-delete
  claims.
- Keep relay token checks intact.
- Strengthen receive-side padding verification fail-closed where needed.
- Keep receive-side identifier errors coarse enough to avoid raw handle output.
- Do not broaden metadata runtime behavior beyond qshield demo proof.

## Harness Marker Requirements

The executable harness must declare and emit:

- `NA0319_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0319_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0319_STALE_HANDLE_REJECT_OK`
- `NA0319_MALFORMED_HANDLE_REJECT_OK`
- `NA0319_IDENTIFIER_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_OUTPUT_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_SECRET_LOG_OK`
- `NA0319_DEFAULT_PADDING_POLICY_OK`
- `NA0319_PADDING_BUCKETS_OK`
- `NA0319_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0319_PADDING_STRIP_VERIFY_OK`
- `NA0319_PADDING_MALFORMED_REJECT_OK`
- `NA0319_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0319_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0319_PADDING_NO_OUTPUT_ON_REJECT_OK`
- `NA0319_PADDING_NO_SECRET_LOG_OK`
- `NA0319_ACK_COMMIT_AFTER_IDENTIFIER_PADDING_VERIFY_OK`
- `NA0319_QSHIELD_EMBEDDED_RELAY_BOUNDARY_OK`
- `NA0319_METADATA_RUNTIME_IDENTIFIER_PADDING_OK`

## Valid Path Requirements

- Queue a bounded default-padding profile candidate.
- Fetch it with `/poll-candidate`.
- Verify opaque candidate handle shape.
- Verify bucket/pad metadata and zero-padding strip.
- Ack only after verification.
- Prove exactly one candidate is deleted.

## Invalid Identifier Requirements

- Malformed ack handles reject.
- Stale ack handles reject.
- Stale peer/session receive-side handles reject.
- Rejected identifier paths must preserve remote candidate state where a
  candidate exists.

## Invalid Padding Requirements

- Invalid padding config rejects before config creation.
- Malformed padded candidate rejects before actor decode.
- Rejected padding paths must preserve remote candidate state.

## No-Remote-Delete Requirements

After invalid identifier or padding reject, a fresh `/poll-candidate` must
return the same candidate ack id. Valid ack after verification may remove only
the verified candidate.

## No-Local-Output / State Requirements

The harness must byte-compare qshield local state before and after invalid
receive rejects and assert no accepted `from <peer>:` plaintext output appears.

## No-Secret-Leak Requirements

Reject output must be scanned for relay token, candidate ack id, raw peer
handle sentinel, plaintext sentinel, padding sentinel, panic, and backtrace.

## Successor-Selection Requirements

If bounded qshield embedded relay identifier/default-padding proof succeeds,
the selected successor is:

`NA-0320 -- Metadata Runtime Sanitized Errors and Retention/Purge Executable Harness`

If identifier or padding proof blocks, select the exact blocker-resolution
successor instead.

## Claim-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay boundary only;
- no qsl-server production relay proof;
- bounded candidate-handle proof is not broad identifier rotation;
- bounded padding proof is not metadata-free behavior;
- no production/public-internet/external-review/anonymity claim.

## Backup-Impact Requirements

Record whether changes create durable artifacts outside the qsl-protocol
worktree or existing Codex response path. Expected result: no backup-plan
update required.

## Required Local Checks

Run or record exact blocker:

- `cargo fmt --package qshield-cli -- --check`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
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
separate closeout may mark NA-0319 DONE and restore:

`NA-0320 -- Metadata Runtime Sanitized Errors and Retention/Purge Executable Harness`

The closeout must not implement NA-0320.
