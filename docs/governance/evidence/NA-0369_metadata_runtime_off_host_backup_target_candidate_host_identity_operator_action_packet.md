Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0369 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0369 converts the NA-0368 target-candidate and host-identity prerequisite
field plan into a no-secret operator action packet template:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json`

The packet is a placeholder-only template. It asks the future operator for
non-secret target candidate, host identity, credential-placeholder,
capacity/quota/retention, monitoring/alerting, and runbook evidence. It also
tells the operator what must never be provided.

NA-0369 does not set up a target, does not connect to a remote host, does not
scan or pin host keys, does not mutate `known_hosts`, does not handle
credentials or secrets, does not initialize a repository, does not install
tools, does not run backup/restore/deploy/rollback operations, and does not
change qsl-server, qsl-attachments, qshield runtime, backup scripts, timers,
fstab, service configuration, dependencies, workflows, public docs, README, or
START_HERE.

Selected successor:

`NA-0370 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake`

Rationale: the packet is now available, but no operator response has supplied a
real non-secret target candidate or host identity evidence. Operator response
intake is the narrowest next lane before credential-boundary, tool-install,
real key custody/recovery, local-ops, review, website/public-claim, or paper
work.

## Live NA-0369 Scope

Live `NEXT_ACTIONS.md` records `NA-0369 -- Metadata Runtime Off-Host Backup
Target Candidate / Host Identity Operator Action Packet` as the sole READY
item. The live scope authorizes qsl-protocol operator-packet/governance work
only and requires:

- convert the NA-0368 prerequisite field plan into a no-secret operator action
  packet;
- preserve no-remote-connection, no-host-key-scan, no-known_hosts-mutation,
  and no-secret boundaries;
- include credential placeholders, capacity/quota/retention fields,
  monitoring/alerting fields, runbook fields, claim boundaries, and stop
  conditions;
- keep all future operator-provided values marked as
  `REQUIRED_OPERATOR_INPUT`, `REQUIRED_BEFORE_CONNECTION`, or
  `FUTURE_AUTHORIZATION_REQUIRED`;
- preserve the public-claim boundary around production readiness,
  public-internet readiness, external-review completion, anonymity,
  metadata-free behavior, untraceability, hidden attachment size, hidden timing,
  hidden traffic shape, complete off-host backup, complete disaster recovery,
  target configuration, host identity verification, real restore completion,
  real key custody, and real key recovery.

The live scope explicitly forbids target setup, remote connection, host-key
scan, `known_hosts` mutation, credential handling, secret handling, repository
init, tool installation, real key handling, recovery-envelope content creation,
backup, restore, deploy, rollback, backup script/timer/fstab mutation, runtime
implementation change, dependency change, workflow change, website/public-doc
change, README/START_HERE change, and public-claim expansion.

## Inherited NA-0368 Operator Prerequisite Plan

NA-0368 defined the operator-provided prerequisite bundle for target candidate
and host identity evidence. It established:

- the target class remains the NA-0355 SSH/SFTP-compatible off-host host class
  with a restic-style encrypted snapshot repository class at class level only;
- no real target candidate is configured;
- host identity is not verified;
- credential, capacity, retention, monitoring, alerting, and operator-runbook
  data remain prerequisite fields only;
- operator fields must be non-secret or explicitly withheld until a future
  authorization lane.

NA-0368 did not create the optional JSON artifact because its live scope did
not explicitly authorize `inputs/`. NA-0369 live scope does authorize this
template artifact.

## Inherited NA-0367 Target-Access Prerequisite Result

NA-0367 selected the target-access / host-identity prerequisite lane after
recording that the target class was selected but the target candidate, host
identity, credential boundary, capacity/quota/retention evidence,
monitoring/alerting, and runbook were absent. It also recorded that future host
identity capture requires explicit authorization and that `known_hosts`
mutation is forbidden until then.

## Inherited NA-0366 Blocker-Resolution Result

NA-0366 recorded `REAL_TARGET_TOOL_IMPLEMENTATION_BLOCKED`. It found no real
SSH/SFTP target, no host identity, no credential boundary, no capacity or
retention evidence, no monitoring boundary, no installed restic/borg/rclone/age
tool, no repository-init boundary, no real key custody, no real key recovery,
no recovery-envelope authorization, and no real restore execution.

## Inherited NA-0365 No-Secret Isolated Restore Harness

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness. It
validates simulated isolated-restore metadata, simulated manifest/checksum
relationships, simulated cleanup, monitoring/runbook metadata, and fail-closed
negative cases. It is not a real restore drill, does not create or mount a
real restore target, and does not prove complete disaster recovery.

## Inherited NA-0363 No-Secret Off-Host Target/Tool Harness

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and
harness. It validates simulated SSH/SFTP target metadata, simulated target
identity metadata, simulated restic-style repository metadata,
snapshot/check/prune/restore relationships, retention/purge metadata, and
monitoring/alert metadata. It is not a real target, not a configured
repository, not tool installation, not a remote connection, and not real backup
or restore execution.

## Inherited NA-0361 No-Secret Key Custody/Recovery Harness

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and
harness. It validates simulated custody/recovery metadata only. It is not real
key custody, not real key recovery, does not create key material, does not
collect passphrases, does not inspect private keys, and does not create
recovery-envelope content.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

| Repository | Source | Authority | CI | Classification |
|---|---|---|---|---|
| qsl-server | Local `/srv/qbuild/work/NA-0237D/qsl-server` at `d40e6003fdf`; remote `HEAD` and `main` at `d40e6003fdf`; PR #56 merged at `d40e6003fdf` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local `/srv/qbuild/work/NA-0237D/qsl-attachments` at `96b9352bd63`; remote `HEAD` and `main` at `96b9352bd63`; PR #37 merged at `96b9352bd63` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundary:

- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Neither repository was mutated by NA-0369.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted on the local host as same-host continuity storage,
  about 916G total with about 886G available.
- `/srv/qbuild` has about 468G total with about 378G available.
- Local daily snapshots, manifests, and logs exist through
  `daily-20260527T023818-0500`.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint and daily local
  continuity snapshots.
- `qsl-backup-daily.timer` is enabled and active/waiting.
- `restic`, `borg`, `rclone`, and `age` are absent.
- `gpg`, `ssh`, and `rsync` are present.
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`,
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`, and
  `/home/victor/work/qsl/codex/requests` are present.
- `/home/victor/work/qsl/codex/responses` is present and includes the prior
  NA-0368 response.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` are absent at inspection.
