Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0346 Metadata Runtime qsl-server Integration Implementation Authorization Test Plan

## Objective

Verify that NA-0346 refreshes qsl-server source/authority/CI evidence and
selects an exact future qsl-server implementation harness without implementing
qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol behavior,
deployment, public ingress, website copy, or dependency changes.

## Protected invariants

- NA-0346 is qsl-protocol governance and authorization only.
- qsl-protocol is the only mutable repository.
- qsl-server is inspected read-only only.
- qsl-attachments is not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule behavior is not changed.
- Cargo manifests, lockfiles, workflows, branch protection, and public-safety
  configuration are not changed.
- Website, README, START_HERE, and docs/public are not changed.
- qsl-attachments PR #37 service-local proof is not presented as qsl-server
  proof.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No claim is made that attachment size, timing metadata, traffic shape, all
  metadata, anonymity, metadata-free behavior, or untraceable behavior is
  achieved.
- No production readiness, public-internet readiness, or external-review
  completion is claimed.

## Allowed scope

- `docs/governance/evidence/NA-0346_metadata_runtime_qsl_server_integration_implementation_authorization.md`
- `tests/NA-0346_metadata_runtime_qsl_server_integration_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server mutation;
- qsl-attachments mutation;
- qshield runtime mutation;
- qsc/qsp/protocol/crypto/key-schedule mutation;
- Cargo manifest or lockfile changes;
- workflow changes;
- branch-protection or public-safety configuration changes;
- service deployment;
- website, README, START_HERE, docs/public, or external website changes;
- public-ingress, timing, traffic-shape, transport padding, cover traffic,
  batching, bounded jitter, retry-cadence, or padding-bucket implementation;
- NA-0347 implementation.

## Prior qsl-server boundary review requirements

The evidence must record:

- NA-0345 qsl-server integration boundary result;
- selected qsl-server source path and SHA from NA-0345;
- qsl-attachments PR #37 as service-local prerequisite evidence only;
- qshield embedded relay/demo evidence as reference/oracle evidence only;
- route/API, storage/proxy, retention/purge, rate/quota, backup,
  deploy/rollback, secrets/env, monitoring/logging, public-ingress,
  external-review, and public-claim boundaries.

## Freshness/authority refresh requirements

The evidence must record:

- qsl-server local path;
- local branch/ref and HEAD SHA;
- local worktree cleanliness;
- remote default branch and remote branch SHA;
- source freshness classification;
- viewer permission;
- branch protection status;
- required checks;
- latest listed run status;
- open PR list;
- mutation authority classification;
- CI authority classification;
- final authorization gate.

## Implementation authorization decision requirements

The evidence must state whether the next lane is:

- `IMPLEMENTATION_AUTHORIZATION_READY`, or
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED`.

If blocked, the evidence must name the exact prerequisite and select a blocker
successor. If ready, the evidence must select an exact NA-0347 successor.

## Future qsl-server bundle requirements

The evidence must define:

- repository;
- local path;
- base branch;
- base SHA refresh rule;
- qsl-server branch naming rule;
- qsl-protocol companion branch naming rule;
- PR order;
- merge strategy;
- post-merge verification;
- no-deploy boundary;
- rollback boundary;
- secrets/env boundary;
- qsl-attachments boundary.

## Future qsl-protocol companion requirements

The evidence must name allowed qsl-protocol companion paths and forbid runtime,
dependency, workflow, public-safety, website, README, START_HERE, docs/public,
formal, input, tool, qsc-desktop, qsl-server vendored, and qsl-attachments
vendored changes unless a future directive explicitly replaces the scope.

## File-map requirements

The evidence must name exact or category-limited qsl-server allowed files and
forbidden files. Allowed source/test files must be limited to the route/API,
opaque payload, storage/proxy, retention/TTL, rate/quota, logging/redaction,
config, and qsl-attachments contract harness surfaces proven by source
inspection.

