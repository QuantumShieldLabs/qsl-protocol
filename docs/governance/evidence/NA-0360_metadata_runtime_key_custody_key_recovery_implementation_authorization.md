Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0360 Metadata Runtime Key Custody Key Recovery Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0360 is a qsl-protocol governance and authorization lane only. It decides
whether future key custody and key recovery work can proceed safely after the
NA-0359 no-secret restore-drill dry-run harness.

Authorization result:

- `NO_SECRET_KEY_CUSTODY_HARNESS_AUTHORIZATION_READY`
- `NO_SECRET_KEY_RECOVERY_HARNESS_AUTHORIZATION_READY`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`

Selected successor:

`NA-0361 -- Metadata Runtime Key Custody / Key Recovery No-Secret Implementation Harness`

The future NA-0361 lane is limited to qsl-protocol no-secret fixture and harness
evidence. It may model simulated custody records, simulated recovery-envelope
metadata, simulated rotation matrices, incident-response markers, operator
runbook markers, backup-plan impact markers, fail-closed negative cases, and
claim-boundary markers. It must not create or upload real keys, collect
passphrases, inspect private keys, handle secret material, set up off-host
backup, run backup or restore tooling, mutate local backup configuration, or
claim real key custody or real key recovery.

NA-0360 itself performed no key generation, no key upload, no passphrase
collection, no private-key inspection, no secret handling, no off-host setup, no
backup, no restore, no restore target creation/mount/copy, no deploy, no
rollback, no local backup script/timer/fstab mutation, no qsl-server mutation,
no qsl-attachments mutation, no qshield runtime mutation, no qsc/qsp/protocol/
crypto mutation, no dependency change, no workflow change, no README or
START_HERE change, no docs/public change, no website change, and no public
claim expansion.

## Live NA-0360 scope

The live queue marks NA-0360 READY and requires a key custody / key recovery
implementation authorization plan selected by NA-0359. Its objective is to
decide whether key custody and key recovery are implementation-ready,
blocker-gated, or should defer to another exact successor before any secret
material, recovery envelope, off-host target, restore target, backup, restore,
deploy, rollback, or public-claim mutation occurs.

Allowed current mutation scope is limited to qsl-protocol governance evidence,
this testplan lane, D-0702, TRACEABILITY, and the rolling operations journal.
Optional non-runtime planning artifacts were not needed by the live scope.

Forbidden current scope includes qsl-server implementation, qsl-attachments
implementation, qshield runtime implementation, qsc/qsp/protocol/crypto
implementation, dependency or workflow mutation, website/public-doc mutation,
README or START_HERE mutation, backup script/timer/fstab mutation, off-host
setup, backup, restore, restore target creation/mount/copy, deploy, rollback,
real key generation, key upload, passphrase collection, private-key inspection,
and secret material handling.

Acceptance requires exactly one READY item, NA-0359 DONE, D-0700 and D-0701
present once, and no NA-0360 implementation or operational mutation.

## Inherited NA-0359 restore-drill dry-run harness

NA-0359 implemented the qsl-protocol no-secret restore-drill dry-run harness.
The harness validates deterministic manifest/checksum relationships, writes a
redacted proof artifact under `/srv/qbuild/tmp/NA-0359_*`, proves seven
fail-closed negative cases, emits all required NA0359 markers, and records
operation counts of zero for restore, restore target, key, off-host, deploy, and
rollback operations.

Inherited classification:

- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `LOCAL_CONTINUITY_PROVEN`
- `OFF_HOST_BACKUP_NOT_READY`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `REAL_RESTORE_NOT_AUTHORIZED`

NA-0359 evidence is dry-run fixture evidence only. It is not real restore
execution, not complete disaster recovery, not off-host backup completion, not
real key custody, and not real key recovery.

## Inherited NA-0358 restore-drill implementation authorization

NA-0358 authorized only the no-secret qsl-protocol dry-run restore harness. It
did not authorize isolated real restore, key handling, off-host operation,
backup, deploy, rollback, restore target creation/mount/copy, service repo
mutation, runtime mutation, dependency mutation, workflow mutation, or public
claim expansion.

The NA-0358 dry-run direction is sufficient prerequisite evidence for a
no-secret key custody/recovery harness because the future harness can model
custody and recovery metadata without relying on real backup repositories,
restore targets, keys, or passphrases.

## Inherited NA-0356 key custody/recovery prerequisite plan

NA-0356 selected a future custody direction of an operator-held repository
secret plus offline recovery envelope and no-secret evidence handling. It
selected a future recovery direction of sealed offline recovery plus isolated
restore verification before reliance.

NA-0356 remained a prerequisite plan. It generated no key, uploaded no key,
collected no passphrase, inspected no private key material, initialized no
repository, configured no remote target, ran no backup, ran no restore, and
implemented no key custody or key recovery.

Inherited real-key blockers:

- backup-plan update before any durable secret-related artifact or source-list
  change;
- local-ops authorization before any tool install, remote target, script,
  timer, fstab, system service, monitoring, or history-index dependency;
- operator runbook for normal custody, recovery envelope inventory, rotation,
  emergency access, lost-key response, and exposed-key response;
- isolated restore verification before relying on a real recovery envelope;
- no-secret evidence discipline for all CI, PR, response, journal, and proof
  artifacts.

## Inherited NA-0355 target/tool selection

NA-0355 selected only target/tool classes:

- target class: SSH/SFTP-compatible off-host host controlled by, or explicitly
  delegated to, the operator;
- tool class: restic-style encrypted snapshot repository with client-side
  encryption, check, prune, and isolated restore support.

The selection remains class-level only. No live host, remote path, account,
credential, repository password, passphrase, schedule, retention value, alert
channel, key, repository initialization, backup, restore, deploy, rollback,
purge, or secret handling exists from NA-0355.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| remote `HEAD` / `main` | `d40e6003fdf0` |
| branch state | detached HEAD, clean |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled; admins enforced |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-server PR #56 remains bounded end-to-end harness evidence only. It is not
production proof, not public-internet proof, and not external-review proof. No
qsl-server mutation was performed.

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| remote `HEAD` / `main` | `96b9352bd63e` |
| branch state | detached HEAD, clean |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-attachments PR #37 remains service-local prerequisite evidence only. It
does not prove public-internet readiness, complete disaster recovery, hot/live
backup, partial restore, or real key recovery. No qsl-attachments mutation was
performed.

## Local backup/key/off-host/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported 21 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported 58 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed local continuity snapshots through
  `daily-20260525T023319-0500`.
- Manifests and logs exist for current local snapshots through 2026-05-25.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.
- Installed tool discovery found `gpg`, `ssh`, and `rsync`.
- Installed tool discovery did not find `restic`, `borg`, `rclone`, or `age`.

Backup/history evidence:

- `QSL_BACKUP_PLAN.md` states the platter backup is local continuity and should
  not be the only disaster recovery copy.
- Backup status lists daily sources for `/srv/qbuild/work`, `/srv/qbuild/tmp`,
  qbuild mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses,
  and `QSL_BACKUP_PLAN.md`.
- `/home/victor/work/qsl/codex/responses` and `/requests` were present.
- `/home/victor/work/qsl/codex/directives` and `/journals` were absent at
  inspection.
- `/home/victor/work/qsl/codex/ops/backup` was present, but the installed
  daily source list does not cover the whole ops tree.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `NO_SECRET_KEY_CUSTODY_HARNESS_READY_FOR_AUTHORIZATION`
- `NO_SECRET_KEY_RECOVERY_HARNESS_READY_FOR_AUTHORIZATION`
- `OFF_HOST_BACKUP_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

