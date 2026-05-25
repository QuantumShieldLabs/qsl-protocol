Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0358 Metadata Runtime Restore Drill Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0358 is a qsl-protocol governance and authorization lane only. It does not
execute a restore drill and does not create, mount, copy into, or preserve a
restore target.

Authorization result:

- `RESTORE_DRILL_DRY_RUN_AUTHORIZATION_READY`
- `DRY_RUN_RESTORE_AUTHORIZATION_READY`
- `RESTORE_DRILL_ISOLATED_REAL_AUTHORIZATION_READY` is not granted
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `OFF_HOST_BACKUP_NOT_READY`
- selected successor:
  `NA-0359 -- Metadata Runtime Restore Drill Dry-Run Implementation Harness`

The authorized future lane is limited to qsl-protocol no-secret dry-run fixture
evidence: manifest parsing, checksum validation, redacted artifact generation,
fail-closed negative cases, cleanup-marker checks, and public-claim boundary
checks. It must not run a live backup, restore, deploy, rollback, off-host
operation, key operation, passphrase operation, restore target creation, mount,
or restored-payload copy.

NA-0358 performed no qsl-server mutation, no qsl-attachments mutation, no
qshield runtime mutation, no qsc/qsp/protocol/crypto mutation, no dependency
change, no workflow change, no website/public-doc change, no backup script,
timer, fstab, or system-service mutation, no off-host setup, no backup, no
deploy, no rollback, no restore, no restore target creation/mount/copy, no key
generation, no key upload, no passphrase collection, no private-key inspection,
and no secret material handling.

## Live NA-0358 scope

The live queue marks NA-0358 READY and requires the next metadata-runtime
restore-drill authorization lane selected by NA-0357. Its objective is to decide
the exact future executable restore-drill boundary and evidence bundle before
any restore target, restore artifact, key handling, off-host backup operation,
backup-plan mutation, local-ops mutation, deployment, rollback, or public-claim
change can proceed.

Allowed scope for NA-0358 is governance evidence, a testplan, D-0698,
TRACEABILITY, and the rolling operations journal. Optional non-runtime planning
artifacts are not required by the live evidence. The forbidden scope includes
restore execution, restore target creation/mount/copy, off-host setup, backup,
deploy, rollback, key generation, passphrase collection, private-key
inspection, secret material handling, local backup script/timer/fstab mutation,
public-safety mutation, workflow mutation, dependency mutation, website or
public-doc mutation, README mutation, START_HERE mutation, and runtime,
protocol, crypto, qsc, qsp, qshield, qsl-server, or qsl-attachments
implementation changes.

Acceptance requires exactly one READY item, NA-0357 DONE, D-0696 and D-0697
present once, and a restore-drill implementation authorization, exact blocker,
or prerequisite successor before any restore execution or key/off-host
operation.

## Inherited NA-0357 restore-drill prerequisite plan

NA-0357 selected restore-drill implementation authorization as the next lane
and classified the posture as:

- `RESTORE_DRILL_READY_FOR_AUTHORIZATION`
- `LOCAL_CONTINUITY_PROVEN`
- `OFF_HOST_BACKUP_NOT_READY`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`

NA-0357 recommended a no-secret qsl-protocol fixture and manifest/checksum
validation path before any live backup or restore operation. It kept isolated
real restore gated on explicit key custody/recovery, backup-plan, local-ops,
cleanup, monitoring, runbook, and no-secret evidence authorization.

NA-0357 did not execute a restore, create a restore target, mount a restore
target, copy restored payloads, generate keys, collect passphrases, inspect
private key material, initialize a repository, configure a remote target, run a
backup, run a deploy, run a rollback, or mutate local backup configuration.

## Inherited NA-0356 key custody/recovery prerequisite plan

NA-0356 selected the future custody direction as an operator-held repository
secret plus a sealed offline recovery envelope. It selected the future recovery
direction as sealed offline recovery plus isolated restore verification before
reliance.

That was a prerequisite plan only. It left these classifications:

- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `TARGET_TOOL_IMPLEMENTATION_DEFERRED`
- `LOCAL_CONTINUITY_PROVEN`
- `OFF_HOST_BACKUP_NOT_READY`

No key custody or recovery implementation exists from NA-0356. No key was
generated or uploaded, no passphrase was collected, no private key material was
inspected, and no secret-dependent evidence was produced.

## Inherited NA-0355 target/tool selection

NA-0355 selected a class-level target/tool strategy:

- target class: SSH/SFTP-compatible off-host host controlled by, or explicitly
  delegated to, the operator;
- tool class: restic-style encrypted snapshot repository with client-side
  encryption, check, prune, and isolated restore support.

The selection remains class-level only. NA-0355 did not choose a live host,
remote path, provider account, credential, repository password, key,
passphrase, schedule, retention value, or alert channel. It did not initialize
a repository and did not run backup, restore, deploy, rollback, purge, key
generation, key upload, passphrase collection, or secret handling.

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

qsl-server PR #56 remains bounded end-to-end harness evidence. It is not
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

qsl-attachments PR #37 remains service-local prerequisite evidence. Its
cold/quiesced full-root restore boundary does not prove hot/live backup,
partial restore, public-internet readiness, or complete disaster recovery. No
qsl-attachments mutation was performed.

## Local backup/key/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported 20 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported 57 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through 2026-05-24.
- Manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.

Installed tool availability:

| Tool | Local result | Restore-drill impact |
| --- | --- | --- |
| `restic` | not found | selected tool class is not installed or implemented |
| `borg` | not found | fallback not available locally |
| `rclone` | not found | cloud/object transport tooling not available locally |
| `age` | not found | envelope-wrapper candidate not available locally |
| `gpg` | found | available but not selected as the primary backup tool |
| `ssh` | found | supports the selected SSH/SFTP-compatible target class |
| `rsync` | found | supports current local continuity model |

Backup/history evidence:

- local backup status lists `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild
  mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md` as daily sources;
- Codex responses and requests paths were present;
- Codex directives and journals paths produced no file listing at inspection;
- the Codex ops backup path was present, but the installed status did not list
  the whole ops tree as a daily source group;
- the local backup plan states that same-host platter backup is local
  continuity only and should not be the only disaster recovery copy.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `RESTORE_AUTHORIZATION_READY`
- `DRY_RUN_RESTORE_AUTHORIZATION_READY`
- `RESTORE_AUTHORIZATION_BLOCKED` for isolated real restore
- `ISOLATED_RESTORE_AUTHORIZATION_BLOCKED`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `OFF_HOST_BACKUP_NOT_READY`

No current evidence proves an off-host encrypted backup target, repository
initialization, repository password/key custody, key recovery envelope, key
rotation procedure, emergency access procedure, off-host restore drill, remote
retention/purge, remote monitoring/alerting, or operator runbook.

## Restore-drill implementation authorization decision

NA-0358 authorizes the next lane to implement a qsl-protocol-only no-secret
dry-run restore harness. The future harness may use deterministic fixtures,
manifest/checksum validation, redacted temporary artifacts, cleanup markers,
and fail-closed negative tests.

Decision categories:

| Category | Decision |
| --- | --- |
| `RESTORE_DRILL_DRY_RUN_AUTHORIZATION_READY` | selected |
| `RESTORE_DRILL_ISOLATED_REAL_AUTHORIZATION_READY` | not selected |
| `RESTORE_DRILL_BLOCKED_KEY_CUSTODY` | applies to isolated real restore |
| `RESTORE_DRILL_BLOCKED_RESTORE_TARGET` | applies to isolated real restore |
| `RESTORE_DRILL_BLOCKED_BACKUP_PLAN` | applies to real target/artifact/key/local-ops work |
| `RESTORE_DRILL_BLOCKED_LOCAL_OPS` | applies to local system or durable operational changes |
| `RESTORE_DRILL_BLOCKED_SECRET_HANDLING` | applies to any key/passphrase/private material path |
| `RESTORE_DRILL_DEFERRED` | applies to live/off-host/production restore |

No-secret dry-run implementation can proceed before key custody because it does
not require a real encrypted repository, repository password, private key, or
passphrase. Isolated real restore cannot proceed before key custody/recovery,
restore target isolation, backup-plan update, local-ops authorization, cleanup,
monitoring, and runbook evidence.

## Future restore-drill implementation bundle

The future NA-0359 bundle is qsl-protocol-only and must remain no-secret.

Allowed future repo:

- `QuantumShieldLabs/qsl-protocol`

Expected allowed files:

- `inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json`
- `scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`
- `tests/NA-0359_metadata_runtime_restore_drill_dry_run_implementation_harness_testplan.md`
- `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_implementation_harness.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden future files and operations:

- qsl-server, qsl-attachments, qshield runtime behavior outside the exact
  harness if not explicitly authorized, qsc/qsp/protocol/crypto, Cargo files,
  dependencies, workflows, website, README, START_HERE, docs/public, backup
  scripts, timers, fstab, service units, restore roots, key paths, passphrase
  paths, remote destination configs, deployment scripts, rollback scripts, and
  branch-protection/public-safety configuration;
- restore execution, restore target creation, restore target mount, restored
  payload copy, backup operation, deploy operation, rollback operation, off-host
  setup, repository initialization, prune/purge, key generation, key upload,
  passphrase collection, private-key inspection, or secret handling.

Expected future commands:

- JSON validation for the fixture.
- POSIX-shell syntax validation for the harness.
- The harness invocation against the fixture.
- Queue/decision/scope/link/leak/goal-lint validation.
- The existing metadata runtime and qshield/qsc/formal checks required by the
  future directive.

Expected future artifacts:

- temporary generated proof under `/srv/qbuild/tmp`;
- no restored payloads;
- no keys;
- no passphrases;
- no private material;
- no endpoint credentials;
- no auth headers;
- redacted manifest/checksum summaries only.

Expected future markers:

- `NA0359_RESTORE_DRILL_AUTHORIZATION_OK`
- `NA0359_DRY_RUN_RESTORE_HARNESS_OK`
- `NA0359_NO_SECRET_FIXTURE_OK`
- `NA0359_MANIFEST_CHECKSUM_VALIDATION_OK`
- `NA0359_ARTIFACT_REDACTION_OK`
- `NA0359_FAILED_VALIDATION_FAILS_CLOSED_OK`
- `NA0359_CLEANUP_MARKER_OK`
- `NA0359_MONITORING_ALERT_PLAN_MARKER_OK`
- `NA0359_OPERATOR_RUNBOOK_MARKER_OK`
- `NA0359_BACKUP_PLAN_IMPACT_OK`
- `NA0359_NO_RESTORE_EXECUTION_OK`
- `NA0359_NO_RESTORE_TARGET_CREATION_OK`
- `NA0359_NO_KEY_GENERATION_OK`
- `NA0359_NO_PASSPHRASE_COLLECTION_OK`
- `NA0359_NO_SECRET_MATERIAL_OK`
- `NA0359_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0359_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0359_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

Backup-plan update is not required for the future qsl-protocol-only fixture if
it writes only checked-in qsl-protocol paths plus temporary artifacts under
`/srv/qbuild/tmp`. Backup-plan update is required before any real restore
target, durable restore artifact outside that scope, key material, recovery
envelope inventory, source-list change, tool install, script/timer/fstab/system
service change, monitoring artifact, backup, restore, deploy, rollback, or
public-claim mutation.

## Dry-run restore implementation authorization analysis

| Option | Safety | Secret risk | Restore operation risk | Backup-plan impact | Confidence gained | Confidence not gained | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| qsl-protocol fixture-only dry-run harness | highest | low if fixture is sentinel-only | none if it never creates target/copies payloads | no if temp-only artifacts | command order, markers, failure behavior | real archive readability | recommended |
| qsl-protocol manifest/checksum-only harness | high | low | none | no if temp-only artifacts | stale/missing/corrupt manifest detection | key recovery and real restore | recommended |
| qsl-protocol no-secret archive simulation | medium-high | low if archive is fake | low if it does not call restore tooling | no if temp-only artifacts | command-shape and artifact handling | real encrypted repository operation | recommended only if exact fixture semantics are frozen |
| qsl-backup list/preflight wrapper harness | medium | low | none if read-only | no | current local continuity visibility | off-host encrypted restore | supportive only |
| qsl-server/qsl-attachments service-local fixture harness | medium | depends on service fixtures | risk of cross-repo mutation | yes if outside current repo | service-specific confidence | qsl-protocol-only governance scope | deferred |
| no dry-run implementation | safest short-term | none | none | none | avoids risk | no executable progress | rejected as final NA-0359 direction |

## Isolated restore implementation authorization analysis

| Future option | Safety | Required authorization | Key/secret dependency | Backup-plan impact | Cleanup/monitoring | Evidence value | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| isolated `/srv/qbuild/tmp` target | best first real restore target if exact path is authorized | future restore/local-ops directive | yes for real encrypted archive | yes if durable artifacts or source lists change | required | real restore mechanics without live overwrite | deferred |
| isolated disposable disk | strong isolation | future device/local-system directive | yes | yes | required | disk-level recovery proof | deferred |
| isolated non-live qbuild restore target | useful with exact path and isolation proof | future qbuild/local-ops directive | yes | yes | required | qbuild tree restore shape | deferred |
| off-host restore to staging machine | strong disaster-recovery signal | future off-host/staging directive | yes | yes | required | remote recovery assumptions | deferred |
| production root restore | destructive risk | production incident directive only | yes | yes | required | high but unsafe for drill | rejected |
| no isolated restore | avoids risk | none | none | none | none | no real restore confidence | rejected as final state |

No isolated real restore is authorized by NA-0358.

## Key custody/recovery dependency decision for implementation

No-secret dry-run implementation can proceed before key custody because it must
not depend on real repository secrets, private keys, passphrases, provider
credentials, auth headers, or remote endpoints.

Isolated real restore requires key custody/recovery implementation first or in
the same future explicitly authorized operation bundle. Required prerequisites:

- repository secret custody owner and access path;
- sealed offline recovery envelope inventory;
- old-key archive compatibility;
- key rotation compatibility;
- emergency access trigger and access log;
- no private-key inspection in evidence;
- no passphrase collection in CI, PRs, response files, or journals;
- no secret logs;
- fail-closed stop if any key is missing, ambiguous, exposed, wrong, or cannot
  be recovered without printing secret material.

Old-key/archive compatibility blocks isolated real restore until the future
lane defines whether old keys remain sealed until old snapshots expire or
whether migration/re-encryption is authorized. Key rotation compatibility and
recovery-envelope implementation must precede reliance on an encrypted real
restore.

## Manifest/checksum/artifact integrity authorization plan

Future manifest fixture source:

- a deterministic qsl-protocol JSON fixture with source-set identifiers,
  fixture snapshot ID, fixture timestamp, expected non-secret path inventory,
  explicit exclusions, and expected validation outcomes.

Future checksum fixture source:

- deterministic checksums over non-secret fixture files only.

Expected validation stages:

1. Parse fixture manifest.
2. Reject missing, stale, malformed, or mismatched manifest data.
3. Validate expected path inventory and explicit exclusions.
4. Validate checksum set.
5. Emit redacted artifact summary under `/srv/qbuild/tmp`.
6. Scan generated artifact for sentinel secret leakage.
7. Verify cleanup markers.
8. Fail closed on any mismatch without fallback.

Artifact rules:

- no secrets, tokens, auth headers, passphrases, private keys, endpoint
  credentials, sensitive endpoint fragments, or restored payloads;
- short SHAs only in narrative evidence;
- no long hex dumps in governance prose;
- preserve marker logs and redacted summaries, not payloads.

## Cleanup/monitoring/alerting/runbook authorization plan

Future dry-run cleanup:

- remove generated fixture output unless explicitly preserved as redacted
  evidence;
- detect stale temp paths and fail closed unless cleanup is explicitly
  authorized;
- preserve only marker summaries and redacted logs.

Future isolated real restore cleanup, if later authorized:

- stop before retry after failure;
- quarantine or remove the isolated target according to the future directive;
- do not prune or purge as part of failure recovery unless explicitly
  authorized;
- verify no live path was touched.

Future monitoring and alerting plan:

- alert on failed restore drill;
- alert on missing manifest/checksum;
- alert on missing key/recovery proof for real restore;
- alert on cleanup failure;
- alert on stale recovery envelope inventory;
- alert on public-claim boundary violation.

Emergency stop conditions:

- wrong target path;
- live path match;
- unexpected network/off-host command;
- restore/backup/deploy/rollback command outside exact authorization;
- key/passphrase/private material appears in logs or artifacts;
- checksum mismatch;
- stale/missing manifest;
- missing cleanup proof;
- missing runbook proof;
- public-claim overreach.

Future runbook sections:

- scope and forbidden operations;
- fixture source and expected markers;
- manifest/checksum validation;
- artifact redaction;
- cleanup proof;
- monitoring/alert proof;
- operator verification checklist;
- stop conditions;
- backup-plan impact.

## Backup-plan impact and local-ops dependency decision

NA-0358 itself requires no backup-plan update because changes remain in
qsl-protocol governance/testplan/journal paths under `/srv/qbuild/work`.

NA-0359 does not require backup-plan update if it stays within qsl-protocol
checked-in paths plus temporary artifacts under `/srv/qbuild/tmp`.

Backup-plan update and exact local-ops authorization are required before any
future lane creates or changes:

- real restore target;
- durable restore artifact outside current qbuild scope;
- key material;
- recovery envelope inventory;
- repository;
- remote target;
- source list;
- tool installation;
- script, timer, fstab, or system service;
- retention/purge policy;
- monitoring artifact;
- backup, restore, deploy, rollback, or public-claim state.

Local workflow-support/history-index work would materially reduce friction but
does not outrank the dry-run restore harness now. It should remain a near-term
follow-on unless NA-0359 proves it is a blocker. Current local evidence shows
responses and requests present; directives and journals were not populated at
inspection; and whole-ops-tree backup coverage is not proven in the installed
status. D132 remains present and untouched.

## Public-ingress/timing/traffic-shape boundary

NA-0358 changes no public ingress and does not create public-ingress evidence.
It does not prove hidden attachment size, hidden timing metadata, hidden
traffic shape, hidden all metadata, or padding that hides all metadata.
qshield embedded relay/demo evidence remains reference/oracle evidence only.
qsl-server PR #56 remains bounded end-to-end harness evidence only.
qsl-attachments PR #37 remains service-local prerequisite evidence only.

## External-review sensitivity

External review remains incomplete. Restore-drill implementation authorization
is not restore execution and is not external-review completion. Any stronger
external-review statement requires key custody evidence, key recovery evidence,
off-host backup evidence, restore drill evidence, service evidence, deployment
evidence, monitoring/log evidence, rollback evidence, and external reviewer
evidence.

## Public claim boundary

NA-0358 must not be described as restore execution, disaster recovery
completion, off-host backup completion, key custody implementation, key
recovery implementation, production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
or hidden all metadata.

Website/public docs remain unchanged. Any future public-claim update requires a
separate explicit directive and evidence that preserves current NOT_READY and
PARTIAL boundaries.

## Future validation/marker/verification plan

Future NA-0359 must prove:

- `NA0359_RESTORE_DRILL_AUTHORIZATION_OK`
- `NA0359_DRY_RUN_RESTORE_HARNESS_OK`
- `NA0359_NO_SECRET_FIXTURE_OK`
- `NA0359_MANIFEST_CHECKSUM_VALIDATION_OK`
- `NA0359_ARTIFACT_REDACTION_OK`
- `NA0359_FAILED_VALIDATION_FAILS_CLOSED_OK`
- `NA0359_CLEANUP_MARKER_OK`
- `NA0359_MONITORING_ALERT_PLAN_MARKER_OK`
- `NA0359_OPERATOR_RUNBOOK_MARKER_OK`
- `NA0359_BACKUP_PLAN_IMPACT_OK`
- `NA0359_NO_RESTORE_EXECUTION_OK`
- `NA0359_NO_RESTORE_TARGET_CREATION_OK`
- `NA0359_NO_KEY_GENERATION_OK`
- `NA0359_NO_PASSPHRASE_COLLECTION_OK`
- `NA0359_NO_SECRET_MATERIAL_OK`
- `NA0359_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0359_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0359_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If NA-0359 is blocked, blocker markers must name the exact missing fixture,
manifest/checksum rule, cleanup rule, monitoring rule, runbook section,
backup-plan/local-ops authorization, or claim-boundary proof.

