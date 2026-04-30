Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0237 Closeout and NA-0238 Restoration Testplan

Goals: G3, G4

## Objective

Prove `NA-0237` closes only after PR `#708` merged the bounded fail-closed KT verifier implementation/evidence with required CI and `public-safety` evidence, and prove `NA-0238` is restored as the sole READY docs-only successor without starting roadmap implementation in the closeout PR.

## Protected Invariant

- `public-safety` remains required and green.
- PR `#708` is treated as complete only from merged evidence.
- PR `#722` remains closed/superseded and unmerged.
- Exactly one READY item exists after closeout.
- qsl-server remains transport-only.
- qsl-attachments remains opaque ciphertext-only.
- qsc-desktop remains untouched.
- No runtime, protocol, crypto, demo, service, workflow, script, Cargo, branch-protection, or public-safety configuration surface changes in this closeout.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0237_closeout_restore_na0238_testplan.md`

Forbidden path proof must confirm no `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `tools/actors/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, runtime/protocol/crypto/demo/service code, branch-protection settings, or public-safety/check configuration changes.

## PR #708 Merge Proof

Required proof:

- PR `#708` state is `MERGED`.
- PR `#708` final validated head is `0c1fa7d54490b9130f9d1fe26b9c41db327def6f`.
- PR `#708` merge commit is `8c18f6306d8cc95f8cf4252f261f112c20406478`.
- `origin/main` begins this closeout at the PR `#708` merge commit.
- D-0440 exists once as the implementation/evidence decision.
- D-0441 exists once after this closeout as the closeout/restoration decision.

## Public-Safety Required/Green Proof

Required proof:

- Branch protection required status checks include `public-safety`.
- Latest `main` has a completed successful `public-safety` check after PR `#708`.
- PR `#708` used no branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge.
- The closeout PR must also pass `public-safety` normally before merge.

## PR #722 Closed/Unmerged Proof

Required proof:

- PR `#722` state is `CLOSED`.
- PR `#722` has `mergedAt: null`.
- PR `#722` merge commit is null.
- PR `#722` branch is not modified by this closeout.

## Queue Parser Proof

Before closeout:

- `READY_COUNT 1`
- `READY NA-0237`
- `NA-0237A DONE`
- `NA-0237B DONE`
- `NA-0237C DONE`
- `NA-0237D DONE`
- `NA-0238 BACKLOG`

After closeout:

- `READY_COUNT 1`
- `READY NA-0238`
- `NA-0237 DONE`
- `NA-0237A DONE`
- `NA-0237B DONE`
- `NA-0237C DONE`
- `NA-0237D DONE`
- `NA-0238` is not BACKLOG.

## Decision Parser Proof

Before closeout:

- D-0439 exists once.
- D-0440 exists once.
- D-0441 does not exist.
- No duplicate decision IDs exist.

After closeout:

- D-0439 exists once.
- D-0440 exists once.
- D-0441 exists once.
- D-0442 does not exist.
- No duplicate decision IDs exist.

## Local Validation Commands

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- deterministic queue parser
- deterministic decision parser
- repo-local goal-lint via synthetic PR event
- markdown inventory commands from `AGENTS.md`
- manual markdown link-integrity runbook from `AGENTS.md`
- leak-safe added-line scan
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`

## Required CI Context Expectations

The closeout PR must satisfy the protected context set:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`CodeQL` may be accepted as neutral only if GitHub branch protection accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.

## References

- `NEXT_ACTIONS.md`
- `DECISIONS.md` (D-0440, D-0441)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- PR `#708`
- PR `#722`