No read-only evidence proves an off-host encrypted repository, repository
initialization, repository password/key custody, recovery envelope, key
rotation implementation, emergency access implementation, incident-response
runbook, off-host restore drill, remote retention/purge, remote monitoring, or
operator runbook.

## Key custody implementation authorization decision

Decision categories:

| Category | Result |
| --- | --- |
| `NO_SECRET_KEY_CUSTODY_HARNESS_AUTHORIZATION_READY` | selected |
| `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to real custody |
| `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to real custody |
| `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to real custody |
| `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK` | applies to real custody |
| `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL` | applies to reliance on real custody |
| `KEY_CUSTODY_IMPLEMENTATION_DEFERRED` | applies only to real custody |

Future no-secret qsl-protocol custody harness work can be authorized now. It
must be deterministic, fixture-only, and secret-free.

Real key custody cannot be authorized now. The repo has no approved backup-plan
update for secret-bearing artifacts, no local-ops authorization for tool or
system changes, no operator runbook for custody and emergency access, no
recovery envelope inventory, no real restore evidence, and no secret-handling
boundary for evidence artifacts.

## Key recovery implementation authorization decision

Decision categories:

| Category | Result |
| --- | --- |
| `NO_SECRET_KEY_RECOVERY_HARNESS_AUTHORIZATION_READY` | selected |
| `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to real recovery |
| `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to real recovery |
| `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to real recovery |
| `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK` | applies to real recovery |
| `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL` | applies to real recovery |
| `KEY_RECOVERY_IMPLEMENTATION_DEFERRED` | applies only to real recovery |

