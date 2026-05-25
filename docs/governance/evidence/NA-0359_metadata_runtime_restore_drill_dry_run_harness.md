Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0359 Metadata Runtime Restore Drill Dry-Run Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0359 implements the qsl-protocol-only no-secret restore-drill dry-run
harness authorized by NA-0358. The harness validates deterministic fixture
manifest/checksum relationships, writes a redacted proof artifact under
`/srv/qbuild/tmp`, proves seven fail-closed negative cases, and emits explicit
NA0359 markers.

This is dry-run fixture evidence only. It does not run a backup, run a restore,
create a restore target, mount a restore target, copy restored payloads,
initialize or contact an off-host destination, generate or upload keys, collect
passphrases, inspect private keys, mutate local backup configuration, deploy,
rollback, or mutate qsl-server/qsl-attachments/qshield runtime behavior.

Selected successor:

`NA-0360 -- Metadata Runtime Key Custody / Key Recovery Implementation Authorization Plan`

## Live NA-0359 scope

The live queue marks NA-0359 READY and requires a deterministic no-secret
qsl-protocol fixture and harness selected by NA-0358. Allowed implementation
scope is limited to:

- `inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json`
- `scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`
- this evidence file
- `tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope remains restore execution, restore target creation/mount/copy,
backup execution, deploy, rollback, off-host setup, key handling, passphrase
collection, private-key inspection, local backup script/timer/fstab mutation,
dependency/workflow mutation, website/public-doc mutation, README/START_HERE
mutation, qsl-server/qsl-attachments mutation, qshield runtime mutation, and
protocol/crypto/qsc/qsp implementation change.

## Inherited NA-0358 authorization

NA-0358 authorized a future qsl-protocol no-secret dry-run restore harness
after NA-0357 prerequisite planning. It classified isolated real restore as not
authorized, key custody/recovery as partial, and off-host backup as not ready.

NA-0358 required deterministic fixture data, manifest/checksum validation,
redacted temporary artifacts, cleanup/monitoring/runbook markers, fail-closed
negative cases, and public-claim boundary checks. NA-0358 explicitly did not
authorize backup, restore, deploy, rollback, restore target creation/mount/copy,
off-host operation, key handling, passphrase handling, private-key inspection,
secret material handling, service repo mutation, runtime mutation, dependency
mutation, workflow mutation, or public-claim expansion.

## Inherited NA-0357 restore-drill prerequisite plan

NA-0357 selected restore-drill implementation authorization as the next lane and
classified the state as `RESTORE_DRILL_READY_FOR_AUTHORIZATION`,
`LOCAL_CONTINUITY_PROVEN`, `OFF_HOST_BACKUP_NOT_READY`, `KEY_CUSTODY_PARTIAL`,
and `KEY_RECOVERY_PARTIAL`.

Its future dry-run direction was no-secret qsl-protocol fixture evidence with
manifest/checksum validation before any live backup or restore operation.
Isolated real restore remained gated on explicit key custody/recovery,
backup-plan, local-ops, cleanup, monitoring, runbook, and no-secret evidence
authorization.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| remote main | `d40e6003fdf0` |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust`; force pushes disabled; deletions disabled; admins enforced |
| open PRs | none listed |
| latest main CI | `ci` success on `d40e6003fdf0` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-server PR #56 remains bounded end-to-end harness evidence only. It is not
production proof, public-internet proof, or external-review proof. No qsl-server
mutation was performed.

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| remote main | `96b9352bd63e` |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust`; force pushes disabled; deletions disabled |
| open PRs | none listed |
| latest main CI | `rust` success on `96b9352bd63e` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-attachments PR #37 remains service-local prerequisite evidence only. It
does not prove hot/live backup, partial restore, public-internet readiness, or
complete disaster recovery. No qsl-attachments mutation was performed.

## Local backup/key/restore evidence refresh

Read-only local evidence at NA-0359 start:

- `/backup/qsl` was mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported about 21 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported about 57 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through 2026-05-25.
- Manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.
- Installed tool discovery found `gpg`, `ssh`, and `rsync`; `restic`, `borg`,
  `rclone`, and `age` were not found.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `DRY_RUN_HARNESS_READY`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `OFF_HOST_BACKUP_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

