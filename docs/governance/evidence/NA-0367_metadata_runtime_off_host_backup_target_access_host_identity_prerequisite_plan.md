# NA-0367 Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

## Executive Summary

NA-0367 accepts the NA-0366 blocker-resolution result and narrows the next real off-host encrypted backup blocker to target-candidate and host-identity prerequisites. This lane is planning evidence only. It does not create or contact an off-host target, does not scan or pin a live host key, does not handle credentials or secrets, does not initialize a backup repository, does not install tools, does not run backup/restore/deploy/rollback operations, and does not mutate qsl-server, qsl-attachments, qshield runtime, backup scripts, timers, fstab, or service configuration.

The selected class remains the NA-0355 SSH/SFTP-compatible off-host target class with a restic-style encrypted snapshot repository class. The real target candidate is absent; therefore the host identity, credential model, capacity/quota/retention envelope, monitoring/alerting model, and operator runbook cannot proceed to a real connection. The exact selected successor is:

`NA-0368 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Plan`

## Live NA-0367 Scope

Live `NEXT_ACTIONS.md` records `NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan` as the sole READY item. The live scope permits qsl-protocol governance/prerequisite evidence only and preserves no-remote-connection, no-host-key-scan, no-secret, no-tool-installation, no-repository-init, no-backup, no-restore, no-deploy, no-rollback, no-runtime-mutation, and no-public-claim-expansion boundaries.

Allowed NA-0367 output is limited to this evidence file, the matching testplan, D-0716, TRACEABILITY updates, and the rolling operations journal entry. The lane does not authorize implementation work in NA-0368.

## Inherited NA-0366 Blocker-Resolution Result

NA-0366 records `REAL_TARGET_TOOL_IMPLEMENTATION_BLOCKED`. It found no real off-host target, no host identity, no credential boundary, no capacity/retention boundary, no monitoring boundary, no installed restic/borg/rclone/age target tool, no repository-init boundary, no real key custody, no real key recovery, no recovery-envelope content authorization, no rotation/runbook, and no real restore execution.

NA-0366 selected NA-0367 because target access and host identity are the first blockers that must be made explicit before any future real target connection or tool/repository work.

## Inherited NA-0365 No-Secret Isolated Restore Harness

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness. That evidence proves simulated isolated-restore metadata, simulated manifest/checksum relationships, simulated cleanup metadata, simulated monitoring/runbook metadata, fail-closed negative cases, and no-secret proof only. It is not a real restore drill, does not create/mount/copy to a real restore target, and does not prove off-host disaster recovery.

## Inherited NA-0363 No-Secret Off-Host Target/Tool Harness

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and harness. That evidence proves simulated SSH/SFTP target metadata, simulated target identity metadata, simulated restic-style repository metadata, simulated snapshot/check/prune/restore relationships, simulated retention/purge metadata, simulated monitoring/alert metadata, fail-closed negative cases, and no-secret proof only. It is not real off-host backup setup, not a real repository, not a real host identity, and not real tool execution.

## Inherited NA-0361 No-Secret Key Custody/Recovery Harness

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and harness. That evidence proves simulated custody/recovery metadata and fail-closed checks only. It is not real key custody, not real key recovery, does not create key material, does not collect passphrases, does not inspect private keys, and does not create recovery-envelope content.

## Inherited NA-0355 Target/Tool Selection

NA-0355 selected the SSH/SFTP-compatible off-host target class and a restic-style encrypted snapshot repository class at class level only. It did not name a real host, account, path, repository root, credential, host key, tool version pin, schedule, retention policy, alert destination, or operator runbook. NA-0367 inherits that class selection and keeps it at prerequisite level.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

qsl-server read-only refresh:

- Local worktree inspected: `/srv/qbuild/work/NA-0237D/qsl-server`.
- Local HEAD: `d40e6003fdf0`.
- Remote default branch HEAD: `d40e6003fdf0`.
- PR #56: merged at `d40e6003fdf0`.
- Latest main CI observed by `gh run list`: success on the PR #56 merge commit.
- Viewer permission observed by `gh repo view`: `ADMIN`.
- Branch protection observed: required `rust` check, strict updates, force pushes disabled, deletions disabled, admins enforced.
- Open PR list observed: none.
- Classification: `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI`.

qsl-attachments read-only refresh:

- Local worktree inspected: `/srv/qbuild/work/NA-0237D/qsl-attachments`.
- Local HEAD: `96b9352bd63e`.
- Remote default branch HEAD: `96b9352bd63e`.
- PR #37: merged at `96b9352bd63e`.
- Latest main CI observed by `gh run list`: success on the PR #37 merge commit.
- Viewer permission observed by `gh repo view`: `ADMIN`.
- Branch protection observed: required `rust` check, strict updates, force pushes disabled, deletions disabled, admins not enforced in current protection evidence.
- Open PR list observed: none.
- Classification: `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI`.

Boundary: qsl-server PR #56 remains bounded end-to-end harness evidence only. qsl-attachments PR #37 remains service-local prerequisite evidence only. Neither repository was mutated.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` was mounted during startup evidence collection.
- `/backup/qsl` had current same-host continuity snapshots, manifests, and logs.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint and daily snapshots.
- `qsl-backup-daily.timer` was active/waiting.
- `restic`, `borg`, `rclone`, and `age` were absent.
- `gpg`, `ssh`, and `rsync` were present.
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md` was present and states the current backup is same-host continuity, not the only disaster-recovery copy.
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md` was present and records current local continuity backup coverage.
- `/home/victor/work/qsl/codex/directives` and `/home/victor/work/qsl/codex/journals` were absent.
- `/home/victor/work/qsl/codex/responses` and `/home/victor/work/qsl/codex/requests` were present.
- The backup status covers `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild mirrors/evidence/logs/archive, Codex logs/responses, and the backup plan; it does not prove full local ops history coverage for directives, journals, requests, or the full ops tree.
- D132 preservation bundle remained present under `/srv/qbuild/tmp/NA-0322_D132_resume_bundle`.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_PROVEN`
- `TARGET_ACCESS_PREREQUISITE_READY_FOR_PLANNING`
- `TARGET_ACCESS_NOT_READY_FOR_CONNECTION`
- `HOST_IDENTITY_NOT_READY`
- `CREDENTIAL_BOUNDARY_NOT_READY`
- `CAPACITY_RETENTION_NOT_READY`
- `MONITORING_NOT_READY`
- `REAL_TARGET_ACCESS_NOT_AUTHORIZED`
- `OFF_HOST_BACKUP_NOT_READY`

## Target Class and Target Candidate Prerequisite Analysis

The target class is selected, but the target candidate is absent. The missing fields before any future target connection are:

- operator-approved target provider or host owner;
- host name or address;
- account name or access principal;
- target path or repository root;
- trust boundary and administrative control model;
- region/jurisdiction and data-residency expectations if applicable;
- cost/quota envelope;
- retention/purge expectations;
- operator authorization for future host-identity capture method;
- operator authorization for future credential model.

Classifications:

- `TARGET_CLASS_SELECTED`
- `TARGET_CANDIDATE_ABSENT`
- `TARGET_CANDIDATE_OPERATOR_REQUIRED`
- `TARGET_CANDIDATE_READY_FOR_NO_CONNECTION_PLAN`
- `TARGET_CANDIDATE_NOT_READY_FOR_CONNECTION`

## Host Identity Capture / Pinning Prerequisite Analysis

No real host identity exists in repository evidence. Because no target candidate exists, host identity cannot be validated or pinned now. NA-0367 does not mutate `known_hosts`, does not run `ssh-keyscan`, does not run `ssh`, `scp`, `sftp`, or `rsync` to a remote host, and does not claim verified host identity.

Future host identity work must use one of two explicitly authorized models:

