Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0370 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0370 intakes the state of any operator response to the NA-0369 no-secret
target-candidate / host-identity operator action packet. The response discovery
searched only the authorized local read-only locations and found no operator
response candidate.

Classification:

- `NO_RESPONSE_CANDIDATE_FOUND`
- `OPERATOR_RESPONSE_NOT_PRESENT`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

Selected successor:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

NA-0370 is intake/governance evidence only. It does not set up an off-host
target, connect to a remote host, scan host keys, mutate `known_hosts`, handle
credentials or secrets, initialize a repository, install tools, run backup or
restore operations, create a restore target, generate keys, upload keys,
collect passphrases, inspect private keys, create recovery-envelope content,
deploy, roll back, mutate qsl-server or qsl-attachments, change qshield runtime,
change qsc/qsp/protocol/crypto behavior, change dependencies, change workflows,
change public docs, mutate local backup scripts/timers/fstab, or expand public
claims.

## Live NA-0370 Scope

Live `NEXT_ACTIONS.md` records `NA-0370 -- Metadata Runtime Off-Host Backup
Target Candidate / Host Identity Operator Response Intake` as the sole READY
item. The live scope authorizes intake or evaluation of the operator response
to the NA-0369 no-secret operator action packet while preserving
no-secret/no-connection/no-host-key-scan boundaries.

The live scope requires protection for:

- no unsupported production, public-internet, external-review, or anonymity
  claims;
- no metadata-free or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- no target setup, remote connection, host-key scan, credential handling,
  secret handling, backup, restore, deploy, rollback, or repository init unless
  future exact scope authorizes it;
- explicit qsl-server/qsl-attachments production boundaries;
- the NA-0369 operator packet remaining no-secret template evidence until a
  future response is provided and validated.

Live scope and this directive align. The optional intake status JSON artifact
was not added because the live queue entry does not explicitly authorize an
`inputs/` response-status artifact.

## Inherited NA-0369 Operator Action Packet

NA-0369 created the no-secret operator action packet:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json`

The packet parses as JSON and contains placeholder-only sections for target
candidate, host identity, credential placeholder, capacity/quota/retention,
monitoring/alerting, runbook, forbidden inputs, stop conditions, public-claim
boundaries, qsl-server/qsl-attachments/qshield boundaries, backup-plan impact,
and future validation markers.

NA-0369 did not receive, validate, store, or verify real operator response
values. The packet is not target setup, not a configured target, not verified
host identity, not off-host backup completion, not real restore completion, and
not disaster recovery completion.

## Inherited NA-0368 Operator Prerequisite Plan

NA-0368 defined the no-connection operator prerequisite field plan for future
target-candidate and host-identity work. It kept the target class at the
NA-0355 SSH/SFTP-compatible off-host host class with a restic-style encrypted
snapshot repository class. It recorded that real target candidate, host
identity, credential boundary, capacity/quota/retention, monitoring/alerting,
and runbook evidence remained absent or future-authorized.

## Inherited NA-0367 Target-Access Prerequisite

NA-0367 selected target access and host identity as the first real blockers
after target/tool class selection. It classified target candidate and host
identity as absent, credential boundary as absent, host identity capture as
future-authorization required, and `known_hosts` mutation as forbidden.

## Inherited NA-0366 Blocker-Resolution Result

NA-0366 recorded `REAL_TARGET_TOOL_IMPLEMENTATION_BLOCKED`. The blocker result
found no real SSH/SFTP target, no host identity, no credential boundary, no
capacity or retention evidence, no monitoring boundary, no installed
restic/borg/rclone/age tool, no repository-init boundary, no real key custody,
no real key recovery, no recovery-envelope authorization, and no real restore
execution.

## Inherited No-Secret Harness Evidence

NA-0365 added a qsl-protocol no-secret isolated restore fixture and harness. It
is simulated isolated-restore evidence only and is not a real restore drill.

NA-0363 added a qsl-protocol no-secret off-host target/tool fixture and harness.
It is simulated target/tool evidence only and is not a real target, configured
repository, tool installation, remote connection, backup, or restore.

NA-0361 added a qsl-protocol no-secret key custody / key recovery fixture and
harness. It is simulated custody/recovery evidence only and is not real key
custody or real key recovery.

NA-0359 added a qsl-protocol no-secret restore-drill dry-run fixture and
harness. It is dry-run evidence only and is not real restore execution.

## Source/Authority/CI Refresh for qsl-server and qsl-attachments

| Repository | Source | Authority | CI | Classification |
|---|---|---|---|---|
| qsl-server | Local read-only worktree at `d40e6003fdf`; remote default branch and PR #56 merge at `d40e6003fdf` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins enforced | Latest listed main `ci` run succeeded on `d40e6003fdf`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |
| qsl-attachments | Local read-only worktree at `96b9352bd63`; remote default branch and PR #37 merge at `96b9352bd63` | Viewer permission `ADMIN`; branch protection requires `rust`; strict checks enabled; force pushes disabled; deletions disabled; admins not enforced in current protection evidence | Latest listed main `rust` run succeeded on `96b9352bd63`; no open PRs | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

Boundaries:

- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Neither repository was mutated by NA-0370.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- The same-host local continuity backup target is mounted and has daily
  snapshots, manifests, and logs through `daily-20260527T023818-0500`.
- The qbuild disk watermark is healthy for this governance lane.
- The local backup script syntax check, preflight, list, and timer status were
  read-only and successful.
- `restic`, `borg`, `rclone`, and `age` are absent.
- `gpg`, `ssh`, and `rsync` are present.
- Backup plan and backup status files are present.
- Response and request history folders are present.
- Directive and journal history folders are absent at inspection.
- The D132 preservation bundle remains present.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_PROVEN`
- `OPERATOR_PACKET_EXISTS`
- `OPERATOR_RESPONSE_NOT_PRESENT`
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

