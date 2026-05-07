Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0252 Closeout And NA-0253 Restoration Test Plan

## Objective

Validate that NA-0252 closes only after PR #754 merged, CodeQL taint-isolation recovery completed, and main public-safety recovered to green; then verify NA-0253 is restored as the sole READY successor without implementing NA-0253.

## Preconditions

- PR #754 is merged as `81213bb0a1ab` from head `d739e8af95e1`.
- D-0471 exists once.
- D-0472 is absent before closeout edits.
- `NEXT_ACTIONS.md` has `READY_COUNT 1` and `READY NA-0252` before closeout edits.
- Branch protection requires `public-safety`.
- Main `public-safety` is successful on `81213bb0a1ab` before closeout edits.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0252 is DONE.
- NA-0253 is READY.
- D-0472 exists once.
- NA-0253 is external website implementation planning, not website implementation.
- No `.github`, `scripts/**`, public-safety helper/configuration, branch-protection, Cargo, qsp, qsc/qsl/qsl-client implementation, apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, external website repo, or runtime/protocol/crypto/demo/service implementation changes occur in this closeout lane.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0252_closeout_restore_na0253_testplan.md`
- `docs/governance/evidence/**` only if current repo convention strictly requires it

Validation commands:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0252_closeout_restore_na0253_testplan.md \
  --forbidden .github/** \
  --forbidden scripts/** \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsp/** \
  --forbidden qsc/** \
  --forbidden qsl/** \
  --forbidden qsl-client/** \
  --forbidden apps/** \
  --forbidden tools/** \
  --forbidden inputs/** \
  --forbidden formal/** \
  --forbidden qsc-desktop/** \
  --forbidden qsl-server/** \
  --forbidden qsl-attachments/** \
  --forbidden website/**
```

Expected result: changed paths stay inside the closeout allowlist and forbidden count is zero.

## Queue Parser Expectation

Run the canonical queue parser and helper queue parser after closeout edits.

Expected result:

- `READY_COUNT 1`
- `READY NA-0253 External Website Evidence-Boundary Implementation Planning`
- `NA-0252 DONE Repo-Local Evidence and CI Recovery Helper Toolkit`
- NA-0251 through NA-0237 remain DONE.

## Decision Parser Expectation

Run the canonical decision parser and helper decision parser after closeout edits.

Expected result:

- D-0110 exists once
- D-0439 through D-0472 exist once each
- duplicate count is zero

## Closeout Evidence Checks

Confirm `NEXT_ACTIONS.md` records:

- PR #754 head `d739e8af95e1bc049c1427344273d4f5c0a30dd4`
- PR #754 merge `81213bb0a1ab8b9c285b8eb648332f0f37289590`
- CodeQL taint-isolation recovery evidence
- initial public-safety timeout-wrapper failure evidence
- watched full-suite success evidence
- authorized public-safety rerun success evidence
- D-0471 and D-0472
- NA-0253 as sole READY successor

Confirm `DECISIONS.md` contains D-0472 and states:

- NA-0252 helper toolkit merged in PR #754
- CodeQL taint-isolation recovery was completed
- NA-0252 closeout occurred only after public-safety recovered
- NA-0253 is external website implementation planning, not website implementation

Confirm `TRACEABILITY.md` links NA-0252 closeout and NA-0253 successor evidence.

## Required Local Validation

Run:

```bash
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --report-only
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths DECISIONS.md TRACEABILITY.md NEXT_ACTIONS.md tests/NA-0252_closeout_restore_na0253_testplan.md
```

Also run:

- canonical queue parser
- canonical decision parser
- goal-lint using a synthetic PR event payload
- markdown inventory and link validation runbook
- branch-protection required-check proof

## PR And CI Acceptance

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

- `origin/main` contains NA-0252 DONE and NA-0253 READY.
- D-0472 exists once.
- public-safety remains required and green.
- PR #722 and PR #750 remain closed and unmerged.
