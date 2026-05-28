Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0373 Metadata Runtime Off-Host Backup Operator Response Availability Blocker Collection Follow-Up Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0373 records the operator response availability blocker left after NA-0371
created a no-secret collection request and NA-0372 found no deliberate
operator response. The blocker is narrow: qsl-protocol has a request, but the
operator has not yet provided the non-secret target-candidate and host-identity
response needed before validation or real off-host backup work.

Classifications:

- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER_OK`
- `COLLECTION_REQUEST_REFERENCE_OK`
- `OPERATOR_RESPONSE_STILL_REQUIRED_OK`
- `TARGET_CANDIDATE_RESPONSE_REQUIRED_OK`
- `HOST_IDENTITY_RESPONSE_REQUIRED_OK`
- `CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NO_REMOTE_CONNECTION_OK`
- `NO_HOST_KEY_SCAN_OK`
- `NO_SECRET_MATERIAL_OK`
- `COLLECTION_FOLLOW_UP_PLAN_READY`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

Selected successor:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

NA-0373 is governance and blocker-follow-up evidence only. It does not set up a
target, connect to a remote host, scan host keys, mutate `known_hosts`, handle
credentials or secrets, initialize a repository, install tools, run backup or
restore operations, create a restore target, generate keys, upload keys,
collect passphrases, inspect private keys, create recovery-envelope content,
deploy, roll back, mutate qsl-server or qsl-attachments, change qshield
runtime, change qsc/qsp/protocol/crypto behavior, change dependencies, change
workflows, change public docs, mutate local backup scripts/timers/fstab, or
expand public claims.

## Live NA-0373 Scope

Live `NEXT_ACTIONS.md` records `NA-0373 -- Metadata Runtime Off-Host Backup
Operator Response Availability Blocker / Collection Follow-Up Plan` as the
sole READY item.

The live objective is to execute the metadata-runtime off-host backup lane
selected by NA-0372, beginning with an availability-blocker and collection
follow-up plan for the still-absent operator response required before target
candidate or host identity validation.

Live scope requires:

- no unsupported production, public-internet, external-review, or anonymity
  claims;
- no metadata-free or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- qsl-server/qsl-attachments production boundaries remain explicit;
- real off-host setup, real key handling, real restore drills,
  backup/restore/deploy/rollback, and backup-plan updates remain explicitly
  authorized before execution;
- no target setup, remote connection, host-key scan, `known_hosts` mutation,
  credential handling, secret handling, repository init, tool installation,
  backup, restore, deploy, rollback, real restore target creation/mount/copy,
  key generation, key upload, passphrase collection, private-key inspection,
  recovery-envelope content creation, backup-script/timer/fstab mutation,
  qsl-server mutation, qsl-attachments mutation, qshield runtime mutation,
  protocol/crypto/qsc/qsp implementation change, dependency change, workflow
  change, website/public-doc change, README change, START_HERE change, or
  public-claim expansion unless future exact scope authorizes it;
- response collection must remain no-secret; if secrets are supplied, stop
  without quoting or copying them.

The live scope and this directive align. A new follow-up `inputs/` artifact was
not added because the live NA-0373 queue entry does not explicitly authorize a
new durable `inputs/metadata_runtime/` artifact path for NA-0373.

## Inherited NA-0372 Response Absence Result

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

NA-0372 selected NA-0373 because target candidate and host identity validation
remain blocked by response availability.

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

The request forbids private keys, passphrases, passwords, tokens, raw
credentials, recovery-envelope contents, secret/private material paths,
secret-bearing command output, live connection output, sensitive `known_hosts`
content, and real host fingerprints unless a future directive explicitly
authorizes collection and storage.

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

The packet is a placeholder-only no-secret template. It includes target
candidate, host identity, credential placeholder, capacity/quota/retention,
monitoring/alerting, runbook, forbidden-input, stop-condition,
public-claim-boundary, qsl-server/qsl-attachments/qshield boundary,
backup-plan-impact, and future validation marker fields.

The NA-0369 packet remains a template only. It is not a response, target setup,
configured target, verified host identity, off-host backup completion, real
restore completion, real key custody/recovery implementation, or complete
disaster recovery.

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
| qsl-server | Local read-only worktree at `d40e6003fdf`; PR #56 merged at `d40e6003fdf`; remote default branch `main` visible through GitHub | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local read-only worktree at `96b9352bd63`; PR #37 merged at `96b9352bd63`; remote default branch `main` visible through GitHub | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Neither repository is mutated by NA-0373.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted as local same-host continuity storage.
- `/backup/qsl` has about 916G total and about 886G available.
- `/srv/qbuild` has about 468G total and about 368G available.
- Daily local continuity snapshots, manifests, and logs exist through
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
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`,
  `/home/victor/work/qsl/codex/responses`, and
  `/home/victor/work/qsl/codex/requests` are present.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` are absent at inspection.
- The backup status source list covers qbuild work, Codex responses, and the
  backup plan, but does not prove full coverage for requests, directives,
  journals, or broader ops/history folders.
- The D132 preservation bundle remains present under the expected local qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `COLLECTION_REQUEST_CREATED`
- `OPERATOR_RESPONSE_STILL_ABSENT`
- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`
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

