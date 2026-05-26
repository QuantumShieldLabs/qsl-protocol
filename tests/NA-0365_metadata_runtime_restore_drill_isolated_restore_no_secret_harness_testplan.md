Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-26

# NA-0365 Metadata Runtime Restore Drill Isolated Restore No-Secret Harness Testplan

## Objective

Validate that NA-0365 adds only qsl-protocol no-secret isolated restore fixture
and harness evidence, with simulated metadata relationships, fail-closed
negative cases, no-secret proof under `/srv/qbuild/tmp`, explicit backup-plan
impact, and no real restore, target, mount, copy, backup, off-host, key, secret,
deploy, rollback, service, runtime, dependency, workflow, website, or public
claim mutation.

## Protected invariants

- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, key schedules, dependencies,
  workflows, README, START_HERE, docs/public, website, branch protection,
  public-safety configuration, backup scripts, timers, fstab, service units,
  restore targets, off-host targets, deployment scripts, rollback scripts,
  local backup configuration, and production service behavior are not changed.
- No real off-host setup, remote connection, repository initialization, tool
  installation, backup, restore, restore target creation/mount/copy, deploy,
  rollback, key generation, key upload, passphrase collection, private-key
  inspection, recovery-envelope content creation, or secret material handling
  occurs.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Dry-run restore evidence is not real restore execution.
- No-secret custody/recovery evidence is not real key custody or real key
  recovery implementation.
- The no-secret target/tool harness is not real off-host backup.
- The no-secret isolated restore harness is not real restore execution.

## Allowed scope

- `inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh`
- `docs/governance/evidence/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness.md`
- `tests/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server, qsl-attachments, qshield runtime, qsc/qsp, protocol, crypto,
  key-schedule, dependency, workflow, website, README, START_HERE, docs/public,
  service, deployment, backup script, timer, fstab, source-list, off-host
  destination, restore, rollback, purge, branch-protection, or public-safety
  configuration mutation.
- Backup execution, restore execution, restore target creation, restore target
  mount, restore payload copy, key generation, key upload, passphrase
  collection, private-key inspection, secret material handling,
  recovery-envelope content creation, repository initialization, remote/off-host
  backup tooling, deploy, rollback, or tool installation.

## Prior isolated restore authorization review requirements

Evidence must review live NA-0365 scope, NA-0364 isolated restore
authorization, NA-0363 no-secret off-host target/tool harness, NA-0361
no-secret key custody / key recovery harness, NA-0359 no-secret restore-drill
dry-run harness, qsl-server PR #56, qsl-attachments PR #37, local
backup/tool/key/off-host/restore posture, public-claim boundaries, and stop
conditions.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record local path/SHA,
remote default branch evidence if available, PR #56 / PR #37 merge status,
latest main CI status, viewer permission, branch protection, open PR list, and
classifications:

- `FRESH_SOURCE` or exact stale/unknown status;
- `COMPLETE_AUTHORITY` or exact authority blocker;
- `COMPLETE_CI` or exact CI blocker.

## Local backup/tool/key/off-host/restore evidence refresh requirements

Record `/backup/qsl` mount status, snapshot list, manifests/logs availability,
backup status file, backup plan file, safe `qsl-backup` syntax/preflight/list
results, installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`,
`ssh`, and `rsync`, no-secret restore dry-run harness status, no-secret key
custody/recovery harness status, no-secret target/tool harness status, real
restore status, real key custody/recovery status, off-host target/tool status,
local history folder presence, backup coverage, and D132 protection status.

## Fixture schema requirements

The fixture must include schema version, artifact class, source
classification, local backup classification, key custody classification,
off-host target/tool classification, restore classification, restore mode,
restore target mode, simulated restore target metadata, simulated manifest
metadata, simulated checksum metadata, simulated old-archive compatibility
metadata, simulated cleanup metadata, simulated monitoring/alert metadata,
simulated operator-runbook metadata, integrity hashes, operation counters,
no-secret sentinels, expected validation outcomes, tamper cases, forbidden
operations, claim boundaries, required markers, backup-plan impact, qsl-server
boundary, qsl-attachments boundary, qshield demo boundary, and selected
successor.

The fixture must contain only benign fake values and no real backup payloads,
real restore paths, real mount paths, real copied payload paths, real remote
hosts, real endpoints, real usernames, real credentials, real keys, real
passphrases, private key text, tokens, real repository names, real off-host
paths, private material paths, raw secret material, or unredacted sensitive
operational data.

## Harness behavior requirements

The harness must:

- be `bash -n` clean;
- accept a fixture path argument;
- validate the fixture exists;
- validate JSON with Python;
- validate required schema fields;
- validate simulated isolated restore target metadata;
- validate simulated manifest/checksum restore relationships;
- validate simulated old-archive compatibility metadata;
- validate simulated cleanup metadata;
- validate simulated monitoring/alert metadata;
- validate simulated operator-runbook metadata;
- validate forbidden operations and operation counters;
- validate claim boundaries;
- validate negative cases fail closed;
- write proof only under `/srv/qbuild/tmp/NA-0365_*`;
- emit all required markers;
- exit nonzero on tampered or incomplete fixtures;
- avoid backup, restore, off-host, network, key, passphrase, secret, service,
  systemd, mount, copy, repository, tool-installation, deploy, rollback, and
  local backup mutation commands.

