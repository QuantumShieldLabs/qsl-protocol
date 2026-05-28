Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0375 Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Input

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0375 performs the required stop after repeated response-intake lanes found no
deliberate no-secret operator response for the off-host backup target-candidate
and host-identity chain.

Final response discovery result:

- `RESPONSE_STILL_ABSENT`
- `OPERATOR_RESPONSE_REQUIRED_STOP`
- `CODEX_ONLY_PROGRESS_ON_OFF_HOST_TARGET_CHAIN_BLOCKED`
- `TARGET_CANDIDATE_REAL_VALUE_ABSENT`
- `HOST_IDENTITY_REAL_VALUE_ABSENT`
- `CREDENTIAL_BOUNDARY_ABSENT`
- `CREDENTIAL_SECRET_HANDLING_FORBIDDEN`
- `CAPACITY_RETENTION_REAL_VALUE_ABSENT`
- `MONITORING_ALERTING_REAL_VALUE_ABSENT`
- `REAL_TARGET_ACCESS_NOT_AUTHORIZED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`
- `OFF_HOST_BACKUP_NOT_READY`

Selected successor:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`

Rationale: the off-host target chain is blocked on external operator input, and
another blind response-intake lane would loop. The live NA-0375 objective allows
the Director to select another lane while the off-host path is held. Local-ops
workflow-support and directive/response/history-index planning can reduce
operational friction without implementing runtime, service, protocol, crypto,
backup, restore, remote, credential, or secret-handling changes.

NA-0375 is governance and required-stop evidence only. It does not set up a
target, connect to a remote host, scan host keys, mutate `known_hosts`, handle
credentials or secrets, initialize a repository, install tools, run backup or
restore operations, create a restore target, generate keys, upload keys,
collect passphrases, inspect private keys, create recovery-envelope content,
deploy, roll back, mutate qsl-server or qsl-attachments, change qshield
runtime, change qsc/qsp/protocol/crypto behavior, change dependencies, change
workflows, change public docs, mutate local backup scripts/timers/fstab, or
expand public claims.

## Live NA-0375 Scope

Live `NEXT_ACTIONS.md` records `NA-0375 -- Metadata Runtime Off-Host Backup
Operator Response Required Stop / Await Operator Input` as the sole READY item.

The live objective is to stop or hold the off-host backup target/host-identity
path until the operator provides a deliberate no-secret response or the
Director selects another lane.

Live scope requires:

- no unsupported production, public-internet, external-review, or anonymity
  claims;
- no metadata-free or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- no target, host, credential, backup, or restore work without exact future
  authorization;
- no secret handling;
- qsl-server/qsl-attachments production boundaries remain explicit;
- no target setup, remote connection, host-key scan, `known_hosts` mutation,
  credential handling, secret handling, repository init, tool installation,
  backup, restore, deploy, rollback, real restore target creation/mount/copy,
  key generation, key upload, passphrase collection, private-key inspection,
  recovery-envelope content creation, backup-script/timer/fstab mutation,
  qsl-server mutation, qsl-attachments mutation, qshield runtime mutation,
  protocol/crypto/qsc/qsp implementation change, dependency change, workflow
  change, website/public-doc change, README change, START_HERE change, or
  public-claim expansion unless future exact scope authorizes it.

The live scope permits a required-stop evidence PR and permits successor
selection because the objective includes the Director selecting another lane.
The optional NA-0375 `inputs/metadata_runtime/` status JSON artifact is not
added because the live NA-0375 queue entry does not explicitly authorize a new
durable `inputs/` artifact.

## Inherited NA-0374 Response Absence After Follow-Up

NA-0374 searched the authorized local read-only response locations after the
NA-0373 collection follow-up and found no deliberate operator response
candidate.

Inherited classifications:

- `RESPONSE_STILL_ABSENT_AFTER_FOLLOW_UP`
- `OPERATOR_RESPONSE_STILL_ABSENT_AFTER_FOLLOW_UP`
- `OPERATOR_RESPONSE_REQUIRED_STOP_READY`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_VALUE_FORBIDDEN`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`
- `OFF_HOST_BACKUP_NOT_READY`

NA-0374 selected NA-0375 as the required-stop successor. PR #1010 remains
closed unmerged and is not merge evidence. Clean replacement PR #1011 merged as
`36529a4ab387`, and PR #1012 restored NA-0375 as the sole READY item on
`origin/main` at `756c292d78ce`.

## Inherited NA-0373 Collection Follow-Up

NA-0373 recorded `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER` and
`COLLECTION_FOLLOW_UP_PLAN_READY` after NA-0372 found no deliberate operator
response after the NA-0371 request. NA-0373 did not add another durable request
artifact; the NA-0371 collection request remains canonical.

## Inherited NA-0372 Response Absence

NA-0372 searched authorized response locations after the NA-0371 collection
request and found no deliberate no-secret operator response candidate.

Inherited classifications:

- `NO_RESPONSE_CANDIDATE_FOUND`
- `OPERATOR_RESPONSE_STILL_ABSENT`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_VALUE_FORBIDDEN`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Inherited NA-0371 Collection Request

