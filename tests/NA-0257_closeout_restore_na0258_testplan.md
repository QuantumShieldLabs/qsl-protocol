Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0257 Closeout and NA-0258 Restoration Test Plan

## Objective

Validate that NA-0257 is closed only after the cross-host/LAN-style demo
reproducibility proof merged, post-merge main public-safety completed green,
D-0481 exists on main, and NA-0258 is restored as the sole READY successor.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0257 is DONE.
- NA-0258 is READY.
- Public-safety remains required and green.
- Demo remains non-production.
- Desktop remains non-production unless later release evidence changes.
- No production relay claim.
- No production-ready desktop claim.
- No hidden protocol mutation.
- No protocol/crypto state-machine, runtime/service, `.github`, Cargo,
  public-safety, branch-protection, website, qsl-server, qsl-attachments,
  qsc-desktop implementation, or external website path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0257_closeout_restore_na0258_testplan.md`.

## Forbidden Scope

- `.github/**`.
- `scripts/**`.
- Cargo manifests and lockfiles.
- qsp, qsc, qsl, qsl-client, qsc-desktop, apps, tools, inputs, formal.
- qsl-server.
- qsl-attachments.
- website or external website repositories.
- runtime/protocol/crypto/demo/service code.
- branch-protection settings.
- public-safety/check configuration.

## Preconditions

Expected before closeout edits:

- PR #764 is merged.
- PR #764 merge commit exists on `origin/main`.
- Post-merge `public-safety` completed success on the merge commit.
- READY_COUNT is `1`.
- READY item is `NA-0257`.
- D-0481 exists once.
- D-0482 is absent.
- No duplicate decision IDs exist.

## Queue Parser Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected after closeout:

- READY_COUNT `1`.
- READY `NA-0258 — Native Desktop Package and Screenshot Proof on Provisioned Host`.
- `NA-0257` status `DONE`.

## Decision Parser Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected after closeout:

- D-0481 exists once.
- D-0482 exists once.
- No duplicate decision IDs.

## Scope Guard Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0257_closeout_restore_na0258_testplan.md
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
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0257_closeout_restore_na0258_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- all commands pass;
- no forbidden paths touched;
- D-0482 exists once;
- D-0483 is absent;
- public-safety remains required and green before PR;
- required PR checks pass normally before merge;
- merge uses merge commit only, no direct push, no admin bypass, no squash, and
  no rebase.

## Post-Fix Hardening Review Checklist

- Correctness under stress: closeout only records already-merged PR #764 and
  post-merge public-safety success.
- Minimality: patch changes only closeout governance/testplan paths.
- Maintainability: NA-0258 successor text directly names package/screenshot
  proof, host prerequisites, and non-production wording.
- Coverage quality: queue, decision, scope, link, leak, dependency, and
  send_commit checks prove the closeout state.
- Cross-lane stability: no desktop implementation, protocol, service, website,
  workflow, or dependency files are changed.
