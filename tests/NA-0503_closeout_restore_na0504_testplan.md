Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-20
Goals: G1, G2, G3, G4, G5

# NA-0503 Closeout and NA-0504 Restoration Testplan

## Objective

Verify that NA-0503 is closed only after the authorization PR merges and
post-merge public-safety is green inside the short attach/early-failure window,
then restore the selected NA-0504 implementation successor as the sole READY
item without implementing NA-0504.

## Scope under test

Allowed closeout changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0503_closeout_restore_na0504_testplan.md`

Forbidden closeout changes:

- qsc source, qsc tests, qsc fuzz, qsc Cargo files, root Cargo files,
  dependencies, lockfiles, workflows, scripts, helpers, corpus/vector/input
  files, formal models, refimpl files, services, public docs, backup paths,
  qsl-backup, qwork, qstart, or qresume.

## Required proof

- PR #1278 merged.
- PR #1278 merge commit is `5c39ddba4991`.
- PR #1278 post-merge public-safety completed success on `5c39ddba4991`.
- D-0995 exists once.
- D-0996 is added once.
- NA-0503 is marked DONE.
- NA-0504 is restored as the single READY item.
- NA-0504 exact allowed and forbidden scope matches D-0995.
- no NA-0504 implementation is performed by closeout.

## Validation commands

Run before the closeout PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0503_closeout_restore_na0504_testplan.md
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run exact five-path closeout scope proof, added-line overclaim scan, PR
body preflight, and goal-lint for the closeout PR.

## Pass criteria

- Changed paths are exactly within the five allowed closeout paths.
- READY_COUNT is 1.
- READY item is NA-0504.
- NA-0503 is DONE.
- D-0995 exists exactly once.
- D-0996 exists exactly once.
- no implementation mutation is present.
- no qsc source/test/fuzz/Cargo mutation is present.
- no corpus/vector/input mutation is present.
- no workflow/script/helper/dependency/lockfile mutation is present.
- no formal/refimpl/service/public/backup mutation is present.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no identity-complete claim is introduced.
- no provider-RNG-complete claim is introduced.
- no secret-material-complete claim is introduced.
- no zeroization-complete claim is introduced.
- no memory-erasure-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Stop conditions

- PR #1278 is not merged.
- post-merge public-safety for `5c39ddba4991` is not green.
- more than one READY item exists.
- NA-0504 scope differs from D-0995.
- closeout introduces any implementation, dependency, workflow, script, helper,
  corpus/vector/input, formal, refimpl, service, public, backup, qwork, qstart,
  qresume, or qsl-backup mutation.
- closeout introduces a public/security/completion overclaim.
