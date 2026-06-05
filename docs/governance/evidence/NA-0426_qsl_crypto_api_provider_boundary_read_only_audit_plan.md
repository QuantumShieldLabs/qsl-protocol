Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0426 QSL Crypto API / Provider Boundary Read-Only Audit Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0426 completed a bounded read-only audit of the QSL crypto API and provider
boundary after the NA-0418/D257 pqcrypto remediation. The audit focused on the
`PqKem768` abstraction, `StdCrypto`, the current RustCrypto `ml-kem` provider
implementation, qsc runtime use of the provider boundary, fail-closed reject
behavior, validation coverage, formal/vector alignment, dependency health, and
public-claim boundaries.

No runtime code, crypto code, dependency manifest, lockfile, workflow, service,
backup, qwork/qstart/qresume/qshell, public docs, README, START_HERE, website,
qsl-server, qsl-attachments, qshield runtime, backup status, backup plan, or
qsl-backup path was mutated by this audit.

No BLOCKER or HIGH runtime issue was found. The audit found follow-up items
that should be triaged under a governance authorization lane before any
implementation or dependency changes occur. The selected successor is:

`NA-0427 -- QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan`

## Live NA-0426 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Allowed mutation paths for the NA-0426 evidence PR:

- `docs/governance/evidence/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_plan.md`
- `tests/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection covered qwork proof files, the stewardship canon,
`NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, the rolling journal,
prior evidence docs, `tools/refimpl/quantumshield_refimpl/`,
`qsl/qsl-client/qsc/`, `qsp/`, `qsc/`, `formal/`, `inputs/`, `Cargo.toml`, and
`Cargo.lock`. The top-level `qsp/` and `qsc/` roots are absent in this checkout;
qsc content is under `qsl/qsl-client/qsc/`.

Forbidden mutation scope:

- no runtime or crypto mutation;
- no dependency, Cargo, or workflow mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup execution;
- no restore execution;
- no qsl-backup mutation;
- no backup status or backup plan mutation;
- no rollback subtree or `/backup/qsl` mutation;
- no public technical paper content;
- no secret material handling;
- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no metadata-free claim;
- no anonymity claim;
- no untraceability claim;
- no off-host-backup-complete claim;
- no disaster-recovery-complete claim;
- no restore-proven claim;
- no backup-complete claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

Acceptance criteria:

- crypto/provider boundary audit remains read-only;
- fail-closed behavior is assessed as evidence and not overclaimed;
- public-claim caveats are explicit;
- no runtime/crypto/dependency/workflow mutation occurs;
- cargo audit remains green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1120 not merged;
- `origin/main` not equal to or descended from PR #1120 merge commit;
- queue not READY NA-0426 at start;
- D-0839 absent or D-0840 already present at start;
- cargo audit not green;
- qsl-backup source-list regression;
- backup or restore execution by Codex;
- forbidden runtime/crypto/dependency/workflow/public/service/local-ops
  mutation;
- public-safety red or missing;
- more than one READY item;
- unsupported public assurance claim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0426/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0426/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported the required values:

