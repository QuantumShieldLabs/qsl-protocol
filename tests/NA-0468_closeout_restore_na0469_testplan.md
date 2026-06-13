Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0468 Closeout Restore NA-0469 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0468 after qsl-protocol evidence PR #1205 merged at
`5d6ede567296` and post-merge public-safety completed success, then restore
`NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam
Implementation Harness` as the sole READY successor.

## Protected invariants

- Exactly one READY item remains.
- NA-0468 is DONE.
- NA-0469 is READY.
- D-0923 exists once.
- D-0924 exists once after closeout.
- D-0925 is absent before NA-0469 begins.
- No duplicate decision IDs exist.
- NA-0468 authorization evidence remains bounded to PR #1205 and D-0923.
- NA-0469 is restored but not implemented by this closeout.
- No qsc source mutation occurs in this closeout.
- No executable qsc test mutation occurs in this closeout.
- No runtime behavior mutation occurs in this closeout.
- No crypto behavior mutation occurs in this closeout.
- No dependency, Cargo manifest, lockfile, workflow, executable test, fuzz target, vector, formal model, refimpl, service, qshield runtime, qshield-cli, website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup-status, backup-plan, rollback, or backup-tree mutation occurs.
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
- `tests/NA-0468_closeout_restore_na0469_testplan.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to qsc source, runtime behavior, crypto behavior, dependencies, Cargo
manifests, lockfiles, workflows, executable implementation tests, fuzz target
source, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
backup plan, rollback, or backup tree paths.

## Evidence PR #1205 merge proof

Run:

```bash
gh pr view 1205 --repo QuantumShieldLabs/qsl-protocol \
  --json number,state,mergedAt,mergeCommit,headRefOid,title,url,statusCheckRollup
```

Required:

- state is MERGED.
- merge commit begins with `5d6ede567296`.
- evidence head is recorded for closeout evidence.

## Public-safety proof for PR #1205

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status \
  --sha 5d6ede567296
```

Required:

- `public-safety` is completed success.
- `qsc-adversarial-smoke` is completed success or accepted by repo policy.
- `qsc-linux-full-suite` is completed success, skipped, or neutral if accepted by repo policy for docs-only evidence scope.
- `macos-qsc-full-serial` is completed success, skipped, or neutral if accepted by repo policy for docs-only evidence scope.
- no required attached check completed failure.
- no public-safety red or ambiguous result exists.

## Closeout path guard

Run a name-only diff against `origin/main` and include untracked files.

Required changed paths are exactly:

```text
DECISIONS.md
NEXT_ACTIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0468_closeout_restore_na0469_testplan.md
```

No other tracked or untracked repo path may be changed.

## Queue proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0469`.
- `NA-0468 DONE`.
- NA-0467 through NA-0435 remain DONE.
- NA-0434 and NA-0429 remain BLOCKED.

## Decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- D-0923 exists once.
- D-0924 exists once after closeout.
- D-0925 is absent.
- duplicate decision count is zero.

## No source, test, workflow, or dependency mutation

The scope guard must prove no mutation to:

- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `tools/refimpl/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsl/qsl-client/qsc/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `.github/workflows/**`
- fuzz targets
- vectors
- formal models
- qsl-server paths
- qsl-attachments paths
- qshield runtime paths
- qshield-cli paths
- website, public docs, README, or START_HERE
- qwork/qstart/qresume/qshell
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup tree paths

## No public claim expansion

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

## NA-0469 restoration checks

Required:

- NA-0469 title is exactly `QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness`.
- NA-0469 status is READY.
- NA-0469 allowed scope includes the exact implementation paths selected by D-0923:
  - `qsl/qsl-client/qsc/src/main.rs`
  - `qsl/qsl-client/qsc/src/identity/mod.rs`
  - `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`
  - `docs/governance/evidence/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_harness.md`
  - `tests/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- NA-0469 acceptance criteria require production semantics unchanged when the seam is inactive.
- NA-0469 acceptance criteria require bounded no-partial-rotation-state testing.
- NA-0469 does not authorize qshield-cli, refimpl, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, qsl-server, qsl-attachments, public docs, website, backup, restore, qsl-backup, qwork/qstart/qresume/qshell, backup status, backup plan, rollback, or backup tree mutation.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
cargo fmt --check
```

Run deterministic local-link, leak, overclaim, PR body preflight, goal-lint, and
scope-guard checks before opening the closeout PR.

## Public claim boundary

This closeout is internal governance evidence only. It does not implement
NA-0469, does not prove CLI identity rotation provider RNG behavior, and does
not expand public security claims. Cargo audit green is dependency-health
evidence only.
