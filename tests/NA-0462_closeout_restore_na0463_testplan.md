Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0462 Closeout Restore NA-0463 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0462 is closed only after its evidence PR merged and post-merge public-safety completed success, and that NA-0463 is restored as the sole READY successor without implementing NA-0463.

## Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0462_closeout_restore_na0463_testplan.md`

Forbidden closeout changes:

- qsc source or executable implementation tests.
- runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup/restore/local-ops, qsl-backup, backup status, backup plan, rollback, backup tree, or branch-protection paths.
- any implementation of NA-0463.
- any A2 pre-mutation no-mutation claim.
- any public-readiness, production-readiness, external-review-complete, crypto-complete, signature-complete, identity-complete, RNG-failure-complete, provider-RNG-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Required Checks

1. Queue proof before merge:
   - `READY_COUNT 1`
   - `READY NA-0463`
   - NA-0462 is DONE
2. Decision proof:
   - D-0911 exists once.
   - D-0912 exists once.
   - D-0913 is absent before future NA-0463 implementation work.
   - duplicate decision count is zero.
3. Scope proof:
   - changed paths are exactly the five allowed closeout paths.
   - no runtime, crypto, dependency, Cargo, lockfile, workflow, executable implementation test, fuzz, vector, formal, refimpl, service, public-surface, qwork, backup, restore, qsl-backup, status, plan, rollback, or backup tree path is changed.
4. Evidence proof:
   - qsl-protocol PR #1193 is merged at `3e4a1bab743d`.
   - post-merge public-safety is success on `3e4a1bab743d`.
   - selected successor is `NA-0463 -- QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness`.
5. Boundary proof:
   - no NA-0463 implementation is present in the closeout patch.
   - future NA-0463 exact paths are preserved.
   - future NA-0463 must prove no A2 output and sanitized `sig_sign_failed` without claiming A2 pre-mutation no-mutation.
   - cargo audit evidence remains dependency-health evidence only.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 tools/goal_lint.py --event-path "$EVENT_JSON"
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

## Acceptance Criteria

- NA-0462 is DONE.
- NA-0463 is the only READY item.
- D-0912 records NA-0462 closeout and NA-0463 restoration.
- TRACEABILITY and the rolling journal record closeout evidence.
- changed paths are limited to the closeout allowlist.
- no implementation mutation occurs.
- no public claim expands beyond bounded internal governance evidence.
