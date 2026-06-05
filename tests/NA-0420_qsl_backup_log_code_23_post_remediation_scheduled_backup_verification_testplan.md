Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0420 QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Testplan

Goals: G4

## Objective

Validate that NA-0420 inspects the first scheduled same-host backup log and
manifest pair after the NA-0418 operator permission remediation, classifies the
code 23 result truthfully, and selects the exact NA-0421 successor without
running backup, restore, sudo, qwork, qstart, qresume, or generated packet
scripts.

## Protected invariants

- qwork proof files are read and verified without rerunning qwork.
- Queue starts with exactly one READY item: NA-0420.
- D-0827 exists once; D-0828 is absent at start.
- Operator remediation is not sufficient without scheduled log proof.
- qsl-backup remains unchanged.
- Codex does not mutate the rollback subtree.
- Codex does not run sudo, backup, restore, or generated packet scripts.
- Backup status and backup plan files remain read-only.
- Same-host continuity caveats remain explicit.
- No public overclaim is introduced.

## Allowed scope

Allowed qsl-protocol mutation paths:

- `docs/governance/evidence/NA-0420_qsl_backup_log_code_23_post_remediation_scheduled_backup_verification_plan.md`
- `tests/NA-0420_qsl_backup_log_code_23_post_remediation_scheduled_backup_verification_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local proof output root:

- `/srv/qbuild/tmp/NA0420_post_remediation_scheduled_backup_verification_retry_<timestamp>/`

Allowed read-only local paths:

- qwork proof files under `/srv/qbuild/work/NA-0420/.qwork/`
- packet-local operator result files
- the NA-0407 rollback directory
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

## Forbidden scope

- Running qwork, qstart, or qresume.
- Running sudo.
- Running generated packet scripts.
- Running backup.
- Running restore.
- Mutating `/usr/local/sbin/qsl-backup`.
- Mutating `/backup/qsl`.
- Mutating the NA-0407 rollback subtree.
- Mutating backup status or backup plan files.
- Mutating systemd units, timers, fstab, source lists, retention, or backup
  scripts.
- Mutating qwork/qstart/qresume/qshell.
- Mutating runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE.
- Creating durable Director State Index output.
- Creating public technical paper content or public readiness claims.

## qwork proof gate

Required checks:

- `.kv` proof exists and has `startup_result=OK`, lane `NA-0420`, repo
  `qsl-protocol`, clean-state fields, READY_COUNT `1`, queue top READY
  `NA-0420`, and requested lane status `READY`.
- JSON proof parses and mirrors the required `.kv` fields.
- live `HEAD` and `origin/main` match the proof after fetch.
- PR #1108 is MERGED with merge commit `d56f2643e87d`.

Stop if any proof is missing, stale, malformed, or inconsistent.

## Queue and decision gate

Required checks:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Acceptance:

- READY_COUNT `1`;
- READY NA-0420;
- NA-0419 DONE;
- latest decision D-0827;
- D-0826 once;
- D-0827 once;
- D-0828 absent at start;
- duplicate decision count zero.

## Operator remediation reconfirmation gate

Required checks:

- stat packet-local `operator_result`;
- stat the live rollback directory;
- checksum the expected rollback file if readable without privilege;
- verify qsl-backup checksum still matches the inherited expected value;
- verify exact Codex ops source inclusion count remains `1`.

Acceptance:

- `operator_result` exists;
- live rollback directory is `root:root` mode `2755`;
- readable rollback file checksum matches inherited evidence;
- qsl-backup is unchanged;
- Codex ops source inclusion count is exactly `1`.

## Scheduled log/manifest discovery gate

Required checks:

- identify operator action time as `2026-06-04T16:23:29Z`;
- list logs and manifests under `/backup/qsl`;
- identify all scheduled logs and manifests newer than the operator action;
- identify matched scheduled pairs by stamp.

Acceptance:

- if no scheduled pair exists after operator action, stop with
  `CODE23_REMEDIATION_APPLIED_PENDING_SCHEDULED_BACKUP_PROOF`;
- if one or more pairs exist, inspect the first and latest pair after operator
  action.

## Post-remediation verification gate

Required log counts:

- `rsync error`
- `code 23`
- `Permission denied`
- `NA0407`
- `rollback`
- exact NA-0407 rollback directory reference
- `qsl-backup.preimage`
- other rsync errors
- nonzero-exit marker

Required manifest counts:

- `/home/victor/work/qsl/codex/ops`
- `NA0407`
- `rollback`
- exact NA-0407 rollback directory reference
- `qsl-backup.preimage`

Classification rules:

- clean scheduled log, no new failure:
  `CODE23_REMEDIATION_VERIFIED_CLEAN_SCHEDULED_LOG`;
- same warning persists:
  `CODE23_REMEDIATION_APPLIED_BUT_WARNING_PERSISTS`;
- new backup failure appears:
  `CODE23_REMEDIATION_NEW_BACKUP_FAILURE_FOUND`;
- proof ambiguous:
  `CODE23_REMEDIATION_AMBIGUOUS_SCHEDULED_PROOF`.

## Successor gate

Select exactly one NA-0421 successor:

- clean log:
  `NA-0421 -- QSL Backup Log Code 23 Clean Follow-Up / Status Refresh Authorization Plan`;
- persistent same-path warning:
  `NA-0421 -- QSL Backup Log Code 23 Persistent Warning After Remediation Triage Plan`;
- new failure:
  `NA-0421 -- QSL Backup Log Post-Remediation New Failure Triage Plan`;
- ambiguous proof:
  `NA-0421 -- QSL Backup Log Code 23 Post-Remediation Evidence Ambiguity Resolution Plan`.

Do not implement NA-0421 in the NA-0420 evidence PR.

## Status/plan gate

Inspect read-only:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Acceptance:

- clean scheduled proof may recommend a future status/plan refresh;
- warning persistence preserves caveats and recommends triage;
- new failure recommends triage before status/plan refresh;
- no status/plan mutation is made by NA-0420.

## Validation bundle

Required commands before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional local gates:

- scope guard confirms only the five allowed qsl-protocol paths changed;
- leak scan reports no secret findings;
- overclaim scan reports no positive public overclaim;
- classifier passes;
- PR body preflight and goal-lint pass;
- public-safety remains required and green before merge.

## Expected result

D-0828 records the scheduled pair reviewed, code 23 classification, Codex ops
manifest count, selected NA-0421 successor, no backup/restore execution by
Codex, no qsl-backup mutation, no rollback subtree mutation by Codex, same-host
caveat, and no public overclaim.
