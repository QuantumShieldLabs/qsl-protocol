Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-19
Replaces: n/a
Superseded-By: n/a

# NA-0499 Closeout and NA-0500 Restoration Testplan

## Purpose

Validate that NA-0499 is closed only after PR #1269 merged and post-merge
public-safety completed success for `c06e20a66283`, and that the selected
NA-0500 qsc secret-material diagnostic / no-output boundary test implementation
successor is restored as the sole READY item without implementing NA-0500.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0499_closeout_restore_na0500_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- `/` usage is below the 95% stop threshold.
- PR #1269 is merged at `c06e20a66283`.
- D378 response exists.
- D-0987 exists exactly once.
- D-0988 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0499.
- Post-merge public-safety on `c06e20a66283` is success.
- qsc-adversarial-smoke on `c06e20a66283` is success.
- qsc-linux-full-suite on `c06e20a66283` is success or accepted by repository
  policy.
- macos-qsc-full-serial on `c06e20a66283` is success or accepted by repository
  policy.

## Required inheritance proof

- PR #1269 merged at `c06e20a66283`.
- D378 response exists and is consumed.
- D378 stopped because post-merge public-safety had not attached or completed
  green during the short attach window; no completed failures were observed.
- D-0987 records the NA-0499 side-channel / secret-material lifecycle assurance
  scope authorization.
- NA-0499 consumed NA-0498/D377 inheritance.
- Lifecycle inventory was completed.
- Evidence classification was completed.
- Option review was completed.
- Hostile cryptographer, red-team, production SRE, release-claim, and
  prioritization reviews were completed.
- Primary classification is `SECRET_MATERIAL_DIAGNOSTIC_NO_OUTPUT_TEST_READY`.
- Selected successor is `NA-0500 -- QSL qsc Secret-Material Diagnostic /
  No-Output Boundary Test Implementation Harness`.
- Future test path is
  `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`.
- NA-0499 remained READY until closeout.

## Required closeout changes

- Mark NA-0499 DONE.
- Add D-0988: `NA-0499 closeout and NA-0500 restoration`.
- Restore `NA-0500 -- QSL qsc Secret-Material Diagnostic / No-Output Boundary
  Test Implementation Harness` as the sole READY successor.
- Update TRACEABILITY for PR #1269 evidence, D378 response, D-0987, PR #1269
  post-merge public-safety proof, D-0988, the closeout PR, and NA-0500
  successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden closeout changes

- Do not implement NA-0500.
- Do not mutate qsc source, tests, fuzz target code, fuzz Cargo metadata, or
  lockfiles.
- Do not mutate workflows.
- Do not mutate scripts or helpers.
- Do not mutate validator scripts.
- Do not mutate corpus, vector, input, internal manifest, dependency, lockfile,
  formal, refimpl, service, public, backup, qsl-backup, qwork, qstart, qresume,
  or qshell paths.
- Do not move, archive, or delete files.
- Do not make a public-readiness claim.
- Do not make a production-readiness claim.
- Do not make a public-internet-readiness claim.
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a KEM-complete claim.
- Do not make a signature-complete claim.
- Do not make an identity-complete claim.
- Do not make a provider-RNG-complete claim.
- Do not make a secret-material-complete claim.
- Do not make a zeroization-complete claim.
- Do not make a memory-erasure-complete claim.
- Do not make a side-channel-free claim.
- Do not make a vulnerability-free claim.
- Do not make a bug-free claim.
- Do not make a perfect-crypto claim.

## Validation commands

Run before closeout mutation:

```bash
git diff --check
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/closeout/pr_body.md"
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Acceptance

NA-0499 is accepted as DONE only when PR #1269 merge proof, D378 inheritance,
D-0987, PR #1269 post-merge public-safety success, qsc-adversarial-smoke
success, qsc-linux-full-suite success or policy acceptance,
macos-qsc-full-serial success or policy acceptance, validator proof, local
validation, exact five-path closeout scope, and one-READY queue proof all pass.
NA-0500 is accepted as READY only as the selected qsc secret-material diagnostic
/ no-output boundary test implementation successor; no NA-0500 execution is
performed by this closeout.
