Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

# NA-0260 Closeout and NA-0261 Restoration Test Plan

## Objective

Close NA-0260 after the attachment demo readiness proof merged and post-merge
public-safety completed green. Restore NA-0261 as the sole READY successor for
public evidence refresh after the KT-negative and attachment demo expansions.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0260 is DONE.
- NA-0261 is READY.
- D-0487 remains present once.
- D-0488 is added once.
- Public-safety remains required and green.
- The demo remains non-production.
- No production attachment, relay, qsl-server, qsl-attachments, desktop, or
  website readiness claim is introduced.
- No protocol/runtime/crypto/demo/service implementation changes occur in this
  closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0260_closeout_restore_na0261_testplan.md`

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
- external website repository files
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration

## Positive Proof

1. `NEXT_ACTIONS.md` marks NA-0260 DONE and records PR #770, validated head,
   merge commit, artifact directory, D-0487, D-0488, proof mode, and
   post-merge public-safety evidence.
2. `NEXT_ACTIONS.md` promotes exactly one successor: NA-0261, with the public
   evidence refresh scope and non-production limits from the directive.
3. `DECISIONS.md` adds D-0488 for NA-0260 closeout and NA-0261 restoration.
4. `TRACEABILITY.md` records NA-0260 closeout evidence and the NA-0261
   successor boundary.

## Negative Proof

1. Scope guard reports no forbidden paths.
2. Queue helper reports `READY_COUNT 1` and `READY NA-0261`.
3. Decision helper reports D-0487 once, D-0488 once, and no duplicate IDs.
4. Link check and leak scan remain clean.
5. Required CI passes normally before merge.

## Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0260_closeout_restore_na0261_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

## CI Expectations

- Required PR checks pass normally.
- CodeQL neutral remains acceptable only if branch protection accepts it.
- `public-safety` remains required.
- Post-merge main public-safety completes success before NA-0261 implementation
  begins.

## Stop Conditions

- More than one READY item exists.
- NA-0261 cannot be promoted as the sole READY item.
- Any forbidden path changes.
- Public-safety is missing, red, or not required.
- Closeout would require implementing NA-0261.
- Closeout would require protocol/runtime/crypto/demo/service, qsl-server,
  qsl-attachments, qsc-desktop, website, `.github`, Cargo, branch-protection,
  or public-safety changes.
