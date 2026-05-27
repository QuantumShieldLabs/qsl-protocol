Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0368 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0368 defines the no-connection operator prerequisite bundle for future
off-host backup target-candidate and host-identity work. The result is an exact
field plan and action-packet design that can be reviewed before any remote
connection, host-key scan, credential handling, repository initialization, tool
installation, backup, restore, deploy, rollback, or local backup mutation.

The target class remains the NA-0355 SSH/SFTP-compatible off-host host class
with a restic-style encrypted snapshot repository class, but no real target
candidate is configured. Host identity is not verified. Credential, capacity,
retention, monitoring, alerting, and operator-runbook data remain prerequisite
fields only.

Selected successor:

`NA-0369 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet`

## Live NA-0368 Scope

Live `NEXT_ACTIONS.md` records `NA-0368 -- Metadata Runtime Off-Host Backup
Target Candidate / Host Identity Operator Prerequisite Plan` as the sole READY
item. The live scope authorizes qsl-protocol prerequisite planning and requires:

- target-candidate operator fields and target-class boundary;
- operator target-selection evidence requirements;
- host identity evidence path without connecting to a target;
- no remote connection and no host-key scan;
- no credential, key, passphrase, private-key, recovery-envelope, or secret
  material handling;
- no repository init, tool installation, backup, restore, deploy, rollback,
  backup script/timer/fstab mutation, qsl-server mutation, qsl-attachments
  mutation, qshield runtime mutation, dependency change, workflow change,
  README/START_HERE change, website/public-doc change, or public-claim
  expansion.

The optional `inputs/metadata_runtime` planning artifact was not added because
the live NA-0368 queue entry does not explicitly authorize that path. This
evidence file and the matching testplan carry the no-secret field bundle.

## Inherited NA-0367 Target-Access Prerequisite Result

NA-0367 selected NA-0368 after classifying:

- `TARGET_CANDIDATE_ABSENT`
- `TARGET_CANDIDATE_OPERATOR_REQUIRED`
- `TARGET_CANDIDATE_READY_FOR_NO_CONNECTION_PLAN`
- `TARGET_CANDIDATE_NOT_READY_FOR_CONNECTION`
- `HOST_IDENTITY_ABSENT`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `HOST_IDENTITY_CAPTURE_REQUIRES_FUTURE_AUTHORIZATION`
- `HOST_IDENTITY_PINNING_REQUIRED`
- `KNOWN_HOSTS_MUTATION_FORBIDDEN_NOW`
- `CREDENTIAL_BOUNDARY_ABSENT`
- `SECRET_HANDLING_FORBIDDEN_NOW`
- `CREDENTIAL_MODEL_SELECTION_REQUIRED`
- `CREDENTIAL_STORAGE_BOUNDARY_REQUIRED`
- `NO_SECRET_CREDENTIAL_PLACEHOLDER_ONLY`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

NA-0367 remains prerequisite planning only. It did not contact a target, scan a
host key, mutate `known_hosts`, handle credentials or secrets, initialize a
repository, install tools, or run any backup/restore/deploy/rollback operation.

## Inherited NA-0366 Blocker-Resolution Result

NA-0366 records `REAL_TARGET_TOOL_IMPLEMENTATION_BLOCKED`. It found no real
SSH/SFTP target, no host identity, no credential boundary, no capacity or
retention evidence, no monitoring boundary, no installed restic/borg/rclone/age
tool, no repository-init boundary, no real key custody, no real key recovery, no
recovery-envelope content authorization, and no real restore execution.

## Inherited NA-0365 No-Secret Isolated Restore Harness

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness. It
validates simulated isolated-restore metadata, simulated manifest/checksum
relationships, simulated cleanup and monitoring/runbook metadata, and
fail-closed negative cases only. It is not a real restore drill, does not create
or mount a real restore target, and does not prove complete disaster recovery.

