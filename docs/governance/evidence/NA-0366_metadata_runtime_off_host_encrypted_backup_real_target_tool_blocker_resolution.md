Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0366 Metadata Runtime Off-Host Encrypted Backup Real Target Tool Blocker Resolution

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0366 resolves the current real off-host encrypted backup target/tool state
after the no-secret target/tool, key/recovery, and isolated restore harnesses.
The result is:

- `REAL_TARGET_TOOL_IMPLEMENTATION_BLOCKED`
- `TARGET_ACCESS_PREREQUISITE_REQUIRED`
- `TOOL_INSTALL_PREREQUISITE_REQUIRED`
- `KEY_CUSTODY_PREREQUISITE_REQUIRED`
- `RESTORE_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `PUBLIC_REVIEW_PREREQUISITE_REQUIRED`

The exact selected successor is:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`

The real target/tool implementation is not authorized now. Current evidence
does not name a real SSH/SFTP host, does not record host identity or host-key
verification policy, does not define a credential boundary, does not prove
capacity/retention/monitoring boundaries, does not install the selected
restic-style tool, does not initialize a repository, does not create or handle
real key material, and does not prove a real isolated restore drill.

This lane changes only qsl-protocol governance/testplan/journal files. It does
not run backup, restore, deploy, rollback, real off-host setup, remote
connection, repository initialization, tool installation, real restore target
creation/mount/copy, real key generation, key upload, passphrase collection,
private-key inspection, recovery-envelope content creation, or secret handling.

## Live NA-0366 Scope

Live `NEXT_ACTIONS.md` marks NA-0366 READY and requires blocker resolution for
the real off-host encrypted backup target/tool prerequisites surfaced by
NA-0365. The live scope protects:

- no unsupported production/public-internet/external-review/anonymity claims;
- no metadata-free or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden;
- explicit qsl-server and qsl-attachments production boundaries;
- explicit authorization before real off-host setup, real key handling, real
  restore drills, backup/restore/deploy/rollback, or backup-plan updates.

Allowed NA-0366 output is governance/blocker-resolution evidence and exact
successor selection. NA-0366 does not implement the successor.

## Inherited NA-0365 No-Secret Isolated Restore Harness

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness:

- evidence:
  `docs/governance/evidence/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness.md`
- testplan:
  `tests/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness_testplan.md`
- qsl-protocol PR #992 merge: `df25562719de`

Boundary: NA-0365 models simulated isolated restore target metadata,
manifest/checksum relationships, old-archive compatibility, cleanup,
monitoring, and runbook metadata. It is not a real restore target, not mount or
copy proof, not real restore execution, and not complete disaster recovery.

## Inherited NA-0363 No-Secret Target/Tool Harness

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and
harness:

- evidence:
  `docs/governance/evidence/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness.md`
- testplan:
  `tests/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness_testplan.md`
- qsl-protocol PR #988 merge: `d9ddd61de122`

Boundary: NA-0363 models simulated SSH/SFTP target metadata, simulated target
identity metadata, a simulated restic-style repository, simulated snapshot/check
/prune/restore relationships, simulated retention/purge metadata, and simulated
monitoring/alert metadata. It is not a real off-host target, not a configured
repository, not tool installation, not a remote connection, not backup
execution, and not restore execution.

## Inherited NA-0361 No-Secret Key Custody/Recovery Harness

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and
harness:

- evidence:
  `docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md`
