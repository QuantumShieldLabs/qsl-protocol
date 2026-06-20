Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-20
Goals: G1, G2, G3, G4, G5

# NA-0504 Closeout and NA-0505 Restoration Testplan

## Objective

Verify that NA-0504 is closed only after PR #1280 is merged and its required
post-merge public-safety/full-suite checks complete successfully, then restore
the selected NA-0505 remote test account / SSH boundary authorization successor
as the sole READY item without performing any remote action.

## Scope under test

Allowed closeout changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0504_closeout_restore_na0505_testplan.md`

Forbidden closeout changes:

- implementation files, qsc source, qsc tests, qsc fuzz, qsc Cargo files, root
  Cargo files, dependencies, lockfiles, workflows, scripts, helpers,
  corpus/vector/input files, formal models, refimpl files, services, public
  docs, backup paths, qsl-backup, qwork, qstart, qresume, or qshell.
- remote SSH setup, remote account creation, SSH key generation or
  installation, local SSH config mutation, remote host mutation, two-machine
  testing, LAN testing, backup, or restore.

## Required proof

- PR #1280 is merged.
- PR #1280 merge commit is `865da9f3e529`.
- PR #1280 implementation commit is `1cb1363f03d8`.
- PR #1280 post-merge public-safety completed success on `865da9f3e529`.
- PR #1280 qsc-adversarial-smoke completed success.
- PR #1280 qsc-linux-full-suite completed success.
- PR #1280 macos-qsc-full-serial completed success.
- `PUBLIC_SAFETY_RED no`.
- `PUBLIC_SAFETY_AMBIGUOUS no`.
- D-0997 exists once.
- D-0998 is added once.
- NA-0504 is marked DONE.
- NA-0505 is restored as the single READY item.
- NA-0505 is authorization-only and no remote action is performed.

## Validation commands

Run before the closeout PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/pr/pr_body.md"
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0504_closeout_restore_na0505_testplan.md
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
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
- READY item is NA-0505.
- NA-0504 is DONE.
- D-0997 exists exactly once.
- D-0998 exists exactly once.
- no implementation mutation is present.
- no qsc source/test/fuzz/Cargo mutation is present.
- no corpus/vector/input mutation is present.
- no workflow/script/helper/dependency/lockfile mutation is present.
- no formal/refimpl/service/public/backup mutation is present.
- no remote SSH setup, remote account creation, SSH key generation or
  installation, local SSH config mutation, remote host mutation, two-machine
  testing, or LAN testing is present.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
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

- PR #1280 is not merged.
- PR #1280 post-merge public-safety is not success.
- PR #1280 qsc-adversarial-smoke, qsc-linux-full-suite, or
  macos-qsc-full-serial is not success.
- more than one READY item exists.
- NA-0505 is restored before NA-0504 is DONE.
- closeout introduces any implementation, qsc, dependency, workflow, script,
  helper, corpus/vector/input, formal, refimpl, service, public, backup, qwork,
  qstart, qresume, qshell, qsl-backup, or remote-action mutation.
- closeout introduces a public/security/completion overclaim.