## Workflow-support and history-index future work note

The accepted local workflow-support request remains valid future local-ops
work. These items would reduce repeated directive friction:

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

Those items are not implemented in NA-0358. They may become a successor only if
future evidence proves they are the exact blocker for truthful executable
restore evidence.

## Selected successor

Selected successor:

`NA-0359 -- Metadata Runtime Restore Drill Dry-Run Implementation Harness`

Rationale:

- NA-0355 selected target/tool classes at class level.
- NA-0356 selected key custody/recovery directions at prerequisite level.
- NA-0357 selected restore-drill implementation authorization as the next
  missing prerequisite.
- Live qsl-server and qsl-attachments source/authority/CI evidence is fresh
  enough for planning.
- Local continuity backup is mounted and current, but off-host encrypted backup
  remains unimplemented.
- A no-secret qsl-protocol fixture harness is the safest executable next step
  and does not require real key handling, restore target creation, off-host
  operation, or backup-plan mutation.

## Rejected alternatives

- Direct restore execution: rejected because NA-0358 is authorization only.
- Isolated real restore now: rejected because key custody/recovery, restore
  target isolation, backup-plan/local-ops, cleanup, monitoring, and runbook
  evidence are not implemented.
- Key custody/recovery implementation authorization as NA-0359: deferred
  because the dry-run harness can safely precede real secrets and will sharpen
  future evidence requirements.
