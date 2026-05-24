Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0350 Metadata Runtime Production Backup Deploy Rollback Hardening Plan Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0350 records a qsl-protocol governance hardening plan for the
qsl-server / qsl-attachments production backup, deploy, and rollback boundary
after NA-0349, without mutating service repos, runtime paths, local backup
configuration, workflows, dependencies, website/public docs, or public claims.

## Protected invariants

- qsl-protocol is the only mutable repository.
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

- `docs/governance/evidence/NA-0350_metadata_runtime_production_backup_deploy_rollback_hardening_plan.md`
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

## Prior proof review requirements

Review and cite:

- live NA-0350 queue entry;
- NA-0349 implementation evidence and closeout testplan;
- NA-0348 evidence plan;
- NA-0347 qsl-server integration harness evidence;
- NA-0344 qsl-attachments production size-class harness evidence;
- D-0680 and D-0681;
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

## Backup/deploy/rollback inventory requirements

Inventory evidence for:

- qsl-server source, packaging, systemd, env, update, verify, audit, rollback,
  and CI wrapper evidence;
- qsl-attachments local storage root, durability/recovery contract, reference
  deployment runbook, cold restore tests, partial restore fail-closed tests, and
  log redaction tests;
- qsl-protocol qbuild continuity docs and local qbuild/codex backup status;
- local history/request/response availability;
- D132 preservation status.

Classify each item as proven, partial, missing, blocked, or future-gated.

## Threat/value model requirements

The plan must describe:

- what production backup/deploy/rollback hardening must prove before stronger
  claims;
- what qsl-server PR #56 and qsl-attachments PR #37 do and do not prove;
- same-host local continuity versus off-host disaster recovery;
- qsl-server state/config/log roots and qsl-attachments object/session/storage
  roots;
- service env/secret, systemd, proxy/TLS, monitoring/logging, deploy, rollback,
  restore, retention/purge, public ingress, operator error, and external-review
  hazards.

## Evidence gap matrix requirements

The evidence doc must include rows for:

- qsl-server source backup;
- qsl-attachments source backup;
- qsl-server build artifacts;
- qsl-attachments build artifacts;
- qsl-server runtime config;
- qsl-attachments runtime config;
- service env/secrets;
- systemd/service units;
- qsl-server runtime data;
- qsl-attachments object/session storage;
- logs and redaction;
- monitoring/alerts;
- deploy scripts;
- deploy manual runbook;
- rollback scripts;
- rollback manual runbook;
- backup restore drill;
- off-host backup;
- encrypted backup;
- local continuity snapshot;
- retention/purge after restore;
- public ingress cutover;
- DNS/TLS/proxy/CDN if relevant;
- qsl-server/qsl-attachments compatibility;
- external review package;
- website/public claims;
- D132/history/workflow-support evidence.

Each row must record current evidence, proof status, risk, future proof,
backup-plan impact, implementation need, blocker/readiness, and successor
relation.

## Strategy option requirements

Evaluate:

- production backup/deploy/rollback implementation authorization plan;
- backup/restore executable drill using local qbuild-only fixtures;
- qsl-server/qsl-attachments deployment runbook evidence package;
- qsl-server/qsl-attachments rollback drill harness;
- off-host encrypted backup prerequisite lane;
- local Codex workflow-support and history index lane;
- external review readiness gap audit;
- website/public claim boundary audit;
- blocker continuation.

The selected successor must be exact.

## Backup-plan impact requirements

The plan must state:

- whether NA-0350 itself requires a backup-plan update;
- whether `/srv/qbuild/work` covers current source checkouts;
- whether Codex responses are covered;
- whether directives, requests, journals, and ops history directories are
  covered or uncovered;
- future backup-plan prerequisites for service data roots, deploy configs,
  rollback artifacts, restore fixtures, monitoring artifacts, and local history
  directories.

## Deploy/rollback/secrets/monitoring requirements

Future prerequisites must cover:

- deployment topology;
- service unit/process manager boundary;
- env/secret source boundary;
- qsl-server runtime config boundary;
- qsl-attachments runtime config boundary;
- qsl-server data/storage boundary;
- qsl-attachments data/storage boundary;
- backup, deploy, rollback, post-deploy, rollback-verification, log-redaction,
  monitoring/alert, operator-runbook, emergency-stop, public-ingress, and
  restore-drill preflights.

## Public-ingress/timing/traffic-shape boundary requirements

The plan must state that:

- public ingress remains future-gated;
- timing metadata and traffic shape are not hidden by current evidence;
- attachment sizes and all metadata are not hidden by current evidence;
- qshield demo evidence is not production-service evidence.

## External-review boundary requirements

The plan must state that external review is not complete and that service-local,
demo-local, or modeled harness evidence is insufficient for an
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
implement them in NA-0350.

## Required local checks

Run and record:

- `date --iso-8601=seconds`
- `date -u --iso-8601=seconds`
- disk watermark commands;
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

Recommended successor if no blocker appears:

`NA-0351 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Authorization Plan`

Closeout, if executed, must restore exactly one READY item and must not
implement NA-0351.
