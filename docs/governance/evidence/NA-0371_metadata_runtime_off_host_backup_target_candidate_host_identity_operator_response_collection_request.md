Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0371 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0371 creates a no-secret operator response collection request after NA-0370
recorded `OPERATOR_RESPONSE_NOT_PRESENT` for the NA-0369 no-secret operator
action packet.

Implemented request artifact:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`

Recommended future operator response location:

- `/home/victor/work/qsl/codex/requests/NA0371_operator_response_<date>.md`
- `/home/victor/work/qsl/codex/requests/NA0371_operator_response_<date>.json`

The request asks only for non-secret response fields. It explicitly forbids
private keys, passphrases, passwords, tokens, raw credentials, recovery
envelope contents, secret/private material paths, secret-bearing command
output, live connection output, sensitive `known_hosts` content, and real host
fingerprints unless a future directive authorizes how to collect and store
them.

Classifications:

- `COLLECTION_REQUEST_FIELD_DESIGN_OK`
- `OPERATOR_RESPONSE_COLLECTION_REQUIRED`
- `NO_SECRET_COLLECTION_REQUEST_OK`
- `NO_REMOTE_CONNECTION_COLLECTION_REQUEST_OK`
- `NO_HOST_KEY_SCAN_COLLECTION_REQUEST_OK`
- `COLLECTION_REQUEST_CREATED`
- `OPERATOR_RESPONSE_STILL_REQUIRED`
- `TARGET_CANDIDATE_RESPONSE_REQUIRED`
- `HOST_IDENTITY_RESPONSE_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

Selected successor:

`NA-0372 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request`

NA-0371 does not set up an off-host target, connect to a remote, scan host
keys, mutate `known_hosts`, handle credentials or secrets, initialize a
repository, install tools, run backup or restore operations, create a restore
target, generate keys, upload keys, collect passphrases, inspect private keys,
create recovery envelope content, deploy, roll back, mutate qsl-server or
qsl-attachments, change qshield runtime, change qsc/qsp/protocol/crypto
behavior, change dependencies, change workflows, change public docs, mutate
local backup scripts/timers/fstab, or expand public claims.

## Live NA-0371 Scope

Live `NEXT_ACTIONS.md` records `NA-0371 -- Metadata Runtime Off-Host Backup
Target Candidate / Host Identity Operator Response Collection Request` as the
sole READY item. The live scope authorizes a qsl-protocol governance/request
lane to request and collect a deliberate no-secret operator response to the
NA-0369 target-candidate / host-identity operator action packet.

The live scope requires:

- no unsupported production, public-internet, external-review, or anonymity
  claims;
- no metadata-free or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- qsl-server/qsl-attachments production boundaries remain explicit;
- real off-host setup, real key handling, real restore drills, backup, restore,
  deploy, rollback, and backup-plan updates remain explicitly authorized
  before execution;
- no target setup, remote connection, host-key scan, `known_hosts` mutation,
  credential handling, secret handling, repository init, tool installation,
  backup, restore, deploy, rollback, real restore target creation/mount/copy,
  key generation, key upload, passphrase collection, private-key inspection,
  recovery envelope content creation, backup-script/timer/fstab mutation,
  qsl-server mutation, qsl-attachments mutation, qshield runtime mutation,
  protocol/crypto/qsc/qsp implementation change, dependency change, workflow
  change, website/public-doc change, README change, START_HERE change, or
  public-claim expansion.

The live scope and this directive align. The optional no-secret request JSON
artifact is permitted because the directive explicitly authorizes the
`inputs/metadata_runtime/` request-artifact path.

## Inherited NA-0370 Response Intake Result

NA-0370 searched only authorized local read-only locations and found no
operator response candidate for the NA-0369 operator action packet.

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

NA-0370 selected NA-0371 because the narrowest truthful next step was to
produce a collection request that tells the operator exactly what non-secret
information to provide and what never to provide.

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
- Neither repository is mutated by NA-0371.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh

Read-only local evidence:

- `/backup/qsl` is mounted as local same-host continuity storage.
- `/backup/qsl` has about 916G total and about 886G available.
- `/srv/qbuild` has about 468G total and about 373G available.
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
  `/home/victor/work/qsl/codex/journals` were absent or empty at inspection.
- The backup status source list covers qbuild work and Codex responses, but
  not all request/journal/history folders as durable local-ops evidence.
- The D132 preservation bundle remains present under the expected local qbuild
  temporary preservation path.

Classifications:

- `LOCAL_CONTINUITY_PROVEN`
- `OPERATOR_PACKET_EXISTS`
- `OPERATOR_RESPONSE_NOT_PRESENT`
- `COLLECTION_REQUEST_READY`
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

## Collection Request Field Design

The collection request has these sections:

1. Metadata and purpose.
2. Recommended future response location.
3. Target candidate non-secret fields.
4. Host identity non-secret fields.
5. Credential placeholder fields.
6. Capacity/quota/retention fields.
7. Monitoring/alerting/runbook fields.
8. Public-claim acknowledgement fields.
9. Forbidden inputs.
10. Stop conditions.
11. Boundary statements.
12. Future validation markers.
13. Successor recommendation.

