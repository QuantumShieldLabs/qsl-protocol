Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0481 QSL refimpl Signature Provider Boundary Test Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0481 consumes NA-0480 and adds the selected refimpl ML-DSA signature provider-boundary integration tests in:

`tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

The test-only implementation covers wrong public-key length, wrong signature length, malformed signing-key length, tampered signature, wrong public key, and Err versus `Ok(false)` classification using the current public refimpl APIs only.

Observed API behavior:

- malformed length inputs return `Err(CryptoError::InvalidKey)`;
- well-shaped but cryptographically invalid verification inputs return `Ok(false)`;
- signing has an Err path for malformed signing-key bytes, but no false path because signing returns a signature or error.

This is bounded internal refimpl evidence only. It is not public readiness, production readiness, public-internet readiness, external-review completion, crypto completion, KEM completion, signature completion, qsc/refimpl equivalence completion, provider-boundary completion, provider-RNG completion, formal-proof completion, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Live NA-0481 scope

Live READY item at startup:

`NA-0481 -- QSL refimpl Signature Provider Boundary Test Implementation Harness`

Allowed implementation path:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

Allowed governance paths:

- this evidence document;
- `tests/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden mutation scope preserved:

- refimpl source mutation;
- qsc source or executable-test mutation;
- runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, service, qshield, qshield-cli, qsl-server, qsl-attachments, website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, status, plan, rollback, backup tree, durable Director State Index, or public technical paper mutation.

Acceptance criteria:

- selected signature provider-boundary tests pass;
- all selected NA0481 markers are emitted;
- Err versus `Ok(false)` semantics are documented truthfully;
- provider RNG, qsc sanitized error mapping, KEM provider-boundary, qsc/refimpl mapping, vectors, fuzz binding, side-channel, and external-review residuals remain caveated;
- no public claim expansion is introduced;
- exactly one READY remains before optional closeout.

Stop conditions preserved:

- stale qwork proof against live HEAD/origin before fetch;
- PR #1231 not merged;
- queue not READY NA-0481 at start;
- D-0950 present at start;
- selected tests require refimpl source or qsc mutation;
- required markers cannot be emitted;
- root or nested audit red;
- public-safety red or missing;
- any forbidden path mutation;
- backup/restore/qsl-backup mutation;
- any unsupported public, release, crypto, KEM, signature, qsc/refimpl equivalence, provider-boundary, provider RNG, side-channel, external-review, backup, restore, vulnerability, bug-free, or perfect-crypto claim must not be introduced.

## qwork proof-file verification

Codex read and copied the qwork proof files:

- `/srv/qbuild/work/NA-0481/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0481/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`;
- lane `NA-0481`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0481/qsl-protocol`;
- clean worktree, index, and untracked state before NA-0481 edits;
- `READY_COUNT 1`;
- sole READY item: NA-0481;
- requested lane status: READY;
- proof HEAD and proof `origin/main`: `d00e8d6d6f96`.

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- PR #1231 was verified MERGED at `d00e8d6d6f96`;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

## NA-0480 inheritance

NA-0480 provided:

- D-0948 refimpl KEM/signature provider-boundary test-scope authorization;
- D-0949 closeout restoring NA-0481;
- selected classification `REFIMPL_SIGNATURE_PROVIDER_BOUNDARY_TEST_SCOPE_READY`;
- exact implementation path `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`;
- selected markers for wrong public-key length, wrong signature length, malformed signing key, tampered signature, wrong public key, and Err versus `Ok(false)` classification.

Inherited residuals:

- KEM provider-boundary evidence remains supporting-only;
- qsc sanitized error mapping remains supporting-only;
- concrete provider RNG failure remains unforceable through current refimpl helpers without a seam or source/runtime mutation;
- qsc/refimpl equivalence is not established;
- binding negative vectors, fuzz binding, side-channel/secret-material review, and external-review readiness remain future residuals.

## Pre-mutation review

Preimage status:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` was absent before NA-0481 edits.

Read-only inspection covered:

- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`;
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`;
- `tools/refimpl/quantumshield_refimpl/src/crypto/mod.rs`;
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`;
- existing refimpl tests;
- NA-0480 evidence/testplans and D-0948/D-0949;
- NA-0479 qsc/refimpl mapping evidence;
- TRACEABILITY and rolling journal state.

Relevant API shape:

- `PqSigMldsa65::sign(&self, privk, msg) -> Result<Vec<u8>, CryptoError>`;
- `PqSigMldsa65::verify(&self, pubk, msg, sig) -> Result<bool, CryptoError>`;
- `StdCrypto::sign` maps malformed expanded signing-key bytes to `CryptoError::InvalidKey`;
- `StdCrypto::verify` maps malformed public-key or signature bytes to `CryptoError::InvalidKey`;
- `StdCrypto::verify` returns `Ok(false)` when provider verification fails for well-shaped inputs.

## Signature provider-boundary test implementation summary

Added:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

Tests added:

- `wrong_public_key_length_rejects_with_error`;
- `wrong_signature_length_rejects_with_error`;
- `malformed_signing_key_rejects_with_error`;
- `tampered_signature_returns_invalid_false`;
- `wrong_public_key_returns_invalid_false`;
- `malformed_inputs_are_errors_but_well_shaped_invalid_inputs_are_false`;
- `common_na0481_markers`.

The tests use:

- `StdCrypto`;
- `runtime_pq_sig_keypair`;
- `runtime_pq_sig_public_key_bytes`;
- `runtime_pq_sig_secret_key_bytes`;
- `runtime_pq_sig_signature_bytes`;
- the public `PqSigMldsa65` trait.

No refimpl source, qsc source/test, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, service, public-doc, website, backup, qsl-backup, or qwork tooling path was mutated.

## Wrong public-key length proof

Test:

- `wrong_public_key_length_rejects_with_error`

Mechanism:

- generate a valid ML-DSA signature keypair and signature;
- remove one byte from the public key;
- call `StdCrypto::verify`.

Observed result:

- `Err(CryptoError::InvalidKey)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_LENGTH_REJECT_OK`

## Wrong signature length proof

Test:

- `wrong_signature_length_rejects_with_error`

Mechanism:

- generate a valid ML-DSA signature;
- remove one byte from the signature;
- call `StdCrypto::verify`.

Observed result:

- `Err(CryptoError::InvalidKey)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_WRONG_SIGNATURE_LENGTH_REJECT_OK`

## Malformed signing-key proof

Test:

- `malformed_signing_key_rejects_with_error`

Mechanism:

- generate a valid expanded signing key;
- remove one byte from the signing key bytes;
- call `StdCrypto::sign`.

Observed result:

- `Err(CryptoError::InvalidKey)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_MALFORMED_SIGNING_KEY_REJECT_OK`

Caveat:

- NA-0481 proves malformed length rejection through the current public API. It does not claim that every length-valid corrupted expanded signing-key byte string is rejected, because that would require provider-internal validity assumptions or source/runtime mutation outside this lane.

## Tampered signature invalid proof

Test:

- `tampered_signature_returns_invalid_false`

Mechanism:

- generate a valid ML-DSA signature;
- flip one bit in the signature while preserving length and shape;
- call `StdCrypto::verify`.

Observed result:

- `Ok(false)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_TAMPERED_SIGNATURE_INVALID_OK`

## Wrong public key invalid proof

Test:

- `wrong_public_key_returns_invalid_false`

Mechanism:

- sign with one generated keypair;
- verify the well-shaped signature with a different generated public key;
- call `StdCrypto::verify`.

Observed result:

- `Ok(false)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_INVALID_OK`

## Err versus Ok(false) classification proof

Test:

- `malformed_inputs_are_errors_but_well_shaped_invalid_inputs_are_false`

Observed malformed-input results:

- wrong public-key length: `Err(CryptoError::InvalidKey)`;
- wrong signature length: `Err(CryptoError::InvalidKey)`;
- malformed signing-key length: `Err(CryptoError::InvalidKey)`.

Observed well-shaped invalid verification results:

- tampered signature: `Ok(false)`;
- wrong public key: `Ok(false)`.

Marker:

- `NA0481_REFIMPL_SIGNATURE_ERR_VS_FALSE_CLASSIFICATION_OK`

API shape caveat:

- Verify exposes both Err and `Ok(false)` directly.
- Sign exposes Err but has no `Ok(false)` classification because signing returns either bytes or an error.
- Concrete provider RNG failure cannot be forced through the current refimpl public APIs without adding a seam or mutating source/runtime behavior.

## Provider RNG caveat

NA-0481 does not force provider RNG failure and does not add a provider-RNG seam. Existing qsc provider-RNG tests remain inherited supporting evidence only. No provider-RNG-complete claim is made.

## qsc sanitized error mapping caveat

qsc sanitized error mapping remains supporting-only. NA-0481 does not mutate qsc source or executable tests. Current qsc mappings collapse provider signature Err and `Ok(false)` into qsc-level `sig_invalid`, while signing Err maps to `sig_sign_failed`; NA-0481 pins the refimpl side of that distinction only.

## KEM supporting-only caveat

KEM provider-boundary evidence remains supporting-only for this lane. Existing refimpl `pqkem768` covers roundtrip, wrong public-key length, wrong secret-key length, wrong ciphertext length, and tamper-changes-secret behavior, but NA-0481 adds no KEM tests and makes no KEM-complete claim.

## Bounded evidence and no completion claim proof

Markers emitted by the new test:

- `NA0481_PROVIDER_BOUNDARY_SCOPE_CONSUMED_OK`
- `NA0481_NO_RUNTIME_CHANGE_OK`
- `NA0481_NO_DEPENDENCY_CHANGE_OK`
- `NA0481_NO_WORKFLOW_CHANGE_OK`
- `NA0481_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0481_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0481_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0481_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0481_NO_PROVIDER_BOUNDARY_COMPLETE_CLAIM_OK`
- `NA0481_NO_QSC_REFIMPL_EQUIVALENCE_COMPLETE_CLAIM_OK`
- `NA0481_NO_PUBLIC_CLAIM_EXPANSION_OK`
- `NA0481_ONE_READY_INVARIANT_OK`

This evidence does not assert public readiness, production readiness, public-internet readiness, external-review completion, crypto completion, KEM completion, signature completion, qsc/refimpl equivalence completion, provider-boundary completion, provider-RNG completion, formal-proof completion, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## No source / qsc / dependency / workflow mutation proof

Expected changed paths for NA-0481 implementation PR:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`;
- `docs/governance/evidence/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_harness.md`;
- `tests/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

No refimpl source path was modified.

No qsc path was modified.

No dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public-doc, README, START_HERE, backup, qsl-backup, status, plan, rollback, backup tree, durable Director State Index, or public technical paper path was modified.

## Applicable stewardship and assurance review

1. Best-Known-Method Review: the best method for this lane is a public-API integration test file that pins the refimpl signature boundary without source mutation.
2. Hostile Cryptographer Review: the important distinction is malformed bytes returning Err versus well-shaped invalid signature material returning `Ok(false)`. NA-0481 tests both classes directly.
3. Red-Team Review: wrong length, malformed signing key, tampered signature, and wrong public key are deterministic negative inputs that exercise the boundary without relying on provider internals.
4. Production SRE Review: no runtime or dependency change was needed; validation remains local plus CI-gated.
5. Side-Channel Caveat: NA-0481 makes no timing, cache, branch, power, fault, memory-access, or secret-lifetime side-channel claim.
6. Formal-Model Mapping Residual: formal qsc binding models remain green/supporting only and do not model provider internals.
7. External-Review Readiness: this is incremental internal evidence only, not external-review completion.
8. Release-Claim Boundary: no release, public-readiness, or completion claim expands.
9. Assurance Gap Review Trigger: after NA-0481, the next highest residual remains binding negative vector suite authorization unless post-merge evidence proves otherwise.

Level 1 stewardship remains active. Level 2 and Level 3 stewardship rollout remain future-gated. The Lead Director remains final authority.

## Validation

Startup validation passed:

- qwork proof-file parse and freshness verification;
- PR #1231 merged verification;
- queue and decision proof;
- current-main public-safety proof;
- root cargo audit;
- nested qsc fuzz lock audit;
- dependency tree probes;
- qsc adversarial marker proof;
- qsl-backup SHA and source-list read-only proof;
- inherited formal, qsc, stable qsc, and refimpl `pqkem768` tests.

Initial new-test validation:

- first run of the new integration test failed at compile time because the test used `assert_eq!` on `Result<bool, CryptoError>` while `CryptoError` does not implement `PartialEq`;
- classification: recoverable in-scope local test-code validation failure with clear cause;
- corrective action: changed only the new test file to unwrap well-shaped invalid verification results and assert `false`;
- final result: new integration test passed with 7 tests and all NA0481 markers emitted.

Required post-patch validation is recorded in the proof root and PR evidence.

## Scope guard

The implementation is expected to pass an exact scope guard limited to the six allowed NA-0481 paths:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`
- `docs/governance/evidence/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_harness.md`
- `tests/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Backup-impact statement

Backup impact classification: none.

NA-0481 changed only qsl-protocol files under `/srv/qbuild/work/NA-0481/qsl-protocol` and did not mutate qsl-backup, backup status files, backup plan files, rollback paths, `/backup/qsl`, systemd/timer/fstab, backup scripts, or local ops source lists. Codex did not run backup or restore.

## Successor selection

Selected successor after successful NA-0481:

`NA-0482 -- QSL Binding Negative Vector Suite Authorization Plan`

Rationale:

- NA-0481 closes the immediate refimpl signature provider-boundary test gap selected by NA-0480.
- KEM provider-boundary remains supporting-only and did not reveal an interaction gap that outranks vectors.
- qsc sanitized error mapping and qsc/refimpl equivalence remain residual but do not outrank a stable negative vector suite authorization based on current NA-0481 results.
- Fuzz binding, side-channel/secret-material, supply-chain/provenance, and external-review readiness remain important future residuals but should follow vector-scope authorization unless later evidence changes priority.

## Next recommendation

Merge NA-0481 only after local validation and required PR checks pass. If post-merge public-safety is green, close NA-0481 and restore NA-0482 as READY without implementing NA-0482.