Forbidden files must include qsl-server Cargo manifests/lockfiles, workflows,
release automation, public README/legal copy, broad deployment automation, and
unrelated refactors unless a future directive explicitly authorizes them.

## Build/test/CI requirements

The evidence must name qsl-server commands:

- `bash scripts/ci/test_aws_update_and_verify.sh`;
- `bash scripts/ci/test_update_checksum.sh`;
- `cargo fmt --all -- --check`;
- `cargo test -q`;
- `cargo clippy -q -- -D warnings`;
- targeted tests for any new harness.

The evidence must name qsl-protocol companion checks for queue, decisions,
scope, links, leaks, goal-lint/PR body, dependency/advisory health, qshield,
qsc, formal, metadata runtime, and public-safety status.

## Storage/retention/purge/backup/deploy/rollback requirements

The evidence must define future requirements for:

- service lifecycle;
- request lifecycle;
- qsl-attachments handoff lifecycle;
- storage/proxy lifecycle;
- retention duration;
- purge triggers;
- stale cleanup;
- failed upload/fetch cleanup;
- backup inclusion/exclusion;
- log redaction;
- artifact redaction;
- monitoring and alert thresholds;
- operator runbook;
- rollback;
- migration/compatibility;
- abuse/cost thresholds;
- secrets/env variables;
- deployment or explicit no-deployment boundary.

Unknown fields must be marked `REQUIRED_BEFORE_IMPLEMENTATION` and must become
future stop conditions if unresolved.

## qsl-attachments integration contract requirements

The evidence must state:

- whether qsl-server can use qsl-attachments service-local behavior without
  modifying qsl-attachments;
- exact integration assumptions;
- whether qsl-server passes through metadata, strips metadata, ignores
  metadata, or proxies opaque objects;
- whether qsl-server needs qsl-attachments API calls;
- whether the integration can be tested without production deployment;
- whether the integration needs secrets/env;
- whether qsl-attachments changes are required;
- qsl-attachments PR #37 remains prerequisite evidence, not qsl-server proof.

## Claim-boundary requirements

The evidence and PR body must preserve:

- no attachment-size-hidden claim;
- no timing-hidden claim;
- no traffic-shape-hidden claim;
- no all-metadata-hidden claim;
- no anonymity, metadata-free, or untraceable claim;
- no production-readiness, public-internet-readiness, or external-review
  completion claim;
- qsl-server/qsl-attachments production-service boundaries;
- qshield demo reference boundary;
- qsl-attachments service-local boundary.

## Backup-impact requirements

The evidence must state whether NA-0346 changes important evidence locations,
response paths, source roots, excluded backup paths, or durable artifacts outside
current backup scope.

Expected result: no backup-plan update is required if changed files stay inside
qsl-protocol governance/testplan paths under `/srv/qbuild/work`.

## Required local checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qshield NA-0339 harness if directly runnable
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
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1` when feasible
- `cargo +stable build -p qshield-cli --locked` when feasible
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` when feasible
- metadata runtime plan harness from NA-0315 if directly runnable
- metadata phase-2 identifier/padding harness
- metadata phase-2 sanitized-errors/retention harness
- metadata conformance smoke
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests when feasible
- qsc NA-0313 harness when directly runnable
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint or PR-body preflight
- classifier proof for the changed path set

## CI expectations

- qsl-protocol PR must pass required checks, including `public-safety`.
- qsl-server is not mutated in NA-0346.
- Future NA-0347 qsl-server PR must pass required `rust` before merge.
- Future qsl-protocol companion PR must pass required `public-safety` before
  merge.

## Successor handoff

If the gate is ready, select:

`NA-0347 -- Metadata Runtime qsl-server Integration Implementation Harness`

If a source, authority, CI, qsl-attachments contract, backup/deploy/rollback,
secrets/env, public-ingress, or public-claim prerequisite regresses, select the
matching blocker successor instead.
