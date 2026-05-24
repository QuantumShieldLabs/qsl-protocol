Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0345 Closeout and NA-0346 Restoration Test Plan

## Objective

Verify that NA-0345 closes only after the qsl-server integration boundary plan
PR merges with required checks green, and restore exactly one READY successor:

`NA-0346 -- Metadata Runtime qsl-server Integration Implementation Authorization Plan`

## Protected invariants

- NA-0345 is marked DONE only after Packet M PR #952 merges.
- NA-0346 is restored as READY but not implemented.
- Exactly one READY item exists after closeout.
- D-0672 remains present exactly once.
- D-0673 is added exactly once.
- qsl-server remains unchanged.
- qsl-attachments remains unchanged.
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
- `tests/NA-0345_closeout_restore_na0346_testplan.md`

## Forbidden scope

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
- runtime/protocol/crypto/demo/service implementation paths
- branch-protection or public-safety configuration
- branch deletion

## Required closeout evidence

The closeout must record:

- qsl-protocol PR #952 head and merge evidence;
- post-merge `public-safety` green on PR #952 merge;
- D-0672 proof;
- selected NA-0346 successor proof;
- qsl-server source/authority result;
- qsl-attachments service-local boundary;
- qshield embedded relay/demo reference boundary;
- backup impact remains unchanged;
- no qsl-server/qsl-attachments/qshield runtime mutation by closeout;
- no stronger privacy/readiness/public claim.

## Queue requirements

Before patch:

- READY_COUNT `1`;
- READY `NA-0345`;
- D-0672 exists once;
- D-0673 absent.

After patch:

- READY_COUNT `1`;
- READY `NA-0346`;
- NA-0345 DONE;
- D-0672 once;
- D-0673 once;
- D-0674 absent.

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

The restored NA-0346 directive must produce a qsl-server integration
implementation authorization plan or exact blocker evidence. It must not
implement NA-0346 unless a future exact directive authorizes mutation.
