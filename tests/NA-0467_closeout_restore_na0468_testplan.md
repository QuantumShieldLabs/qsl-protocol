Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0467 Closeout Restore NA-0468 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0467 after qsl-protocol PR #1203 merged at `79c35061cc74`
and post-merge public-safety completed success, then restore
`NA-0468 -- QSL qsc CLI Identity Rotation Provider RNG Failure Scope
Authorization Plan` as the sole READY successor.

## Protected invariants

- Exactly one READY item remains.
- NA-0467 is DONE.
- NA-0468 is READY.
- D-0921 exists once.
- D-0922 exists once after closeout.
- D-0923 is absent before NA-0468 begins.
- No duplicate decision IDs exist.
- NA-0467 implementation evidence remains bounded to PR #1203 and D-0921.
- NA-0468 is restored but not implemented by this closeout.
- No qsc source mutation occurs in this closeout.
- No executable qsc test mutation occurs in this closeout.
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
- `tests/NA-0467_closeout_restore_na0468_testplan.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to qsc source, runtime behavior, crypto behavior, dependencies, Cargo
manifests, lockfiles, workflows, executable implementation tests, fuzz target
source, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
backup plan, rollback, or backup tree paths.

## qwork proof verification

Read, but do not regenerate, the qwork proof files:

```text
/srv/qbuild/work/NA-0467/.qwork/startup.qsl-protocol.kv
/srv/qbuild/work/NA-0467/.qwork/startup.qsl-protocol.json
```

Required:

- startup result is OK.
- lane is NA-0467.
- repo is qsl-protocol.
- proof path is `/srv/qbuild/work/NA-0467/qsl-protocol`.
- proof HEAD equals live HEAD before fetch.
- proof origin/main equals live origin/main before fetch.
- clean-state fields are all yes.
- READY_COUNT is 1.
- queue top READY is NA-0467.
- requested lane status is READY.
- JSON mirrors the `.kv` proof for the required fields.

## PR #1203 merge proof

Run:

```bash
gh pr view 1203 --repo QuantumShieldLabs/qsl-protocol \
  --json number,state,mergedAt,mergeCommit,headRefOid,title,url,statusCheckRollup
```

Required:

- state is MERGED.
- merge commit begins with `79c35061cc74`.
- implementation head is recorded for evidence.

## Public-safety proof for PR #1203

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status \
  --sha 79c35061cc74ba7bc30233f92bfccf2980862dbd
```

Required:

- `public-safety` is completed success.
- `qsc-adversarial-smoke` is completed success or accepted by repo policy.
- `qsc-linux-full-suite` is completed success if attached and required.
- `macos-qsc-full-serial` is completed success if attached and required.
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
tests/NA-0467_closeout_restore_na0468_testplan.md
```

No other tracked or untracked repo path may be changed.

## Queue proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0468`.
- `NA-0467 DONE`.
- NA-0466 through NA-0435 remain DONE.
- NA-0434 and NA-0429 remain BLOCKED.

## Decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- D-0921 exists once.
- D-0922 exists once after closeout.
- D-0923 is absent.
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
- backup, restore, qsl-backup, backup status, backup plan, rollback, or
  backup tree paths

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

## NA-0468 restoration checks

Verify the restored NA-0468 block preserves:

- authorization-only scope.
- no runtime mutation unless a later exact directive authorizes it.
- no crypto mutation unless a later exact directive authorizes it.
- no dependency, Cargo, lockfile, workflow, test, fuzz, vector, or formal
  mutation unless a later exact directive authorizes it.
- no public overclaim.
- exactly one READY item.

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

## Post-merge public-safety checks

After the closeout PR merges:

- closeout PR state is MERGED.
- final main is clean.
- READY_COUNT is 1.
- READY is NA-0468.
- NA-0467 is DONE.
- D-0922 exists once.
- D-0923 is absent.
- duplicate decision count is zero.
- changed paths from PR #1203 merge to closeout merge are exactly the five
  closeout paths.
- public-safety completes success on the closeout merge commit.
- qsc-adversarial-smoke is green or accepted by public-safety policy.
- docs-only full-suite skip shape is accepted if applicable.