## Inherited NA-0363 No-Secret Off-Host Target/Tool Harness

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and harness.
It validates simulated SSH/SFTP target metadata, simulated target identity
metadata, simulated restic-style repository metadata, simulated snapshot/check
/prune/restore relationships, simulated retention/purge metadata, and simulated
monitoring/alert metadata only. It is not a real target, not a real repository,
not tool installation, not a remote connection, and not real backup or restore
execution.

## Inherited NA-0361 No-Secret Key Custody/Recovery Harness

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and
harness. It validates simulated custody/recovery metadata only. It is not real
key custody, not real key recovery, does not create key material, does not
collect passphrases, does not inspect private keys, and does not create
recovery-envelope content.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

| Repository | Source | Authority | CI | Classification |
|---|---|---|---|---|
| qsl-server | Local `/srv/qbuild/work/NA-0237D/qsl-server` at `d40e6003fdf`; remote `HEAD` at `d40e6003fdf`; PR #56 merged at `d40e6003fdf` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local `/srv/qbuild/work/NA-0237D/qsl-attachments` at `96b9352bd63`; remote `HEAD` at `96b9352bd63`; PR #37 merged at `96b9352bd63` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Neither repository was mutated by NA-0368.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only evidence:

- `/backup/qsl` is mounted at `/dev/sda1` as ext4, about 916G total, 22G used,
  and 886G available.
- `/srv/qbuild` has about 468G total, 63G used, and 381G available.
- Local snapshots, manifests, and logs exist through
  `daily-20260527T023818-0500`.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint and daily continuity
  snapshots.
- `qsl-backup-daily.timer` is active and waiting for the next scheduled run.
- `restic`, `borg`, `rclone`, and `age` are absent.
- `gpg`, `ssh`, and `rsync` are present.
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`,
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`, and
  `/home/victor/work/qsl/codex/ops/backup/qsl-backup.candidate` are present.
- `/home/victor/work/qsl/codex/responses` and
  `/home/victor/work/qsl/codex/requests` are present.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` are absent.
- The backup status covers qbuild work, tmp, mirrors, evidence, logs, archive,
  Codex logs, Codex responses, and the backup plan. It does not prove full
  backup coverage for directives, journals, requests, or the whole ops tree.
- The D132 preservation bundle remains present under the expected qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_PROVEN`
- `TARGET_CANDIDATE_ABSENT`
- `TARGET_CANDIDATE_OPERATOR_REQUIRED`
- `TARGET_CANDIDATE_TEMPLATE_READY`
- `HOST_IDENTITY_ABSENT`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `CREDENTIAL_BOUNDARY_ABSENT`
- `CREDENTIAL_SECRET_HANDLING_FORBIDDEN`
- `CAPACITY_RETENTION_ABSENT`
- `MONITORING_ALERTING_ABSENT`
- `REAL_TARGET_ACCESS_NOT_AUTHORIZED`
- `OFF_HOST_BACKUP_NOT_READY`

## Target Candidate Prerequisite Field Plan

Operator-supplied non-secret fields required before any future target connection
can be considered:

| Field | Required content | Boundary |
|---|---|---|
| `target_label` | Human label or alias for the candidate | Non-secret label only |
| `target_class` | `SSH/SFTP-compatible off-host host` | Class must remain the NA-0355 class unless future governance changes it |
| `target_owner` | Provider, host owner, or administrative owner | No account secrets |
| `operator_contact` | Responsible operator or team contact | No credentials |
| `emergency_stop_contact` | Contact authorized to stop future access work | No credentials |
| `trust_boundary` | Who administers host, storage, account, and network controls | Must identify control model, not grant access |
| `jurisdiction_location_class` | Region/location/data-residency class if safe to record | May be coarse if exact location is sensitive |
| `network_reachability_class` | Operator statement of intended reachability, not a test result | No ping, SSH, SFTP, rsync, or scan now |
| `host_name_or_address_placeholder` | Candidate host identifier or placeholder for future action packet | May be withheld if sensitive; if withheld, connection remains blocked |
| `access_principal_label` | Account/principal label or placeholder | No private key, password, token, passphrase, or agent socket |
| `repository_path_class` | Intended repository path class or non-sensitive placeholder | No sensitive path token or secret-bearing path |
| `retention_intent` | Snapshot count/window intent | Not a configured retention policy |
| `capacity_quota_estimate` | Minimum target capacity, expected growth, cost ceiling | Not target-proven capacity |
| `availability_expectation` | Expected target availability and maintenance windows | Operator expectation only |
| `failure_mode_expectation` | Expected fail-closed behavior when target is unavailable | No script mutation |
| `monitoring_expectation` | Expected alert/runbook class | No monitoring setup |
| `credential_absence_statement` | Explicit statement that no credential material is included | Required for acceptance |
| `no_connection_statement` | Explicit statement that NA-0368 contacts no target | Required for acceptance |

Forbidden operator fields in this lane:

- private key;
- passphrase;
- token;
- password;
- raw credential;
- secret path with sensitive identifier;
- provider console session or API credential;
- ssh-agent socket value;
- hardware token PIN or recovery code;
- recovery-envelope content;
- command output exposing secrets;
- backup repository password or encryption key.

Classifications:

- `TARGET_CANDIDATE_FIELD_PLAN_OK`
- `TARGET_CANDIDATE_OPERATOR_ACTION_REQUIRED`
- `TARGET_CANDIDATE_NO_CONNECTION_OK`
- `TARGET_CANDIDATE_NO_SECRET_OK`
- `TARGET_CANDIDATE_NOT_READY_FOR_BACKUP`

## Host Identity / Host-Key Pinning Prerequisite Field Plan

Operator-supplied host identity fields:

| Field | Required content | Boundary |
|---|---|---|
| `host_identity_evidence_type` | Operator-supplied fingerprint, provider-console fingerprint, signed operator statement, or future explicitly authorized capture | NA-0368 captures none now |
| `host_key_algorithm` | Expected host-key algorithm, such as an approved SSH host-key algorithm class | No private host key |
| `fingerprint_format` | SHA256/base64 or other exact fingerprint format | Public fingerprint only |
| `fingerprint_value_placeholder` | Placeholder or operator-supplied public host-key fingerprint | If absent, future connection remains blocked |
| `fingerprint_source` | Provider console, operator statement, documented host owner source, or future authorized scan | Source must be documented |
| `fingerprint_timestamp` | Timestamp/source freshness | Operator evidence only |
| `verifier_identity` | Person/team that verified the fingerprint source | No credential material |
| `pinning_storage_plan` | Future path/model for recording a pinned public host key | No `known_hosts` mutation now |
| `mismatch_response` | Required fail-closed response if captured key differs from expected fingerprint | Must stop before connection |
| `rotation_response` | Required operator approval and dual-control record for host-key rotation | No automatic trust-on-first-use |
| `revocation_response` | Required stop/revoke path if host identity is revoked | Fail closed |

Future-capture fields that require a later exact directive:

- named target host and access context;
- exact host-key capture method and bounded command authorization;
- output redaction and storage path;
- pinning update path;
- mismatch, rotation, and revocation handling;
- proof that `known_hosts` mutation is authorized and scoped.

NA-0368 does not run `ssh-keyscan`, `ssh`, `scp`, `sftp`, or `rsync` against a
remote host. NA-0368 does not mutate `known_hosts` and does not claim verified
host identity.

Classifications:

- `HOST_IDENTITY_FIELD_PLAN_OK`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `HOST_IDENTITY_FUTURE_CAPTURE_AUTHORIZATION_REQUIRED`
- `HOST_IDENTITY_NO_SCAN_OK`
- `HOST_IDENTITY_NO_KNOWN_HOSTS_MUTATION_OK`
- `HOST_IDENTITY_NOT_VERIFIED_YET`