- D132 preservation bundle remains present under the expected local qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_PROVEN`
- `TARGET_CANDIDATE_PACKET_READY`
- `TARGET_CANDIDATE_REAL_VALUE_ABSENT`
- `HOST_IDENTITY_REAL_VALUE_ABSENT`
- `CREDENTIAL_BOUNDARY_ABSENT`
- `CREDENTIAL_SECRET_HANDLING_FORBIDDEN`
- `CAPACITY_RETENTION_REAL_VALUE_ABSENT`
- `MONITORING_ALERTING_REAL_VALUE_ABSENT`
- `REAL_TARGET_ACCESS_NOT_AUTHORIZED`
- `OFF_HOST_BACKUP_NOT_READY`

## Operator Packet Field Design

The packet sections are:

1. Packet metadata: schema version, artifact class, generator, no-secret
   statement, no-connection statement, no-host-key-scan statement, and
   no-backup/restore statement.
2. Target candidate: label/alias, target class, owner/contact label,
   jurisdiction/location class, trust boundary, reachability class, intended
   repository path class, capacity estimate, retention intent, availability,
   failure mode, cost/quota concern, and emergency stop contact.
3. Host identity: evidence source type, fingerprint algorithm, fingerprint
   format, fingerprint placeholder, source timestamp, verifier label, future
   pinning storage plan, mismatch response, rotation response, and revocation
   response.
4. Credential placeholder: credential model class, storage boundary, rotation
   expectation, revocation expectation, and no-secret proof requirement.
5. Capacity / retention: expected source size, expected growth, minimum
   capacity, free-space threshold, transfer estimate, cost boundary, retention
   window/count, and prune model.
6. Monitoring / runbook: success/failure/missed/target-unreachable/capacity
   /retention/restore-drill/credential-key alerts, notification destination
   class, acknowledgement, emergency stop, manual verification, and escalation.
7. Forbidden inputs.
8. Stop conditions.
9. Public claim boundaries.
10. Future validation markers.

Classifications:

- `OPERATOR_PACKET_FIELD_DESIGN_OK`
- `OPERATOR_PACKET_NO_SECRET_DESIGN_OK`
- `OPERATOR_PACKET_NO_CONNECTION_DESIGN_OK`
- `OPERATOR_PACKET_NO_HOST_KEY_SCAN_DESIGN_OK`

## No-Secret Operator Action Packet Implementation

Implemented artifact:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json`

