Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-05
Replaces: n/a
Superseded-By: n/a

# NA-0250 Closeout and NA-0251 Restoration Test Plan

Goals: G1, G3, G5

## Objective

Validate that NA-0250 closes only after PR #749 restores main public-safety, that the one-time PR #749 branch-protection exception was restored immediately, and that NA-0251 is promoted as the sole READY successor without implementing website changes.

## Preconditions

- PR #748 is merged as `98c631a5dc18` from head `b5fa512ba315`.
- PR #749 is merged as `a78746f5d864` from approved head `c7fce4c0c1a`.
- PR #750 is closed and unmerged.
- Main `public-safety` is successful on `a78746f5d864`.
- Branch protection requires `public-safety`.
- `NEXT_ACTIONS.md` has `READY_COUNT 1` and `READY NA-0250` before closeout edits.
- D-0466 and D-0467 exist once.
- D-0468 is absent before closeout edits.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0250_closeout_restore_na0251_testplan.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo metadata, qsp, qsc/qsl/qsl-client implementation paths, apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## Branch-Protection Exception Proof

Required evidence:

- operator approval line is recorded;
- before snapshot exists under `/srv/qbuild/tmp/na0250_pr749_public_safety_exception_20260506T022300Z/`;
- during snapshot shows only `public-safety` removed;
- restore snapshot shows `public-safety` present again;
- strict, admin enforcement, force-push, and deletion settings are not weakened;
- PR #749 merged with merge commit only at the exact approved head;
- no admin bypass, direct push, check spoofing, squash, or rebase merge was used.

## Queue Parser Expectation

After closeout, the canonical queue parser must report:

```text
READY_COUNT 1
READY NA-0251 Public Website Evidence-Boundary Implementation Handoff
NA-0250 DONE External Review and Release-Readiness Evidence Package
```

## Decision Parser Expectation

After closeout, the canonical decision parser must report:

- D-0110 exists once.
- D-0439 through D-0468 exist once each.
- No duplicate decision IDs exist.

## NA-0251 Successor Validation

`NEXT_ACTIONS.md` must state that NA-0251:

- is a qsl-protocol handoff package, not external website implementation;
- has Goals G1, G3, G5;
- allows no wire/behavior change;
- allows no crypto/state-machine change;
- is docs-only;
- excludes qsl-protocol website implementation changes;
- excludes external website repo edits;
- protects evidence-boundary copy, no production-readiness overclaim, no proven true Triple Ratchet overclaim, no anonymity or metadata-elimination overclaim, external product separation, and scoped implementation instructions.

## Required Local Validation

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run:

- canonical queue parser;
- canonical decision parser;
- goal-lint using a synthetic PR event payload;
- markdown inventory and link validation runbook;
- leak-safe added-line scan;
- branch-protection required-check proof.

## PR And CI Acceptance

Acceptance:

- changed paths stay inside the closeout allowlist;
- no forbidden paths are touched;
- PR #750 remains closed and unmerged;
- all required CI contexts attach and pass normally;
- `public-safety` remains required and green;
- no admin bypass, direct push, check spoofing, additional branch-protection exception, squash merge, or rebase merge is used.
