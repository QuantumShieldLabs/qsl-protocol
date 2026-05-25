Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0358 Metadata Runtime Restore Drill Implementation Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0358 records the restore-drill implementation authorization
decision after NA-0357 prerequisite planning without executing restore, backup,
deploy, rollback, key handling, off-host setup, local backup mutation, service
repo mutation, runtime mutation, dependency mutation, or public-claim
expansion.

## Protected invariants

- NA-0358 is implementation authorization planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  changed.
- Local backup scripts, timers, fstab, service units, source lists, targets,
  keys, passphrases, remote destinations, backup operations, restore
  operations, restore target creation/mount/copy, deploy operations, rollback
  operations, and purge operations are not changed or executed.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Restore-drill authorization is not restore execution.
- Key custody/key recovery planning is not key custody/recovery
  implementation.
- qsl-server PR #56 remains bounded end-to-end harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo proof remains reference/oracle evidence only.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, hidden all metadata, or padding hiding all metadata.

## Allowed scope

- `docs/governance/evidence/NA-0358_metadata_runtime_restore_drill_implementation_authorization.md`
- `tests/NA-0358_metadata_runtime_restore_drill_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.
- Restore execution, restore target creation, restore target mount, restore
  payload copy, key generation, key upload, passphrase collection, private key
  inspection, secret material handling, repository initialization, or
  remote/off-host backup tooling.

## Prior restore prerequisite review requirements

The evidence must review NA-0357 and confirm:

- NA-0357 selected restore-drill implementation authorization as successor.
- NA-0357 classified local continuity as proven, off-host backup as not ready,
  and key custody/recovery as partial.
- NA-0357 recommended qsl-protocol no-secret dry-run fixture evidence before
  any real restore.
- NA-0357 performed no restore, restore target creation/mount/copy, backup,
  off-host operation, key handling, local backup mutation, service repo
  mutation, runtime mutation, dependency mutation, workflow mutation, website
  mutation, README mutation, START_HERE mutation, or public-doc mutation.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record:

- local path and SHA if present;
- remote default branch SHA;
- PR #56 / PR #37 merge status;
- latest main CI status;
- viewer permission;
- branch protection;
- open PR list;
- classifications: `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, and `COMPLETE_CI`, or
  exact blocker classifications if evidence differs.

## Local backup/key/restore evidence refresh requirements

