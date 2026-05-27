Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0372 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0372 intakes the state of any deliberate no-secret operator response created
from the NA-0371 collection request. Authorized discovery found no such
operator response candidate after the collection request.

Classification:

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

Selected successor:

`NA-0373 -- Metadata Runtime Off-Host Backup Operator Response Availability Blocker / Collection Follow-Up Plan`

NA-0372 is intake/governance evidence only. It does not set up an off-host
target, connect to a remote host, scan host keys, mutate `known_hosts`, handle
credentials or secrets, initialize a repository, install tools, run backup or
restore operations, create a restore target, generate keys, upload keys,
collect passphrases, inspect private keys, create recovery-envelope content,
deploy, roll back, mutate qsl-server or qsl-attachments, change qshield
runtime, change qsc/qsp/protocol/crypto behavior, change dependencies, change
workflows, change public docs, mutate local backup scripts/timers/fstab, or
expand public claims.

## Live NA-0372 Scope

Live `NEXT_ACTIONS.md` records `NA-0372 -- Metadata Runtime Off-Host Backup
Target Candidate / Host Identity Operator Response Intake After Collection
Request` as the sole READY item. The live objective is to begin with intake of
any deliberate no-secret operator response created from the NA-0371 collection
request.

The live scope requires:

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

The live scope and this directive align. The optional intake status JSON
artifact was not added because the live queue entry does not explicitly
authorize a new `inputs/` status artifact.

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

The request explicitly forbids private keys, passphrases, passwords, tokens,
raw credentials, recovery-envelope contents, secret/private material paths,
secret-bearing command output, live connection output, sensitive
`known_hosts` content, and real host fingerprints unless a future directive
authorizes how to collect and store them.

## Inherited NA-0370 Intake Result

NA-0370 searched authorized local read-only locations before the NA-0371
collection request and found no operator response candidate for the NA-0369
operator action packet.

Inherited classifications:

- `NO_RESPONSE_CANDIDATE_FOUND`
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
| qsl-server | Local read-only worktree at `d40e6003fdf`; remote `main` at `d40e6003fdf`; PR #56 merged at `d40e6003fdf` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local read-only worktree at `96b9352bd63`; remote `main` at `96b9352bd63`; PR #37 merged at `96b9352bd63` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Neither repository is mutated by NA-0372.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted as local same-host continuity storage.
- `/backup/qsl` has about 916G total and about 886G available.
- `/srv/qbuild` has about 468G total and about 370G available.
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
  `/home/victor/work/qsl/codex/journals` were absent or had no files at
  inspection.
- The backup status source list covers qbuild work, Codex responses, and the
  backup plan, but not all request/journal/history folders as durable
  local-ops evidence.
- The D132 preservation bundle remains present under the expected local qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `COLLECTION_REQUEST_CREATED`
- `OPERATOR_RESPONSE_STILL_ABSENT`
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

## Operator Response Discovery After Collection Request

Authorized response discovery locations:

- `/home/victor/work/qsl/codex/requests/`, filename-matched for NA-0371
  operator response, NA-0372 operator response intake, target candidate
  response, host identity response, and off-host response terms;
- qsl-protocol `inputs/metadata_runtime/`, filename-matched for operator
  response, target-candidate response, and host-identity response terms.

Result:

- No candidate response file was found in the authorized requests directory.
- The only qsl-protocol metadata input file matching the response-oriented
  filename pattern was the NA-0371 collection request artifact:
  `inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`.
- That artifact is classified as request evidence, not an operator response.
- No candidate response content was parsed because no response candidate
  existed.
- Exact candidate response files: none.

Classification:

- `NO_RESPONSE_CANDIDATE_FOUND`
- `OPERATOR_RESPONSE_STILL_ABSENT`

## Operator Response Intake Classification

Because no operator response candidate was present, NA-0372 records fail-closed
absence rather than inferring target evidence.

