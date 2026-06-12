Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0466 Closeout Restore NA-0467 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0466 after qsl-protocol PR #1201 merged at `230c24e61221` and
post-merge public-safety completed success, then restore
`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test
Seam Implementation Harness` as the sole READY successor.

## Protected invariants

- Exactly one READY item remains.
- NA-0466 is DONE.
- NA-0467 is READY.
- D-0919 exists once.
- D-0920 exists once after closeout.
- D-0921 is absent before NA-0467 begins.
- No duplicate decision IDs exist.
- NA-0466 remains authorization-only evidence.
- NA-0467 is restored but not implemented by this closeout.
- The exact future NA-0467 implementation paths are preserved:
  `qsl/qsl-client/qsc/src/identity/mod.rs` and
  `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`.
- No qsc source mutation occurs in this closeout.
- No runtime behavior mutation occurs in this closeout.
- No crypto behavior mutation occurs in this closeout.
- No dependency, Cargo manifest, lockfile, workflow, executable test, fuzz
  target, vector, formal model, refimpl, service, qshield runtime, qshield-cli,
  website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell,
  backup, restore, qsl-backup, backup-status, backup-plan, rollback, or
  backup-tree mutation occurs.
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

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0466_closeout_restore_na0467_testplan.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to qsc source, runtime behavior, crypto behavior, dependencies, Cargo
manifests, lockfiles, workflows, executable implementation tests, fuzz target
source, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
backup plan, rollback, or backup tree paths.

## Queue proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0467`.
- `NA-0466 DONE`.
- `NA-0465 DONE`.
- Prior closed items remain closed or blocked as previously recorded.

## Decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- latest decision is D-0920.
- D-0919 exists once.
- D-0920 exists once.
- D-0921 is absent.
- duplicate decision count is zero.

## Scope guard

Run a name-only diff against `origin/main` and verify the changed paths are
exactly:

```text
DECISIONS.md
NEXT_ACTIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0466_closeout_restore_na0467_testplan.md
```

## Link check

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
```

Required:

- `TOTAL_MISSING 0`.

## Leak scan

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Required:

- `SECRET_FINDING_COUNT 0`.

## Overclaim scan

Run an added-line overclaim scan.

Required:

- no affirmative public-readiness claim.
- no affirmative production-readiness claim.
- no affirmative public-internet-readiness claim.
- no affirmative external-review-complete claim.
- no affirmative crypto-complete claim.
- no affirmative KEM-complete claim.
- no affirmative signature-complete claim.
- no affirmative identity-complete claim.
- no affirmative RNG-failure-complete claim.
- no affirmative provider-RNG-complete claim.
- no affirmative secret-material-complete claim.
- no affirmative side-channel-free claim.
- no affirmative vulnerability-free claim.
- no affirmative bug-free claim.
- no affirmative perfect-crypto claim.

## Dependency health

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

## PR body preflight and goal-lint

The closeout PR body must include:

- `Goals: G1, G2, G3, G4, G5`
- `Impact:`
- `No-regression:`
- `Tests/Vectors:`

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY" --scan-overclaims
python3 tools/goal_lint.py
```

Required:

- PR body preflight PASS.
- goal-lint PASS.

## Public-safety

Before merge:

- required PR checks must pass.
- public-safety must be green.

After merge:

- public-safety must be green on the closeout merge commit.

## Closeout boundary

This closeout does not implement NA-0467. Legacy/public-record identity
provider RNG failure implementation remains future work until the restored
NA-0467 directive executes.
