Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0345 Metadata Runtime qsl-server Integration Boundary Test Plan

## Objective

Verify that NA-0345 records a qsl-server integration boundary plan after the
NA-0344 qsl-attachments service-local size-class proof, without implementing
qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol behavior,
deployment, public ingress, or public-copy changes.

## Protected invariants

- NA-0345 is governance/planning only.
- qsl-protocol is the only mutable repository.
- qsl-server is inspected read-only only.
- qsl-attachments is not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule behavior is not changed.
- Dependencies, manifests, lockfiles, workflows, branch protection, and
  public-safety configuration are not changed.
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

- `docs/governance/evidence/NA-0345_metadata_runtime_qsl_server_integration_boundary_plan.md`
- `tests/NA-0345_metadata_runtime_qsl_server_integration_boundary_testplan.md`
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
- NA-0346 implementation.

## Prior qsl-attachments proof review requirements

The evidence must record:

- qsl-attachments PR #37 merge evidence;
- qsl-protocol PR #950 companion evidence;
- qsl-protocol PR #951 closeout evidence;
- policy `qsl_attachments_production_size_class_v1`;
- valid small/medium/large proof;
- invalid-config, oversize, malformed metadata, retention/purge, backup, and
  no-secret-artifact proof;
- qsl-server boundary and qshield demo reference boundary.

## qsl-server source/authority inventory requirements

The evidence must record:

- local qsl-server path discovery;
- selected source path and SHA;
- branch/ref state;
- clean worktree state;
- remote URL;
- live remote freshness;
- viewer permission;
- branch protection;
- required checks;
- open PR state;
- latest listed CI state;
- known build/test/CI entrypoints;
- secrets/env and deploy/rollback source status;
- qsl-attachments integration status;
- source/authority classification.

## Integration threat/value model requirements

The evidence must cover:

- attachment object-size metadata flow;
- upload/fetch or opaque handoff routing;
- public ingress;
- service logs;
- timing and traffic-shape observation;
- rate limits and quotas;
- storage/proxy behavior;
- qsl-attachments API compatibility;
- residual risks and non-goals.

## Integration boundary requirements

The evidence must classify:

- qsl-server source/authority;
- qsl-attachments integration prerequisite;
- API/route boundary;
- storage/proxy boundary;
- timing/traffic-shape boundary;
- rate-limit/quota boundary;
- retention/purge boundary;
- backup boundary;
- deploy/rollback boundary;
- secrets/env boundary;
- monitoring/logging boundary;
- public-claim boundary;
- external-review boundary;
- test/CI boundary;
- qshield demo reference boundary.

## qsl-attachments compatibility requirements

The evidence must state:

- qsl-attachments PR #37 service-local size-class proof is a prerequisite;
- qsl-server integration must not change qsl-attachments without future
  cross-repo authorization;
- future qsl-server integration must either pass through qsl-attachments state
  opaquely or define an exact qsl-attachments service contract;
- no claim may state that qsl-server hides attachment sizes unless exact future
  evidence proves it;
- qsl-attachments production evidence remains service-local until qsl-server
  integration is proved.

## Storage / retention / purge / backup / deploy / rollback requirements

The evidence must define future prerequisites for:

- service lifecycle;
- request lifecycle;
- object lifecycle;
- qsl-attachments handoff lifecycle;
- retention duration;
- purge trigger;
- stale cleanup;
- failed upload/fetch cleanup;
- backup inclusion/exclusion;
- log and artifact redaction;
- monitoring and alert thresholds;
- operator runbook;
- rollback;
- migration/compatibility;
- abuse/cost threshold;
- secrets/env variables;
- deployment or explicit no-deployment boundary.

## Public-ingress / timing / traffic-shape boundary requirements

The evidence must state:

- public ingress remains unproven unless future source, service, deployment,
  monitoring, and public-ingress evidence proves it;
- qsl-server integration is review-sensitive;
- no public claim may imply attachment sizes, timing metadata, traffic shape,
  all metadata, anonymity, metadata-free behavior, or untraceable behavior is
  achieved;
- no website/public docs update is made by NA-0345.

## External-review boundary requirements

The evidence must keep external review incomplete and future-gated. Any future
stronger claim must require implementation, service, deployment, monitoring,
logging, and review evidence.

## Claim-boundary requirements

The evidence and PR body must avoid unsupported claims and must explicitly
preserve:

- qsl-attachments service-local boundary;
- qshield demo reference boundary;
- qsl-server production boundary;
- public-ingress boundary;
- timing/traffic-shape boundary;
- external-review boundary.

## Backup-impact requirements

The evidence must state whether NA-0345 changes any important evidence
locations, response paths, source roots, excluded backup paths, or durable
artifacts outside current backup scope.

Expected result: no backup-plan update is required if changed files stay inside
qsl-protocol governance/testplan paths under `/srv/qbuild/work`.

## Required local checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qshield-cli metadata runtime harness coverage through package test
- qshield-cli build
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests when feasible
- qsc NA-0313 harness when feasible
- queue and decisions helpers
- scope guard with exact allowed paths
- link check
- leak scan
- goal-lint / PR body preflight
- classifier proof for changed paths

## CI expectations

The qsl-protocol PR must pass required checks, including `public-safety`, before
merge. After merge, `public-safety` must remain required and green on `main`.

## Successor handoff

If NA-0345 merges successfully, the successor is:

`NA-0346 -- Metadata Runtime qsl-server Integration Implementation Authorization Plan`

The successor must not be implemented by NA-0345.