- testplan:
  `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
- qsl-protocol PR #984 merge: `c2b10dcbaf78`

Boundary: NA-0361 validates simulated custody records, simulated recovery
envelope metadata, simulated rotation, simulated emergency access, simulated old
archive compatibility, and no-secret proof. It is not real key custody, not real
key recovery, not passphrase handling, and not recovery-envelope content
creation.

## Inherited NA-0359 Restore Dry-Run Harness

NA-0359 added a qsl-protocol no-secret restore-drill dry-run fixture and
harness:

- evidence:
  `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md`
- testplan:
  `tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md`
- qsl-protocol PR #980 merge: `35128654290a`

Boundary: NA-0359 validates a restore-plan dry run and fail-closed negative
cases without a real restore target, backup payload, off-host operation, key
handling, or restore execution.

## Inherited NA-0355 Target/Tool Selection

NA-0355 selected only classes:

- target class: SSH/SFTP-compatible off-host host controlled by, or explicitly
  contracted for, QSL operations;
- tool class: restic-style encrypted snapshot repository with client-side
  encryption, manifest/check verification, prune, and isolated restore support.

Boundary: NA-0355 did not choose a live host, remote path, provider account,
credential, key, repository, install method, backup schedule, restore target, or
monitoring channel. It did not install tools or initialize a repository.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

| Repository | Local source | Remote/main proof | Authority | CI | Classification |
|---|---:|---:|---|---|---|
| qsl-server | `/srv/qbuild/work/NA-0237D/qsl-server` at `d40e6003fdf` | PR #56 merged at `d40e6003fdf`; remote main also `d40e6003fdf` | GitHub viewer permission ADMIN; branch protection requires `rust`; force pushes disabled; deletions disabled; admins enforced | latest main `ci` run success on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | `/srv/qbuild/work/NA-0237D/qsl-attachments` at `96b9352bd63` | PR #37 merged at `96b9352bd63`; remote main also `96b9352bd63` | GitHub viewer permission ADMIN; branch protection requires `rust`; force pushes disabled; deletions disabled | latest main `rust` run success on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only. It is not
  production/public-internet proof, off-host backup proof, or external-review
  completion.
- qsl-attachments PR #37 remains service-local prerequisite evidence only. It
  is not production/public-internet proof, not complete disaster recovery proof,
  and not proof that hot/live backup or partial restore is supported.
- No qsl-server or qsl-attachments files are changed by NA-0366.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted as ext4 and writable.
- `/backup/qsl` has about 916G size, about 21G used, and about 886G available.
- `/srv/qbuild` has about 468G size, about 58G used, and about 386G available.
- Local snapshots, manifests, and logs are present through
  `daily-20260526T023618-0500`.
- `qsl-backup preflight` reports the local target mounted and daily sources
  present.
- `qsl-backup list` lists checkpoint and daily local continuity snapshots.
- `qsl-backup-daily.timer` is enabled and waiting for the next scheduled run.
- Installed tool detection found `gpg`, `ssh`, and `rsync`.
- Installed tool detection did not find `restic`, `borg`, `rclone`, or `age`.
- The local backup plan states that the platter backup is same-host local
  continuity and should not be treated as the only disaster recovery copy.
- The backup status file states that the local continuity target is mounted,
  scheduled, and has a non-destructive restore drill, but remains same-host
  continuity only.
- Read-only history paths present: responses and requests.
- Read-only history paths absent: directives and journals.
- The D132 preservation bundle exists at the expected local qbuild temporary
  preservation path.

Classification:

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

No read-only evidence proves a real off-host encrypted repository, target
account, host identity, repository secret, real key custody, real key recovery,
real recovery envelope, real off-host restore drill, remote retention/purge
boundary, or remote monitoring/alerting.

## Real Off-Host Target-Access and Host-Identity Blocker Analysis

Selected target class: SSH/SFTP-compatible off-host host.

Current state:

- No real off-host host is named.
- No host key or host identity pin is recorded.
- No host-key verification policy is recorded.
- No remote path or repository path is authorized.
- No network reachability test is authorized in NA-0366.
- No credential boundary is authorized.
- No capacity/quota/retention boundary is recorded.
- No provider/operator trust boundary is recorded.
- No monitoring/alert channel is recorded for remote target failure or missed
  runs.

Classification:

- `TARGET_ACCESS_BLOCKED_NO_TARGET`
- `TARGET_ACCESS_BLOCKED_NO_HOST_IDENTITY`
- `TARGET_ACCESS_BLOCKED_NO_CREDENTIAL_BOUNDARY`
- `TARGET_ACCESS_BLOCKED_NO_CAPACITY_RETENTION`
- `TARGET_ACCESS_BLOCKED_NO_MONITORING`
- `TARGET_ACCESS_NOT_AUTHORIZED_NOW`

Not selected:

- `TARGET_ACCESS_READY_FOR_AUTHORIZATION`

Decision: target access and host identity are the first load-bearing blockers.
A future lane can plan host identity, target class boundary, credential
boundary, capacity/retention, and monitoring without connecting to a remote host
or handling secrets.

## Real Backup-Tool Availability / Installation / Repository-Init Blocker Analysis

Selected tool class: restic-style encrypted snapshot repository.

Current state:

- `restic` is absent.
- `borg`, `rclone`, and `age` are absent.
- `gpg`, `ssh`, and `rsync` are present but are not the selected backup engine.
- No package installation directive is active.
- No version pin or acceptable package source is recorded for restic.
- No install/test boundary is recorded.
- No repository init boundary is recorded.
- No repository check/prune/restore boundary is recorded.
- No secret-safe repository password/key plan exists.

Classification:

- `TOOL_INSTALL_BLOCKED_TOOL_ABSENT`
- `TOOL_INSTALL_BLOCKED_NO_INSTALL_AUTHORITY`
- `TOOL_INSTALL_BLOCKED_NO_VERSION_PIN`
- `TOOL_INSTALL_BLOCKED_NO_REPOSITORY_INIT_BOUNDARY`
- `TOOL_INSTALL_BLOCKED_NO_RESTORE_CHECK_BOUNDARY`
- `TOOL_INSTALL_NOT_AUTHORIZED_NOW`

Not selected:

- `TOOL_INSTALL_READY_FOR_AUTHORIZATION`

Decision: tool installation is blocked and must follow an exact local-ops
package authorization and version/repository boundary. It should not precede
target-access planning because repository init depends on the chosen host/path
and credential boundary.

## Real Key Custody / Key Recovery / Recovery-Envelope Blocker Analysis

Current state:

- NA-0361 proves only a no-secret harness.
- No real repository key/passphrase custody process exists.
- No real recovery process exists.
- No recovery-envelope content creation is authorized.
- No passphrase collection/handling path is authorized.
- No rotation plan is authorized.
- No old-archive compatibility policy for real encrypted repositories is
  recorded.
- No emergency access plan is authorized.
- No exposed/lost key response plan is recorded.

Classification:

- `KEY_CUSTODY_BLOCKED_NO_REAL_CUSTODY`
- `KEY_RECOVERY_BLOCKED_NO_REAL_RECOVERY`
- `RECOVERY_ENVELOPE_BLOCKED_NO_CONTENT_AUTHORIZATION`
- `KEY_ROTATION_BLOCKED_NO_RUNBOOK`
- `KEY_CUSTODY_NOT_AUTHORIZED_NOW`

Not selected:

- `KEY_CUSTODY_READY_FOR_AUTHORIZATION`

Decision: real key custody/recovery remains blocked. A future target-access plan
must not collect passphrases, inspect private keys, generate keys, upload keys,
or create recovery-envelope content.

## Real Restore-Drill / Isolated Target / Cleanup / Monitoring Blocker Analysis

Current state:

- NA-0365 proves only a no-secret isolated restore harness.
- No real isolated restore target is named.
- No mount/copy boundary is authorized.
- No backup payload retrieval boundary exists.
- No real repository exists.
- No real key exists.
- No failed-restore cleanup procedure is authorized.
- No monitoring/alerting procedure for real restore drill results exists.
- No real restore runbook is authorized.
- No retention/prune policy is authorized.

Classification:

- `RESTORE_BLOCKED_NO_REAL_TARGET`
- `RESTORE_BLOCKED_NO_KEY`
- `RESTORE_BLOCKED_NO_REPOSITORY`
- `RESTORE_BLOCKED_NO_CLEANUP`
- `RESTORE_BLOCKED_NO_MONITORING`
- `RESTORE_BLOCKED_NO_RUNBOOK`
- `RESTORE_NOT_AUTHORIZED_NOW`

Not selected:

- `RESTORE_READY_FOR_AUTHORIZATION`

Decision: real isolated restore remains blocked behind target access, tool
installation, repository initialization, real key custody/recovery, cleanup,
monitoring, and runbook prerequisites.

## Local-Ops / Backup-Plan / History-Index Blocker Analysis

Current state:

- The local continuity backup covers qbuild and at least the response history
  surface, but it remains same-host continuity only.
- The read-only history directories for responses and requests are present.
- The read-only history directories for directives and journals are absent.
- The workflow-support request is present and remains future local-ops work.
- The current NA-0366 evidence stays under qsl-protocol governance/testplan
  paths covered by the current qbuild worktree backup posture.
- Future durable evidence outside current backup scope, real target state,
  key material, recovery envelopes, monitoring artifacts, backup source-list
  changes, scripts, timers, fstab, services, backup, restore, deploy, rollback,
  and public-claim mutation remain backup-plan gated.

Classification:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_BLOCKED_NO_MANIFEST`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`

Decision: local-ops work would materially reduce friction but is not the first
NA-0367 successor because a target-access/host-identity prerequisite can be
planned without implementing workflow support. Local-ops should remain a
near-term supporting lane before any real operations that create durable local
state outside the current backup posture.

## Public-Ingress / Timing / Traffic-Shape Boundary

NA-0366 changes no public ingress, network path, timing behavior, traffic-shape
behavior, attachment padding behavior, relay behavior, or service runtime. The
current qshield embedded relay/demo evidence remains reference/oracle evidence
only. qsl-server and qsl-attachments production boundaries remain explicit.

## External-Review Sensitivity

External review remains incomplete. NA-0366 is a blocker-resolution lane, not an
implementation or review-completion lane. Stronger external-review-facing claims
would require real key custody evidence, real key recovery evidence, off-host
backup evidence, real restore-drill evidence, service evidence, deployment
evidence, monitoring/log evidence, rollback evidence, and review evidence.

## Public Claim Boundary

NA-0366 does not support any claim that:

- local continuity is complete disaster recovery;
- off-host encrypted backup is complete;
- a real restore drill has been executed;
- real key custody or real key recovery is implemented;
- production readiness has been achieved;
- public-internet readiness has been achieved;
- external review is complete;
- attachment size, timing metadata, traffic shape, or all metadata is hidden;
- QSL is not claimed to be metadata-free, anonymous, or untraceable.

No website, README, START_HERE, docs/public, external website, service,
workflow, dependency, branch-protection, or public-safety configuration is
changed by NA-0366.

## Blocker-Resolution Decision Matrix

| Area | Status | Evidence | Blocker | Next action | Must precede real off-host backup? | No-secret-only model still useful? | Ready for authorization? | NA-0367 candidate? |
|---|---|---|---|---|---|---|---|---|
| Target access / host identity | Blocked | no named host, no host identity, no remote path | no target/host identity/credential/capacity/monitoring boundary | plan target access and host identity | yes | yes | no | yes, selected |
| Tool installation / version pin | Blocked | `restic` absent; no install policy | no install authority/version pin | plan package/version/install boundary after target plan | yes | yes | no | later |
| Repository initialization | Blocked | no repository exists or path authorized | no target/key/tool boundary | define init/check/prune/restore boundary after target/tool/key plans | yes | yes | no | later |
| Credential / secret handling | Blocked | no credential boundary | NA-0366 forbids secrets | define secret-safe credential custody before use | yes | limited | no | included in selected plan boundary |
| Key custody | Blocked | NA-0361 no-secret only | no real custody path | future key custody authorization | yes | yes | no | later |
| Key recovery | Blocked | NA-0361 no-secret only | no real recovery path | future key recovery authorization | yes | yes | no | later |
| Recovery envelope | Blocked | no content authorization | no real envelope content path | future content authorization with secret-safe evidence policy | yes | yes | no | later |
| Isolated restore target | Blocked | NA-0365 no-secret only | no real target/mount/copy boundary | future real restore authorization after repository/key | yes | yes | no | later |
| Cleanup | Blocked | simulated cleanup only | no real cleanup runbook | define cleanup before real restore | yes | yes | no | later |
| Monitoring/alerting | Blocked | simulated monitoring only | no live channel or thresholds | include target monitoring boundary in successor | yes | yes | no | included in selected plan |
| Retention/purge | Blocked | simulated retention/purge only | no remote capacity/retention policy | include capacity/retention in successor | yes | yes | no | included in selected plan |
| Operator runbook | Blocked | simulated runbook only | no real runbook | staged runbook after target/tool/key | yes | yes | no | later |
| Backup-plan/local-ops | Partial | same-host continuity mounted and scheduled | off-host and history/index gaps | keep future operations backup-plan gated | yes for real ops | yes | partial | later |
| External review | Blocked | no external review completion evidence | implementation evidence incomplete | defer until real evidence exists | before stronger public claims | no | no | later |
| Public claims | Blocked for stronger claims | public boundary docs remain cautious | real evidence absent | no website/public claim change now | before stronger claims | no | no | later |

## Future Staged Implementation Strategy

Recommended staged order:

1. NA-0367 target access / host identity prerequisite plan:
   define the acceptable SSH/SFTP target class, real host selection criteria,
   host-key identity capture method, credential boundary, capacity/retention
   boundary, monitoring boundary, and no-remote-connection proof.
2. Tool installation authorization:
   authorize restic package source/version, install test boundary, rollback
   boundary, and repository init preconditions without initializing a repository.
3. Real key custody / recovery authorization:
   define repository key/passphrase custody, recovery, rotation, emergency
   access, old-archive compatibility, and lost/exposed key response without
   leaking secret material in evidence.
4. Repository initialization authorization:
   initialize only after target, tool, credential, and key boundaries are
   explicit and backup-plan impact is reviewed.
5. Real isolated restore authorization:
   create/mount/copy only in an explicitly isolated target after repository and
   key custody are proven.
6. Monitoring/retention/runbook hardening:
   prove missed-run, failed-check, quota, prune, restore, and alert handling.
7. Public-claim and external-review lanes:
   update public-facing claims only after exact implementation, restore,
   monitoring, rollback, and review evidence exists.

## Future Validation / Marker / Verification Plan

Because NA-0367 is selected as target-access / host-identity planning, its
future evidence should emit or record:

- `NA0367_TARGET_ACCESS_PREREQUISITE_OK`
- `NA0367_TARGET_CLASS_BOUNDARY_OK`
- `NA0367_HOST_IDENTITY_BOUNDARY_OK`
- `NA0367_CREDENTIAL_BOUNDARY_OK`
- `NA0367_CAPACITY_RETENTION_BOUNDARY_OK`
- `NA0367_MONITORING_BOUNDARY_OK`
- `NA0367_NO_REMOTE_CONNECTION_OK`
- `NA0367_NO_SECRET_MATERIAL_OK`
- `NA0367_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0367_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0367_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0367_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0367_NO_METADATA_FREE_CLAIM_OK`
- `NA0367_NO_ANONYMITY_CLAIM_OK`
- `NA0367_NO_UNTRACEABLE_CLAIM_OK`

## Workflow-Support and History-Index Future Work Note

The local workflow-support request would materially reduce repeated friction:

- qstart/qresume fast-forward to expected origin/main before handoff;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives/requests/journals/ops history folders.

NA-0366 does not implement these items. They remain useful local-ops work, but
target-access/host-identity planning is the selected NA-0367 successor.

## Selected Successor

Selected:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`

