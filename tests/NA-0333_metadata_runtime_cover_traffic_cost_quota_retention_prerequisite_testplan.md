Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0333 Metadata Runtime Cover Traffic Cost Quota Retention Prerequisite Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0333 defines only the prerequisite cost, quota, retention,
purge, backup, abuse/DoS, deployment, qshield-demo, and service-production
boundaries required before any future cover-traffic prototype or production
cover-traffic lane can be authorized.

## Protected Invariants

- NA-0333 is prerequisite planning only.
- No cover traffic implementation is included.
- No cover traffic prototype implementation is included.
- No runtime timing mitigation, batching, bounded-jitter, retry-cadence, broad
  queue scheduling, send scheduling, receive scheduling, or transport padding
  implementation change is included.
- qshield embedded relay/demo evidence remains local/demo only.
- qsl-server and qsl-attachments production behavior remains unproven and
  cross-repo gated.
- No claim says timing metadata is hidden.
- No claim says traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.
- No qshield runtime, qsl-server, qsl-attachments, qsc/qsp/protocol/crypto,
  key-schedule, dependency, workflow, website, README, START_HERE, docs/public,
  qsc-desktop, branch-protection, or public-safety configuration change is
  included.

## Allowed Scope

- `docs/governance/evidence/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_plan.md`
- `tests/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_testplan.md`
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
- NA-0334 implementation.

## Prior Risk-Gate Review Requirements

Evidence must review NA-0332 and confirm:

- cover traffic remains deferred;
- direct cover-traffic implementation is rejected for the current phase;
- direct qshield-demo prototype authorization was deferred until NA-0333;
- cost, quota, retention, purge, backup, abuse, and deployment boundaries are
  the blocker before any future prototype authorization;
- qsl-server and qsl-attachments production cover traffic remain cross-repo
  gated;
- timing and traffic-shape gaps remain explicit.

Evidence must review NA-0331 and confirm:

- qshield batching is opt-in local/demo only;
- maximum batch size is four;
- send and ack batches validate all-or-nothing;
- invalid receive candidates do not cause remote delete before local
  verification;
- retry-cadence and bounded-jitter caps remain preserved;
- batching does not prove cover traffic or production-service behavior.

## Cost / Bandwidth / Storage Model Requirements

Evidence must define:

- bandwidth ingress and egress dimensions;
- relay queue item and byte dimensions;
- local qshield store and ledger byte growth;
- attachment object-store impact, explicitly zero for qshield-demo-only work;
- CPU/memory and queue-depth impact;
- backup growth and artifact growth impact;
- operator monitoring and stop thresholds;
- upper bounds per minute, per hour, and per run or local day;
- deterministic test-mode thresholds;
- unsupported public/privacy claim boundaries.

Required minimum caps for any future qshield-demo prototype authorization:

- no more than 4 cover items per minute;
- no more than 32 cover items per hour;
- no more than 64 cover items per run or local day;
- no more than 8192 bytes payload per cover item unless a future lane narrows
  the limit;
- no more than 1 MiB estimated request bytes per run;
- no more than 16 queued cover items globally and 4 per route;
- deterministic test mode no more than 8 cover items and 128 KiB estimated
  request bytes.

## Quota / Abuse / DoS Model Requirements

Evidence must cover:

- honest local demo cover;
- malicious queued-message injector;
- malicious cover-trigger amplification;
- repeated invalid cover interactions;
- quota exhaustion;
- relay queue exhaustion;
- backup growth;
- attachment storage growth;
- service deployment pressure;
- public-internet abuse;
- log/artifact growth;
- operator runaway cost.

Required controls:

- per-run cap;
- per-route cap;
- per-session cap;
- per-time-window cap;
- maximum queued cover items;
- maximum retained cover artifacts;
- deterministic test-mode caps;
- abuse stop condition;
- no-production-service default;
- no recursive cover generation.

## Retention / Purge / Backup / Ops Model Requirements

Evidence must define:

- cover item lifecycle;
- real item versus cover item distinction without leaking secrets;
- purge triggers;
- stale cover cleanup;
- failed cover cleanup;
- backup inclusion or exclusion boundary;
- log redaction;
- artifact redaction;
- operator monitoring;
- alert thresholds;
- rollback requirements;
- cleanup procedure;
- backup-plan impact analysis.

Evidence must explicitly state:

- no backup-plan update is required for NA-0333 if only qsl-protocol
  governance/testplan files are added;
- a future cover-traffic implementation lane may require backup-plan updates
  if it creates durable non-rebuildable artifacts or grows `/srv/qbuild/tmp`,
  service storage, or backup scope.

## Prototype Authorization Decision Requirements

Evidence must decide whether:

- a future qshield-demo-only cover-traffic prototype may be authorized later;
- cover traffic remains deferred until service timing cross-repo
  authorization;
- padding bucket expansion should be the next lane instead;
- production cover traffic remains cross-repo gated.

The decision must account for:

- cost model completeness;
- quota model completeness;
- retention/purge model completeness;
- backup impact;
- abuse/DoS risk;
- operator burden;
- external-review sensitivity;
- public claim safety;
- qshield demo value;
- production-service relevance.

## Marker-Plan Requirements

Evidence must include the selected NA-0334 marker plan.

Always required:

- `NA0334_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0334_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0334_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0334_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0334_NO_METADATA_FREE_CLAIM_OK`
- `NA0334_NO_ANONYMITY_CLAIM_OK`

If the selected successor is qshield-demo prototype authorization, also
required:

- `NA0334_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0334_COVER_TRAFFIC_COST_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_QUOTA_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_RETENTION_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_BACKUP_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_ABUSE_BOUNDARY_OK`
- `NA0334_NO_PRODUCTION_COVER_TRAFFIC_OK`
- `NA0334_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo cover traffic remains local/demo only;
- qsl-server production cover traffic requires cross-repo authorization;
- qsl-attachments production object cover requires cross-repo authorization;
- public-internet cover traffic remains future-gated;
- cover traffic can create bandwidth, storage, backup, retention, purge, and
  operations costs.

## Claim-Boundary Requirements

Evidence, decision, traceability, journal, PR body, and response must not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- cover traffic is implemented;
- a cover traffic prototype exists;
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
- No runtime artifacts may be created as part of NA-0333.
- The D132 preservation bundle must not be deleted or modified.
- If a future durable cover-traffic evidence location is required, the lane
  must stop and recommend a backup-plan update.

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
- queue/decision checks;
- scope guard with exact allowed paths;
- link-check;
- leak-scan;
- overclaim scan;
- goal-lint;
- classifier proof for changed paths.

## CI Expectations

- PR body includes standalone `Goals: G1, G2, G3, G4, G5`.
- PR body states NA-0333 is prerequisite planning only.
- PR body states selected NA-0334 successor exactly.
- Required checks complete green before merge.
- `public-safety` remains required and green.
- Merge uses normal merge with `--match-head-commit`.
- No branch deletion flag is used.

## Successor Handoff

Expected successor after NA-0333 closeout:

`NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`

NA-0334 must not be implemented by NA-0333 closeout.