## Operator Response Discovery

Authorized response discovery locations:

- the local Codex requests directory, filename-matched for NA-0369/NA-0370,
  operator response, target candidate, and host identity response terms;
- qsl-protocol `inputs/metadata_runtime/`, filename-matched for operator
  response, target-candidate response, and host-identity response terms.

Result:

- No candidate response file was found in the authorized requests directory.
- No qsl-protocol metadata input file matched the response filename patterns.
- A broader operator filename listing found only the NA-0369 action packet
  template.
- No candidate file content was parsed because no candidate existed.
- Exact candidate files: none.

Classification:

- `NO_RESPONSE_CANDIDATE_FOUND`
- `OPERATOR_RESPONSE_NOT_PRESENT`

## Operator Response Intake Classification

Because no operator response candidate was present, NA-0370 records fail-closed
absence rather than inferring target evidence.

| Intake item | Classification | Evidence | Next action |
|---|---|---|---|
| Operator response | `OPERATOR_RESPONSE_NOT_PRESENT` | No authorized candidate file found | Request no-secret operator response |
| Target candidate | `TARGET_CANDIDATE_VALUE_ABSENT` | No response value | Required operator input |
| Host identity | `HOST_IDENTITY_VALUE_ABSENT` | No response value | Required operator input; future validation remains separate |
| Credential response | `CREDENTIAL_PLACEHOLDER_ONLY` and `CREDENTIAL_VALUE_FORBIDDEN` | NA-0369 packet forbids secret material | Credential boundary remains future-authorized |
| Capacity/retention | `CAPACITY_RETENTION_VALUE_ABSENT` | No response value | Required operator input |
| Monitoring/runbook | `MONITORING_RUNBOOK_VALUE_ABSENT` | No response value | Required operator input |

## No-Secret Validation / Sensitive-Material Handling Decision

The NA-0369 operator action packet JSON parses successfully and remains a
no-secret template. Since no response candidate was found, no response content
was copied, quoted, parsed, or stored.

Sensitive-material stop behavior remains:

- If a future candidate includes private keys, passphrases, passwords, tokens,
  raw credentials, recovery-envelope contents, private material paths,
  secret-bearing command output, live connection output, or sensitive
  unredacted host-identity material, intake must stop without copying or
  quoting the sensitive content.
- If a future response includes a host fingerprint, it must be classified as
  unverified response evidence unless future exact scope authorizes collection,
  storage, and verification.

Classification:

- `NO_SECRET_VALIDATION_OK`
- `SENSITIVE_MATERIAL_STOP_PATH_DEFINED`
- `NO_RAW_SECRET_COPIED`

## Response Field Matrix