## Credential Boundary / No-Secret Placeholder Plan

Allowed placeholder fields:

- `credential_model_class`: operator-held key, agent-mediated key, hardware
  token, service account, or future selected class;
- `operator_held_key_class`: class only, no key material;
- `ssh_agent_future_model`: future agent-mediated access model and failure
  behavior, no socket value;
- `hardware_token_future_model`: class and policy only, no PIN/recovery code;
- `service_account_future_model`: account class only, no token/password;
- `credential_storage_boundary`: where references may live and what cannot be
  stored;
- `credential_rotation_expectation`: rotation cadence and approval model;
- `credential_revocation_expectation`: revocation owner and fail-closed action;
- `no_secret_proof_requirement`: future evidence must prove no secret material
  is present in governance/test output;
- `prohibited_evidence_examples`: private key, passphrase, token, password,
  provider credential, raw key, secret-bearing path, recovery-envelope contents,
  or command output exposing secrets.

NA-0368 does not select a real credential, inspect private keys, generate keys,
upload keys, collect passphrases, or handle secret material.

Classifications:

- `CREDENTIAL_BOUNDARY_FIELD_PLAN_OK`
- `CREDENTIAL_MODEL_SELECTION_REQUIRED`
- `CREDENTIAL_STORAGE_BOUNDARY_REQUIRED`
- `SECRET_HANDLING_FORBIDDEN_NOW`
- `NO_SECRET_CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_NOT_READY_FOR_CONNECTION`

## Capacity / Quota / Retention Prerequisite Field Plan

Required fields:

| Field | Required content | Boundary |
|---|---|---|
| `expected_backup_source_size` | Current source-size estimate and evidence source | Estimate only |
| `expected_snapshot_growth` | Expected daily/weekly growth | Estimate only |
| `retention_count_or_window` | Snapshot count or retention window | Not configured |
| `target_capacity_minimum` | Minimum usable target capacity before repository init | Operator evidence required |
| `minimum_free_space_threshold` | Stop threshold before backup/check/prune | Future policy only |
| `max_monthly_transfer_estimate` | Transfer estimate or upper bound | No target contacted |
| `max_monthly_storage_cost_boundary` | Cost ceiling or review trigger | Operator statement |
| `prune_retention_model` | Intended prune model and authorization owner | No prune now |
| `failed_backup_cleanup_requirement` | Cleanup action after failed future backup | Future runbook only |
| `failed_restore_cleanup_requirement` | Cleanup action after failed future restore | Future runbook only |
| `capacity_alert_threshold` | Alert threshold and owner | No monitoring setup now |
| `capacity_evidence_source` | Provider quota page, contract, operator statement, or future authorized command | No target assertion without evidence |

Classifications:

- `CAPACITY_FIELD_PLAN_OK`
- `QUOTA_FIELD_PLAN_OK`
- `RETENTION_FIELD_PLAN_OK`
- `CAPACITY_EVIDENCE_REQUIRED`
- `CAPACITY_NOT_READY_FOR_BACKUP`

## Monitoring / Alerting / Operator Runbook Prerequisite Field Plan

Required fields:

- backup success alert;
- backup failure alert;
- missed backup alert;
- target unreachable alert;
- capacity alert;
- retention/prune alert;
- restore-drill alert;
- credential/key error alert;
- notification destination class, with no webhook secret, token, address secret,
  or credential;
- operator acknowledgement procedure;
- emergency stop procedure;
- manual verification checklist;
- escalation boundary;
- evidence retention and redaction rule;
- statement that NA-0368 sets up no monitoring.

Classifications:

- `MONITORING_FIELD_PLAN_OK`
- `ALERTING_FIELD_PLAN_OK`
- `OPERATOR_RUNBOOK_FIELD_PLAN_OK`
- `MONITORING_MODEL_ONLY`
- `REAL_MONITORING_SETUP_FORBIDDEN_NOW`

## Operator Action Packet Design

