Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0244 Closeout and NA-0245 Restoration Test Plan

## Objective

Close NA-0244 from merged PR #736 evidence and restore exactly one READY successor, NA-0245, for website truthfulness, repo-sync, and public-claims audit.

## Scope Guard

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0244_closeout_restore_na0245_testplan.md`

Forbidden paths include `.github/**`, `scripts/**`, Cargo files, qsp/qsc/qsl/qsl-client runtime paths, `apps/**`, tools, inputs, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## Protected Invariants

- NA-0244 closes only after PR #736 merge and post-merge public-safety success.
- D-0454 remains the implementation/evidence decision.
- D-0455 records closeout and successor restoration.
- Exactly one READY item exists after the patch.
- NA-0245 is docs-only audit/plan scope, not website implementation.
- No runtime, service, protocol, crypto, demo, workflow, script, Cargo, qsl-server, qsl-attachments, qsc-desktop, or website implementation file is touched.

## Parser Expectations

After the patch:

```text
READY_COUNT 1
READY NA-0245 Website Truthfulness, Repo-Sync, and Public Claims Audit
NA-0244 DONE Metadata Conformance Negative Expansion
```

Decision parser expectations:

- D-0110 exists once.
- D-0439 through D-0455 exist once each.
- No duplicate decision IDs exist.

## Validation

Required local validation:

```bash
git diff --check
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Additional governance validation:

- changed-path scope guard;
- canonical queue parser;
- canonical decision parser;
- goal-lint;
- markdown inventory and link validation;
- leak-safe added-line scan;
- public-safety required/green proof.

## CI Expectations

- Required GitHub contexts pass normally.
- `public-safety` remains required and green.
- Merge uses a merge commit only.
