Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0476 qsc KEM / Signature / Transcript Binding Negative Test Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qsc integration-test implementation selected by NA-0475.
The lane must add only the selected executable qsc test file plus governance
evidence, then prove KEM, signature, transcript, replay, suite-confusion, stale
public-record, and no-session-mutation negative cases reject without runtime or
crypto source mutation.

## Protected invariants

- qsc runtime/source files are unchanged outside the selected test file.
- qsc crypto implementation is unchanged.
- dependencies, Cargo manifests, lockfiles, and workflows are unchanged.
- refimpl, fuzz targets, vectors, and formal models are unchanged.
- qsl-server, qsl-attachments, qshield, qshield-cli, website, public docs,
  README, and START_HERE are unchanged.
- qwork/qstart/qresume/qshell are not mutated.
- backup and restore are not run.
- qsl-backup, backup status, backup plan, rollback, and backup tree paths are
  not mutated.
- Rejected negative cases do not create completed sessions.
- Existing session bytes are not mutated by the stale public-record reject
  case.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Exactly one READY item remains before optional closeout.

## Allowed scope

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `docs/governance/evidence/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime/source mutation outside the selected test file.
- qsc crypto implementation mutation.
- dependency, Cargo manifest, lockfile, or workflow mutation.
- fuzz target, vector, or formal model mutation.
- refimpl mutation.
- qsl-server, qsl-attachments, qshield, qshield-cli, website, public docs,
  README, or START_HERE mutation.
- qwork, qstart, qresume, or qshell mutation.
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation.
- public technical paper work.
- durable Director State Index output.

## KEM negative tests

Required checks:

- wrong peer KEM public key rejects through identity/public-record pin mismatch;
- stale KEM public record rejects after identity rotation while preserving an
  existing accepted session;
- corrupted B1 KEM ciphertext rejects fail-closed;
- no B1/A2 or completed session is created by rejected cases.

Required markers:

- `NA0476_KEM_WRONG_PUBLIC_KEY_REJECT_OK`
- `NA0476_KEM_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0476_KEM_WRONG_CIPHERTEXT_REJECT_OK`

## Signature negative tests

Required checks:

- wrong signature public record rejects after B1 signature verification;
- A2 signature replayed into a B1 signature field rejects in B1 signature
  verification;
- rejected signature cases do not emit A2 or create completed sessions.

Required markers:

- `NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK`
- `NA0476_SIGNATURE_CROSS_MESSAGE_REPLAY_REJECT_OK`

## Transcript / replay / suite negative tests

Required checks:

- B1 transcript-bound field mutation rejects;
- A1 replay rejects while responder pending state exists;
- downgrade-style wrong-suite parameter block rejects in suite-required mode.

Required markers:

- `NA0476_TRANSCRIPT_MUTATION_REJECT_OK`
- `NA0476_TRANSCRIPT_REPLAY_REJECT_OK`
- `NA0476_SUITE_CONFUSION_REJECT_OK`

## Stale public-record negative tests

Required checks:

- identity rotation with stale trusted peer record rejects;
- current accepted session bytes remain unchanged on the stale-record reject.

Required marker:

- `NA0476_STALE_PUBLIC_RECORD_REJECT_OK`

## No session mutation tests

Required checks:

- pre-session rejects leave `qsp_sessions/*.qsv` absent for the affected peer;
- stale-record reject leaves the pre-existing Bob session blob unchanged;
- rejected cases do not emit handshake completion or qsp plaintext output.

Required marker:

- `NA0476_NEGATIVE_TESTS_NO_SESSION_MUTATION_OK`

## Inherited tests

Run inherited qsc tests:

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

## Root audit

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Root audit green is dependency-health evidence only.

## Nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Nested audit green is dependency-health evidence only.

## qsc adversarial check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## Formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

## Scope guard

Required before PR:

- `git diff --check`
- changed paths limited to the six allowed NA-0476 implementation/governance
  paths
- no qsc `src/**` mutation
- no Cargo/lockfile/workflow/refimpl/fuzz/vector/formal/service/public/backup
  mutation
- queue proof: READY_COUNT 1, READY NA-0476
- decision proof: D-0940 once, D-0941 absent, duplicate decision count zero

## Public claim boundary

Added lines and PR body must not introduce public-readiness claims.
Added lines and PR body must not introduce production-ready claims.
Added lines and PR body must not introduce external-review-complete claims.
Added lines and PR body must not introduce crypto-complete claims.
Added lines and PR body must not introduce KEM-complete claims.
Added lines and PR body must not introduce signature-complete claims.
Added lines and PR body must not introduce identity-complete claims.
Added lines and PR body must not introduce transcript-complete claims.
Added lines and PR body must not introduce downgrade-proof claims.
Added lines and PR body must not introduce replay-proof claims.
Added lines and PR body must not introduce side-channel-free claims.
Added lines and PR body must not introduce vulnerability-free claims.
Added lines and PR body must not introduce bug-free claims.
Added lines and PR body must not introduce perfect-crypto claims.

## Closeout prerequisites

Optional closeout to NA-0477 may run only after the NA-0476 implementation PR
is merged and post-merge public-safety is green. Closeout must not implement
NA-0477 and must preserve exactly one READY item.

## Expected results

- `cargo fmt --check`: PASS.
- New negative test target: PASS.
- All required NA0476 markers appear in source and output.
- Inherited qsc tests: PASS.
- refimpl `pqkem768`: PASS.
- formal checks: PASS.
- root cargo audit: PASS.
- nested qsc fuzz lock audit: PASS.
- qsc adversarial syntax: PASS.
- qsc adversarial smoke: PASS locally or recorded as unavailable at local
  cargo-fuzz step with PR CI required.
- public-safety: PASS before merge and after merge.
