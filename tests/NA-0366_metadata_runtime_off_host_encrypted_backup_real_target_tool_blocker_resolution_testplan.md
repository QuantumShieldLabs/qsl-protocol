Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0366 Metadata Runtime Off-Host Encrypted Backup Real Target Tool Blocker Resolution Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0366 produces evidence-backed blocker resolution for real
off-host encrypted backup target/tool work after NA-0365, NA-0363, NA-0361, and
NA-0359 no-secret harnesses, then selects one exact NA-0367 successor without
implementing it.

## Protected Invariants

- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- No qshield runtime, qsc/qsp/protocol/crypto/key-schedule, dependency,
  workflow, branch-protection, public-safety, website, README, START_HERE, or
  docs/public change is introduced.
- No local backup script, timer, fstab, service unit, source list, restore
  path, off-host destination, key path, passphrase path, recovery envelope, or
  monitoring configuration is mutated.
- No backup, restore, deploy, rollback, remote connection, off-host setup,
  repository initialization, tool installation, real restore target
  creation/mount/copy, real key generation, key upload, passphrase collection,
  private-key inspection, recovery-envelope content creation, or secret handling
  occurs.
- Local continuity backup is not complete disaster recovery.
- No-secret harness evidence is not real off-host backup, real restore
  execution, real key custody, or real key recovery.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceability,
  hidden attachment size, hidden timing, hidden traffic shape, all metadata
  hidden, off-host backup completion, real restore completion, real key custody
  implementation, real key recovery implementation, or disaster recovery
  completion.

## Allowed Scope

- `docs/governance/evidence/NA-0366_metadata_runtime_off_host_encrypted_backup_real_target_tool_blocker_resolution.md`
- `tests/NA-0366_metadata_runtime_off_host_encrypted_backup_real_target_tool_blocker_resolution_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server mutation.
- qsl-attachments mutation.
- qshield runtime mutation.
- qsc/qsp/protocol/crypto implementation mutation.
- Cargo or dependency mutation.
- `.github` workflow mutation.
- branch-protection or public-safety mutation.
- website, external website, README, START_HERE, or docs/public mutation.
- local backup script/timer/fstab/service/source-list mutation.
- off-host setup, remote connection, repository init, tool install, backup,
  restore, deploy, rollback, real restore target creation/mount/copy, key
  generation, key upload, passphrase collection, private-key inspection,
  recovery-envelope content creation, or secret handling.

## Prior Real Target/Tool Blocker Review Requirements

Evidence must review:

- live NA-0366 queue scope;
- NA-0365 no-secret isolated restore harness;
- NA-0363 no-secret target/tool harness;
- NA-0361 no-secret key custody/recovery harness;
- NA-0359 restore dry-run harness;
- NA-0355 target/tool class selection;
- qsl-server PR #56 and qsl-attachments PR #37 boundaries;
- local backup/tool/key/off-host/restore state;
- public-claim boundaries and stop conditions.

## Source/Authority Refresh Requirements

Evidence must record qsl-server and qsl-attachments:

- local path and SHA if present;
- remote/default branch SHA if available;
- PR merge status and merge SHA;
- branch protection state;
- open PR list;
- latest main CI status;
- viewer permission if available;
- classification as `FRESH_SOURCE`, `STALE_SOURCE`, or `UNKNOWN_SOURCE`;
- classification as `COMPLETE_AUTHORITY`, `PARTIAL_AUTHORITY`, or
  `BLOCKED_AUTHORITY`;
- classification as `COMPLETE_CI`, `PARTIAL_CI`, or `BLOCKED_CI`.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh Requirements

Evidence must record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status file and backup plan file presence;
- qsl-backup syntax/preflight/list results if safe;
- installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`, `ssh`,
  and `rsync`;
- off-host target configuration presence or absence;
- repository presence or absence;
- SSH/SFTP target host identity presence or absence;
- credential boundary presence or absence;
- key custody/recovery/recovery-envelope status;
- real restore drill and isolated target status;
- monitoring/alerting and operator runbook status;
- local history/directive folder coverage status;
- D132 bundle status.

