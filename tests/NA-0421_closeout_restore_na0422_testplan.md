Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0421 Closeout Restore NA-0422 Testplan

Goals: G4

## Objective

Validate the closeout-only lane that marks NA-0421 DONE after the status
refresh authorization PR merged and post-merge public-safety passed, then
restores
`NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`
as the sole READY successor.

This closeout does not implement NA-0422.

## Protected invariants

- NA-0421 evidence PR #1111 is merged before closeout.
- Post-merge public-safety is green on the NA-0421 merge commit.
- D-0830 exists once before closeout.
- D-0831 is added once by closeout.
- NA-0421 becomes DONE.
- NA-0422 is the exact sole READY successor.
- NA-0422 preserves exact local status/plan implementation scope.
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
- `tests/NA-0421_closeout_restore_na0422_testplan.md`

## Forbidden scope

- Implementing NA-0422.
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
- Handling secret material.

## Preconditions

Required before closeout patch:

- PR #1111 is MERGED.
- PR #1111 merge commit is present on `origin/main`.
- Post-merge public-safety on the PR #1111 merge commit is completed success.
- Queue helper reports READY_COUNT `1` and READY NA-0421.
- Decision helper reports latest D-0830 and duplicate count zero.
- D-0831 is absent before closeout.

## Queue patch requirements

NA-0421 block requirements:

- Status changes from READY to DONE.
- Implementation note records PR #1111, merge commit, classification
  `STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`, both exact local
  future mutable candidates, and selected successor NA-0422.
- The note preserves no-backup/no-restore/no-sudo/no-generated-script
  execution, no qsl-backup mutation, no rollback subtree mutation by Codex, no
  status/plan mutation by closeout, same-host caveat, and public-claim
  boundaries.

NA-0422 block requirements:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective implements only the exact local status/plan refresh authorized by
  NA-0421.
- Allowed scope includes the exact local status/plan files, qsl-protocol
  governance evidence/testplan, D/traceability/journal, and read-only
  inspection of backup artifacts, operator result, and qsl-backup.
- Forbidden scope includes backup execution, restore execution, qsl-backup
  mutation, rollback subtree mutation, runtime/dependency/workflow/public-doc/
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
- changed-line overclaim scan reports no positive public overclaim;
- classifier passes;
- PR body preflight passes;
- goal-lint passes;
- public-safety is green before merge and after merge.

## Expected result

After merge, queue proof reports READY_COUNT `1` and READY NA-0422. D-0831
exists once. NA-0421 is DONE. NA-0422 remains implementation-pending and
limited to the exact local status/plan refresh authorized by NA-0421.
