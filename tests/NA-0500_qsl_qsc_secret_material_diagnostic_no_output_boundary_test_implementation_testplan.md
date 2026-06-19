Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0500 qsc Secret-Material Diagnostic No-Output Boundary Test Implementation Testplan

## Objective

Implement and validate a bounded qsc integration test that exercises selected real qsc reject/error/diagnostic surfaces and asserts captured stdout, stderr, error text, and test-visible diagnostic strings do not expose synthetic secret-shaped markers or high-entropy-looking material.

## Protected invariants

- qsc reject/error paths remain deterministic and fail closed.
- Selected diagnostics do not expose private-key, passphrase, KEM secret, signature secret, shared-secret, backup/recovery key, runtime/service secret, private endpoint, user data, or operator data marker labels.
- Synthetic scanner fixtures contain no real secrets.
- The lane does not imply public-readiness, crypto-complete, secret-material-complete, zeroization-complete, memory-erasure-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto status.
- Exactly one READY item remains mandatory during implementation.

## Allowed scope

- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `docs/governance/evidence/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_harness.md`
- `tests/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No qsc source mutation. No existing qsc test mutation outside the new test file. No qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, root Cargo lockfile, dependency, corpus, vector, input, validator script, qsc-adversarial script, workflow, formal model, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, public doc, website, backup, qsl-backup, backup status/plan, rollback, qwork, qstart, qresume, qshell, archive, move, or delete mutation.

## diagnostic path selection

Required selected paths:

- CLI invalid config reject: `config set policy-profile bad`.
- CLI util-sanitize usage reject: `util sanitize` without `--print`, including stderr usage text.
- qsc handshake reject: `handshake init` after local identity rotation and peer route setup but without a peer identity pin, producing `identity_unknown` and `handshake_reject`.

At least two real qsc output/error surfaces must be exercised. At least one path must involve CLI stdout/stderr capture. At least one path must involve handshake/reject diagnostics if feasible.

## output scanner behavior

The scanner must reject synthetic labels for:

- private-key marker;
- passphrase marker;
- KEM secret marker;
- signature secret marker;
- shared-secret marker;
- backup/recovery key marker;
- runtime/service secret marker;
- private endpoint marker;
- operator/user data marker;
- route-token marker;
- qsp session-store key marker;
- pending-handshake secret marker;
- identity-signing secret marker.

It must also reject long high-entropy-looking tokens while avoiding real secret fixtures.

## synthetic secret marker fail proof

Run `diagnostic_scrubber_rejects_synthetic_secret_markers`. Expected result: every synthetic marker label produces at least one scanner finding, and the test prints the NA-0500 private-key/passphrase/KEM/signature/shared-secret marker-absence OK markers.

## qsc diagnostic pass proof

Run `reject_diagnostics_do_not_contain_secret_markers`. Expected result: selected qsc diagnostics pass the scanner and the test prints:

- `NA0500_NO_SECRET_OUTPUT_BOUNDARY_OK`
- `NA0500_DIAGNOSTIC_REJECT_PATHS_CHECKED_OK`

## inherited key lifecycle/provider-error tests

Run:

- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`

Both must pass.

## validator scans

Run:

- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`

Both must pass with zero findings.

## audit/fmt checks

Run before PR:

- `git diff --check`
- exact allowed path scope guard
- link-check
- leak-scan
- added-line overclaim scan
- classifier
- PR body preflight
- goal-lint
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

If the local full qsc adversarial script reaches missing `cargo-fuzz`, record exact output and rely on PR CI qsc-adversarial-smoke only if green.

## public claim boundary

NA-0500 is selected internal qsc diagnostic/no-output evidence only. It makes no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, provider-RNG-complete, secret-material-complete, zeroization-complete, memory-erasure-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## closeout prerequisites

Close out to NA-0501 only after the implementation PR merges and post-merge public-safety is attached and green within the short attach/early-failure window. If public-safety remains running but healthy after the short window, stop and hand off closeout.

## post-fix hardening review

Correctness under stress:
The test must fail if selected qsc diagnostic output contains a forbidden synthetic secret marker or long high-entropy-looking token.

Minimality:
The only implementation path is the new qsc integration test. No qsc source, Cargo, dependency, workflow, script, helper, corpus, vector, input, refimpl, formal, service, public, or backup path changes are allowed.

Maintainability:
The scanner is local to the integration test, uses obvious synthetic labels, and reuses existing qsc integration-test helpers.

Coverage quality:
The test exercises real qsc output surfaces and proves the scanner fails on synthetic marker fixtures; it is not a hardcoded-string-only proof.

Cross-lane stability:
Validation includes inherited qsc lifecycle/provider-error tests and Linux-friendly shell syntax checks. macOS/Linux CI remains authoritative for PR protection.
