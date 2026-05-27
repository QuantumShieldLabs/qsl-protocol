# NA-0367 Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

## Objective

Validate that NA-0367 produces an evidence-bounded target-access and host-identity prerequisite plan for the SSH/SFTP-compatible off-host target class without connecting to any target, scanning host keys, handling secrets, installing tools, initializing repositories, running backup/restore/deploy/rollback operations, or changing runtime/service/backup configuration.

## Protected Invariants

- NA-0367 is prerequisite planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime is unchanged.
- qsc/qsp/protocol/crypto/key-schedule implementation is unchanged.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths, rollback paths, remote destinations, key paths, passphrase paths, and recovery-envelope paths are unchanged.
- No remote/off-host connection, host-key scan, repository init, tool installation, backup, restore, deploy, rollback, real restore target creation/mount/copy, real key generation, key upload, passphrase collection, credential handling, private-key inspection, recovery-envelope content creation, or secret handling occurs.
- No claim states or implies production readiness, public-internet readiness, external-review completion, anonymity, metadata-free behavior, untraceability, hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all metadata, complete off-host backup, complete disaster recovery, real restore completion, real key custody implementation, or real key recovery implementation.

## Allowed Scope

Allowed files:

- `docs/governance/evidence/NA-0367_metadata_runtime_off_host_backup_target_access_host_identity_prerequisite_plan.md`
- `tests/NA-0367_metadata_runtime_off_host_backup_target_access_host_identity_prerequisite_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime, qsc/qsp implementation, protocol/crypto, dependencies, workflows, README, START_HERE, docs/public, website, service configuration, deployment scripts, rollback scripts, backup scripts, timers, fstab, local backup configuration, off-host destinations, restore paths, key material, passphrase paths, credential paths, recovery-envelope content, and any real target/repository/tool setup.

## Prior Target-Access Blocker Review Requirements

Validation must confirm the evidence reviews and preserves:

- NA-0366 real target/tool blocker-resolution result.
- NA-0365 no-secret isolated restore harness boundary.
- NA-0363 no-secret off-host target/tool harness boundary.
- NA-0361 no-secret key custody/recovery harness boundary.
- NA-0355 SSH/SFTP-compatible target class and restic-style repository class selection.

## Source/Authority Refresh Requirements

Validation must confirm read-only qsl-server and qsl-attachments status:

- local path and SHA if present;
- remote default branch SHA if available;
- PR #56 / PR #37 merge status;
- latest main CI status if available;
- viewer permission if available;
- branch protection if available;
- open PR list;
- classification as fresh/stale/unknown source, complete/partial/blocked authority, and complete/partial/blocked CI.

No qsl-server or qsl-attachments mutation is allowed.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh Requirements

Validation must confirm the evidence records:

- `/backup/qsl` mount status if available;
- local snapshot, manifest, and log availability;
- backup plan/status file presence;
- qsl-backup syntax/preflight/list results if safely run;
- installed-tool status for restic, borg, rclone, age, gpg, ssh, and rsync;
- off-host repository, target host identity, credential, backup tool, repository-init, key custody, key recovery, recovery-envelope, real restore target, monitoring, alerting, and runbook status;
- local history/directive folder coverage gaps;
- D132 bundle status.

Expected classifications include `LOCAL_CONTINUITY_PROVEN`, `NO_SECRET_DRY_RUN_RESTORE_PROVEN`, `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`, `NO_SECRET_TARGET_TOOL_PROVEN`, `NO_SECRET_ISOLATED_RESTORE_PROVEN`, `TARGET_ACCESS_PREREQUISITE_READY_FOR_PLANNING`, `TARGET_ACCESS_NOT_READY_FOR_CONNECTION`, `HOST_IDENTITY_NOT_READY`, `CREDENTIAL_BOUNDARY_NOT_READY`, `CAPACITY_RETENTION_NOT_READY`, `MONITORING_NOT_READY`, `REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY`.

## Target Class/Candidate Requirements

Evidence must include:

- `TARGET_CLASS_SELECTED`
- `TARGET_CANDIDATE_ABSENT`
- `TARGET_CANDIDATE_OPERATOR_REQUIRED`
- `TARGET_CANDIDATE_READY_FOR_NO_CONNECTION_PLAN`
- `TARGET_CANDIDATE_NOT_READY_FOR_CONNECTION`

The plan must name missing fields before any future target connection.

## Host Identity Requirements

Evidence must include:

- `HOST_IDENTITY_ABSENT`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `HOST_IDENTITY_CAPTURE_REQUIRES_FUTURE_AUTHORIZATION`
- `HOST_IDENTITY_PINNING_REQUIRED`
- `KNOWN_HOSTS_MUTATION_FORBIDDEN_NOW`

The plan must not present host-identity planning as verified host identity.

## Credential Boundary Requirements

Evidence must include:

- `CREDENTIAL_BOUNDARY_ABSENT`
- `SECRET_HANDLING_FORBIDDEN_NOW`
- `CREDENTIAL_MODEL_SELECTION_REQUIRED`
- `CREDENTIAL_STORAGE_BOUNDARY_REQUIRED`
- `NO_SECRET_CREDENTIAL_PLACEHOLDER_ONLY`

The plan must not inspect, create, upload, collect, copy, log, or handle secret material.

## Capacity/Quota/Retention Requirements

Evidence must include:

- `CAPACITY_BOUNDARY_ABSENT`
- `QUOTA_BOUNDARY_ABSENT`
- `RETENTION_BOUNDARY_ABSENT`
- `CAPACITY_EVIDENCE_REQUIRED`
- `CAPACITY_NOT_READY_FOR_BACKUP`

The plan must not assert real target capacity without a named target and evidence source.

## Monitoring/Alerting/Runbook Requirements

Evidence must include:

- `MONITORING_BOUNDARY_ABSENT`
- `ALERTING_BOUNDARY_ABSENT`
- `OPERATOR_RUNBOOK_REQUIRED`
- `MONITORING_MODEL_ONLY`
- `REAL_MONITORING_SETUP_FORBIDDEN_NOW`

The plan must not configure real monitoring.

## Local-Ops Requirements

Evidence must include:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

The plan must decide whether local-ops/history-index is the next successor or a later prerequisite.

## Public-Claim Boundary Requirements

Evidence must state that:

- target-access prerequisite planning is not target setup;
- host-identity prerequisite planning is not host identity proof;
- no remote connection occurs;
- no credential is handled;
- no-secret target/tool harness evidence is not real off-host backup;
- no-secret isolated restore harness evidence is not real restore;
- no-secret key harness evidence is not real key custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- external review is not complete;
- no public claim implies production/public-internet readiness, metadata-free/anonymity/untraceable behavior, or hidden attachment size, timing metadata, traffic shape, or all metadata.

## Decision Matrix Requirements

The evidence must include a matrix covering:

- target class;
- target candidate;
- host identity;
- credential model;
- credential storage;
- capacity/quota;
- retention/purge;
- monitoring/alerting;
- operator runbook;
- local-ops/backup-plan;
- external review/public claims.

The matrix must identify status, evidence, blocker, next action, whether the item must precede real off-host backup, whether it can be modeled no-secret only, whether it is ready for implementation authorization, and whether it should become NA-0368.

## Successor Selection Requirements

Evidence must select exactly one successor and must not implement it. Expected successor:

`NA-0368 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Plan`

## Required Local Checks

Minimum local validation:

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime backup/deploy/rollback harness syntax and fixture run
- restore dry-run harness syntax and fixture run
- key custody/recovery no-secret harness syntax and fixture run
- off-host target/tool no-secret harness syntax and fixture run
- isolated restore no-secret harness syntax and fixture run
- qsl evidence helper queue/decisions checks
- qsl evidence helper scope guard for allowed paths
- qsl evidence helper link-check
- qsl evidence helper leak-scan in added mode
- classifier proof for changed paths

Additional heavy checks should run when feasible under the directive, including qshield-cli build/test, demo smoke/stress/soak, metadata phase-2 harnesses, metadata conformance smoke, qsc send_commit, formal models, NA-0310 suite-id vector JSON parse, NA-0310 refimpl oracle, and full refimpl tests.

## CI Expectations

The PR must include a standalone `Goals: G1, G2, G3, G4, G5` line near the top of the body. Required checks, including `public-safety`, must pass before merge. Merge must use a normal merge with `--match-head-commit`, no admin bypass, no squash, no rebase, no direct push, and no delete-branch flag.

## Successor Handoff

After Packet Q merges and post-merge public-safety is green, optional closeout may mark NA-0367 DONE and restore the exact selected NA-0368 successor as the sole READY item. The closeout must not implement NA-0368.
