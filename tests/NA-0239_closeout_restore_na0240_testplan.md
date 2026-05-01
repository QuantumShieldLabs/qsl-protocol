Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0239 Closeout and NA-0240 Restoration Testplan

Goals: G2, G3, G4

## Objective

Prove `NA-0239` closes only after PR `#725` merged the bounded public-safety red-main deadlock prevention implementation/evidence with required CI and `public-safety` evidence, and prove `NA-0240` is restored as the sole READY executable SCKA persistence/monotonicity vector-hardening successor.

## Protected Invariant

- `public-safety` remains required and green.
- PR `#725` is treated as complete only from merged evidence.
- PR `#722` remains closed/superseded and unmerged.
- Exactly one READY item exists after closeout.
- `NA-0240` cannot start until this closeout merges.
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
- `tests/NA-0239_closeout_restore_na0240_testplan.md`

Forbidden path proof must confirm no `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `tools/actors/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, runtime/protocol/crypto/demo/service code, branch-protection settings, or public-safety/check configuration changes.

## PR #725 Merge Proof

Required proof:

- PR `#725` state is `MERGED`.
- PR `#725` final validated head is `819b36aebe8f7606153dcf42fae740c22fdb26e2`.
- PR `#725` merge commit is `b466620237adc88e94bc55209b99c310f5ceb111`.
- `origin/main` begins this closeout at the PR `#725` merge commit.
- D-0443 exists once as the implementation/evidence decision.
- D-0444 exists once after this closeout as the closeout/restoration decision.

## Public-Safety Required/Green Proof

Required proof:

- Branch protection required status checks include `public-safety`.
- Latest `main` has a completed successful `public-safety` check after PR `#725`.
- PR `#725` used no branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge.
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
- `READY NA-0239`
- `NA-0239 READY`
- `NA-0238 DONE`
- `NA-0237 DONE`
- `NA-0237A DONE`
- `NA-0237B DONE`
- `NA-0237C DONE`
- `NA-0237D DONE`

After closeout:

- `READY_COUNT 1`
- `READY NA-0240`
- `NA-0240 READY`
- `NA-0239 DONE`
- `NA-0238 DONE`
- `NA-0237 DONE`
- `NA-0237A DONE`
- `NA-0237B DONE`
- `NA-0237C DONE`
- `NA-0237D DONE`

## Decision Parser Proof

Before closeout:

- D-0439 exists once.
- D-0440 exists once.
- D-0441 exists once.
- D-0442 exists once.
- D-0443 exists once.
- D-0444 does not exist.
- No duplicate decision IDs exist.

After closeout:

- D-0439 exists once.
- D-0440 exists once.
- D-0441 exists once.
- D-0442 exists once.
- D-0443 exists once.
- D-0444 exists once.
- D-0445 does not exist.
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
- `DECISIONS.md` (D-0443, D-0444)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- PR `#725`
- PR `#722`