The artifact is deterministic JSON and contains:

- only template fields and placeholder markers;
- no real target endpoint;
- no real host identity value;
- no real credential;
- no real host-key fingerprint;
- no command to execute;
- no connection, backup, restore, deploy, rollback, repository-init, or
  tool-install instruction;
- `REQUIRED_OPERATOR_INPUT`, `REQUIRED_BEFORE_CONNECTION`,
  `FUTURE_AUTHORIZATION_REQUIRED`, `DO_NOT_INCLUDE_SECRET`,
  `NO_CONNECTION_IN_THIS_PACKET`, and `NO_HOST_KEY_SCAN_IN_THIS_PACKET`
  placeholders.

## Operator Instructions / Forbidden-Input Guide

Operator should provide only:

- non-secret target label or alias;
- target class confirmation;
- non-secret ownership/contact label;
- high-level location or jurisdiction class if safe to record;
- host identity fingerprint evidence only by a future process explicitly
  authorized for collection and storage;
- capacity and retention estimates;
- monitoring destination class, not a credential;
- emergency contact label;
- runbook owner label.

Operator must not provide:

- private keys;
- passphrases;
- passwords;
- tokens;
- raw credentials;
- recovery-envelope contents;
- secret paths;
- private material paths;
- screenshots or command output containing secrets;
- live connection outputs;
- unredacted `known_hosts` content if sensitive;
- host fingerprints unless a future directive explicitly authorizes how to
  collect and store them.

## Packet Validation / JSON Parse / No-Secret Checks

Required checks:

- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null`
- confirm target candidate fields exist;
- confirm host identity fields exist;
- confirm credential placeholder fields exist;
- confirm capacity/retention fields exist;
- confirm monitoring/runbook fields exist;
- confirm forbidden-input guidance exists;
- confirm placeholder markers exist;
- confirm no real target endpoint, no real credential, no real host key, no
  executable command, no connection command, and no backup/restore command;
- explain any sensitive-word matches as prohibited-input guidance only.

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Current backup posture remains same-host local continuity. It does not prove
complete disaster recovery and does not prove full local ops history coverage.
The current daily source list covers qbuild work, qbuild tmp, mirrors,
evidence/logs/archive, Codex logs, Codex responses, and the backup plan, but
does not prove durable coverage for directives, journals, requests, or the
whole ops tree.

The workflow-support request remains relevant. Fast-forwarding qstart/qresume
worktrees, a response-file writer, bounded PR/public-safety polling helpers,
machine-readable directive manifests, validation profiles, per-directive
allow-files, read-only source/authority helpers, claim-boundary scanners,
directive/response/journal indexes, and backup coverage for local ops history
would reduce friction.

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

Local-ops does not outrank operator response intake for NA-0370 because the
packet now exists and the next direct blocker is the missing operator response.
Local-ops should precede real operations that touch durable evidence outside
current backup scope, real targets, credentials, tools, repositories, keys,
restores, monitoring, backup source lists, scripts, timers, fstab, or public
claims.

## Public-Ingress / Timing / Traffic-Shape Boundary

NA-0369 is not public-ingress evidence, not public-internet service evidence,
not qsl-server production proof, not qsl-attachments production proof, and not
qshield production proof. It changes no traffic behavior and does not support a
claim that attachment size, timing metadata, traffic shape, or all metadata is
hidden.

## External-Review Sensitivity

External review remains incomplete. Stronger claims require real target
evidence, verified host identity, credential boundary evidence, real key
custody evidence, real key recovery evidence, off-host backup evidence, real
restore drill evidence, service evidence, deployment evidence, monitoring/log
evidence, rollback evidence, and external review evidence. NA-0369 supplies
none of those as completed implementation proof.

## Public Claim Boundary

Allowed claim:

- NA-0369 creates a no-secret operator action packet template for future
  target-candidate and host-identity evidence.

Forbidden claims:

- the operator packet is target setup;
- the packet template is a configured target;
- host identity placeholder is verified host identity;
- local continuity backup is complete disaster recovery;
- off-host encrypted backup is complete;
- a real restore drill has been executed;
- real key custody or real key recovery is implemented;
- service-local or demo evidence is production or public-internet evidence;
- external review is complete;
- attachment size, timing metadata, traffic shape, or all metadata is hidden;
- the system is metadata-free, anonymous, untraceable, release ready, or
  production ready.

## Decision Matrix

| Area | Status | Required operator field | Placeholder included | Future operator response needed | Evidence source | Blocker | Next action | Before remote connection | Before credential handling | Before real backup | No-secret model now | Ready for implementation authorization | NA-0370 fit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Target candidate packet | Created | n/a | yes | yes | NA-0369 JSON | no operator response | intake response | yes | yes | yes | yes | no | yes |
| Target class | Class selected | class confirmation | yes | yes | NA-0355/NA-0368 | no real target | intake response | yes | yes | yes | yes | no | yes |
| Operator identity / owner | Absent | owner/contact/emergency labels | yes | yes | NA-0369 JSON | no owner/contact | intake response | yes | yes | yes | yes | no | yes |
| Host identity | Absent | evidence type, algorithm, format, fingerprint placeholder, source, verifier | yes | yes | NA-0369 JSON | no fingerprint/pin | intake or future capture authorization | yes | yes | yes | yes | no | yes |
| Credential placeholder | Placeholder only | model/storage/rotation/revocation class | yes | yes | NA-0369 JSON | no credential boundary | later credential plan | yes | yes | yes | yes | no | later |
| Capacity/quota | Absent | source size, growth, target minimum, quota/cost ceiling | yes | yes | NA-0369 JSON | no real capacity evidence | intake response | yes | before real target use | yes | yes | no | yes |
| Retention/purge | Absent | window/count and prune owner | yes | yes | NA-0369 JSON | no repository/authority | intake response | yes | no | yes | yes | no | yes |
| Monitoring/alerting | Absent | alert classes and destination class | yes | yes | NA-0369 JSON | no monitoring model | intake response | before real operations | no | yes | yes | no | yes |
| Operator runbook | Absent | emergency stop, manual verification, escalation | yes | yes | NA-0369 JSON | no runbook owner | intake response | yes | yes | yes | yes | no | yes |
| Local-ops/backup-plan | Partial | history/index and backup coverage plan | no | yes | backup plan/status and workflow request | coverage gaps | later local-ops authorization | before real operations | before durable secret evidence | before real backup | yes | no | no |
| External review/public claims | Not complete | none for NA-0369 | boundary only | no | governance evidence | implementation/review gaps | keep claims bounded | before stronger claims | before stronger claims | before stronger claims | yes | no | no |

Decision categories:

- `OPERATOR_ACTION_PACKET_CREATED`
- `OPERATOR_RESPONSE_REQUIRED`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `CAPACITY_RETENTION_PREREQUISITE_REQUIRED`
- `MONITORING_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. NA-0370 should intake the operator response to this packet or record exact
   missing fields/blockers.
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