- `startup_result=OK`
- `lane=NA-0426`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0426/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0426`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `36b342c4e71e`. PR #1120 was verified MERGED with merge
commit `36b342c4e71e`.

Proof root:

`/srv/qbuild/tmp/NA0426_crypto_api_provider_boundary_audit_20260605T150323-0500`

The qwork proof files were copied into the proof root under `qwork/`.

## NA-0425 inheritance

NA-0425/D-0838 resumed the deferred code/crypto audit follow-up stream after
the backup/log-code chain completed through NA-0422 and the stewardship canon
landed through NA-0424. NA-0425 selected the crypto API / provider boundary as
the first audit domain because NA-0418/D257 recently replaced the runtime
reachable pqcrypto ML-KEM provider with RustCrypto `ml-kem` while preserving
the `PqKem768` trait/helper boundary.

NA-0425 closeout D-0839 restored NA-0426 as the sole READY successor. Inherited
constraints remained active: advisory-only stewardship, exactly one READY item,
no runtime/crypto/dependency/workflow/public/service/backup mutation, no
qwork/qstart/qresume execution, no public overclaim, and no public technical
paper work.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Does the current provider boundary preserve fail-closed
behavior and provider ownership after the `ml-kem` replacement?

Evidence reviewed: `PqKem768` trait in
`tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `StdCrypto`
implementation in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`;
provider tests in `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`;
qsc handshake use in `qsl/qsl-client/qsc/src/handshake/mod.rs`; NA-0418
remediation evidence.

Findings: `PqKem768` remains the provider trait. `StdCrypto` implements it
through `ml-kem` types and converts malformed public key, secret key, and
ciphertext byte inputs into `CryptoError` values. qsc uses runtime helper
functions and the trait rather than naming the third-party KEM provider
directly. Follow-up is needed for stronger qsc-boundary negative tests,
property/fuzz/differential coverage, and secret-memory hygiene classification.

Risk classification: MEDIUM and EVIDENCE_INCOMPLETE.

Public-claim impact: no crypto-complete claim, no side-channel-free claim, no
external-review-complete claim, no vulnerability-free claim, no bug-free claim,
and no perfect-crypto claim is supported.

Scope impact: future remediation or test expansion requires exact future scope.

Recommended action: select findings triage and remediation authorization as
NA-0427.

### CI / Dependency / Release Health Steward

Review question: Does dependency health remain green after the provider
replacement, and are dependency-family boundaries clear?

Evidence reviewed: `cargo audit --deny warnings`; `cargo tree -i
rustls-webpki --locked`; inverse trees for `ml-kem` and pqcrypto packages;
root `cargo metadata --locked --format-version=1`; root and nested Cargo
files/locks.

Findings: root cargo audit is green. `rustls-webpki v0.103.13` remains active.
Root `Cargo.lock` and root metadata show `ml-kem v0.2.1` through
`quantumshield_refimpl`. Root inverse trees show `pqcrypto-mlkem`,
`pqcrypto-traits`, and `pqcrypto-internals` absent. A nested, separate
`qsl/qsl-client/qsc/fuzz/Cargo.lock` still contains pqcrypto package entries;
that fuzz workspace is outside the root workspace metadata, but it is a
dependency-health evidence gap if fuzz lanes are revived.

Risk classification: MEDIUM and EVIDENCE_INCOMPLETE.

Public-claim impact: cargo audit green is dependency-health evidence only; it
is no vulnerability-free proof, no bug-free proof, and no perfect-crypto proof.

Scope impact: no Cargo mutation is allowed in NA-0426; future triage should
decide whether the nested fuzz lock requires remediation.

Recommended action: carry the nested fuzz lock finding into NA-0427.

### Public Claims / External Review Steward

Review question: Does the provider-boundary audit introduce unsupported public
claims?

Evidence reviewed: NA-0425 public-claim caveats, stewardship canon,
TRACEABILITY rows, NA-0418 evidence, and current audit findings.

Findings: this audit is internal governance evidence only. It is not external
review. It does not establish public service readiness or deployment safety.
It does not establish crypto completion. It does not establish side-channel
status. It does not establish vulnerability absence. It does not establish
metadata privacy. No anonymity claim is made. No untraceability claim is made.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no public-readiness claim, no production-readiness claim,
no public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no side-channel-free claim, no metadata-free claim, no
anonymity claim, no untraceability claim, no vulnerability-free claim, no
bug-free claim, and no perfect-crypto claim.

Scope impact: no README, START_HERE, public docs, website, or public technical
paper work is in scope.

Recommended action: keep all findings internal and future-gated.

### Product / Demo / Service Boundary Steward

Review question: Does provider-boundary evidence blur refimpl, demo, qsc, or
service boundaries?

Evidence reviewed: `quantumshield_refimpl` provider code/tests, qsc dependency
selection, NA-0425 demo/service boundary inheritance, stewardship canon, and
TRACEABILITY service-boundary rows.

Findings: provider evidence is reference/runtime-boundary evidence for
qsl-protocol and qsc. It does not mutate qshield runtime, qsl-server, or
qsl-attachments, and it does not prove service-local or public-internet
deployment properties. qsl-server and qsl-attachments were boundary references
only.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no demo-as-production claim, no production-readiness
claim, no public-internet-readiness claim, and no external-review-complete
claim.

Scope impact: no sibling repo mutation and no service-runtime mutation.

Recommended action: preserve service/demo caveats in NA-0427.

### Local Ops / Backup / Restore Steward

Review question: Were qwork proof files, qsl-backup, backup state, and local
ops boundaries preserved?

Evidence reviewed: qwork proof files, live git state, qsl-backup SHA, source
inclusion count, prior response file, and rolling journal.

Findings: qwork proof files were read and copied to the proof root; Codex did
not run qwork, qstart, or qresume. qsl-backup SHA matched the required value,
and the Codex ops source inclusion count was exactly one. Codex did not run
backup or restore and did not mutate backup status, backup plan, qsl-backup,
rollback subtree paths, or `/backup/qsl`.

Risk classification: INFO.

Public-claim impact: no backup-complete claim, no off-host-backup-complete
claim, no disaster-recovery-complete claim, and no restore-proven claim.

Scope impact: no local mutable path changed except the directive-required
temporary proof root and repository governance paths.

Recommended action: continue no-backup/no-restore discipline.

## Provider boundary file inventory

Files present:

- `tools/refimpl/quantumshield_refimpl/src/crypto/mod.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `qsl/qsl-client/qsc/Cargo.toml`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- root `Cargo.toml`
- root `Cargo.lock`
- `formal/README.md`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/model_scka_bounded.py`
- `formal/run_model_checks.py`
- `inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json`
- `inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json`
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- NA-0418 remediation evidence and testplan.

Files or roots absent:

- top-level `qsp/`
- top-level `qsc/`

Provider implementation files:

- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`

