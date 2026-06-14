Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0475 qsc KEM / Signature / Transcript Binding Negative Test Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0475 governance-only authorization lane. The lane must consume
NA-0474, inventory exact candidate qsc negative-test surfaces, decide whether a
combined future implementation lane is safe, select exactly one successor, and
preserve no implementation mutation in NA-0475.

## Protected invariants

- NA-0475 remains authorization-only before optional closeout.
- Exactly one READY item remains before optional closeout.
- NA-0474 is DONE and D-0936/D-0937 are consumed.
- D-0938 records the NA-0475 authorization decision.
- No executable qsc tests are implemented by NA-0475.
- No runtime mutation occurs.
- No crypto mutation occurs.
- No dependency mutation occurs.
- No Cargo manifest mutation occurs.
- No lockfile mutation occurs.
- No workflow mutation occurs.
- No fuzz target mutation occurs.
- No vector mutation occurs.
- No formal model mutation occurs.
- No refimpl mutation occurs.
- No qsl-server mutation occurs.
- No qsl-attachments mutation occurs.
- No qshield runtime mutation occurs.
- No qshield-cli mutation occurs.
- No website mutation occurs.
- No public docs mutation occurs.
- No README mutation occurs.
- No START_HERE mutation occurs.
- No qwork/qstart/qresume/qshell mutation occurs.
- No backup is run.
- No restore is run.
- No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path is mutated.
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
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0475_qsl_qsc_kem_signature_transcript_binding_negative_test_scope_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- Runtime source mutation.
- Crypto source mutation.
- Dependency, Cargo manifest, or lockfile mutation.
- Workflow mutation.
- Executable qsc test mutation.
- Fuzz target mutation.
- Vector mutation.
- Formal model mutation.
- refimpl mutation.
- qsl-server mutation.
- qsl-attachments mutation.
- qshield runtime mutation.
- qshield-cli mutation.
- Website, public docs, README, or START_HERE mutation.
- qwork, qstart, qresume, or qshell mutation.
- Backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  backup tree, timers, fstab, or systemd mutation.
- Public technical paper work.
- Durable Director State Index output.

## Required review evidence

The evidence doc must include:

- executive summary;
- live NA-0475 scope;
- qwork proof-file verification;
- NA-0474 inheritance;
- applicable stewardship and assurance review;
- candidate negative-test surface inventory;
- KEM negative-test scope review;
- signature negative-test scope review;
- transcript / replay / suite negative-test scope review;
- stale public-record / identity rollback scope review;
- combined vs split scope decision;
- authorization decision;
- future scope bundle;
- future validation / marker plan;
- public claim / external review / website boundary;
- rejected alternatives;
- backup-impact statement;
- next recommendation.

## Validation commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
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

Also run the directive scope guard, link check, leak scan, overclaim scan,
classifier, PR body preflight, goal-lint, and qsc adversarial smoke if feasible.

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## Expected results

- READY_COUNT is 1.
- The sole READY item is NA-0475 before optional closeout.
- NA-0474 is DONE.
- D-0938 exists exactly once after the patch.
- D-0939 is absent before optional closeout.
- Duplicate decision count is zero.
- Changed paths are limited to the five allowed NA-0475 governance paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Inherited qsc tests are green.
- Provider-error no-mutation test is green.
- qsc adversarial script syntax is green.
- refimpl `pqkem768` is green.
- Formal checks are green.
- No public overclaim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No external-review-complete claim is introduced.
- No backup or restore is run.
- No qsl-backup mutation occurs.
- No status or plan mutation occurs.

## Markers

- `NA0475_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0475_NA0474_CONSUMED_OK`
- `NA0475_CANDIDATE_SURFACE_INVENTORY_OK`
- `NA0475_KEM_NEGATIVE_TEST_SCOPE_READY_OK`
- `NA0475_SIGNATURE_NEGATIVE_TEST_SCOPE_READY_OK`
- `NA0475_TRANSCRIPT_REPLAY_NEGATIVE_TEST_SCOPE_READY_OK`
- `NA0475_STALE_PUBLIC_RECORD_NEGATIVE_TEST_SCOPE_READY_OK`
- `NA0475_COMBINED_SCOPE_SELECTED_OK`
- `NA0475_BINDING_NEGATIVE_TEST_COMBINED_SCOPE_READY_OK`
- `NA0475_NA0476_SUCCESSOR_SELECTED_OK`
- `NA0475_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0475_NO_PUBLIC_OVERCLAIM_OK`
- `NA0475_BACKUP_BOUNDARY_OK`