## Response-Availability Blocker Review

Authorized response discovery locations:

- `/home/victor/work/qsl/codex/requests/`, filename-matched for NA-0371
  operator response, NA-0372 intake response, NA-0373 follow-up response,
  off-host target candidate response, host identity response, and off-host
  response terms;
- qsl-protocol `inputs/metadata_runtime/`, filename-matched for operator
  response, target-candidate response, and host-identity response terms.

Result:

- No candidate response file was found in the authorized requests directory.
- The only qsl-protocol metadata input file matching the response-oriented
  filename pattern was the NA-0371 collection request artifact:
  `inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`.
- That artifact is request evidence, not an operator response.
- No candidate response content was parsed because no response candidate
  existed.

Classification:

- `RESPONSE_STILL_ABSENT`
- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`

Exact blocker: qsl-protocol has no deliberate no-secret operator response
after the NA-0371 collection request, so no target candidate, host identity,
capacity/retention, monitoring/runbook, no-secret affirmation, or public-claim
acknowledgement value can be used for validation.

## Collection Follow-Up Plan

Missing response:

- a deliberate no-secret operator response to the NA-0371 collection request;
- a non-secret target label/alias and target class confirmation;
- a non-secret owner/contact label and safe location/jurisdiction class;
- a non-secret host identity evidence source description;
- fingerprint algorithm/format class if safe, without a real fingerprint value
  unless a future directive authorizes collection/storage;
- credential model/storage boundary class, without credential material;
- capacity, quota, retention, monitoring, alerting, runbook, emergency contact,
  no-secret affirmation, and public-claim boundary acknowledgements.

Existing request artifact:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`

Why absence blocks validation:

- target validation needs at least a non-secret target candidate response;
- host identity validation needs at least a non-secret evidence-source and
  algorithm/format-class response;
- credential, tool, key, repository, monitoring, restore, backup, deploy, and
  rollback work would be premature because there is no target/host response to
  anchor the boundary;
- fail-closed governance must preserve absence rather than infer values.

The existing NA-0371 request is sufficient as the canonical request. The
follow-up plan is to re-run intake after this blocker evidence lands, not to
create a second request artifact in NA-0373. Operator action must occur outside
Codex by placing a no-secret response in the authorized request path. Local
ops/history-index work remains valuable, but it should not supersede the next
response intake because the current primary blocker remains response
availability.

## Operator Instruction / Forbidden-Input Refinement

The operator should provide only:

- non-secret target label/alias;
- target class confirmation;
- non-secret owner/contact label;
- high-level location/jurisdiction class if safe;
- non-secret host identity evidence source description;
- fingerprint format/algorithm class if safe;
- capacity/retention estimates;
- monitoring destination class, not credential;
- emergency contact label;
- runbook owner label;
- public-claim boundary acknowledgement;
- no-secret affirmation.

The operator must never provide:

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
- real host fingerprints unless a future directive explicitly authorizes
  collection and storage.

## Optional No-Secret Follow-Up Artifact Implementation

No optional follow-up JSON artifact was added in NA-0373.

Rationale: the directive listed an optional path, but only if live NA-0373 scope
explicitly permits it. The live queue entry authorizes the blocker/follow-up
plan and requires no-secret collection boundaries, but it does not explicitly
authorize a new durable `inputs/metadata_runtime/` artifact path. The existing
NA-0371 collection request remains the canonical no-secret request artifact.

## Follow-Up Artifact Validation / No-Secret Checks

Not applicable because no new follow-up JSON artifact was added.

Validation still confirmed:

- the NA-0369 action packet JSON parses;
- the NA-0371 collection request JSON parses;
- authorized response discovery found no response candidate;
- no response content was copied, quoted, parsed, or stored;
- no real target endpoint, real host key, credential, private key, token,
  passphrase, recovery-envelope content, connection command, backup command, or
  restore command was introduced.

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

No NA-0373 backup-plan update is required. This PR changes only qsl-protocol
governance, traceability, testplan, and rolling-journal files and stores no
real operator response.

