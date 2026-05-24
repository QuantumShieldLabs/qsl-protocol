Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0348 Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Evidence Plan Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0348 records only the end-to-end qsl-server / qsl-attachments
integration evidence plan after qsl-server PR #55 and qsl-attachments PR #37,
then selects the exact NA-0349 successor without runtime implementation or
public/privacy/readiness overclaim.

## Protected invariants

- qsl-protocol is the only mutable repository.
- qsl-server is read-only.
- qsl-attachments is read-only.
- qshield runtime remains unchanged.
- qsc/qsp/protocol/crypto/key-schedule behavior remains unchanged.
- No dependency, workflow, branch-protection, public-safety, deployment,
  website, README, START_HERE, docs/public, or production service path changes.
- qsl-server PR #55 is bounded qsl-server harness evidence only.
- qsl-attachments PR #37 is bounded service-local evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No claim states or implies that attachment size, timing metadata, traffic
  shape, all metadata, anonymity, metadata-free behavior, or untraceable
  behavior is achieved.
- No production-readiness, public-internet-readiness, or external-review
  completion claim is made.

## Allowed scope

- `docs/governance/evidence/NA-0348_metadata_runtime_end_to_end_qsl_server_qsl_attachments_integration_evidence_plan.md`
- `tests/NA-0348_metadata_runtime_end_to_end_qsl_server_qsl_attachments_integration_evidence_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server implementation or branch mutation.
- qsl-attachments implementation or branch mutation.
- qshield runtime implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- `Cargo.toml`, `Cargo.lock`, `.github/**`, branch-protection, or
  public-safety configuration changes.
- README, START_HERE, docs/public, website, external website, deployment, or
  production service changes.

## Prior proof review requirements

The evidence plan must record:

- live NA-0348 scope;
- NA-0347 qsl-server PR #55 proof and limits;
- NA-0344 qsl-attachments PR #37 proof and limits;
- NA-0339 qshield demo/reference proof and limits;
- qsl-protocol governance proof from prior companion lanes.

## Source / authority refresh requirements

The evidence plan must record for qsl-server and qsl-attachments:

- local path/SHA if present;
- remote default branch SHA;
- PR merge status and merge SHA;
- latest default-branch CI status;
- viewer permission;
- branch protection and required checks;
- open PR list;
- source, authority, and CI classification.

If future live refresh shows stale source, red required CI, insufficient
authority, or blocking PRs, the successor must be blocker resolution.

## Inherited proof inventory requirements

The evidence plan must include a matrix for:

- qsl-server PR #55 source SHA, changed file, harness markers, CI status,
  limitations, and not-proven areas;
- qsl-attachments PR #37 source SHA, changed files, harness coverage, CI
  status, limitations, and not-proven areas;
- qshield demo/reference evidence and limits;
- qsl-protocol governance evidence and limits.

## Threat / value model requirements

The evidence plan must explain:

- what true end-to-end service evidence should prove;
- what service-local proofs do and do not prove;
- public-ingress observations;
- route-token/auth behavior;
- object-size class flow;
- opaque payload flow;
- storage/proxy handoff;
- retention/purge consistency;
- backup consistency;
- logs/redaction;
- monitoring/alerts;
- deploy/rollback;
- qshield demo reference limitations;
- attacker observations and claim boundaries.

## Evidence gap matrix requirements

The evidence plan must cover:

- source/authority freshness;
- qsl-server route/API;
- qsl-attachments object lifecycle;
- qsl-server to qsl-attachments handoff;
- object-size class propagation or opacity;
- descriptor/ciphertext treatment;
- route-token/auth behavior;
- quota/rate/max body;
- retention/purge consistency;
- backup/restore consistency;
- logs/redaction;
- monitoring/alerts;
- deploy/rollback;
- public ingress;
- external review;
- website/public claims;
- qshield demo reference;
- qsc/suite-id unaffected;
- CI/advisories/cost-control;
- D132/history/backup operations.

Each row must record current evidence, remaining gap, risk, future proof, and
successor relation.

## Strategy option requirements

The evidence plan must evaluate:

- qsl-server-only end-to-end harness;
- paired qsl-server and qsl-attachments harness;
- cross-repo integration test without production code changes;
- contract vector or fixture;
- backup/deploy/rollback prerequisite lane;
- external-review readiness lane;
- website/public-claim boundary lane;
- blocker continuation.

Each option must be recommended, rejected, or deferred.

## Backup / deploy / rollback / secrets / monitoring requirements

The evidence plan must record:

- no backup-plan update required for NA-0348 if changes remain under
  qsl-protocol governance/testplan paths;
- future backup-plan stop conditions for artifacts or service data outside
  current backup scope;
- deploy and rollback boundaries;
- secrets/env boundaries;
- monitoring/logging boundaries;
- artifact retention and operator runbook expectations.

## Public-ingress / timing / traffic-shape boundary requirements

The evidence plan must state that current evidence does not prove public-ingress
service behavior, does not prove timing metadata is hidden, does not prove
traffic shape is hidden, and does not prove attachment size or all metadata is
hidden.

## External-review boundary requirements

The evidence plan must state that external review is not complete and that any
stronger public or release claim requires implementation, service, deployment,
monitoring/log, backup/restore, public-claim, and external-review evidence.

## Claim-boundary requirements

Changed lines must not introduce affirmative claims for:

- attachment-size-hidden behavior;
- timing-hidden behavior;
- traffic-shape-hidden behavior;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion;
- padding hiding all metadata.

Negative and prohibited-claim wording is allowed when it preserves the boundary.

## Backup-impact requirements

The PR must list changed paths and record whether a backup-plan update is
required. Expected result for NA-0348: no backup-plan update required.

## Required local checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qshield reference harnesses when directly runnable
- qsc `send_commit`
- formal/model checks
- metadata conformance/harness checks when directly runnable
- queue helper
- decisions helper
- scope guard
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof for changed paths
- goal-lint / PR-body preflight

## CI expectations

- qsl-protocol PR checks must complete successfully.
- `public-safety` must remain required and green before merge.
- Post-merge qsl-protocol `main` must show `public-safety` success.

## Successor handoff

If NA-0348 evidence planning merges successfully, closeout may restore exactly:

`NA-0349 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness`

The closeout must not implement NA-0349.

## Selected successor

The selected successor must remain exact:

`NA-0349 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness`