NA-0371 created:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`

The artifact parses as JSON and is a no-secret request, not a response. It asks
for non-secret target candidate, host identity source/format class, credential
placeholder, capacity/quota/retention, monitoring/alerting, operator runbook,
stop-condition, and public-claim-boundary fields only.

Recommended response locations from NA-0371:

- `/home/victor/work/qsl/codex/requests/NA0371_operator_response_<date>.md`
- `/home/victor/work/qsl/codex/requests/NA0371_operator_response_<date>.json`

## Inherited NA-0370 Intake Result

NA-0370 searched authorized local read-only locations before the NA-0371
collection request and found no operator response candidate for the NA-0369
operator action packet.

Inherited classifications:

- `OPERATOR_RESPONSE_NOT_PRESENT`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Inherited NA-0369 Operator Action Packet

NA-0369 created:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json`

The packet parses as JSON and remains a placeholder-only no-secret template. It
is not a response, target setup, configured target, verified host identity,
off-host backup completion, real restore completion, real key custody/recovery
implementation, or complete disaster recovery.

## Inherited No-Secret Harness Evidence from NA-0365, NA-0363, NA-0361, NA-0359

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness. It
is simulated isolated-restore evidence only and is not a real restore drill.

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and
harness. It is simulated target/tool evidence only and is not a real target,
configured repository, tool installation, remote connection, backup, or
restore.

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and
harness. It is simulated custody/recovery evidence only and is not real key
custody or real key recovery.

NA-0359 added a qsl-protocol no-secret restore-drill dry-run fixture and
harness. It is dry-run evidence only and is not real restore execution.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

| Repository | Source | Authority | CI | Classification |
|---|---|---|---|---|
| qsl-server | Local read-only worktree at `d40e6003fdf0`; remote `HEAD` at `d40e6003fdf0`; PR #56 merged at `d40e6003fdf0` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf0`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local read-only worktree at `96b9352bd63e`; remote `HEAD` at `96b9352bd63e`; PR #37 merged at `96b9352bd63e` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63e`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only, not
  production/public-internet proof.
- qsl-attachments PR #37 remains service-local prerequisite evidence only, not
  production/public-internet proof.
- Neither repository is mutated by NA-0375.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted as local same-host continuity storage.
- `/backup/qsl` has about 916G total and about 885G available.
- `/srv/qbuild` has about 468G total and about 362G available.
- Daily local continuity snapshots, manifests, and logs exist through
  `daily-20260528T023303-0500`.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint and daily local
  continuity snapshots.
