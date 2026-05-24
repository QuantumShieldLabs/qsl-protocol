Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0351 Metadata Runtime Production Backup Deploy Rollback Implementation Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0351 records a qsl-protocol governance authorization plan for
future metadata-runtime production backup/deploy/rollback implementation
harness work without mutating service repos, runtime paths, local backup
configuration, workflows, dependencies, website/public docs, or public claims.

## Protected invariants

- qsl-protocol is the only mutable repository in NA-0351.
- qsl-server and qsl-attachments are inspected read-only only.
- No qshield runtime, qsc, qsp, protocol, crypto, key-schedule, dependency,
  workflow, branch-protection, public-safety, website, README, START_HERE,
  docs/public, deploy, rollback, restore, service, or backup-script mutation.
- No production deployment, public ingress, rollback, restore, purge, backup
  operation, or secret-dependent test.
- No claim that attachment size, timing metadata, traffic shape, all metadata,
  anonymity, metadata-free behavior, untraceability, production readiness,
  public-internet readiness, or external-review completion is achieved.
- qsl-server PR #56 remains bounded end-to-end harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0351_metadata_runtime_production_backup_deploy_rollback_implementation_authorization.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server or qsl-attachments mutation.
- qshield runtime, qsc, qsp, qsl, protocol, crypto, key schedule, service, app,
  formal, input, tool/refimpl, workflow, dependency, backup script, timer, fstab,
  website, README, START_HERE, docs/public, branch-protection, and public-safety
  configuration mutation.
- Production deploy, rollback, restore, public-ingress enablement, purge, or
  backup operation.

## Prior hardening plan review requirements

Review and cite:

- live NA-0351 queue entry;
- NA-0350 hardening evidence and testplan;
- NA-0350 closeout testplan;
- NA-0349 implementation evidence;
- NA-0348 evidence plan;
- D-0682 and D-0683;
- TRACEABILITY boundaries for qsl-server/qsl-attachments and qshield demo
  evidence.

## Source/authority refresh requirements

For qsl-server and qsl-attachments, record:

- local path and local SHA if present;
- remote default branch SHA;
- target PR merge status and merge SHA;
- viewer permission;
- branch protection, required checks, force-push posture, deletion posture, and
  admin enforcement state where available;
- latest listed main CI status;
- open PR list;
- classification as `FRESH_SOURCE`, `STALE_SOURCE`, or `UNKNOWN_SOURCE`;
- classification as `COMPLETE_AUTHORITY`, `PARTIAL_AUTHORITY`, or
  `BLOCKED_AUTHORITY`;
- classification as `COMPLETE_CI`, `PARTIAL_CI`, or `BLOCKED_CI`.

## Authorization decision requirements

The evidence must select one or more exact categories:

- `IMPLEMENTATION_AUTHORIZATION_READY`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_BACKUP`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_DEPLOY`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_ROLLBACK`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SECRETS`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_MONITORING`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SOURCE_AUTHORITY`
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SCOPE`
- `IMPLEMENTATION_AUTHORIZATION_DEFERRED_LOCAL_OPS`

If the ready category is selected, the evidence must state that live production
operations remain forbidden and that future production/public claims remain
blocked until executable service evidence, off-host encrypted backup, restore,
rollback, monitoring, and review evidence exist.

## Future implementation bundle requirements

The evidence must define:

- future repo ownership and staging order;
- future allowed files;
- future forbidden files;
- future allowed commands;
- future forbidden commands;
- future tests;
- future artifacts;
- future PR and merge order;
- future post-merge verification;
- future backup-plan update triggers;
- future stop conditions.

If exact allowed files cannot be determined, the selected successor must be a
blocker or prerequisite lane.

## Future marker/verification requirements

The evidence must include future success marker candidates for:

- production backup/deploy/rollback authorization;
- source backup scope;
- runtime config backup scope;
- service data backup scope;
- local continuity boundary;
- off-host backup boundary;
- restore drill authorization;
- deploy authorization;
- rollback authorization;
- secrets/env boundary;
- monitoring/logging boundary;
- public-ingress boundary;
- no unsupported production/public/external-review/privacy claim.

If blocked, blocker marker candidates must be listed.

## Backup-plan impact requirements

The evidence must state:

- whether NA-0351 itself requires a backup-plan update;
- whether `/srv/qbuild/work` covers current source checkouts;
- whether Codex responses are covered;
- whether directives, requests, journals, and ops history directories are
  covered or uncovered;
- future backup-plan prerequisites for service data roots, deploy configs,
  rollback artifacts, restore fixtures, monitoring artifacts, and local history
  directories;
- off-host encrypted backup remains required before production/public claims or
  disaster-recovery claims.

## Deploy/rollback/restore/secrets/monitoring requirements

The evidence must state authorization status for:

- deploy;
- rollback;
- restore;
- backup scripts;
- runtime config;
- secrets/env;
- monitoring/logging;
- public ingress;
- emergency stop/runbook.

The evidence must list must-not-run commands, future allowed dry-run/read-only
commands, future allowed test-fixture commands, and future forbidden production
commands.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that:

- public ingress remains future-gated;
- timing metadata and traffic shape are not hidden by current evidence;
- attachment sizes and all metadata are not hidden by current evidence;
- qshield demo evidence is not production-service evidence.

## External-review boundary requirements

The evidence must state that external review is not complete and that
service-local, demo-local, or modeled harness evidence is insufficient for an
external-review-complete claim.

## Claim-boundary requirements

Changed lines must not introduce affirmative unsupported claims for:

- production readiness;
- public-internet readiness;
- external review completion;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hiding all metadata;
- complete disaster recovery from local continuity backup.

Negative, prohibited, future-gated, or caveated references are allowed.

## Workflow-support deferral requirements

Record whether future workflow-support items would reduce friction, but do not
implement them in NA-0351.

## Required local checks

Run and record:

- timestamp and disk watermark commands;
- qsl-protocol clean-tree, origin/main, queue, decisions, branch protection,
  public-safety, cargo audit, rustls-webpki, and classifier checks;
- qsl-server/qsl-attachments read-only source/authority/CI refresh;
- qsl-protocol heavy checks listed by the directive where feasible;
- changed-path scope guard, link-check, leak-scan, overclaim scan, classifier,
  and goal-lint before PR merge.

## CI expectations

- qsl-protocol PR required checks complete successfully.
- `public-safety` remains required before merge.
- Post-merge `public-safety` completes success.
- qsl-server and qsl-attachments remain read-only.

## Successor handoff

Recommended successor if the authorization PR merges and post-merge
public-safety is green:

`NA-0352 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Harness`

Closeout, if executed, must restore exactly one READY item and must not
implement NA-0352.