The future operator action packet should be a no-secret packet with these
sections:

1. Target candidate identity: label, class, owner, contact, emergency stop,
   trust boundary, location class, reachability class, host identifier
   placeholder, access principal label, repository path class.
2. Host identity: evidence type, public fingerprint placeholder, algorithm,
   format, source, timestamp, verifier, pinning plan, mismatch/rotation
   /revocation responses.
3. Credential boundary: model class, storage boundary, rotation/revocation
   expectations, no-secret proof.
4. Capacity/quota/retention: source-size estimate, growth estimate, target
   capacity minimum, free-space threshold, transfer/cost ceilings, retention
   window, cleanup requirements, evidence source.
5. Monitoring/runbook: alert classes, notification destination class,
   acknowledgement, emergency stop, verification checklist, escalation,
   evidence retention.
6. Claim boundaries: not target setup, not verified host identity, not off-host
   backup completion, not disaster-recovery completion, not production or
   public-internet readiness.
7. Stop conditions: missing target candidate, missing host identity, missing
   credential boundary, missing capacity evidence, missing monitoring/runbook,
   any secret material, any required remote connection, any host-key mismatch,
   any need for backup script/timer/fstab mutation, or any public-claim
   expansion.

Action-packet labels:

- `REQUIRED_BEFORE_CONNECTION`
- `FUTURE_AUTHORIZATION_REQUIRED`
- `NO_SECRET_PLACEHOLDER_ONLY`
- `NO_REMOTE_CONNECTION`
- `NO_HOST_KEY_SCAN`
- `NO_BACKUP_OPERATION`

## Local-Ops / Backup-Plan / History-Index Dependency Analysis

The current backup posture supports same-host local continuity. It does not
prove complete disaster recovery and does not prove full coverage for
directives, journals, requests, or the whole ops tree. The workflow-support
request remains relevant because the NA-0368 startup recovery again proved that
fast-forward handoff, directive manifests, response-file writing, bounded
polling helpers, validation profiles, allow-files, source/authority helpers,
claim-boundary scanning, directive/response/journal indexing, and backup
coverage for local ops history would reduce operational friction.

Local-ops does not outrank the target-candidate action packet as NA-0369 because
no real off-host access can begin until the operator supplies non-secret target
candidate and host identity evidence. Local-ops must still precede real
operations that create durable evidence outside the current backup scope or
touch real targets, credentials, tools, repositories, keys, restores,
monitoring, backup source lists, scripts, timers, fstab, or public claims.

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

## Public-Ingress / Timing / Traffic-Shape Boundary

NA-0368 is not public-ingress evidence, not public-internet service evidence,
not qsl-server production proof, not qsl-attachments production proof, and not
qshield production proof. It does not change traffic behavior and does not
support a claim that attachment size, timing metadata, traffic shape, or all
metadata is hidden.

## External-Review Sensitivity

External review remains incomplete. Stronger claims require real target
evidence, verified host identity, credential boundary evidence, real key custody
evidence, real key recovery evidence, off-host backup evidence, real restore
drill evidence, service evidence, deployment evidence, monitoring/log evidence,
rollback evidence, and external review evidence. NA-0368 supplies none of
those as completed implementation proof.

## Public Claim Boundary

Allowed claim:

- NA-0368 defines the operator-provided target-candidate and host-identity
  prerequisite packet before any future off-host connection or host-key scan can
  be considered.

Forbidden claims:

- target-candidate planning is target setup;
- the operator field template is a configured target;
- host identity planning is verified host identity;
- local continuity backup is complete disaster recovery;
- off-host encrypted backup is complete;
- a real restore drill has been executed;
- real key custody or real key recovery is implemented;
- service-local or demo evidence is production or public-internet evidence;
- external review is complete;
- attachment size, timing metadata, traffic shape, or all metadata is hidden;
- the system is metadata-free, anonymous, untraceable, release ready, or
  production ready.

## Prerequisite Decision Matrix

