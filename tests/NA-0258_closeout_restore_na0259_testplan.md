Goals: G1, G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0258 Closeout and NA-0259 Restoration Test Plan

## Objective

Close NA-0258 only after the native desktop package/screenshot evidence PR has
merged, post-merge main `public-safety` is green, D-0483 exists on main, and
NA-0259 is restored as the sole READY successor without implementing NA-0259.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0258 is DONE.
- NA-0259 is READY.
- D-0483 exists before closeout.
- D-0484 records the closeout/restoration.
- public-safety remains required and green.
- Desktop/demo surfaces remain non-production.
- No production-ready desktop claim.
- No KT-negative fake evidence.
- No protocol/crypto state-machine change.
- No qsl-server or qsl-attachments production change.
- No website/external website change.
- No `.github`, scripts, Cargo, runtime, qsc-desktop, branch-protection, or
  public-safety configuration change in this closeout.

## Scope

Allowed paths:

```text
NEXT_ACTIONS.md
DECISIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0258_closeout_restore_na0259_testplan.md
```

Forbidden paths include runtime/protocol/crypto/demo/service implementation,
qsl-server, qsl-attachments, qsc-desktop, website/external website,
`.github`, scripts, Cargo files, branch-protection settings, and public-safety
configuration.

## Required Local Validation

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check origin/main...HEAD
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed 'NEXT_ACTIONS.md' \
  --allowed 'DECISIONS.md' \
  --allowed 'TRACEABILITY.md' \
  --allowed 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md' \
  --allowed 'tests/NA-0258_closeout_restore_na0259_testplan.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- READY_COUNT `1`, READY `NA-0259`.
- NA-0258 is DONE.
- D-0483 exists once.
- D-0484 exists once.
- No duplicate decision IDs.
- Scope guard reports no forbidden paths.
- Link-check reports no missing links.
- Leak scan reports no findings.
- Public-safety is required and green before PR.

## PR / Merge Expectations

- Branch: `na-0258-closeout-restore-na0259`.
- PR title: `NA-0258: closeout and restore NA-0259`.
- PR body includes `Goals: G1, G3, G4`.
- Required checks pass normally.
- Merge uses merge commit only with `--match-head-commit`.
- No direct push, admin bypass, squash, rebase, branch-protection exception,
  public-safety weakening, or check spoofing.

## Post-Merge Expectations

After merge:

```bash
git fetch origin main --prune
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- READY_COUNT `1`.
- READY `NA-0259`.
- NA-0258 DONE.
- D-0484 present.
- public-safety remains required and completes green on final main.

## Post-Fix Hardening Review Checklist

- Correctness under stress: closeout is based on merged PR #766 and
  post-merge green public-safety, not on local-only proof.
- Minimality: only governance closeout paths change.
- Maintainability: NA-0259 successor text carries explicit scope, protected
  invariants, deliverables, and acceptance criteria.
- Coverage quality: queue, decision, scope, link, leak, dependency-health, and
  PR checks all verify different failure surfaces.
- Cross-lane stability: no runtime/protocol/crypto/qsl-server/qsl-attachments/
  qsc-desktop/website/workflow/Cargo path changes occur.
