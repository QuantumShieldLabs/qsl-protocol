Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0361 Metadata Runtime Key Custody Key Recovery No-Secret Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0361 adds the qsl-protocol-only no-secret key custody / key recovery
fixture and harness authorized by NA-0360. The fixture uses simulated key IDs,
simulated custody records, simulated recovery-envelope metadata, simulated
rotation state, simulated old-archive compatibility, simulated incident
response, and simulated emergency-access markers only.

This lane does not generate, upload, read, inspect, collect, store, or handle
real key material, passphrases, private keys, recovery-envelope contents, backup
payloads, restore payloads, remote targets, or off-host credentials. It writes a
temporary no-secret proof under `/srv/qbuild/tmp` and keeps all production,
public-internet, external-review, anonymity, metadata, disaster-recovery,
restore, and real key custody/recovery boundaries explicit.

Selected successor:

`NA-0362 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Implementation Authorization Plan`

## Live NA-0361 scope

The live queue marks NA-0361 READY and requires deterministic qsl-protocol
fixture/harness evidence for key custody / key recovery boundaries. Allowed
implementation scope is limited to:

- `inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- this evidence file
- `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The lane forbids qsl-server mutation, qsl-attachments mutation, qshield runtime
mutation, qsc/qsp/protocol/crypto/key-schedule implementation change,
dependency or workflow mutation, website/public-doc mutation, README or
START_HERE mutation, backup script/timer/fstab mutation, off-host setup,
backup, restore, restore target creation/mount/copy, deploy, rollback, real key
generation, key upload, passphrase collection, private-key inspection,
recovery-envelope content creation, and secret material handling.

## Inherited NA-0360 authorization

NA-0360 authorized only a future qsl-protocol no-secret fixture/harness. It
recorded:

- `NO_SECRET_KEY_CUSTODY_HARNESS_AUTHORIZATION_READY`
- `NO_SECRET_KEY_RECOVERY_HARNESS_AUTHORIZATION_READY`
- real key custody remains blocked by backup-plan, local-ops, secret-handling,
  operator-runbook, and restore-drill prerequisites;
- real key recovery and real recovery-envelope handling remain blocked by the
  same prerequisite set.

NA-0360 did not authorize real keys, passphrases, recovery-envelope contents,
off-host setup, backup, restore, deploy, rollback, service mutation, runtime
mutation, dependency mutation, workflow mutation, website/public-doc mutation,
or public-claim expansion.

## Inherited NA-0359 restore-drill dry-run harness

NA-0359 implemented a no-secret restore-drill dry-run fixture and harness. It
validated manifest/checksum relationships, wrote redacted temporary proof under
`/srv/qbuild/tmp/NA-0359_*`, emitted NA0359 markers, proved seven negative cases
fail closed, and recorded zero restore, restore target, key, off-host, deploy,
and rollback operations.

NA-0359 remains dry-run evidence only. It is not real restore execution, not
complete disaster recovery, not off-host backup completion, not real key
custody, and not real key recovery.

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
production proof, public-internet proof, or external-review proof. No
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

qsl-attachments PR #37 remains service-local prerequisite evidence only. It is
not production/public-internet proof and does not prove complete disaster
recovery. No qsl-attachments mutation was performed.

## Local backup/key/off-host/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted as ext4 and has current daily snapshots through
  2026-05-25.
- `/backup/qsl` reported about 21 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported about 58 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from 2026-05-17
  and daily snapshots through 2026-05-25.
- Local manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.
- Installed tool discovery found `gpg`, `ssh`, and `rsync`; `restic`, `borg`,
  `rclone`, and `age` were not found.

Backup/history evidence:

- `QSL_BACKUP_PLAN.md` states that platter backup is local continuity and not
  complete disaster recovery.
- Backup status lists `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild mirrors,
  qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md` as daily sources.
- `/home/victor/work/qsl/codex/responses` and `/requests` were present.
- `/home/victor/work/qsl/codex/directives` and `/journals` were absent.
- `/home/victor/work/qsl/codex/ops/backup` was present, but the installed daily
  source list does not cover the whole ops tree.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `NO_SECRET_KEY_CUSTODY_HARNESS_READY`