Rationale:

- A real off-host backup cannot be safely authorized without a named target
  class boundary, host identity proof, credential boundary, capacity/retention
  boundary, and monitoring boundary.
- This lane can remain no-secret and no-remote-connection while resolving the
  first real target blocker.
- Tool installation, repository init, key custody, and restore authorization
  depend on the target-access boundary.

## Rejected Alternatives

- Restic / backup tool installation authorization now: rejected because there is
  no target host/path, host identity, credential boundary, or repository init
  boundary yet.
- Real key custody / key recovery implementation authorization now: rejected
  because target and repository boundaries are absent, and real secret handling
  remains prohibited.
- Restore drill isolated real restore authorization now: rejected because no
  real repository, key, target, cleanup, monitoring, or runbook exists.
- QSL local ops workflow support as NA-0367: useful, but not the primary
  off-host target/tool blocker; keep it near-term.
- External review readiness gap audit: premature until real backup/key/restore
  evidence exists.
- Website / public claim boundary audit: no public claim change is authorized
  now; stronger claims remain blocked.
- Public technical position paper draft plan: premature until real operational
  evidence is stronger.

## Backup-Plan Impact Statement

No NA-0366 backup-plan update is required because this lane changes only
qsl-protocol governance/testplan/journal paths under `/srv/qbuild/work` and does
not create durable evidence outside the current local backup posture.

Future real target/tool implementation, repository init, key custody/recovery,
recovery-envelope content, real isolated restore targets, durable restored
artifacts, monitoring artifacts, source-list changes, scripts, timers, fstab,
services, backup, restore, deploy, rollback, and public-claim mutation remain
backup-plan and local-ops gated.

## Next Recommendation

Close NA-0366 after PR merge and green required checks, then restore exactly one
READY successor:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`
