Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0479 QSL qsc/refimpl KEM / Signature Binding Mapping Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0479 consumes the qsc binding negative tests and the NA-0478 bounded formal
model, then maps qsc KEM/signature assumptions against the refimpl provider
boundary.

The mapping is accepted as internal supporting evidence only. The strongest
next residual is not another governance-only mapping pass and not immediate
vector/fuzz/public packaging. The selected successor is:

`NA-0480 -- QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan`

Reason: KEM behavior has direct refimpl integration evidence for roundtrip,
tamper, and wrong-length fail-closed behavior. Signature behavior maps cleanly
in code, but direct provider-boundary evidence is thinner for wrong signature
length, wrong public-key length, wrong public key, and invalid signature
classification. That gap should be scoped before negative vectors or external
review packaging.

NA-0479 does not implement tests, mutate runtime code, mutate crypto code,
mutate refimpl, mutate qsc source, mutate dependencies, mutate Cargo files,
mutate lockfiles, mutate workflows, mutate fuzz targets, mutate vectors, mutate
formal models, mutate service/public/qshield paths, or mutate backup/restore
state.

## Live NA-0479 scope

Live READY item at startup:

`NA-0479 -- QSL qsc/refimpl KEM / Signature Binding Mapping Authorization Plan`

Allowed mutation paths for this evidence lane:

- this evidence doc;
- `tests/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Read-only inspection covered qwork proof files, queue/governance files, qsc
handshake/identity code, qsc binding/provider-error tests, refimpl crypto
traits/provider code, refimpl provider tests, formal models, CI scripts,
Cargo manifests/locks, backup status/plan evidence, and qsl-backup script hash.

Forbidden mutation scope preserved:

- implementation, runtime, qsc source, qsc executable tests, crypto code,
  refimpl source/tests, dependencies, Cargo manifests, lockfiles, workflows,
  fuzz targets, vectors, formal models, services, public docs, qshield,
  qshield-cli, website, README, START_HERE, qwork/qstart/qresume/qshell,
  qsl-backup, backup status, backup plan, rollback paths, backup tree paths,
  durable Director State Index output, and public technical paper content.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0478 inheritance consumed;
- qsc assumptions inventoried;
- refimpl KEM and signature surfaces reviewed;
- qsc/refimpl error semantics mapped;
- formal/refimpl mapping reviewed;
- mapping gap matrix accepted;
- exactly one NA-0480 successor selected;
- no public overclaim introduced;
- exactly one READY item remains before optional closeout.

Stop conditions preserved:

- stale qwork proof against live HEAD/origin before fetch;
- missing PR #1227 merge proof;
- queue not READY NA-0479 at start;
- D-0946 already present at start;
- omitted qsc/refimpl/formal mapping sections;
- unsafe successor ambiguity;
- red root or nested audit;
- public-safety red or missing;
- any forbidden path mutation;
- backup/restore/qsl-backup mutation;
- any unsupported public, release, crypto, KEM, signature, equivalence,
  side-channel, external-review, backup, restore, vulnerability, or perfect
  crypto claim.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0479/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0479/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`;
- lane `NA-0479`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0479/qsl-protocol`;
- clean worktree, index, and untracked state;
- `READY_COUNT 1`;
- sole READY item: NA-0479;
- requested lane status: READY;
- proof HEAD and proof `origin/main`: `76db00307d10`.

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- PR #1227 was verified MERGED at `76db00307d10`;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

## NA-0478 inheritance

NA-0478 added:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`;
- runner integration in `formal/run_model_checks.py`;
- D-0944 implementation evidence;
- D-0945 closeout evidence restoring NA-0479.

Inherited modeled behavior:

- valid A1/B1/A2 trace completion;
- wrong KEM public-key rejection;
- stale KEM/public-record rejection;
- wrong KEM ciphertext rejection;
- wrong signature rejection;
- cross-message signature replay rejection;
- transcript mutation rejection;
- replay rejection;
- suite confusion rejection;
- stale public-record rejection;
- no completed-session mutation on selected rejects;
- no success output on selected rejects.

Inherited claim boundary:

- bounded internal formal evidence only;
- opaque tokens only;
- no cryptographic proof completeness;
- no qsc/refimpl equivalence completion;
- no side-channel conclusion;
- no vector or fuzz coverage completion;
- no public/release readiness claim.

