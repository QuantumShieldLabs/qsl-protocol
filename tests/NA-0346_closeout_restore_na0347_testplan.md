Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0346 Closeout and NA-0347 Restoration Test Plan

## Objective

Verify that NA-0346 closes only after the qsl-server integration implementation
authorization PR merges with required checks green, and restore exactly one READY
successor:

`NA-0347 -- Metadata Runtime qsl-server Integration Implementation Harness`

## Protected invariants

- NA-0346 is marked DONE only after Packet M PR #954 merges.
- NA-0347 is restored as READY but not implemented.
- Exactly one READY item exists after closeout.
- D-0674 remains present exactly once.
- D-0675 is added exactly once.
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
- `tests/NA-0346_closeout_restore_na0347_testplan.md`

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

- qsl-protocol PR #954 head and merge evidence;
- post-merge `public-safety` green on PR #954 merge;
- D-0674 proof;
- selected NA-0347 successor proof;
- qsl-server source/authority/CI result;
- qsl-attachments service-local prerequisite boundary;
- qshield embedded relay/demo reference boundary;
- backup impact remains unchanged;
- no qsl-server/qsl-attachments/qshield runtime mutation by closeout;
- no stronger privacy/readiness/public claim.

## Queue requirements

- Before closeout: READY_COUNT 1 and READY NA-0346.
- After closeout patch: READY_COUNT 1 and READY NA-0347.
- NA-0346 is DONE.
- D-0674 exists once.
- D-0675 exists once.
- D-0676 is absent.

## Validation requirements

Run and record:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- goal-lint for the closeout PR body
- classifier proof for the changed path set

## CI expectations

- Required PR checks complete successfully.
- `public-safety` remains required.
- Post-merge `public-safety` completes success on `main`.

## Successor handoff

NA-0347 must start by refreshing qsl-server live source/authority/CI evidence,
then either execute the exact implementation harness under a separate directive
or stop with a precise prerequisite blocker. NA-0346 closeout is not permission
to implement NA-0347.