Future no-secret qsl-protocol recovery harness work can be authorized now. It
may model simulated recovery envelopes, simulated old-key compatibility,
rotation states, emergency-access markers, and incident-response markers.

Real recovery envelope implementation cannot be authorized now. There is no
real repository secret, no sealed envelope, no approved storage location, no
break-glass runbook, no rotation/revocation plan tied to old archives, no
restore verification, and no evidence process that can prove recovery without
printing or copying secret material.

## Future no-secret key custody/recovery implementation bundle

Future repo:

- qsl-protocol only.

Future allowed files:

- `inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- `docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md`
- `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden files and operations:

- qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto, Cargo,
  dependency, workflow, website, README, START_HERE, docs/public, backup
  script, timer, fstab, system service, local backup source-list, deploy,
  rollback, restore target, off-host target, live repository, key, passphrase,
  recovery envelope, private-key, monitoring configuration, and secret-handling
  paths.
- Any command that runs backup, restore, off-host tool setup, remote connection,
  repository initialization, key generation, key upload, passphrase collection,
  private-key inspection, deploy, rollback, service mutation, branch-protection
  mutation, or public-safety mutation.

Future commands:

- `bash -n scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- `python3 -m json.tool inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json >/dev/null`
- `bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- existing qsl-protocol queue, decisions, scope, link, leak, goal-lint,
  classifier, advisory, format, metadata, qshield, qsc, formal, and refimpl
  checks required by the future directive.

Future artifacts:

- Temporary proof under `/srv/qbuild/tmp/NA-0361_*`.
- No durable artifact outside qsl-protocol tracked files and `/srv/qbuild/tmp`.
- No secret, key, passphrase, private-key, endpoint credential, or live remote
  value in fixture, output, PR body, response file, or journal.

Future backup-plan impact:

- No backup-plan update is required for a pure qsl-protocol no-secret fixture
  and harness with temporary `/srv/qbuild/tmp/NA-0361_*` proof.
- Backup-plan update is required before any real key material, real recovery
  envelope, durable secret-related artifact, off-host repository, restore
  target, monitoring artifact, source-list change, script/timer/fstab/service
  change, backup, restore, deploy, rollback, or public-claim mutation.

Future public-claim boundary:

- The harness must state that simulated custody and simulated recovery are not
  real key custody or real key recovery.