## Applicable Stewardship and Assurance Review

1. Crypto / Protocol Steward: qsc KEM/signature use is fail-closed at the
   provider call boundary and through transcript/signature checks. The mapping
   is useful but does not prove qsc/refimpl equivalence. Recommended action:
   add future provider-boundary test-scope authorization.

2. CI / Dependency / Release Health Steward: current main public-safety is
   green, root `cargo audit --deny warnings` is green, nested qsc fuzz lock
   audit is green, and absent `pqcrypto-*` inverse tree probes are recorded as
   absence evidence only.

3. Public Claims / External Review Steward: evidence remains internal
   governance evidence only. No public readiness, external-review completion,
   KEM completion, signature completion, formal proof completion, or
   qsc/refimpl equivalence completion claim is made.

4. Product / Demo / Service Boundary Steward: no qsl-server, qsl-attachments,
   qshield runtime, qshield-cli, website, README, START_HERE, public docs, or
   service boundary mutation is authorized or performed.

5. Local Ops / Backup / Restore Steward: qwork proof files were consumed
   read-only; qsl-backup SHA remained `e9ecff3d22ed`; source inclusion count
   for the codex ops path remained exactly 1; no backup or restore was run.

6. Best-Known-Method Review: `BEST_KNOWN_METHOD_FOR_SCOPE`. The strongest
   bounded method for this lane is read-only code/test/formal/provider mapping
   plus explicit future scope selection, not implementation.

7. Hostile Cryptographer Review: provider wrappers expose simple Result/bool
   semantics, but absence of direct signature negative provider-boundary tests
   means code inspection should not be inflated into completion.

8. Red-Team Review: malformed/wrong KEM, tampered ciphertext, wrong signature,
   replay, suite confusion, and transcript mutation have qsc evidence. Refimpl
   signature wrong-length/wrong-public-key negative provider evidence should be
   scoped next.

9. Production SRE Review: no production readiness conclusion is drawn. Green CI
   and cargo audit are release-gate/dependency-health signals only.

10. Side-Channel Caveat: no timing, memory-access, cache, branch, power, fault,
    or secret-lifetime side-channel conclusion is made.

11. Formal-Model Mapping Residual:
    `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`. NA-0478 opaque tokens map to
    qsc-visible fields and refimpl outputs, but do not model provider internals.

