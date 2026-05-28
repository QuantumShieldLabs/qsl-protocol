Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-28

# NA-0374 Closeout / NA-0375 Restoration Test Plan

## Scope

This test plan covers the governance-only closeout that marks NA-0374 DONE and
restores:

`NA-0375 -- Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Operator Input`

The closeout must not implement NA-0375 and must not perform target setup,
remote connection, host-key scan, credential handling, secret handling, backup,
restore, deploy, rollback, key generation, repository init, tool installation,
or backup-script/timer/fstab mutation.

## Expected Queue State

- READY_COUNT is exactly `1`.
- READY item is `NA-0375`.
- NA-0374 is `DONE`.
- D-0730 exists once.
- D-0731 exists once.
- D-0732 is absent.

## Required Evidence

- PR #1010 is closed unmerged and is not merge evidence.
- PR #1011 is the clean replacement evidence PR and merged as
  `36529a4ab387`.
- Post-merge `public-safety` is green on `36529a4ab387`.
- NA-0374 evidence records `OPERATOR_RESPONSE_STILL_ABSENT_AFTER_FOLLOW_UP`.
- NA-0375 is selected because operator response, target candidate, host
  identity, capacity/retention, monitoring/runbook, and real-operation
  prerequisites remain absent or blocked.

## Local Validation Commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0374_closeout_restore_na0375_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0374_closeout_restore_na0375_testplan.md
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

## Claim-Boundary Checks

The closeout must not introduce any affirmative claim of:

- no production readiness.
- no public-internet readiness.
- no external review completion.
- no anonymity, metadata-free behavior, or untraceable behavior.
- no hidden attachment size, hidden timing, or hidden traffic shape.
- no configured target.
- no verified host identity.
- no off-host backup completion.
- no real restore completion.
- no disaster recovery completion.
- no real key custody/recovery implementation.

All such wording must remain negated, prohibited, or future-gated.