- `qsl-backup-daily.timer` is enabled and active/waiting.
- `restic`, `borg`, `rclone`, and `age` are absent.
- `gpg`, `ssh`, and `rsync` are present.
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`,
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`,
  `/home/victor/work/qsl/codex/responses`, and
  `/home/victor/work/qsl/codex/requests` are present.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` were absent at inspection.
- The backup status source list covers qbuild work, Codex responses, and the
  backup plan, but does not prove full coverage for requests, directives,
  journals, or broader ops/history folders.
- The D132 preservation bundle remains present under the expected local qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `OPERATOR_RESPONSE_REQUIRED_STOP`
- `TARGET_CANDIDATE_REAL_VALUE_ABSENT`
- `HOST_IDENTITY_REAL_VALUE_ABSENT`
- `CREDENTIAL_BOUNDARY_ABSENT`
- `CREDENTIAL_SECRET_HANDLING_FORBIDDEN`
- `CAPACITY_RETENTION_REAL_VALUE_ABSENT`
- `MONITORING_ALERTING_REAL_VALUE_ABSENT`
- `REAL_TARGET_ACCESS_NOT_AUTHORIZED`
- `OFF_HOST_BACKUP_NOT_READY`

Local backup remains same-host continuity only. It is not complete disaster
recovery, and no off-host encrypted backup is implemented or proven.

## Final Operator Response Discovery

Authorized discovery searched:

- `/home/victor/work/qsl/codex/requests/` for NA-0371/NA-0372/NA-0373/NA-0374
  /NA-0375 operator response, target candidate response, host identity
  response, and off-host response filename patterns.
- qsl-protocol `inputs/metadata_runtime/` for operator response,
  target-candidate response, and host-identity response filename patterns.

Discovery result:

- No matching response candidate exists under
  `/home/victor/work/qsl/codex/requests/`.
- qsl-protocol `inputs/metadata_runtime/` matched only
  `off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`.
  That file is the NA-0371 collection request, not an operator response.

Classification:

- `RESPONSE_STILL_ABSENT`
- `OPERATOR_RESPONSE_REQUIRED_STOP`

No sensitive candidate response content was found, copied, or quoted.

## Required-Stop Classification

NA-0375 records the required stop:

- Codex cannot continue the real off-host target/host-identity chain without
  operator input.
- Target candidate remains absent.
- Host identity remains absent.
- Credential boundary remains absent and credential material remains forbidden.
- Capacity/quota/retention evidence remains absent.
- Monitoring/alerting/runbook evidence remains absent.
- Real target connection is blocked.
- Real backup operation is blocked.
- Real restore execution is blocked.
- Real key custody and recovery remain unimplemented.

Continuing another blind off-host-backup response-intake lane without operator
input would repeat the same absence proof. The queue should pivot to bounded
local-ops planning while awaiting external input.

## Missing Input / Forbidden Input Operator Instruction

Required non-secret operator input:

- non-secret target label or alias;
- target class confirmation;
- non-secret owner/contact label;
- high-level location or jurisdiction class if safe;
- non-secret host identity evidence source description;
- fingerprint format/algorithm class if safe;
- capacity and retention estimates;
- monitoring destination class, not credential material;
- emergency contact label;
- runbook owner label;
- public-claim boundary acknowledgement;
- no-secret affirmation.

The operator must not provide:

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
- unredacted sensitive `known_hosts` content;
- host fingerprints unless a future directive explicitly authorizes collection
  and storage.

## Local-Ops / History-Index Successor Analysis

The off-host target chain is blocked on external operator response. Local-ops
workflow-support has been repeatedly identified as useful and bounded. It can
reduce repeated command-shape, handoff, response-file, polling, validation, and
history-index friction without touching secrets, remotes, runtime behavior, or
backup scripts if scoped as planning/authorization first.

Future local-ops improvements that would materially reduce friction:

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

Classification:

- `LOCAL_OPS_PRIMARY_SUCCESSOR_WHILE_AWAITING_OPERATOR_INPUT`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Current local backup evidence proves same-host continuity, not complete
disaster recovery. The current status file proves coverage for qbuild work,
Codex responses, and the backup plan, but not full coverage for requests,
directives, journals, or broader ops/history folders. The request and response
history access request supports a future history-index lane.

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_PRIMARY_SUCCESSOR_WHILE_AWAITING_OPERATOR_INPUT`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

No NA-0375 backup-plan update is required because this PR changes only
qsl-protocol governance/testplan/traceability/journal paths and stores no real
operator response. Future operator response storage, target connection, host
identity capture, credential handling, repository/tool setup, real key
custody/recovery, monitoring, local-ops/history-index implementation, backup,
restore, deploy, rollback, or public-claim mutation remains backup-plan and
exact-authorization gated.

## Public-Ingress / Timing / Traffic-Shape Boundary

NA-0375 does not change public ingress, timing, retry, padding, batching, cover
traffic, attachment size-class, traffic-shape, or service behavior. It does not
claim that attachment size, timing metadata, traffic shape, or all metadata is
hidden.

## External-Review Sensitivity

External review remains not complete. qsl-server PR #56 remains bounded harness
evidence only. qsl-attachments PR #37 remains service-local prerequisite
evidence only. qshield embedded relay/demo evidence remains reference/oracle
evidence only.

## Public Claim Boundary

NA-0375 introduces no claim of production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceability,
complete off-host backup, complete disaster recovery, target configuration,
verified host identity, real restore completion, real key custody, or real key
recovery.

No website, public docs, README, or START_HERE update is made.

## Decision Matrix

| Area | Status | Evidence source | Blocker | Next action | Operator input required | Local-ops can proceed in parallel |
|---|---|---|---|---|---|---|
| Operator response existence | `OPERATOR_RESPONSE_REQUIRED_STOP` | NA-0375 discovery | No response candidate | Await no-secret operator response | yes | yes |
| Target candidate | absent | NA-0369/NA-0371 templates; NA-0375 discovery | no operator target label/class/contact | collect non-secret target fields | yes | yes |
| Host identity | absent | NA-0369/NA-0371 templates; NA-0375 discovery | no non-secret source/format class | collect source description and class data | yes | yes |
| Credential boundary | absent/forbidden | NA-0369/NA-0371 forbidden input rules | no boundary plan, secrets forbidden | future credential-boundary prerequisite | yes, non-secret only | yes |
| Capacity/quota | absent | NA-0371 request fields | no estimate | collect class estimates | yes | yes |
| Retention/purge | absent | NA-0371 request fields | no operator intent | collect retention class | yes | yes |
| Monitoring/alerting | absent | NA-0371 request fields | no destination class/runbook owner | collect non-secret classes | yes | yes |
| Operator runbook | absent | NA-0371 request fields | no owner label | collect owner label | yes | yes |
| Local-ops/backup-plan | partial | local backup status and requests | history index and coverage gaps | NA-0376 planning | no for planning | yes |
| External review/public claims | not complete/not expanded | public-claim boundary evidence | review not complete; target not ready | keep claims bounded | no | yes |

