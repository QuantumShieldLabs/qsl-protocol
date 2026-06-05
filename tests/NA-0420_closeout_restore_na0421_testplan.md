Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0420 Closeout Restore NA-0421 Testplan

Goals: G4

## Objective

Validate the closeout-only lane that marks NA-0420 DONE after the scheduled
backup verification PR merged and post-merge public-safety passed, then
restores
`NA-0421 -- QSL Backup Log Code 23 Clean Follow-Up / Status Refresh Authorization Plan`
as the sole READY successor.

This closeout does not implement NA-0421.

## Protected invariants

- NA-0420 evidence PR #1109 is merged before closeout.
- Post-merge public-safety is green on the NA-0420 merge commit.
- D-0828 exists once before closeout.
- D-0829 is added once by closeout.
- NA-0420 becomes DONE.
- NA-0421 is the exact sole READY successor.
- NA-0421 preserves clean follow-up/status-refresh authorization scope.
- No backup or restore is run.
- qsl-backup is not mutated.
- The rollback subtree is not mutated.
- Backup status and backup plan files are not mutated by this closeout.
- Same-host continuity caveats remain explicit.
- No public overclaim is introduced.

## Allowed scope

Allowed qsl-protocol mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0420_closeout_restore_na0421_testplan.md`

## Forbidden scope

- Implementing NA-0421.
- Running qwork, qstart, or qresume.
- Running sudo.
- Running generated packet scripts.
- Running backup.
- Running restore.
- Mutating `/usr/local/sbin/qsl-backup`.
- Mutating `/backup/qsl`.
- Mutating rollback subtree paths.
- Mutating backup status or backup plan files.
- Mutating systemd units, timers, fstab, source lists, retention, or backup
  scripts.
- Mutating qwork/qstart/qresume/qshell.
- Mutating runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE.
- Creating durable Director State Index output.
- Creating public technical paper content or public readiness claims.

## Preconditions

Required before closeout patch:

- PR #1109 is MERGED.
- PR #1109 merge commit is present on `origin/main`.
- Post-merge public-safety on the PR #1109 merge commit is completed success.
- Queue helper reports READY_COUNT `1` and READY NA-0420.
- Decision helper reports latest D-0828 and duplicate count zero.
- D-0829 is absent before closeout.

## Queue patch requirements

NA-0420 block requirements:

- Status changes from READY to DONE.
- Implementation note records PR #1109, merge commit, classification
  `CODE23_REMEDIATION_VERIFIED_CLEAN_SCHEDULED_LOG`, and selected successor
  NA-0421.
- The note preserves no-backup/no-restore/no-sudo/no-generated-script
  execution, no qsl-backup mutation, no rollback subtree mutation by Codex, no
  status/plan mutation, same-host caveat, and public-claim boundaries.

NA-0421 block requirements:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective authorizes a bounded local-ops status/plan refresh after NA-0420
  verified a clean scheduled same-host log.
- Allowed scope is governance evidence/testplan, D/traceability/journal, and
  read-only inspection of backup artifacts, operator result, and qsl-backup.
- Future local status/plan mutation is allowed only if the NA-0421 live scope
  explicitly authorizes exact files and exact wording.
- Forbidden scope includes backup execution, restore execution, qsl-backup
  mutation, rollback subtree mutation, backup status/plan mutation unless
  exact future scope authorizes it, runtime/dependency/workflow/public-doc/
  website mutation, public technical paper work, and public overclaims.
- Acceptance criteria preserve same-host, no-backup, no-restore,
  no-qsl-backup-mutation, no-rollback-mutation, one-READY, and public-safety
  gates.

## Validation bundle

Required commands before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
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

Additional gates:

- scope guard confirms only the five allowed qsl-protocol paths changed;
- PR body preflight passes;
- goal-lint passes;
- public-safety is green before merge and after merge.

## Expected result

After merge, queue proof reports READY_COUNT `1` and READY NA-0421. D-0829
exists once. NA-0420 is DONE. NA-0421 remains unimplemented and limited to
clean follow-up/status-refresh authorization.
