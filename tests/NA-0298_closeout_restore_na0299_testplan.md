Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0298 Closeout and NA-0299 Restoration Testplan

## Objective

Validate that NA-0298 is closed only as an operator-action blocker packet and
that NA-0299 is restored as the sole READY successor for core protocol, crypto,
formal, vector, demo, metadata, and service-boundary assurance re-entry.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0299.
- NA-0298 is DONE.
- D-0573 exists once.
- D-0574 remains absent.
- The operator bundle remains classified as `OPERATOR_BUNDLE_INCOMPLETE`.
- Website implementation remains blocked.
- No website or external website repository is mutated.
- No qsl-protocol runtime, protocol-core, crypto, demo, service,
  qsc-desktop, qsl-server, qsl-attachments, workflow, script, Cargo,
  dependency, branch-protection, public-safety, formal, input, tool, or app
  implementation path changes.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, guaranteed-secure, website-updated,
  source-verified, deploy-ready, or implementation claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0298_closeout_restore_na0299_testplan.md`

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
- external website repositories
- runtime, protocol, crypto, demo, or service implementation paths
- branch-protection or public-safety configuration
- branch deletion

## Required Checks

- `git diff --check`
- direct changed-line overclaim phrase scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the allowed paths in this testplan
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint using a synthetic PR event if needed locally
- changed-path classifier proof

## Queue Expectations

- Before closeout: READY_COUNT 1, READY NA-0298.
- After closeout patch: READY_COUNT 1, READY NA-0299.
- D-0572 exists once.
- D-0573 exists once.
- D-0574 is absent.

## CI Expectations

Open the closeout PR only after local validation passes. Merge only after
required protected checks complete normally and `public-safety` is green.

No admin bypass, direct push, squash, rebase, or branch deletion is allowed.

## Successor Handoff

NA-0299 is audit and test-matrix work only unless a later directive explicitly
authorizes implementation. The successor must keep protocol-core, crypto
state-machine, dependency, service, and website implementation changes out of
scope while producing the assurance matrix and next executable hardening lane
recommendation.
