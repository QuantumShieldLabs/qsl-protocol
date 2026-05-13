Goals: G1, G3, G4, G5

# NA-0276 Closeout and NA-0277 Restoration Testplan

## Objective

Verify that NA-0276 closes only after qsl-server PR #51 and qsl-protocol PR
#805 merged with required checks green, and that NA-0277 is restored as the
sole READY successor without implementing NA-0277.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0276 is marked DONE.
- NA-0277 is the sole READY item.
- D-0522 remains the accepted NA-0276 evidence decision.
- D-0523 exists once as the closeout/restoration decision.
- qsl-server PR #51 evidence remains recorded.
- qsl-protocol PR #805 evidence remains recorded.
- public-safety remains required and green.
- No qsl-protocol runtime, protocol, wire, crypto, state-machine, qsp
  protocol-core, qsc/qsl runtime, qsl-attachments implementation,
  qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced.
- No production readiness or deployment readiness claim is introduced.

## Closeout Evidence Requirements

The closeout must record:

- qsl-server PR #51 head and merge SHAs;
- qsl-protocol PR #805 head and merge SHAs;
- chosen invalid config semantics;
- D-0522 evidence decision;
- D-0523 closeout decision;
- post-merge public-safety success on PR #805 merge;
- NA-0277 successor scope, objective, protected invariants, deliverables, and
  acceptance criteria.

## NA-0277 Successor Scope

NA-0277 is qsl-server abuse / rate-limit / queue-cap harness work. It must:

- add executable qsl-server abuse/rate/queue harness evidence or stop with a
  prerequisite justification;
- keep qsl-protocol changes to governance/evidence/testplan only;
- avoid qsl-protocol runtime/crypto changes;
- avoid qsl-attachments changes;
- avoid website changes;
- preserve deterministic overload behavior, explicit queue/resource caps,
  reject no-mutation behavior, and no secret logging under pressure.

## Validation Expectations

Local validation should include:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- direct overclaim phrase scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint or helper PR-body preflight with the exact Goals line.

## CI Expectations

Required CI must pass normally before merge. The closeout is
docs/governance/testplan-only, so NA-0262A docs-only cost control may skip the
heavy full-suite jobs while keeping public-safety required and green.

## No Production-Readiness Claim

This closeout may claim only that NA-0276 evidence is merged and NA-0277 is
restored. It must not claim production service approval, public internet
exposure safety, production deployment review completion, external review
completion, metadata elimination, anonymity, untraceability, or qsl-server
production readiness.