- Off-host target/tool implementation authorization as NA-0359: deferred
  because the dry-run restore harness should establish executable
  manifest/checksum and fail-closed evidence first.
- QSL Local Ops Codex Workflow Support and History Index Plan: useful but not
  the next blocking prerequisite.
- External review, website/public-claim audit, or technical-position-paper
  work: deferred until backup, key, restore, service, deployment, monitoring,
  rollback, and review evidence are stronger.
- Claiming disaster recovery completion: rejected because current evidence is
  local continuity only and off-host encrypted backup remains unimplemented.

## Backup-plan impact statement

No NA-0358 backup-plan update is required. Future qsl-protocol-only no-secret
dry-run fixture work can proceed without backup-plan update if artifacts remain
temporary under `/srv/qbuild/tmp`. Any real restore target, durable restore
artifact, key material, recovery envelope inventory, repository, remote target,
source-list, script, timer, fstab, system service, retention/purge policy,
monitoring artifact, backup, restore, deploy, rollback, or public-claim change
requires backup-plan update and exact local-ops authorization first.

## Next recommendation

After NA-0358 merges and public-safety is green, close out NA-0358 and restore
`NA-0359 -- Metadata Runtime Restore Drill Dry-Run Implementation Harness` as
the sole READY successor. NA-0359 must not execute a restore unless a later
explicit directive authorizes exact operation, target isolation, key handling,
backup-plan update, local-ops mutation, cleanup, monitoring, and no-secret
evidence.
