# NA-0261 Closeout and NA-0262 Restoration Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Close NA-0261 after the public evidence refresh merged and post-merge public-safety completed green, then restore NA-0262 as the sole READY successor for executable demo adversarial stress, chaos, and abuse testing.

## Protected Invariants

- Exactly one READY item exists after the closeout.
- NA-0261 is DONE.
- NA-0262 is READY.
- D-0489 remains present exactly once.
- D-0490 is added exactly once.
- Public-safety remains required and green.
- Demo evidence remains non-production.
- No production-readiness claim is introduced.
- No protocol, runtime, crypto, demo implementation, qsl-server, qsl-attachments, qsc-desktop, website, workflow, Cargo, branch-protection, or public-safety configuration change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0261_closeout_restore_na0262_testplan.md`

## Forbidden Scope

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
- external website source
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration

## Queue Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0262 Demo Adversarial Stress, Chaos, and Abuse Testing Harness`
- `NA-0261 DONE Public Evidence Refresh After Demo Expansion`

## Decision Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- D-0489 exists once.
- D-0490 exists once.
- No duplicate decision IDs.

## Scope Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0261_closeout_restore_na0262_testplan.md
```

Expected:

- Changed paths are limited to the allowed closeout files.
- No forbidden implementation, workflow, Cargo, branch-protection, public-safety, qsl-server, qsl-attachments, qsc-desktop, website, or external website path appears.

## No Overclaim Proof

Run direct phrase scans over added lines and final docs for high-risk release claims:

```bash
git diff --unified=0 origin/main...HEAD -- NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0261_closeout_restore_na0262_testplan.md
```

Expected:

- Any occurrence of production-readiness language is a protected/forbidden-boundary statement, not an affirmative readiness claim.
- NA-0262 remains a future executable demo stress harness lane, not implemented in this closeout.

## Link / Leak / Goal-Lint Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Expected:

- Link check passes.
- Added-line leak scan reports zero secret findings.
- Goal-lint passes with the PR body Goals line.

## Dependency / Main Health Expectations

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Expected:

- Cargo audit passes.
- `rustls-webpki` remains at the patched locked version.
- `send_commit` tests pass.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance basis.
- Public-safety remains required on `main`.
- Post-merge main public-safety completes green before NA-0262 work starts.
