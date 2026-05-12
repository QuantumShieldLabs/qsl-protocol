Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12

# NA-0271 Closeout and NA-0272 Restoration Test Plan

## Objective

Close NA-0271 after the qsl-attachments read-only audit/test-harness design
merged, then restore exactly one READY successor: NA-0272, qsl-server
docs/API contract repair and hardening harness prep.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0271 is DONE and references PR #795, D-0512, and D-0513.
- NA-0272 is READY and remains docs/API contract repair and harness prep only.
- No qsl-server or qsl-attachments implementation path changes occur.
- No protocol, wire, crypto, auth, or state-machine semantics change.
- No production readiness claim is introduced.
- public-safety remains required and green.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0271_closeout_restore_na0272_testplan.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, or service code
- branch-protection or public-safety/check configuration

## Queue Expectations

- Before closeout, main has READY_COUNT 1 with READY NA-0271.
- After closeout, local and PR heads have READY_COUNT 1 with READY NA-0272.
- NA-0271 remains DONE with audit/design PR #795 evidence.
- NA-0272 contains the approved successor scope and must-protect invariants.

## Decision Expectations

- D-0512 exists once and records the NA-0271 qsl-attachments read-only audit
  and test-harness design.
- D-0513 exists once and records NA-0271 closeout plus NA-0272 restoration.
- No duplicate decision IDs exist.

## Traceability Expectations

- TRACEABILITY records PR #795 head and merge evidence.
- TRACEABILITY links D-0513 and this test plan.
- TRACEABILITY states that no qsl-server or qsl-attachments implementation
  change is authorized or performed.
- TRACEABILITY records NA-0272 as the sole READY successor.

## Validation Commands

Run these before PR creation and after any closeout patch adjustment:

```bash
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0271_closeout_restore_na0272_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

## Overclaim Scan

Scan added lines for prohibited production or cryptographic overclaims. Any
match must be explicitly negated, listed as prohibited wording, or listed as a
future/unproven category.

## CI Expectations

- PR goal-lint passes with a standalone `Goals: G1, G3, G4, G5` line.
- Required protected checks complete normally.
- CodeQL neutral is acceptable only under the existing helper allowance.
- public-safety remains required and green before merge and after merge.
- Docs/governance-only closeout scope should skip `qsc-linux-full-suite` and
  `macos-qsc-full-serial` under NA-0262A while preserving
  `qsc-adversarial-smoke`.

## Success Criteria

- NA-0271 is DONE.
- NA-0272 is READY.
- D-0513 exists once.
- Scope guard reports only allowed closeout paths.
- Link-check and leak-scan pass.
- Required PR checks and post-merge public-safety are accepted.