Required classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_PROVEN`
- `REAL_TARGET_ACCESS_NOT_READY`
- `REAL_TOOL_INSTALL_NOT_READY`
- `REAL_REPOSITORY_INIT_NOT_READY`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `REAL_ISOLATED_RESTORE_BLOCKED`
- `OFF_HOST_BACKUP_NOT_READY`

## Target-Access Blocker Requirements

Evidence must classify:

- `TARGET_ACCESS_BLOCKED_NO_TARGET`
- `TARGET_ACCESS_BLOCKED_NO_HOST_IDENTITY`
- `TARGET_ACCESS_BLOCKED_NO_CREDENTIAL_BOUNDARY`
- `TARGET_ACCESS_BLOCKED_NO_CAPACITY_RETENTION`
- `TARGET_ACCESS_BLOCKED_NO_MONITORING`
- `TARGET_ACCESS_NOT_AUTHORIZED_NOW`

If `TARGET_ACCESS_READY_FOR_AUTHORIZATION` is selected, evidence must prove a
named target, host identity, credential boundary, capacity/retention boundary,
and monitoring boundary without using secrets or connecting remotely.

## Host-Identity Blocker Requirements

Evidence must require host identity proof before any real remote access:

- host key or equivalent identity capture method;
- verification/pinning boundary;
- mismatch handling;
- operator trust boundary;
- no credential disclosure in evidence.

## Tool-Install Blocker Requirements

Evidence must classify:

- `TOOL_INSTALL_BLOCKED_TOOL_ABSENT`
- `TOOL_INSTALL_BLOCKED_NO_INSTALL_AUTHORITY`
- `TOOL_INSTALL_BLOCKED_NO_VERSION_PIN`
- `TOOL_INSTALL_BLOCKED_NO_REPOSITORY_INIT_BOUNDARY`
- `TOOL_INSTALL_BLOCKED_NO_RESTORE_CHECK_BOUNDARY`
- `TOOL_INSTALL_NOT_AUTHORIZED_NOW`

If `TOOL_INSTALL_READY_FOR_AUTHORIZATION` is selected, evidence must prove
package/version/install/rollback boundaries and explain why installation is safe
before target, repository, and key prerequisites.

## Repository-Init Blocker Requirements

Evidence must block repository initialization until target, tool, credential,
key custody, restore-check, retention, monitoring, and backup-plan prerequisites
are explicit. NA-0366 must not initialize any repository.

## Key/Recovery Blocker Requirements

Evidence must classify:

- `KEY_CUSTODY_BLOCKED_NO_REAL_CUSTODY`
- `KEY_RECOVERY_BLOCKED_NO_REAL_RECOVERY`
- `RECOVERY_ENVELOPE_BLOCKED_NO_CONTENT_AUTHORIZATION`
- `KEY_ROTATION_BLOCKED_NO_RUNBOOK`
- `KEY_CUSTODY_NOT_AUTHORIZED_NOW`

NA-0366 must not generate keys, upload keys, collect passphrases, inspect private
keys, create recovery-envelope content, or handle secret material.

## Restore Blocker Requirements

Evidence must classify:

- `RESTORE_BLOCKED_NO_REAL_TARGET`
- `RESTORE_BLOCKED_NO_KEY`
- `RESTORE_BLOCKED_NO_REPOSITORY`
- `RESTORE_BLOCKED_NO_CLEANUP`
- `RESTORE_BLOCKED_NO_MONITORING`
- `RESTORE_BLOCKED_NO_RUNBOOK`
- `RESTORE_NOT_AUTHORIZED_NOW`

NA-0366 must not create/mount/copy a real restore target and must not run a real
restore.

## Local-Ops Blocker Requirements

Evidence must classify:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_BLOCKED_NO_MANIFEST`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`

Evidence must record whether workflow-support improvements would reduce
friction and whether they should be the next successor.

## Public-Claim Boundary Requirements

Evidence must explicitly preserve that:

- external review is not complete;
- no-secret target/tool harness is not real off-host backup;
- no-secret isolated restore harness is not real restore;
- no-secret key harness is not real key custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- production/public-internet readiness is not claimed;
- attachment size, timing metadata, traffic shape, and all metadata are not
  claimed hidden;
- metadata-free, anonymity, and untraceable behavior are not claimed.

## Decision Matrix Requirements

Evidence must include a decision matrix covering:

- target access / host identity;
- tool installation / version pin;
- repository initialization;
- credential / secret handling;
- key custody;
- key recovery;
- recovery envelope;
- isolated restore target;
- cleanup;
- monitoring/alerting;
- retention/purge;
- operator runbook;
- backup-plan/local-ops;
- external review;
- public claims.

Each row must record status, evidence, blocker, next action, whether it must
precede real off-host backup, whether it can be modeled no-secret only, whether
it is ready for authorization, and whether it should become NA-0367.

## Successor Selection Requirements

Evidence must select one exact NA-0367 successor and reject alternatives. NA-0367
must not be implemented.

Expected selected successor:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`

## Required Local Checks

Run or record the directed validation set, including:

- queue and decisions helper;
- cargo audit and rustls-webpki proof;
- cargo fmt;
- metadata runtime backup/deploy/rollback, restore dry-run, key custody,
  target/tool, and isolated restore no-secret harnesses;
- directly runnable qshield/qsc/refimpl/formal smoke checks where feasible;
- scope guard with the authorized paths;
- link-check;
- leak-scan;
- goal-lint;
- classifier proof for changed paths.

## CI Expectations

- qsl-protocol branch protection must require `public-safety`.
- Packet P PR must merge only after required checks are green.
- Post-merge public-safety must complete success.
- Optional closeout must run only after Packet P merge and green post-merge
  public-safety.

## Successor Handoff

If NA-0366 closes, closeout may restore exactly one successor:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`

The closeout must not implement NA-0367.
