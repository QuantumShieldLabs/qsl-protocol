Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0336 Metadata Runtime Padding Bucket Expansion Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0336 produces a padding bucket expansion authorization/design
plan or an exact prerequisite stop, without implementing padding bucket
expansion or changing runtime, production, dependency, workflow, or public-copy
behavior.

## Protected Invariants

- NA-0336 is authorization/design only.
- No padding bucket expansion implementation is included.
- No runtime transport padding change is included.
- No qshield runtime change is included.
- No qsl-server or qsl-attachments production change is included.
- No protocol, crypto, qsc, qsp, key schedule, dependency, workflow,
  branch-protection, public-safety, website, README, START_HERE, docs/public,
  qsc-desktop, formal, input, tools/refimpl, or production-service change is
  included.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- No claim of anonymity, metadata-free behavior, untraceable behavior,
  production readiness, public-internet readiness, external review completion,
  or hidden timing/traffic-shape behavior is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0336_metadata_runtime_padding_bucket_expansion_authorization.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No optional input or script artifact is required for NA-0336 because the live
scope authorizes a design/authorization plan, not a new harness fixture.

## Forbidden Scope

- qshield runtime implementation changes.
- padding bucket expansion implementation.
- transport padding behavior changes.
- qsl-server implementation changes.
- qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Dependency, Cargo, workflow, branch-protection, or public-safety changes.
- Website, docs/public, README, START_HERE, qsc-desktop, formal, input,
  tools/refimpl, external website, or production-service changes.
- NA-0337 implementation.

## Prior Padding Review Requirements

Review and cite:

- live NA-0336 scope;
- NA-0319 default-padding evidence and testplan;
- NA-0314 transition plan;
- NA-0291 identifier/padding policy fixture evidence;
- NA-0293 sanitized-error/retention fixture evidence;
- NA-0335 cover prototype evidence and testplan;
- qshield send/recv/relay/init/config/relay_client padding behavior;
- relevant traceability and decision entries.

Required review result:

- current qshield padding support is described;
- inherited valid/invalid padding proof is described;
- inherited cover-traffic compatibility proof is described;
- production boundary is explicit;
- stop conditions are listed.

## Padding Bucket Semantic Design Requirements

The authorization evidence must define:

- future policy name;
- future deterministic bucket table;
- max padded payload size;
- max overhead expectation or exact future measurement requirement;
- invalid-config rejection rules;
- missing/malformed padding metadata rejection rules;
- valid strip/verify behavior;
- no remote delete before local verify;
- no accepted state/output on reject;
- artifact/log safety;
- compatibility with no-padding and current default-padding behavior.

## Future Implementation-Boundary Requirements

The authorization evidence must define exact future files that NA-0337 may
consider, plus future-gated files that require separate authorization.

NA-0337 must stop if it requires qsl-server, qsl-attachments, qsc/qsp,
protocol/crypto, dependency, workflow, branch-protection, public-safety,
website, README, START_HERE, docs/public, qsc-desktop, or production-service
changes.

## Abuse / Cost / Latency Matrix Requirements

The evidence must include a matrix with scenario, risk, proposed bound, future
test, failure mode, stop condition, compatibility impact, and claim boundary.

Required scenarios:

- valid small message;
- valid medium message;
- valid large message within demo cap;
- oversized payload;
- invalid padding bytes;
- missing padding metadata;
- malformed padding metadata;
- invalid bucket config;
- repeated invalid padded messages;
- batching plus padding;
- jitter/retry plus padding;
- cover prototype plus padding;
- attachment-size padding as future-gated;
- qsl-server production padding as future-gated;
- public-internet traffic observation as future-gated.

## Marker-Plan Requirements

If implementation is authorized, the evidence must list future NA-0337 markers:

- `NA0337_PADDING_BUCKET_AUTHORIZATION_OK`
- `NA0337_PADDING_BUCKET_POLICY_OK`
- `NA0337_DETERMINISTIC_TEST_PADDING_OK`
- `NA0337_VALID_SMALL_MESSAGE_PADDING_OK`
- `NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK`
- `NA0337_VALID_LARGE_MESSAGE_PADDING_OK`
- `NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0337_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0337_PADDING_STRIP_VERIFY_OK`
- `NA0337_PADDING_MALFORMED_REJECT_OK`
- `NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK`
- `NA0337_PADDING_NO_SECRET_ARTIFACT_OK`
- `NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK`
- `NA0337_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0337_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0337_NO_METADATA_FREE_CLAIM_OK`
- `NA0337_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0337_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`

If implementation is not safe, the evidence must define blocker markers and
select the blocker-resolution successor.

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo padding bucket expansion is local/demo only;
- qsl-server production padding requires cross-repo authorization;
- qsl-attachments production object-size padding requires cross-repo
  authorization;
- public-internet traffic observation remains future-gated;
- external review remains separate before any stronger privacy claim.

## Claim-Boundary Requirements

Evidence and PR text must not claim:

- anonymity;
- metadata-free behavior;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- hidden timing metadata;
- hidden traffic shape;
- that padding removes all metadata.

## Backup-Impact Requirements

Record whether the lane creates durable evidence outside qsl-protocol or the
existing response archive. Expected result: no backup-plan update required.
Stop if a new durable evidence location outside current backup scope is needed.

## Required Local Checks

Run or record an exact blocker:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qshield NA-0335 harness if directly runnable
- qshield NA-0331 harness if directly runnable
- qshield NA-0329 harness if directly runnable
- qshield NA-0327 harness if directly runnable
- qshield NA-0324 harness if directly runnable
- qshield NA-0322 harness if directly runnable
- qshield NA-0320 harness if directly runnable
- qshield NA-0319 harness if directly runnable
- qshield NA-0318 harness if directly runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1` if feasible
- `cargo +stable build -p qshield-cli --locked` if feasible
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted NA-0310 refimpl oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- queue, decision, scope, link, leak, goal-lint, classifier, and overclaim
  checks.

## CI Expectations

The PR may merge only if required checks, including `public-safety`, pass
normally. No admin bypass, direct push, squash, rebase, branch deletion,
delete-branch flag, or protection mutation is allowed.

## Successor Handoff

If NA-0336 succeeds and post-merge public-safety is green, a separate closeout
may mark NA-0336 DONE and restore exactly one READY item:

`NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`

The closeout must not implement NA-0337.
