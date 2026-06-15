Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0480 QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0480 consumes NA-0479 and authorizes the next provider-boundary test scope.

Primary classification:

`REFIMPL_SIGNATURE_PROVIDER_BOUNDARY_TEST_SCOPE_READY`

Selected successor:

`NA-0481 -- QSL refimpl Signature Provider Boundary Test Implementation Harness`

The selected future lane should implement direct refimpl ML-DSA provider-boundary tests only, in:

`tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

The future signature test scope should cover wrong public-key length, wrong signature length, malformed signing key, tampered signature, wrong public key, and explicit Err versus `Ok(false)` classification. It should not mutate refimpl source, qsc source, qsc executable tests, runtime code, crypto code, dependencies, Cargo files, lockfiles, workflows, vectors, fuzz targets, formal models, service paths, public docs, backup files, or qsl-backup.

KEM provider-boundary evidence remains supporting-only for this successor because current `pqkem768` coverage already directly tests KEM roundtrip, wrong public-key length, wrong secret-key length, wrong ciphertext length, and tamper-changes-secret behavior. Additional KEM tests can remain backlog unless a later directive needs a combined provider-boundary file.

qsc sanitized error mapping remains supporting-only for this successor. Future NA-0481 should not mutate qsc source or qsc executable tests. Provider RNG failure remains a residual because concrete provider RNG failure is not forceable through current refimpl helpers without adding a test seam or changing provider/runtime code.

This evidence is internal governance evidence only. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No KEM-complete claim is made. No signature-complete claim is made. No qsc/refimpl-equivalence-complete claim is made. No provider-boundary-complete claim is made. No provider-RNG-complete claim is made. No formal-proof-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## Live NA-0480 scope

Live READY item at startup:

`NA-0480 -- QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan`

Allowed mutation paths for NA-0480 evidence PR:

- this evidence document;
- `tests/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Read-only inspection covered qwork proof files, NA-0479 evidence, NA-0479 response/testplans, D-0946, D-0947, refimpl crypto traits/provider/tests, qsc handshake/identity code, qsc tests, formal models, dependency health, public-safety, qsc adversarial marker evidence, backup status/plan evidence, and qsl-backup hash/source-list evidence.

Forbidden mutation scope preserved:

- implementation mutation;
- runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector, formal model, qsc source, refimpl source/test, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback, backup tree, durable Director State Index, or public technical paper mutation.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0479 inheritance consumed;
- KEM provider-boundary scope reviewed;
- signature provider-boundary scope reviewed;
- combined versus split decision recorded;
- provider RNG caveat preserved;
- qsc sanitized error mapping implications recorded;
- exact NA-0481 successor selected;
- no implementation mutation performed;
- no public overclaim introduced;
- exactly one READY remains before optional closeout.

Stop conditions preserved:

- stale qwork proof against live HEAD/origin before fetch;
- PR #1229 not merged;
- queue not READY NA-0480 at start;
- D-0948 present at start;
- omitted candidate inventory, KEM review, signature review, or combined/split decision;
- unsafe successor ambiguity;
- root or nested audit red;
- public-safety red or missing;
- any forbidden path mutation;
- backup/restore/qsl-backup mutation;
- no unsupported public, release, crypto, KEM, signature, equivalence, provider-boundary, provider-RNG, side-channel, external-review, backup, restore, vulnerability, bug-free, or perfect-crypto claim.

## qwork proof-file verification

Codex read and copied the qwork proof files:

- `/srv/qbuild/work/NA-0480/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0480/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`;
- lane `NA-0480`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0480/qsl-protocol`;
- clean worktree, index, and untracked state before NA-0480 edits;
- `READY_COUNT 1`;
- sole READY item: NA-0480;
- requested lane status: READY;
- proof HEAD and proof `origin/main`: `c0ae3e08e799`.

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- PR #1229 was verified MERGED at `c0ae3e08e799`;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

## NA-0479 inheritance

NA-0479 provided:

- D-0946 qsc/refimpl KEM and signature binding mapping authorization;
- D-0947 closeout restoring NA-0480;
- qsc KEM/signature assumption inventory;
- refimpl KEM surface review;
- refimpl signature surface review;
- qsc/refimpl error semantics mapping;
- formal/refimpl mapping review;
- mapping gap matrix.

Inherited classifications:

- `REFIMPL_KEM_MAPPING_ACCEPTED_SUPPORTING_ONLY`, with test-scope residuals;
- `REFIMPL_SIGNATURE_MAPPING_TEST_SCOPE_NEXT`;
- `ERROR_SEMANTICS_MAPPING_TEST_SCOPE_NEXT`;
- `FORMAL_REFIMPL_MAPPING_ACCEPTED_SUPPORTING_ONLY`.

Inherited residuals:

- direct signature provider-boundary evidence is thinner than KEM evidence;
- qsc sanitized error mapping is direct qsc evidence but supporting-only for refimpl;
- concrete provider RNG failure is not forceable through current refimpl helpers;
- qsc/refimpl equivalence is not established;
- vectors, fuzz binding, side-channel review, and external-review readiness remain future residuals.

## Applicable Stewardship and Assurance Review

1. Crypto / Protocol Steward: direct refimpl signature boundary tests should be authorized first because `PqSigMldsa65` exposes distinct Err and `Ok(false)` paths that qsc collapses to `sig_invalid`. KEM has stronger direct refimpl evidence already. No protocol, wire, crypto, or state-machine semantics change is authorized.

2. CI / Dependency / Release Health Steward: root cargo audit, nested qsc fuzz lock audit, current-main public-safety, inherited qsc tests, formal checks, and refimpl `pqkem768` were green at startup. Cargo audit green remains dependency-health evidence only.

3. Public Claims / External Review Steward: evidence remains internal governance evidence only. External-review readiness is incremental, not complete.

4. Product / Demo / Service Boundary Steward: no qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, README, START_HERE, public docs, demo surface, or service boundary mutation is authorized or performed.

5. Local Ops / Backup / Restore Steward: qwork proof files were consumed read-only. qsl-backup SHA matched the expected installed-script hash. The codex ops source appears once in the installed qsl-backup daily source list and once in reviewed/latest daily manifests. No backup or restore was run.

6. Best-Known-Method Review: `BEST_KNOWN_METHOD_FOR_SCOPE`. The best method for this authorization lane is read-only source/test/formal/evidence review plus exact successor selection, not implementation.

7. Hostile Cryptographer Review: the main adversarial concern is semantic drift between malformed signature inputs that return Err and invalid signatures that return `Ok(false)`. Direct refimpl tests should pin that boundary before external-review packaging.

8. Red-Team Review: wrong signature length, wrong public-key length, malformed signing key, tampered signature, and wrong public key are the smallest high-value direct refimpl negative set. KEM wrong-length/tamper already has direct evidence.

9. Production SRE Review: no production readiness conclusion is drawn. The future implementation can be tests-only and should avoid runtime or dependency changes.

10. Side-Channel Caveat: no timing, cache, branch, power, fault, memory-access, or secret-lifetime side-channel conclusion is made.

11. Formal-Model Mapping Residual: `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`. NA-0478 opaque tokens map to qsc-visible fields and refimpl byte values, but provider internals and side-channel behavior are out of model scope.

12. External-Review Readiness: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. Signature boundary tests would improve readiness, but this lane is not external review.

13. Release-Claim Boundary: no release or public security claim expands. No provider-boundary-complete claim is made.

14. Assurance Gap Review Trigger: `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`, because the exact signature provider-boundary successor is selected.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No separate Directors, independent READY promotion, independent merge authority, or Lead Director authority transfer is created.

## Refimpl provider-boundary candidate inventory

| Candidate | Current code path | Current test coverage | Classification | Future test-only? | Source/runtime mutation needed? | Future test file if selected | Expected result | qsc mapping relevance | Priority |
|---|---|---|---|---|---|---|---|---|---|
| KEM wrong public-key length | `StdCrypto::encap` parses ML-KEM public key bytes and maps parse failure to `CryptoError::InvalidKey` | `pqkem768_wrong_length_inputs_fail_closed` | direct | yes | no | not selected; existing `pqkem768.rs` already covers | Err `InvalidKey` | maps to `pq_encap_failed` supporting evidence | P2 |
| KEM wrong secret-key length | `StdCrypto::decap` parses ML-KEM secret key bytes and maps parse failure to `CryptoError::InvalidKey` | `pqkem768_wrong_length_inputs_fail_closed` | direct | yes | no | not selected; existing `pqkem768.rs` already covers | Err `InvalidKey` | maps to `pq_decap_failed` supporting evidence | P2 |
| KEM wrong ciphertext length | `StdCrypto::decap` parses ML-KEM ciphertext bytes and maps parse failure to `CryptoError::InvalidKey` | `pqkem768_wrong_length_inputs_fail_closed` | direct | yes | no | not selected; existing `pqkem768.rs` already covers | Err `InvalidKey` | maps to `pq_decap_failed` supporting evidence | P2 |
| KEM tampered ciphertext | `StdCrypto::decap` decapsulates length-valid ciphertext; tamper may return a different shared secret | `pqkem768_tamper_changes_secret`; qsc transcript negative tests | direct/supporting | yes | no | not selected; existing `pqkem768.rs` already covers provider effect | `Ok` with different secret or fail-closed reject; no accept claim | maps to transcript MAC reject, not direct qsc provider error | P2 |
| KEM wrong public key / wrong secret key pair | `encap`/`decap` operate on length-valid but unrelated KEM material | supporting from tamper and qsc wrong-public-key tests | supporting | yes | no | backlog only if later combined KEM scope is selected | derived secret mismatch; no completion claim | maps to qsc wrong KEM/public-record reject | P2 |
| KEM encapsulation error shape | `encap` returns Result and maps malformed public key to `InvalidKey`; provider RNG failure not forced | wrong-length test plus qsc cfg seam | supporting/direct | yes for malformed key only | no for malformed key; provider RNG forcing would need seam/source change | backlog only | Err `InvalidKey` for malformed public key | maps to `pq_encap_failed` supporting-only | P2 |
| KEM decapsulation error shape | `decap` returns Result and maps malformed secret/ciphertext to `InvalidKey`, provider auth fail to `AuthFail` | wrong-length test plus qsc no-mutation test | supporting/direct | yes for malformed inputs | no for malformed inputs | backlog only | Err `InvalidKey` or `AuthFail`; tamper may be `Ok(different secret)` | maps to `pq_decap_failed` and transcript reject caveat | P2 |
| signature wrong public-key length | `StdCrypto::verify` parses `EncodedVerifyingKey` and maps parse failure to `CryptoError::InvalidKey` | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | Err `InvalidKey` | maps to qsc `sig_invalid` Err branch | P1 |
| signature wrong signature length | `StdCrypto::verify` parses ML-DSA signature and maps parse failure to `CryptoError::InvalidKey` | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | Err `InvalidKey` | maps to qsc `sig_invalid` Err branch | P1 |
| signature wrong signing-key length | `StdCrypto::sign` parses expanded signing key and maps parse failure to `CryptoError::InvalidKey` | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | Err `InvalidKey` | maps to qsc `sig_sign_failed` | P1 |
| signature malformed signing key | `StdCrypto::sign` constructs an ML-DSA signing key from expanded bytes after parse | no standalone refimpl integration test | none/direct code review | yes if malformed input can be expressed by length or corrupted expanded key | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | Err `InvalidKey` for malformed parse; otherwise document if provider accepts length-valid bytes | maps to qsc `sig_sign_failed` caveat | P1 |
| signature tampered signature | `StdCrypto::verify` returns `Ok(false)` when provider verification fails | unit test in `stdcrypto.rs`, no standalone integration test | supporting | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | `Ok(false)` | maps to qsc `sig_invalid` false branch | P1 |
| signature wrong public key | `StdCrypto::verify` returns `Ok(false)` for valid signature checked against a different valid public key | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | `Ok(false)` | maps to qsc wrong identity/signature public-record rejects | P1 |
| signature verify invalid `Ok(false)` versus Err classification | `hs_sig_verify` collapses both provider false and Err to qsc `sig_invalid`; refimpl preserves distinction | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | length/malformed inputs Err; tamper/wrong key `Ok(false)` | pins qsc sanitized mapping boundary | P1 |
| signature sign Err classification | `StdCrypto::sign` returns Err for malformed signing key | no standalone refimpl integration test | none/direct code review | yes | no | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` | Err `InvalidKey` | maps to qsc `sig_sign_failed` | P1 |
| provider RNG failure boundary caveat | runtime keypair/sign/encap helpers use provider RNG; qsc has cfg seams | direct qsc cfg/no-cfg tests; no concrete refimpl RNG failure forcing | supporting/residual | no without seam/source change | yes if concrete RNG forcing is required | not selected | remain residual | prevents provider-RNG-complete overclaim | P1 caveat |
| qsc sanitized error mapping implications | qsc maps provider KEM/signature errors to coarse reasons | direct qsc tests; refimpl mapping supporting-only | supporting | not in signature-only refimpl lane | qsc test mutation would be out of selected future scope | not selected for NA-0481 | remain supporting-only | prevents qsc/refimpl-equivalence overclaim | P1 caveat |

