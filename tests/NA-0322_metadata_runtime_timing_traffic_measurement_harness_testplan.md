Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322 Metadata Runtime Timing and Traffic-Shape Measurement Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0322 adds a bounded qshield embedded relay/demo measurement
harness for observable timing and traffic-shape surfaces without implementing
runtime mitigation or broadening metadata/privacy claims.

## Protected Invariants

- Measurement remains distinct from mitigation.
- Timing metadata and traffic shape remain classified as observable.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.
- No anonymity, metadata-free behavior, untraceability, production-readiness,
  public-internet-readiness, external-review-complete, timing-hidden, or
  traffic-hidden claim is introduced.
- No runtime timing, jitter, batching, cover traffic, send scheduling, receive
  scheduling, transport padding, or service deployment behavior is implemented.
- No qsl-server, qsl-attachments, qsc/qsp/protocol/crypto/key-schedule,
  dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration change is introduced.

## Allowed Scope

- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`
- `tests/NA-0322_metadata_runtime_timing_traffic_measurement_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qshield runtime source changes.
- qsl-server or qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule implementation changes.
- Runtime timing mitigation, jitter, batching, cover traffic, send scheduling,
  receive scheduling, transport padding, or service deployment changes.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, `README.md`, `START_HERE.md`,
  `docs/public/**`, website/external website paths, qsc-desktop paths,
  dependency changes, branch-protection mutation, or public-safety mutation.
- NA-0323 implementation.

## Prior Threat-Model Review Requirements

Review and preserve:

- NA-0321 timing and traffic-shape threat model.
- NA-0320 sanitized-error and retention/purge executable harness boundary.
- NA-0319 identifier/default-padding executable harness boundary.
- NA-0318 qshield ack/commit executable harness boundary.
- Demo smoke/stress/soak artifact boundaries.
- qsl-server and qsl-attachments production timing boundaries.

## Measurement-Surface Requirements

The harness must measure or classify:

- sender cadence through bounded qshield/demo send operations;
- receiver cadence through candidate fetch and invalid receive operations;
- queue cadence through queue-depth snapshots, candidate fetch without delete,
  and valid ack deletion;
- ack/commit timing;
- invalid retry cadence;
- size/padding bucket classes;
- ordering/correlation visibility.

If a required surface cannot be measured truthfully without runtime hooks, the
harness must omit the marker and the evidence must record the blocker.

## Artifact Schema Requirements

The measurement artifact must use bounded JSON/JSONL records with:

- `schema_version`;
- `run_id`;
- monotonic sequence or relative timing fields;
- coarse event labels;
- measured duration or ordering metric;
- bucket class or size class where applicable;
- classification output;
- explicit `measurement_not_mitigation` artifact class.

## Artifact Safety Requirements

The artifact must not contain:

- relay route tokens;
- raw handles;
- raw ack IDs or candidate IDs when sensitive;
- plaintext sentinels;
- padding sentinels;
- passphrase sentinels;
- raw key sentinels;
- panic/backtrace output;
- raw command output;
- raw local artifact paths.

The test must scan the artifact and require:

`ARTIFACT_SECRET_FINDING_COUNT 0`

## Harness Marker Requirements

Required measurement markers:

- `NA0322_TIMING_SURFACE_INVENTORY_OK`
- `NA0322_TRAFFIC_SHAPE_THREAT_MODEL_OK`
- `NA0322_QSHIELD_DEMO_TIMING_MEASUREMENT_OK`
- `NA0322_SENDER_CADENCE_MEASURED_OK`
- `NA0322_RECEIVER_CADENCE_MEASURED_OK`
- `NA0322_QUEUE_CADENCE_MEASURED_OK`
- `NA0322_ACK_COMMIT_TIMING_MEASURED_OK`
- `NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK`
- `NA0322_PADDING_SIZE_DISTRIBUTION_OK`
- `NA0322_ORDERING_CORRELATION_CLASSIFIED_OK`
- `NA0322_NO_SECRET_TIMING_ARTIFACT_OK`
- `NA0322_MEASUREMENT_NOT_MITIGATION_OK`
- `NA0322_METADATA_TIMING_MEASUREMENT_HARNESS_OK`

Required boundary markers:

- `NA0322_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0322_QSL_SERVER_TIMING_NOT_PROVEN_OK`
- `NA0322_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK`
- `NA0322_NO_METADATA_FREE_CLAIM_OK`

## Sender/Receiver Cadence Requirements

Sender cadence proof must record bounded elapsed timing around explicit send
operations. Receiver cadence proof must record bounded elapsed timing around
candidate fetch and invalid receive operations. Neither proof may claim hidden
timing or production receive scheduling.

## Queue Cadence Requirements

Queue cadence proof must show:

- zero candidates before send;
- candidates visible after send;
- candidate fetch does not delete;
- valid ack deletes one matching candidate;
- invalid receive retry retains the candidate.

## Size/Padding Distribution Requirements

Size/padding proof must record bucket classes, not raw plaintext or
secret-bearing plaintext size. It must preserve the boundary that padding
bucket evidence is not a claim that traffic shape is hidden.

## Ordering/Correlation Requirements

Ordering/correlation proof must classify visible ordering and queue correlation
signals. It must not claim contact-graph, route/session, or ordering
correlation resistance.

## Measurement-Versus-Mitigation Requirements

The harness and evidence must explicitly state that the artifact records
observable timing and traffic-shape signals. It must not implement or imply
jitter, batching, cover traffic, send scheduling, receive scheduling, transport
padding, or traffic-shape hiding.

## Successor-Selection Requirements

If the bounded qshield/demo measurement harness succeeds, select:

`NA-0323 -- Metadata Runtime Timing and Traffic-Shape Instrumentation / Mitigation Design Plan`

If measurement requires runtime hooks first, select an exact instrumentation
plan. If production service timing is the next blocker, select a cross-repo
service timing authorization lane. Do not implement NA-0323.

## Claim-Boundary Requirements

Evidence and PR text must state:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production/public-internet readiness claim;
- no external-review-complete claim;
- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.

## Backup-Impact Requirements

Record whether the lane creates durable evidence outside the qsl-protocol
worktree or existing Codex response path. Expected result: no backup-plan
update required because tracked evidence remains under `/srv/qbuild/work` and
runtime artifacts remain temporary.

## Required Local Checks

Run or record exact blocker:

- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- qshield NA-0320, NA-0319, and NA-0318 harnesses if directly runnable.
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` when feasible.
- metadata runtime plan and phase-2 harness scripts.
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- formal/model checks.
- NA-0310 vector JSON parse and refimpl oracle where available.
- queue/decision/scope/link/leak/goal-lint checks.
- classifier proof for changed paths.

## CI Expectations

Required checks must pass before merge, including `public-safety`. Full-suite
or docs-only cost-control skips are acceptable only when CI reports them as
skipped and `public-safety` remains green.

## Successor Handoff

After the NA-0322 implementation PR merges and post-merge public-safety is
green, a separate closeout may mark NA-0322 DONE and restore NA-0323 with the
exact selected title. That closeout must not implement NA-0323.
