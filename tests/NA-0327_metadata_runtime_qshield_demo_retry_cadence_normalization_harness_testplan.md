Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0327 Metadata Runtime qshield Demo Retry Cadence Normalization Harness Testplan

## Objective

Validate bounded qshield embedded relay/demo retry-cadence normalization and
its executable harness after NA-0326 authorized the lane.

## Protected Invariants

- Retry-cadence normalization is bounded to qshield embedded relay/demo.
- qsl-server production timing remains unproven.
- qsl-attachments production timing remains unproven.
- No remote candidate is deleted before local verification.
- Valid ack deletes exactly one candidate.
- Stale and duplicate ack attempts fail closed or deterministic no-op.
- Invalid retry cadence is bounded.
- Empty poll cadence is bounded when represented.
- Invalid retry creates no accepted local state and no plaintext output.
- Retry artifacts and logs contain no secrets.
- No unsupported privacy, production, public-internet, or external-review
  claim is introduced.

## Allowed Scope

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `tests/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Dependency, Cargo, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration changes.
- Jitter, batching, cover traffic, transport padding expansion, send
  scheduling, production service timing behavior, or NA-0328 implementation.

## Prior Authorization Review Requirements

Review NA-0326 authorization evidence, NA-0325 option matrix, NA-0324
instrumentation evidence, NA-0322 measurement evidence, and NA-0318 through
NA-0320 qshield metadata-runtime invariants before editing.

## Retry-Cadence Implementation Requirements

- Policy name is `qshield_demo_retry_cadence_v1`.
- Invalid candidate retry uses a 60 second local window.
- Invalid candidate processing is capped at four attempts per local candidate
  tag per window.
- Backoff classes are immediate first attempt, then 500 ms, 1000 ms, and 2000
  ms capped.
- Empty poll retries use the same capped classes without failing merely
  because the queue is empty.
- Deterministic test mode must avoid long sleeps.
- The retry ledger must avoid raw route tokens, raw handles, raw ack IDs, raw
  candidate IDs, plaintext, padding sentinels, passphrases, and raw key
  material.

## Harness Marker Requirements

The executable harness must truthfully emit:

- `NA0327_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0327_RETRY_NORMALIZATION_POLICY_OK`
- `NA0327_INVALID_RETRY_BOUNDED_OK`
- `NA0327_EMPTY_POLL_RETRY_BOUNDED_OK`
- `NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK`
- `NA0327_DUPLICATE_ACK_RETRY_FAIL_CLOSED_OK`
- `NA0327_VALID_ACK_ONCE_OK`
- `NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK`
- `NA0327_NO_SECRET_RETRY_ARTIFACT_OK`
- `NA0327_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0327_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0327_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0327_NO_METADATA_FREE_CLAIM_OK`
- `NA0327_METADATA_RUNTIME_RETRY_CADENCE_NORMALIZATION_OK`

## Abuse / DoS / Latency Requirements

- Repeated invalid candidate work must be capped.
- Empty polls must not create unbounded retry loops.
- Deterministic test mode must avoid long sleeps in CI.
- Cap failures must be fail-closed and coarse.
- Valid-message compatibility must remain intact.

## Valid Path Requirements

- Candidate fetch before ack must not delete the candidate.
- Valid ack must delete exactly one queued candidate.
- A remaining queued candidate must survive duplicate/stale ack attempts.

## Invalid Retry Requirements

- Invalid candidate retries must record 0, 500, 1000, and 2000 ms classes.
- The fifth invalid attempt in-window must fail closed at the cap.
- The remote candidate must remain queued.
- `state.json` must remain unchanged.
- No plaintext output is produced.

## Stale / Duplicate Ack Requirements

- Duplicate ack after a valid commit fails closed or no-ops.
- Stale syntactically valid ack fails closed or no-ops.
- Neither path may delete a remaining candidate.

## No-Remote-Delete Requirements

Repeated candidate fetch and invalid receive must preserve the queued candidate
until a valid ack is submitted after local verification.

## No-Local-Output / State Requirements

Invalid retry must not mutate accepted qshield store state and must not print a
`from <peer>:` plaintext output line.

## No-Secret-Artifact Requirements

Scan retry ledgers and command output for route token sentinel, raw handle
sentinel, candidate/ack sentinel, concrete ack ID, plaintext sentinel, padding
sentinel, passphrase sentinel, raw key sentinel, panic/backtrace text, and
sensitive absolute local path prefixes.

Expected result:

- `RETRY_ARTIFACT_SECRET_FINDING_COUNT 0`

## Production-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay/demo only;
- qsl-server production timing remains unproven and cross-repo-gated;
- qsl-attachments production timing remains unproven and cross-repo-gated;
- no service production behavior is changed.

## Claim-Boundary Requirements

Evidence and PR text must state no claim of:

- timing metadata hidden;
- traffic shape hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion.

## Backup-Impact Requirements

No backup-plan update is required if tracked files remain under qsl-protocol
and runtime retry ledgers remain temporary test/demo artifacts. The D132 bundle
must not be deleted.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0327_metadata_runtime_retry_cadence_normalization -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh` if feasible
- metadata runtime, identifier/padding, sanitized-error/retention, and
  conformance smoke harnesses
- qsc send_commit, formal model checks, NA-0310 vector/refimpl checks, and
  qsc NA-0313 harness if directly runnable
- queue, decisions, scope guard, link-check, leak-scan, classifier proof,
  changed-line overclaim scan, and goal-lint

## CI Expectations

Required PR checks must pass normally before merge, including `public-safety`.
Cost-control skips are acceptable only when reported by CI and public-safety
remains green.

## Successor Handoff

If NA-0327 merges and post-merge public-safety is green, the closeout should
restore exactly one READY successor:

`NA-0328 -- Metadata Runtime qshield Demo Bounded Jitter Authorization Plan`
