Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-28

# NA-0375 Closeout / NA-0376 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Scope

This test plan covers the governance-only closeout that marks NA-0375 DONE and
restores:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`

The closeout must not implement NA-0376 and must not perform runtime, service,
protocol, crypto, qsc/qsp, dependency, workflow, website/public-doc, backup,
restore, deploy, rollback, key, credential, recovery-envelope, target setup,
remote connection, host-key scan, known_hosts mutation, repository init, tool
installation, backup-script/timer/fstab mutation, qsl-server mutation,
qsl-attachments mutation, qshield runtime mutation, qsc-desktop mutation, or
public-claim expansion.

## Expected Queue State

- READY_COUNT is exactly `1`.
- READY item is `NA-0376`.
- NA-0375 is `DONE`.
- D-0732 exists once.
- D-0733 exists once.
- D-0734 is absent.

## Required Evidence

- PR #1013 is merged as `aada0bc6e0c2`.
- PR #1013 public-safety recovered after one bounded rerun and passed.
- Post-merge `public-safety` is green on `aada0bc6e0c2`.
- NA-0375 evidence records `OPERATOR_RESPONSE_REQUIRED_STOP`.
- No deliberate no-secret operator response exists.
- The off-host target/host-identity chain remains blocked pending operator
  input.
- NA-0376 is selected as a planning/authorization lane only.

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
  --allowed tests/NA-0375_closeout_restore_na0376_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0375_closeout_restore_na0376_testplan.md
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
- disaster recovery completion.
- real key custody/recovery implementation.

All such wording must remain negated, prohibited, or future-gated.