The local same-host continuity backup is not complete disaster recovery. No
off-host encrypted repository, repository secret, recovery envelope, key
rotation procedure, off-host restore drill, remote retention/purge,
monitoring/alerting, or operator runbook is implemented by NA-0359.

## Harness design and fixture schema

The fixture schema lives at
`inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json` and includes
schema version, artifact class, source classification, backup state
classification, restore state classification, target type, restore mode,
no-secret sentinels, manifest entries, checksums, expected validation outcomes,
tamper/negative cases, cleanup plan, monitoring/alert plan, operator runbook
summary, forbidden operations, claim boundaries, required markers, backup-plan
impact, qsl-server/qsl-attachments boundary, and qshield demo boundary.

The fixture contains only benign fake values. It contains no real backup
payloads, private key text, passphrases, tokens, credentials, provider
endpoints, or real restore target paths. Sentinel labels appear only in the
fixture sentinel list so the harness can prove that sentinel leakage into proof
artifacts would fail closed.

## No-secret fixture summary

The fixture includes three deterministic manifest entries:

- `manifest-alpha`
- `manifest-beta`
- `restore-plan-summary`

Each entry has inline fake content and a SHA-256 checksum. The harness recomputes
each checksum and verifies that the manifest entries and checksum table agree.

The fixture includes these fail-closed cases:

- missing manifest;
- checksum mismatch;
- missing cleanup plan;
- prohibited operation field;
- missing claim boundary;
- missing no-secret fixture marker;
- sentinel leak detection.

## Harness implementation summary

The harness lives at
`scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`. It accepts an
optional fixture path argument, validates that temporary proof output stays under
`/srv/qbuild/tmp`, validates JSON with Python, validates required schema fields,
recomputes manifest SHA-256 checksums, validates forbidden operations and
public-claim boundaries, runs all negative cases in memory, writes a redacted
text proof artifact under `/srv/qbuild/tmp/NA-0359_*`, emits all required
NA0359 markers, and exits nonzero on tampered or incomplete fixtures.

The harness does not call backup, restore, off-host, deploy, rollback, key,
passphrase, service, systemd, mount, network, or local backup mutation commands.

## Harness execution and marker evidence

Local execution:

```text
bash -n scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh
python3 -m json.tool inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json
```

Result:

- proof artifact:
  `/srv/qbuild/tmp/NA-0359_restore_drill_dry_run.Gokqjf/na0359_restore_drill_dry_run_proof.txt`
- proof artifact size: 1482 bytes
- proof artifact SHA-256: `ca0a9666d0f6`
- `RESTORE_DRY_RUN_SECRET_FINDING_COUNT 0`
- `NA0359_OPERATION_EXECUTED_COUNT 0`
- `NA0359_RESTORE_TARGET_CREATED_COUNT 0`
- `NA0359_KEY_OPERATION_COUNT 0`
- `NA0359_OFF_HOST_OPERATION_COUNT 0`
- `NA0359_DEPLOY_ROLLBACK_OPERATION_COUNT 0`

All required markers were emitted, including
`NA0359_METADATA_RUNTIME_RESTORE_DRY_RUN_OK`.

## Negative/fail-closed fixture validation

The harness executes seven negative cases in memory and requires each to fail
closed:

| Case | Fail-closed proof |
| --- | --- |
| missing manifest | removing `manifest_entries` is rejected |
| checksum mismatch | replacing a manifest checksum is rejected |
| missing cleanup plan | removing `cleanup_plan` is rejected |
| prohibited operation field | setting `operation_executed_by_harness` true is rejected |
| missing claim boundary | removing `production_readiness` boundary is rejected |
| missing no-secret fixture marker | removing `NA0359_NO_SECRET_FIXTURE_OK` is rejected |
| sentinel leak detection | injecting a sentinel into proof output is detected |

The proof emitted `NA0359_NEGATIVE_CASES_PASSED 7`.

## Artifact redaction, secret-scan, cleanup, and `/srv/qbuild/tmp` proof

The harness writes exactly one proof file under the generated
`/srv/qbuild/tmp/NA-0359_*` directory. The proof is text and rebuildable.

Redaction and scan properties:

- proof artifact is under `/srv/qbuild/tmp`;
- proof artifact contains no no-secret sentinel labels;
- proof artifact contains no private-key, token, credential, or passphrase-like
  pattern;
