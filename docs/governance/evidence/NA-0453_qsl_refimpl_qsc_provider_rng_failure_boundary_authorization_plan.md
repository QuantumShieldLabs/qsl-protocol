Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0453 QSL refimpl / qsc Provider RNG Failure Boundary Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0453 consumes the NA-0447 through NA-0452 RNG failure evidence chain and
classifies provider-dependent RNG boundaries across qsc and the refimpl provider
layer.

Primary classification:

`PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`

Selected successor:

`NA-0454 -- QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

Reason: qsc and refimpl expose some provider `Result` boundaries, but current
APIs do not let tests force concrete provider RNG failure without a deliberate
fake or test-seam design. A direct implementation lane would risk changing
provider traits or production semantics before the exact boundary is authorized.

NA-0453 is authorization-only. It changes no runtime behavior, crypto behavior,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qsl-backup, backup status, backup plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
Cargo audit green remains dependency-health evidence only.

## Live NA-0453 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0453.
- NA-0452 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0892.
- D-0891 exists once.
- D-0892 exists once.
- D-0893 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0453 mutation paths are exactly:

- `docs/governance/evidence/NA-0453_qsl_refimpl_qsc_provider_rng_failure_boundary_authorization_plan.md`
- `tests/NA-0453_qsl_refimpl_qsc_provider_rng_failure_boundary_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public-doc, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
rollback, and backup tree paths.

Acceptance criteria:

- provider-dependent qsc and refimpl RNG surfaces are classified;
- whether failures can be forced through existing APIs is answered;
- qsc/refimpl split and successor are selected;
- future allowed paths, forbidden paths, and markers are recorded;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, missing
public-safety, missing D-0892, D-0893 already present, failed root or nested
audit, unclassifiable provider surfaces, unsafe successor selection, backup
boundary regression, or any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0453/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0453/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0453`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0453/qsl-protocol`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0453`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`2c73503c0a67`. Fetch did not advance `origin/main`.

PR #1174 was verified MERGED at `2c73503c0a67`.

Proof root:

`/srv/qbuild/tmp/NA0453_provider_rng_boundary_auth_20260609T224414Z`

## NA-0452 inheritance

NA-0452 / D-0891 implemented the bounded qsc route/contact/attachment RNG
failure test-only seam:

- handshake session ID RNG coverage was inherited from NA-0449;
- selected vault/session-store RNG coverage was inherited from NA-0449;
- route/default-route/relay token RNG coverage was implemented;
- contact route-token RNG coverage was implemented;
- attachment ID, CEK, and nonce-prefix RNG coverage was implemented.

NA-0452 explicitly deferred:

- provider-dependent qsc RNG;
- refimpl/provider RNG;
- qshield-cli demo RNG;
- formal/model RNG;
- fuzz/vector RNG.

NA-0453 consumes that deferred provider boundary only as authorization evidence.
It does not implement provider seams, test doubles, runtime code, crypto code, or
new tests.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- provider-dependent RNG boundary work must remain bounded;
- qsc provider call-site behavior must stay distinct from refimpl/provider
  internals;
- production semantics must be preserved;
- No RNG-failure-complete claim is made;
- No provider-RNG-complete claim is made.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green;
- nested qsc fuzz-lock audit is green;
- cfg RNG failure tests are green;
- route/contact/attachment seam evidence is green;
- qsc key lifecycle and provider-error tests are green;
- qsc adversarial script shape is present and syntax-green locally;
- public-safety is green on current main;
- cargo audit green is dependency-health evidence only.

Public Claims / External Review Steward:

- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- Provider RNG boundary scope is internal governance evidence only.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local.
- qsl-server remains a service boundary.
- qsl-attachments remains a service boundary.
- No qshield, website, or public-service readiness claim is made.

Local Ops / Backup / Restore Steward:

- Codex did not run backup or restore.
- Codex did not mutate local ops, qsl-backup, backup status, backup plan,
  rollback, or backup tree paths.
- qsl-backup proof remains boundary evidence only.

## qsc provider-dependent RNG surface inventory

| Surface | Path(s) | Provider operation | RNG role | Current error handling | Existing coverage | Existing qsc RNG seam can reach it? | Future lane fit | Risk | Priority |
|---|---|---|---|---|---|---|---|---|---|
| qsc KEM keypair provider boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_kem_keypair()` via refimpl runtime ML-KEM keypair helper | provider keypair generation uses concrete provider RNG | infallible tuple from qsc perspective | identity and handshake paths exercise success only | no | fake/seam strategy, then exact qsc/refimpl split | medium: trait or helper changes could affect production semantics | high |
| qsc KEM encap boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::encap()` during responder handshake | provider encapsulation uses concrete provider RNG | `Result`; failure maps to sanitized `pq_encap_failed` reject | prior provider-error evidence records defensive boundary; no external forcing path | no | fake/seam strategy | medium: must not create externally reachable bypass | high |
| qsc KEM decap boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::decap()` during initiator finalize | no RNG expected at decap time | `Result`; failure maps to sanitized `pq_decap_failed` reject before session mutation | `handshake_provider_error_no_mutation` covers reject/no-mutation | no RNG seam needed for decap; provider fake already useful for error boundary | documentation plus future fake/seam strategy for parity | low | medium |
| qsc signature keypair boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` via refimpl runtime ML-DSA keypair helper | provider seed generation uses concrete provider RNG | infallible tuple from qsc perspective | success paths only | no | fake/seam strategy | medium: identity writes must remain fail-closed if made fallible | high |
| qsc signing boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` | no RNG role confirmed in current refimpl signing path after key generation | `Result`; failure maps to `sig_sign_failed` | not specifically forced in qsc tests | no | fake/seam strategy or provider-error documentation | low | medium |
| qsc signature verify boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::verify()` | no RNG role | `Result<bool>` mapped to sanitized `sig_invalid` | success and reject paths exist indirectly | no | documentation unless later provider-error lane selects it | low | low |
| qsc X25519 ephemeral keypair boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::keypair()` | X25519 private key bytes use concrete provider RNG | infallible tuple from qsc perspective | success paths only | no | fake/seam strategy | medium: occurs before pending-state write in init path | high |
| qsc identity bootstrap provider boundary | `qsl/qsl-client/qsc/src/identity/mod.rs` | KEM and signature identity keypair creation | provider RNG for generated identity secrets | infallible helpers; subsequent vault/public-record writes | success paths and separate key lifecycle coverage | no | fake/seam strategy | high: partial identity write ordering must be exact if made fallible | high |

