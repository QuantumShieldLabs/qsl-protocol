Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0328 Metadata Runtime qshield Demo Bounded Jitter Authorization Testplan

## Objective

Validate that NA-0328 authorizes a future bounded qshield embedded relay/demo
jitter implementation harness without implementing jitter, runtime mitigation,
service behavior, protocol behavior, dependency changes, or workflow changes.

## Protected Invariants

- NA-0328 is authorization/design only.
- No bounded jitter implementation is included.
- No runtime timing mitigation is implemented.
- Measurement and instrumentation evidence remain distinct from mitigation.
- Retry-cadence normalization remains bounded qshield embedded relay/demo
  evidence.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server production timing remains unproven.
- qsl-attachments production timing remains unproven.
- Timing metadata and traffic shape are not claimed hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0328_metadata_runtime_qshield_demo_bounded_jitter_authorization.md`
- `tests/NA-0328_metadata_runtime_qshield_demo_bounded_jitter_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Bounded jitter implementation.
- Runtime timing mitigation implementation.
- qshield runtime changes.
- qsl-server or qsl-attachments changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Cargo or dependency changes.
- Workflow, website, README, START_HERE, branch-protection, or public-safety
  configuration changes.
- NA-0329 implementation.

## Evidence Section Requirements

The evidence must include:

- executive summary;
- live NA-0328 scope;
- inherited NA-0327 retry-cadence proof;
- inherited NA-0326/NA-0325 jitter context;
- sources inspected;
- bounded jitter semantic design;
- future implementation boundary;
- abuse, DoS, latency, and compatibility matrix;
- future validation and marker plan;
- production boundary;
- external-review sensitivity;
- public claim boundary;
- selected successor;
- rejected alternatives;
- backup-plan impact statement;
- next recommendation.

## Bounded Jitter Semantic Requirements

The evidence must define:

- a named future local/demo policy;
- an opt-in switch;
- deterministic test mode;
- non-secret deterministic test seed handling;
- exact eligible event classes;
- minimum and maximum jitter bounds;
- retry-cadence interaction bounds;
- ack-after-verify boundary;
- valid receive output boundary;
- invalid retry boundary;
- failure posture for state or deterministic evidence errors.

## Future Implementation-Boundary Requirements

The evidence must list future allowed qshield files and future forbidden files.
It must require proof that:

- valid send and receive still succeed;
- selected jitter delay is inside bounds;
- deterministic test mode is reproducible without long sleeps;
- retry cadence and jitter composition preserves the invalid-attempt cap;
- no candidate is remotely deleted before local verification succeeds;
- valid ack deletes exactly one candidate after verification;
- stale and duplicate ack fail closed;
- invalid retry creates no accepted state and no plaintext output;
- jitter artifacts and logs contain no secrets;
- qsl-server and qsl-attachments production boundaries remain explicit.

## Abuse/DoS/Latency Matrix Requirements

The matrix must include:

- valid send with jitter enabled;
- candidate poll with jitter enabled;
- empty poll plus retry cadence;
- invalid candidate retry plus jitter;
- valid ack after verify;
- duplicate/stale ack;
- multiple queued candidates;
- deterministic CI;
- secret-bearing artifacts;
- qsl-server production equivalent as future-gated;
- qsl-attachments production equivalent as future-gated;
- cover traffic or batching escalation as forbidden.

For each scenario, record risk, proposed bound, future test, failure mode, stop
condition, compatibility impact, and claim boundary.

## Marker-Plan Requirements

The future marker plan must include:

- `NA0329_BOUNDED_JITTER_AUTHORIZATION_OK`
- `NA0329_JITTER_POLICY_BOUND_OK`
- `NA0329_DETERMINISTIC_JITTER_TEST_MODE_OK`
- `NA0329_SEND_JITTER_BOUNDS_OK`
- `NA0329_RECEIVE_POLL_JITTER_BOUNDS_OK`
- `NA0329_RETRY_CADENCE_INTERACTION_OK`
- `NA0329_VALID_SEND_RECEIVE_COMPATIBILITY_OK`
- `NA0329_ACK_AFTER_VERIFY_UNCHANGED_OK`
- `NA0329_VALID_ACK_ONCE_OK`
- `NA0329_STALE_DUPLICATE_ACK_FAIL_CLOSED_OK`
- `NA0329_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0329_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0329_NO_OUTPUT_ON_INVALID_RETRY_OK`
- `NA0329_NO_SECRET_JITTER_ARTIFACT_OK`
- `NA0329_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0329_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0329_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0329_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0329_NO_METADATA_FREE_CLAIM_OK`
- `NA0329_METADATA_RUNTIME_BOUNDED_JITTER_OK`

## Production-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay/demo bounded jitter is local/demo only;
- qsl-server production jitter or timing mitigation requires a cross-repo lane;
- qsl-attachments production timing and object-size class behavior require a
  cross-repo lane;
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
- no claim that runtime jitter was implemented;
- all runtime gaps remain visible.

## Backup-Impact Requirements

Record whether durable evidence or artifacts were created outside current
backup scope. Expected result: no backup-plan update required if tracked
evidence remains under `/srv/qbuild/work` and no durable external artifact is
created. The preserved D132 bundle must not be deleted.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0327_metadata_runtime_retry_cadence_normalization -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1` if feasible
- `cargo +stable build -p qshield-cli --locked` if feasible
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

If the authorization plan completes, select exactly one NA-0329 successor
without implementing it. The selected successor is:

`NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`
