Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0352 Metadata Runtime Production Backup Deploy Rollback Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0352 implements a bounded qsl-protocol production backup/deploy/rollback
authorization harness. The harness is local-only, deterministic, fixture-based,
and explicitly not a live backup, deploy, rollback, restore, purge, production
service, public-ingress, secret-dependent, or local backup operation.

Result marker:

`NA0352_METADATA_RUNTIME_PRODUCTION_HARDENING_HARNESS_OK`

Selected successor:

`NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`

The successor is selected because the harness proves the current authorization
boundaries but also keeps the same-host continuity limitation visible. Off-host
encrypted backup remains the next production-hardening prerequisite before any
production readiness, public-internet readiness, or disaster-recovery claim.

## Live NA-0352 scope

The live queue restored NA-0352 as:

`Metadata Runtime Production Backup / Deploy / Rollback Implementation Harness`

The scope requires refreshing qsl-server and qsl-attachments source, authority,
and CI read-only, then implementing a bounded non-production harness or stopping
with exact blocker evidence before any live operation or public claim expansion.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that attachment size, timing metadata, traffic shape, or all metadata
  is hidden;
- qsl-server and qsl-attachments production boundaries remain explicit;
- backup, deploy, rollback, and restore operations remain explicitly gated;
- qshield embedded relay/demo proof remains reference/oracle evidence only.

## Inherited NA-0351 authorization

NA-0351 recorded `IMPLEMENTATION_AUTHORIZATION_READY` for bounded executable
non-production harness evidence after NA-0350. It did not authorize live backup,
restore, deploy, rollback, public-ingress cutover, secret-dependent testing, or
production service operation.

Inherited evidence:

- qsl-protocol PR #964 merged the authorization as `6e823887785d`.
- qsl-protocol PR #965 restored NA-0352 as READY as `83399fa72e2a`.
- qsl-server PR #56 remains bounded end-to-end modeled harness evidence at
  `d40e6003fdf0`.
- qsl-attachments PR #37 remains service-local prerequisite evidence at
  `96b9352bd63e`.
- local backup remains same-host continuity only, not complete disaster
  recovery.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| remote main SHA | `d40e6003fdf0` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| remote main SHA | `96b9352bd63e` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled in current protection |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

No source, authority, open-PR, or CI blocker was found.

## Harness feasibility and file selection

The selected implementation shape is qsl-protocol-only:

- `scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh`
- `inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json`

The harness is feasible because the lane only needs to prove authorization and
claim boundaries. It does not need to mutate qsl-server, qsl-attachments,
qshield runtime, qsc/qsp, protocol/crypto, workflows, dependencies, service
configuration, local backup scripts, timers, fstab, website, README,
START_HERE, or docs/public.

## Fixture summary

The fixture records:

- same-host local continuity backup as `LOCAL_CONTINUITY_ONLY`;
- off-host encrypted backup as `FUTURE_GATE` with no completion evidence;
- qsl-server and qsl-attachments source, authority, and CI classifications;
- runtime config, service data, backup, restore, deploy, rollback, secrets/env,
  monitoring/logging, and public-ingress scopes as future-gated or forbidden for
  current live operation;
- explicit forbidden operations, including live backup, restore, deploy,
  rollback, purge, secret-dependent operation, service mutation, backup
  script/timer/fstab mutation, and public-ingress cutover;
- claim boundaries prohibiting production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable, size-hidden,
  timing-hidden, traffic-shape-hidden, padding-hides-all-metadata, and
  local-continuity-is-disaster-recovery claims;
- all NA0352 required markers;
- secret sentinel labels that must not appear outside the sentinel list or proof
  artifact.

## Harness summary

The harness parses the JSON fixture with Python from a POSIX shell wrapper,
validates required fields and marker sets, refuses current authorization for
live backup/deploy/rollback/restore, refuses off-host backup completion without
evidence, refuses local continuity as full disaster recovery, refuses live
operations, checks the claim-boundary set, scans sentinels, and writes a
temporary proof artifact under `/srv/qbuild/tmp/NA-0352_*`.

The harness emits `SECRET_FINDING_COUNT 0` only after validating that secret
sentinel labels do not appear in proof output or outside the fixture sentinel
list.

## Marker proof

Required markers emitted by the harness:

- `NA0352_PRODUCTION_BACKUP_DEPLOY_ROLLBACK_AUTHORIZATION_OK`
- `NA0352_SOURCE_BACKUP_SCOPE_OK`
- `NA0352_RUNTIME_CONFIG_BACKUP_SCOPE_OK`
- `NA0352_SERVICE_DATA_BACKUP_SCOPE_OK`
- `NA0352_LOCAL_CONTINUITY_BOUNDARY_OK`
- `NA0352_OFF_HOST_BACKUP_BOUNDARY_OK`
- `NA0352_RESTORE_DRILL_AUTHORIZATION_OK`
- `NA0352_DEPLOY_AUTHORIZATION_OK`
- `NA0352_ROLLBACK_AUTHORIZATION_OK`
- `NA0352_SECRETS_ENV_BOUNDARY_OK`
- `NA0352_MONITORING_LOGGING_BOUNDARY_OK`
- `NA0352_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0352_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0352_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0352_NO_EXTERNAL_REVIEW_COMPLETE_CLAIM_OK`
- `NA0352_NO_METADATA_FREE_CLAIM_OK`
- `NA0352_NO_ANONYMITY_CLAIM_OK`
- `NA0352_NO_BACKUP_OPERATION_OK`
- `NA0352_NO_DEPLOY_OPERATION_OK`
- `NA0352_NO_ROLLBACK_OPERATION_OK`
- `NA0352_NO_RESTORE_OPERATION_OK`
- `NA0352_METADATA_RUNTIME_PRODUCTION_HARDENING_HARNESS_OK`