| Intake item | Classification | Evidence | Next action |
|---|---|---|---|
| Operator response | `OPERATOR_RESPONSE_STILL_ABSENT` | No authorized response candidate file found after NA-0371 request | Follow up collection |
| Target candidate | `TARGET_CANDIDATE_VALUE_ABSENT` | No response value | Required operator input |
| Host identity | `HOST_IDENTITY_VALUE_ABSENT` | No response value | Required operator input; future validation remains separate |
| Credential response | `CREDENTIAL_PLACEHOLDER_ONLY` and `CREDENTIAL_VALUE_FORBIDDEN` | NA-0371 request forbids secret material | Credential boundary remains future-authorized |
| Capacity/retention | `CAPACITY_RETENTION_VALUE_ABSENT` | No response value | Required operator input |
| Monitoring/runbook | `MONITORING_RUNBOOK_VALUE_ABSENT` | No response value | Required operator input |

Required blockers remain:

- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## No-Secret Validation / Sensitive-Material Handling Decision

The NA-0369 operator action packet JSON parses successfully. The NA-0371
collection request JSON parses successfully and contains request markers only;
it does not contain real target endpoints, real host fingerprints, real
credentials, private key material, passphrases, tokens, recovery-envelope
contents, connection commands, backup commands, or restore commands.

Since no response candidate was found, no response content was copied, quoted,
parsed, or stored.

Sensitive-material stop behavior remains:

- If a future candidate includes private keys, passphrases, passwords, tokens,
  raw credentials, recovery-envelope contents, private material paths,
  secret-bearing command output, unredacted sensitive `known_hosts` content,
  live connection output, or host fingerprint material supplied outside an
  authorized storage boundary, intake must stop.
- If a future candidate includes a host fingerprint in a safe authorized
  no-secret context, it can only be classified as
  `HOST_IDENTITY_VALUE_PRESENT_UNVERIFIED`. NA-0372 performs no live
  verification.
- Sensitive content must not be quoted, copied, moved, or stored in
  qsl-protocol evidence.

## Response Field Matrix

| Field | Status | Raw value stored? | Redacted summary | Blocker | Next action | Future authorization required | Can precede remote connection? | Can precede credential handling? | Can precede real backup? | Drives NA-0373? |
|---|---|---:|---|---|---|---|---|---|---|---|
| target label | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for collection only | Yes, as non-secret response | Yes, as non-secret response | No | Yes |
| target class | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for collection only | Yes, as non-secret response | Yes, as non-secret response | No | Yes |
| target owner/contact label | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for collection only | Yes, as non-secret response | Yes, as non-secret response | No | Yes |
| trust boundary | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for collection only | Yes, as non-secret response | Yes, as non-secret response | No | Yes |
| host identity evidence source | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for collection only | Yes, as response evidence only | Yes, as response evidence only | No | Yes |
| fingerprint algorithm | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | May require future storage boundary | Yes, as response evidence only | Yes, as response evidence only | No | Yes |
| fingerprint format | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | May require future storage boundary | Yes, as response evidence only | Yes, as response evidence only | No | Yes |
| fingerprint value | `ABSENT` | No | Not supplied; request keeps real value future-authorized | Future authorization and response unavailable | Do not collect until authorized | Yes | No live verification | No credential use | No | No |
| credential model class | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for class only | Yes | Yes, as planning input | No | Yes |
| credential storage boundary | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for boundary class only | Yes | Yes, as planning input | No | Yes |
| capacity estimate | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for class only | Yes | Yes | No | Yes |
| retention intent | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for class only | Yes | Yes | No | Yes |
| monitoring destination class | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for class only | Yes | Yes | No | Yes |
| operator runbook owner | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for label only | Yes | Yes | No | Yes |
| emergency stop contact | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No, for label only | Yes | Yes | No | Yes |
| public-claim boundary acknowledgement | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No | Yes | Yes | No | Yes |
| no-secret affirmation | `ABSENT` | No | No response candidate | Operator response unavailable | Follow up collection | No | Yes | Yes | No | Yes |

Completeness result:

- Target candidate completeness: `INCOMPLETE`.
- Host identity completeness: `INCOMPLETE`; no value is present and no live
  verification occurred.
- Credential boundary completeness: `INCOMPLETE`; placeholder-only and
  secret-handling remains forbidden.
- Capacity/retention completeness: `INCOMPLETE`.
- Monitoring/runbook completeness: `INCOMPLETE`.
- Public-claim acknowledgement completeness: `INCOMPLETE`.
- No-secret affirmation completeness: `INCOMPLETE`.

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

