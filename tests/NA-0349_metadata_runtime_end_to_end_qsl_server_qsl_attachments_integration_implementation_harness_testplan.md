Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0349 Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0349 implements or precisely blocks the bounded qsl-server /
qsl-attachments integration harness after NA-0348, without deployment,
dependency, workflow, runtime-protocol, or public-claim drift.

## Protected invariants

- qsl-server and qsl-attachments source/authority/CI must be refreshed before
  mutation.
- qsl-server changes, if any, stay in focused test/source files authorized by
  live scope.
- qsl-attachments changes, if any, stay in focused test/source files authorized
  by live scope.
- qsl-protocol companion changes stay in governance evidence, testplan,
  decisions, traceability, and rolling journal paths.
- No qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, branch-protection,
  public-safety, deploy, or secret configuration change.
- No claim that attachment size, timing metadata, traffic shape, all metadata,
  anonymity, metadata-free behavior, untraceability, production readiness,
  public-internet readiness, or external-review completion is achieved.

## Allowed implementation scope

Expected service implementation scope is limited to:

- focused qsl-server tests under the existing qsl-server test layout;
- focused qsl-attachments tests under the existing qsl-attachments test layout;
- `src/lib.rs` or `src/main.rs` only if exact source evidence shows a minimal
  helper is required and live scope permits it.

## Forbidden implementation scope

- qsl-server or qsl-attachments manifests/lockfiles unless a future live scope
  explicitly authorizes them.
- `.github/**`, deployment scripts, service manager files, branch protection,
  public-safety configuration, public docs, website, README, START_HERE,
  qshield runtime, qsc/qsp, protocol, crypto, key schedules, dependencies, or
  production deployment behavior.

## Allowed qsl-protocol companion scope

- `docs/governance/evidence/NA-0349_metadata_runtime_end_to_end_qsl_server_qsl_attachments_integration_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Prior evidence-plan review requirements

Review and cite:

- live NA-0349 queue entry;
- NA-0348 evidence plan and closeout;
- qsl-server PR #55 proof and limitations;
- qsl-attachments PR #37 proof and limitations;
- qshield demo/reference proof and limitations;
- D-0678 and D-0679.

## Source / authority refresh requirements

For both qsl-server and qsl-attachments, record:

- local path and SHA;
- remote default branch SHA;
- PR #55 / PR #37 merge state as applicable;
- latest main required CI status;
- viewer permission;
- branch protection and required checks;
- open PR list;
- `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, and `COMPLETE_CI`, or exact blocker.

## Implementation strategy requirements

Accepted strategies:

- qsl-server-only harness with deterministic qsl-attachments contract fixture;
- qsl-attachments-only harness with deterministic qsl-server contract fixture;
- paired qsl-server and qsl-attachments harness if exact scope permits both;
- exact blocker if deployment, secrets, dependency, workflow, public ingress,
  or broad refactor is required.

The selected strategy must keep qsl-server/qsl-attachments production boundaries
explicit.

## Harness / marker requirements

The implementation or companion evidence must record:

- `NA0349_END_TO_END_SOURCE_AUTHORITY_OK`
- `NA0349_QSL_SERVER_MAIN_PROOF_OK`
- `NA0349_QSL_ATTACHMENTS_MAIN_PROOF_OK`
- `NA0349_QSL_SERVER_QSL_ATTACHMENTS_CONTRACT_OK`
- `NA0349_ROUTE_API_BOUNDARY_OK`
- `NA0349_ATTACHMENT_OBJECT_LIFECYCLE_OK`
- `NA0349_SIZE_CLASS_FLOW_BOUNDARY_OK`
- `NA0349_OPAQUE_PAYLOAD_BOUNDARY_OK`
- `NA0349_ROUTE_TOKEN_AUTH_BOUNDARY_OK`
- `NA0349_QUOTA_RATE_BOUNDARY_OK`
- `NA0349_RETENTION_PURGE_CONSISTENCY_OK`
- `NA0349_BACKUP_RESTORE_BOUNDARY_OK`
- `NA0349_LOG_REDACTION_BOUNDARY_OK`
- `NA0349_MONITORING_BOUNDARY_OK`
- `NA0349_DEPLOY_ROLLBACK_BOUNDARY_OK`
- `NA0349_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0349_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`
- `NA0349_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0349_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0349_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0349_NO_METADATA_FREE_CLAIM_OK`
- `NA0349_METADATA_RUNTIME_END_TO_END_INTEGRATION_OK`

## CI requirements

For each mutated service repo, run repository-local required validation and
merge only after required PR checks pass.

Expected qsl-server validation:

- `bash scripts/ci/test_aws_update_and_verify.sh`
- `bash scripts/ci/test_update_checksum.sh`
- `cargo fmt --all -- --check`
- focused NA-0349 test
- `cargo test -q`
- `cargo clippy -q -- -D warnings`

Expected qsl-attachments validation:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo build --locked`
- `cargo test --locked`

## qshield demo boundary requirements

Any qshield demo reference must remain reference/oracle evidence only and must
not be presented as qsl-server/qsl-attachments production proof.

## Backup / deploy / secrets requirements

- Tests must not require production secrets.
- Tests must not create durable non-rebuildable artifacts outside source trees
  or `/srv/qbuild/tmp`.
- No production deployment, rollback, service manager, public ingress, or
  backup configuration may be changed.
- If a durable external artifact or backup-plan change is required, stop.

## Public claim boundary requirements

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

## Required qsl-protocol local checks

- `git diff --check`
- queue and decisions helper
- scope guard for authorized paths
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof
- goal-lint / PR body preflight
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- qsc `send_commit`
- formal/model checks

## CI expectations

- qsl-server required `rust` passes if qsl-server is mutated.
- qsl-attachments required `rust` remains green if qsl-attachments is mutated.
- qsl-protocol PR checks complete successfully.
- qsl-protocol `public-safety` remains required and green before and after
  qsl-protocol merge.

## Successor handoff

If NA-0349 succeeds and the qsl-protocol companion merges, closeout may restore
exactly:

`NA-0350 -- Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`

The closeout must not implement NA-0350.
