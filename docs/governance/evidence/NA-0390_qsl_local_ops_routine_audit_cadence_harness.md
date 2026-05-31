Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0390 QSL Local Ops Routine Audit Cadence Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0390 implements the bounded routine audit cadence harness authorized by
NA-0389. The implementation is temp-output only and validates audit profiles,
depth levels, trigger policy, severity taxonomy, queue insertion policy, report
boundaries, and public-claim safeguards.

This is not a scheduler, not a full audit, not a durable audit report store, and
not public, production, privacy, or external-review readiness proof.

## Live NA-0390 scope

Live `NEXT_ACTIONS.md` recorded:

- READY_COUNT `1`.
- READY `NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`.
- NA-0389 DONE.
- D-0760 exists once.
- D-0761 exists once.
- D-0762 absent at startup.
- public-safety required and green on `origin/main` `1b199440c5a3`.

Allowed scope matched this lane:

- `scripts/ci/qsl_routine_audit_cadence.py`;
- `inputs/local_ops/routine_audit_cadence_fixtures/`;
- this evidence file;
- `tests/NA-0390_qsl_local_ops_routine_audit_cadence_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- temp proof under `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`.

## Inherited NA-0389 authorization

NA-0389 selected:

`ROUTINE_AUDIT_CADENCE_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

NA-0389 required the first implementation lane to remain standalone,
standard-library-only, temp-output-only, no scheduler, no workflow, no durable
audit reports, no archive mutation, no secret handling, no runtime change, no
dependency change, no qsl-server/qsl-attachments mutation, and no public-claim
expansion.

NA-0380 audit report checksums were still present:

- overall audit report SHA-256 prefix `66dd26c0b35b`;
- code/crypto audit report SHA-256 prefix `70c21179e7a5`.

## Implemented helper path

Helper added:

- `scripts/ci/qsl_routine_audit_cadence.py`

Helper behavior:

- standard library only;
- no network, GitHub, subprocess, shelling out, scheduler, cron, workflow, or
  background job behavior;
- strict JSON policy validation;
- strict fixture matrix;
- deterministic human and JSON summaries;
- validate and simulate modes;
- output only under `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`;
- new-file writes only for helper output, with overwrite rejection;
- no deletion;
- no full response body copy;
- no secret material in generated output;
- no public-claim expansion.

## Fixture matrix and markers

Fixture proof log:

`/srv/qbuild/tmp/NA0390_routine_audit_cadence_20260530T193850-0500/fixture_matrix.log`

Fixture result:

- cases: `42`;
- passed: `42`;
- failed: `0`.

Required markers present:

- `NA0390_ROUTINE_AUDIT_CADENCE_AUTHORIZATION_OK`
- `NA0390_ROUTINE_AUDIT_CADENCE_HELPER_OK`
- `NA0390_OVERALL_PROJECT_AUDIT_PROFILE_OK`
- `NA0390_CODE_CRYPTO_AUDIT_PROFILE_OK`
- `NA0390_LOCAL_OPS_HISTORY_BACKUP_AUDIT_PROFILE_OK`
- `NA0390_PUBLIC_CLAIM_REVIEW_AUDIT_PROFILE_OK`
- `NA0390_TARGETED_INCIDENT_AUDIT_PROFILE_OK`
- `NA0390_EXTERNAL_STANDARDS_TECH_WATCH_FUTURE_GATED_OK`
- `NA0390_AUDIT_TRIGGER_POLICY_OK`
- `NA0390_AUDIT_DEPTH_LEVELS_OK`
- `NA0390_AUDIT_SEVERITY_TAXONOMY_OK`
- `NA0390_AUDIT_QUEUE_INSERTION_POLICY_OK`
- `NA0390_TEMP_OUTPUT_BOUNDARY_OK`
- `NA0390_NO_DURABLE_REPORT_WRITE_OK`
- `NA0390_NO_BACKGROUND_SCHEDULER_OK`
- `NA0390_NO_WORKFLOW_CHANGE_OK`
- `NA0390_NO_DEPENDENCY_CHANGE_OK`
- `NA0390_NO_RUNTIME_CHANGE_OK`
- `NA0390_NO_SECRET_MATERIAL_OK`
- `NA0390_NO_BUG_FREE_CLAIM_OK`
- `NA0390_NO_CRYPTO_PERFECT_CLAIM_OK`
- `NA0390_NO_METADATA_FREE_CLAIM_OK`
- `NA0390_NO_ANONYMITY_CLAIM_OK`
- `NA0390_NO_UNTRACEABLE_CLAIM_OK`
- `NA0390_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0390_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0390_METADATA_RUNTIME_ROUTINE_AUDIT_CADENCE_OK`

