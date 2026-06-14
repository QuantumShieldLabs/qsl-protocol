Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0478 qsc KEM / Signature / Transcript Binding Formal Model Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qsc KEM/signature/transcript binding formal model selected
by NA-0477 and integrated by NA-0478.

## Protected invariants

- The model uses opaque tokens only.
- The model is bounded and deterministic.
- Valid A1/B1/A2 baseline reaches completed-session state.
- Wrong KEM public key rejects.
- Stale KEM/public-record tokens reject.
- Wrong KEM ciphertext rejects.
- Wrong signature identity rejects.
- Cross-message signature message-context replay rejects.
- Transcript mutation rejects.
- Replay rejects.
- Suite confusion rejects.
- Stale public-record tokens reject.
- Rejected selected traces preserve completed-session state.
- Rejected traces emit no success output.
- No qsc runtime/source file is mutated.
- No qsc executable test is mutated.
- No crypto implementation is mutated.
- No dependencies, Cargo manifests, lockfiles, or workflows are mutated.
- No fuzz target or vector is mutated.
- No refimpl path is mutated.
- No qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
  public docs, README, or START_HERE path is mutated.
- No qwork/qstart/qresume/qshell path is mutated.
- No backup, restore, qsl-backup, backup status, backup plan, rollback,
  systemd, timer, fstab, or backup tree path is mutated.
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

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime/source mutation;
- qsc executable-test mutation;
- crypto implementation mutation;
- dependency, Cargo manifest, lockfile, or workflow mutation;
- fuzz target or vector mutation;
- refimpl mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, or START_HERE mutation;
- qwork, qstart, qresume, or qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation;
- public technical paper work;
- durable Director State Index output.

## Standalone model command

Run:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
```

Required:

- command exits zero;
- all NA0478 markers are emitted;
- accepted-trace count is nonzero;
- rejected-trace count exceeds accepted-trace count;
- completed-session count is nonzero;
- no completed-session mutation assertion count is nonzero;
- no success-output-on-reject assertion count is nonzero.

## Runner command

Run:

```bash
python3 formal/run_model_checks.py
```

Required:

- existing SCKA model passes;
- existing Suite-2 negotiation model passes;
- existing qsc suite-id model passes;
- new qsc KEM/signature/transcript binding model passes.

## Modeled traces

Required traces:

- valid baseline trace;
- wrong KEM public-key reject;
- stale KEM/public-record reject;
- wrong KEM ciphertext reject;
- wrong signature identity reject;
- cross-message signature replay reject;
- transcript mutation reject;
- transcript replay/replay reject;
- suite confusion reject;
- stale public-record reject;
- selected no completed-session mutation after reject;
- no success output on reject;
- valid trace is not vacuously rejected.

## Marker plan

Required markers:

- `NA0478_FORMAL_MAPPING_SCOPE_CONSUMED_OK`
- `NA0478_BINDING_MODEL_VALID_TRACE_OK`
- `NA0478_BINDING_MODEL_WRONG_KEM_REJECT_OK`
- `NA0478_BINDING_MODEL_STALE_KEM_PUBLIC_RECORD_REJECT_OK`
- `NA0478_BINDING_MODEL_WRONG_CIPHERTEXT_REJECT_OK`
- `NA0478_BINDING_MODEL_WRONG_SIGNATURE_REJECT_OK`
- `NA0478_BINDING_MODEL_CROSS_MESSAGE_SIGNATURE_REPLAY_REJECT_OK`
- `NA0478_BINDING_MODEL_TRANSCRIPT_MUTATION_REJECT_OK`
- `NA0478_BINDING_MODEL_REPLAY_REJECT_OK`
- `NA0478_BINDING_MODEL_SUITE_CONFUSION_REJECT_OK`
- `NA0478_BINDING_MODEL_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0478_BINDING_MODEL_NO_SESSION_MUTATION_OK`
- `NA0478_BINDING_MODEL_NO_SUCCESS_OUTPUT_ON_REJECT_OK`
- `NA0478_NO_RUNTIME_CHANGE_OK`
- `NA0478_NO_DEPENDENCY_CHANGE_OK`
- `NA0478_NO_WORKFLOW_CHANGE_OK`
- `NA0478_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0478_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0478_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0478_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0478_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0478_NO_TRANSCRIPT_COMPLETE_CLAIM_OK`
- `NA0478_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0478_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0478_NO_FORMAL_PROOF_COMPLETE_CLAIM_OK`
- `NA0478_ONE_READY_INVARIANT_OK`

## Inherited qsc tests

Run:

```bash
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
```

Required: all commands pass before PR.

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

Required: root cargo audit passes. Optional inverse-tree probes may report
package absence under the directive-approved command shape.

## Nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: nested qsc fuzz lock audit passes. pqcrypto residual scan has zero
matches or is recorded under the directive-approved command shape.

## qsc adversarial check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## Public claim boundary

This testplan validates bounded internal formal-model evidence only.

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

## Closeout prerequisites

Optional closeout may run only after:

- implementation PR is merged;
- post-merge public-safety is completed success;
- D-0944 exists once on main;
- READY_COUNT remains 1 and READY NA-0478 before closeout;
- selected NA-0479 block preserves no-runtime, no-crypto, no-dependency,
  no-workflow, and no public-overclaim boundaries.

Do not implement NA-0479 in the closeout.
