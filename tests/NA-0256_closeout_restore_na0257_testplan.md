Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0256 Closeout and NA-0257 Restoration Test Plan

## Objective

Validate that NA-0256 is closed only after the demo/desktop readiness package merged, post-merge main public-safety completed green, D-0479 exists on main, and NA-0257 is restored as the sole READY successor.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0256 is DONE.
- NA-0257 is READY.
- Public-safety remains required and green.
- Demo remains non-production.
- No production relay, desktop, qsl-server, or qsl-attachments readiness claim is introduced.
- No protocol/crypto state-machine, runtime/service, `.github`, Cargo, public-safety, branch-protection, website, qsl-server, or qsl-attachments path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0256_closeout_restore_na0257_testplan.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- Cargo manifests and lockfiles
- qsp, qsc, qsl, qsl-client, qsc-desktop, apps, tools, inputs, formal
- qsl-server
- qsl-attachments
- website or external website repositories
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration

## Preconditions

Expected before closeout edits:

- PR #762 is merged.
- PR #762 merge commit exists on `origin/main`.
- Post-merge `public-safety` completed success on the merge commit.
- READY_COUNT is `1`.
- READY item is `NA-0256`.
- D-0479 exists once.
- D-0480 is absent.
- No duplicate decision IDs exist.

## Queue Parser Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected after closeout:

- READY_COUNT `1`
- READY `NA-0257 — Cross-Host / Tailscale Public Demo Reproducibility`
- `NA-0256` status `DONE`

## Decision Parser Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected after closeout:

- D-0479 exists once.
- D-0480 exists once.
- No duplicate decision IDs.

## Scope Guard Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0256_closeout_restore_na0257_testplan.md
```

Expected:

- changed paths are exactly closeout governance/testplan paths;
- `FORBIDDEN_COUNT 0`.

## Validation Commands

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check origin/main...HEAD
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Also run repo-local goal-lint with a synthetic pull-request event payload containing:

```text
Goals: G1, G4, G5
```

## PR / CI Expectations

- PR title: `NA-0256: closeout and restore NA-0257`.
- PR body includes Goals, Impact, No-regression, and Tests/Vectors.
- Required checks pass normally before merge.
- Merge uses merge commit only with `--match-head-commit`.
- No admin bypass, direct push, squash merge, rebase merge, public-safety weakening, or branch-protection exception.

## Post-Merge Expectations

After closeout merge:

- `origin/main` contains D-0480.
- `NEXT_ACTIONS.md` shows READY_COUNT `1`, READY `NA-0257`, and `NA-0256` DONE.
- `public-safety` remains required and completes success on final main.