Request classifications:

- `COLLECTION_REQUEST_FIELD_DESIGN_OK`
- `OPERATOR_RESPONSE_COLLECTION_REQUIRED`
- `NO_SECRET_COLLECTION_REQUEST_OK`
- `NO_REMOTE_CONNECTION_COLLECTION_REQUEST_OK`
- `NO_HOST_KEY_SCAN_COLLECTION_REQUEST_OK`

## No-Secret Collection Request Artifact Implementation

Implemented artifact:

`inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`

The artifact is deterministic JSON and records request fields only. It contains
no operator response values, no real endpoint, no real host identity value, no
real credential, no private key material, no passphrase, no token, no password,
no recovery envelope content, no host-key scan output, no connection command,
no executable command, no backup command, and no restore command.

All requested values are marked with:

- `REQUEST_OPERATOR_NON_SECRET_INPUT`
- `DO_NOT_INCLUDE_SECRET`
- `REQUIRED_BEFORE_CONNECTION`
- `FUTURE_AUTHORIZATION_REQUIRED`
- `NO_CONNECTION_IN_THIS_REQUEST`
- `NO_HOST_KEY_SCAN_IN_THIS_REQUEST`
- `NO_BACKUP_OR_RESTORE_IN_THIS_REQUEST`

## Operator Instructions / Forbidden-Input Guide

The operator should provide only:

- non-secret target label or alias;
- target class confirmation;
- non-secret owner/contact label;
- high-level location or jurisdiction class if safe to record;
- non-secret host identity evidence source description;
- fingerprint format or algorithm class if safe, without providing a real
  fingerprint unless a future directive authorizes collection and storage;
- capacity/quota/retention estimates;
- monitoring destination class, not credentials;
- emergency contact label;
- operator runbook owner label;
- public-claim boundary acknowledgement.

The operator must never provide:

- private keys;
- passphrases;
- passwords;
- tokens;
- raw credentials;
- recovery envelope contents;
- secret paths;
- private material paths;
- screenshots or command output containing secrets;
- live connection outputs;
- unredacted `known_hosts` content if sensitive;
- real host fingerprints unless a future directive explicitly authorizes how
  they are collected and stored.

If sensitive material appears in a future response candidate, intake must stop
without quoting or copying the sensitive content.

## Request Validation / JSON Parse / No-Secret Checks

Validation expectations:

- request JSON parses with `python3 -m json.tool`;
- required request sections exist;
- forbidden-input guide exists;
- request markers exist;
- no real target endpoint exists;
- no real host identity value exists;
- no credential value exists;
- no private key material exists;
- no recovery envelope content exists;
- no connection, backup, restore, deploy, rollback, repository-init, or
  tool-install command exists;
- no public-claim overreach exists.

## Backup-Plan / Local-Ops / History-Index Dependency Analysis

Classifications:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

The collection request itself does not require a backup-plan update because it
changes only qsl-protocol governance/testplan paths and one no-secret request
artifact under `inputs/metadata_runtime`.

Future operator response storage, target connection, host identity capture,
credential handling, repository creation, key custody/recovery, monitoring,
local-ops/history-index maintenance, backup, restore, deploy, rollback, and
public-claim mutation remain backup-plan and exact-authorization gated.

Local-ops/history-index work would materially reduce friction, especially for
qstart/qresume fast-forwarding, response-file writing, bounded CI polling,
machine-readable directive manifests, validation profiles, per-directive
allow-files, read-only source/authority refresh, claim-boundary scanning,
directive/response/journal indexing, and backup coverage for directives,
requests, journals, and ops history. It is not the primary NA-0372 successor
because the immediate blocker is still the absent operator response after the
collection request.

## Public-Ingress / Timing / Traffic-Shape Boundary

This request is not target setup, not operator response intake, not host
identity verification, not a remote connection, not a host-key scan, not
credential handling, not backup, not restore, not deployment, and not rollback.

The qshield embedded relay/demo evidence remains reference/oracle evidence
only. It is not qsl-server/qsl-attachments production proof.

The no-secret off-host target/tool harness is not real off-host backup. The
no-secret isolated restore harness is not real restore. The no-secret key
custody/recovery harness is not real key custody or real key recovery.

## External-Review Sensitivity

External review remains incomplete. NA-0371 does not add external-review
evidence, reviewer acceptance, public-internet service proof, production
service proof, or release-readiness evidence.

## Public Claim Boundary

No public claim may state or imply:

- production readiness;
- public-internet readiness;
- external-review completion;
- anonymity;
- metadata-free behavior;
- untraceability;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- all metadata hidden;
- complete off-host backup;
- complete disaster recovery;
- real restore completion;
- verified host identity;
- configured target;
- real key custody implementation;
- real key recovery implementation.

No website/public-doc update is needed for NA-0371. Public technical position
paper work remains future-gated.

## Decision Matrix

