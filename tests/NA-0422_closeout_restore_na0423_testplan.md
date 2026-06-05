Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0422 Closeout Restore NA-0423 Testplan

Goals: G4

## Objective

Validate the closeout-only lane that marks NA-0422 DONE after the clean
status/plan refresh PR merged and post-merge public-safety passed, then
restores
`NA-0423 -- QSL Domain Stewardship / Director Workflow Governance Authorization Plan`
as the sole READY successor.

This closeout does not implement NA-0423.

## Protected invariants

- NA-0422 evidence PR #1113 is merged before closeout.
- Post-merge public-safety is green on the NA-0422 merge commit.
- D-0832 exists once before closeout.
- D-0833 is added once by closeout.
- NA-0422 becomes DONE.
- NA-0423 is the exact sole READY successor.
- Lead Director final directive authority is preserved.
- Exactly one READY item remains.
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
- `tests/NA-0422_closeout_restore_na0423_testplan.md`

## Forbidden scope

- Implementing NA-0423.
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
- Creating public technical paper content or public readiness claims.
- Creating multiple independent autonomous Directors.
- Allowing more than one READY item.
- Changing branch protection.
- Handling secret material.

## Preconditions

Required before closeout patch:

- PR #1113 is MERGED.
- PR #1113 merge commit is present on `origin/main`.
- Post-merge public-safety on the PR #1113 merge commit is completed success.
- Queue helper reports READY_COUNT `1` and READY NA-0422.
- Decision helper reports latest D-0832 and duplicate count zero.
- D-0833 is absent before closeout.

## Queue patch requirements

NA-0422 block requirements:

- Status changes from READY to DONE.
- Implementation note records PR #1113, merge commit, D-0832, exact local files
  updated, SHA256 prefixes, rollback path, clean scheduled log/manifest paths,
  code 23 cleared wording, Codex ops manifest presence, and selected successor
  NA-0423.
- The note preserves no-qwork/no-qstart/no-qresume/no-sudo/no-generated-script
  execution, no backup/restore execution, no qsl-backup mutation, no rollback
  subtree mutation by Codex, no status/plan mutation by closeout, same-host
  caveat, and public-claim boundaries.

NA-0423 block requirements:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective defines advisory domain stewardship while preserving Lead Director
  final authority and the exactly-one-READY invariant.
- Allowed scope is qsl-protocol governance evidence/testplan plus D/trace/journal
  and read-only inspection of governance/local-ops evidence.
- Forbidden scope includes multiple independent autonomous Directors, more than
  one READY item, branch-protection changes, runtime/dependency/workflow/public
  mutation, backup/restore/qsl-backup mutation, qwork mutation, public technical
  paper work, and public overclaims.
- Acceptance criteria preserve advisory-only steward roles unless explicitly
  authorized otherwise, Lead Director authority, one-READY, public-claim
  boundaries, no runtime/dependency/workflow/public/backup mutation, and
  public-safety gates.

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
- changed-line positive-overclaim scan reports no positive public overclaim;
- classifier passes;
- PR body preflight passes;
- goal-lint passes;
- public-safety is green before merge and after merge.

## Expected result

After merge, queue proof reports READY_COUNT `1` and READY NA-0423. D-0833
exists once. NA-0422 is DONE. NA-0423 remains implementation-pending and
limited to stewardship governance authorization.
