Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0352 Metadata Runtime Production Backup Deploy Rollback Implementation Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0352 adds a bounded qsl-protocol fixture and harness proving
production backup/deploy/rollback authorization boundaries without performing
live backup, restore, deploy, rollback, purge, service mutation, secret-dependent
operation, or local backup script/timer/fstab mutation.

## Protected invariants

- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only only.
- No qshield runtime, qsc, qsp, protocol, crypto, key-schedule, dependency,
  workflow, branch-protection, public-safety, website, README, START_HERE,
  docs/public, backup-script, timer, fstab, service, deploy, rollback, restore,
  purge, or production operation mutation.
- No claim that production readiness, public-internet readiness, external
  review completion, anonymity, metadata-free behavior, untraceability, hidden
  attachment size, hidden timing metadata, hidden traffic shape, hidden all
  metadata, or complete disaster recovery is achieved.
- qsl-server PR #56 remains bounded end-to-end harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Allowed scope

- `scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh`
- `inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json`
- `docs/governance/evidence/NA-0352_metadata_runtime_production_backup_deploy_rollback_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc, qsp, qsl, protocol, crypto, key-schedule, service,
  formal, tool/refimpl, app runtime, qsc-desktop, website, README, START_HERE,
  docs/public, workflow, Cargo, dependency, branch-protection, and public-safety
  configuration mutation.
- Production backup, deploy, rollback, restore, purge, public ingress,
  monitoring mutation, secret-dependent operation, backup source-list edit,
  backup script/timer/fstab mutation, and local system mutation.

## Prior authorization review requirements

Review and cite:

- live NA-0352 queue entry;
- NA-0351 authorization evidence and testplan;
- NA-0351 closeout testplan;
- NA-0350 hardening plan;
- NA-0349 end-to-end implementation evidence;
- D-0684 and D-0685;
- TRACEABILITY boundaries for qsl-server, qsl-attachments, and qshield demo
  evidence.

## Source/authority refresh requirements

For qsl-server and qsl-attachments, record:

- local path and SHA if present;
- remote default branch SHA;
- PR merge status and merge SHA;
- viewer permission;
- branch protection summary;
- latest main CI status;
- open PR list;
- `FRESH_SOURCE`, `STALE_SOURCE`, or `UNKNOWN_SOURCE`;
- `COMPLETE_AUTHORITY`, `PARTIAL_AUTHORITY`, or `BLOCKED_AUTHORITY`;
- `COMPLETE_CI`, `PARTIAL_CI`, or `BLOCKED_CI`.

## Harness fixture requirements

The fixture must include:

- `schema_version`;
- `artifact_class: boundary_harness_not_operation`;
- `local_continuity_status`;
- `off_host_backup_status`;
- `qsl_server_source_scope`;
- `qsl_attachments_source_scope`;
- `runtime_config_roots`;
- `service_data_roots`;
- `backup_scope`;
- `restore_scope`;
- `deploy_scope`;
- `rollback_scope`;
- `secrets_env_scope`;
- `monitoring_logging_scope`;
- `public_ingress_scope`;
- `forbidden_operations`;
- `future_authorized_operations`;
- `claim_boundaries`;
- `required_markers`;
- `no_secret_sentinels`.

## Harness script requirements

The harness must:

- parse the fixture deterministically;
- validate all required fields;
- validate operation classifications as `AUTHORIZED`, `FORBIDDEN`, or
  `FUTURE_GATE`;
- refuse live backup, restore, deploy, rollback, purge, service mutation,
  secret-dependent operation, backup script/timer/fstab mutation, or public
  ingress;
- refuse local continuity as full disaster recovery;
- refuse off-host encrypted backup completion without evidence;
- refuse deploy, rollback, or restore as currently authorized;
- require the forbidden-operation set and claim-boundary set;
- scan sentinel labels and output for secret findings;
- write only a temporary proof artifact under `/srv/qbuild/tmp/NA-0352_*`;
- emit `SECRET_FINDING_COUNT 0` only when clean;
- exit nonzero on invariant violation.

## Marker requirements

The harness must emit:

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

## Artifact safety requirements

- Artifact path must be under `/srv/qbuild/tmp`.
- Artifact must not contain raw tokens, secrets, keys, passphrases,
  credentials, production endpoints, route-token values, bearer values, fetch
  capabilities, plaintext, or private service data.
- Harness output must include `SECRET_FINDING_COUNT 0`.
- No durable artifact location outside current backup scope may be required.

## Backup-plan impact requirements

The evidence must state:

- whether NA-0352 requires a current backup-plan update;
- whether changed paths remain under qsl-protocol source in `/srv/qbuild/work`;
- whether temporary artifacts remain under `/srv/qbuild/tmp`;
- future backup-plan triggers for off-host backup, production service roots,
  deploy configs, rollback artifacts, restore fixtures, monitoring artifacts,
  local directive/request/journal/ops history, and Codex response history.

## Deploy/rollback/restore/secrets/monitoring requirements

The harness and evidence must state:

- live deploy is forbidden;
- live rollback is forbidden;
- live restore is forbidden;
- live backup and purge are forbidden;
- secret values and secret-dependent tests are forbidden;
- monitoring/logging mutation is forbidden;
- only fixture, dry-run, or read-only future proof may be authorized by a later
  directive.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that:

- public ingress remains future-gated;
- attachment size is not hidden;
- timing metadata is not hidden;
- traffic shape is not hidden;
- padding does not hide all metadata;
- qshield demo evidence is not production-service evidence.

## External-review boundary requirements

The evidence must state that external review is not complete and that this
harness is not external-review-complete evidence.

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
- complete disaster recovery from same-host local continuity.

Negated, prohibited, future-gated, or caveated references are allowed.

## Workflow-support deferral requirements

Record whether workflow-support/history-index work would reduce friction, but do
not implement those local-ops items in NA-0352.

## Required local checks

Run and record:

```bash
bash -n scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh
bash scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json
python3 -m json.tool inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json >/dev/null
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Run the directive's heavier qshield/qsc/formal/refimpl checks where feasible.

## CI expectations

- PR body includes standalone `Goals: G1, G2, G3, G4, G5`.
- Goal-lint passes.
- Required PR checks pass normally.
- `public-safety` remains required and green before merge.
- Merge uses normal merge with `--match-head-commit`.
- No squash, rebase, direct push, admin bypass, or delete-branch flag is used.
- Post-merge `public-safety` completes success.

## Successor handoff

Recommended successor after merge and public-safety success:

`NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`

Closeout, if executed, must restore exactly one READY item and must not
implement NA-0353.
