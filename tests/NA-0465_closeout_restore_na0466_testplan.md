Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0465 Closeout Restore NA-0466 Testplan

## Objective

Close out NA-0465 after qsl-protocol PR #1199 merged at `d180b3265aa5` and post-merge public-safety completed success, then restore `NA-0466 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Scope Authorization Plan` as the sole READY successor.

## Protected Invariants

- Exactly one READY item remains.
- NA-0465 is DONE.
- NA-0466 is READY.
- D-0918 exists once.
- D-0919 is absent before NA-0466 begins.
- No duplicate decision IDs exist.
- NA-0465 evidence remains lazy-identity-only evidence.
- NA-0466 is authorization-only and does not implement legacy/public-record identity provider RNG failure.

## Allowed Scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0465_closeout_restore_na0466_testplan.md`.

## Forbidden Scope

- No qsc source mutation.
- No runtime behavior mutation.
- No crypto behavior mutation.
- No dependency mutation.
- No Cargo manifest mutation.
- No lockfile mutation.
- No workflow mutation.
- No executable implementation test mutation.
- No fuzz target mutation.
- No vector mutation.
- No formal model mutation.
- No refimpl mutation.
- No qsl-server mutation.
- No qsl-attachments mutation.
- No qshield runtime mutation.
- No qshield-cli mutation.
- No website mutation.
- No public docs mutation.
- No README mutation.
- No START_HERE mutation.
- No qwork/qstart/qresume/qshell mutation.
- No backup or restore.
- No qsl-backup mutation.
- No backup status or backup plan mutation.
- No rollback subtree or backup tree mutation.

## Queue Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0466`.
- `NA-0465 DONE`.
- Prior closed items remain closed or blocked as previously recorded.

## Decision Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- latest decision is D-0918.
- D-0917 exists once.
- D-0918 exists once.
- D-0919 is absent.
- duplicate decision count is zero.

## Scope Guard

Run a name-only diff against `origin/main` and verify the changed paths are exactly:

```text
DECISIONS.md
NEXT_ACTIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0465_closeout_restore_na0466_testplan.md
```

## Link Check

Run the repository markdown link-integrity check used by governance closeouts.

Required:

- `TOTAL_MISSING 0`.

## Leak Scan

Run the governance leak scan over the closeout diff.

Required:

- no sensitive endpoints, tokens, auth headers, route tokens, or long-hex dumps are introduced.

## Overclaim Scan

Run the added-line overclaim scan.

Required:

- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No KEM-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.

## Dependency Health

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required:

- root audit PASS.
- nested qsc fuzz lock audit PASS.
- cargo audit green remains dependency-health evidence only.

## Formatting

Run:

```bash
cargo fmt --check
```

Required:

- PASS.

## PR Body Preflight

Run the PR body preflight against the closeout PR body.

Required:

- `Goals:` line is present near the top.
- `Impact:` is present.
- `No-regression:` is present.
- `Tests/Vectors:` is present.
- prohibited overclaim phrases are absent.

## Public-Safety

Before merge:

- required PR checks must pass.
- public-safety must be green.

After merge:

- public-safety must be green on the closeout merge commit.

## Closeout Boundary

This closeout does not implement NA-0466. Legacy/public-record identity provider RNG failure scope selection remains future work. CLI identity rotation, TUI account bootstrap identity generation, X25519 / ephemeral generation, qshield-cli demo RNG, formal/model RNG, fuzz/vector RNG, and refimpl provider RNG remain residual unless a later exact directive authorizes them.