## Artifact safety proof

The proof artifact is temporary and under `/srv/qbuild/tmp/NA-0352_*`. It
contains only the artifact class, zero-operation count, backup-plan decision,
selected successor, artifact path, marker lines, and `SECRET_FINDING_COUNT 0`.

No production endpoint, credential, raw token, passphrase, key, bearer value,
fetch capability, service data, runtime config, deployment artifact, restore
state, rollback artifact, or backup artifact is written by the harness.

## Backup-plan impact and local-ops dependency decision

No backup-plan update is required for NA-0352. Changed files stay under
qsl-protocol source in `/srv/qbuild/work`, which is included by the current
local continuity backup. Harness artifacts are temporary under `/srv/qbuild/tmp`.
The required Codex response file is under the existing responses path, which is
covered by the installed local continuity source list.

Future backup-plan review is required before relying on:

- off-host encrypted backup;
- production service data roots;
- deploy configuration roots;
- rollback artifacts;
- restore fixtures;
- monitoring artifacts;
- local directive/request/journal/ops history directories as durable roots.

Local workflow-support and a directive/response/journal index would materially
reduce friction, but they are deferred because off-host encrypted backup is the
more direct production-hardening prerequisite.

## Deploy/rollback/restore/secrets/monitoring boundary

Current authorization states:

- live backup: forbidden.
- live restore: forbidden.
- live deploy: forbidden.
- live rollback: forbidden.
- purge: forbidden.
- production service mutation: forbidden.
- backup script, timer, fstab, and source-list mutation: forbidden.
- secret-dependent tests and secret values in evidence: forbidden.
- live monitoring/logging system mutation: forbidden.

Future lanes may use dry-run, fixture, and read-only checks only when explicitly
authorized.

## Public-ingress/timing/traffic-shape boundary

Public ingress remains future-gated. The harness proves only that current public
ingress is not authorized by this lane.

The current evidence does not prove hidden attachment size, hidden timing
metadata, hidden traffic shape, hidden all metadata, or padding that hides all
metadata. These remain visible gaps for future work.

## External-review sensitivity

External review remains not complete. This harness is internal boundary proof,
not external-review-complete evidence.

Any stronger statement requires implementation evidence, service evidence,
deployment evidence, monitoring/log evidence, backup/restore evidence, rollback
evidence, off-host backup evidence, public-ingress evidence if relevant, and
review evidence.

## Public claim boundary

NA-0352 does not update website copy, public docs, README, or START_HERE.

Future public wording must not imply:

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
- same-host continuity backup as complete disaster recovery.

## Workflow-support and history-index future work note

Read-only history availability during NA-0352:

- `/home/victor/work/qsl/codex/directives`: absent.
- `/home/victor/work/qsl/codex/responses`: present; D165/NA0351 response was
  present and improved identity confidence.
- `/home/victor/work/qsl/codex/journals`: absent.
- `/home/victor/work/qsl/codex/requests`: present; workflow-support and
  history-access requests were inspected read-only.

Future local-ops improvements that would reduce friction:

- qstart/qresume fast-forward to expected `origin/main`;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives, requests, journals, and ops history folders.

These items are not implemented by NA-0352.

## Selected successor

Selected:

`NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`

Rationale:

- qsl-server and qsl-attachments source, authority, and CI are fresh.
- The qsl-protocol harness proves authorization and claim boundaries without
  live operation.
- Same-host local continuity remains the highest production-hardening gap before
  any live operation or disaster-recovery claim.
- Off-host encrypted backup is explicitly future-gated in NA-0350 and NA-0351
  evidence.

## Rejected alternatives

- `Metadata Runtime Production Backup / Deploy / Rollback Blocker Resolution`:
  rejected because no current source, authority, CI, or harness-scope blocker
  was found.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: deferred
  because it reduces friction but does not outrank off-host backup as a
  production-hardening prerequisite.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred because
  backup/deploy/rollback prerequisites remain more basic.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because
  this lane changes no public docs or website copy.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  backup/deploy/rollback and off-host backup boundaries are clearer.

## Backup-plan impact statement

No current backup-plan update is required for NA-0352. Future production
hardening requires backup-plan review before it introduces or relies on durable
service roots, deploy configs, rollback artifacts, restore fixtures, monitoring
artifacts, or local history directories outside current scope.

Same-host local continuity remains local continuity only. Off-host encrypted
backup and restore evidence are required before any disaster-recovery claim.

## Next recommendation

Close NA-0352 after the harness PR merges and post-merge public-safety is green,
then restore:

`NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`