- fixture sentinel labels are benign labels only and are not emitted into the
  proof artifact;
- no restore payloads or real backup contents are written;
- no durable evidence location outside the repo and `/srv/qbuild/tmp` is
  required.

Cleanup behavior is marker-only for this dry-run: no staging payloads are
created. Any future real restore cleanup remains separately authorized work.

## Backup-plan impact and local-ops dependency decision

No NA-0359 backup-plan update is required because the durable changes are
tracked qsl-protocol files and temporary proof artifacts are under
`/srv/qbuild/tmp`.

Future backup-plan update and local-ops authorization are required before
isolated real restore target creation, durable restore artifacts, key material
or recovery envelope inventory, off-host target or repository setup, backup
source-list changes, backup/restore tool installation, scripts, timers, fstab,
system service mutation, retention, purge, monitoring, backup, restore, deploy,
rollback, or public claim mutation.

Local workflow support remains useful but is not the immediate successor. The
startup fast-forward issue in this directive confirms that qstart/qresume
fast-forward support, response-file writing, bounded polling helpers,
machine-readable directive manifests, validation profiles, allow-files,
source/authority helpers, claim-boundary scanning, and directive/response
history indexes would reduce friction. They do not block this dry-run harness.

## Public-ingress/timing/traffic-shape boundary

NA-0359 changes no public ingress and does not update website or public docs.
It does not prove hidden attachment size, hidden timing metadata, hidden traffic
shape, hidden all metadata, or padding that hides all metadata.

qsl-server PR #56 remains bounded harness evidence only, qsl-attachments PR #37
remains service-local prerequisite evidence only, and qshield embedded
relay/demo evidence remains reference/oracle evidence only.

## External-review sensitivity

External review remains incomplete. The dry-run harness is not real restore
execution, not disaster recovery completion, not off-host backup completion, not
key custody/recovery implementation, and not production/public-internet
readiness evidence.

Any stronger claim requires key custody evidence, key recovery evidence,
off-host backup evidence, real restore-drill evidence, service evidence,
deployment evidence, monitoring/log evidence, rollback evidence, and external
reviewer evidence.

## Public claim boundary

No NA-0359 text may state or imply production readiness, public-internet
readiness, external-review completion, anonymity, metadata-free behavior,
untraceable behavior, hidden attachment size, hidden timing metadata, hidden
traffic shape, hidden all metadata, local continuity as complete disaster
recovery, off-host encrypted backup completion, real restore-drill completion,
or key custody/recovery implementation.

## Successor selection

Selected successor:

`NA-0360 -- Metadata Runtime Key Custody / Key Recovery Implementation Authorization Plan`

Rationale:

- NA-0359 now freezes executable no-secret manifest/checksum and fail-closed
  restore-drill dry-run evidence.
- The next blocker before real encrypted backup or isolated real restore is
  still key custody/recovery authorization.
- Off-host backup implementation and isolated real restore remain unsafe to
  authorize before repository secret custody, recovery envelope, emergency
  access, rotation, and no-secret handling boundaries are implementation-grade.

## Rejected alternatives

- Direct real restore execution: rejected because NA-0359 authorizes fixture
  dry-run validation only.
- Off-host encrypted backup implementation as NA-0360: deferred until key
  custody/recovery implementation authorization is complete.
- Isolated real restore authorization as NA-0360: deferred until key
  custody/recovery and backup-plan/local-ops prerequisites are stronger.
- Local ops workflow support as NA-0360: useful, but not the next restore-chain
  blocker.
- Website/public-claim audit, external review, or position paper work: deferred
  until backup, key, restore, service, deployment, monitoring, rollback, and
  review evidence are stronger.

## Backup-plan impact statement

No NA-0359 backup-plan update is required. Future real restore, off-host
backup, key custody/recovery, monitoring, source-list, script/timer/fstab,
service, deploy/rollback, or public-claim mutation requires backup-plan update
and exact local-ops authorization first.

## Next recommendation

After NA-0359 merges with public-safety green, close out NA-0359 and restore
`NA-0360 -- Metadata Runtime Key Custody / Key Recovery Implementation Authorization Plan`
as the sole READY successor. Do not implement NA-0360 in the NA-0359 closeout.