- `NO_SECRET_KEY_RECOVERY_HARNESS_READY`
- `OFF_HOST_BACKUP_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

No read-only evidence proves off-host encrypted repository initialization,
repository password/key custody, recovery-envelope content, real key rotation,
emergency access execution, real incident response, off-host restore drill,
remote retention/purge, remote monitoring, or production operator runbook
completion.

## No-secret fixture design and schema

The fixture schema lives at
`inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json` and
includes schema version, artifact class, source classification, local backup
classification, off-host classification, simulated-only custody and recovery
modes, simulated key IDs, simulated custody records, simulated
recovery-envelope metadata, simulated rotation matrix, simulated old-archive
compatibility matrix, simulated incident-response cases, simulated emergency
access cases, operator runbook markers, integrity hashes, operation counters,
no-secret sentinels, expected outcomes, tamper cases, forbidden operations,
claim boundaries, required markers, backup-plan impact, and qsl-server /
qsl-attachments / qshield boundaries.

The fixture contains only benign fake values. It contains no real keys, real
passphrases, private key text, tokens, credentials, real recovery-envelope
contents, real off-host endpoints, real backup repository names, real secret
paths, private material paths, raw secret material, or unredacted sensitive
operational data.

## No-secret fixture implementation summary

The fixture includes:

- two simulated key IDs: one active and one retired;
- one simulated custody record;
- one simulated recovery-envelope metadata record;
- one simulated rotation entry;
- one simulated old-archive compatibility entry;
- one simulated incident-response case;
- one simulated emergency-access case;
- six SHA-256 integrity hashes over deterministic no-secret strings;
- eight negative/fail-closed cases;
- explicit no-secret sentinel labels that must never appear in proof output.

## Harness implementation summary

The harness lives at
`scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`. It
accepts a fixture path argument, validates the fixture exists, validates JSON
with Python, checks required schema fields, validates simulated custody,
recovery metadata, rotation, old-archive compatibility, incident response,
emergency access, operator runbook markers, operation counters, forbidden
operations, claim boundaries, and required markers, recomputes fixture hashes,
executes fail-closed negative cases in memory, writes a redacted text proof
artifact under `/srv/qbuild/tmp/NA-0361_*`, and exits nonzero on incomplete or
tampered fixture data.

The harness does not call backup, restore, off-host, deploy, rollback, key,
passphrase, service, systemd, mount, network, or local backup mutation commands.

## Harness execution and marker evidence

Local execution:

```text
bash -n scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json
```

Result:

- proof artifact:
  `/srv/qbuild/tmp/NA-0361_key_custody_recovery_no_secret.qShP9n/na0361_key_custody_recovery_no_secret_proof.txt`
- proof artifact size: 1999 bytes
- proof artifact SHA-256:
  `8118122e913329a4cd5d8934532148fe4e564c1cfc54c1e6144af114445105cf`
- `KEY_CUSTODY_RECOVERY_SECRET_FINDING_COUNT 0`
- `NA0361_SENTINEL_LEAK_FINDING_COUNT 0`
- `NA0361_OPERATION_EXECUTED_COUNT 0`
- `NA0361_NEGATIVE_CASES_PASSED 8`

All required NA0361 markers were emitted, including
`NA0361_METADATA_RUNTIME_KEY_CUSTODY_RECOVERY_NO_SECRET_OK`.

## Negative/fail-closed fixture validation

The harness executes eight negative cases in memory and requires each to fail
closed:

| Case | Fail-closed proof |
| --- | --- |
| missing custody record | removing `simulated_custody_records` is rejected |
| missing recovery metadata | removing `simulated_recovery_envelope_metadata` is rejected |
| simulated rotation mismatch | replacing the rotation target key ID is rejected |
| missing old-archive compatibility | removing the compatibility matrix is rejected |
| prohibited operation field | setting a real key generation counter nonzero is rejected |
| missing claim boundary | removing `production_readiness` is rejected |
| missing no-secret marker | removing `NA0361_NO_SECRET_MATERIAL_OK` is rejected |
| sentinel leak detection | injecting a sentinel into proof output is detected |

The proof emitted `NA0361_NEGATIVE_CASES_PASSED 8`.

## Artifact redaction, secret-scan, cleanup, and `/srv/qbuild/tmp` proof

The harness writes exactly one proof file under the generated
`/srv/qbuild/tmp/NA-0361_*` directory. The proof is text and rebuildable.

Redaction and scan properties:

- proof artifact is under `/srv/qbuild/tmp`;
- proof artifact contains no no-secret sentinel labels;
- proof artifact contains no private-key, token, credential, or
  passphrase-like pattern;
- fixture sentinel labels are benign labels only and are not emitted into the
  proof artifact;
- no backup payloads, restore payloads, key material, private key content,
  passphrases, recovery-envelope contents, remote endpoints, or credentials are
  written;
- no durable evidence location outside the repo and `/srv/qbuild/tmp` is
  required.

Cleanup behavior is marker-only for this no-secret harness: no staging payloads
are created, and the only temporary output is the redacted proof artifact.

## Backup-plan impact and local-ops dependency decision

No NA-0361 backup-plan update is required. Changed tracked paths stay under
qsl-protocol and proof artifacts stay under `/srv/qbuild/tmp`.

Future real key custody/recovery, recovery-envelope handling, durable
secret-related artifacts, off-host targets, source-list changes, scripts,
timers, fstab, system services, monitoring artifacts, real restore artifacts,
backup, restore, deploy, rollback, or public-claim mutation require explicit
backup-plan and local-ops authorization before execution.

Workflow-support/local-ops improvements would reduce repeated friction,
especially qstart/qresume fast-forwarding, response-file writing, bounded
polling helpers, validation profiles, per-directive allow-files, read-only
source/authority helpers, claim-boundary scanners, and directive/response/
journal indexing. They are not implemented by NA-0361. Local workflow support
remains a future lane unless selected by a later directive.

## Public-ingress/timing/traffic-shape boundary

NA-0361 changes no public ingress and no runtime traffic behavior. It does not
prove hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, or padding that hides all metadata.

## External-review sensitivity

External review remains incomplete. No-secret key custody/recovery harness
evidence is not external-review proof, not production proof, and not
public-internet proof. Stronger claims require real key custody evidence, real
key recovery evidence, off-host backup evidence, real restore-drill evidence,
service evidence, deployment evidence, monitoring/log evidence, rollback
evidence, and review evidence.

## Public claim boundary

NA-0361 introduces no production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, local continuity as complete disaster recovery, off-host
encrypted backup completion, real restore-drill completion, real key custody
implementation, or real key recovery implementation claim.

No website or public docs update is required.

## Successor selection

Selected successor:

`NA-0362 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Implementation Authorization Plan`

Rationale:

- NA-0361 delivered executable no-secret custody/recovery boundary evidence.
- Real key custody and recovery are still not implemented, but the no-secret
  harness makes the next safe step an implementation authorization plan for the
  off-host target/tool lane rather than a real backup, restore, or key-handling
  operation.
- The successor can decide whether the off-host target/tool implementation path
  is ready, still blocked by real key custody/recovery, or should defer to
  local-ops/blocker work.

## Rejected alternatives

- `Metadata Runtime Restore Drill Isolated Restore Authorization Plan`:
  deferred until off-host target/tool authorization and real key/recovery
  prerequisites are resolved.
- `Metadata Runtime Key Custody / Key Recovery Blocker Resolution`: not
  selected because NA-0361 has no no-secret harness blocker; real-key blockers
  remain documented future gates.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: useful but not
  the direct metadata-runtime dependency after this harness.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred until
  off-host backup and restore evidence is stronger.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because no
  public-claim mutation occurred.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  off-host backup, restore, and review boundaries are stronger.

## Backup-plan impact statement

Current NA-0361 changes do not require a backup-plan update. Future real keys,
real recovery envelopes, off-host targets, real restore artifacts, monitoring
artifacts, source-list changes, backup scripts, timers, fstab, system services,
backup, restore, deploy, rollback, and public-claim mutation remain
backup-plan gated.

## Next recommendation

Close out NA-0361 after PR merge and green required checks, then restore
exactly one READY item:

`NA-0362 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Implementation Authorization Plan`
