Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0474 QSL KEM / Signature / Transcript Binding Read-Only Audit Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0474 governance-only, read-only cryptographic assurance audit.
The lane must consume NA-0473, inventory qsc/refimpl KEM, signature,
transcript, identity, public-record, suite, replay, downgrade, and state
transition binding surfaces, complete Level-1 stewardship and D328 assurance
reviews, select exactly one NA-0475 successor, and preserve no implementation
mutation.

## Protected invariants

- NA-0474 remains governance-only before optional closeout.
- Exactly one READY item remains before optional closeout.
- NA-0473 is DONE and D-0934/D-0935 are consumed.
- D-0936 records the NA-0474 read-only audit.
- No implementation mutation occurs.
- No runtime mutation occurs.
- No crypto mutation occurs.
- No dependency mutation occurs.
- No Cargo manifest mutation occurs.
- No lockfile mutation occurs.
- No workflow mutation occurs.
- No executable test mutation occurs.
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

- `docs/governance/evidence/NA-0474_qsl_kem_signature_transcript_binding_read_only_audit_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- Runtime source mutation.
- Crypto source mutation.
- Dependency, Cargo manifest, or lockfile mutation.
- Workflow mutation.
- Executable test mutation.
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
  `/backup/qsl`, timers, fstab, or systemd mutation.
- Public technical paper work.
- Durable Director State Index output.

## Required review evidence

The evidence doc must include:

- executive summary;
- live NA-0474 scope;
- qwork proof-file verification;
- NA-0473 inheritance;
- applicable stewardship and assurance review;
- KEM binding review;
- signature binding review;
- transcript / KDF / confirmation binding review;
- identity / public record / stale record review;
- downgrade / replay / suite-confusion review;
- qsc / refimpl mapping review;
- formal / vector / fuzz coverage review;
- threat / attack scenario review;
- findings matrix and prioritization;
- exactly one selected NA-0475 successor;
- future scope bundle;
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
cargo tree -i ml-kem --locked
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
- The sole READY item is NA-0474 before optional closeout.
- NA-0473 is DONE.
- D-0936 exists exactly once after the patch.
- D-0937 is absent before optional closeout.
- Duplicate decision count is zero.
- Changed paths are limited to the five allowed NA-0474 governance paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Inherited identity/provider RNG tests are green.
- Provider-error no-mutation test is green.
- qsc adversarial script syntax is green.
- refimpl `pqkem768` is green.
- Formal checks are green.
- No public overclaim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No external-review-complete claim is introduced.
- No backup or restore is run.
- No qsl-backup mutation occurs.
- No status or plan mutation occurs.

## Markers

- `NA0474_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0474_NA0473_CONSUMED_OK`
- `NA0474_KEM_BINDING_REVIEW_OK`
- `NA0474_SIGNATURE_BINDING_REVIEW_OK`
- `NA0474_TRANSCRIPT_KDF_CONFIRM_REVIEW_OK`
- `NA0474_IDENTITY_PUBLIC_RECORD_STALE_REVIEW_OK`
- `NA0474_DOWNGRADE_REPLAY_SUITE_REVIEW_OK`
- `NA0474_QSC_REFIMPL_MAPPING_REVIEW_OK`
- `NA0474_FORMAL_VECTOR_FUZZ_REVIEW_OK`
- `NA0474_THREAT_SCENARIO_REVIEW_OK`
- `NA0474_FINDINGS_MATRIX_ACCEPTED_OK`
- `NA0474_SUCCESSOR_SELECTED_OK`
- `NA0474_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0474_NO_PUBLIC_OVERCLAIM_OK`
- `NA0474_BACKUP_BOUNDARY_OK`
