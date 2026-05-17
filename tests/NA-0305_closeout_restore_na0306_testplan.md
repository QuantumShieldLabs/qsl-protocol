Status: Supporting
Owner: qsl-protocol maintainers
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0305 Closeout and NA-0306 Restoration Testplan

## Objective

Close NA-0305 after PR #869 merged and post-merge public-safety completed
success, then restore exactly one READY successor: NA-0306 -- qsc Handshake
Suite-ID Wire-Format Change Authorization Plan.

## Protected invariants

- NA-0305 is marked DONE only after PR #869 and post-merge public-safety are
  green.
- NA-0306 is restored as the sole READY item.
- NA-0306 is not implemented by this closeout.
- The qsc handshake suite-id seam limitation from NA-0305 remains visible.
- No protocol, crypto state-machine, handshake implementation, key schedule, or
  QSP wire-format behavior changes are introduced.
- No dependency, workflow, branch-protection, public-safety configuration,
  service, website, README, START_HERE, docs/public, qsc-desktop, formal,
  input, tools/refimpl, or app implementation path changes are introduced.
- No production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, broad-readiness, or complete-proof claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0305_closeout_restore_na0306_testplan.md`

## Forbidden scope

- Runtime, protocol-core, crypto, handshake, key schedule, QSP wire-format, demo,
  service, website, qsc-desktop, qsl-server, qsl-attachments, formal, input,
  tools/refimpl, app implementation, dependency, workflow, script,
  branch-protection, public-safety configuration, README, START_HERE, and
  docs/public changes.

## Queue requirements

- Before closeout: READY_COUNT 1 and READY NA-0305.
- After closeout patch: READY_COUNT 1 and READY NA-0306.
- NA-0305 must be DONE.
- D-0590 must exist once.
- D-0591 must be absent.

## Successor handoff

NA-0306 is selected from NA-0305 evidence: current qsc `QHSM` A1/B1/A2 frames
do not expose an explicit suite-id field, and a test-only appended field would
prove only generic decode failure. The successor is a design/authorization lane
for exact qsc handshake suite-id wire-format semantics and proof requirements.

## Required local checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0305_closeout_restore_na0306_testplan.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint using a PR body containing `Goals: G1, G2, G3, G4, G5`

## CI expectations

The closeout is governance/testplan-only. Required CI and public-safety must
complete normally. Docs/governance-only full-suite skips are acceptable when
the classifier and public-safety gates report them as intentional cost-control
behavior.