The current qsl-protocol evidence patch requires no backup-plan update because
the changed paths stay inside qsl-protocol governance/testplan/journal files.
No operator response is stored in this PR.

Local-ops/history-index work remains valuable because the current local backup
status covers qbuild work and Codex responses, but request, directive, journal,
and broader ops history coverage is incomplete. That work should precede real
target connection, credential handling, repository creation, key
custody/recovery implementation, monitoring setup, backup execution, restore
execution, deployment, rollback, and durable storage of any future response
outside existing backup scope.

Local-ops should not displace NA-0373 in this lane because the primary blocker
is still response availability: there is no deliberate no-secret operator
response to intake.

The workflow-support request remains relevant. The following future
improvements would materially reduce friction:

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

NA-0372 does not implement those workflow-support items.

## Public-Ingress/Timing/Traffic-Shape Boundary

Response intake is not public-ingress implementation, target setup, host
identity verification, credential handling, or backup execution. It provides no
new evidence about public internet readiness, attachment size confidentiality,
timing metadata, traffic shape, or all metadata.

qsl-server PR #56 remains bounded end-to-end harness evidence only.
qsl-attachments PR #37 remains service-local prerequisite evidence only.
qshield embedded relay/demo evidence remains reference/oracle evidence only.

## External-Review Sensitivity

External review remains incomplete. NA-0372 does not create, update, or claim
an external-review package. The public technical position paper remains
future-gated until response availability, target/host identity planning,
credential boundary planning, real off-host backup prerequisites, and public
claim boundaries have settled.

## Public Claim Boundary

NA-0372 preserves these boundaries:

- response intake is not target setup;
- response intake is not host identity verification;
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

| Area | Status | Evidence source | Blocker | Next action | Extra collection needed? | Credential plan needed? | Target/host validation can proceed? | Must precede remote connection? | Must precede credential handling? | Must precede real backup? | NA-0373 candidate? |
|---|---|---|---|---|---|---|---|---|---|---|---|
| operator response existence | `OPERATOR_RESPONSE_STILL_ABSENT` | Authorized discovery | No response candidate | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| target candidate | `TARGET_CANDIDATE_VALUE_ABSENT` | No response candidate | No target value | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| host identity | `HOST_IDENTITY_VALUE_ABSENT` | No response candidate | No response evidence | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| credential boundary | `CREDENTIAL_PLACEHOLDER_ONLY` | NA-0371 request | Secret handling forbidden | Future boundary plan after response | Yes | Yes | No | Yes | Yes | Yes | Later |
| capacity/quota | `CAPACITY_RETENTION_VALUE_ABSENT` | No response candidate | No value | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| retention/purge | `CAPACITY_RETENTION_VALUE_ABSENT` | No response candidate | No value | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| monitoring/alerting | `MONITORING_RUNBOOK_VALUE_ABSENT` | No response candidate | No value | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| operator runbook | `MONITORING_RUNBOOK_VALUE_ABSENT` | No response candidate | No value | Collection follow-up | Yes | No | No | Yes | Yes | Yes | Yes |
| local-ops/backup-plan | `LOCAL_OPS_NOT_PRIMARY_BLOCKER` | Backup/status evidence | Coverage/index gaps | Future local-ops plan | No | No | No real operations | Yes, before real operations | Yes, before real operations | Yes | Later |
| external review/public claims | `NOT_READY` | Boundary evidence | Evidence gaps remain | Future audit/paper plan | No | No | No | Yes | Yes | Yes | Later |

Decision categories:

- `OPERATOR_RESPONSE_STILL_ABSENT`
- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. Restore the exact NA-0373 response availability blocker / collection
   follow-up plan.
2. Obtain a deliberate no-secret operator response in an authorized request
   path, or record why it remains unavailable.
3. If a response becomes available, validate no-secret boundaries before
   copying or summarizing any detail.
4. Only after a materially complete no-secret response, plan target/host
   identity validation without live verification claims.
5. Only after target/host identity validation planning, authorize credential
   boundary planning without handling secrets.
