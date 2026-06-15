Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0483 QSL Binding Negative Vector Suite Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0483 implementation lane. The lane must create the selected
internal negative binding vector README and JSON manifest, cover the required
KEM/signature/transcript/replay/suite/stale/refimpl/formal cases, preserve
secret-material and public-claim boundaries, and avoid all forbidden runtime,
source, dependency, workflow, public, and backup mutation.

## Protected invariants

- qwork proof files are read and copied, not regenerated.
- Exactly one READY item remains.
- NA-0482 authorization is consumed.
- Internal vector files live only under
  `inputs/suite2/internal_negative_binding_vectors/`.
- Public/conformance vectors under `inputs/suite2/vectors/` are not modified.
- Checked-in vectors contain no private keys, KEM secret keys, signing keys,
  passphrases, runtime keys, backup keys, operator data, user data, live
  service data, or private production endpoint data.
- Any secret material required by future validation is generated ephemerally by
  tests and not checked in.
- No runtime, crypto, qsc source/test, refimpl source/test, dependency, Cargo,
  lockfile, workflow, fuzz target, formal model, qsl-server, qsl-attachments,
  qshield runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
  rollback, durable Director State Index, public technical paper, or backup
  tree mutation occurs.
- No public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, crypto-complete, vector-complete, KEM-complete,
  signature-complete, identity-complete, transcript-complete,
  qsc/refimpl-equivalence-complete, provider-boundary-complete,
  provider-RNG-complete, formal-proof-complete, replay-proof, downgrade-proof,
  side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is
  introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- `docs/governance/evidence/NA-0483_qsl_binding_negative_vector_suite_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- `inputs/suite2/vectors/`;
- runtime/source or crypto mutation;
- qsc source or executable-test mutation;
- refimpl source or executable-test mutation;
- dependency, Cargo manifest, lockfile, or workflow mutation;
- fuzz target or formal model mutation;
- service, public, qshield, qsl-server, qsl-attachments, qshield-cli,
  website, public-doc, README, START_HERE mutation;
- qwork, qstart, qresume, qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation.

## README validation

Verify that `inputs/suite2/internal_negative_binding_vectors/README.md`
exists and states:

- internal negative evidence only;
- not public/conformance/interoperability vectors;
- no checked-in private or secret material;
- future tests must generate any needed secret material ephemerally;
- manifest sections `qsc_binding`,
  `refimpl_signature_provider_boundary`, and `formal_token_mapping`;
- JSON validation command;
- no public/security completion claim.

Required README markers:

- `NA0483_VECTOR_SCOPE_CONSUMED_OK`
- `NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK`
- `NA0483_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0483_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0483_NO_VECTOR_COMPLETE_CLAIM_OK`

## Manifest JSON validation

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
```

Required:

- valid JSON;
- top-level object;
- `schema_version` is `"1"`;
- `vectors` is a non-empty array;
- at least 34 vector entries;
- all vector IDs are unique.

## Secret-material validation

Run a deterministic uncommitted Python validator that checks:

- every vector has `material_policy`;
- every vector sets `contains_secret_material` false;
- every vector sets `contains_private_key` false;
- every vector sets `contains_passphrase` false;
- every vector sets `contains_user_data` false;
- no disallowed field names appear outside false `material_policy` booleans;
- no value contains a PEM/private-key header-like marker;
- no value contains a live endpoint secret-like marker;
- no value contains an obvious passphrase assignment marker;
- `inputs/suite2/vectors/` is not modified.

Expected result: PASS.

## Required vector IDs

Required KEM entries:

- `kem_wrong_peer_public_key`
- `kem_stale_public_record`
- `kem_wrong_ciphertext`
- `kem_wrong_key_ciphertext_pair`

Required signature entries:

- `signature_wrong_identity_public_record`
- `signature_cross_message_replay_b1_as_a2`
- `signature_wrong_message_context`
- `signature_tampered_signature`
- `signature_wrong_public_key`