- operator-supplied fingerprint and key-algorithm evidence with an independent verification path; or
- a future exact directive authorizing bounded host-key capture, pinning, and transcript/evidence handling for a named target.

Classifications:

- `HOST_IDENTITY_ABSENT`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `HOST_IDENTITY_CAPTURE_REQUIRES_FUTURE_AUTHORIZATION`
- `HOST_IDENTITY_PINNING_REQUIRED`
- `KNOWN_HOSTS_MUTATION_FORBIDDEN_NOW`

## Credential Boundary / Secret-Handling Prerequisite Analysis

No credential type, storage model, passphrase handling model, ssh-agent model, hardware-token model, recovery model, or operator-held-secret boundary is authorized. NA-0367 does not inspect private key material, does not collect passphrases, does not create or upload keys, does not handle credentials, and does not create recovery-envelope content.

Before any target connection, the project needs a credential boundary that states:

- whether access is operator-held, agent-mediated, hardware-token backed, host-scoped key based, or another model;
- where credential references may be recorded without exposing secret material;
- what must never be logged;
- what failure mode occurs if a credential is absent or unavailable;
- how credential rotation, revocation, recovery, and audit evidence are authorized.

Classifications:

- `CREDENTIAL_BOUNDARY_ABSENT`
- `SECRET_HANDLING_FORBIDDEN_NOW`
- `CREDENTIAL_MODEL_SELECTION_REQUIRED`
- `CREDENTIAL_STORAGE_BOUNDARY_REQUIRED`
- `NO_SECRET_CREDENTIAL_PLACEHOLDER_ONLY`

## Capacity / Quota / Retention Prerequisite Analysis

No real target capacity, quota, cost envelope, retention policy, purge authority, or capacity-monitoring source exists. Capacity cannot be asserted without a named target and operator-provided or independently collected capacity evidence.

Future work must define:

- minimum usable free space before initial repository creation;
- minimum free-space stop condition before each backup;
- quota and cost ceiling;
- retention policy and purge authority;
- old-archive compatibility expectations;
- capacity evidence source and refresh cadence;
- behavior when capacity evidence is missing or stale.

Classifications:

- `CAPACITY_BOUNDARY_ABSENT`
- `QUOTA_BOUNDARY_ABSENT`
- `RETENTION_BOUNDARY_ABSENT`
- `CAPACITY_EVIDENCE_REQUIRED`
- `CAPACITY_NOT_READY_FOR_BACKUP`

## Monitoring / Alerting / Runbook Prerequisite Analysis

No real off-host backup monitoring, alert destination, target-capacity alert, retention/purge alert, restore-drill alert, or operator runbook exists. NA-0367 can model the monitoring boundary only; it does not configure monitoring.

Future work must define:

- backup success/failure signal;
- repository check signal;
- retention/purge signal;
- capacity/quota signal;
- restore-drill signal;
- alert destination and escalation owner;
- emergency stop condition;
- manual verification steps;
- evidence retention and redaction rules.

Classifications:

- `MONITORING_BOUNDARY_ABSENT`
- `ALERTING_BOUNDARY_ABSENT`
- `OPERATOR_RUNBOOK_REQUIRED`
- `MONITORING_MODEL_ONLY`
- `REAL_MONITORING_SETUP_FORBIDDEN_NOW`

## Local-Ops / Backup-Plan / History-Index Prerequisite Analysis

Current local backup evidence supports same-host continuity for the qbuild work area and selected Codex logs/responses, but it does not prove full history-index coverage or full local ops continuity. The workflow-support request remains useful future work because it would reduce friction and evidence risk around directive manifests, response writing, bounded polling, validation profiles, allow-file generation, source/authority refresh, claim-boundary scanning, and directive/response/journal indexing.

Local-ops does not outrank the missing target candidate for NA-0368 because no real target connection can happen without a target candidate and host identity boundary. However, local-ops/history-index and backup-plan coverage must be resolved before real operations that create durable evidence outside the current backup scope.

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

