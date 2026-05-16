Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0299 Closeout Restore NA-0300 Testplan

## Objective

Validate that NA-0299 is closed only after PR #857 merged and post-merge public-safety was green, then restore exactly one READY successor: NA-0300 Core Protocol Replay / Reject / No-Mutation Adversarial Harness.

## Protected Invariants

- NA-0299 is DONE.
- NA-0300 is the sole READY item.
- D-0575 exists exactly once.
- D-0576 is absent.
- NA-0300 is not implemented in this closeout.
- No protocol-core, crypto state-machine, QSP wire, handshake, key schedule, replay, downgrade, reject, demo/runtime, service, website, workflow, script, Cargo, dependency, branch-protection, or public-safety configuration change.
- No production-readiness, public-internet-readiness, external-review-complete, anonymity, metadata-free, untraceable, quantum-proof, unbreakable, guaranteed-secure, or implementation claim.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0299_closeout_restore_na0300_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- protocol, crypto, runtime, demo, formal, input, refimpl, app, service, desktop, qsl-server, qsl-attachments, website, and external website implementation paths
- branch deletion or branch-protection/public-safety mutation

## Required Checks

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0299_closeout_restore_na0300_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0299_closeout_restore_na0300_testplan.md
```

## Success Criteria

- Queue helper reports `READY_COUNT 1` and READY NA-0300.
- Decision helper reports D-0575 once, D-0576 absent, and no duplicates.
- Scope guard reports `FORBIDDEN_COUNT 0`.
- Link-check reports `TOTAL_MISSING 0`.
- Added-line leak scan reports `SECRET_FINDING_COUNT 0`.
- Changed-line overclaim scan has no unsafe affirmative matches.
- Required CI is green before merge and post-merge public-safety is green.

## Successor Handoff

The restored successor is:

**NA-0300 - Core Protocol Replay / Reject / No-Mutation Adversarial Harness**

The next directive must add or consolidate executable proof or stop with exact prerequisites. It must not silently change protocol or crypto semantics.
