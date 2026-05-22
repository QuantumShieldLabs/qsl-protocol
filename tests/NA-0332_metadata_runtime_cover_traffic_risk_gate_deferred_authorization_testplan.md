Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0332 Metadata Runtime Cover Traffic Risk Gate Deferred Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0332 produces only a cover-traffic risk gate and deferred
authorization plan, preserves inherited qshield demo batching, retry-cadence,
bounded-jitter, production-boundary, backup-impact, and claim-boundary
invariants, and selects one exact NA-0333 successor without implementing cover
traffic.

## Protected Invariants

- NA-0332 is risk-gate/design only.
- No cover traffic implementation or prototype is included.
- No batching, bounded-jitter, retry-cadence, broad scheduling, send
  scheduling, receive scheduling, transport padding, or runtime timing
  mitigation implementation changes are included.
- qshield embedded relay/demo evidence remains local/demo only.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- Cover traffic remains high-risk/high-cost unless future evidence proves a
  narrower bounded result.
- No claim says timing metadata is hidden.
- No claim says traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.
- No qshield runtime, qsl-server, qsl-attachments, qsc/qsp/protocol/crypto,
  key-schedule, dependency, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration change is included.

## Allowed Scope

- `docs/governance/evidence/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization.md`
- `tests/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- cover traffic implementation;
- cover traffic prototype;
- batching implementation changes;
- bounded-jitter implementation changes;
- retry-cadence implementation changes;
- runtime timing mitigation implementation;
- broad queue scheduling, send scheduling, receive scheduling, or transport
  padding implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule implementation changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, qsc-desktop,
  branch-protection, or public-safety configuration changes;
- NA-0333 implementation.

## Prior Batching Review Requirements

Evidence must review NA-0331 and confirm:

- batching is opt-in qshield embedded relay/demo only;
- `qshield_demo_batching_v1` is capped at four send, receive candidate, and
  ack members;
- send and ack batches validate all-or-nothing;
- receive fail-closed behavior prevents remote delete before local
  verification on invalid candidates;
- valid single-message behavior remains unchanged;
- retry-cadence and bounded-jitter caps remain preserved;
- qsl-server and qsl-attachments production behavior is not proven.

## Cover Traffic Threat / Value Model Requirements

Evidence must state threats cover traffic could address:

- idle/active timing observability;
- traffic-volume observability;
- queue-drain observability;
- burst correlation;
- route/contact correlation in bounded classes;
- attachment timing/size correlation only in a future qsl-attachments lane.

Evidence must state threats cover traffic does not solve:

- endpoint compromise;
- production logging;
- route/contact relationship leakage;
- content/key compromise;
- broad unsupported metadata-free behavior;
- public-internet deployment readiness.

## Risk / Cost / Abuse / Ops Matrix Requirements

The matrix must cover:

- no cover traffic;
- demo-only synthetic cover;
- active-session cover only;
- fixed-rate cover;
- batch-fill cover;
- attachment-size cover;
- production relay cover;
- production attachment cover;
- adversarial cover amplification;
- quota exhaustion;
- backup growth;
- public-internet deployment.

The matrix must analyze:

- threat addressed;
- threat not addressed;
- abuse risk;
- DoS risk;
- bandwidth cost;
- storage cost;
- latency cost;
- CPU/memory cost;
- backup impact;
- retention/purge impact;
- observability/logging risk;
- deployment complexity;
- qshield demo feasibility;
- qsl-server/qsl-attachments production feasibility;
- external-review sensitivity;
- claim boundary;
- recommendation.

## Defer / Reject / Authorize Requirements

Evidence must decide whether cover traffic:

- remains deferred;
- is rejected for current phase;
- requires a cost/quota/retention prerequisite lane;
- permits or rejects a bounded qshield demo-only prototype authorization lane;
- requires qsl-server/qsl-attachments cross-repo service authorization before
  production behavior.

The decision must preserve:

- no current cover traffic implementation;
- no claim of hidden timing or hidden traffic shape;
- no metadata-free, anonymity, or untraceable claim;
- no production or public-internet claim.

## Marker-Plan Requirements

For selected NA-0333, evidence must include:

- `NA0333_COVER_TRAFFIC_COST_QUOTA_PLAN_OK`
- `NA0333_BANDWIDTH_STORAGE_BOUNDARY_OK`
- `NA0333_BACKUP_IMPACT_BOUNDARY_OK`
- `NA0333_ABUSE_DOS_BOUNDARY_OK`
- `NA0333_COVER_TRAFFIC_COST_MODEL_REQUIRED_OK`
- `NA0333_COVER_TRAFFIC_QUOTA_MODEL_REQUIRED_OK`
- `NA0333_COVER_TRAFFIC_RETENTION_MODEL_REQUIRED_OK`
- `NA0333_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`
- `NA0333_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0333_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0333_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0333_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0333_NO_METADATA_FREE_CLAIM_OK`
- `NA0333_NO_ANONYMITY_CLAIM_OK`

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo cover traffic would be local/demo only;
- qsl-server production cover traffic requires cross-repo authorization;
- qsl-attachments production object cover requires cross-repo authorization;
- public-internet timing remains future-gated;
- cover traffic can create cost, abuse, backup, retention, and deployment
  burdens.

## Claim-Boundary Requirements

Evidence, decision, traceability, journal, PR body, and response must not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- cover traffic is implemented;
- prohibited metadata-free behavior claim;
- prohibited anonymity claim;
- prohibited untraceable behavior claim;
- production readiness;
- public-internet readiness;
- external-review completion.

Any mention of those phrase families must be negated, prohibited,
future-gated, or boundary-only.

## Backup-Impact Requirements

- Changed evidence must remain under qsl-protocol paths covered by
  `/srv/qbuild/work`.
- No durable evidence location outside current backup scope may be created.
- No runtime artifacts may be created as part of NA-0332.
- The D132 preservation bundle must not be deleted or modified.
- If a future durable cover-traffic evidence location is required, the lane
  must stop and recommend a backup-plan update.

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

- PR body includes a standalone `Goals: G1, G2, G3, G4, G5` line near the top.
- PR body states NA-0332 is risk-gate/design-only.
- Required checks complete green before merge.
- `public-safety` remains required.
- Merge uses normal merge with `--match-head-commit`.
- No branch deletion flag is used.
- Post-merge public-safety completes success.

## Successor Handoff

Selected successor:

`NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`

The closeout directive may restore NA-0333 only after the NA-0332 risk-gate PR
merges and post-merge public-safety is green. The closeout must not implement
NA-0333.
