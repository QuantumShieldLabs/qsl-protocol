Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0419 Closeout Restore NA-0420 Testplan

Goals: G4

## Objective

Validate the closeout-only lane that marks NA-0419 DONE after the operator
packet execution verification PR merged and post-merge public-safety passed,
then restores
`NA-0420 -- QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Plan`
as the sole READY successor.

This closeout does not implement NA-0420.

## Protected Invariants

- NA-0419 evidence PR #1107 is merged before closeout.
- Post-merge public-safety is green on the NA-0419 merge commit.
- D-0826 exists once before closeout.
- D-0827 is added once by closeout.
- NA-0419 becomes DONE.
- NA-0420 is the exact sole READY successor.
- NA-0420 preserves scheduled-log/manifest verification scope.
- No backup or restore is run.
- qsl-backup is not mutated.
- The rollback subtree is not mutated.
- Backup status and backup plan files are not mutated.
- Same-host continuity caveats remain explicit.
- No public overclaim is introduced.

## Allowed Scope

Allowed qsl-protocol mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0419_closeout_restore_na0420_testplan.md`

## Forbidden Scope

- Implementing NA-0420.
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

- PR #1107 is MERGED.
- PR #1107 merge commit is present on `origin/main`.
- Post-merge public-safety on the PR #1107 merge commit is completed success.
- Queue helper reports READY_COUNT `1` and READY NA-0419.
- Decision helper reports latest D-0826 and duplicate count zero.
- D-0827 is absent before closeout.

## Queue Patch Requirements

NA-0419 block requirements:

- Status changes from READY to DONE.
- Implementation note records PR #1107, merge commit, classification
  `CODE23_REMEDIATION_APPLIED_PENDING_SCHEDULED_BACKUP_PROOF`, and selected
  successor NA-0420.
- The note preserves no-backup/no-restore/no-sudo/no-generated-script
  execution, no qsl-backup mutation, no rollback subtree mutation by Codex, no
  status/plan mutation, same-host caveat, and public-claim boundaries.

NA-0420 block requirements:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective is scheduled same-host log/manifest verification after operator
  remediation.
- Allowed read-only paths include `/backup/qsl/logs`,
  `/backup/qsl/manifests`, packet-local `operator_result`, and
  `/usr/local/sbin/qsl-backup`.
- Forbidden scope includes backup execution, restore execution, qsl-backup
  mutation, rollback subtree mutation, status/plan mutation unless later exact
  scope authorizes it, runtime/dependency/workflow/public-doc/website mutation,
  public technical paper work, and public overclaims.
- Acceptance criteria preserve same-host, no-backup, no-restore,
  no-qsl-backup-mutation, no-rollback-mutation, one-READY, and public-safety
  gates.

## Validation Bundle

Required commands before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main --paths ...
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

## Expected Result

After merge, queue proof reports READY_COUNT `1` and READY NA-0420. D-0827
exists once. NA-0419 is DONE. NA-0420 remains unimplemented and limited to
scheduled log/manifest verification.