| Field | Status | Raw value stored | Redacted summary | Blocker | Next action | Future authorization required | Before remote connection | Before credential handling | Before real backup | Drives NA-0371 |
|---|---|---:|---|---|---|---|---|---|---|---|
| target label | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| target class | present from packet only | no | SSH/SFTP-compatible class | no real target | request confirmation | no | yes | yes | yes | yes |
| target owner/contact label | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| trust boundary | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| host identity evidence source | absent | no | no operator value | response missing | request value | yes for verification | yes | yes | yes | yes |
| fingerprint algorithm | absent | no | no operator value | response missing | request value | yes for verification | yes | yes | yes | yes |
| fingerprint format | absent | no | no operator value | response missing | request value | yes for verification | yes | yes | yes | yes |
| fingerprint value | absent | no | no operator value | response missing | request value or authorized withholding | yes for storage/verification | yes | yes | yes | yes |
| credential model class | future-authorization-required | no | placeholder only | credential boundary absent | plan separately after response request | yes | yes | yes | yes | no |
| credential storage boundary | future-authorization-required | no | placeholder only | credential boundary absent | plan separately after response request | yes | yes | yes | yes | no |
| capacity estimate | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| retention intent | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| monitoring destination class | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| operator runbook owner | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| emergency stop contact | absent | no | no operator value | response missing | request value | no | yes | yes | yes | yes |
| public-claim boundary acknowledgement | absent | no | no operator value | response missing | request acknowledgement | no | yes | yes | yes | yes |
| no-secret affirmation | absent | no | no operator value | response missing | request affirmation | no | yes | yes | yes | yes |

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

Local-ops work would reduce friction, especially a directive/response/journal
index, response-file writer, bounded polling helper, validation profiles,
per-directive allow-files, read-only source/authority helper, claim-boundary
scanner, and backup coverage for directive/request/journal/ops history. It is
not the primary NA-0371 blocker because no operator response exists. Local-ops
must be addressed before real operations that create durable local history,
secret-handling artifacts, target state, repository state, or monitoring state.

## Public-Ingress/Timing/Traffic-Shape Boundary

NA-0370 changes no public ingress, runtime timing behavior, traffic-shape
behavior, padding behavior, service behavior, website, or public docs. It does
not alter qsl-server, qsl-attachments, qshield runtime, qsc/qsp, protocol,
crypto, key schedules, deployment, rollback, backup scripts, timers, fstab, or
local backup configuration.

## External-Review Sensitivity

External review remains incomplete. Operator response intake is not external
review, not public-internet proof, not production proof, and not service proof.
qsl-server PR #56 remains bounded end-to-end harness evidence only.
qsl-attachments PR #37 remains service-local prerequisite evidence only.
qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Public Claim Boundary

NA-0370 introduces:

- no claim of production readiness;
- no claim of public-internet readiness;
- no claim of external-review completion;
- no claim of anonymity;
- no claim of metadata-free behavior;
- no claim of untraceability;
- no claim of hidden attachment size;
- no claim of hidden timing metadata;
- no claim of hidden traffic shape;
- no claim of hidden all metadata;
- no claim of complete off-host backup;
- no claim of complete disaster recovery;
- no claim of target configuration;
- no claim of verified host identity;
- no claim of real restore completion;
- no claim of real key custody;
- no claim of real key recovery.

Any stronger future claim requires exact evidence for real target availability,
host-identity verification, credential boundary, key custody, key recovery,
off-host backup execution, real restore drill execution, service behavior,
deployment, monitoring/logging, rollback, and external review.

## Decision Matrix

| Dimension | Status | Evidence source | Blocker | Next action | Collection needed | Credential boundary needed | Target/host validation can proceed | Must precede remote connection | Must precede credential handling | Must precede real backup | Ready for implementation authorization | Should become NA-0371 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| operator response existence | `OPERATOR_RESPONSE_NOT_PRESENT` | authorized discovery | no response | collection request | yes | later | no | yes | yes | yes | no | yes |
| target candidate value | absent | no response | missing target data | request value | yes | later | no | yes | yes | yes | no | yes |
| host identity value | absent | no response | missing host identity data | request value | yes | later | no | yes | yes | yes | no | yes |
| credential boundary | absent/future-authorized | NA-0369 packet | secret handling forbidden | keep placeholder; plan later | no | yes | no | yes | yes | yes | no | no |
| capacity/quota | absent | no response | missing capacity data | request value | yes | no | no | yes | yes | yes | no | yes |
| retention/purge | absent | no response | missing retention data | request value | yes | no | no | yes | yes | yes | no | yes |
| monitoring/alerting | absent | no response | missing monitoring data | request value | yes | no | no | yes | yes | yes | no | yes |
| operator runbook | absent | no response | missing runbook data | request value | yes | no | no | yes | yes | yes | no | yes |
| local-ops/backup-plan | partial | backup plan/status and history paths | index/coverage gaps | authorize separately before real ops | no | no | no | before real ops | before real ops | yes | not now | no |
| external review/public claims | not complete/not expanded | governance evidence | evidence gaps | keep bounded | no | no | no | yes | yes | yes | no | no |