12. External-Review Readiness:
    `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. Evidence improves internal
    readiness but is not external review.

13. Release-Claim Boundary: all wording remains internal. No public/release
    assurance is expanded.

14. Assurance Gap Review Trigger:
    `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`, because a concrete
    NA-0480 provider-boundary test-scope authorization successor is selected.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors, independent READY promotion, independent merge authority,
or Lead Director authority transfer is created.

## qsc KEM / signature assumption inventory

| Area | qsc code path | qsc evidence | Expected provider/refimpl behavior | Formal token mapping | Risk if semantics differ | Classification |
|---|---|---|---|---|---|---|
| KEM keypair generation | `identity_self_kem_keypair` and bootstrap/rotation wrappers call `hs_kem_keypair` / `runtime_pq_kem_keypair`; cfg seams return `rng_failure_forced` | lazy identity, rotation, TUI bootstrap, and KEM provider cfg/no-cfg tests | returns fixed-size public/secret key pair; concrete provider RNG failure is not forceable without qsc cfg seam | KEM public-key token and identity/public-record token | partial identity or pending state if failure is not fail-closed | direct qsc, supporting refimpl |
| KEM encapsulation | responder poll calls `StdCrypto.encap(&init.kem_pk)` and maps Err to `pq_encap_failed` | KEM provider RNG failure cfg test; binding negative tests | Result returns ciphertext/shared secret or error; bad public key errors | KEM ciphertext token derived from KEM public-key token | B1 output or responder mutation after encap error | direct qsc, supporting refimpl |
| KEM decapsulation | initiator poll calls `StdCrypto.decap(&pending.kem_sk, &resp.kem_ct)` and maps Err to `pq_decap_failed` | provider-error no-mutation test; wrong ciphertext binding negative test | Result returns shared secret or error; wrong length errors; tamper may produce different shared secret rather than error | wrong ciphertext token | session mutation or A2 output after decap reject | direct qsc, direct/supporting refimpl |
| KEM wrong length / tamper | qsc frame decoders enforce KEM field lengths before provider calls; decap errors reject | binding negative tests; `pqkem768` wrong-length/tamper tests | wrong lengths return `InvalidKey`; tamper changes shared secret or provider can reject | wrong ciphertext/KEM token | acceptance of malformed KEM material | direct qsc, direct KEM refimpl |
| Signature keypair generation | identity creation/migration/rotation wrappers call `hs_sig_keypair` / `runtime_pq_sig_keypair`; cfg seams return `rng_failure_forced` | lazy identity, legacy/public-record, rotation, TUI bootstrap, A2/B1 cfg tests | fixed-size public/secret key pair; concrete provider RNG failure is not forceable without qsc cfg seam | signature public-key token | partial public record or secret state | direct qsc, supporting refimpl |
| Signing | B1 and A2 paths call `StdCrypto.sign` and map Err to `sig_sign_failed` | B1/A2 signature provider cfg tests | Result returns signature or `InvalidKey` | signature message-context token | unsigned B1/A2 output or misleading success | direct qsc, supporting refimpl |
| Verify | `hs_sig_verify` maps Ok(false) and Err to `sig_invalid` | wrong identity and cross-message replay tests | verify returns Ok(true), Ok(false), or Err for malformed material | signature public-key and message-context tokens | invalid signature accepted or bad error split | direct qsc, supporting refimpl |
| Invalid signature rejection | B1/A2 verify failures emit `sig_invalid` and do not complete selected handshakes | binding negative tests | tampered signature returns Ok(false); malformed lengths return Err | wrong signature token | session creation on invalid signature | direct qsc, supporting refimpl |
| Public record signatures | public record stores KEM and signature public keys; optional signature pin check binds peer signature public key | wrong signature public-record negative test | provider key bytes must be stable and fixed-size | public-record/trusted-pin tokens | stale/wrong identity accepted | direct qsc, supporting refimpl |
| Fingerprints | qsc hashes KEM public key for identity fingerprint and signature public key for optional signature pin | identity/public-record and binding tests | stable byte serialization of provider keys | identity/signature public-key tokens | mismatched trust binding | direct qsc, supporting refimpl |
| Sanitized reasons | `pq_encap_failed`, `pq_decap_failed`, `sig_sign_failed`, `sig_invalid` | provider/error tests and binding negative tests | provider errors collapse to coarse qsc reasons | reject reason tokens | information leakage or drift in fail-closed behavior | direct qsc |
| No mutation / no output | provider failure tests assert no partial state or no output for selected paths | cfg/no-cfg qsc tests and provider-error no-mutation test | provider errors must be propagated and not bypassed | no completed-session mutation and no success output | partial session or misleading output | direct qsc |

## refimpl KEM surface review

KEM API:

- `PqKem768::encap(&self, pubk) -> Result<(Vec<u8>, Vec<u8>), CryptoError>`;
- `PqKem768::decap(&self, privk, ct) -> Result<Vec<u8>, CryptoError>`;
- `runtime_pq_kem_keypair() -> (Vec<u8>, Vec<u8>)`;
- length helpers for public key, secret key, and ciphertext.

Concrete `StdCrypto` behavior:

- keypair generation uses ML-KEM-768 provider RNG and returns public/secret key
  byte vectors;
- encapsulation parses public key bytes and returns ciphertext/shared secret or
  `CryptoError::InvalidKey`;
- decapsulation parses secret key and ciphertext bytes; wrong lengths map to
  `CryptoError::InvalidKey`; provider decapsulation failure maps to
  `CryptoError::AuthFail`;
- shared secrets are 32 bytes in current tests;
- tampered ciphertext is not guaranteed to error; current evidence shows the
  resulting shared secret differs.

Current direct KEM tests:

- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs` covers roundtrip,
  tamper-changes-secret, and wrong public-key/secret-key/ciphertext length
  fail-closed behavior;
- `StdCrypto` unit tests cover length helpers matching provider outputs.

Classification:

- `REFIMPL_KEM_MAPPING_ACCEPTED_SUPPORTING_ONLY`;
- residual: `REFIMPL_KEM_MAPPING_TEST_SCOPE_NEXT` for broader wrong-key/tamper
  boundary documentation if NA-0480 chooses it.

