Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0320 Metadata Runtime Sanitized Errors and Retention/Purge Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate executable qshield embedded-relay/demo proof for sanitized-error and
retention/purge runtime behavior after the NA-0318 ack/commit proof and the
NA-0319 identifier/default-padding proof.

## Protected Invariants

- Invalid sanitized-error and retention/purge rejects must not leak route
  tokens, raw handles, candidate ack ids, plaintext sentinels, padding
  sentinels, passphrases, raw key material, panic text, backtraces, or
  secret-bearing diagnostics.
- Invalid receive and attachment receive rejects must not delete a remote
  queued candidate in the qshield embedded relay candidate/ack boundary.
- Invalid rejects must not create accepted local state or output.
- Valid ack/commit may delete exactly one queued candidate after local
  verification.
- qshield embedded relay proof must not be presented as qsl-server or
  qsl-attachments production proof.
- No anonymity, metadata-free, untraceable, public-internet readiness,
  production readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`
- `docs/governance/evidence/NA-0320_metadata_runtime_sanitized_errors_retention_purge_harness.md`
- `tests/NA-0320_metadata_runtime_sanitized_errors_retention_purge_harness_testplan.md`
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
- NA-0321 implementation.

## Prior Metadata-Runtime Review Requirements

Review and preserve:

- NA-0318 qshield `/poll-candidate` and `/ack` proof;
- NA-0319 qshield identifier/default-padding proof;
- NA-0293 policy-fixture sanitized-error and retention/purge proof;
- qsl-server and qsl-attachments production boundaries.

## Implementation Requirements

- Prefer an executable harness when current qshield runtime behavior already
  supports truthful proof.
- Make only minimal authorized qshield runtime changes if a reject path leaks
  forbidden data.
- Stop if proof requires qsl-server, qsl-attachments, qsc/qsp, protocol,
  crypto, dependency, workflow, or public-safety changes.

## Harness Marker Requirements

The executable harness must declare and emit:

- `NA0320_SANITIZED_ERROR_POLICY_OK`
- `NA0320_RECV_INVALID_HANDLE_ERROR_REDACTED_OK`
- `NA0320_RECV_INVALID_PADDING_ERROR_REDACTED_OK`
- `NA0320_RECV_DECODE_ERROR_REDACTED_OK`
- `NA0320_ATTACHMENT_ERROR_REDACTED_OK`
- `NA0320_NO_ROUTE_TOKEN_LEAK_OK`
- `NA0320_NO_CANDIDATE_ACK_ID_LEAK_OK`
- `NA0320_NO_PLAINTEXT_SENTINEL_LEAK_OK`
- `NA0320_NO_PADDING_SENTINEL_LEAK_OK`
- `NA0320_NO_PANIC_BACKTRACE_LEAK_OK`
- `NA0320_RETENTION_PURGE_POLICY_OK`
- `NA0320_VALID_ACK_PURGES_ONE_CANDIDATE_OK`
- `NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK`
- `NA0320_STALE_ACK_FAIL_CLOSED_OK`
- `NA0320_REPEATED_INVALID_RECV_BOUNDED_OK`
- `NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK`
- `NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK`
- `NA0320_LOCAL_ARTIFACT_CLEANUP_OK`
- `NA0320_QSHIELD_EMBEDDED_RELAY_RETENTION_BOUNDARY_OK`
- `NA0320_METADATA_RUNTIME_SANITIZED_RETENTION_OK`

## Valid Path Requirements

- Queue at least two relay candidates.
- Fetch candidates with `/poll-candidate` without deletion.
- Locally verify the candidate shape before ack.
- Ack one candidate and prove exactly one candidate is removed.

## Invalid Sanitized-Error Requirements

- Exercise invalid handle, invalid padding, malformed decode, malformed ack,
  invalid padding metadata, and attachment receive reject paths.
- Assert reject diagnostics are coarse and do not include forbidden sentinels.

## Invalid Retention/Purge Requirements

- Invalid receive rejects must leave the same candidate available.
- Malformed ack rejects must leave the queue unchanged.
- Invalid padding metadata rejects must not add or delete queued candidates.
- Stale ack must fail closed and leave the remaining candidate intact.

## No-Remote-Delete Requirements

After each invalid receive or attachment reject, a fresh `/poll-candidate` must
return the same candidate ack id. Legacy destructive `/poll` must not be used
for no-delete proof.

## No-Local-Output/State Requirements

The harness must byte-compare qshield local state before and after invalid
receive/attachment receive rejects and assert no accepted `from <peer>:` output
or attachment output directory is created.

## No-Secret-Leak Requirements

Reject output must be scanned for route token, raw handle, candidate ack id,
plaintext sentinel, padding sentinel, attachment sentinel, passphrase sentinel,
raw key sentinel, panic text, and backtrace text.

## Successor-Selection Requirements

If bounded qshield embedded relay sanitized-error and retention/purge proof
succeeds, select:

`NA-0321 -- Metadata Runtime Timing and Traffic-Shape Threat Model / Executable Evidence Plan`

If sanitized-error or retention/purge proof blocks, select the exact
blocker-resolution successor instead.

## Claim-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay/demo boundary only;
- no qsl-server production relay proof;
- no qsl-attachments production proof;
- no production/public-internet/external-review/anonymity/metadata-free or
  untraceable claim;
- remaining timing/traffic-shape and deployment metadata gaps are visible.

## Backup-Impact Requirements

Record whether changes create durable artifacts outside the qsl-protocol
worktree or existing Codex response path. Expected result: no backup-plan
update required.

## Required Local Checks

Run or record exact blocker:

- `cargo fmt --package qshield-cli -- --check`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
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
- queue/decision/scope/link/leak/goal-lint checks

## CI Expectations

The PR must keep required checks green, including `public-safety`. Runtime
critical qshield changes should run qshield tests/build and demo smoke/stress
checks. Do not merge with required red or missing checks.

## Successor Handoff

After implementation PR merge and post-merge `public-safety` success, a
separate closeout may mark NA-0320 DONE and restore:

`NA-0321 -- Metadata Runtime Timing and Traffic-Shape Threat Model / Executable Evidence Plan`

The closeout must not implement NA-0321.