## Positive cases

Validated positive fixtures:

- valid full cadence policy;
- valid minimal cadence policy;
- overall project audit profile;
- code/crypto audit profile;
- local-ops/history/backup audit profile;
- public-claim/external-review audit profile;
- targeted incident/regression audit profile;
- external standards / threat / technology watch profile marked future-gated;
- PR-count trigger;
- NA-count trigger;
- risk/event trigger;
- public-paper precondition trigger;
- severity taxonomy;
- queue insertion policy;
- temp-output report boundary;
- JSON summary fixture.

## Negative/fail-closed cases

Validated rejection fixtures:

- missing audit profiles;
- missing code/crypto profile;
- missing public-claim boundary;
- unknown severity;
- CRITICAL finding without stop/escalation;
- finding that auto-promotes READY;
- multiple READY candidates;
- durable report output requested;
- scheduler/cron/workflow requested;
- background automation requested;
- secret sentinel in report text;
- prohibited bug-free claim fixture;
- prohibited perfect-crypto claim fixture;
- prohibited production-ready claim fixture;
- prohibited public-internet-ready claim fixture;
- prohibited metadata-free/anonymity/untraceable claim fixture;
- prohibited external-review-complete claim fixture;
- public technical paper allowed without preconditions;
- external standards watch performed instead of future-gated;
- malformed JSON;
- unknown top-level key.

## Temp-output cadence simulation proof

Simulation output root:

`/srv/qbuild/tmp/NA0390_routine_audit_cadence_20260530T193850-0500/simulate`

Scenarios:

- quick overall audit due by PR count: one `overall_project` recommendation,
  `BACKLOG_CANDIDATE`, `ready_mutation=false`, `audit_executed=false`;
- code/crypto audit due before public technical paper: one `code_crypto`
  recommendation, `BACKLOG_CANDIDATE`, `ready_mutation=false`,
  `audit_executed=false`;
- public-claim audit due before website/public-doc change: one
  `public_claim_external_review` recommendation, `BACKLOG_CANDIDATE`,
  `ready_mutation=false`, `audit_executed=false`;
- external standards / threat / technology watch request: one
  `external_standards_threat_technology_watch` recommendation,
  `BACKLOG_CANDIDATE`, `future_gated=true`, `audit_executed=false`;
- no audit due: zero recommendations.

## Report output path and SHA-256

Validation summary:

- path: `/srv/qbuild/tmp/NA0390_routine_audit_cadence_20260530T193850-0500/validate/validation_summary.json`;
- SHA-256: `3a762629c6a2e3a5fd16ee6b83a3338e25b3ef98f7d813871617090fbdf1b547`.

Simulation summary SHA-256 values:

- code/crypto before public paper: `c3815fa8ac12ab6611697141f0053a3120fc908a7f91d768f58d5799f2aa9a90`;
- external watch future-gated: `2c0d50ece27a6b820e47e62471315f8dee442129b8bd2c0af368939e6a0d3d63`;
- no audit due: `3d1a064b60b459578ff2656858069bb83a10288f015a9208663ff47162ee01db`;
- public claim before website: `3fa943d81d373e748f3c0dc0f7dc71097760e4f6a193bfece02e6d0b2a685f01`;
- quick overall by PR count: `dde9f718f51df416ac471999285364da9978b5d22d5f6f50c13fe3be8931f07f`.

