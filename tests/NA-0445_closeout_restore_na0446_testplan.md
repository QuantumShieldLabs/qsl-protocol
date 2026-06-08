# NA-0445 Closeout and NA-0446 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0445 is closed only after PR #1159 merged and post-merge
public-safety passed, and that exactly one successor, NA-0446, is restored as
READY without implementing NA-0446.

## Protected invariants

- READY_COUNT remains 1.
- NA-0445 is DONE after closeout.
- NA-0446 is READY after closeout.
- D-0877 exists once.
- D-0878 exists once after closeout.
- D-0879 remains absent until future NA-0446 work.
- Exact future qsc test path remains
  `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`.
- No NA-0446 implementation occurs in this closeout.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0445_closeout_restore_na0446_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, public technical paper work, and
NA-0446 implementation.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0446.
- NA-0445 DONE.
- NA-0444 DONE.
- NA-0443 DONE.
- NA-0442 DONE.
- NA-0441 DONE.
- NA-0440 DONE.
- NA-0439 DONE.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0877 exists once.
- D-0878 exists once.
- D-0879 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0445_closeout_restore_na0446_testplan.md`

## Link, leak, classifier, PR-body, and goal checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
```

Required:

- no whitespace errors;
- link check passes;
- added-line leak scan has zero findings;
- added-line overclaim scan has zero affirmative findings;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- goal-lint passes;
- classifier reports governance/docs scope only.

## Dependency and qsc checks

Run:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required:

- inherited provider-error no-mutation test passes and emits NA-0436 markers;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches.

## Public claim boundary

Confirm:

- no production-readiness claim is introduced;
- no public-internet-readiness claim is introduced;
- no external-review-complete claim is introduced;
- no crypto-complete claim is introduced;
- no secret-material-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed;
- cargo audit green is dependency-health evidence only;
- qshield-cli demo evidence is not represented as qsc runtime or service
  readiness.

## Backup / restore boundary

Confirm:

- no backup was run;
- no restore was run;
- no sudo was run;
- qsl-backup was not mutated;
- backup status files and backup plan files were not mutated;
- rollback subtree paths and `/backup/qsl` were not mutated.

## Post-merge checks

After merge, verify:

- closeout PR merged;
- READY is NA-0446;
- NA-0445 is DONE;
- D-0878 exists once on main;
- D-0879 remains absent until future NA-0446 work;
- public-safety is green on the merge commit;
- qsc-adversarial-smoke is green or explicitly recorded if docs-only CI skips
  full-suite jobs under public-safety policy;
- no qwork is run post-merge.