Local-ops/history-index work remains useful because read-only history for
responses and requests exists, but directive and journal history paths are
absent and request/journal/ops history backup coverage is not fully proven.
That work should precede real target connection, credential handling,
repository creation, key custody/recovery implementation, monitoring setup,
backup execution, restore execution, deployment, rollback, and durable storage
of any future response outside existing backup scope.

Local-ops does not become NA-0374 because the immediate blocker remains
response availability and the next useful step is response intake after this
follow-up evidence lands.

## Public-Ingress/Timing/Traffic-Shape Boundary

NA-0373 is not public-ingress implementation, target setup, host identity
verification, credential handling, or backup execution. It provides no new
evidence about public-internet readiness, attachment size confidentiality,
timing metadata, traffic shape, or all metadata.

qsl-server PR #56 remains bounded end-to-end harness evidence only.
qsl-attachments PR #37 remains service-local prerequisite evidence only.
qshield embedded relay/demo evidence remains reference/oracle evidence only.

## External-Review Sensitivity

External review remains incomplete. NA-0373 does not create, update, or claim
an external-review package. The public technical position paper remains
future-gated until response availability, target/host identity planning,
credential boundary planning, real off-host backup prerequisites, and public
claim boundaries have settled.

## Public Claim Boundary

NA-0373 preserves these boundaries:

- response-availability blocker planning is not target setup;
- response-availability blocker planning is not host identity verification;
- no remote connection occurs;
- no host-key scan occurs;
- no credential is handled;
- no-secret target/tool harness evidence is not real off-host backup;
- no-secret isolated restore harness evidence is not real restore;
- no-secret key harness evidence is not real key custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- no website/public docs update is made;
- no claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden
  traffic shape, hidden all metadata, complete disaster recovery, complete
  off-host backup, real restore completion, verified host identity, configured
  target, implemented real key custody, or implemented real key recovery.

## Decision Matrix

| Area | Status | Evidence source | Blocker | Next action | Extra collection needed? | Credential plan needed? | Target/host validation can proceed? | Must precede remote connection? | Must precede credential handling? | Must precede real backup? | NA-0374 candidate? |
|---|---|---|---|---|---|---|---|---|---|---|---|
| response availability | `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER` | Authorized discovery | No response candidate | Intake after follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| existing collection request | `COLLECTION_REQUEST_REFERENCE_OK` | NA-0371 JSON | Response not supplied | Reuse canonical request | No new artifact | No | No | Yes | Yes | Yes | Supports |
| follow-up request | `COLLECTION_FOLLOW_UP_PLAN_READY` | NA-0373 evidence | Artifact not added due live scope | Intake after evidence merge | Yes | No | No | Yes | Yes | Yes | Yes |
| target candidate | `TARGET_CANDIDATE_RESPONSE_REQUIRED` | No response candidate | No target value | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| host identity | `HOST_IDENTITY_RESPONSE_REQUIRED` | No response candidate | No source/format class | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| credential boundary | `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED` | NA-0371 request | Secret handling forbidden | Future boundary plan after response | Yes | Yes | No | Yes | Yes | Yes | Later |
| capacity/quota | `CAPACITY_RETENTION_REAL_VALUE_ABSENT` | No response candidate | No value | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| retention/purge | `CAPACITY_RETENTION_REAL_VALUE_ABSENT` | No response candidate | No value | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| monitoring/alerting | `MONITORING_ALERTING_REAL_VALUE_ABSENT` | No response candidate | No value | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| operator runbook | `MONITORING_ALERTING_REAL_VALUE_ABSENT` | No response candidate | No value | Operator response intake | Yes | No | No | Yes | Yes | Yes | Yes |
| local-ops/backup-plan | `LOCAL_OPS_NOT_PRIMARY_BLOCKER` | Backup/status evidence | Coverage/index gaps | Future local-ops plan | No | No | No real operations | Yes, before real operations | Yes, before real operations | Yes | Later |
| external review/public claims | `NOT_READY` | Boundary evidence | Evidence gaps remain | Future audit/paper plan | No | No | No | Yes | Yes | Yes | Later |

Decision categories:

- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`
- `COLLECTION_FOLLOW_UP_PLAN_READY`
- `OPERATOR_RESPONSE_STILL_REQUIRED`
- `TARGET_CANDIDATE_RESPONSE_REQUIRED`
- `HOST_IDENTITY_RESPONSE_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. Merge NA-0373 blocker/follow-up evidence.
2. Restore the exact NA-0374 response-intake-after-follow-up successor.
3. Re-check authorized request locations for a deliberate no-secret operator
   response.
