# NA-0302 Closeout and NA-0303 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-17

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0302 after its Suite-2 negotiation vector and qsc receive-path
cross-surface harness PR merged, then restore NA-0303 as the sole READY
successor for qsc handshake activation negotiation cross-surface hardening.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0303.
- NA-0302 is marked DONE and references PR #863 evidence.
- D-0584 exists once and D-0585 is absent.
- NA-0303 is not implemented by this closeout.
- No protocol, wire, crypto state-machine, handshake, key schedule, downgrade,
  replay, or QSP wire-format semantics are changed.
- No Cargo/dependency, workflow, public-safety, branch-protection, service,
  desktop, website, README, START_HERE, docs/public, formal, input, app, tool,
  or runtime implementation path is changed.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, guaranteed-secure, or complete-proof claim is
  introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0302_closeout_restore_na0303_testplan.md`

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
3. `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0302_closeout_restore_na0303_testplan.md`
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

NA-0303 should extend qsc negotiation cross-surface proof toward handshake
activation/admission surfaces using existing public or test APIs where feasible.
If no authorized test seam exists, NA-0303 should stop with exact blocker
evidence rather than changing protocol or crypto semantics silently.
