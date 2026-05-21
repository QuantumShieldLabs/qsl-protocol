# NA-0330 Closeout and NA-0331 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-21
Replaces: N/A
Superseded-By: N/A

## Objective

Close NA-0330 after the qshield embedded relay/demo batching authorization
plan merged, then restore exactly one successor: NA-0331 -- Metadata Runtime
qshield Demo Batching Implementation Harness.

## Protected Invariants

- NA-0330 is DONE.
- Exactly one READY item exists after closeout: NA-0331.
- D-0642 and D-0643 each exist once.
- No NA-0331 implementation is included in the closeout.
- No batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, transport padding, runtime timing mitigation, service deployment
  behavior, qshield implementation, qsl-server, qsl-attachments,
  qsc/qsp/protocol/crypto/key-schedule, Cargo/dependency, workflow,
  branch-protection, public-safety, qsc-desktop, website, README, START_HERE,
  docs/public, formal, input, tools/refimpl, app runtime, or service
  implementation change is included.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- No prohibited claim wording is introduced: anonymity, metadata-free.
- No prohibited claim wording is introduced: untraceable,
  prohibited production-readiness, prohibited public-internet-readiness.
- No prohibited claim wording is introduced: external-review-complete,
  prohibited timing-hidden, or prohibited traffic-shape-hidden.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0330_closeout_restore_na0331_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
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
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, or service implementation paths
- branch-protection or public-safety configuration

## Queue and Decision Requirements

1. `python3 scripts/ci/qsl_evidence_helper.py queue` reports
   `READY_COUNT 1` and READY NA-0331.
2. NA-0330 is marked DONE in `NEXT_ACTIONS.md`.
3. `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest
   D-0643, no duplicate decision IDs, D-0642 once, and D-0644 absent.

## Closeout Evidence Requirements

- `NEXT_ACTIONS.md` records PR #922, validated head `83a2c14969a`, merge
  `ec3b87661ce2`, the batching authorization plan, D-0642, D-0643, and the
  exact NA-0331 successor.
- `DECISIONS.md` adds D-0643 with Goals G1, G2, G3, G4, G5.
- `TRACEABILITY.md` records the closeout and successor boundary.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records the closeout patch,
  recoveries, validation notes, and next-watch items.

## Required Local Checks

- `git status --porcelain=v1 --branch`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed closeout paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint using a synthetic PR body with `Goals: G1, G2, G3, G4, G5`
- classifier proof for the changed path set

## CI Expectations

- Required PR checks attach normally and complete successfully.
- `public-safety` remains required before merge.
- Post-merge `public-safety` attaches to `main` and completes successfully.
- No admin bypass, direct push, squash, rebase, or branch deletion command is
  used.

## Successor Handoff

NA-0331 may implement only what a future directive explicitly authorizes. The
closeout itself only restores the selected successor and preserves all NA-0330
claim boundaries.