Decision categories:

- `OPERATOR_RESPONSE_REQUIRED_STOP`
- `OPERATOR_RESPONSE_STILL_ABSENT`
- `LOCAL_OPS_PRIMARY_SUCCESSOR_WHILE_AWAITING_OPERATOR_INPUT`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

Target and host validation must precede any remote connection. Credential
boundary planning must precede credential handling. Target/host/credential/key
and repository prerequisites must precede real backup operations.

## Future Staged Implementation Strategy

1. NA-0376 should produce a local-ops workflow-support/history-index plan only.
2. A future operator-response lane should intake only deliberate no-secret
   response data.
3. A future validation lane should classify target candidate and host identity
   evidence before connection.
4. A future credential-boundary lane should define secret handling without
   collecting secrets unless explicitly authorized.
5. Only after those prerequisites should any target connection, host-key
   verification, repository initialization, tool installation, key custody,
   backup, restore, monitoring, deploy, rollback, or public-claim work be
   considered.

## Future Validation / Marker / Verification Plan

Markers for the selected NA-0376 local-ops successor:

- `NA0376_LOCAL_OPS_WORKFLOW_SUPPORT_PLAN_OK`
- `NA0376_HISTORY_INDEX_PLAN_OK`
- `NA0376_BACKUP_COVERAGE_PLAN_OK`
- `NA0376_QSTART_QRESUME_FAST_FORWARD_PLAN_OK`
- `NA0376_RESPONSE_WRITER_PLAN_OK`
- `NA0376_BOUNDED_POLLING_HELPER_PLAN_OK`
- `NA0376_DIRECTIVE_MANIFEST_PLAN_OK`
- `NA0376_VALIDATION_PROFILE_PLAN_OK`
- `NA0376_ALLOWED_FILE_PLAN_OK`
- `NA0376_SOURCE_AUTHORITY_HELPER_PLAN_OK`
- `NA0376_CLAIM_BOUNDARY_SCANNER_PLAN_OK`
- `NA0376_NO_RUNTIME_CHANGE_OK`
- `NA0376_NO_BACKUP_SCRIPT_CHANGE_OK`
- `NA0376_NO_SECRET_MATERIAL_OK`
- `NA0376_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0376_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0376_NO_METADATA_FREE_CLAIM_OK`
- `NA0376_NO_ANONYMITY_CLAIM_OK`
- `NA0376_NO_UNTRACEABLE_CLAIM_OK`

## Workflow-Support and History-Index Future Work Note

The local workflow-support request at
`/home/victor/work/qsl/codex/requests/codex_workflow_support_request_20260523.md`
is relevant and should be used as input for NA-0376. NA-0375 does not implement
any of those items.

## Selected Successor

Selected:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`

## Rejected Alternatives

- `NA-0376 -- Metadata Runtime Off-Host Backup Operator Response Awaiting External Input`: rejected because a pure wait keeps the queue blocked when bounded local-ops planning can proceed.
- `NA-0376 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Completion Request`: rejected because another request/intake loop would repeat absence without operator action.
- `NA-0376 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Response Validation Plan`: rejected because there is no response to validate.
- `NA-0376 -- Metadata Runtime Off-Host Backup Credential Boundary / Secret Handling Prerequisite Plan`: rejected as premature until target/host non-secret input exists.
- `NA-0376 -- Metadata Runtime External Review Readiness Gap Audit`: rejected as lower priority than local-ops friction while target input is absent.
- `NA-0376 -- Metadata Runtime Website / Public Claim Boundary Audit`: rejected because NA-0375 makes no website/public claim mutation.
- `NA-0376 -- Public Technical Position Paper Evidence-Bounded Draft Plan`: rejected as future-gated until operational evidence and claim boundaries are more stable.

## Backup-Plan Impact Statement

No NA-0375 backup-plan update is required. The lane changes only qsl-protocol
governance/testplan/traceability/journal paths and stores no real operator
response. No real target, host identity value, credential, key, recovery
envelope, repository, backup artifact, restore target, monitoring destination,
or off-host path is created.

## Next Recommendation

After NA-0375 evidence merges and public-safety is green, close NA-0375 and
restore:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`