## Public-Ingress/Timing/Traffic-Shape Boundary

NA-0367 is not public-ingress evidence, not public-internet service evidence, not qsl-server production proof, not qsl-attachments production proof, and not qshield production proof. It does not change traffic behavior and does not support a claim that attachment size, timing metadata, traffic shape, or all metadata is hidden.

## External-Review Sensitivity

External review remains incomplete. Stronger public claims would require real target evidence, host identity evidence, credential boundary evidence, real key custody evidence, real key recovery evidence, off-host backup evidence, real restore drill evidence, service evidence, deployment evidence, monitoring/log evidence, rollback evidence, and external review evidence. NA-0367 supplies none of those as completed implementation proof.

## Public Claim Boundary

Allowed claim:

- NA-0367 defines prerequisite planning for target access and host identity before future real off-host encrypted backup target work.

Forbidden claims:

- local continuity backup is complete disaster recovery;
- off-host encrypted backup is complete;
- a real restore drill has been executed;
- real key custody or real key recovery is implemented;
- a target class is a configured host;
- a host-identity plan is verified host identity;
- service-local or demo evidence is public-internet evidence;
- external review is complete;
- attachment size, timing metadata, traffic shape, or all metadata is hidden;
- the system is metadata-free, anonymous, untraceable, release ready, or production ready.

## Prerequisite Decision Matrix

| Area | Status | Evidence | Blocker | Next action | Must precede real off-host backup | No-secret model only now | Ready for implementation authorization | NA-0368 candidate |
|---|---|---|---|---|---|---|---|---|
| Target class | Selected | NA-0355 | none at class level | Preserve SSH/SFTP-compatible class | yes | yes | no real implementation now | no |
| Target candidate | Absent | NA-0366 and NA-0367 read-only review | operator has not supplied target | Select target candidate and operator evidence boundary | yes | yes | prerequisite-plan authorization only | yes |
| Host identity | Absent | No host/fingerprint evidence | target candidate absent | Define operator-supplied or future capture/pinning path | yes | yes | prerequisite-plan authorization only | yes |
| Credential model | Absent | No credential boundary | no secret-handling authorization | Choose no-secret credential boundary plan | yes | yes | no | later |
| Credential storage | Absent | No storage boundary | no storage/agent/hardware-token model | Define storage/reference rules | yes | yes | no | later |
| Capacity/quota | Absent | No target capacity evidence | no target candidate | Define capacity evidence and stop conditions | yes | yes | no | later |
| Retention/purge | Absent | No real retention authority | no target/repository | Define retention/purge authority | yes | yes | no | later |
| Monitoring/alerting | Absent | No alert destination | no target/repository/owner | Define alert model and runbook | yes | yes | no | later |
| Operator runbook | Required | NA-0366 blocker review | no named owner/actions | Define operator action and stop conditions | yes | yes | no | later |
| Local-ops/backup-plan | Partial | Backup plan/status and workflow request | history-index/coverage gaps | Plan workflow support and coverage before real operations | before real operations | yes | no | later |
| External review/public claims | Not complete | Governance evidence | no real target/service/review evidence | Keep claims bounded | before stronger claims | yes | no | no |

Decision categories:

- `TARGET_ACCESS_PREREQUISITE_PLAN_ACCEPTED`
- `TARGET_CANDIDATE_REQUIRED`
- `HOST_IDENTITY_PREREQUISITE_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `CAPACITY_RETENTION_PREREQUISITE_REQUIRED`
- `MONITORING_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. NA-0368 should define the target-candidate/operator prerequisite and host-identity evidence path without connecting to a target.
2. A later credential-boundary lane should define secret-handling, storage, agent/hardware-token, rotation, revocation, recovery, and no-log rules without exposing secret material.
3. A later backup-tool authorization lane should decide tool installation authority, version pinning, repository-init constraints, and restore-check constraints.
4. A later key custody/recovery authorization lane should define real custody, recovery, recovery-envelope, and rotation boundaries.
5. A later real target/repository lane may proceed only after target candidate, host identity, credential boundary, capacity/retention, monitoring/runbook, backup-plan/local-ops, and tool/key prerequisites are exact and authorized.
6. A later real restore lane must remain separate from setup and must include cleanup, monitoring, and no-public-overclaim evidence.