Provider tests:

- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- unit tests inside `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`

Relevant dependency declarations:

- `tools/refimpl/quantumshield_refimpl/Cargo.toml` declares feature `pqkem =
  ["dep:ml-kem"]` and historical feature `pqcrypto = ["pqkem", "dep:ml-dsa"]`.
- `tools/refimpl/quantumshield_refimpl/Cargo.toml` declares `ml-kem =
  { version = "0.2.1", features = ["zeroize"], optional = true }`.
- `qsl/qsl-client/qsc/Cargo.toml` depends on `quantumshield_refimpl` with
  `features = ["pqcrypto"]`.
- Root `Cargo.toml` includes `tools/refimpl/quantumshield_refimpl` and
  `qsl/qsl-client/qsc` as workspace members.

Relevant qsc usage:

- `qsl/qsl-client/qsc/src/main.rs` imports runtime PQ helper functions,
  `StdCrypto`, and `PqKem768`.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` wraps KEM lengths and keypair
  generation through provider helper functions and uses `StdCrypto.encap` /
  `StdCrypto.decap` for the handshake path.
- `qsl/qsl-client/qsc/src/identity/mod.rs` stores the generated KEM secret key
  through qsc vault helpers.

Formal/vector roots that may support future audit:

- `formal/`
- `inputs/suite2/`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`

## PqKem768 / ml-kem provider boundary review

Where `PqKem768` is defined:

- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` defines
  `PqKem768` with `encap(&self, pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>),
  CryptoError>` and `decap(&self, privk: &[u8], ct: &[u8]) ->
  Result<Vec<u8>, CryptoError>`.

Where `StdCrypto` is implemented:

- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` defines
  `StdCrypto` and implements `PqKem768` behind `#[cfg(feature = "pqkem")]`.

Which `ml-kem` types/functions are used:

- `ml_kem::MlKem768`
- `ml_kem::MlKem768Params`
- `ml_kem::kem::DecapsulationKey`
- `ml_kem::kem::EncapsulationKey`
- `ml_kem::Ciphertext`
- `ml_kem::Encoded`
- `ml_kem::EncodedSizeUser`
- `ml_kem::KemCore`
- `Encapsulate::encapsulate`
- `Decapsulate::decapsulate`

Representations:

- public key: `Vec<u8>` at the trait/helper boundary, converted into
  `ml_kem::Encoded<EncapsulationKey<_>>`;
- secret key: `Vec<u8>` at the helper boundary, converted into
  `ml_kem::Encoded<DecapsulationKey<_>>`;
- ciphertext: `Vec<u8>` from encapsulation and `&[u8]` at decapsulation,
  converted into `ml_kem::Ciphertext<ml_kem::MlKem768>`;
- shared secret: `Vec<u8>` returned from `ss.as_slice().to_vec()`, currently
  length-tested as 32 bytes.

Input length and conversion behavior:

- public-key conversion uses `ml_kem::Encoded::<MlKem768Ek>::try_from(pubk)`
  and maps failure to `CryptoError::InvalidKey`;
- secret-key conversion uses `ml_kem::Encoded::<MlKem768Dk>::try_from(privk)`
  and maps failure to `CryptoError::InvalidKey`;