- The harness must not claim complete disaster recovery, off-host backup
  completion, real restore completion, production readiness, public-internet
  readiness, external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing, hidden traffic shape,
  or hidden all metadata.

Future stop conditions:

- Any need for real keys, passphrases, private-key inspection, secret material,
  off-host setup, restore target creation/mount/copy, backup, restore, deploy,
  rollback, service mutation, dependency/workflow mutation, public-doc mutation,
  or backup-plan mutation stops NA-0361 unless a later directive explicitly
  authorizes it.

## Real key custody / real recovery boundary and blocker analysis

Real key generation is blocked because there is no approved generation
authority, entropy-handling record, storage path, backup exclusion/inclusion
policy, evidence redaction boundary, or rotation/revocation plan.

Real key storage is blocked because current backup status covers qsl-protocol
worktrees and Codex responses but not a designed secret store, recovery envelope
inventory, or full Codex ops/history coverage.

Real passphrase collection is blocked because no directive authorizes
interactive passphrase handling and no evidence procedure can prove the flow
without printing or retaining the passphrase.

Real recovery envelope creation is blocked because it would create a durable
secret-related artifact and require backup-plan, local-ops, access-control,
inventory, emergency-access, and rotation authorization.

Real operator-held secret, hardware-token, split-secret, old-archive key,
emergency-access, and incident-response operations are blocked until exact
operator runbooks, retention rules, old-key compatibility rules, isolated
restore verification, and secret-safe evidence procedures exist.

## No-secret fixture and simulated custody/recovery authorization analysis

| Option | Safety | Backup impact | Confidence gained | Confidence not gained | Result |
| --- | --- | --- | --- | --- | --- |
| qsl-protocol simulated custody/recovery fixture | high if deterministic and sentinel-only | none beyond current qsl-protocol/tmp scope | proves schema, markers, fail-closed checks, and claim boundaries | does not prove real secrets or restore | recommended |
| qsl-protocol marker/check harness | high if shell/Python only and no secret commands | none beyond current qsl-protocol/tmp scope | proves CI-checkable policy enforcement | does not prove operator execution | recommended |
| incident-response runbook marker harness | high if no-secret summary only | none beyond current qsl-protocol/tmp scope | proves lost/exposed-key branches exist | does not prove real incident handling | recommended |
| simulated old-key/rotation matrix | high if fake key ids only | none beyond current qsl-protocol/tmp scope | proves rotation compatibility states are named | does not re-encrypt or restore archives | recommended |
| qsl-server/qsl-attachments service-local fixture harness | lower for NA-0361 because sibling repos are read-only and outside selected scope | cross-repo scope risk | could prove service-local behavior later | not needed for no-secret authorization | deferred |
| no no-secret implementation | safe but unhelpful | none | avoids risk | leaves executable evidence gap | rejected |

## Recovery envelope / rotation / emergency-access authorization analysis

Recovery-envelope implementation may proceed only as no-secret simulation now.
The future harness may use simulated envelope IDs, fake key IDs, fake custody
role labels, fake inventory timestamps, and negative cases for missing envelope,
stale key ID, invalid rotation sequence, missing emergency-access marker, and
missing incident-response marker.

Real recovery envelope implementation remains blocked. It would require a real
secret or real secret-adjacent durable artifact, a physical/digital custody
decision, inventory procedure, access log, rotation trigger list, old archive
compatibility rule, exposed-key response, lost-key response, and isolated
restore verification.

Rotation can be simulated no-secret now. Old archive compatibility can be
modeled with fake key IDs and retention states. Emergency access and incident
response can be modeled with marker-only runbook branches. None of those
simulations may inspect private keys, collect passphrases, write secret logs, or
claim real recoverability.

## Restore-drill / off-host target-tool dependency analysis

The no-secret key custody/recovery harness should precede off-host target/tool
implementation because the selected restic-style encrypted repository depends
on repository-secret custody and recovery semantics.