4. If a response exists, validate no-secret boundaries before copying or
   summarizing any detail.
5. If no response exists, record the still-absent state and decide whether a
   required-stop gate or local-ops/history-index lane should come next.
6. Only after a materially complete no-secret response, plan target/host
   identity validation without live verification claims.
7. Only after target/host identity validation planning, authorize credential
   boundary planning without handling secrets.
8. Only after credential, local-ops/backup coverage, tool, repository, key
   custody/recovery, monitoring, and restore prerequisites are explicitly
   authorized may real operations be considered.

## Future Validation/Marker/Verification Plan

For the selected NA-0374 successor, require:

- `NA0374_COLLECTION_FOLLOW_UP_REFERENCE_OK`
- `NA0374_OPERATOR_RESPONSE_INTAKE_AFTER_FOLLOW_UP_OK`
- `NA0374_OPERATOR_RESPONSE_PRESENT_OR_BLOCKED_OK`
- `NA0374_TARGET_CANDIDATE_RESPONSE_PRESENT_OR_BLOCKED_OK`
- `NA0374_HOST_IDENTITY_RESPONSE_PRESENT_OR_BLOCKED_OK`
- `NA0374_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0374_NO_REMOTE_CONNECTION_OK`
- `NA0374_NO_HOST_KEY_SCAN_OK`
- `NA0374_NO_SECRET_MATERIAL_OK`
- `NA0374_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0374_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0374_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0374_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0374_NO_METADATA_FREE_CLAIM_OK`
- `NA0374_NO_ANONYMITY_CLAIM_OK`
- `NA0374_NO_UNTRACEABLE_CLAIM_OK`

If NA-0374 instead finds a secret-bearing or invalid response, it must stop
without quoting or copying the sensitive material and preserve all
no-connection/no-host-key-scan/no-real-operation boundaries.

## Workflow-Support and History-Index Future Work Note

Read-only response and request history paths were present and improved handoff
confidence. Directive and journal history paths were absent at inspection. A
future local-ops maintenance directive should create or update an index mapping
directive IDs, NA IDs, response paths, PRs, decisions, selected successors, and
merge SHAs.

The accepted workflow-support request remains relevant. These future local-ops
items would materially reduce friction:

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

NA-0373 does not implement those workflow-support items.

## Selected Successor

Selected successor:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

Rationale: the operator response remains absent, but NA-0373 clarifies the
follow-up plan and the existing NA-0371 request remains sufficient. The next
truthful lane is to perform another no-secret intake after this follow-up
evidence, not to wait indefinitely before checking again and not to skip ahead
to credential, tool, key, restore, local-ops, external-review, website, or
paper work.

## Rejected Alternatives

- `NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Operator Input`: rejected for this handoff because the follow-up plan is concrete and a bounded intake after follow-up can truthfully determine whether the operator has responded.
- `NA-0374 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Completion Request`: rejected because there is no partial response to complete.
- `NA-0374 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Response Validation Plan`: rejected because there is no response to validate.
- `NA-0374 -- Metadata Runtime Off-Host Backup Credential Boundary / Secret Handling Prerequisite Plan`: rejected as premature before non-secret target/host response availability.
- `NA-0374 -- QSL Local Ops Codex Workflow Support and History Index Plan`: rejected as useful but not the primary blocker for this lane.
- `NA-0374 -- Metadata Runtime External Review Readiness Gap Audit`: rejected as premature before off-host backup evidence gaps are resolved.
- `NA-0374 -- Metadata Runtime Website / Public Claim Boundary Audit`: rejected as not the current blocker; no public docs are changed.
- `NA-0374 -- Public Technical Position Paper Evidence-Bounded Draft Plan`: rejected as premature before evidence gaps settle.

## Backup-Plan Impact Statement

No NA-0373 backup-plan update is required. This PR changes only qsl-protocol
governance, traceability, testplan, and rolling-journal files and stores no
real operator response.

Future operator response storage outside the current backed-up scope, real
target connection, host identity capture, credential handling, repository/tool
setup, real key custody/recovery, monitoring, local-ops/history-index work,
backup, restore, deploy, rollback, and public-claim mutation remain
backup-plan and exact-authorization gated.

## Next Recommendation

Close NA-0373 after this evidence merges and restore:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

NA-0374 must not implement target setup, remote connection, host-key scan,
credential handling, secret handling, repository init, tool installation,
backup, restore, deploy, rollback, real restore target creation/mount/copy, key
generation, key upload, passphrase collection, private-key inspection,
recovery-envelope content creation, local backup script/timer/fstab mutation,
or public-claim expansion unless future exact scope authorizes it.