- ciphertext conversion uses `ml_kem::Ciphertext::<ml_kem::MlKem768>::try_from(ct)`
  and maps failure to `CryptoError::InvalidKey`;
- encapsulation maps provider failure to `CryptoError::InvalidKey`;
- decapsulation maps provider decapsulation failure to `CryptoError::AuthFail`.

Fail-closed assessment:

- malformed public keys fail before encapsulation completes;
- malformed secret keys fail before decapsulation uses the key;
- wrong-length ciphertexts fail before decapsulation;
- valid-length tampered ciphertexts decapsulate to a different shared secret,
  consistent with implicit-rejection behavior covered by tests;
- qsc maps provider encap/decap failures into sanitized `handshake_reject`
  markers with `pq_encap_failed` or `pq_decap_failed` reasons.

Error boundary assessment:

- provider-level errors are deterministic enough for current tests: malformed
  lengths map to `CryptoError::InvalidKey`;
- qsc public/operator boundary receives sanitized reason labels rather than raw
  provider internals;
- direct qsc tests for provider failure paths are incomplete, especially for
  malformed-but-frame-shaped KEM public keys or ciphertexts.

Secret material exposure:

- the trait exposes raw shared-secret bytes and helper functions expose raw KEM
  secret key bytes as `Vec<u8>` because that is the existing boundary contract;
- no evidence proves KEM `Vec<u8>` shared-secret or secret-key buffers are
  zeroized on drop after crossing the boundary;
- this is a future memory-hygiene/secret-material classification item, not an
  in-scope NA-0426 code change.

qsc runtime use:

- qsc uses `quantumshield_refimpl` with `features = ["pqcrypto"]`;
- that historical feature currently enables `pqkem` plus `ml-dsa`, not
  pqcrypto KEM crates;
- qsc consumes runtime PQ helper functions and `PqKem768` rather than directly
  naming `ml-kem`.

Historical feature-name caveat:

- the `pqcrypto` feature name remains as a compatibility label while mapping to
  `ml-kem`/`ml-dsa` providers;
- this is potentially misleading to maintainers and should be triaged for
  future naming/documentation migration, but no rename is authorized by NA-0426.

pqcrypto crate status:

- root locked graph: `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` are absent;
- nested qsc fuzz lock: the separate `qsl/qsl-client/qsc/fuzz/Cargo.lock`
  still contains pqcrypto package entries and should be future-triaged.

## Fail-closed / negative test coverage review

Provider-level coverage present:

- `pqkem768_roundtrip_matches`
- `pqkem768_tamper_changes_secret`
- `pqkem768_wrong_length_inputs_fail_closed`
- `stdcrypto::tests::pqkem768_roundtrip_and_lengths`
- `stdcrypto::tests::pqkem768_tamper_changes_secret`
- `stdcrypto::tests::runtime_pq_boundary_helpers_match_provider_lengths`

Required checks:

- roundtrip test exists: yes.
- tampered ciphertext test exists or equivalent: yes, valid-length tamper
  changes the shared secret.
- wrong-length public key reject test exists: yes.
- wrong-length secret key reject test exists: yes.
- wrong-length ciphertext reject test exists: yes.
- deterministic reject behavior is asserted where feasible: yes for
  provider-level wrong-length errors through `CryptoError::InvalidKey`.
- no mutation-on-reject evidence exists or is identified as a gap:
  provider-level KEM operations are stateless; qsc has broader handshake
  no-mutation tests, but direct provider-error no-mutation at qsc boundary is
  not specifically proven.
- malformed inputs are covered only at provider level or also higher API
  boundary: provider-level length errors are covered directly; qsc covers frame
  length, transcript, replay, and tamper rejects broadly, but not every
  provider error path directly.
- test names and assertions clearly document fail-closed behavior: provider
  wrong-length test does; tamper test documents implicit-rejection behavior by
  changed secret rather than rejecting.

Coverage gaps:

- no direct qsc test forces `pq_encap_failed` through a malformed but
  frame-shaped KEM public key;
- no direct qsc test forces `pq_decap_failed` beyond broader tamper/parse paths;
- no property/fuzz/differential test currently targets the `PqKem768` provider
  boundary;
- no KAT/differential vector harness maps the active `ml-kem` provider outputs
  to external or generated ML-KEM fixtures;
