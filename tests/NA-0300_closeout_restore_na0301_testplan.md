# NA-0300 Closeout and NA-0301 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-16

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0300 after its replay/reject/no-mutation harness PR merged and restore
NA-0301 as the sole READY successor for Suite-2 negotiation / downgrade
expansion harness work.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0301.
- NA-0300 is marked DONE and references PR #859 evidence.
- D-0580 exists once and D-0581 is absent.
- NA-0301 is not implemented by this closeout.
- No protocol, wire, crypto state-machine, handshake, key schedule, downgrade,
  replay, or QSP wire-format semantics are changed.
- No Cargo/dependency, workflow, public-safety, branch-protection, service,
  desktop, website, README, START_HERE, docs/public, formal, input, app, or
  runtime implementation path is changed.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, guaranteed-secure, or complete-proof claim is
  introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0300_closeout_restore_na0301_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
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
- external website sources
- runtime, protocol, crypto, demo, service, or desktop implementation paths

## Required Local Checks

1. `python3 scripts/ci/qsl_evidence_helper.py queue`
2. `python3 scripts/ci/qsl_evidence_helper.py decisions`
3. `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0300_closeout_restore_na0301_testplan.md`
4. `python3 scripts/ci/qsl_evidence_helper.py link-check`
5. `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
6. `cargo audit --deny warnings`
7. `cargo tree -i rustls-webpki --locked`
8. `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
9. `python3 formal/run_model_checks.py`
10. Goal-lint PR body validation or accepted equivalent preflight.
11. Classifier proof for the changed path set.

## CI Expectations

- Required PR checks attach and complete successfully.
- `public-safety` remains required and green.
- For this governance/testplan-only closeout, full-suite cost-control skips may
  be accepted only if public-safety truthfully classifies the changed paths.

## Successor Handoff

NA-0301 should extend executable proof toward Suite-2 negotiation,
downgrade-like, unsupported-parameter, and refimpl/vector consistency fixtures
without changing protocol or crypto semantics unless a future dedicated
directive explicitly authorizes a fail-closed fix lane.