| Area | Status | Required operator field | Evidence source | Blocker | Next action | Before remote connection | Before credential handling | Before real backup | No-secret model now | Ready for implementation authorization | NA-0369 fit |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Target candidate | Absent | target label, owner, contact, trust boundary, host placeholder, path class | operator packet | no candidate supplied | operator action packet | yes | yes | yes | yes | no | yes |
| Target class | Selected at class level | confirm SSH/SFTP-compatible class | NA-0355, NA-0367 | no real target | preserve class boundary | yes | yes | yes | yes | no | yes |
| Operator identity / owner | Absent | target owner, operator contact, emergency stop contact | operator packet | no owner/contact | collect non-secret owner fields | yes | yes | yes | yes | no | yes |
| Host identity | Absent | public fingerprint, algorithm, source, verifier, pinning plan | operator packet or future capture | no fingerprint/pin | collect operator fingerprint or authorize future capture | yes | yes | yes | yes | no | yes |
| Credential model | Absent | model class and storage boundary only | future credential boundary plan | no model selected | later credential-boundary lane | yes | yes | yes | yes | no | later |
| Credential storage | Absent | storage/reference rule only | future credential boundary plan | no storage boundary | later credential-boundary lane | yes | yes | yes | yes | no | later |
| Capacity/quota | Absent | source-size estimate, target minimum, quota/cost ceiling | operator packet | no capacity evidence | collect non-secret estimate/evidence source | yes | before real credential use for target | yes | yes | no | yes |
| Retention/purge | Absent | retention window/count, prune authority | operator packet | no repository/authority | collect intended policy | yes | no | yes | yes | no | yes |
| Monitoring/alerting | Absent | alert classes, destination class, acknowledgement | operator packet | no monitoring model | collect model/runbook fields | before real operations | no | yes | yes | no | yes |
| Operator runbook | Required | emergency stop, verification checklist, escalation | operator packet | no runbook owner/actions | collect runbook fields | yes | yes | yes | yes | no | yes |
| Local-ops/backup-plan | Partial | history/index and backup coverage plan | backup plan/status, workflow request | coverage gaps | later local-ops authorization before real operations | before real operations | before real secret handling if durable evidence changes | before real backup | yes | no | no |
| External review/public claims | Not complete | none for NA-0368 | governance evidence | implementation/review gaps | keep claims bounded | before stronger claims | before stronger claims | before stronger claims | yes | no | no |

Decision categories:

- `TARGET_CANDIDATE_OPERATOR_PACKET_ACCEPTED`
- `TARGET_CANDIDATE_OPERATOR_ACTION_REQUIRED`
- `HOST_IDENTITY_OPERATOR_PACKET_ACCEPTED`
- `HOST_IDENTITY_OPERATOR_ACTION_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `CAPACITY_RETENTION_PREREQUISITE_REQUIRED`
- `MONITORING_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. NA-0369 should produce the target-candidate / host-identity operator action
   packet, still no-secret and no-connection.
2. A later credential-boundary lane should define secret-handling, storage,
   agent/hardware-token, service-account, rotation, revocation, recovery, and
   no-log rules without exposing secret material.
3. A later backup-tool authorization lane should decide tool installation
   authority, version pinning, repository-init constraints, and restore-check
   constraints.
4. A later key custody/recovery authorization lane should define real custody,
   recovery, recovery-envelope, and rotation boundaries.
5. A later real target/repository lane may proceed only after target candidate,
   host identity, credential boundary, capacity/retention, monitoring/runbook,
   backup-plan/local-ops, and tool/key prerequisites are exact and authorized.
6. A later real restore lane must remain separate from setup and must include
   cleanup, monitoring, and no-public-overclaim evidence.

## Future Validation/Marker/Verification Plan

For the selected NA-0369 operator-action successor, future markers should
include:

- `NA0369_TARGET_CANDIDATE_OPERATOR_PACKET_OK`
- `NA0369_TARGET_CLASS_BOUNDARY_OK`
- `NA0369_OPERATOR_TARGET_SELECTION_REQUIRED_OK`
- `NA0369_HOST_IDENTITY_FIELD_PLAN_OK`
- `NA0369_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0369_CAPACITY_RETENTION_FIELD_PLAN_OK`
- `NA0369_MONITORING_RUNBOOK_FIELD_PLAN_OK`
- `NA0369_NO_REMOTE_CONNECTION_OK`
- `NA0369_NO_HOST_KEY_SCAN_OK`
- `NA0369_NO_SECRET_MATERIAL_OK`
- `NA0369_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0369_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0369_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0369_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0369_NO_METADATA_FREE_CLAIM_OK`
- `NA0369_NO_ANONYMITY_CLAIM_OK`
- `NA0369_NO_UNTRACEABLE_CLAIM_OK`

Verification bundle requirements should include:

- no connection/no host-key-scan proof;
- no secret-material proof;
- no repository-init/tool-install proof;
- no backup/restore/deploy/rollback proof;
- no backup-plan mutation proof unless explicitly authorized;
- source/authority refresh for qsl-server and qsl-attachments if still relevant;
- public-claim boundary scan.

## Workflow-Support and History-Index Future Work Note

The local workflow-support request remains relevant. The following items would
materially reduce friction and evidence risk but are not implemented in NA-0368:

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

These should remain future local-ops work unless an exact future directive
selects them.

## Selected Successor

`NA-0369 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet`

Rationale: target-candidate absence and host-identity absence are still the
first blockers that prevent any future remote connection. The safest next step
is a no-secret operator action packet that asks for the exact non-secret fields
defined here. Credential-boundary, tool-installation, real key custody/recovery,
local-ops, external review, website/public-claim, and public technical paper
work remain necessary later but do not outrank the missing operator target and
host-identity packet.

## Rejected Alternatives

- Credential-boundary / secret-handling prerequisite plan: rejected as the
  immediate NA-0369 successor because no target candidate or host identity
  exists to bind the credential model.
- Restic / backup tool installation authorization plan: rejected because tool
  installation cannot safely precede target-candidate, host-identity,
  credential, capacity, and runbook boundaries.
- Real key custody / key recovery implementation authorization plan: rejected
  because real custody/recovery remains downstream of no-secret target and
  credential prerequisites.
- QSL local ops workflow support and history index plan: rejected as primary
  NA-0369 because it is important but does not supply the missing target
  candidate; it remains required before real operations.
- External review readiness gap audit: rejected because prerequisite evidence is
  still incomplete.
- Website / public claim boundary audit: rejected because no public claim
  expansion is authorized and the public boundary is already preserved here.
- Public technical position paper evidence-bounded draft plan: rejected because
  the current target/key/restore/service evidence gaps would dominate the paper
  boundary.
- Direct target setup, host-key scan, credential handling, repository init,
  backup, restore, deploy, or rollback: rejected as forbidden in NA-0368.

## Backup-Plan Impact Statement

No NA-0368 backup-plan update is required because the changed qsl-protocol
governance/testplan/journal paths remain under `/srv/qbuild/work`, which the
current local continuity backup covers. No optional input artifact was added.

Future target connection, host identity capture, credential handling,
repository creation, backup tool installation, real key custody/recovery,
recovery-envelope work, monitoring setup, real restore target creation,
local-ops/history-index changes, backup-plan changes, backup script/timer/fstab
changes, or public-claim work remains separately authorized and backup-plan
gated.

## Next Recommendation

Proceed to NA-0369 as the no-secret target-candidate / host-identity operator
action packet. NA-0369 should still forbid remote connection, host-key scan,
known_hosts mutation, credential handling, secret handling, off-host setup,
repository init, tool installation, backup, restore, deploy, rollback, backup
script/timer/fstab mutation, and public-claim expansion unless a future exact
directive changes that scope.