- no dedicated secret-buffer zeroization test for KEM `Vec<u8>` outputs;
- no side-channel/timing validation for provider operations.

Recommended future coverage:

- qsc boundary tests for provider encap/decap failure markers and no mutation;
- property tests for wrong-length and random malformed inputs;
- differential/KAT checks against maintained ML-KEM fixtures if future scope
  authorizes vectors;
- fuzz harness refresh or dependency cleanup before fuzz evidence is used.

## Dependency / feature boundary review

Commands run/read:

- `cargo audit --deny warnings`: green.
- `cargo tree -i rustls-webpki --locked`: `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked`: `ml-kem v0.2.1` through
  `quantumshield_refimpl`, `qsc`, `qsl-tui`, and `refimpl_actor`.
- `cargo tree -i pqcrypto-mlkem --locked || true`: package ID absent from the
  root locked graph.
- `cargo tree -i pqcrypto-traits --locked || true`: package ID absent from the
  root locked graph.
- `cargo tree -i pqcrypto-internals --locked || true`: package ID absent from
  the root locked graph.
- `cargo metadata --locked --format-version=1 --no-deps`: root workspace has
  five packages; `quantumshield_refimpl` features map `pqkem` to `ml-kem` and
  `pqcrypto` to `pqkem` plus `ml-dsa`; `qsc` depends on
  `quantumshield_refimpl` with `features = ["pqcrypto"]`.

Dependency findings:

- direct root provider dependency is `ml-kem 0.2.1` with `zeroize` feature;
- qsc feature selection still uses the historical `pqcrypto` name;
- root pqcrypto unmaintained crates are absent;
- nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` still records pqcrypto package
  entries outside root workspace metadata;
- no dependency-family duplication blocker was found in the root locked graph,
  but nested fuzz dependency evidence should not be ignored if fuzz validation
  is revived.

## Formal / vector / implementation alignment review

Formal roots:

- `formal/README.md`
- `formal/model_scka_bounded.py`
- `formal/model_suite2_negotiation_bounded.py`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`

Vector roots:

- `inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json`
- `inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json`
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`

Current alignment:

- formal models are deliberately crypto-agnostic and check state-machine,
  negotiation, suite-id, and no-mutation properties;
- `inputs/suite2/vectors/README.md` names ML-KEM-768 fixtures and invalid
  input size rejects as CAT-SCKA-KEM-001 coverage intent;
- `qshield_suite2_scka_kem_vectors_v1.json` contains roundtrip, wrong-key,
  tamper, and invalid-size KEM vector expectations;
- qsc suite-id vectors are consumed by refimpl oracle and qsc tests for
  handshake parameter-block semantics, not direct provider outputs.

Gaps:

- provider boundary tests are independent of protocol transcript tests;
- formal models do not directly model KEM provider byte conversions, provider
  failure classes, or side-channel behavior;
- vector evidence exists for ML-KEM-768 categories, but current audit did not
  prove direct execution of all provider outputs against those vectors;
- future audit should map provider behavior to suite/vector/formal assumptions
  before expanding public claims.

Recommended future work:

- map `PqKem768` provider behavior to SCKA KEM vector categories;
- decide whether a KAT/differential harness is needed;
- decide whether qsc provider-error paths should become explicit model/vector
  assumptions or only implementation tests.

## Public claim / service / demo boundary review

This provider boundary audit is internal governance evidence only.

- It is not external review.
- It is not production-readiness evidence.
- It is not public-internet-readiness evidence.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.
- It is not metadata-free proof.
- It is not anonymity proof.
- It is not untraceability proof.
- It is not public technical paper content.
- It does not update README, START_HERE, public docs, docs-public, or website.
- It does not mutate qsl-server or qsl-attachments.
- Cargo audit green is dependency-health evidence, not vulnerability-free
  proof.

## Findings matrix

| ID | Title | Domain | Severity | Evidence references | Affected files/roots | Steward domain | Public-claim impact | Recommended queue action | Immediate blocker successor needed |
|---|---|---|---|---|---|---|---|---|---|
| F-0426-01 | Provider boundary preserved after ml-kem remediation | Provider boundary current status | INFO | `traits.rs` `PqKem768`; `stdcrypto.rs` `impl PqKem768`; NA-0418 evidence | `tools/refimpl/quantumshield_refimpl/src/crypto/`; `qsl/qsl-client/qsc/` | Crypto / Protocol | No crypto-complete or external-review-complete claim | Carry into NA-0427 as accepted baseline | No |
| F-0426-02 | Provider-level wrong-length rejects are covered; qsc provider-error paths need targeted proof | Fail-closed coverage status | MEDIUM / EVIDENCE_INCOMPLETE | `tests/pqkem768.rs`; qsc `pq_encap_failed` / `pq_decap_failed` markers | `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`; `qsl/qsl-client/qsc/src/handshake/mod.rs`; qsc tests | Crypto / Protocol | No fail-closed-complete claim | NA-0427 triage should decide provider-error qsc tests | No |
| F-0426-03 | Historical `pqcrypto` feature name now maps to ml-kem/ml-dsa | Feature naming compatibility caveat | LOW / CLAIM_BOUNDARY | `quantumshield_refimpl/Cargo.toml`; `qsc/Cargo.toml`; `cargo metadata` | Cargo manifests | CI / Dependency | Avoid implying pqcrypto provider is active in root graph | NA-0427 triage should decide naming/documentation migration | No |
| F-0426-04 | Nested qsc fuzz lock still contains pqcrypto packages outside root workspace | Dependency health evidence gap | MEDIUM / EVIDENCE_INCOMPLETE | `qsl/qsl-client/qsc/fuzz/Cargo.lock`; root `cargo metadata`; root inverse-tree absence | `qsl/qsl-client/qsc/fuzz/`; root Cargo files | CI / Dependency | Cargo audit green is not fuzz-lock vulnerability-free proof | NA-0427 triage should decide fuzz lock remediation scope | No |
| F-0426-05 | Formal/vector evidence does not directly prove provider implementation alignment | Formal/vector alignment gap | MEDIUM / EVIDENCE_INCOMPLETE | `formal/README.md`; KEM vectors; provider tests | `formal/`; `inputs/suite2/`; provider tests | Crypto / Protocol | No formally-proven implementation claim | NA-0427 triage should decide KAT/differential/vector mapping lane | No |
| F-0426-06 | Property/fuzz/differential coverage for provider boundary is incomplete | Property/fuzz/differential coverage gap | MEDIUM / EVIDENCE_INCOMPLETE | qsc fuzz crate; provider tests; NA-0425 audit domain matrix | `qsl/qsl-client/qsc/fuzz/`; `tests/`; `tools/refimpl/` | Crypto / Protocol plus CI / Dependency | No exhaustive-testing, bug-free, or vulnerability-free claim | NA-0427 triage should decide coverage expansion | No |
| F-0426-07 | KEM shared-secret and secret-key outputs cross boundary as raw `Vec<u8>` | Secret material / memory hygiene caveat | MEDIUM / EVIDENCE_INCOMPLETE | `PqKem768` trait; `runtime_pq_kem_keypair`; qsc identity vault storage | `tools/refimpl/quantumshield_refimpl/src/crypto/`; `qsl/qsl-client/qsc/src/identity/mod.rs` | Crypto / Protocol | No secret-handling-complete or memory-safety-complete claim | NA-0427 triage should decide zeroization/memory hygiene lane | No |
| F-0426-08 | Provider side-channel/timing status is not established | Side-channel/timing caveat | CLAIM_BOUNDARY / EVIDENCE_INCOMPLETE | NA-0425 side-channel audit inheritance; provider code/deps | `tools/refimpl/`; dependency tree | Crypto / Protocol plus Public Claims | No side-channel-free or constant-time-guaranteed claim | NA-0427 triage should preserve caveat and defer side-channel audit | No |
| F-0426-09 | Service/demo/public-readiness boundaries remain separate | Service/demo/public-claim caveat | CLAIM_BOUNDARY | Stewardship canon; NA-0425 inheritance; TRACEABILITY service rows | governance evidence; qsl-server/qsl-attachments references only | Product / Demo / Service plus Public Claims | No production-readiness or public-internet-readiness claim | NA-0427 triage should keep service/demo caveats explicit | No |

## Successor selection

Selected successor:

`NA-0427 -- QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan`

Selection rationale:

- no BLOCKER or HIGH runtime issue was found;
- the findings matrix contains meaningful provider-boundary follow-up items;
- remediation, feature-name migration, fuzz-lock refresh, KAT/differential
  coverage, qsc provider-error tests, and memory-hygiene changes would require
  future exact scope;
- moving directly to nonce/key/RNG would skip unresolved provider-boundary
  findings.

Rejected successor options:

- `QSL Crypto API / Provider Boundary Blocker Remediation Authorization Plan`:
  rejected because no clear BLOCKER/HIGH runtime issue was found.
- `QSL Crypto API / Provider Boundary Evidence Gap Resolution Plan`: rejected
  because evidence was sufficient to create a findings matrix and normal triage
  successor.
- `QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`: rejected for now
  because provider-boundary findings should be triaged first.

## Future path/scope bundle

Future allowed paths for normal NA-0427:

- `docs/governance/evidence/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_plan.md`
- `tests/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0427 may:

- triage F-0426 findings;
- decide whether implementation remediation is needed;
- decide whether feature-name clarification is needed;
- decide whether property/fuzz/differential coverage is needed;
- decide whether formal/vector alignment is needed;
- decide whether claim-boundary follow-up is needed.

Future forbidden unless exact scope authorizes:

- runtime/crypto implementation changes;
- dependency or Cargo changes;
- workflow changes;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- backup/restore/qsl-backup/status/plan changes;
- public claims.

## Future validation/marker plan

Future NA-0427 markers:

- `NA0427_PROVIDER_BOUNDARY_FINDINGS_TRIAGE_OK`
- `NA0427_FINDINGS_MATRIX_CONSUMED_OK`
- `NA0427_NO_RUNTIME_CHANGE_OK`
- `NA0427_NO_DEPENDENCY_CHANGE_OK`
- `NA0427_NO_WORKFLOW_CHANGE_OK`
- `NA0427_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0427_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0427_NO_SECRET_MATERIAL_OK`
- `NA0427_ONE_READY_INVARIANT_OK`
- `NA0427_STEWARD_REVIEW_TEMPLATE_USED_OK`

No blocker-specific or evidence-gap-specific marker is selected because the
normal findings triage successor is selected.

## Public claim/external review/website boundary

Required boundary statements:

- provider boundary audit is internal governance evidence only;
- provider boundary audit is not external review;
- provider boundary audit is not production-readiness evidence;
- provider boundary audit is not public-internet-readiness evidence;
- provider boundary audit is not crypto-complete proof;
- provider boundary audit is not side-channel-free proof;
- provider boundary audit is not bug-free proof;
- provider boundary audit is not vulnerability-free proof;
- provider boundary audit is not perfect-crypto proof;
- provider boundary audit is not public technical paper content;
- provider boundary audit does not support metadata-free claims;
- provider boundary audit does not support anonymity claims;
- provider boundary audit does not support untraceability claims;
- no README, START_HERE, docs-public, public docs, or website update occurred;
- cargo audit green is dependency-health evidence only.

## Rejected alternatives

- Mutate provider code now: rejected because NA-0426 is read-only.
- Rename the historical `pqcrypto` feature now: rejected because Cargo changes
  are out of scope.
- Refresh the nested fuzz lock now: rejected because Cargo/lockfile changes are
  out of scope.
- Add qsc provider-error tests now: rejected because test changes are out of
  scope.
- Add KAT/differential vectors now: rejected because test/vector changes are
  out of scope.
- Create website or public-doc claim text: rejected because public surfaces are
  out of scope and public claims remain unsupported.
- Select a blocker successor: rejected because no BLOCKER/HIGH runtime issue
  was found.
- Move directly to nonce/key/RNG audit: rejected because provider-boundary
  findings should be triaged first.

## Backup-impact statement

Codex did not run backup or restore. Codex did not mutate qsl-backup,
`/backup/qsl`, backup logs, backup manifests, backup status files, backup plan
files, rollback subtree paths, systemd, timers, fstab, source lists, retention,
or backup scripts.

Read-only proof:

- qsl-backup SHA matched
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- qsl-backup source inclusion count for `/home/victor/work/qsl/codex/ops` was
  exactly `1`.

This audit makes no backup-complete claim, no off-host-backup-complete claim,
no disaster-recovery-complete claim, and no restore-proven claim.

## Next recommendation

Merge the NA-0426 read-only audit evidence PR if validation and required checks
remain green. After post-merge public-safety is green, close out NA-0426 and
restore the selected normal NA-0427 findings triage / remediation
authorization plan as the sole READY item. Do not implement NA-0427 in the
NA-0426 audit PR.
