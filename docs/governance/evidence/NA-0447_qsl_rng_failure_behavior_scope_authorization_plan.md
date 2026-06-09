Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0447 QSL RNG Failure Behavior Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0447 consumes F-0441-03 and the NA-0441 through NA-0446 evidence chain. It
classifies RNG/randomness surfaces across qsc, refimpl, qshield-cli demo/test
code, fuzz targets, formal models, vectors, and governance evidence.

Selected primary classification:

`RNG_FAILURE_SCOPE_QSC_TEST_SEAM_NEXT`

Selected successor:

`NA-0448 -- QSL qsc RNG Failure Test Seam Authorization Plan`

This is scope authorization only. NA-0447 changes no runtime code, crypto code,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, public docs, website, README, START_HERE, backup/local-ops state,
or qwork tooling.

## Live NA-0447 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0447 -- QSL RNG Failure Behavior Scope Authorization Plan`

Status: READY.

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0447_qsl_rng_failure_behavior_scope_authorization_plan.md`
- `tests/NA-0447_qsl_rng_failure_behavior_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime code, crypto code, dependency
metadata, Cargo manifests, lockfiles, workflows, executable tests, fuzz targets,
vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status files, backup plan
files, rollback subtree paths, `/backup/qsl`, public technical paper content,
branch protection, and public claim surfaces.

Acceptance criteria:

- qwork proof files are verified without running qwork, qstart, or qresume;
- F-0441-03 is consumed as an RNG failure behavior evidence gap;
- NA-0446 implementation evidence is consumed as the preceding qsc
  key-lifecycle evidence;
- qsc/refimpl/qshield-cli/formal/fuzz/vector surfaces are classified;
- an exact NA-0448 successor is selected;
- no implementation mutation occurs;
- no public claim expansion occurs;
- root and nested fuzz lock cargo audits remain green;
- inherited qsc key lifecycle and provider-error tests remain green;
- public-safety remains green;
- exactly one READY item remains mandatory.

Stop conditions:

- qwork proof files are missing, malformed, stale, or inconsistent with live
  repo state;
- PR #1162 is not merged at the expected merge commit;
- queue state is not READY NA-0447 with READY_COUNT 1;
- D-0880 is absent or D-0881 exists before patching;
- F-0441-03 cannot be consumed truthfully;
- RNG surfaces cannot be safely classified;
- successor selection is ambiguous;
- any forbidden path would need mutation;
- public-safety, dependency health, or required inherited qsc tests fail
  conclusively.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0447/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0447/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0447`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0447/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0447`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, clean-state fields, READY count, top READY item, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `cbea6fc1b75d`. PR #1162 was verified MERGED with merge
commit `cbea6fc1b75d`.

Recorded timestamps:

- Local: `2026-06-08T20:18:12-05:00`
- UTC: `2026-06-09T01:18:12+00:00`

Proof root:

`/srv/qbuild/tmp/NA0447_rng_failure_behavior_scope_20260609T011928Z`

## F-0441-03 and NA-0446 inheritance

F-0441-03 from NA-0441 states that runtime cryptographic randomness sites use
`OsRng` or provider randomness where inspected, while deterministic RNGs are
test/demo/vector-bound, but RNG failure behavior is not directly modeled or
injectable.

NA-0442 preserved F-0441-03 as `RNG_FAILURE_SCOPE_AUTHORIZATION_NEEDED` and as
the next candidate after key lifecycle cleanup scope. It explicitly did not
claim RNG failure testing or formal modeling was complete.

NA-0446 then implemented bounded qsc key-lifecycle cleanup / zeroization tests
for pending-handshake cleanup, selected reject no-mutation boundaries,
session-store insertion only after success, encrypted-at-rest boundaries, and
redaction/passphrase sentinels. NA-0446 selected NA-0447 as the successor and
kept RNG failure behavior out of scope.

Inherited objective for NA-0447:

- define what RNG failure behavior means by surface;
- decide whether existing APIs can test it;
- decide whether a test seam/fake RNG is needed;
- decide whether formal/model work should precede qsc test-seam scoping;
- preserve claim boundaries and exact future scope.

## Applicable Stewardship Review

Level 1 stewardship is active in this evidence lane. Level 2 and Level 3 remain
future-gated. Stewards are advisory only: no separate Directors, no independent
READY promotion, no independent merge authority, and Lead Director final
authority is preserved.

Crypto / Protocol Steward:

- RNG failure behavior is a fail-closed evidence question, separate from RNG
  source quality.