## KEM provider-boundary test scope review

Classification:

`REFIMPL_KEM_PROVIDER_BOUNDARY_SUPPORTING_ONLY`

Answers:

- Current `pqkem768` tests are enough for the first-order wrong public-key length, wrong secret-key length, wrong ciphertext length, and tamper-changes-secret provider-boundary questions.
- Missing KEM cases exist but do not outrank signature: wrong but length-valid KEM keypair behavior and broader error-shape taxonomy could be added later if a combined provider-boundary lane is selected.
- Selected KEM cases can be tested in refimpl tests only, but they are not selected for NA-0481 because the signature gap is higher value.
- Provider RNG failure is not forceable through current refimpl helpers without a new seam or source change; it remains residual.
- KEM should not be bundled with signature for the next implementation lane because bundling would dilute the main P1 signature gap and expand the future PR blast radius.
- qsc does not need a change to benefit from the selected signature tests. qsc mapping remains supporting-only.

Acceptance criteria for KEM if a later lane selects it:

- wrong KEM public-key length rejects with `InvalidKey`;
- wrong KEM secret-key length rejects with `InvalidKey`;
- wrong KEM ciphertext length rejects with `InvalidKey`;
- tampered ciphertext is documented as fail-closed by mismatch/transcript behavior, not as guaranteed provider Err;
- no provider-RNG-complete claim is made.