Dry-run restore evidence from NA-0359 is sufficient for the no-secret key
custody/recovery harness. It is not sufficient for real key custody, real
recovery, isolated real restore, or off-host backup reliance.

Real key custody and real recovery must precede any claim that an encrypted
off-host repository can be recovered after primary loss. Isolated real restore
authorization still requires exact restore target isolation, cleanup,
monitoring, operator runbook, backup-plan update, and no-secret evidence first.

Off-host target/tool implementation remains blocked because `restic` is not
installed, no remote target exists, no account/path/quota/credential is
selected, no repository secret exists, no backup-plan update is approved, and no
local-ops mutation is authorized.

## Backup-plan impact and local-ops dependency decision

NA-0360 requires no backup-plan update. The changed paths are qsl-protocol
governance/testplan/journal files under `/srv/qbuild/work`.

A future pure qsl-protocol no-secret NA-0361 fixture/harness requires no
backup-plan update if generated proof remains temporary under `/srv/qbuild/tmp`
and no durable location outside the current backup scope is introduced.

Any real key material, recovery envelope, durable secret-related artifact,
off-host repository, restore target, monitoring artifact, source-list change,
script/timer/fstab/system-service change, backup, restore, deploy, rollback, or
public-claim mutation requires backup-plan update and exact local-ops
authorization first.

Local workflow-support/history-index work would materially reduce friction. The
startup worktree was clean but stale before manual fast-forward to the expected
`origin/main`, which confirms the value of qstart/qresume fast-forward support.
Response writer, bounded check polling, directive manifests, validation
profiles, per-directive allow-files, read-only source/authority helper,
claim-boundary scanner, directive/response/journal index, and backup coverage
for directive/request/journal/ops history folders remain useful future
local-ops work.

That local-ops work does not outrank NA-0361 because the no-secret harness can
proceed safely without mutating backup configuration or handling secrets. It
should precede real key custody/recovery or any durable operational history
dependency.

## Public-ingress/timing/traffic-shape boundary

NA-0360 changes no public ingress and does not prove hidden attachment size,
hidden timing metadata, hidden traffic shape, hidden all metadata, or padding
that hides all metadata.

qsl-server PR #56 remains bounded end-to-end harness evidence. qsl-attachments
PR #37 remains service-local prerequisite evidence. qshield embedded relay/demo
evidence remains reference/oracle evidence only.

## External-review sensitivity

External review remains incomplete. NA-0360 authorization evidence, future
no-secret harness evidence, NA-0359 dry-run restore evidence, qsl-server PR #56,
qsl-attachments PR #37, and qshield embedded relay/demo evidence do not prove
external-review completion.

Any stronger external-review statement requires explicit review scope,
reviewer evidence, issue disposition, runtime/service evidence, key custody
evidence, key recovery evidence, off-host backup evidence, real restore-drill
evidence, deployment evidence, monitoring/log evidence, and rollback evidence.

## Public claim boundary

No public docs or website files are updated by NA-0360.

Allowed claim posture:

- key custody/recovery no-secret harness authorization is ready;
- real key custody is not implemented;
- real key recovery is not implemented;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- dry-run restore evidence is not real restore completion;
- production and public-internet readiness are not established;
- anonymity, metadata-free behavior, and untraceability are not established;
- hidden attachment size, hidden timing metadata, hidden traffic shape, and
  hidden all metadata are not established.

## Future validation/marker/verification plan

Future NA-0361 markers:

- `NA0361_KEY_CUSTODY_AUTHORIZATION_OK`
- `NA0361_KEY_RECOVERY_AUTHORIZATION_OK`
- `NA0361_NO_SECRET_KEY_CUSTODY_HARNESS_OK`
- `NA0361_NO_SECRET_KEY_RECOVERY_HARNESS_OK`
- `NA0361_SIMULATED_CUSTODY_FIXTURE_OK`
- `NA0361_SIMULATED_RECOVERY_ENVELOPE_OK`
- `NA0361_SIMULATED_ROTATION_MATRIX_OK`
- `NA0361_INCIDENT_RESPONSE_MARKER_OK`
- `NA0361_OPERATOR_RUNBOOK_MARKER_OK`
- `NA0361_BACKUP_PLAN_IMPACT_OK`
- `NA0361_NO_REAL_KEY_GENERATION_OK`
- `NA0361_NO_KEY_UPLOAD_OK`
- `NA0361_NO_PASSPHRASE_COLLECTION_OK`
- `NA0361_NO_PRIVATE_KEY_INSPECTION_OK`
- `NA0361_NO_SECRET_MATERIAL_OK`
- `NA0361_NO_SECRET_ARTIFACT_OK`
- `NA0361_NO_REAL_KEY_CUSTODY_CLAIM_OK`
- `NA0361_NO_REAL_KEY_RECOVERY_CLAIM_OK`
- `NA0361_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0361_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0361_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

Future verification must include JSON parsing, shell syntax, harness execution,
negative fail-closed cases, secret/leak scan, qsl_evidence_helper queue and
decisions proof, scope guard, link-check, classifier proof, goal-lint, cargo
audit, rustls-webpki proof, qshield metadata/runtime checks, qsc send_commit,
formal model checks, and refimpl checks as required by the future directive.

## Workflow-support and history-index future work note

Read-only history paths were useful where present. The prior response file and
responses directory improved handoff confidence. The `requests` directory
confirmed already-recorded workflow-support needs. `directives` and `journals`
were absent, and the current backup source list does not cover the whole
requests/journals/directives/ops history set.

Future local-ops work should create or update a directive/response/journal
index and should decide backup coverage for directives, requests, journals, and
ops history before real key custody/recovery or other secret-adjacent
operations rely on those folders as evidence.

## Selected successor

`NA-0361 -- Metadata Runtime Key Custody / Key Recovery No-Secret Implementation Harness`

Rationale: current evidence is strong enough for a deterministic qsl-protocol
no-secret custody/recovery fixture and harness, but not for real key custody,
real recovery envelopes, off-host setup, or isolated real restore.

## Rejected alternatives

- Direct real key generation: rejected because no secret-handling, backup-plan,
  local-ops, runbook, recovery envelope, or restore verification boundary is
  authorized.
- Direct passphrase handling: rejected because no directive authorizes
  collection and no evidence path can handle it safely.
- Direct off-host setup: rejected because target/account/path/credential,
  restic installation, repository secret, backup-plan update, and local-ops
  authorization are absent.
- Isolated real restore authorization: rejected as the immediate successor
  because real recovery remains blocked and no-secret custody/recovery harness
  is the smaller executable proof step.
- Local ops/history index as NA-0361: deferred because it would reduce friction
  but is not the direct key custody/recovery evidence blocker for no-secret
  harness work.
- External review, website/public-claim audit, and technical-position-paper
  work: deferred because key custody/recovery and off-host/restore gaps remain
  explicit and unresolved.

## Backup-plan impact statement

No NA-0360 backup-plan update is required. Future no-secret qsl-protocol
NA-0361 work can proceed without backup-plan update only if it stays in tracked
qsl-protocol files and temporary `/srv/qbuild/tmp/NA-0361_*` proof.

Backup-plan update and exact local-ops authorization are prerequisites before
any real key material, real recovery envelope, durable secret-related artifact,
off-host repository, restore target, monitoring artifact, source-list change,
script/timer/fstab/system-service change, backup, restore, deploy, rollback, or
public-claim mutation.

## Next recommendation

Run NA-0361 as the qsl-protocol no-secret key custody/recovery implementation
harness. Keep real key custody, real key recovery, off-host backup, isolated
real restore, and public-claim changes blocked until exact future evidence and
local-ops authorization exist.