## refimpl signature surface review

Signature API:

- `PqSigMldsa65::sign(&self, privk, msg) -> Result<Vec<u8>, CryptoError>`;
- `PqSigMldsa65::verify(&self, pubk, msg, sig) -> Result<bool, CryptoError>`;
- `runtime_pq_sig_keypair() -> (Vec<u8>, Vec<u8>)`;
- length helpers for public key, secret key, and signature.

Concrete `StdCrypto` behavior:

- keypair generation fills an ML-DSA seed from provider RNG and returns encoded
  verifying key plus expanded signing key bytes;
- signing parses expanded signing key bytes and returns an encoded signature or
  `CryptoError::InvalidKey`;
- verify parses public key and signature bytes, returning `Ok(true)` for valid
  signatures, `Ok(false)` for cryptographically invalid signatures, or
  `CryptoError::InvalidKey` for malformed public key/signature material;
- qsc performs domain separation by constructing distinct B1/A2 signature
  message bytes before calling the primitive wrapper.

Current direct signature tests:

- `StdCrypto` unit coverage includes ML-DSA sign/verify roundtrip and tampered
  signature rejection;
- length helpers are covered by provider-boundary unit tests;
- there is no standalone integration test parallel to `pqkem768.rs` for wrong
  signature length, wrong public-key length, wrong public key, and `Err` versus
  `Ok(false)` classification.

Classification:

- `REFIMPL_SIGNATURE_MAPPING_TEST_SCOPE_NEXT`.

## qsc / refimpl error semantics mapping

| qsc reason | qsc path | refimpl behavior | Direct tests | Current gap | Future action |
|---|---|---|---|---|---|
| `pq_encap_failed` | responder `StdCrypto.encap` Err branch | `encap` returns `InvalidKey` for malformed public key; RNG failure not forceable in concrete helper | qsc cfg seam, refimpl wrong-length KEM | direct concrete provider failure beyond wrong length is not forced | NA-0480 provider-boundary test-scope review |
| `pq_decap_failed` | initiator `StdCrypto.decap` Err branch | wrong secret/ciphertext length returns `InvalidKey`; decap auth failure maps to `AuthFail` | qsc no-mutation test, refimpl wrong-length KEM | tamper may be `Ok(different secret)`, so qsc catches through transcript MAC | document/test KEM tamper-to-transcript reject relationship |
| `sig_sign_failed` | B1/A2 signing Err branch | malformed signing key returns `InvalidKey`; RNG failure not forceable in concrete helper | qsc cfg seam | no direct refimpl malformed signing-key integration test | NA-0480 scope candidate |
| `sig_invalid` | B1/A2 verify false or Err | invalid signature returns `Ok(false)`; malformed key/signature returns Err | qsc binding tests, refimpl unit tamper test | missing direct wrong-length/wrong-key signature provider tests | NA-0480 scope candidate |
| identity keypair failure | identity wrappers and cfg seams | runtime helpers appear infallible except provider RNG/panic class outside current seam | qsc cfg/no-cfg tests | concrete provider RNG failure not forceable | retain provider RNG boundary residual |

Classification:

- `ERROR_SEMANTICS_MAPPING_TEST_SCOPE_NEXT`.

## formal model / refimpl mapping review

Opaque model tokens that correspond to qsc-visible fields:

- KEM public key: A1 `kem_pk`, identity public record KEM public key, and KEM
  fingerprint material;
- KEM ciphertext: B1 `kem_ct`;
- signature public key: A1/B1 public-record and optional signature-pin material;
- signature message context: qsc B1/A2 signature message construction;
- transcript token: encoded A1/B1-no-auth material, transcript MAC/hash, A2
  confirm MAC;
- suite token: Suite-2 parameter block and suite-required rejection paths;
- public-record/trusted-pin tokens: identity public record, contact pin, and
  optional signature pin state;
- replay token: pending/session state and replay detection.

qsc-visible fields generated by refimpl provider values:

- ML-KEM public/secret key bytes;
- ML-KEM ciphertext and shared secret bytes;
- ML-DSA signature public/secret key bytes;
- ML-DSA signatures.

qsc-only assumptions:

- suite parameter-block parsing;
- transcript MAC/hash construction;
- B1/A2 signature message domain construction;
- identity pin and optional signature pin policy;
- pending/session no-mutation behavior;
- sanitized marker/error reason emission.

Assumptions depending on refimpl fail-closed behavior:

- malformed KEM public/secret/ciphertext material returns provider errors or
  produces transcript mismatch instead of accept;
- malformed signature public key/signature material returns Err or false;
- malformed signing key returns Err;
- provider key/signature/ciphertext lengths remain stable through helper APIs.

Classification:

- `FORMAL_REFIMPL_MAPPING_ACCEPTED_SUPPORTING_ONLY`, with NA-0480 selected to
  authorize direct provider-boundary test scope before stronger claims.

## mapping gap matrix

| ID | Domain | qsc assumption | refimpl behavior | Existing evidence | Direct/supporting/none | Gap | Risk | Severity | Likelihood | Public-claim risk | Recommended action | Candidate successor | Disposition |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| G-01 | KEM keypair | fixed-size keypair or forced failure seam | helper returns byte vectors | qsc cfg tests, refimpl length tests | supporting | concrete RNG failure not forceable | partial identity state if mishandled | P1 | low | medium | scope provider-boundary evidence | NA-0480 provider boundary | P1 next |
| G-02 | KEM encap | Result success/failure maps to `pq_encap_failed` | Result, bad key errors | qsc cfg test, KEM wrong length test | direct/supporting | malformed key breadth limited | B1 output on failure | P1 | low | medium | add scoped provider-boundary negative tests | NA-0480 provider boundary | P1 next |
| G-03 | KEM decap | Result success/failure maps to `pq_decap_failed` | Result, wrong lengths error | qsc no-mutation, KEM wrong length | direct | tamper can return different secret | transcript reject must carry safety | P1 | medium | medium | document/test tamper-to-transcript path | NA-0480 provider boundary | P1 next |
| G-04 | KEM wrong length/tamper | wrong material rejects or transcript mismatches | wrong length errors; tamper changes secret | `pqkem768` and qsc negative tests | direct | combine mapping in direct evidence | unsupported completion claim | P2 | low | medium | keep supporting-only | NA-0480 provider boundary | accepted caveat |
| G-05 | KEM public-key typing/length | qsc frame length plus provider parser | provider parser length checks | qsc decoder, KEM wrong length | direct/supporting | wrong but length-valid key behavior not isolated | peer mismatch or provider error ambiguity | P2 | low | medium | scope if cheap | NA-0480 provider boundary | P2 |
| G-06 | signature keypair | fixed-size keypair or forced failure seam | helper returns byte vectors | qsc cfg tests, refimpl helper length unit | supporting | concrete RNG failure not forceable | partial identity state | P1 | low | medium | scope provider-boundary review | NA-0480 provider boundary | P1 next |
| G-07 | signing | Result maps to `sig_sign_failed` | malformed signing key returns `InvalidKey` | qsc cfg tests, code review | supporting | no direct malformed signing-key integration test | B1/A2 output on failure | P1 | low | medium | authorize test-scope review | NA-0480 provider boundary | P1 next |
| G-08 | verification / invalid signature | false or Err maps to `sig_invalid` | tamper false; malformed material Err | qsc tests, refimpl unit tamper | supporting | no direct wrong-length/wrong-key integration test | invalid signature accepted if drift | P1 | medium | high | authorize provider-boundary test scope | NA-0480 provider boundary | P1 next |
| G-09 | signature wrong length/tamper | decoder/provider rejects or false | wrong length Err in code; tamper false in unit | qsc cross-message/tamper, refimpl unit | supporting | direct wrong-length test absent | error semantic drift | P1 | medium | high | scope explicit tests | NA-0480 provider boundary | P1 next |
| G-10 | public-record signatures | signature key pinned independently from KEM key | stable public key bytes | qsc wrong signature public-record test | direct qsc | refimpl public-key malformed test absent | stale/wrong identity accepted | P1 | low | high | scope wrong public key tests | NA-0480 provider boundary | P1 next |
| G-11 | qsc sanitized provider errors | coarse reasons on provider errors | errors are `InvalidKey`/`AuthFail` or false | qsc provider/error tests | direct qsc | provider error taxonomy not directly tested for all signature cases | leak/drift in reasons | P1 | medium | medium | provider-boundary tests | NA-0480 provider boundary | P1 next |
| G-12 | provider RNG boundary | qsc seams simulate failure | concrete helpers use provider RNG | qsc cfg/no-cfg tests | direct qsc | provider RNG failure not forceable | overclaiming provider RNG completeness | P1 | low | high | keep residual explicit | NA-0480 provider boundary | accepted caveat |
| G-13 | formal opaque-token assumptions | tokens map to qsc-visible fields | provider outputs feed those fields | NA-0478, code review | supporting | no provider internals modeled | overclaim formal completeness | P2 | medium | high | keep supporting-only | later formal/docs if needed | accepted caveat |
| G-14 | qsc/refimpl equivalence boundary | mapping is evidence only | no differential equivalence suite | this lane | none/ supporting | equivalence not proven | public overclaim | P1 | medium | high | no equivalence-complete claim | NA-0480 provider boundary | accepted caveat |

