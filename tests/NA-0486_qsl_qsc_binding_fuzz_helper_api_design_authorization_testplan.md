Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15
Replaces:
Superseded-By:

# NA-0486 QSL qsc Binding Fuzz Helper API Design Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0486 helper/API design authorization lane. The lane must
consume NA-0485, inventory helper/API candidates, compare process-harness and
in-process fuzz options, decide exact Cargo/script/CI implications, classify
helper safety, decide corpus/seed strategy, select one NA-0487 successor, and
preserve no-implementation/no-public-overclaim boundaries.

## Protected invariants

- qwork proof files are read and copied, not regenerated.
- Exactly one READY item remains.
- NA-0485 inheritance is consumed.
- helper/API candidate inventory is recorded.
- process-harness versus in-process fuzz review is recorded.
- Cargo/script/CI exact scope is recorded.
- helper safety classification is recorded.
- corpus/seed strategy is recorded.
- no qsc source mutation occurs in NA-0486.
- no fuzz target, fuzz Cargo, fuzz corpus, script, workflow, dependency, or
  lockfile mutation occurs in NA-0486.
- no vector/input, formal model, executable test, refimpl, runtime, crypto,
  service, public-doc, website, qwork/qstart/qresume/qshell, backup, restore,
  qsl-backup, status, plan, rollback, or backup tree mutation occurs.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no identity-complete claim is introduced.
- no transcript-complete claim is introduced.
- no qsc/refimpl-equivalence-complete claim is introduced.
- no provider-boundary-complete claim is introduced.
- no provider-RNG-complete claim is introduced.
- no formal-proof-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc source or runtime mutation;
- qsc fuzz target, corpus, Cargo, or lockfile mutation;
- qsc-adversarial script or workflow mutation;
- qsc executable-test mutation;
- refimpl source or test mutation;
- dependency or lockfile mutation;
- vector/input mutation;
- formal model mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, service, website,
  public-doc, README, or START_HERE mutation;
- qwork, qstart, qresume, or qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation.

## Startup validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required:

- READY_COUNT 1;
- READY NA-0486;
- NA-0485 DONE;
- D-0958 exists once;
- D-0959 exists once;
- D-0960 absent at startup;
- duplicate decision count zero;
- public-safety success on current `origin/main`;
- root cargo audit green;
- nested qsc fuzz lock audit green.

## Helper/API candidate inventory validation

Run:

```bash
rg --files qsl/qsl-client/qsc/src/adversarial qsl/qsl-client/qsc/src/handshake qsl/qsl-client/qsc/src/identity qsl/qsl-client/qsc/tests qsl/qsl-client/qsc/fuzz/fuzz_targets
rg -n "pub mod adversarial|pub mod envelope" qsl/qsl-client/qsc/src/lib.rs
sed -n '1,220p' qsl/qsl-client/qsc/src/adversarial/mod.rs
sed -n '1,220p' qsl/qsl-client/qsc/fuzz/Cargo.toml
sed -n '1,220p' scripts/ci/qsc_adversarial.sh
sed -n '1,220p' .github/workflows/qsc-adversarial.yml
rg -n "fn hs_decode_init|fn hs_decode_resp|fn hs_decode_confirm|fn hs_transcript_hash|fn hs_transcript_mac|pub\\(crate\\) fn handshake_init|pub\\(crate\\) fn handshake_poll" qsl/qsl-client/qsc/src/handshake/mod.rs
rg -n "^fn |NA0476_|wrong|stale|replay|suite|mutation|transcript|signature|ciphertext" qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs
```

Required classifications:

- current public adversarial helpers are parser helpers;
- handshake semantic binding helpers are private or crate-internal;
- test helpers prove useful process/temp-root patterns but are not reusable
  from fuzz without source/test mutation;
- selected helper path is
  `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`;
- selected export path is `qsl/qsl-client/qsc/src/adversarial/mod.rs`;
- selected fuzz target path is
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`;
- selected future qsc fuzz Cargo path is `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- selected future script path is `scripts/ci/qsc_adversarial.sh`;
- workflow mutation is not selected.

## Binding and vector inheritance validation

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
```

Required:

- manifest valid JSON;
- formal binding model green;
- formal runner green;
- qsc binding negative tests green;
- refimpl signature provider-boundary test green;
- manifest remains traceability metadata only and no vector/input mutation
  occurs.

## Inherited qsc/refimpl tests

Run:

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
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required:

- all listed tests pass;
- provider-RNG/key-lifecycle/provider-error inherited evidence remains green;
- stable qsc `send_commit` green;
- refimpl `pqkem768` green.

## Scope guard

Changed paths must be exactly limited to:

- `docs/governance/evidence/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_plan.md`
- `tests/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Require zero changes under:

- `qsl/`;
- `qsl/qsl-client/qsc/src/`;
- `qsl/qsl-client/qsc/fuzz/`;
- `scripts/ci/`;
- `.github/`;
- `inputs/`;
- `formal/`;
- `tools/refimpl/`;
- `apps/`;
- service/public/backup/qwork tooling paths.

## Governance validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo fmt --check
```

Also run repository link-check, leak-scan, overclaim scan, PR body preflight,
goal-lint, classifier, and exact changed-path scope guard before PR.

Expected final classifications:

- `HELPER_DESIGN_SAFE_TEST_FUZZ_ONLY`;
- `HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_READY`;
- `FUZZ_SEED_NO_CORPUS_FIRST`;
- `FUZZ_SEED_EPHEMERAL_GENERATION_ONLY`;
- `FUZZ_SEED_MANIFEST_TRACEABILITY_ONLY`;
- `FUZZ_SEED_CORPUS_SEPARATE_LANE`;
- `QSC_BINDING_FUZZ_HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_IMPLEMENTATION_READY`.

## PR requirements

PR body must include:

- `Goals: G1, G2, G3, G4, G5`;
- Impact;
- No-regression;
- Tests/Vectors.

PR body must state:

- qsc binding fuzz helper/API design authorization;
- selected successor;
- no implementation mutation;
- no qsc/fuzz/vector/input mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/formal/refimpl
  mutation;
- no public overclaim;
- no external-review-complete claim;
- no fuzz/vector/KEM/signature/replay/downgrade/qsc-refimpl-equivalence/formal-proof-complete
  claim.

## Expected result

NA-0486 records the helper/API design authorization and selects the exact
NA-0487 implementation successor while preserving authorization-only scope. It
does not implement helpers, fuzz targets, Cargo metadata, script changes,
workflows, corpora, vectors, formal models, refimpl tests, runtime behavior, or
public claims.
