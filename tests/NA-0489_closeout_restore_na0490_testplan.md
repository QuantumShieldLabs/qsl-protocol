Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0489 Closeout and NA-0490 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the closeout-only governance change that marks NA-0489 DONE and
restores the selected NA-0490 successor. This closeout must not implement
NA-0490 and must not mutate implementation, corpus, vector, runtime, workflow,
dependency, or backup paths.

## Protected invariants

- qwork proof files exist, parse OK, and match live HEAD/origin before fetch.
- PR #1249 is merged at `6e1004e86b55`.
- PR #1249 post-merge public-safety is completed success.
- PR #1249 post-merge qsc-adversarial-smoke is completed success.
- PR #1249 post-merge qsc-linux-full-suite and macos-qsc-full-serial are
  success or accepted by explicit repo policy.
- D-0967 exists once before closeout.
- D-0968 is absent before closeout and exists once after closeout.
- D-0969 remains absent.
- D352 stopped before closeout because public-safety was still in progress.
- D353 verified PR #1249 public-safety success but stopped on disk pressure and
  response-writing breach.
- D354 reduced `/` below the stop threshold and did not close out NA-0489.
- NA-0489 is marked DONE.
- exactly one READY item remains.
- READY item is NA-0490.
- no implementation mutation is introduced.
- no corpus/vector/input mutation is introduced.
- no qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile,
  qsc-adversarial script, workflow, dependency, lockfile, formal, refimpl,
  service, public-doc, README, START_HERE, backup, qsl-backup, status, plan,
  rollback, qwork, qstart, qresume, or qshell mutation is introduced.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no corpus-complete claim is introduced.
- no vector-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- NA-0490 implementation.
- qsc source/fuzz/Cargo/script mutation.
- qsc fuzz lockfile mutation.
- corpus/vector/input mutation.
- workflow mutation.
- dependency mutation.
- lockfile mutation.
- formal/refimpl/service/public-doc mutation.
- README, START_HERE, or website mutation.
- qsl-server, qsl-attachments, qshield runtime, or qshield-cli mutation.
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree mutation.
- public claim expansion.

## Required startup proof

Verify:

```bash
git status --porcelain=v1 --branch
git diff --name-only
git ls-files --others --exclude-standard
git rev-parse HEAD
git rev-parse origin/main
```

Required initial queue and decision proof:

- READY_COUNT 1.
- READY NA-0489.
- NA-0488 DONE.
- NA-0487 DONE.
- D-0965 count 1.
- D-0966 count 1.
- D-0967 count 1.
- D-0968 count 0.
- D-0969 count 0.
- duplicate decision count 0.

## D354 disk verification

Run:

```bash
df -h /
df -h /backup/qsl
df -i /
df -i /backup/qsl
```

Required:

- `/` is below 95%.
- `/backup/qsl` is mounted with ample free space.
- active NA-0489 worktree and qwork proofs exist.
- D352, D353, and D354 response files exist.
- D354 archive indexes exist.
- archived D354 worktrees exist under `/backup/qsl/qbuild-archives/20260616_D354/worktrees/`.
- source worktree directories archived by D354 are absent from `/srv/qbuild/work/`.

## PR #1249 implementation proof

Verify:

- PR #1249 state is MERGED.
- merge commit begins with `6e1004e86b55`.
- post-merge public-safety completed success.
- post-merge qsc-adversarial-smoke completed success.
- no required check completed failure.
- PR #1249 changed exactly the six implementation/governance paths recorded by
  D-0967.
- all required NA-0489 markers are present.
- no corpus, input, qsc source/helper, qsc fuzz Cargo, qsc fuzz lockfile, root
  lockfile, script, workflow, dependency, formal, refimpl, service, public-doc,
  or backup path was changed by PR #1249.

## Required validation

Run before PR creation and again after the closeout patch where applicable:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body-file>
python3 tools/goal_lint.py
cargo fmt --check
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run inherited qsc provider-RNG, key-lifecycle, and provider-error tests as
needed by the closeout directive:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Run exact closeout scope guard against the closeout branch:

```bash
git diff --name-only origin/main...HEAD
```

Required changed paths:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0489_closeout_restore_na0490_testplan.md`

## Required final queue proof

- READY_COUNT 1.
- READY NA-0490.
- NA-0489 DONE.
- D-0967 count 1.
- D-0968 count 1.
- D-0969 count 0.
- duplicate decision count 0.

## PR requirements

The closeout PR body must include:

- `Goals: G1, G2, G3, G4, G5`
- Impact
- No-regression
- Tests/Vectors
- closeout-only statement.
- PR #1249 public-safety success.
- qsc-adversarial-smoke success.
- D354 disk-pressure recovery completed.
- NA-0489 DONE.
- NA-0490 READY.
- no implementation mutation.
- no source/fuzz/Cargo/script/workflow/vector/refimpl/formal mutation.
- no public overclaim.

## Post-merge validation

After merge:

- fetch origin.
- fast-forward local main.
- verify READY_COUNT 1.
- verify READY NA-0490.
- verify NA-0489 DONE.
- verify D-0968 exists once.
- verify D-0969 absent.
- verify public-safety on the closeout merge commit is success.
- verify qsc-adversarial-smoke success or accepted by repo policy.
- do not run qwork post-merge.