| Area | Status | Requested operator field | Forbidden data | Blocker | Next action | Drives NA-0372 |
|---|---|---|---|---|---|---|
| Collection request | `COLLECTION_REQUEST_CREATED` | none; request artifact exists | secrets and commands | operator response absent | intake response after request | yes |
| Response location | guidance exists | response file path chosen by operator | secret-bearing files | no response file yet | operator creates no-secret response later | yes |
| Target candidate | absent | target label/class/owner/location/trust/capacity | endpoint secrets, credentials, command output | operator response absent | request response | yes |
| Host identity | absent | source description, algorithm/format class, verifier label | real fingerprint unless future authorized | operator response absent | request response | yes |
| Credential boundary | absent | placeholder model/storage class | private keys, passphrases, passwords, tokens, raw credentials | secret handling forbidden | future credential plan after response intake | no |
| Capacity/quota | absent | size/growth/cost/free-space classes | provider secrets or billing credentials | operator response absent | request response | yes |
| Retention/purge | absent | retention window/count/prune model | destructive commands | operator response absent | request response | yes |
| Monitoring/alerting | absent | destination class and alert classes | webhook secrets or tokens | operator response absent | request response | yes |
| Operator runbook | absent | owner/escalation/emergency stop labels | secret runbook paths | operator response absent | request response | yes |
| Local-ops/backup-plan | partial | none for this request | backup script/timer/fstab mutation | history index and coverage gaps | future local-ops lane before real ops | no |
| External review/public claims | incomplete | acknowledgement only | readiness/metadata overclaims | no review complete | keep future-gated | no |

## Future Staged Implementation Strategy

1. NA-0372 intakes the operator response created from this request, or records
   continued absence without inferring target evidence.
2. If non-secret response fields are present and safe, a later lane can plan
   validation of target candidate and host identity without connecting.
3. Credential boundary / secret handling planning must precede any credential
   use.
4. Tool installation authorization must precede any restic/borg/rclone/age
   installation or use.
5. Real key custody/recovery authorization must precede any key material,
   passphrase, private key, or recovery envelope handling.
6. Local-ops/history-index and backup-plan coverage should be authorized before
   real operations store durable operational evidence outside currently covered
   locations.
7. Remote connection, host-key scanning, repository init, backup, restore,
   deploy, and rollback remain future exact-authorization work only.

## Future Validation / Marker / Verification Plan

For the selected NA-0372 successor, use:

- `NA0372_COLLECTION_REQUEST_CREATED_OK`
- `NA0372_OPERATOR_RESPONSE_INTAKE_AFTER_REQUEST_OK`
- `NA0372_TARGET_CANDIDATE_RESPONSE_PRESENT_OR_BLOCKED_OK`
- `NA0372_HOST_IDENTITY_RESPONSE_PRESENT_OR_BLOCKED_OK`
- `NA0372_CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NA0372_NO_REMOTE_CONNECTION_OK`
- `NA0372_NO_HOST_KEY_SCAN_OK`
- `NA0372_NO_SECRET_MATERIAL_OK`
- `NA0372_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0372_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0372_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0372_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0372_NO_METADATA_FREE_CLAIM_OK`
- `NA0372_NO_ANONYMITY_CLAIM_OK`
- `NA0372_NO_UNTRACEABLE_CLAIM_OK`

## Selected Successor

Selected successor:

`NA-0372 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request`

Rationale: the collection request now exists, but no operator response value is
present. Intake after the request is the narrowest next lane before validation,
credential planning, tool installation authorization, local-ops, external
review, website/public-claim audit, or public technical position paper work.

## Rejected Alternatives

- Credential boundary / secret handling prerequisite plan: rejected for NA-0372
  because there is still no operator response identifying the target/host
  context to plan around.
- Restic / backup tool installation authorization plan: rejected because target
  candidate and host identity response remain absent.
- Real key custody / key recovery authorization plan: rejected because real
  secret handling remains future-gated and target response is absent.
- Local ops workflow support and history index plan: useful future work, but
  not the primary blocker before response intake.
- External review readiness gap audit: future-gated until prerequisite runtime
  and operations evidence is stronger.
- Website / public claim boundary audit: not the immediate blocker; no public
  claims are changed here.
- Public technical position paper draft plan: future-gated until evidence gaps
  and public-claim boundaries are more settled.
- Direct target setup, host-key scan, credential handling, repository init,
  backup, restore, deploy, or rollback: forbidden by scope.

## Backup-Plan Impact Statement

No NA-0371 backup-plan update is required. Changed paths are limited to
qsl-protocol governance/testplan/journal files and the no-secret request
artifact under `inputs/metadata_runtime`.

Future real operator response storage, target connection, host identity
capture, credential handling, repository/tool setup, key custody/recovery,
monitoring, local-ops/history-index, backup, restore, deploy, rollback, or
public-claim mutation remains backup-plan and exact-authorization gated.

## Workflow-Support and History-Index Future Work Note

Read-only history paths were partially present: `responses` and `requests`
were present, while directive and journal history were absent or empty at
inspection. History improved confidence for prior response identity and local
workflow friction, but a future maintenance directive should create/update an
index that maps directive IDs, NA IDs, PRs, decisions, response files,
successors, and merge SHAs.

## Next Recommendation

After NA-0371 merges and closes out, restore:

`NA-0372 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request`

NA-0372 must not be implemented by NA-0371.