qsc classification summary:

- `QSC_PROVIDER_RNG_ERROR_PROPAGATION_PARTIAL`
- `PROVIDER_RNG_FAKE_OR_SEAM_NEEDED`

The existing qsc provider-error no-mutation test covers `pq_decap_failed`, but
it does not prove provider RNG failure for keypair, encapsulation, signature
keypair, or X25519 keypair paths.

## refimpl provider RNG surface inventory

| Surface | Path(s) | Provider operation | RNG role | Error return shape | Can RNG failure be forced through existing API? | Underlying fallible RNG exposed? | Future test shape | Risk | Priority |
|---|---|---|---|---|---|---|---|---|---|
| refimpl ML-KEM keypair | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | `runtime_pq_kem_keypair()` | ML-KEM key generation uses concrete `OsRng` | infallible tuple | no | no | cfg seam or provider helper trait strategy | medium | high |
| refimpl ML-KEM encap | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | `PqKem768::encap()` | encapsulation uses concrete `OsRng` | `Result<(Vec<u8>, Vec<u8>), CryptoError>` | partial: fake `PqKem768` can return `Err`, but concrete RNG failure is not forceable | no | fake provider strategy can model provider error; seam needed for concrete RNG | medium | high |
| refimpl ML-KEM decap | same as above | `PqKem768::decap()` | no RNG expected | `Result<Vec<u8>, CryptoError>` | yes for provider error via fakes; no for RNG because no RNG role | not applicable | documentation plus provider-error fakes | low | medium |
| refimpl ML-DSA keypair | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | `runtime_pq_sig_keypair()` | seed bytes use concrete `OsRng` | infallible tuple | no | no | cfg seam or helper trait strategy | medium | high |
| refimpl ML-DSA sign | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | `PqSigMldsa65::sign()` | no RNG role confirmed after key generation | `Result<Vec<u8>, CryptoError>` | yes for generic provider error via fakes; not RNG-specific | no | provider fake documentation or exact fake strategy | low | medium |
| refimpl ML-DSA verify | same as above | `PqSigMldsa65::verify()` | no RNG role | `Result<bool, CryptoError>` | yes for generic provider error via fakes | not applicable | documentation | low | low |
| refimpl X25519 keypair | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | `X25519Dh::keypair()` | private key bytes use concrete `OsRng` | infallible tuple | no | no | trait strategy needed before fallible test proof | medium | high |
| refimpl random nonce helper | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | `Rng12::random_nonce12()` | nonce bytes use concrete `OsRng` | infallible `[u8; 12]` | no | no | separate fallible RNG trait decision or documentation | medium | medium |
| refimpl qsp handshake injected providers | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` | injected `PqKem768`, `PqSigMldsa65`, `X25519Dh` | provider fakes can model provider errors; DH keypair remains infallible | mixed Result and infallible trait methods | partial | no concrete RNG failure exposed | fake/seam strategy | medium | high |
| refimpl qsp ratchet injected providers | `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs` | injected `PqKem768`, `X25519Dh`, `Rng12` | PQ encap and nonce generation can be modeled, but `Rng12` is infallible | mixed Result and infallible trait methods | partial | no concrete RNG failure exposed | fake/seam strategy | medium | medium |

refimpl classification summary:

- `REFIMPL_PROVIDER_RNG_BOUNDARY_PARTIAL`
- `PROVIDER_RNG_FAKE_OR_SEAM_NEEDED`

Existing refimpl tests already use provider fakes for deterministic provider
error behavior. They do not prove concrete provider RNG failure because the
current RNG-bearing methods use concrete `OsRng` or infallible trait shapes.

## provider RNG error propagation review

Questions and answers:

1. Provider-dependent RNG operations returning `Result` today: ML-KEM encap
   returns `Result`; qsc maps responder encap failure to `pq_encap_failed`.
   Provider sign and verify also return `Result`, but current signing does not
   expose a confirmed RNG role after key generation.
2. Randomness-dependent operations effectively infallible from qsc/refimpl
   perspective: ML-KEM keypair, ML-DSA keypair, X25519 keypair, and `Rng12`
   nonce generation.
3. qsc already handles provider failures at decap, encap, sign, and verify call
   sites with sanitized reject/error strings.
4. refimpl maps provider errors to `CryptoError` and qsp call sites propagate
   deterministic errors before selected state commits in existing tests.
5. Observed qsc provider errors are sanitized and non-secret-bearing.
6. No-mutation evidence exists for qsc decap reject and selected refimpl qsp
   failure tests, but not for concrete provider RNG failure.
7. A future provider fake or test seam is needed to force provider RNG failure.
8. Such a fake/seam must be designed so production semantics remain unchanged.
9. The qsc `qsc_rng_failure_test_seam` pattern may be reusable at qsc local RNG
   labels, but provider-boundary injection is cleaner for provider internals.
10. This domain needs a narrower fake/seam strategy authorization lane before
    implementation.

Classifications:

- `QSC_PROVIDER_RNG_ERROR_PROPAGATION_PARTIAL`
- `REFIMPL_PROVIDER_RNG_BOUNDARY_PARTIAL`
- `PROVIDER_RNG_FAKE_OR_SEAM_NEEDED`
- `PROVIDER_RNG_BACKLOG`

## qsc vs refimpl split decision

The next successor should not be qsc-only or refimpl-only yet. qsc observes
provider failures at handshake and identity call sites, while refimpl owns the
provider trait and concrete `StdCrypto` behavior. The main unresolved decision
is not which side to implement first; it is how to force provider RNG failure
without widening production semantics.

Rejected split choices:

- qsc-only boundary authorization: rejected because key RNG behavior is hidden
  inside refimpl helpers and concrete `StdCrypto`.
- refimpl-only boundary authorization: rejected because qsc identity and
  handshake no-mutation boundaries still need explicit call-site decisions.
- direct combined implementation: rejected because exact mutable source/test
  paths are not safe until the fake/seam strategy is selected.
- documentation-only: rejected because confirmed testability gaps remain
  actionable through a bounded strategy lane.
- next KEM/signature/transcript audit immediately: rejected because provider
  RNG boundary evidence is the selected residual from D-0891 and is not yet
  resolved.

Selected split:

`PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`

The successor should authorize whether the future proof uses provider fakes,
cfg-only seams, trait extraction, or documentation-only residual classification
for each qsc/refimpl surface.

## authorization matrix

| Surface | Path(s) | Provider/RNG operation | Result/error shape | Existing coverage | Existing no-mutation evidence | Existing API enough? | Future seam/fake needed? | Future mutable paths, if selected | Recommended lane type | Priority | Risk | Selected for successor? | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| qsc KEM key/encap/decap provider boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | ML-KEM keypair, encap, decap via provider | keypair infallible; encap/decap `Result` | success paths; decap reject test | decap no-mutation yes; keypair/encap RNG no | partial | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | No provider-RNG-complete claim is made. | G1, G2, G3, G4, G5 |
| qsc signature/provider-dependent identity boundary | same qsc handshake/identity paths | ML-DSA keypair, sign, verify | keypair infallible; sign/verify `Result` | success paths | not for RNG failure | no | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | No crypto-complete claim is made. | G1, G2, G3, G4, G5 |
| qsc handshake provider RNG boundaries | `qsl/qsl-client/qsc/src/handshake/mod.rs` | X25519 keypair, KEM encap/keypair, signature keypair | mixed infallible and `Result` | qsc handshake tests, provider-error decap | decap no-mutation only | partial | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | No RNG-failure-complete claim is made. | G1, G2, G3, G4, G5 |
| refimpl ML-KEM provider RNG boundary | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `traits.rs` | keypair and encap use concrete RNG | keypair infallible; encap `Result` | pqkem768 tests | generic provider-error only | partial | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | Bounded internal evidence only. | G1, G2, G3, G4, G5 |
| refimpl ML-DSA provider RNG boundary | same refimpl crypto paths | keypair seed RNG; sign/verify provider ops | keypair infallible; sign/verify `Result` | indirect qsc/refimpl success | generic provider-error only | partial | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | No public-readiness claim is made. | G1, G2, G3, G4, G5 |
| refimpl X25519 provider randomness boundary | same refimpl crypto paths | X25519 keypair uses concrete RNG | infallible tuple | success paths | none for RNG failure | no | yes | not authorized by NA-0453 | fake/seam strategy | high | medium | yes | No production-readiness claim is made. | G1, G2, G3, G4, G5 |
| refimpl helper random bytes / Rng12 boundary | same refimpl crypto paths; qsp ratchet | nonce helper uses concrete RNG | infallible array | deterministic fakes for success tests | none for RNG failure | no | yes or documentation | not authorized by NA-0453 | fake/seam strategy | medium | medium | yes | No side-channel-free claim is made. | G1, G2, G3, G4, G5 |
| qshield-cli demo RNG boundary | `apps/qshield-cli/` | demo token generation | demo-local fallible file/RNG behavior | read-only evidence only | none selected | no | no current action | not authorized by NA-0453 | backlog/documentation | low | low | no | Demo-local only; no public-service claim is made. | G4, G5 |
| formal/model provider RNG residual | `formal/` | model-level RNG/provider failures | not implemented | existing model checks green | not provider-RNG-specific | no | no current action | not authorized by NA-0453 | backlog | low | low | no | No formal proof expansion is claimed. | G4, G5 |
| fuzz/vector provider RNG residual | `qsl/qsl-client/qsc/fuzz/`; `inputs/` | provider RNG fuzz/vector behavior | not implemented | nested fuzz lock audit green | none selected | no | no current action | not authorized by NA-0453 | backlog | low | low | no | No vector-complete claim is made. | G4, G5 |

## authorization decision

Selected primary classification:

`PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`

The highest-priority successor is a strategy authorization lane, not direct
implementation. Future work must first decide whether provider failures are best
modeled with existing provider fakes, new cfg-only seams, trait shape changes,
or documentation-only residual classification per surface.

NA-0453 authorizes no runtime, crypto, dependency, Cargo, lockfile, workflow,
test-source, fuzz-target, vector, or formal-model mutation.

## successor selection

Selected exact NA-0454 successor:

`NA-0454 -- QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

