Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0461 Closeout / Restore NA-0462 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0461 after the qsc B1 signature provider RNG failure seam implementation merged and post-merge public-safety completed success, then restore `NA-0462 -- QSL qsc A2 Signature Provider RNG Failure Scope Authorization Plan` as the sole READY successor.

## Protected Invariants

- NA-0461 is DONE only after PR #1191 merged and post-merge public-safety completed success on merge commit `37a540983595`.
- NA-0462 is READY and authorization-only.
- Exactly one READY item remains.
- NA-0461 B1 signing evidence remains bounded internal qsc forced-seam evidence.
- A2 signing remains unimplemented by this closeout and remains the selected next scope because it needs a different post-mutation invariant.
- Identity provider RNG, X25519 provider RNG, refimpl provider RNG, qshield-cli RNG, formal/model RNG, and fuzz/vector RNG remain residual.
- Cargo audit green remains dependency-health evidence only.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0461_closeout_restore_na0462_testplan.md`

## Forbidden Scope

- qsc source mutation.
- qsc implementation test mutation.
- refimpl mutation.
- dependency, Cargo, lockfile, or workflow mutation.
- fuzz target, vector, or formal model mutation.
- qsl-server or qsl-attachments mutation.
- qshield runtime or qshield-cli mutation.
- website, public docs, README, or START_HERE mutation.
- qwork, qstart, qresume, or qshell mutation.
- backup or restore execution.
- qsl-backup, backup status, backup plan, rollback subtree, or backup tree mutation.
- Any public-readiness, production-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, RNG-failure-complete, provider-RNG-complete, side-channel-free, vulnerability-free, bug-free, perfect-crypto, metadata-free, anonymity, or untraceable claim.

## Required Checks

- Queue helper proves READY_COUNT 1 and READY NA-0462.
- Decision helper proves D-0910 exists once, D-0911 absent, and duplicate decision count zero.
- Scope guard proves changed paths are exactly the allowed closeout paths.
- Link check reports no missing local markdown links.
- Leak scan reports no added-line secret findings.
- Overclaim scan reports no forbidden positive public claims.
- PR body preflight passes with Goals, Impact, No-regression, and Tests/Vectors.
- Goal-lint passes for the closeout PR.
- Root `cargo audit --deny warnings` passes.
- Nested qsc fuzz lock audit passes.
- Public-safety is green before merge and after merge.

## Closeout Acceptance

- NA-0461 is marked DONE.
- NA-0462 is restored as READY.
- D-0910 records NA-0461 closeout and NA-0462 restoration.
- TRACEABILITY links the closeout to PR #1191, D-0910, D-0909, the NA-0461 evidence/testplan, and this closeout testplan.
- Rolling journal records the implementation PR merge, post-merge public-safety extension, and closeout branch state.
- No implementation, dependency, workflow, service, public-doc, backup, qwork, or refimpl mutation occurs.