## Future Validation / Marker / Verification Plan

For the selected operator-response successor, future markers should include:

- `NA0370_OPERATOR_ACTION_PACKET_CREATED_OK`
- `NA0370_OPERATOR_RESPONSE_REQUIRED_OK`
- `NA0370_TARGET_CANDIDATE_VALUE_PRESENT_OR_BLOCKED_OK`
- `NA0370_HOST_IDENTITY_VALUE_PRESENT_OR_BLOCKED_OK`
- `NA0370_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0370_NO_REMOTE_CONNECTION_OK`
- `NA0370_NO_HOST_KEY_SCAN_OK`
- `NA0370_NO_SECRET_MATERIAL_OK`
- `NA0370_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0370_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0370_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0370_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0370_NO_METADATA_FREE_CLAIM_OK`
- `NA0370_NO_ANONYMITY_CLAIM_OK`
- `NA0370_NO_UNTRACEABLE_CLAIM_OK`

## Workflow-Support and History-Index Future Work Note

The local workflow-support request remains relevant. The qstart/qresume
fast-forward item would have reduced friction in this directive because the
local worktree was initially clean but stale and missing current helper
scripts. A future local-ops lane should consider qstart/qresume fast-forward,
response-file writer, bounded polling helpers, directive manifests, validation
profiles, allow-files, source/authority helpers, claim-boundary scanners,
history indexes, and backup coverage for directives/requests/journals/ops
history folders.

## Selected Successor

`NA-0370 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake`

Rationale: no operator response data is present. Credential-boundary, tool
installation, real key custody/recovery, local-ops, external review,
website/public-claim audit, and public technical paper work remain necessary
later, but the immediate blocker after creating the packet is whether the
operator can provide safe non-secret target candidate and host identity
evidence or must record why they remain blocked.

## Rejected Alternatives

- Credential-boundary / secret-handling prerequisite plan: rejected as the
  immediate successor because the operator packet has not yet received target
  and host identity response data.
- Restic / backup tool installation authorization plan: rejected because tool
  installation cannot safely precede target-candidate, host-identity,
  credential, capacity, and runbook boundaries.
- Real key custody / key recovery implementation authorization plan: rejected
  because real custody/recovery remains downstream of no-secret target and
  credential prerequisites.
- QSL local ops workflow support and history index plan: rejected as primary
  NA-0370 because it is important but does not supply the missing operator
  target/host-identity response; it remains required before real operations.
- External review readiness gap audit: rejected because prerequisite evidence
  is still incomplete.
- Website / public claim boundary audit: rejected because no public claim
  expansion is authorized and the public boundary is already preserved here.
- Public technical position paper evidence-bounded draft plan: rejected because
  current target/key/restore/service evidence gaps would dominate the paper
  boundary.
- Direct target setup, host-key scan, credential handling, repository init,
  backup, restore, deploy, or rollback: rejected as forbidden in NA-0369.

## Backup-Plan Impact Statement

No NA-0369 backup-plan update is required because the changed qsl-protocol
governance/testplan/journal paths and the no-secret JSON template remain under
`/srv/qbuild/work`, which the current local continuity backup covers. The JSON
template contains no secrets, no real target, no real host identity, no real
credential, and no durable evidence outside current qsl-protocol scope.

Future target connection, host identity capture, credential handling,
repository creation, backup tool installation, real key custody/recovery,
recovery-envelope work, monitoring setup, real restore target creation,
local-ops/history-index changes, backup-plan changes, backup script/timer/fstab
changes, or public-claim work remains separately authorized and backup-plan
gated.

## Next Recommendation

Proceed to NA-0370 as the no-secret operator response intake lane. NA-0370
should still forbid remote connection, host-key scan, known_hosts mutation,
credential handling, secret handling, off-host setup, repository init, tool
installation, backup, restore, deploy, rollback, backup script/timer/fstab
mutation, and public-claim expansion unless a future exact directive changes
that scope.