## Marker requirements

The harness must emit:

- `NA0365_ISOLATED_RESTORE_AUTHORIZATION_OK`
- `NA0365_NO_SECRET_ISOLATED_RESTORE_HARNESS_OK`
- `NA0365_NO_SECRET_RESTORE_TARGET_HARNESS_OK`
- `NA0365_SIMULATED_RESTORE_TARGET_OK`
- `NA0365_SIMULATED_MANIFEST_CHECKSUM_RESTORE_OK`
- `NA0365_SIMULATED_OLD_ARCHIVE_COMPATIBILITY_OK`
- `NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK`
- `NA0365_BACKUP_PLAN_IMPACT_OK`
- `NA0365_NO_REAL_RESTORE_TARGET_CREATION_OK`
- `NA0365_NO_MOUNT_OK`
- `NA0365_NO_COPY_OK`
- `NA0365_NO_REAL_BACKUP_OK`
- `NA0365_NO_REAL_RESTORE_OK`
- `NA0365_NO_KEY_GENERATION_OK`
- `NA0365_NO_PASSPHRASE_COLLECTION_OK`
- `NA0365_NO_SECRET_MATERIAL_OK`
- `NA0365_NO_RESTORE_DRILL_COMPLETE_CLAIM_OK`
- `NA0365_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0365_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0365_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0365_METADATA_RUNTIME_ISOLATED_RESTORE_NO_SECRET_OK`

## Negative/fail-closed requirements

The harness must prove these tamper cases fail closed:

- missing restore target metadata;
- missing manifest;
- checksum mismatch;
- missing old-archive compatibility;
- missing cleanup;
- missing monitoring/alert;
- prohibited operation field;
- mount attempted;
- copy attempted;
- missing claim boundary;
- missing no-secret marker;
- sentinel leak detection.

## Artifact redaction/secret-scan requirements

Proof artifacts must be text or JSONL, must live under `/srv/qbuild/tmp`, must
not contain no-secret sentinel strings, and must scan clean for common
private-key, token, passphrase, and credential-shaped patterns. The final proof
must record `ISOLATED_RESTORE_SECRET_FINDING_COUNT 0` and zero counts for real
target creation, mount, copy, backup, restore, key, off-host, deploy, rollback,
and local backup mutation.

## Backup-plan impact requirements

Evidence must decide whether NA-0365 requires a backup-plan update. Expected:
`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW` if durable changes stay in tracked
qsl-protocol paths and proof artifacts are temporary under `/srv/qbuild/tmp`.

Evidence must record that future real restore target, durable restored
artifact, key material, recovery envelope, off-host target, source-list,
script/timer/fstab/system-service, monitoring artifact, backup, restore,
deploy, rollback, public-claim mutation, or durable local history/index work
requires backup-plan and local-ops authorization before execution.

## Public-ingress/timing/traffic-shape boundary requirements

Confirm no public ingress, timing, traffic-shape, padding, cover-traffic,
website, service, public-doc, or public-claim behavior is changed. Confirm no
claim states that attachment size, timing metadata, traffic shape, or all
metadata is hidden.

## External-review boundary requirements

Confirm external review remains incomplete and no service-local, no-secret,
demo, dry-run, key, target/tool, isolated-restore, or authorization evidence is
presented as external-review completion.

## Claim-boundary requirements

Confirm no production-readiness, public-internet-readiness, anonymity,
metadata-free, untraceable, hidden-size, hidden-timing, hidden-traffic-shape,
off-host-backup-complete, disaster-recovery-complete, restore-drill-complete,
real-restore-complete, real-key-custody-implemented, or
real-key-recovery-implemented claim is introduced.

## Workflow-support deferral requirements

Record whether qstart/qresume fast-forwarding, response-file writing, bounded
polling helpers, machine-readable manifests, validation profiles,
per-directive allow-files, source/authority helpers, claim-boundary scanners,
directive/response/journal indexes, and local history backup coverage would
reduce friction. Do not implement workflow-support items in NA-0365.

## Required local checks

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- `bash -n scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh`
- `python3 -m json.tool inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json >/dev/null`
- `bash scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with the exact allowed NA-0365 paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `bash scripts/ci/classify_ci_scope.sh <changed paths>`
- changed-line overclaim scan for high-risk claim phrases.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- Required metadata runtime predecessor harnesses.
- Feasible qshield/qsc/formal/refimpl validation from the active directive.

## CI expectations

- PR body has `Goals: G1, G2, G3, G4, G5` near the top.
- Required checks attach and complete successfully.
- `public-safety` remains required by branch protection and completes success.
- No admin bypass, squash, rebase, direct push, branch deletion, or
  branch-protection change is used.

## Successor handoff

After the implementation PR merges and post-merge public-safety is green,
closeout may restore exactly one successor:

`NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool Blocker Resolution`

Closeout must not implement NA-0366.
