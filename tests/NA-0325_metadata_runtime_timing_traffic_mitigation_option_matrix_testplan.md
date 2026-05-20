Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0325 Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix Testplan

## Objective

Validate that NA-0325 creates a mitigation option matrix grounded in NA-0324
instrumentation evidence without implementing runtime mitigation.

## Protected Invariants

- NA-0325 is analysis/design evidence only.
- Instrumentation remains measurement evidence, not mitigation.
- Proposed mitigations are future options, not current behavior.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server production timing remains unproven.
- qsl-attachments production timing remains unproven.
- Timing metadata and traffic shape are not claimed hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix.md`
- `tests/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime mitigation implementation.
- Instrumentation implementation.
- Jitter, batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, transport padding, retry normalization, service deployment, or
  production behavior implementation.
- qshield runtime source changes.
- qsl-server or qsl-attachments changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Dependency, Cargo, workflow, website, README, START_HERE,
  branch-protection, or public-safety configuration changes.
- NA-0326 implementation.

## Prior Instrumentation Review Requirements

The evidence must review and preserve:

- live NA-0325 scope;
- inherited NA-0324 instrumentation evidence;
- inherited NA-0323 mitigation design context;
- inherited NA-0322 measurement evidence;
- inherited NA-0321 threat model boundaries;
- qshield embedded relay/demo boundary;
- qsl-server and qsl-attachments production timing boundaries;
- implementation authorization boundary.

## Option Inventory Requirements

The inventory must include:

1. fixed interval sender cadence;
2. fixed interval receiver polling cadence;
3. bounded jitter on send/receive;
4. batching by count;
5. batching by time window;
6. queue drain scheduling;
7. retry cadence normalization;
8. padding bucket expansion;
9. attachment-size class handling;
10. local-demo-only mitigation;
11. qsl-server production relay timing mitigation;
12. qsl-attachments production object timing/size mitigation;
13. cover traffic;
14. hybrid staged mitigation.

For each option, record what it addresses, what it does not address, future
files/scope, abuse and DoS risk, latency cost, bandwidth/storage cost,
correctness risk, CI/test burden, backup impact, external-review sensitivity,
claim boundary, and recommendation.

## Risk / Cost / Abuse Matrix Requirements

The matrix must include:

- option;
- metadata benefit;
- timing benefit;
- traffic-shape benefit;
- abuse risk;
- DoS risk;
- latency cost;
- bandwidth cost;
- storage cost;
- reliability risk;
- correctness risk;
- implementation scope;
- testing scope;
- external-review sensitivity;
- production boundary;
- claim boundary;
- recommended successor relation.

The evidence must explain:

- why cover traffic is high-risk/high-cost and not a first implementation;
- why qsl-server/qsl-attachments production mitigation requires cross-repo
  authorization;
- why qshield local/demo mitigation is not production proof;
- why mitigation does not equal metadata-free behavior.

## Candidate Prioritization Requirements

Rank candidates by:

- security value;
- metadata-reduction value;
- implementation minimality;
- testability;
- abuse resistance;
- reversibility;
- compatibility;
- CI cost;
- claim safety;
- external-review readiness.

The selected NA-0326 successor must be exact.

## Marker-Plan Requirements

The future marker plan must include shared boundary markers:

- `NA0326_MITIGATION_OPTION_MATRIX_OK`
- `NA0326_SELECTED_MITIGATION_SCOPE_OK`
- `NA0326_MEASUREMENT_BEFORE_MITIGATION_OK`
- `NA0326_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0326_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0326_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0326_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0326_NO_METADATA_FREE_CLAIM_OK`

If retry cadence normalization is selected, include:

- `NA0326_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0326_RETRY_ABUSE_BOUNDARY_OK`
- `NA0326_INVALID_RETRY_COMPATIBILITY_OK`

## Production-Boundary Requirements

Evidence and PR text must state:

- qshield embedded relay/demo mitigation is local/demo proof only;
- qsl-server production relay timing requires cross-repo authorization;
- qsl-attachments production object-size/timing requires cross-repo
  authorization;
- production service timing is not claimed;
- public-internet timing is not claimed.

## Claim-Boundary Requirements

Evidence and PR text must state:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production/public-internet readiness claim;
- no external-review-complete claim;
- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim that runtime mitigation was implemented;
- all runtime gaps remain visible.

## Backup-Impact Requirements

Record whether durable evidence or artifacts were created outside current
backup scope. Expected result: no backup-plan update required if tracked
evidence remains under `/srv/qbuild/work` and no durable external artifact is
created.

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

Required checks must pass before merge, including `public-safety`. Cost-control
skips are acceptable only when CI reports them as skipped and public-safety
remains green.

## Successor Handoff

If the matrix completes, select and hand off exactly one NA-0326 successor
without implementing it. The selected successor is expected to be:

`NA-0326 -- Metadata Runtime qshield Demo Retry Cadence Normalization Authorization Plan`