6. Only after credential, local-ops/backup coverage, tool, repository, key
   custody/recovery, monitoring, and restore prerequisites are explicitly
   authorized may real operations be considered.

## Future Validation/Marker/Verification Plan

For the selected NA-0373 successor, require:

- `NA0373_OPERATOR_RESPONSE_AVAILABILITY_BLOCKER_OK`
- `NA0373_COLLECTION_REQUEST_REFERENCE_OK`
- `NA0373_OPERATOR_RESPONSE_STILL_REQUIRED_OK`
- `NA0373_TARGET_CANDIDATE_RESPONSE_REQUIRED_OK`
- `NA0373_HOST_IDENTITY_RESPONSE_REQUIRED_OK`
- `NA0373_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0373_NO_REMOTE_CONNECTION_OK`
- `NA0373_NO_HOST_KEY_SCAN_OK`
- `NA0373_NO_SECRET_MATERIAL_OK`
- `NA0373_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0373_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0373_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0373_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0373_NO_METADATA_FREE_CLAIM_OK`
- `NA0373_NO_ANONYMITY_CLAIM_OK`
- `NA0373_NO_UNTRACEABLE_CLAIM_OK`

## Workflow-Support and History-Index Future Work Note

Read-only history paths for responses and requests were present and improved
handoff confidence. Directive and journal history paths were absent or had no
files at inspection. A future local-ops maintenance directive should create or
update an index mapping directive IDs, NA IDs, response paths, PRs, decisions,
selected successors, and merge SHAs.

## Selected Successor

Selected successor:

`NA-0373 -- Metadata Runtime Off-Host Backup Operator Response Availability Blocker / Collection Follow-Up Plan`

Rationale: no deliberate no-secret operator response exists after the NA-0371
collection request. The narrowest truthful next lane is to resolve response
availability and collection follow-up before response completion, validation,
credential-boundary, tool-install, local-ops, external-review, website/public
claim, or paper-draft work.

## Rejected Alternatives

- `NA-0373 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Completion Request`: rejected because there is no partial response to complete.
- `NA-0373 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Response Validation Plan`: rejected because there is no response to validate.
- `NA-0373 -- Metadata Runtime Off-Host Backup Credential Boundary / Secret Handling Prerequisite Plan`: rejected as premature before non-secret target/host response availability.
- `NA-0373 -- Metadata Runtime Restic / Backup Tool Installation Authorization Plan`: rejected as premature and real operations remain blocked.
- `NA-0373 -- Metadata Runtime Real Key Custody / Key Recovery Implementation Authorization Plan`: rejected as premature and secret/key handling remains forbidden.
- `NA-0373 -- QSL Local Ops Codex Workflow Support and History Index Plan`: rejected as useful but not the primary blocker for this lane.
- `NA-0373 -- Metadata Runtime External Review Readiness Gap Audit`: rejected as premature before off-host backup evidence gaps are resolved.
- `NA-0373 -- Metadata Runtime Website / Public Claim Boundary Audit`: rejected as not the current blocker; no public docs are changed.
- `NA-0373 -- Public Technical Position Paper Evidence-Bounded Draft Plan`: rejected as premature before evidence gaps settle.

## Backup-Plan Impact Statement

No NA-0372 backup-plan update is required. This PR changes only qsl-protocol
governance, traceability, testplan, and rolling-journal files and stores no
real operator response.

Future operator response storage outside the current backed-up scope, real
target connection, host identity capture, credential handling, repository/tool
setup, real key custody/recovery, monitoring, local-ops/history-index work,
backup, restore, deploy, rollback, and public-claim mutation remain
backup-plan and exact-authorization gated.

## Next Recommendation

Close NA-0372 after this evidence merges and restore:

`NA-0373 -- Metadata Runtime Off-Host Backup Operator Response Availability Blocker / Collection Follow-Up Plan`

NA-0373 must not implement target setup, remote connection, host-key scan,
credential handling, secret handling, repository init, tool installation,
backup, restore, deploy, rollback, real restore target creation/mount/copy, key
generation, key upload, passphrase collection, private-key inspection,
recovery-envelope content creation, local backup script/timer/fstab mutation,
or public-claim expansion unless future exact scope authorizes it.
