Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0294 Closeout and NA-0295 Restoration Test Plan

## Objective

Close NA-0294 after the public evidence navigation refresh and restore NA-0295
as the sole READY successor.

## Protected Invariants

- Exactly one READY item exists after the patch.
- NA-0294 is DONE.
- NA-0295 is READY.
- D-0565 exists exactly once and D-0566 is absent.
- NA-0295 is not implemented in this closeout.
- Website and external website sources remain untouched.
- No protocol, crypto, runtime, service, demo, workflow, script, Cargo,
  dependency, branch-protection, or public-safety configuration changes occur.
- No anonymity, metadata-free, untraceable, external-review-complete,
  production-readiness, or public-internet-readiness claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0294_closeout_restore_na0295_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/INDEX.md`
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
- runtime, protocol, crypto, demo, service, workflow, dependency,
  branch-protection, and public-safety configuration paths.

## Validation Commands

```bash
git diff --name-only origin/main...HEAD
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

## Success Criteria

- Queue helper reports `READY_COUNT 1` and `READY NA-0295`.
- Decision helper reports no duplicates, latest decision D-0565, and no D-0566.
- Scope is limited to the allowed closeout paths.
- Link-check, leak-scan, dependency health, send_commit, formal/model checks,
  goal-lint, and required CI pass.
- No branch deletion, direct push, squash, rebase, admin bypass,
  branch-protection change, or public-safety weakening occurs.

## NA-0262A Cost-Control Closeout Timing Proof

This closeout is docs/governance-only. The expected post-merge public-safety
behavior is that docs-only full suites may skip under the NA-0262A policy while
`public-safety` itself remains required and green.
