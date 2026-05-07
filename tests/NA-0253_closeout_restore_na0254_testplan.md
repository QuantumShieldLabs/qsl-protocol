Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0253 Closeout And NA-0254 Restoration Test Plan

## Objective

Validate that NA-0253 closes only after PR #756 merged, PR #757 recovered main public-safety, the one-time PR #757-only public-safety required-check exception was restored immediately, and main public-safety completed green; then verify NA-0254 is restored as the sole READY successor without implementing NA-0254.

## Preconditions

- PR #756 is merged as `59ae6f25d39e` from head `2d6000e543c6`.
- PR #757 is merged as `b62948c86ca1` from approved head `892f7bb8d4bf`.
- Branch-protection snapshot evidence exists under `/srv/qbuild/tmp/na0253_pr757_public_safety_exception_20260507T184958Z/`.
- public-safety is restored as a required check.
- Main `public-safety` is successful on `b62948c86ca1` before closeout edits.
- D-0473 and D-0474 exist once.
- D-0475 is absent before closeout edits.
- `NEXT_ACTIONS.md` has `READY_COUNT 1` and `READY NA-0253` before closeout edits.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0253 is DONE.
- NA-0254 is READY.
- D-0475 exists once.
- public-safety remains required and green.
- NA-0254 is public-safety timeout hardening, not public-safety weakening.
- No `.github`, `scripts/**`, public-safety helper/configuration, branch-protection, Cargo, qsp, qsc/qsl/qsl-client implementation, apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, external website repo, or runtime/protocol/crypto/demo/service implementation changes occur in this closeout lane.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0253_closeout_restore_na0254_testplan.md`
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
  --allowed tests/NA-0253_closeout_restore_na0254_testplan.md \
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
- `READY NA-0254 Public-Safety Timeout-Resilient Push-Suite Polling Hardening`
- `NA-0253 DONE External Website Evidence-Boundary Implementation Planning`
- NA-0252 through NA-0237 remain DONE.

## Decision Parser Expectation

Run the canonical decision parser and helper decision parser after closeout edits.

Expected result:

- D-0110 exists once
- D-0439 through D-0475 exist once each
- duplicate count is zero

## Closeout Evidence Checks

Confirm `NEXT_ACTIONS.md` records:

- PR #756 head `2d6000e543c6`
- PR #756 merge `59ae6f25d39e`
- PR #757 head `892f7bb8d4bf`
- PR #757 merge `b62948c86ca1`
- branch-protection exception approval, snapshot, removal, restore, and final public-safety success evidence
- D-0473, D-0474, and D-0475
- NA-0254 as sole READY successor

Confirm `DECISIONS.md` contains D-0475 and states:

- NA-0253 closeout occurred only after public-safety recovered
- PR #757 merged under the explicit one-time PR #757-only public-safety required-check exception
- public-safety was restored immediately
- NA-0254 is public-safety timeout hardening, not public-safety weakening

Confirm `TRACEABILITY.md` links NA-0253 closeout, branch-protection snapshot/restore evidence, public-safety recovery evidence, and NA-0254 successor evidence.

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
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths DECISIONS.md TRACEABILITY.md NEXT_ACTIONS.md tests/NA-0253_closeout_restore_na0254_testplan.md
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

- `origin/main` contains NA-0253 DONE and NA-0254 READY.
- D-0475 exists once.
- public-safety remains required and green.
- PR #722 and PR #750 remain closed and unmerged.
