Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0477 KEM / Signature / Transcript Formal Model Mapping Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only authorization plan for mapping NA-0476 qsc KEM,
signature, transcript, replay, suite-confusion, stale public-record, and
no-session-mutation negative evidence into a future bounded formal model.

This lane must not implement or mutate formal models.

## Protected invariants

- No formal model file is mutated.
- No qsc runtime/source file is mutated.
- No crypto implementation is mutated.
- No dependencies, Cargo manifests, lockfiles, or workflows are mutated.
- No executable tests, fuzz targets, or vectors are mutated.
- No refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
  website, public docs, README, or START_HERE path is mutated.
- No qwork/qstart/qresume/qshell path is mutated.
- No backup or restore is run.
- No qsl-backup, backup status, backup plan, rollback, systemd, timer, fstab,
  or backup tree path is mutated.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No formal-proof-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Exactly one READY item remains before optional closeout.

## Allowed scope

- `docs/governance/evidence/NA-0477_qsl_kem_signature_transcript_formal_model_mapping_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- formal model mutation;
- qsc runtime/source mutation;
- crypto implementation mutation;
- dependency, Cargo manifest, lockfile, or workflow mutation;
- executable test, fuzz target, or vector mutation;
- refimpl mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, or START_HERE mutation;
- qwork, qstart, qresume, or qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation;
- public technical paper work;
- durable Director State Index output.

## Startup proof

Required:

- qwork proof files exist and verify `startup_result=OK`.
- proof HEAD and proof `origin_main` match live `HEAD` and live `origin/main`
  before fetch.
- fetch does not advance `origin/main`.
- PR #1223 is MERGED at `fcf434b1746b`.
- startup queue is READY_COUNT 1 and READY NA-0477.
- D-0940 exists once.
- D-0941 exists once.
- D-0942 is absent.
- duplicate decision count is zero.
- current main public-safety is green.

## Formal inventory checks

Read-only inventory must cover:

- `formal/README.md`
- `formal/model_scka_bounded.py`
- `formal/model_suite2_negotiation_bounded.py`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`

Required classification:

- no existing model directly maps all KEM/signature/transcript/identity binding
  properties;
- the qsc suite-id model directly covers suite-id/downgrade and abstract
  transcript/key-context only;
- a new bounded qsc binding model is selected as the future exact path.

## qsc behavior mapping checks

Read-only inventory must map:

- wrong KEM public key reject;
- stale KEM public record reject;
- wrong KEM ciphertext reject;
- wrong signature identity reject;
- cross-message signature replay reject;
- transcript mutation reject;
- transcript replay reject;
- suite confusion reject;
- stale public-record reject;
- no session mutation on selected rejects;
- provider RNG failure no partial state as related but out of the selected first
  model;
- suite-id fail-closed/no-mutation as already modeled and referenced.

## Successor selection checks

Required selected classification:

`FORMAL_MAPPING_QSC_BINDING_MODEL_IMPLEMENTATION_READY`

Required selected successor:

`NA-0478 -- QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

Required future paths:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_harness.md`
- `tests/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Local validation

Run:

```bash
git diff --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
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
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If local cargo-fuzz is unavailable during qsc adversarial smoke, record exact
output and rely on PR CI `qsc-adversarial-smoke`.

## Scope guard

Before PR:

- changed paths must be limited to the five allowed NA-0477 paths;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/executable-test/fuzz/
  vector/formal/refimpl/service/public/backup mutation may appear;
- queue proof must show READY_COUNT 1 and READY NA-0477;
- D-0942 must exist once after patch;
- D-0943 must be absent;
- duplicate decision count must be zero.

## Link, leak, claim, classifier, and PR-body checks

Required:

- deterministic local markdown link check reports `TOTAL_MISSING 0`;
- leak-safe scan reports no secret findings in changed files;
- overclaim scan reports no affirmative public/crypto/formal completion claim;
- classifier reports docs/governance-only scope;
- PR body preflight and goal-lint pass;
- PR body includes `Goals: G1, G2, G3, G4, G5`.

## Public claim boundary

This testplan validates internal governance authorization only.

- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No formal-proof-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
