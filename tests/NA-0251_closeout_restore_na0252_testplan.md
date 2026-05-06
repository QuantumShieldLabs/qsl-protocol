Goals: G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-06
Replaces: n/a
Superseded-By: n/a

# NA-0251 Closeout And NA-0252 Restoration Test Plan

## Objective

Validate that NA-0251 is closed only after Packet A merged and post-merge public-safety passed, and that exactly one successor, NA-0252, is restored as READY without implementing NA-0252.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0251 is DONE.
- NA-0252 is READY.
- D-0469 and D-0470 exist once each.
- NA-0252 is helper tooling only and must not weaken public-safety.
- No `.github`, public-safety helper/configuration, branch-protection, Cargo, protocol/runtime/crypto/demo/service, qsc/qsl apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, or external website repo implementation changes occur in this closeout lane.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0251_closeout_restore_na0252_testplan.md`
- `docs/governance/evidence/**` only if current repo convention strictly requires it

Validation commands:

```bash
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
```

Expected result: changed paths are only closeout allowed paths.

## Queue Parser Expectation

Run the canonical queue parser after closeout edits.

Expected result:

- `READY_COUNT 1`
- `READY NA-0252 Repo-Local Evidence and CI Recovery Helper Toolkit`
- `NA-0251 DONE Public Website Evidence-Boundary Implementation Handoff`
- NA-0250 through NA-0237 remain DONE.

## Decision Parser Expectation

Run the canonical decision parser after closeout edits.

Expected result:

- D-0110 exists once
- D-0439 through D-0470 exist once each
- duplicate count is zero

## Closeout Evidence Checks

Confirm `NEXT_ACTIONS.md` records:

- Packet A PR #752
- Packet A head `6cbe86e6ee11`
- Packet A merge `e569599db9fe`
- D-0469
- D-0470
- no website/external repo implementation changes
- NA-0252 as sole READY successor

Confirm `DECISIONS.md` contains D-0470 and states NA-0252 is helper tooling, not public-safety weakening.

Confirm `TRACEABILITY.md` links NA-0251 closeout and NA-0252 successor evidence.

## CI Expectations

Local validation bundle:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

Expected result: all pass.

Required PR checks must pass normally before merge:

- ci-4a
- ci-4b
- ci-4c
- ci-4d
- ci-4d-dur
- demo-cli-build
- demo-cli-smoke
- formal-scka-model
- goal-lint
- metadata-conformance-smoke
- suite2-vectors
- CodeQL
- macos-qsc-qshield-build
- public-safety

Post-merge expectation:

- `origin/main` contains NA-0251 DONE and NA-0252 READY.
- D-0470 exists once.
- public-safety remains required and green.
