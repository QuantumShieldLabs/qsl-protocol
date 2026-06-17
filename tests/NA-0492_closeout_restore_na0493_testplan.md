Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-17

# NA-0492 Closeout and NA-0493 Restoration Testplan

## Scope

This closeout marks NA-0492 DONE and restores NA-0493 READY. It does not
implement NA-0493 and does not add corpus files.

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0492_closeout_restore_na0493_testplan.md`

## Required proof

- NA-0492 evidence PR #1255 merged.
- Post-merge public-safety on the PR #1255 merge commit is success.
- Post-merge qsc-adversarial-smoke on the PR #1255 merge commit is success or
  accepted skipped for docs-only evidence.
- D-0973 exists once before closeout.
- D-0974 is absent before closeout and exists once after closeout.
- Queue has exactly one READY item after closeout: NA-0493.
- NA-0492 is DONE.
- NA-0493 is READY.
- No duplicate decision IDs exist.

## Scope guard

Changed paths must be exactly the five closeout paths listed above. The closeout
must not mutate corpus, vectors, inputs, qsc source, qsc fuzz target, qsc fuzz
Cargo files, lockfiles, scripts, workflows, dependencies, formal models,
refimpl, services, public docs, backup tooling, or qwork tooling.

## Future NA-0493 boundary

NA-0493 is limited to exactly seven raw binary seed files under
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` plus NA-0493 governance
evidence/testplan/decision/traceability/journal paths. Every future seed file
must be at most 64 bytes and must pass the validator before commit.

No public-readiness claim is allowed. no production-readiness claim is allowed.
no public-internet-readiness claim is allowed. no external-review-complete claim
is allowed. no crypto-complete claim is allowed. no fuzz-complete claim is
allowed. no corpus-complete claim is allowed. no vector-complete claim is
allowed. no replay-proof claim is allowed. no downgrade-proof claim is allowed.
no side-channel-free claim is allowed. no vulnerability-free claim is allowed.
no bug-free claim is allowed. no perfect-crypto claim is allowed.