Record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status and backup plan availability;
- `qsl-backup` syntax/preflight/list read-only results if safe;
- installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`, `ssh`,
  and `rsync`;
- whether off-host encrypted backup is configured or proven;
- whether restore-drill documentation, dry-run restore documentation, isolated
  restore target documentation, manifest/checksum process, key custody, key
  recovery, retention/purge, monitoring/alerting, and operator runbook exist;
- whether Codex directives, requests, journals, ops, responses, and D132 are
  present and/or backup-covered.

## Restore authorization decision requirements

The evidence must decide:

- whether no-secret dry-run restore harness can be authorized now;
- whether isolated real restore can be authorized now;
- whether restore target creation remains blocked;
- whether key custody must precede real restore;
- whether backup-plan update must precede any restore target or durable
  artifact;
- whether qsl-protocol fixture harness can precede live restore;
- whether local-ops workflow/history index must precede implementation;
- the selected exact NA-0359 successor.

## Future implementation bundle requirements

If authorization proceeds, the evidence must define:

- future repository scope;
- future allowed files;
- future forbidden files;
- future commands;
- future tests;
- future artifacts;
- future markers;
- future PR/order;
- future backup-plan update requirements;
- future key-handling requirements;
- future restore target requirements;
- future monitoring/logging requirements;
- future public-claim boundary;
- future stop conditions.

## Dry-run restore authorization requirements

Evaluate and classify:

- qsl-protocol fixture-only dry-run harness;
- qsl-protocol manifest/checksum-only harness;
- qsl-protocol no-secret archive simulation;
- qsl-backup list/preflight wrapper harness;
- qsl-server/qsl-attachments service-local fixture harness;
- no dry-run implementation.

Each option must record safety, secret risk, restore operation risk,
backup-plan impact, confidence gained, confidence not gained, CI feasibility,
and recommended/deferred/rejected status.

## Isolated restore authorization requirements

Evaluate and classify:

- isolated `/srv/qbuild/tmp` target;
- isolated disposable disk;
- isolated non-live qbuild restore target;
- off-host restore to staging machine;
- production root restore;
- no isolated restore.

Each option must record safety, required authorization, key/secret dependency,
backup-plan impact, cleanup requirements, monitoring/logging requirements,
evidence value, and recommended/deferred/rejected status.

## Key dependency requirements

The evidence must decide:

- whether no-secret dry-run implementation can proceed before key custody;
- whether isolated real restore requires key custody implementation;
- whether old-key/archive compatibility blocks isolated restore;
- whether key rotation compatibility must be implemented first;
- whether recovery-envelope implementation must occur first;
- no private-key inspection;
- no passphrase collection;
- no secret logs;
- future stop conditions.

## Manifest/checksum/artifact requirements

The evidence must define:

- future manifest fixture source;
- future checksum fixture source;
- expected validation stages;
- artifact redaction;
- no-secret proof;
- failed validation behavior;
- output artifact location;
- cleanup behavior;
- future evidence retention;
- backup-plan impact;
- future markers.

## Cleanup/monitoring/runbook requirements

The evidence must define:

- failed dry-run cleanup;
- failed isolated restore cleanup if later authorized;
- stale temp path cleanup;
- alert on failed restore drill;
- alert on missing manifest/checksum;
- alert on missing key/recovery proof;
- emergency stop;
- operator verification;
- runbook sections;
- no-secret logs;
- audit artifact summary.

## Backup-plan impact requirements

The evidence must decide:

- whether NA-0358 itself requires a backup-plan update;
- whether future restore-drill implementation needs backup-plan update before
  any restore target/artifact exists;
- whether local workflow-support/history-index backup coverage should precede
  or follow dry-run implementation;
- whether directive/response history index should become NA-0359 or later;
- whether `/home/victor/work/qsl/codex/directives`, `/requests`, `/journals`,
  `/ops`, and `/responses` are present and/or backup-covered;
- whether D132 remains protected and untouched.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that public ingress remains future-gated and that
current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding hiding all metadata.

## External-review boundary requirements

The evidence must state that external review remains incomplete and that
restore-drill implementation authorization is not external-review completion.

## Claim-boundary requirements

The evidence must not claim or imply:

- restore execution;
- disaster recovery completion;
- off-host encrypted backup completion;
- key custody/recovery implementation;
- production readiness;
- public-internet readiness;
- external-review completion;
- anonymity, metadata-free behavior, or untraceable behavior;
- hidden attachment size, hidden timing metadata, hidden traffic shape, hidden
  all metadata, or padding hiding all metadata.

## Workflow-support deferral requirements

The evidence must record whether future local-ops improvements would reduce
friction, including qstart/qresume fast-forward, response-file writer, bounded
polling helper, machine-readable directive manifest, validation profiles,
per-directive allow-file, read-only source/authority helper, claim-boundary
scanner, directive/response/journal index, and backup coverage for history
folders. It must not implement those items.

## Required local checks

- `date --iso-8601=seconds`
- `date -u --iso-8601=seconds`
- `df -BG /srv/qbuild`
- `df -h /backup/qsl || true`
- `git status --porcelain=v1 --branch`
- `git rev-parse origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `bash -n scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh`
- `bash scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json`
- `python3 -m json.tool inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json >/dev/null`
- qshield NA-0339, NA-0337, NA-0335, NA-0331, NA-0329, NA-0327, NA-0324,
  NA-0322, NA-0320, NA-0319, and NA-0318 harnesses if directly runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- metadata runtime plan harness from NA-0315 if directly runnable
- metadata phase-2 identifier/padding harness
- metadata phase-2 sanitized-errors/retention harness
- metadata conformance smoke
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint against the PR body
- classifier proof for the changed path set
- changed-line overclaim scan

## CI expectations

The PR must merge normally only after required checks complete green, including
`public-safety`. No admin bypass, direct push, squash, rebase, delete-branch
flag, branch-protection mutation, or public-safety mutation is allowed.

## Successor handoff

The selected successor is expected to be
`NA-0359 -- Metadata Runtime Restore Drill Dry-Run Implementation Harness`
unless live evidence changes before closeout. NA-0359 must remain bounded to
qsl-protocol no-secret fixture evidence unless a future directive explicitly
authorizes more. NA-0359 must not run a backup, restore, deploy, rollback,
off-host operation, key operation, passphrase operation, restore target
creation, restore target mount, or restored-payload copy.