NA-0454 must remain authorization-only unless a later directive gives exact
implementation paths. It must not implement provider seams, fakes, runtime code,
crypto code, test source, fuzz targets, vectors, formal models, dependencies,
Cargo files, lockfiles, workflows, public docs, website, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, backup, restore, or qwork tooling.

## future path/scope bundle

Future allowed NA-0454 mutation paths:

- `docs/governance/evidence/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future read-only inspection may include:

- `tools/refimpl/`
- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts and workflows read-only.

Future forbidden unless exact scope authorizes:

- runtime or crypto implementation changes;
- dependency, Cargo, lockfile, or workflow changes;
- executable test, fuzz target, vector, or formal model changes;
- public docs, website, README, or START_HERE changes;
- qsl-server, qsl-attachments, qshield runtime, or qshield-cli changes;
- qwork/qstart/qresume/qshell changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree changes;
- public claim expansion.

## future validation/marker plan

Common NA-0454 markers:

- `NA0454_PROVIDER_RNG_BOUNDARY_AUTHORIZATION_CONSUMED_OK`
- `NA0454_NEXT_SCOPE_SELECTED_OK`
- `NA0454_NO_RUNTIME_CHANGE_OK`
- `NA0454_NO_DEPENDENCY_CHANGE_OK`
- `NA0454_NO_WORKFLOW_CHANGE_OK`
- `NA0454_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0454_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0454_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0454_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0454_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0454_ONE_READY_INVARIANT_OK`
- `NA0454_PROVIDER_RNG_FAKE_OR_SEAM_STRATEGY_SELECTED_OK`

Future NA-0454 validation must include queue and decision proof, exact path
guard, link check, leak scan, overclaim scan, PR body preflight, goal-lint,
root audit, nested qsc fuzz-lock audit, relevant qsc/refimpl tests, formal model
checks, public-safety, and any qsc adversarial smoke required by policy.

## public claim/external review/website boundary

Provider RNG boundary authorization is internal governance evidence only.

No production-readiness claim is made.

No public-internet-readiness claim is made.

No crypto-complete claim is made.

No side-channel-free claim is made.

No RNG-failure-complete claim is made.

No provider-RNG-complete claim is made.

No bug-free claim is made.

No vulnerability-free claim is made.

No perfect-crypto claim is made.

No public technical paper is created.

No README, START_HERE, public docs, or website update is made.

Cargo audit green is dependency-health evidence only.

Future tests, if authorized, must be described as bounded evidence only.

## rejected alternatives

- `PROVIDER_RNG_BOUNDARY_QSC_SCOPE_NEXT`: rejected because qsc cannot force
  provider RNG failure without refimpl/provider strategy decisions.
- `PROVIDER_RNG_BOUNDARY_REFIMPL_SCOPE_NEXT`: rejected because qsc call-site
  no-mutation and sanitized-error boundaries must remain part of the strategy.
- `PROVIDER_RNG_BOUNDARY_COMBINED_SCOPE_NEXT`: rejected for immediate
  implementation because exact mutable paths are not safely selected yet.
- `PROVIDER_RNG_DOCUMENTATION_ONLY`: rejected because actionable fake/seam
  strategy questions remain.
- `PROVIDER_RNG_BACKLOG_NO_ACTION`: rejected because D-0891 selected provider
  RNG as the current residual domain.
- `NEXT_AUDIT_DOMAIN_KEM_SIGNATURE_TRANSCRIPT`: rejected until provider RNG
  fake/seam strategy is either selected or consciously deferred.

## backup-impact statement

Backup impact: none.

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths.

Read-only qsl-backup proof matched expected SHA `e9ecff3d22ed`; the codex-ops
source inclusion count remained exactly one.

## next recommendation

Restore NA-0454 as:

`QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

The first NA-0454 decision should pick the lowest-risk proof mechanism per
surface:

- existing provider fakes where generic provider errors are enough;
- cfg-only seams where concrete provider RNG failure must be forced;
- trait/API changes only if explicitly justified and bounded;
- documentation-only residuals where no safe forcing mechanism is justified.
