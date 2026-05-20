Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0323 Metadata Runtime Timing and Traffic-Shape Instrumentation Mitigation Design Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0323 produces a design-only instrumentation and mitigation
plan for metadata runtime timing and traffic-shape reduction, grounded in
NA-0321 and NA-0322 evidence, without implementing runtime instrumentation or
mitigation.

## Protected Invariants

- Measurement remains distinct from mitigation.
- A threat model is not mitigation.
- A measurement harness is not mitigation.
- Timing metadata and traffic shape are not claimed hidden.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server and qsl-attachments production timing remain unproven and
  future-gated.
- No anonymity, metadata-free behavior, untraceability, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.
- No qshield runtime, qsl-server, qsl-attachments, qsc/qsp/protocol/crypto,
  dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration change is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design.md`
- `tests/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime instrumentation implementation.
- Runtime timing mitigation.
- Jitter, batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, or transport padding implementation.
- qshield runtime implementation changes.
- qsl-server or qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule implementation changes.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, website/external website paths,
  `README.md`, `START_HERE.md`, `docs/public/**`, qsc-desktop paths, branch
  deletion, branch-protection mutation, or public-safety mutation.
- NA-0324 implementation.

## Prior Measurement Review Requirements

The evidence must review and preserve:

- live NA-0323 scope;
- inherited NA-0321 timing and traffic-shape threat model;
- inherited NA-0322 qshield embedded relay/demo measurement evidence;
- what NA-0322 proves;
- what NA-0322 does not prove;
- qsl-server and qsl-attachments production boundaries;
- stop conditions for future work.

## Instrumentation-Design Requirements

The design must analyze future instrumentation for:

- qshield demo timing instrumentation around send, candidate fetch, ack,
  invalid retry, and output;
- qshield queue cadence instrumentation with queue depth classes and candidate
  state transitions;
- padding/size instrumentation with bucket classes and padded length classes;
- invalid retry instrumentation with retry count classes and cadence;
- script-based instrumentation around existing commands;
- test-only Rust instrumentation with `Instant`;
- service instrumentation boundaries requiring cross-repo authorization.

Each option must identify future files, pros, risks, artifact safety, CI cost,
backup impact, claim boundary, and recommendation.

## Mitigation-Design Requirements

The design must analyze future mitigation options for:

- fixed-interval send/receive scheduling;
- bounded jitter;
- batching;
- cover traffic;
- queue drain scheduling;
- retry cadence normalization;
- padding bucket expansion;
- attachment-size class handling;
- local-demo-only mitigation;
- qsl-server/qsl-attachments production mitigation.

Each option must identify the threat addressed, what remains unaddressed,
implementation complexity, abuse/DoS risk, latency/cost impact, correctness
risk, metadata claim risk, test markers, stop conditions, and external review
recommendation.

## Risk Cost Compatibility Requirements

The evidence must include a matrix with:

- option;
- security benefit;
- metadata benefit;
- reliability risk;
- cost;
- implementation scope;
- test scope;
- claim boundary;
- recommendation.

## Production-Boundary Requirements

The evidence must state:

- qshield demo instrumentation is local proof only;
- qsl-server production timing is not proven unless a future cross-repo lane
  authorizes it;
- qsl-attachments production timing and size behavior are not proven unless a
  future cross-repo lane authorizes them;
- public internet timing remains future-gated;
- deployment/proxy/CDN/mobile/desktop timing remains out of scope or
  future-gated;
- no production readiness claim is made.

## Marker-Plan Requirements

The evidence must define these future instrumentation markers:

- `NA0324_TIMING_INSTRUMENTATION_PLAN_OK`
- `NA0324_QSHIELD_DEMO_TRACE_ARTIFACT_SCHEMA_OK`
- `NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK`
- `NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK`
- `NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK`
- `NA0324_INVALID_RETRY_INSTRUMENTATION_OK`
- `NA0324_NO_SECRET_TRACE_ARTIFACT_OK`

The evidence must also define these future mitigation-design markers:

- `NA0324_MITIGATION_OPTION_MATRIX_OK`
- `NA0324_JITTER_DESIGN_BOUNDARY_OK`
- `NA0324_BATCHING_DESIGN_BOUNDARY_OK`
- `NA0324_COVER_TRAFFIC_DESIGN_BOUNDARY_OK`
- `NA0324_MEASUREMENT_BEFORE_MITIGATION_OK`
- `NA0324_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0324_NO_METADATA_FREE_CLAIM_OK`

## Successor-Selection Requirements

If qshield/demo test-only instrumentation is feasible without user-facing
runtime changes, select:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

If mitigation design is still too broad, select a mitigation option matrix. If
service timing is required first, select a cross-repo authorization lane. If
evidence is insufficient, select blocker resolution. Do not implement NA-0324.

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
- all runtime gaps remain visible;
- qsl-server/qsl-attachments production boundaries remain explicit.

## Backup-Impact Requirements

Record whether the lane creates durable evidence outside the qsl-protocol
worktree or current Codex response path. Expected result: no backup-plan update
required because tracked evidence remains under `/srv/qbuild/work` and no
durable non-rebuildable artifact outside current backup scope is introduced.

## Required Local Checks

Run or record exact blocker:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- qshield NA-0320, NA-0319, and NA-0318 harnesses if directly runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1` if feasible
- `cargo +stable build -p qshield-cli --locked` if feasible
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh` if feasible
- metadata runtime plan harness from NA-0315
- metadata phase-2 identifier/padding harness
- metadata phase-2 sanitized-errors/retention harness
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
- scope guard, link-check, leak-scan, goal-lint, and classifier proof

## CI Expectations

Required checks must pass before merge, including `public-safety`. Docs-only or
full-suite cost-control skips are acceptable only when CI reports them as
skipped and `public-safety` remains green.

## Successor Handoff

After the NA-0323 design PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0323 DONE and restore exactly one READY
successor:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

The closeout must not implement NA-0324.
