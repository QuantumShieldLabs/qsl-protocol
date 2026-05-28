Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-28

# NA-0377 Closeout / NA-0378 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Scope

This test plan covers the governance-only closeout that marks NA-0377 DONE and
restores:

`NA-0378 -- QSL Local Ops qstart/qresume Fast-Forward Guard Implementation Harness`

The closeout must not implement NA-0378 and must not perform runtime, service,
protocol, crypto, qsc/qsp, dependency, workflow, website/public-doc, backup,
restore, deploy, rollback, key, credential, recovery-envelope, target setup,
remote connection, host-key scan, known_hosts mutation, repository init, tool
installation, backup-script/timer/fstab mutation, qstart/qresume mutation,
helper implementation, response-writer implementation, polling-helper
implementation, validation-profile implementation, directive-manifest
implementation, allow-file implementation, history-index creation,
claim-scanner implementation, qsl-server mutation, qsl-attachments mutation,
qshield runtime mutation, qsc-desktop mutation, `/home/victor/work/qsl/codex`
history-root mutation, or public-claim expansion.

## Expected Queue State

- READY_COUNT is exactly `1`.
- READY item is `NA-0378`.
- NA-0377 is `DONE`.
- D-0736 exists once.
- D-0737 exists once.
- D-0738 is absent.

## Required Evidence

- PR #1017 is merged as `697c88e746e1`.
- PR #1017 head is `9e8654d68663`.
- Post-merge `public-safety` is green on `697c88e746e1`.
- NA-0377 evidence records local-ops implementation authorization only.
- NA-0378 is selected as the qstart/qresume fast-forward guard implementation
  harness.
- The closeout does not implement NA-0378.

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
  --allowed tests/NA-0377_closeout_restore_na0378_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0377_closeout_restore_na0378_testplan.md
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

## Claim-Boundary Checks

The closeout must not introduce any affirmative claim of:

- production readiness.
- public-internet readiness.
- external review completion.
- anonymity, metadata-free behavior, or untraceable behavior.
- hidden attachment size, hidden timing, or hidden traffic shape.
- configured target.
- verified host identity.
- off-host backup completion.
- real restore completion.
- complete disaster recovery.
- real key custody/recovery implementation.
- implemented local-ops tooling.

All such wording must remain negated, prohibited, or future-gated.

## Successor Handoff

NA-0378 must begin from refreshed main, prove the qstart/qresume source,
authority, and backup boundary again, and stop if `/srv/qbuild/tools/qshell.sh`
or any required local backup/source state is dirty, stale, absent, or outside
the exact future scope selected by NA-0377.
