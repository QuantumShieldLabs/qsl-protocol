# NA-0446 QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0446 implements only the exact qsc test path authorized by
NA-0445 and proves bounded internal cleanup, no-mutation, redaction, and
encrypted-at-rest evidence without changing runtime, crypto, dependency,
workflow, public, service, or backup/local-ops scope.

## Protected invariants

- READY_COUNT remains 1.
- NA-0446 remains READY until optional closeout.
- D-0879 exists once after the implementation patch.
- D-0880 remains absent until optional closeout.
- D-0877 and D-0878 remain present once.
- Exact implemented qsc test path is
  `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`.
- No tests outside the exact authorized qsc test file are mutated.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target,
  vector, formal model, service, public-surface, or backup/local-ops path is
  changed.
- Direct runtime memory overwrite, allocator behavior, `Drop` behavior, and
  side-channel behavior are not claimed.
- No all-material coverage is claimed.
- No secret-material-complete coverage is claimed.
- qshield-cli remains demo-local boundary evidence only.
- refimpl remains deferred for cleanup/zeroization scope.
- Selected successor is
  `NA-0447 -- QSL RNG Failure Behavior Scope Authorization Plan`.
- No public claim expansion is introduced.

## Allowed scope

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`
- `docs/governance/evidence/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_harness.md`
- `tests/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime code, crypto code, dependency metadata,
Cargo manifests, lockfiles, workflows, executable tests outside the exact qsc
test file, fuzz target source, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, and public technical paper work.

## Required test names

The exact qsc test must include:

- `pending_handshake_secret_cleanup_success_and_reject_boundaries`
- `session_secret_store_inserted_only_after_success_and_encrypted_at_rest`
- `key_lifecycle_output_redaction_sentinel_scan`
- `reject_paths_preserve_pending_session_vault_state`
- `session_and_vault_encrypted_at_rest_boundaries`

Optional implemented test:

- `vault_passphrase_redaction_and_no_plaintext_boundary`

## Required markers

The exact qsc test output or source must include:

- `NA0446_KEY_LIFECYCLE_TEST_IMPLEMENTATION_OK`
- `NA0446_PENDING_SECRET_CLEANUP_SUCCESS_BOUNDARY_OK`
- `NA0446_REJECT_NO_MUTATION_BOUNDARY_OK`
- `NA0446_SESSION_SECRET_STORE_BOUNDARY_OK`
- `NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK`
- `NA0446_REDACTION_SENTINEL_BOUNDARY_OK`
- `NA0446_NO_RUNTIME_HOOK_USED_OK`
- `NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK`
- `NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK`
- `NA0446_REFIMPL_SCOPE_DEFERRED_OK`

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0446 before optional closeout.
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
- D-0879 exists once after patching.
- D-0880 absent before optional closeout.
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
- `docs/governance/evidence/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_harness.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`
- `tests/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_testplan.md`

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
- classifier does not report forbidden runtime, crypto, dependency, Cargo,
  lockfile, workflow, fuzz-target, vector, formal, public, service, or backup
  mutation.

## Required validation commands

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
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

Run local qsc adversarial script if feasible:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

Required:

- new qsc test passes and emits required NA-0446 markers;
- inherited provider-error no-mutation test passes;
- qsc `send_commit` passes;
- provider `pqkem768` passes;
- adversarial script syntax checks pass;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes;
- formal checks pass.

If local qsc adversarial execution reaches local cargo-fuzz unavailability
after stable Rust phases and provider-error step pass, record exact output and
require PR CI `qsc-adversarial-smoke`.

## Public claim boundary

Confirm:

- no production-readiness claim is introduced;
- no public-readiness claim is introduced;
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
  readiness;
- refimpl evidence is not represented as qsc runtime cleanup proof.

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

- implementation PR merged;
- D-0879 exists once on main;
- D-0880 remains absent until optional closeout;
- public-safety is green on the merge commit;
- qsc-adversarial-smoke is green or explicitly accepted under required check
  shape;
- READY remains NA-0446 until optional closeout;
- no qwork is run post-merge.