Required transcript / replay / suite entries:

- `transcript_mutation`
- `transcript_truncation`
- `replayed_a1`
- `replayed_b1`
- `replayed_a2`
- `wrong_role_replay`
- `suite_confusion_wrong_suite_token`
- `downgrade_wrong_suite_block`

Required stale identity / rollback entries:

- `stale_public_record_replay`
- `public_record_rollback`
- `identity_rotation_stale_peer_state`
- `stale_trusted_pin_mismatch`

Required refimpl signature provider-boundary entries:

- `refimpl_signature_wrong_public_key_length`
- `refimpl_signature_wrong_signature_length`
- `refimpl_signature_malformed_signing_key`
- `refimpl_signature_tampered_signature_invalid`
- `refimpl_signature_wrong_public_key_invalid`
- `refimpl_signature_err_vs_false_classification`

Required formal-token entries:

- `formal_wrong_kem_token`
- `formal_wrong_signature_token`
- `formal_transcript_mutation`
- `formal_replay`
- `formal_suite_confusion`
- `formal_stale_public_record`
- `formal_no_session_mutation_on_reject`

## Marker plan

Required manifest markers include:

- `NA0483_VECTOR_SCOPE_CONSUMED_OK`
- `NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK`
- `NA0483_VECTOR_KEM_WRONG_PUBLIC_KEY_CASE_OK`
- `NA0483_VECTOR_KEM_WRONG_CIPHERTEXT_CASE_OK`
- `NA0483_VECTOR_SIGNATURE_WRONG_IDENTITY_CASE_OK`
- `NA0483_VECTOR_SIGNATURE_CROSS_MESSAGE_REPLAY_CASE_OK`
- `NA0483_VECTOR_TRANSCRIPT_MUTATION_CASE_OK`
- `NA0483_VECTOR_REPLAY_CASE_OK`
- `NA0483_VECTOR_SUITE_CONFUSION_CASE_OK`
- `NA0483_VECTOR_STALE_PUBLIC_RECORD_CASE_OK`
- `NA0483_VECTOR_REFIMPL_SIGNATURE_BOUNDARY_CASE_OK`
- `NA0483_NO_RUNTIME_CHANGE_OK`
- `NA0483_NO_DEPENDENCY_CHANGE_OK`
- `NA0483_NO_WORKFLOW_CHANGE_OK`
- `NA0483_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0483_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0483_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0483_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0483_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0483_NO_TRANSCRIPT_COMPLETE_CLAIM_OK`
- `NA0483_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0483_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0483_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0483_ONE_READY_INVARIANT_OK`

## Inherited qsc/refimpl/formal tests

Run:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
PYTHONDONTWRITEBYTECODE=1 python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
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

Expected result: PASS.

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

Expected result: root audit PASS. Optional inverse dependency probes may report
zero matches under the directive-approved `|| true` shape.

## Nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Expected result: nested audit PASS. Optional pqcrypto scan may produce zero
matches.

## qsc adversarial check

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## Public claim boundary

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Run an added-line overclaim scan against the diff. Expected result: no
affirmative public-readiness, production-readiness, external-review-complete,
crypto-complete, vector-complete, KEM-complete, signature-complete,
qsc/refimpl-equivalence-complete, provider-boundary-complete,
formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or
perfect-crypto claim.

## Closeout prerequisites

NA-0483 may close out only after:

- implementation PR is merged;
- post-merge public-safety is green;
- READY remains NA-0483 before closeout;
- D-0954 exists once on main;
- D-0955 is absent before closeout;
- exactly one READY item remains;
- no public/conformance vector, runtime, source, dependency, workflow, test,
  fuzz, formal, public, service, backup, or qwork mutation occurred outside
  the exact NA-0483 scope.

Selected closeout successor if green:

`NA-0484 -- QSL Fuzz Binding Coverage Scope Authorization Plan`
