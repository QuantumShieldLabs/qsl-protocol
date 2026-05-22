# NA-0334 Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-22

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0334 is authorization/design only and decides whether a future
bounded qshield demo-only cover-traffic prototype implementation harness is
safe to authorize as NA-0335.

## Protected Invariants

- NA-0334 implements no cover traffic.
- NA-0334 implements no cover-traffic prototype.
- NA-0334 changes no qshield runtime source.
- NA-0334 changes no qsl-server or qsl-attachments production behavior.
- NA-0334 changes no batching, bounded-jitter, retry-cadence, broad queue
  scheduling, send scheduling, receive scheduling, or transport padding
  implementation.
- qshield embedded relay/demo evidence remains local/demo only.
- qsl-server and qsl-attachments production behavior remains unproven and
  cross-repo gated.
- No claim says timing metadata is hidden.
- No claim says traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.
- No dependency, Cargo, workflow, website, README, START_HERE, docs/public,
  qsc-desktop, branch-protection, or public-safety configuration change is
  included.

## Allowed Scope

- `docs/governance/evidence/NA-0334_metadata_runtime_qshield_demo_cover_traffic_prototype_authorization.md`
- `tests/NA-0334_metadata_runtime_qshield_demo_cover_traffic_prototype_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- cover traffic implementation;
- cover traffic prototype implementation;
- runtime timing mitigation implementation;
- batching implementation changes;
- bounded-jitter implementation changes;
- retry-cadence implementation changes;
- broad queue scheduling, send scheduling, receive scheduling, or transport
  padding implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule implementation changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, qsc-desktop,
  branch-protection, or public-safety configuration changes;
- NA-0335 implementation.

## Prior Prerequisite Review Requirements

Evidence must review NA-0333 and confirm:

- cost, bandwidth, storage, quota, abuse/DoS, retention, purge, backup,
  operations, deployment, qshield-demo, and service-production boundaries
  exist;
- qshield demo candidate caps are inherited exactly or narrowed;
- fixed-rate and production cover remain rejected or gated;
- NA-0334 remains authorization/design only.

Evidence must review NA-0332 and confirm:

- cover traffic was deferred behind a risk gate;
- direct cover-traffic implementation was rejected for the current phase;
- qsl-server and qsl-attachments production cover traffic remain cross-repo
  gated;
- timing and traffic-shape gaps remain explicit.

Evidence must review NA-0331 and confirm:

- batching is opt-in local/demo only;
- maximum batch size is four;
- send and ack batches validate all-or-nothing;
- invalid receive candidates do not cause remote delete before local
  verification;
- retry-cadence and bounded-jitter caps remain preserved;
- batching does not prove cover traffic or production-service behavior.

## Prototype Mode Decision Requirements

Evidence must evaluate:

- qshield demo synthetic local cover;
- qshield demo active-session cover;
- qshield demo batch-fill cover;
- qshield demo fixed-rate cover;
- qsl-server production relay cover;
- qsl-attachments production object cover;
- no prototype / defer.

The decision must account for:

- cost and cap fit from NA-0333;
- quota fit;
- retention/purge fit;
- backup/ops fit;
- abuse/DoS fit;
- deterministic testability;
- qshield demo value;
- production-service relevance;
- claim safety;
- external-review sensitivity.

## Future Implementation-Boundary Requirements

Evidence must define exact future allowed files for NA-0335 and must state that
NA-0334 creates no implementation files.

Required future proof:

- no real user message corrupted;
- cover items are locally distinguishable without secret leaks;
- cover items obey quota/cost caps;
- cover items purge by TTL, cap, failure, or run end;
- backup impact remains bounded;
- cover artifacts/logs are secret-free;
- batching, retry, and jitter semantics remain bounded;
- qsl-server/qsl-attachments production boundary is explicit;
- no claim says timing metadata or traffic shape is hidden.

## Cost / Quota / Retention / Abuse Boundary Requirements

Evidence must define a future boundary matrix covering:

- max payload per cover item;
- max cover items per run;
- max cover items per minute;
- max cover items per hour;
- max cover items queued globally;
- max cover items queued per route;
- max retained cover artifacts;
- max retained bytes;
- TTL;
- purge triggers;
- low disk threshold;
- abuse stop;
- deterministic test mode;
- no production default;
- no public-internet default;
- no service production behavior.

Required inherited caps:

- no more than 8192 bytes payload per cover item;
- no more than 4 cover items per minute;
- no more than 32 cover items per hour;
- no more than 64 cover items per run or local day;
- no more than 512 KiB cover payload per run;
- no more than about 1 MiB estimated request bytes per run;
- no more than 16 queued cover items globally and 4 per route;
- no more than 4 retained cover artifacts per run and 1 MiB total;
- abort before generation if any cap would be exceeded or `/srv/qbuild` free
  disk falls below 10 GiB.

## Marker-Plan Requirements

If a future qshield demo prototype is authorized, evidence must include:

- `NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK`
- `NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK`
- `NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK`
- `NA0335_COVER_ITEM_PURGE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK`
- `NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK`
- `NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK`

Always required:

- `NA0335_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0335_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0335_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0335_NO_METADATA_FREE_CLAIM_OK`
- `NA0335_NO_ANONYMITY_CLAIM_OK`

If the prototype remains deferred, evidence must include:

- `NA0335_COVER_TRAFFIC_DEFERRED_GATE_OK`
- `NA0335_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo cover traffic remains local/demo only;
- qsl-server production cover traffic requires cross-repo authorization;
- qsl-attachments production object cover requires cross-repo authorization;
- public-internet cover traffic remains future-gated;
- qshield demo proof is not service production proof.

## Claim-Boundary Requirements

Evidence, decision, traceability, journal, PR body, and response must not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- cover traffic is implemented by NA-0334;
- a cover traffic prototype exists after NA-0334;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion.

Mentions of those phrase families must be negated, prohibited, future-gated,
or boundary-only.

## Backup-Impact Requirements

- Changed evidence must remain under qsl-protocol paths covered by
  `/srv/qbuild/work`.
- No durable evidence location outside current backup scope may be created.
- No runtime artifacts may be created as part of NA-0334.
- The D132 preservation bundle must not be deleted or modified.
- If future durable cover-traffic evidence outside current backup scope is
  required, the future lane must stop and recommend a backup-plan update.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0331_metadata_runtime_batching -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0329_metadata_runtime_bounded_jitter -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0327_metadata_runtime_retry_cadence_normalization -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1 --nocapture`
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
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1 --nocapture`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- classifier proof for changed paths
- goal-lint through PR CI/preflight.

## CI Expectations

- The PR must keep required checks green.
- `public-safety` must remain required and green before merge.
- Merge must use normal merge with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, branch deletion, or delete-branch
  flag is allowed.

## Successor Handoff

If this authorization merges and post-merge public-safety is green, the
selected successor is:

`NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`

The successor must not implement production cover traffic, fixed-rate cover,
qsl-server behavior, qsl-attachments behavior, public-internet behavior, or
unsupported public/privacy claims.