No P0 immediate blocker was found. Multiple P1 next gaps point to provider
boundary test-scope authorization before vectors/fuzz/external packaging.

## successor selection

Selected:

`NA-0480 -- QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan`

Rejected alternatives:

- Mapping evidence harness: rejected because this lane already provides the
  governance-only mapping and the remaining gap is direct provider-boundary
  scope.
- Refimpl negative test implementation harness: rejected because this directive
  must not implement NA-0480 and should authorize exact scope first.
- Binding negative vector suite: deferred until provider-boundary signature
  evidence scope is selected or rejected.
- Fuzz binding coverage: deferred behind provider-boundary and vector scope.
- Side-channel / secret-material assurance scope: important residual, but not
  P0 for this mapping lane.
- External review readiness package: premature before provider-boundary and
  vector/fuzz residuals are handled.
- Mapping residual triage: unnecessary because the successor is clear.

## future scope bundle

Future successor:

`NA-0480 -- QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan`

Allowed future governance paths:

- `docs/governance/evidence/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_plan.md`
- `tests/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future read-only paths:

- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `formal/`
- existing evidence docs and testplans.

Future forbidden scope unless a later exact implementation directive authorizes
it:

- implementation mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- executable test/fuzz/vector/formal mutation;
- qsc source mutation;
- refimpl mutation;
- service/public/qshield/qsl-server/qsl-attachments mutation;
- backup/restore/qsl-backup mutation;
- public overclaim or completion claims.

Future NA-0480 questions:

- Should refimpl KEM provider-boundary tests be expanded beyond existing
  `pqkem768` coverage?
- Should refimpl ML-DSA provider-boundary tests cover wrong signature length,
  wrong public-key length, wrong public key, malformed signing key, tamper,
  and Result false/Error classification?
- Should qsc sanitized error mapping be asserted by direct refimpl provider
  tests or remain qsc-only evidence?
- Which exact implementation paths, if any, should a later NA authorize?

## public claim / external review / website boundary

This evidence is internal governance evidence only.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
transcript-complete claim is made. No qsc/refimpl-equivalence-complete claim is
made. No provider-RNG-complete claim is made. No downgrade-proof claim is made.
No replay-proof claim is made. No formal-proof-complete claim is made. No
side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made.

No website, README, START_HERE, public docs, or public technical paper mutation
is performed.

## rejected alternatives

- Treating current refimpl signature evidence as complete: rejected because
  direct wrong-length/wrong-key signature provider-boundary tests are absent.
- Selecting direct implementation for NA-0480: rejected because this lane is an
  authorization lane and should first select exact test scope.
- Claiming qsc/refimpl equivalence: rejected because no equivalence harness or
  differential suite is present.
- Moving directly to external review readiness: rejected because provider
  boundary, vector, fuzz, and side-channel caveats remain.
- Treating cargo audit as cryptographic assurance: rejected; cargo audit green
  is dependency-health evidence only.

## backup-impact statement

No backup, restore, qsl-backup, backup status, backup plan, rollback,
systemd/timer/fstab, or backup tree path was mutated. qsl-backup was inspected
read-only by hash and source inclusion count only.

## next recommendation

Close NA-0479 only after this evidence PR is merged and post-merge
public-safety is green. Then restore exactly one READY successor:

`NA-0480 -- QSL refimpl KEM / Signature Provider Boundary Test Scope Authorization Plan`
