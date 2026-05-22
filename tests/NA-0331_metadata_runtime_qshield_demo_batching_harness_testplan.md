Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0331 Metadata Runtime qshield Demo Batching Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0331 implements only bounded qshield embedded relay/demo
batching and executable proof, preserves inherited ack/commit, retry-cadence,
bounded-jitter, sanitized-error, retention, identifier, padding, and
production-boundary invariants, and selects one exact NA-0332 successor without
implementing NA-0332.

## Protected Invariants

- qshield embedded relay/demo batching is opt-in and local/demo only.
- Default single-message behavior remains unchanged when batching is disabled.
- Valid batches deliver deterministically.
- Batch ordering is preserved or fails closed.
- Invalid send and ack batch members cause all-or-nothing rejection.
- Invalid receive candidates cause no remote delete before local verification.
- Invalid receive candidates create no accepted local state and no plaintext output.
- Retry-cadence caps from NA-0327 remain unchanged.
- Bounded-jitter caps from NA-0329 remain unchanged.
- No secret batch artifacts or logs are created.
- qsl-server and qsl-attachments production behavior remains unproven and
  cross-repo-gated.
- No claim that timing metadata or traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim.

## Allowed Scope

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- `docs/governance/evidence/NA-0331_metadata_runtime_qshield_demo_batching_harness.md`
- `tests/NA-0331_metadata_runtime_qshield_demo_batching_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Cargo or dependency changes.
- Workflow, branch-protection, public-safety, website, README, START_HERE,
  docs/public, qsc-desktop, formal, input, tools/refimpl, or service changes.
- Cover traffic implementation.
- Transport padding expansion.
- Broad queue scheduling.
- Production service timing behavior.
- NA-0332 implementation.

## Prior Authorization Review Requirements

Before merge, confirm:

- live `NEXT_ACTIONS.md` shows READY NA-0331;
- NA-0330 is DONE;
- D-0642 exists exactly once;
- D-0643 exists exactly once;
- D-0644 was absent at startup and exists exactly once after the patch;
- NA-0330 authorized only qshield embedded relay/demo batching;
- NA-0327 retry cadence and NA-0329 bounded jitter are preserved;
- qsl-server and qsl-attachments production timing remain unproven.

## Batching Implementation Requirements

The implementation must provide:

- policy `qshield_demo_batching_v1`;
- opt-in `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- maximum batch size 4;
- maximum send-side batch window 750 ms or a stricter no-wait implementation;
- send-batch all-or-nothing validation before mutation;
- receive candidate batches of at most 4 under batching mode;
- ack batches of at most 4 locally verified ack IDs;
- ack-batch all-or-nothing validation before deletion;
- no default behavior drift when batching is disabled.

## Harness Marker Requirements

The executable harness must declare and emit:

- `NA0331_BATCHING_AUTHORIZATION_OK`
- `NA0331_BATCH_POLICY_OK`
- `NA0331_DETERMINISTIC_TEST_BATCHING_OK`
- `NA0331_VALID_SINGLE_MESSAGE_UNCHANGED_OK`
- `NA0331_VALID_BATCH_DELIVERY_OK`
- `NA0331_BATCH_ORDERING_PRESERVED_OK`
- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`
- `NA0331_PARTIAL_INVALID_BATCH_FAIL_CLOSED_OK`
- `NA0331_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0331_BOUNDED_JITTER_STILL_BOUNDED_OK`
- `NA0331_NO_ACCEPTED_STATE_ON_INVALID_BATCH_OK`
- `NA0331_NO_OUTPUT_ON_INVALID_BATCH_OK`
- `NA0331_NO_SECRET_BATCH_ARTIFACT_OK`
- `NA0331_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0331_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0331_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0331_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0331_NO_METADATA_FREE_CLAIM_OK`
- `NA0331_METADATA_RUNTIME_BATCHING_OK`

## Abuse / DoS / Latency Requirements

- Empty send and ack batches reject.
- Oversize send and ack batches reject.
- Existing relay body, total queue, per-token queue, and per-recipient queue
  caps remain authoritative.
- Send-batch capacity is checked before mutation.
- Ack-batch membership is checked before deletion.
- Deterministic test mode avoids sleeps and timing races.

## Valid Path Requirements

- Normal `/send` plus `/ack` succeeds when batching is disabled.
- `/send-batch` is unavailable when batching is disabled.
- A valid four-member `/send-batch` queues exactly four candidates.
- Candidate ordering matches request ordering.
- Valid `/ack-batch` deletes exactly verified candidates.
- Remaining candidates preserve order after partial valid ack batches.

## Invalid Batch Requirements

- Invalid send-batch member rejects the whole batch and queues nothing.
- Stale ack inside ack-batch rejects the whole batch and deletes nothing.
- Duplicate ack inside ack-batch rejects the whole batch and deletes nothing.
- Invalid receive candidate in batching mode stops processing and leaves the
  invalid and later candidates queued.

## No-Remote-Delete Requirements

- Invalid receive attempts must leave the same candidate ack IDs queued.
- Invalid ack-batch attempts must leave valid candidates queued.
- Remote delete remains after local verification only.

## No-Local-Output / State Requirements

- Invalid receive attempts must leave `state.json` unchanged.
- Invalid receive output must not include sender plaintext output.
- Invalid receive output must not include plaintext sentinels.

## No-Secret-Artifact Requirements

Scan command output and ledgers for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel;
- actual relay token;
- actual ack IDs;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace text;
- sensitive local path prefixes.

Required result:

- `BATCH_ARTIFACT_SECRET_FINDING_COUNT 0`

## Production-Boundary Requirements

- Evidence must state qshield embedded relay/demo boundary exactly.
- Evidence must state qsl-server production batching/timing remains unproven.
- Evidence must state qsl-attachments timing/object behavior remains unproven.
- No qsl-server or qsl-attachments file may change.

## Claim-Boundary Requirements

Evidence, decision, traceability, journal, PR body, and response must not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion;
- qshield local/demo proof is production-service proof.

Any mention of those phrase families must be negated, prohibited, future-gated,
or boundary-only.

## Backup-Impact Requirements

- Tracked evidence must remain under qsl-protocol paths covered by
  `/srv/qbuild/work`.
- Runtime artifacts must remain temporary and removable.
- No durable evidence location outside current backup scope may be created.
- The D132 preservation bundle must not be deleted or modified.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0331_metadata_runtime_batching -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0329_metadata_runtime_bounded_jitter -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0327_metadata_runtime_retry_cadence_normalization -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh`
- `bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `bash scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `bash scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted NA-0310 refimpl oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- queue/decision checks
- scope guard with exact allowed paths
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof for changed paths
- goal-lint-compatible PR body proof

## CI Expectations

Required PR checks must pass normally before merge, including
`public-safety`. Cost-control skips are acceptable only when CI reports them as
skipped and public-safety remains green. No admin bypass, squash, rebase,
direct push, branch deletion command, branch-protection mutation, or
public-safety mutation is allowed.

## Successor Handoff

If NA-0331 merges and post-merge public-safety is green, closeout may restore
exactly one successor:

`NA-0332 -- Metadata Runtime Cover Traffic Risk Gate and Deferred Authorization Plan`

The closeout must not implement NA-0332.
