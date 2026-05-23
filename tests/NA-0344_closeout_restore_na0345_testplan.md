Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0344 Closeout and NA-0345 Restoration Test Plan

## Objective

Verify that NA-0344 is closed only after the qsl-attachments implementation PR
and qsl-protocol governance companion PR have both merged with required checks
green, and restore exactly one READY successor:

`NA-0345 -- Metadata Runtime qsl-server Integration Boundary Plan`

## Protected invariants

- NA-0344 is marked DONE only after qsl-attachments PR #37 and qsl-protocol PR
  #950 are merged.
- NA-0345 is restored as READY but not implemented.
- Exactly one READY item exists after closeout.
- D-0670 remains present exactly once.
- D-0671 is added exactly once.
- qsl-server remains unchanged.
- qsl-attachments remains unchanged by the qsl-protocol closeout.
- qshield runtime, qsc/qsp/protocol/crypto/key-schedule behavior,
  dependencies, workflows, branch protection, public-safety configuration,
  README, START_HERE, docs/public, website, qsc-desktop, formal, inputs,
  scripts, tools/refimpl, apps, and production deployment paths are unchanged.
- No unsupported production, public-internet, external-review, anonymity,
  metadata-free, untraceable, attachment-size-hidden, timing-hidden,
  traffic-shape-hidden, or padding-hides-all-metadata claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0344_closeout_restore_na0345_testplan.md`

## Required closeout evidence

The closeout must record:

- qsl-attachments PR #37 head and merge evidence;
- qsl-attachments required `rust` green on PR head and post-merge `main`;
- qsl-protocol companion PR #950 head and merge evidence;
- qsl-protocol post-merge `public-safety` green on PR #950 merge;
- D-0670 proof;
- selected successor proof;
- backup impact remains unchanged;
- no qsl-server/qsl-attachments/qshield runtime mutation by closeout;
- no stronger privacy/readiness/public claim.

## Queue requirements

Before patch:

- READY_COUNT `1`;
- READY `NA-0344`;
- D-0670 exists once;
- D-0671 absent.

After patch:

- READY_COUNT `1`;
- READY `NA-0345`;
- NA-0344 DONE;
- D-0670 once;
- D-0671 once;
- D-0672 absent.

## Validation requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- PR body preflight / goal-lint with standalone `Goals: G1, G2, G3, G4, G5`
- changed-line overclaim scan
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI expectations

The closeout PR must pass required checks, including `public-safety`, before
merge. After merge, `public-safety` must be required and green on `main`.

## Successor handoff

The restored NA-0345 directive must plan the qsl-server integration boundary
after NA-0344 qsl-attachments service-local size-class proof. It must not
implement qsl-server behavior unless a future exact directive authorizes files,
CI, rollback, deploy, backup, qsl-attachments integration, and public-claim
boundaries.