- qsc OS randomness sites, refimpl provider randomness, qshield-cli demo
  determinism, and formal/model abstractions must be classified separately.
- Existing qsc tests cover provider-error no-mutation and key-lifecycle
  boundaries, but do not force OS RNG failure.
- No side-channel-free claim is made.
- No perfect-crypto claim is made.
- No vulnerability-free claim is made.

CI / Dependency / Release Health Steward:

- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` is green.
- `key_lifecycle_zeroization` and `handshake_provider_error_no_mutation` are
  green.
- qsc adversarial script marker and provider-error command remain present.
- public-safety is green on current `origin/main`.
- Cargo audit green is dependency-health evidence only.

Public Claims / External Review Steward:

- No RNG-failure-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- Evidence gaps are called gaps, not guarantees.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local.
- qsc runtime randomness and refimpl/provider randomness are separate
  implementation/provider boundaries.
- No qsl-server readiness claim is made. No qsl-attachments readiness claim is
  made. No qshield or website public-service readiness claim is made.

Local Ops / Backup / Restore Steward:

- No backup, restore, or local-ops mutation is authorized or performed.
- qsl-backup proof remains boundary evidence only.
- qsl-backup checksum matched the expected boundary value, and the Codex ops
  source-list inclusion count was exactly one.

## RNG surface inventory

| Surface group | Paths | RNG/randomness role | Failure observable through existing APIs? | Test seam/fake RNG needed? | Boundary |
|---|---|---|---|---|---|
| qsc handshake session ID | `qsl/qsl-client/qsc/src/handshake/mod.rs` | Runtime crypto session ID generation via `OsRng.fill_bytes` | No. `hs_session_id` and `hs_rand_bytes` are infallible and private | Yes | qsc |
| qsc handshake DH/KEM/signature provider | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | X25519 keypair, ML-KEM keypair/encap/decap, ML-DSA keypair/sign | Partly. KEM encap/decap/sign return `Result`, but keypair and OS RNG fill paths do not | Yes for RNG failure; existing provider-error APIs cover only bounded provider rejects | qsc/refimpl provider |
| qsc session store encryption | `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | Session-store key creation and random 12-byte AEAD nonce generation | No. `OsRng.fill_bytes` is infallible in current API shape | Yes | qsc |
| qsc vault encryption/init | `qsl/qsl-client/qsc/src/vault/mod.rs` | Vault nonces, salt, keychain fallback key bytes, and default route token generation | No for RNG failure; encryption/KDF failures are separate provider failures | Yes | qsc |
| qsc attachments | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Attachment IDs, CEKs, and nonce prefixes | No | Yes, but not immediate successor | qsc backlog |
| qsc contacts/TUI init | `qsl/qsl-client/qsc/src/contacts/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | Route tokens and account verification seed generation | No | Yes, but can be secondary to handshake/session/vault surfaces | qsc |
| qsc tests | `qsl/qsl-client/qsc/tests/**` | Deterministic seed fallback, fixtures, no-mutation tests, provider-error tests | Existing tests can assert behavior around malformed state, not OS RNG failure | New seam likely needed | qsc tests |
| refimpl `StdCrypto` | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | ML-KEM, ML-DSA, X25519, KEM encap, `StdRng` nonce generation | Partly. KEM APIs return `Result`; `X25519Dh::keypair` and `Rng12::random_nonce12` are infallible | Runtime propagation would need trait changes | refimpl provider |
| refimpl tests | `tools/refimpl/quantumshield_refimpl/tests/**` | Fixed RNGs and deterministic arrays for tests/vectors | Test fixtures can control values, not failure | Yes for failure | refimpl tests |
| qshield-cli demo | `apps/qshield-cli/src/**`; `apps/qshield-cli/tests/**` | Deterministic demo session IDs and `/dev/urandom` relay-token fallback | `/dev/urandom` token fallback returns read/open errors; deterministic demo establishment has no RNG failure | No immediate qsc scope; demo claim-boundary only | qshield-cli demo |
| qsc fuzz | `qsl/qsl-client/qsc/fuzz/**` | Parser/boundary fuzzing over input bytes | No RNG failure behavior exercised | Backlog if RNG API exists later | fuzz |
| formal models | `formal/*.py`; `formal/README.md` | Deterministic SCKA/negotiation/qsc suite-id fail-closed models | No RNG health state modeled | Model extension only after behavior contract is scoped | formal |
| vectors/inputs | `inputs/**` | Deterministic vectors for suite, KDF, SCKA, KEM, transcript, replay, crash/restart | No runtime RNG failure covered | Backlog after behavior contract exists | vectors |
| governance evidence | `docs/governance/evidence/**` | Prior evidence chain and caveats | Documents gap only | Supporting only | docs |

## qsc RNG failure scope review

qsc operations depending on OS/provider randomness:

- handshake session ID generation;
- handshake X25519 ephemeral keypair generation;
- identity KEM/signature keypair generation through refimpl runtime provider
  helpers;
- KEM encapsulation randomness through refimpl provider;
- session-store key and session blob nonce generation;
- vault salt, nonce, keychain fallback key bytes, and route token generation;
- contacts route token and TUI account verification seed generation;
- attachment ID, content encryption key, and nonce-prefix generation.

Explicit qsc error returns already exist for selected provider/error paths:

- KEM encapsulation, KEM decapsulation, signature signing, signature
  verification, length parsing, vault/KDF/encryption, and storage failures have
  bounded error paths.
- NA-0436 through NA-0440 and NA-0446 provide inherited provider-error and
  key-lifecycle evidence.

Current qsc tests cannot force OS RNG failure through existing public APIs. The
direct `OsRng.fill_bytes` sites and private helper functions are infallible at
the qsc API boundary. A future test would need a test-only RNG/provider seam or
equivalent fake provider strategy before executable RNG failure evidence is
truthful.

Runtime error propagation may be needed later if the future seam review decides
that fail-closed RNG failure semantics must be represented in production API
types. NA-0447 does not authorize that implementation.

Candidate future implementation paths, if a later directive explicitly
authorizes implementation after NA-0448:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- a future exact qsc test file, likely under `qsl/qsl-client/qsc/tests/`

qsc classification:

`QSC_RNG_FAILURE_TEST_SEAM_SCOPE_NEEDED`

Secondary classification:

`QSC_RNG_FAILURE_RUNTIME_ERROR_PROPAGATION_SCOPE_NEEDED`

The runtime-propagation classification is future-gated. The immediate successor
should authorize or reject the test-seam strategy first.

## refimpl RNG failure scope review

refimpl provider randomness surfaces:

- `runtime_pq_kem_keypair()` uses `MlKem768::generate(&mut OsRng)`;
- `runtime_pq_sig_keypair()` fills an ML-DSA seed from `OsRng`;
- `StdCrypto::keypair()` fills X25519 private key bytes from `OsRng`;
- `StdCrypto::encap()` uses KEM encapsulation with `OsRng`;
- `StdRng::random_nonce12()` fills 12 random bytes from `OsRng`.

APIs exposing failure:

- KEM `encap` and `decap` return `Result`.
- ML-DSA sign/verify return `Result`.
- AEAD open returns `Result`.

APIs assuming infallible randomness:

- `runtime_pq_kem_keypair()`;
- `runtime_pq_sig_keypair()`;
- `X25519Dh::keypair()`;
- `Rng12::random_nonce12()`.

refimpl has fixture seams such as `Rng12` and fixed RNGs, but `Rng12` returns a
nonce value directly rather than a `Result`, so it can control nonce values but
cannot model RNG failure without trait/runtime contract changes.

refimpl classification:

`REFIMPL_RNG_FAILURE_PROVIDER_BOUNDARY_BACKLOG`

refimpl should remain provider-boundary backlog for this lane. Immediate qsc
test-seam scope is higher leverage because qsc is the runtime client surface
that consumes F-0441-03 and can define expected fail-closed evidence before
refimpl trait changes are considered.

## qshield-cli / demo / test RNG boundary review

qshield-cli demo establishment derives demo session IDs and setup material
deterministically from IDs and placeholder public material. That is intentional
demo-local behavior and not a production randomness source.

qshield-cli relay-token fallback reads `/dev/urandom` and returns explicit
errors if opening or reading that device fails. That is a product/demo boundary
surface, not the immediate qsc RNG failure evidence target.

qsc tests use deterministic seed fallback through `QSC_QSP_SEED` only when
`QSC_ALLOW_SEED_FALLBACK` is enabled. That supports deterministic test
execution but does not model OS RNG failure.

Classifications:

- `QSHIELD_DEMO_RNG_CLAIM_BOUNDARY_ONLY`
- `QSHIELD_DEMO_RNG_BACKLOG`
- `TEST_RNG_BOUNDARY_SUPPORTING_ONLY`

qshield-cli should not be included in immediate RNG failure scope. It remains
demo-local claim-boundary work unless a future directive authorizes exact demo
storage/randomness review.

## Formal / model / fuzz RNG failure review

Current formal models represent deterministic state-machine and fail-closed
logic for bounded SCKA, negotiation, and qsc suite-id behavior. They do not
represent RNG source health, entropy failure, OS/provider failure, nonce
uniqueness across random draws, or injectable RNG failure semantics.

Current qsc fuzz targets exercise route HTTP parsing, payload boundary parsing,
and vault envelope parsing over fuzz input bytes. They do not exercise RNG
failure behavior.

Current vectors are deterministic inputs for Suite-2 KDF, SCKA, KEM, establish,
transcript, replay, crash/restart, and parsing behavior. They do not cover
runtime RNG failure.

Classifications:

- `RNG_FAILURE_FORMAL_SUPPORTING_ONLY`
- `RNG_FAILURE_FUZZ_BACKLOG`
- `RNG_FAILURE_VECTOR_BACKLOG`

Formal/model work should not run before qsc test-seam scoping. A model becomes
more meaningful after qsc/refimpl failure semantics are scoped.

## Scope decision matrix

| Surface | RNG/randomness role | Current evidence | Failure observability | Existing API enough? | Test seam needed? | Runtime error propagation needed? | Candidate future paths | Scope recommendation | Risk | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|
| qsc session ID generation | Handshake session ID from `OsRng` | NA-0441 source audit; qsc handshake tests | Not observable | No | Yes | Maybe later | `handshake/mod.rs`; future qsc RNG test | qsc test seam next | Medium evidence gap | No RNG-failure-complete claim is made | G1, G2, G4 |
| qsc vault/session nonce generation | Vault/session keys, salts, AEAD nonces | NA-0446 encrypted-at-rest tests; NA-0441 source audit | Not observable | No | Yes | Maybe later | `vault/mod.rs`; `protocol_state/mod.rs`; future qsc RNG test | qsc test seam next | Medium evidence gap | No crypto-complete claim is made | G2, G4, G5 |
| qsc KEM/key/provider-dependent generation | KEM/signature keypairs, KEM encap, X25519 keypair | Provider-error chain; refimpl provider tests | Partly for provider errors, not OS RNG failure | No | Yes | Maybe later | `handshake/mod.rs`; refimpl provider boundary | qsc seam first; refimpl backlog | Medium evidence gap | No vulnerability-free claim is made | G1, G2, G4 |
| refimpl provider randomness | ML-KEM, ML-DSA, X25519, `StdRng` | refimpl tests and source audit | Partly for provider errors; no infallible RNG failure | No | Yes | Yes for trait-level failure | `stdcrypto.rs`; `traits.rs` if later authorized | provider-boundary backlog | Medium | No perfect-crypto claim is made | G1, G2, G4 |
| qshield-cli demo/test randomness | Deterministic demo session IDs; `/dev/urandom` token fallback | demo-local tests and README boundary | `/dev/urandom` fallback errors observable; deterministic demo has no RNG failure | Not for qsc | No immediate | No immediate | qshield-cli paths only if future exact scope | claim-boundary/backlog | Low to medium claim risk | No production-readiness claim is made | G3, G4, G5 |
| qsc tests / deterministic seed surfaces | `QSC_QSP_SEED` fallback and fixtures | many deterministic qsc tests | Supports values, not RNG failure | No | Yes | No immediate | future qsc test seam authorization | supporting only | Low | No public-readiness claim is made | G4 |
| fuzz targets | parser/boundary fuzz input | qsc fuzz targets and fuzz lock audit | Not RNG failure | No | Only after API exists | No immediate | fuzz targets only if future exact scope | backlog | Low | No fuzz-complete claim is made | G4 |
| formal/model abstractions | deterministic state models | formal checks green | Not RNG health | No | Model extension after semantics | No immediate | formal files only if future exact scope | supporting/backlog | Medium evidence overclaim | No formal-complete claim is made | G2, G4 |

## Authorization decision

Selected primary classification:

`RNG_FAILURE_SCOPE_QSC_TEST_SEAM_NEXT`

Rationale:

- F-0441-03 is a real evidence gap: RNG failure behavior is not directly
  modeled or injectable.
- qsc is the runtime client surface with the clearest fail-closed relevance.
- Existing qsc APIs can test provider-error no-mutation and key-lifecycle
  cleanup, but cannot force OS RNG failure.
- A test seam or fake provider strategy appears safely scopeable, but exact
  implementation paths and semantics must be authorized before code changes.
- Refimpl/formal/fuzz/vector work remains useful but should follow or support a
  qsc semantics decision rather than precede it.
- Documentation-only evidence would preserve a caveat but would not improve
  executable fail-closed evidence.

NA-0447 authorizes no implementation mutation. NA-0447 authorizes no
runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
changes. NA-0447 expands no public claims.

## Successor selection

Selected exact successor:

`NA-0448 -- QSL qsc RNG Failure Test Seam Authorization Plan`

NA-0448 objective:

Authorize the exact future scope for a qsc test seam or equivalent bounded test
strategy that can exercise RNG failure behavior without changing production
semantics and without making any RNG-failure-complete claim.

Exactly one READY successor remains mandatory. NA-0447 does not implement
NA-0448.

## Future path/scope bundle

Future NA-0448 allowed mutation paths:

- `docs/governance/evidence/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_plan.md`
- `tests/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0448 may inspect read-only:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts/workflows read-only

Future forbidden scope unless a later exact directive authorizes it:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, website, README, or START_HERE changes;
- qsl-server or qsl-attachments changes;
- backup, restore, qsl-backup, status, plan, rollback, or `/backup/qsl`
  mutation;
- public claim expansion.

## Future validation/marker plan

Common future NA-0448 markers:

- `NA0448_RNG_FAILURE_SCOPE_CONSUMED_OK`
- `NA0448_NEXT_SCOPE_SELECTED_OK`
- `NA0448_NO_RUNTIME_CHANGE_OK`
- `NA0448_NO_DEPENDENCY_CHANGE_OK`
- `NA0448_NO_WORKFLOW_CHANGE_OK`
- `NA0448_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0448_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0448_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0448_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0448_ONE_READY_INVARIANT_OK`
- `NA0448_QSC_RNG_TEST_SEAM_AUTHORIZATION_OK`

Future validation should include queue/decision proof, exact scope guard,
link-check, leak-scan, overclaim scan, classifier, PR body preflight,
goal-lint, root cargo audit, nested qsc fuzz lock audit, inherited qsc
key-lifecycle and provider-error tests, qsc `send_commit`, refimpl `pqkem768`,
formal checks, qsc adversarial syntax checks, and public-safety before and
after merge.

## Public claim/external review/website boundary

RNG failure scope authorization is internal governance evidence only.

- No production-readiness claim is made.
- No public-readiness claim is made.
- No public-internet-readiness claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No RNG-failure-complete claim is made.
- No bug-free claim is made.
- No vulnerability-free claim is made.
- No perfect-crypto claim is made.
- No external-review-complete claim is made.
- No public technical paper content is created.
- No README, START_HERE, public docs, or website update is made.
- Cargo audit green is dependency-health evidence only.
- Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

`RNG_FAILURE_SCOPE_QSC_RUNTIME_PROPAGATION_NEXT`:

Rejected for immediate successor because the first truthful step is to
authorize a bounded qsc seam strategy and decide exact test/runtime boundaries.
Runtime error propagation may still be future-gated if NA-0448 finds that an
implementation contract change is required.

`RNG_FAILURE_SCOPE_FORMAL_MODEL_NEXT`:

Rejected as the immediate successor because current formal models do not define
RNG health semantics, and model extension should follow scoped qsc behavior.

`RNG_FAILURE_SCOPE_DOCUMENTATION_ONLY`:

Rejected because documentation-only evidence would preserve caveats but would
not improve executable fail-closed RNG failure evidence.

`RNG_FAILURE_SCOPE_SPLIT_TRIAGE_NEXT`:

Rejected because qsc/refimpl/qshield/formal scopes are separable enough for
qsc test-seam authorization to be the next bounded lane.

`RNG_FAILURE_SCOPE_BACKLOG`:

Rejected because F-0441-03 is the next medium evidence gap after NA-0446 and is
safe to scope without implementation mutation.

## Backup-impact statement

No backup, restore, or local-ops mutation is authorized or performed. Codex did
not run backup or restore. Codex did not mutate qsl-backup, backup status
files, backup plan files, rollback subtree paths, timers, fstab, source lists,
retention, backup scripts, or backup tree paths.

The qsl-backup checksum matched
`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`, and the
Codex ops source-list inclusion count was exactly one.

## Next recommendation

Open and merge the NA-0447 evidence PR after validation and public-safety.
After merge and post-merge public-safety success, close out NA-0447 and restore
exactly one READY successor:

`NA-0448 -- QSL qsc RNG Failure Test Seam Authorization Plan`

NA-0448 should decide whether a qsc test seam/fake RNG strategy is accepted,
rejected, or refined. It must not implement runtime, crypto, dependency,
workflow, executable test, fuzz, vector, formal, public-claim, service, or
backup changes unless a later exact directive authorizes them.