## Signature provider-boundary test scope review

Classification:

`REFIMPL_SIGNATURE_PROVIDER_BOUNDARY_SCOPE_READY`

Answers:

- Direct ML-DSA/refimpl signature provider-boundary tests are not present as a standalone integration test file.
- Wrong signature length can be tested in refimpl only.
- Wrong public-key length can be tested in refimpl only.
- Wrong signing-key length and malformed signing-key behavior can be tested in refimpl only, with the caveat that length-valid corrupted expanded-key bytes may be provider-accepted and should be documented rather than forced into an incorrect expected result.
- Wrong public key verify behavior can be tested in refimpl only using two generated keypairs.
- Tampered signature behavior can be tested in refimpl only.
- Err versus `Ok(false)` semantics can be asserted directly: malformed length/material should return Err, while cryptographically invalid but well-formed signature cases should return `Ok(false)`.
- Provider RNG failure is not forceable through current refimpl helpers and remains residual.
- Signature boundary tests should be the first implementation successor.

Selected future file:

`tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

Selected future markers:

- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_SIGNATURE_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_MALFORMED_SIGNING_KEY_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_TAMPERED_SIGNATURE_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_ERR_VS_FALSE_CLASSIFICATION_OK`
- `NA0481_NO_RUNTIME_CHANGE_OK`
- `NA0481_NO_DEPENDENCY_CHANGE_OK`
- `NA0481_NO_PUBLIC_CLAIM_EXPANSION_OK`

## Combined vs split test-scope decision

| Option | Decision | Evidence | Future paths if selected | Validation markers | Public-claim caveat |
|---|---|---|---|---|---|
| 1. Combined refimpl KEM / Signature Provider Boundary Test Implementation | rejected | KEM is already directly covered for main wrong-length/tamper cases; signature is the P1 gap | not selected; possible later `tools/refimpl/quantumshield_refimpl/tests/provider_boundary.rs` | would need KEM and signature markers | no provider-boundary-complete claim |
| 2. Signature-only provider boundary implementation | selected | exact test path is clear, source mutation is not needed, and it closes the highest-value P1 direct-evidence gap | `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs` plus NA-0481 governance paths | selected NA0481 signature markers | no signature-complete claim |
| 3. KEM-only provider boundary implementation | rejected | KEM has stronger current direct evidence than signature | not selected | KEM markers would be backlog | no KEM-complete claim |
| 4. Split-scope authorization again | rejected | exact signature path and marker plan are clear enough | not selected | n/a | no public overclaim |
| 5. Negative vector suite first | rejected | provider-boundary direct signature evidence should precede broader vector packaging | not selected | future vector markers only if separately authorized | no vector-coverage-complete claim |
| 6. Fuzz binding first | rejected | fuzz is valuable but lower priority than deterministic provider-boundary semantics | not selected | future fuzz markers only if separately authorized | no fuzz-complete claim |
| 7. Side-channel / secret-material next | rejected | remains important residual but not immediate P0 for this lane | not selected | future side-channel markers only if separately authorized | no side-channel-free claim |
| 8. External review readiness package | rejected | premature before direct signature provider-boundary tests | not selected | future package markers only if separately authorized | no external-review-complete claim |

## Authorization decision

Primary classification:

`REFIMPL_SIGNATURE_PROVIDER_BOUNDARY_TEST_SCOPE_READY`

Decision:

- NA-0479 is consumed.
- KEM provider-boundary candidates are inventoried and classified as supporting-only/backlog for NA-0481.
- Signature provider-boundary candidates are inventoried and classified as ready for exact future implementation.
- KEM and signature tests should be split for the next implementation lane.
- Signature provider-boundary implementation should be first.
- qsc sanitized error mapping should remain supporting-only for NA-0481; no qsc source or executable-test mutation should be authorized by the selected successor.
- Concrete provider RNG failure remains unforceable through current refimpl helpers and remains residual.
- Future implementation can be confined to tests only.
- Negative vectors, fuzz binding, side-channel scope, and external-review readiness do not outrank direct refimpl signature provider-boundary tests.
- Public-claim caveats remain active.
- Exactly one READY successor remains mandatory after closeout.

Selected NA-0481:

`NA-0481 -- QSL refimpl Signature Provider Boundary Test Implementation Harness`

## Future scope bundle

Candidate future allowed paths for NA-0481:

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`
- `docs/governance/evidence/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_harness.md`
- `tests/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless exact scope authorizes otherwise:

- refimpl source mutation;
- qsc source or qsc executable-test mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- formal/fuzz/vector mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup tree mutation;
- no public readiness, crypto completion, KEM completion, signature completion, qsc/refimpl equivalence completion, provider-boundary completion, provider-RNG completion, side-channel freedom, vulnerability freedom, bug freedom, or perfect-crypto claims.

Suggested future command:

```bash
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary
```

## Future validation / marker plan

Common future no-overclaim markers:

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
- `NA0481_ONE_READY_INVARIANT_OK`

Selected signature markers:

- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_SIGNATURE_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_MALFORMED_SIGNING_KEY_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_TAMPERED_SIGNATURE_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_ERR_VS_FALSE_CLASSIFICATION_OK`
- `NA0481_NO_PUBLIC_CLAIM_EXPANSION_OK`

Acceptance criteria for NA-0481:

- signature wrong public-key length returns Err `InvalidKey`;
- signature wrong signature length returns Err `InvalidKey`;
- malformed or wrong-length signing key returns Err `InvalidKey` or the test documents any provider-accepted length-valid malformed-key caveat without weakening fail-closed claims;
- tampered signature returns `Ok(false)`;
- wrong public key returns `Ok(false)`;
- Err versus `Ok(false)` classification is explicit;
- no runtime, crypto, dependency, Cargo, lockfile, workflow, qsc, vector, fuzz, formal, service, public, backup, restore, or qsl-backup mutation is introduced;
- no public overclaim is introduced;
- exactly one READY remains.

## Public claim / external review / website boundary

This evidence is internal governance evidence only.

No website, README, START_HERE, public docs, public technical paper, qsl-server, qsl-attachments, qshield runtime, or qshield-cli mutation is performed.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No KEM-complete claim is made. No signature-complete claim is made. No qsc/refimpl-equivalence-complete claim is made. No provider-boundary-complete claim is made. No provider-RNG-complete claim is made. No formal-proof-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

Cargo audit green is dependency-health evidence only.

## Rejected alternatives

- Combined KEM/signature implementation: rejected because it broadens the immediate successor while KEM already has direct core evidence.
- KEM-only implementation: rejected because it does not address the highest-value P1 signature residual.
- Another split-scope authorization lane: rejected because the exact signature future path and markers are clear.
- qsc sanitized error mapping in the same future lane: rejected because the selected future lane should stay refimpl tests-only and avoid qsc executable-test mutation.
- Negative vector suite first: rejected because direct provider-boundary semantics should be pinned first.
- Fuzz binding first: rejected because deterministic signature provider-boundary coverage is lower risk and higher signal now.
- Side-channel / secret-material next: rejected because it remains residual but is not immediate P0.
- External-review readiness package: rejected because provider-boundary/vector/fuzz residuals remain.

## Backup-impact statement

No backup, restore, qsl-backup, backup status, backup plan, rollback, systemd/timer/fstab, or backup tree path was mutated.

Read-only proof:

- qsl-backup installed script SHA matched `e9ecff3d22ed`;
- codex ops source appeared exactly once in the installed daily source list;
- codex ops source appeared exactly once in the reviewed daily manifest;
- codex ops source appeared exactly once in the latest daily manifest.

Same-host continuity remains same-host evidence only. No off-host backup, restore/key-custody, disaster-recovery, backup-complete, or restore-proven claim is made.

## Next recommendation

After this evidence PR merges and post-merge public-safety is green, close NA-0480 and restore exactly one READY successor:

`NA-0481 -- QSL refimpl Signature Provider Boundary Test Implementation Harness`

NA-0481 should implement only the selected refimpl signature provider-boundary test file plus governance evidence/testplan/decision/traceability/journal updates.