Decision categories:

- `OPERATOR_RESPONSE_NOT_PRESENT`
- `OPERATOR_RESPONSE_COLLECTION_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Future Staged Implementation Strategy

1. Collect a no-secret operator response that references the NA-0369 packet and
   supplies only non-secret field values or explicit withheld/required markers.
2. Validate the response structure in a separate lane if a response is present.
3. Plan credential boundary and secret-handling rules without accepting secret
   material into qsl-protocol.
4. Authorize tool installation/repository initialization only after target,
   host identity, credential boundary, capacity, retention, monitoring, and
   backup-plan prerequisites are satisfied.
5. Authorize key custody/recovery and real restore-drill lanes separately.
6. Audit website/public claims and external-review readiness only after real
   evidence exists or after an explicit bounded-not-ready audit lane.

## Future Validation / Marker / Verification Plan

For the selected collection-request successor, future evidence should include:

- `NA0371_OPERATOR_RESPONSE_COLLECTION_REQUEST_OK`
- `NA0371_OPERATOR_PACKET_REFERENCE_OK`
- `NA0371_TARGET_CANDIDATE_RESPONSE_REQUIRED_OK`
- `NA0371_HOST_IDENTITY_RESPONSE_REQUIRED_OK`
- `NA0371_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0371_NO_REMOTE_CONNECTION_OK`
- `NA0371_NO_HOST_KEY_SCAN_OK`
- `NA0371_NO_SECRET_MATERIAL_OK`
- `NA0371_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0371_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0371_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0371_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0371_NO_METADATA_FREE_CLAIM_OK`
- `NA0371_NO_ANONYMITY_CLAIM_OK`
- `NA0371_NO_UNTRACEABLE_CLAIM_OK`

## Workflow-Support and History-Index Future Work Note

The local workflow-support request remains relevant. The following future
local-ops improvements would materially reduce friction:

- qstart/qresume fast-forward to expected origin/main before handoff;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directive/request/journal/ops history folders.

NA-0370 does not implement those improvements. They can follow the operator
response collection request unless future evidence shows local-ops must precede
response collection.

## Selected Successor

Selected successor:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

Rationale: no operator response exists. Collection/request is the narrowest
truthful successor and must precede validation, credential boundary planning,
tool installation authorization, key custody/recovery authorization,
target/host validation, external-review readiness, website/public-claim audit,
and public technical position paper drafting.

## Rejected Alternatives

- Operator response validation plan: rejected because no response candidate is
  present.
- Credential boundary / secret-handling prerequisite plan: rejected as the next
  immediate successor because target/host response fields are still absent and
  no secret material may be collected.
- Restic / backup tool installation authorization plan: rejected because no
  target, host identity, credential boundary, capacity/retention, monitoring,
  repository boundary, or backup-plan gate is satisfied.
- Real key custody / key recovery implementation authorization plan: rejected
  because real target/tool and credential prerequisites remain absent.
- Local ops workflow support and history index plan: useful, but not the
  primary blocker to collecting a no-secret operator response.
- External review readiness gap audit: rejected because current blocker is the
  missing operator response.
- Website / public claim boundary audit: rejected because no public claim
  mutation is needed and current claims remain bounded.
- Public technical position paper plan: rejected until evidence gaps can be
  stated with less operational reconstruction.
- Direct remote target setup, host-key scan, credential handling, repository
  init, backup, or restore: rejected as out of scope and unsafe.

## Backup-Plan Impact Statement

No NA-0370 backup-plan update is required. Durable changes are limited to
qsl-protocol governance, testplan, traceability, and journal files under the
existing qbuild-backed repository lane. No real operator response is stored.

Future real target connection, host identity capture, credential handling,
repository/tool implementation, key custody/recovery implementation, real
isolated restore implementation, local-ops history coverage, monitoring state,
backup/restore/deploy/rollback operations, and public-claim mutation remain
backup-plan and local-ops authorization gated.

## Next Recommendation

Close NA-0370 after its evidence PR merges and public-safety remains green.
Restore exactly one READY successor:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`
