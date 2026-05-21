Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0326 Metadata Runtime qshield Demo Retry Cadence Normalization Authorization Testplan

## Objective

Validate that NA-0326 authorizes a future bounded qshield embedded relay/demo
retry-cadence normalization implementation harness without implementing runtime
retry normalization or any other timing mitigation.

## Protected Invariants

- NA-0326 is authorization/design only.
- No retry-cadence normalization is implemented in NA-0326.
- No timing mitigation is implemented in NA-0326.
- Measurement remains distinct from mitigation.
- Instrumentation remains measurement evidence, not mitigation.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server production timing remains unproven.
- qsl-attachments production timing remains unproven.
- Timing metadata and traffic shape are not claimed hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0326_metadata_runtime_qshield_demo_retry_cadence_normalization_authorization.md`
- `tests/NA-0326_metadata_runtime_qshield_demo_retry_cadence_normalization_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime retry-cadence normalization implementation.
- Runtime timing mitigation.
- Instrumentation implementation.
- Jitter, batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, transport padding, qshield runtime implementation, qsl-server
  implementation, qsl-attachments implementation, qsc/qsp/protocol/crypto
  implementation, key-schedule changes, dependency changes, workflow changes,
  website or external website changes, README or START_HERE changes,
  branch-protection changes, and public-safety configuration changes.
- NA-0327 implementation.

## Prior Option-Matrix Review Requirements

The evidence must review:

- live NA-0326 scope;
- inherited NA-0325 mitigation option matrix;
- inherited NA-0324 instrumentation evidence;
- inherited NA-0323 design context;
- inherited NA-0322 measurement evidence;
- retry-cadence problem statement;
- production service boundary;
- implementation authorization boundary;
- stop conditions.

## Retry-Cadence Semantic Design Requirements

The evidence must define:

- invalid receive retry cadence semantics;
- candidate fetch retry cadence semantics;
- ack/commit retry cadence semantics;
- retry state scope;
- normalization policy constants;
- fixed minimum interval and bounded backoff;
- capped retry count;
- coarse retry classes;
- deterministic test mode;
- exact behavior that a future implementation may change;
- exact behavior that a future implementation must not change.

## Abuse/DoS/Latency Matrix Requirements

The matrix must include:

- one invalid queued message;
- repeated invalid queued message;
- repeated empty polling;
- stale ack replay;
- duplicate valid ack;
- slow valid receiver;
- multiple candidates;
- attachment candidate flow;
- local demo stress;
- production qsl-server equivalent as future-gated;
- qsl-attachments equivalent as future-gated.

For each scenario, record risk, proposed bound, future test, failure mode, stop
condition, compatibility impact, and claim boundary.

## Future Implementation-Boundary Requirements

The evidence must list future allowed qshield files and future forbidden files.
It must require proof that:

- valid message delivery still succeeds;
- invalid retry cadence is bounded;
- repeated invalid candidate fetches are normalized or bounded;
- stale ack retry is deterministic and fail-closed;
- no candidate is remotely deleted before local verification succeeds;
- no accepted state is created on invalid retry;
- no plaintext output is created on invalid retry;
- no secret retry artifacts are created;
- qsl-server and qsl-attachments production boundaries remain explicit.

## Marker-Plan Requirements

The future marker plan must include:

- `NA0327_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0327_RETRY_NORMALIZATION_POLICY_OK`
- `NA0327_INVALID_RETRY_BOUNDED_OK`
- `NA0327_EMPTY_POLL_RETRY_BOUNDED_OK`
- `NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK`
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

## Production-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay/demo retry-cadence normalization is local/demo only;
- qsl-server production retry-cadence normalization requires a cross-repo lane;
- qsl-attachments production retry and retention timing requires a cross-repo
  lane;
- public-internet timing remains future-gated;
- external review is recommended before stronger claims;
- no claim says timing metadata or traffic shape is hidden.

## Claim-Boundary Requirements

Evidence and PR text must state:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production/public-internet readiness claim;
- no external-review-complete claim;
- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim that runtime retry normalization was implemented;
- all runtime gaps remain visible.

## Backup-Impact Requirements

Record whether durable evidence or artifacts were created outside current
backup scope. Expected result: no backup-plan update required if tracked
evidence remains under `/srv/qbuild/work` and no durable external artifact is
created. The preserved D132 bundle and stash must not be deleted.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh` if feasible
- `bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `bash scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `bash scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- exact allowed-path scope guard
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- classifier proof for changed paths
- local goal-lint via synthetic PR event

## CI Expectations

Required PR checks must pass normally before merge, including `public-safety`.
Cost-control skips are acceptable only when CI reports them as skipped and
public-safety remains green.

## Successor Handoff

If the authorization plan completes, select exactly one NA-0327 successor
without implementing it. The selected successor is:

`NA-0327 -- Metadata Runtime qshield Demo Retry Cadence Normalization Implementation Harness`