Combined temp-output file-list digest:

`aa6527ec11c995fe82a0a99f44011a2112563b55440c87785969ab3086c98b44`

## No scheduler / no background automation proof

The helper contains no scheduler, cron, timer, workflow generation, background
job, network, GitHub, subprocess, or shell execution path. Fixture policy
rejects scheduler, cron, workflow, and background-job requests.

## No durable report proof

Generated files were written only under:

`/srv/qbuild/tmp/NA0390_routine_audit_cadence_20260530T193850-0500/`

No durable audit report path, Codex archive path, qsl-protocol generated report
store, workflow, cron file, or backup path was created.

## No auto-READY insertion proof

The queue insertion policy requires:

- `auto_promote_ready=false`;
- `max_ready_candidates=0`;
- `allowed_candidate_statuses=["BACKLOG_CANDIDATE"]`;
- `candidate_output_only=true`;
- `one_ready_required=true`.

Simulation output contains `ready_mutation=false` and no READY mutations.

## External standards / threat / technology watch future-gated proof

The external standards / threat / technology watch profile is represented only
as a future-gated profile. NA-0390 did not browse, perform web/news watching,
fetch external sources, or make current technology claims.

The selected successor remains a separate authorization plan for a future
source-cited watch process.

## No-secret / no bug-free / no perfect-crypto proof

The helper rejects a secret sentinel in report text and rejects prohibited
public/readiness/privacy claim fixtures. The harness does not claim absence of
bugs and does not claim perfect crypto.

## Backup/local continuity caveat

`/backup/qsl` was mounted read-only for status checks during this lane, but no
backup, restore, deploy, off-host target setup, key handling, script/timer/fstab
mutation, or backup plan edit was performed.

Temp proof under `/srv/qbuild/tmp` is not durable audit evidence. Future durable
audit report storage requires separate backup-impact review.

Same-host local continuity remains distinct from disaster recovery.

## Runtime/service/dependency/workflow boundary

NA-0390 changed no runtime, service, protocol, crypto, qsc/qsp/qsl
implementation, qshield runtime, dependency, Cargo, workflow, or
public-safety-gate path.

## qsl-server/qsl-attachments boundary

qsl-server PR #56 remained read-only bounded harness evidence at `d40e6003fdf0`.
qsl-attachments PR #37 remained read-only service-local prerequisite evidence at
`96b9352bd63`.

No qsl-server or qsl-attachments repository was cloned, edited, or used as
public-internet readiness proof.

## Public-claim boundary

NA-0390 does not prove or claim:

- no production readiness claim;
- no public-internet readiness claim;
- no external review completion claim;
- no metadata-free behavior claim;
- no anonymity claim;
- no untraceability claim;
- no absence-of-bugs claim;
- no perfect crypto claim.

## Public technical paper timing

Public technical paper work remains future-gated pending a fresh public-claim
boundary audit, external-review readiness assessment, current code/crypto audit
status, service/backup/restore status, and evidence mapping.

## Selected successor

Selected successor:

`NA-0391 -- QSL External Standards / Threat / Technology Watch Authorization Plan`

## Rejected alternatives

- creating a workflow, cron job, timer, or scheduler now;
- writing durable audit reports now;
- running a full project audit or full code/crypto audit now;
- starting public technical paper work now;
- mutating runtime, crypto, qsl-server, qsl-attachments, backup scripts, or
  workflows now;
- storing audit reports under local Codex ops paths before backup-impact review.

## Next recommendation

After NA-0390 merges and public-safety is green, close NA-0390 and restore
NA-0391 as the exact future-gated external standards / threat / technology watch
authorization plan. Do not implement NA-0391 during closeout.
