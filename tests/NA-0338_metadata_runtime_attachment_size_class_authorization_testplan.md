Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0338 Metadata Runtime Attachment Size-Class Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0338 produces an authorization/design plan for future
attachment size-class handling, selects an exact NA-0339 successor, and
preserves all qshield demo versus qsl-server/qsl-attachments production
boundaries without implementing runtime behavior.

## Protected Invariants

- NA-0338 remains authorization/design only.
- No attachment size-class implementation is included.
- No attachment object-size padding implementation is included.
- No qshield runtime behavior changes are included.
- No qsl-server or qsl-attachments behavior changes are included.
- No qsc/qsp/protocol/crypto/key-schedule behavior changes are included.
- No dependency, workflow, branch-protection, public-safety, website, README,
  START_HERE, docs/public, qsc-desktop, formal, input, tool/refimpl, app
  runtime, or service implementation change is included.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- No claim says attachment size, timing metadata, traffic shape, or all
  metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.
- Exactly one READY item remains NA-0338 until closeout.

## Allowed Scope

- `docs/governance/evidence/NA-0338_metadata_runtime_attachment_size_class_authorization.md`
- `tests/NA-0338_metadata_runtime_attachment_size_class_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- attachment size-class implementation
- attachment object-size padding implementation
- qshield runtime implementation changes
- qsl-server implementation changes
- qsl-attachments implementation changes
- qsc/qsp/protocol/crypto/key-schedule implementation changes
- Cargo manifest or lockfile changes
- dependency changes
- workflow, branch-protection, or public-safety changes
- website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tools/refimpl, app runtime, production-service, or public-copy changes
- NA-0339 implementation

## Prior Attachment / Padding Review Requirements

Before patching, review:

- live NA-0338 scope in `NEXT_ACTIONS.md`;
- NA-0337 padding bucket expansion evidence and testplan;
- NA-0336 padding bucket authorization evidence;
- NA-0335 cover prototype evidence;
- NA-0319 identifier/default-padding evidence;
- NA-0321 and NA-0324 metadata timing/traffic evidence;
- NA-0260 qshield attachment demo readiness evidence;
- canonical attachment descriptor, QATT service, and encryption-context docs;
- qshield `attachment`, `send`, `recv`, `relay`, `relay_client`, and config
  sources;
- available qshield metadata-runtime tests;
- traceability and decision state.

## Attachment Size-Class Semantic Design Requirements

The evidence must define:

- future policy name;
- deterministic future qshield demo attachment size-class table;
- maximum padded attachment object size;
- maximum overhead;
- descriptor metadata boundary;
- ciphertext object boundary;
- invalid config rejection;
- oversized object rejection;
- malformed object rejection;
- strip/verify behavior where applicable;
- no-output/no-accepted-state behavior on invalid objects;
- no remote delete/purge before the stated local verification boundary;
- retention/purge behavior;
- backup and artifact-safety impact;
- production boundary.

## Future Implementation-Boundary Requirements

The evidence must list future allowed qshield files and future forbidden files.

Future allowed scope may include only bounded qshield demo attachment code,
config only if exact future scope requires it, qshield tests, and optional
fixture/script paths only if the future directive explicitly permits them.

Future forbidden scope must include:

- `qsl-attachments/**`;
- `qsl-server/**`;
- `qsc/**`;
- `qsp/**`;
- protocol, crypto, or key-schedule implementation paths;
- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `website/**`;
- `README.md`;
- `START_HERE.md`;
- `qsc-desktop/**`;
- branch-protection or public-safety configuration.

## Abuse / Cost / Latency Matrix Requirements

The evidence must include a matrix with:

- scenario;
- risk;
- proposed bound;
- future test;
- failure mode;
- stop condition;
- compatibility impact;
- claim boundary.

Required scenarios:

- valid small attachment;
- valid medium attachment;
- valid large demo attachment within cap;
- oversized attachment;
- malformed descriptor;
- malformed ciphertext object;
- invalid size-class config;
- repeated invalid attachment attempts;
- attachment retention/purge failure;
- backup growth;
- local demo stress;
- qshield cover prototype plus attachment size-class;
- batching/jitter/retry interactions;
- qsl-attachments production equivalent, future-gated;
- public-internet object-size observation, future-gated.

## Marker-Plan Requirements

If a qshield demo implementation harness is authorized, future markers must
include:

- `NA0339_ATTACHMENT_SIZE_CLASS_AUTHORIZATION_OK`
- `NA0339_ATTACHMENT_SIZE_CLASS_POLICY_OK`
- `NA0339_DETERMINISTIC_TEST_ATTACHMENT_SIZE_CLASS_OK`
- `NA0339_VALID_SMALL_ATTACHMENT_OK`
- `NA0339_VALID_MEDIUM_ATTACHMENT_OK`
- `NA0339_VALID_LARGE_ATTACHMENT_OK`
- `NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK`
- `NA0339_ATTACHMENT_OVERSIZE_REJECT_OK`
- `NA0339_ATTACHMENT_MALFORMED_REJECT_OK`
- `NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK`
- `NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK`
- `NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK`
- `NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0339_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0339_QSL_ATTACHMENTS_PRODUCTION_BOUNDARY_OK`
- `NA0339_NO_METADATA_FREE_CLAIM_OK`
- `NA0339_NO_SIZE_HIDDEN_CLAIM_OK`
- `NA0339_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0339_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`

If implementation is not safe, the evidence must define blocker markers
instead.

## Production-Boundary Requirements

The evidence must state:

- qshield embedded relay/demo attachment size-class evidence would be
  local/demo only;
- qsl-attachments production object-size padding requires cross-repo
  authorization;
- qsl-server production timing/storage behavior requires cross-repo
  authorization;
- public internet object-size behavior remains future-gated;
- external review is recommended before stronger privacy claims;
- website/public language must remain conservative.

## Claim-Boundary Requirements

Do not claim:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public internet readiness;
- external review completion;
- quantum-proof hype, unbreakable, military-grade, or guaranteed secure
  properties.

Allowed only when explicitly negated, prohibited, or classified as future/
unproven.

## Backup-Impact Requirements

Record whether the NA-0338 patch creates evidence outside the current covered
qbuild worktree.

Expected result: no backup-plan update required because tracked changes remain
under qsl-protocol paths already covered by `/srv/qbuild/work`.

Stop if a new durable evidence location outside current backup scope is
required.

## Required Local Checks

Run or record an exact blocker:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qshield NA-0337 harness if directly runnable
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
- metadata runtime plan harness from NA-0315
- metadata phase-2 identifier/padding harness
- metadata phase-2 sanitized-errors/retention harness
- metadata conformance smoke
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- queue, decisions, scope-guard, link-check, leak-scan, goal-lint, and
  classifier proof.

## CI Expectations

The PR may merge only after required checks attach and complete successfully.
`public-safety` must remain required and green before merge and after merge.

## Successor Handoff

If authorization completes, select exactly one future NA-0339 successor. This
testplan expects:

`NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`

Do not implement NA-0339 in this lane.