## Future Validation/Marker/Verification Plan

For the selected NA-0368 target-candidate/operator-prerequisite successor, future markers should include:

- `NA0368_TARGET_CANDIDATE_PREREQUISITE_OK`
- `NA0368_TARGET_CLASS_BOUNDARY_OK`
- `NA0368_OPERATOR_TARGET_SELECTION_REQUIRED_OK`
- `NA0368_NO_REMOTE_CONNECTION_OK`
- `NA0368_NO_SECRET_MATERIAL_OK`
- `NA0368_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0368_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0368_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0368_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0368_NO_METADATA_FREE_CLAIM_OK`
- `NA0368_NO_ANONYMITY_CLAIM_OK`
- `NA0368_NO_UNTRACEABLE_CLAIM_OK`

Verification bundle requirements should include:

- named unresolved target fields;
- no connection/no host-key-scan proof;
- no secret-material proof;
- no repository-init/tool-install proof;
- no backup/restore/deploy/rollback proof;
- no backup-plan mutation proof unless explicitly authorized;
- source/authority refresh for qsl-server and qsl-attachments if still relevant;
- public-claim boundary scan.

## Workflow-Support and History-Index Future Work Note

The local workflow-support request remains relevant. The following items would materially reduce friction and evidence risk but are not implemented in NA-0367:

- qstart/qresume fast-forward to expected `origin/main` before handoff;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives/requests/journals/ops history folders.

These should remain future local-ops work unless an exact future directive selects them.

## Selected Successor

`NA-0368 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Plan`

Rationale: target candidate absence blocks host identity, credential model, capacity/quota/retention, monitoring/runbook, tool/repository work, key custody/recovery work, real backup operations, and real restore work. Selecting the target-candidate/operator prerequisite first preserves fail-closed ordering and avoids secret handling or remote connection.

## Rejected Alternatives

- Credential-boundary successor: rejected for immediate NA-0368 because no target candidate exists to bind the credential model to a host/account/trust boundary.
- Restic / backup tool installation authorization: rejected because tool installation cannot safely precede target-candidate, host-identity, credential, capacity, and runbook boundaries.
- Real key custody / recovery implementation authorization: rejected because real custody/recovery is still needed but does not resolve missing target candidate or host identity.
- Restore drill isolated real restore authorization: rejected because real restore remains downstream of target/repository/key/tool setup and cleanup/monitoring authorization.
- Local ops workflow support and history index: rejected as primary NA-0368 because it is important but does not unblock the first target-access decision; it remains required before real operations.
- External review readiness gap audit: rejected because prerequisite evidence is still incomplete.
- Website / public claim boundary audit: rejected because no public claim expansion is authorized and the implementation evidence remains incomplete.
- Public technical position paper draft plan: rejected because the current target/key/restore/service evidence gaps would dominate the paper boundary.
- Direct target setup, host-key scan, credential handling, repository init, backup, restore, deploy, or rollback: rejected as forbidden in NA-0367.

## Backup-Plan Impact Statement

No NA-0367 backup-plan update is required because the changed qsl-protocol governance/testplan/journal paths remain under `/srv/qbuild/work`, which current local backup status covers. Future target access, host identity capture, real target connection, credential handling, repository creation, key custody/recovery, recovery-envelope work, monitoring setup, real restore target creation, local-ops/history-index changes, backup-plan changes, backup script/timer/fstab changes, or public-claim work remains separately authorized and backup-plan gated.

## Next Recommendation

Proceed with NA-0368 as the target-candidate/operator prerequisite and host-identity evidence-path lane. NA-0368 should still be no-remote-connection and no-secret unless an exact future directive authorizes otherwise.
