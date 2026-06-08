# NA-0445 QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Scope Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0445 consumes the NA-0444 evidence policy, classifies qsc
key-lifecycle cleanup / zeroization testability, selects exact future qsc test
scope, and preserves no implementation mutation or public overclaim.

## Protected invariants

- READY_COUNT remains 1.
- NA-0445 is READY until optional closeout.
- D-0877 exists once after the evidence patch.
- D-0878 remains absent until optional closeout.
- NA-0444 DONE remains true.
- NA-0443 DONE remains true.
- NA-0442 DONE remains true.
- NA-0441 DONE remains true.
- NA-0434 and NA-0429 remain BLOCKED.
- Selected primary classification is
  `QSC_ZEROIZATION_TEST_SCOPE_IMPLEMENTATION_READY`.
- Selected successor is
  `NA-0446 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness`.
- Exact future qsc test path is
  `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`.
- Direct runtime memory overwrite, side-channel, all-material, refimpl, and
  qshield-cli coverage remain out of scope.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, current executable
  test, fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_plan.md`
- `tests/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source outside the future NA-0446 exact
scope, fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
runtime, qshield-cli runtime/demo code, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, and backup tree paths.

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
- READY NA-0445.
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
- D-0875 exists once.
- D-0876 exists once.
- D-0877 exists once after patching.
- D-0878 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_testplan.md`

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

## Dependency, provider-error, qsc, refimpl, and formal checks

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- adversarial script syntax checks pass;
- inherited provider-error no-mutation test passes and emits NA-0436 markers;
- qsc `send_commit` passes;
- provider `pqkem768` passes;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes;
- formal checks pass.

If local qsc adversarial execution reaches a local `cargo fuzz` availability
limit after pre-fuzz phases pass, record the exact output and rely on PR CI
qsc-adversarial-smoke as cargo-fuzz-backed proof.

## Test-scope checks

Confirm:

- pending handshake material is classified as implementation-ready for bounded
  store cleanup/no-mutation tests through existing APIs;
- session/shared-secret store material is classified as implementation-ready
  for insertion/absence/encrypted-at-rest tests through existing APIs;
- vault/passphrase/runtime key material is classified as partial because direct
  memory zeroization/drop proof requires runtime hooks;
- redaction/logging is classified as ready for bounded output tests;
- exact future test path is
  `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`;
- NA-0446 implementation scope excludes runtime, crypto, dependency, Cargo,
  lockfile, workflow, fuzz target, vector, formal, public, service, backup,
  qshield-cli, and refimpl mutation unless future exact scope changes.

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
- formal/model checks passing remains bounded evidence;
- qshield-cli demo evidence is not represented as qsc runtime or service
  readiness;
- secret-material lifecycle gaps are called gaps or residuals, not completions.

## Backup / restore boundary

Confirm:

- no backup was run;
- no restore was run;
- no sudo was run;
- qsl-backup was not mutated;
- backup status files and backup plan files were not mutated;
- rollback subtree paths and `/backup/qsl` were not mutated;
- qsl-backup checksum and source-list inclusion count remain boundary evidence
  only.

## Post-merge checks

After merge, verify:

- evidence PR merged;
- D-0877 exists once on main;
- D-0878 remains absent until optional closeout;
- public-safety is green on the merge commit;
- qsc-adversarial-smoke is green or explicitly recorded if docs-only CI skips
  full-suite jobs under public-safety policy;
- READY remains NA-0445 until optional closeout;
- no qwork is run post-merge.
