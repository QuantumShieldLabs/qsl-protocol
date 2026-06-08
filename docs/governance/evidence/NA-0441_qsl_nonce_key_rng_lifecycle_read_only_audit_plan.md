Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0441 QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0441 completed a read-only audit of nonce, key, and RNG lifecycle evidence
across qsc, the reference implementation, qshield demo/runtime surfaces,
formal models, vectors, executable tests, and recent governance evidence.

Selected classification:

`NONCE_KEY_RNG_EVIDENCE_GAPS_FOUND`

No active blocker and no high runtime risk was identified from this read-only
review. Meaningful evidence gaps remain around comprehensive secret-material
zeroization/wipe expectations, RNG failure modeling, and demo-boundary
treatment of deterministic qshield establishment material. Those gaps should
be triaged before moving to a broader implementation lane.

Selected successor:

`NA-0442 -- QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Plan`

No implementation mutation is authorized by NA-0441. No runtime, crypto,
dependency, Cargo, lockfile, workflow, executable-test, fuzz-target, vector,
formal-model, public-surface, service, qwork, backup, restore, qsl-backup, or
local-ops mutation is authorized or performed.

## Live NA-0441 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0441 -- QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`

Status: READY.

Allowed NA-0441 mutation paths:

- `docs/governance/evidence/NA-0441_qsl_nonce_key_rng_lifecycle_read_only_audit_plan.md`
- `tests/NA-0441_qsl_nonce_key_rng_lifecycle_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only audit surfaces included qsc source/tests/fuzz metadata, the
reference implementation source/tests, qshield demo/runtime source/tests,
formal models, Suite-2 input vectors, recent provider-error governance
evidence, Cargo manifests and locks, CI scripts, and workflow metadata.

Forbidden current-lane mutation scope includes runtime code, crypto code,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal model files, qsl-server, qsl-attachments, qshield
runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup status, backup plan, rollback subtree, backup tree, and
backup/local-ops state.

Acceptance criteria:

- nonce/key/RNG lifecycle surfaces are inventoried read-only;
- findings are classified with evidence and public-claim implications;
- provider-error caveats are preserved as background;
- exact NA-0442 successor is selected;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- provider-error test and qsc adversarial script evidence remain healthy;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions included missing or inconsistent qwork proof, PR #1150 not
merged, queue drift from READY NA-0441, D-0868 absence, D-0869 preexistence,
audit failures, missing inherited evidence, unsafe classification, unsafe
successor selection, forbidden mutation, backup/restore execution,
qsl-backup/source-list regression, public overclaim, or more than one READY.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0441/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0441/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0441`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0441/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0441`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status, and
clean-state fields.

Initial live `HEAD` and `origin/main` matched the qwork proof. After
`git fetch --all --prune`, `origin/main` still matched the proof and equaled
the PR #1150 merge commit prefix `127a5542d04c`. PR #1150 was verified MERGED.

Proof root:

`/srv/qbuild/tmp/NA0441_nonce_key_rng_lifecycle_audit_20260608T034431Z`

## Provider-error chain inheritance

NA-0441 consumed provider-error evidence as background only.

NA-0436 inheritance:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs` provides a
  deterministic `pq_decap_failed` no-mutation test.
- The test corrupts a test-local pending KEM secret after a valid B1 response,
  observes `pq_decap_failed`, and verifies no session or pending/vault mutation
  occurs on reject.
- Local NA-0441 preflight reran the test and observed the NA-0436 markers,
  including `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`.

NA-0437 inheritance:

- `pq_encap_failed` remains defensive-branch documentation only.
- No executable coverage claim is made for `pq_encap_failed`.

NA-0439 inheritance:

- `scripts/ci/qsc_adversarial.sh` still contains
  `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`.
- The provider-error no-mutation test command remains present before the
  cargo-fuzz phases.

NA-0440 inheritance:

- Formal/model provider-error evidence remains supporting-only.
- Existing models do not directly prove qsc provider-error implementation
  behavior, qsc pending store state, qsc session store state, or exact
  `StdCrypto` encap/decap semantics.

## Applicable Stewardship Review

### Crypto / Protocol Steward

Nonce/key/RNG lifecycle is a high-sensitivity audit domain. This evidence
distinguishes lifecycle evidence from correctness proof. Secret-material and
side-channel caveats remain open unless an exact future lane closes them with
implementation and validation evidence.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
The provider-error no-mutation test passed. The qsc adversarial script retained
the NA-0439 marker and command. Public-safety was required and green on current
main. Cargo audit green is dependency-health evidence only.

### Public Claims / External Review Steward

NA-0441 makes no crypto-complete claim. It makes no side-channel-free claim. It
makes no vulnerability-free claim. It makes no bug-free claim. It makes no
perfect-crypto claim. It makes no public-readiness claim. It makes no
production-readiness claim. It makes no external-review-complete claim.

### Product / Demo / Service Boundary Steward

qsc/refimpl/qshield lifecycle evidence is internal engineering evidence.
qshield-cli deterministic establishment is demo-boundary evidence, not a
service-readiness proof. NA-0441 makes no qsl-server readiness claim, no
qsl-attachments readiness claim, no qshield runtime readiness claim, no website
readiness claim, and no public-service readiness claim.

### Local Ops / Backup / Restore Steward

No backup, restore, or local-ops mutation is authorized or performed.
qsl-backup proof remains boundary evidence only. The qsl-backup checksum matched
the expected boundary value, and the Codex ops source-list inclusion count was
exactly one.

Level 1 stewardship is active in this evidence lane. Level 2 and Level 3 remain
future-gated. Stewards remain advisory only: no separate Directors, no
independent READY promotion, no independent merge authority, and Lead Director
final authority is preserved.

## Surface inventory

| Surface group | File paths | Lifecycle role | Evidence type | Follow-up suggested |
|---|---|---|---|---|
| qsc handshake | `qsl/qsl-client/qsc/src/handshake/mod.rs` | Generates 16-byte handshake session IDs, stores pending KEM/DH/signature material, checks KEM/signature/public-key/ciphertext lengths, binds session ID and suite context into KDF labels, gates session insertion on successful transcript and provider paths | Code | Yes, for pending-secret wipe/zeroization expectations |
| qsc vault | `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/tests/vault.rs` | Encrypts secrets at rest with random vault nonces/salt, zeroizes some passphrase/key buffers, rejects invalid unlock/provider profiles without mutation | Code and tests | Yes, for comprehensive secret value zeroization policy |
| qsc session state | `qsl/qsl-client/qsc/src/protocol_state/mod.rs`; `qsl/qsl-client/qsc/tests/session_state_at_rest.rs` | Creates a vault-stored session encryption key, encrypts session snapshots with random 12-byte nonces and peer-bound AAD, rejects tampered blobs without mutation | Code and tests | Yes, for plaintext snapshot/session-key wipe expectations |
| qsc ratchet/send durability | `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs`; qsc send tests | Covers retry replay from outbox and abort burn behavior that avoids recomputed ciphertext reuse after abort | Tests | No immediate blocker; keep as supporting nonce evidence |
| qsc attachments | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Generates attachment IDs, content encryption keys, and nonce prefixes from `OsRng`; validates encoded context sizes | Code | Backlog only; exact attachment lifecycle audit would need separate scope |
| refimpl crypto provider | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | Uses `OsRng` for ML-KEM, ML-DSA seed, X25519 keypair, KEM encap, and random 12-byte nonces; X25519 private keys implement zeroize traits | Code and unit tests | Yes, for broader PQ/shared-secret zeroization evidence |
| refimpl qsp / Suite-2 | `tools/refimpl/quantumshield_refimpl/src/qsp/*.rs`; `tools/refimpl/quantumshield_refimpl/src/suite2/*.rs`; refimpl tests | Enforces session/key lengths, deterministic nonce derivation, replay/no-mutation rejects, chainkey readiness, transcript and Suite-2 binding | Code, tests, vectors | Yes, for vector/formal mapping of lifecycle properties |
| qshield demo/runtime | `apps/qshield-cli/src/**`; `apps/qshield-cli/tests/**` | Uses deterministic demo establishment material, persists local demo state, passes session material to refimpl actor, includes secret-safe output tests in metadata lanes | Code and tests | Yes, for claim-boundary and storage triage |
| formal models | `formal/*.py`; `formal/README.md` | Model bounded SCKA, negotiation, and qsc suite-id fail-closed/no-mutation behavior | Formal supporting evidence | Yes, because nonce uniqueness, RNG failure, and zeroization are not directly modeled |
| vectors | `inputs/suite2/**` | Capture Suite-2 establishment, transcript, KDF, crash/restart, replay, SCKA, KEM, and parse behavior | Vectors | Yes, for mapping vectors to lifecycle findings |
| governance evidence | recent NA-0436 through NA-0440 evidence docs | Preserves provider-error evidence, caveats, and stewardship boundaries | Governance evidence | No provider-error reopening unless lifecycle triage requires it |

## Nonce lifecycle review

Nonce/session-id generation evidence:

- qsc handshake session IDs are 16 bytes generated from `OsRng`.
- qsc vault envelopes use random 12-byte AEAD nonces and 16-byte salts.
- qsc session blobs use random 12-byte AEAD nonces with peer-bound AAD.
- qsc attachments generate attachment IDs, content encryption keys, and nonce
  prefixes from `OsRng`.
- refimpl `StdRng` uses `OsRng` for random 12-byte nonces.
- Suite-2 header/body nonces are deterministic derivations bound to session ID,
  DH public key, labels, and counters.

Nonce validation and replay evidence:

- qsc tests assert vault/session nonce lengths in encrypted envelopes.
- `handshake_seed_env_does_not_steer_session_id` verifies a test seed does not
  make production handshake session IDs reproducible.
- ratchet durability tests verify retry replay uses the saved ciphertext and
  abort burns state before a new send, avoiding ciphertext reuse after abort.
- Suite-2 and qsc suite-id vectors/tests cover replay and no-mutation rejects.

Classification:

- qsc and refimpl nonce generation/length evidence is strong implementation
  evidence for the reviewed surfaces.
- Formal evidence is supporting-only for nonce lifecycle because current models
  do not directly model nonce uniqueness or RNG failure.
- Finding priority is LOW / EVIDENCE_INCOMPLETE, not an active blocker.

## Key lifecycle review

Key generation/loading evidence:

- qsc identity stores KEM and ML-DSA signing secrets in the vault as secret
  entries and keeps public identity records separate.
- qsc session storage creates or loads a 32-byte session-store encryption key
  from the vault and fails closed when the vault is unavailable outside explicit
  test fallback.
- refimpl `StdCrypto` uses `OsRng` for ML-KEM keypairs, ML-DSA seed material,
  X25519 keypairs, and KEM encapsulation randomness.
- refimpl provider helpers expose runtime public key, ciphertext, signature,
  and secret-key lengths, and tests assert provider key sizes.

Key parsing/fail-closed evidence:

- qsc handshake decode enforces KEM public key, KEM ciphertext, signature public
  key, and signature length checks before accepting messages.
- qsc identity secret loads fail closed on missing or locked vault state.
- qsc session decrypt enforces magic, version, nonce length, ciphertext length,
  AAD, and AEAD authentication before restoring session state.
- refimpl establishment rejects invalid input lengths and unauthenticated
  inputs.
- refimpl Suite-2 send/receive rejects unset chainkeys and invalid ciphertexts
  without committing state.

Secret exposure / persistence evidence:

- qsc vault and session-state tests check encrypted-at-rest storage and secret
  redaction in outputs.
- qsc vault zeroizes passphrase buffers and runtime vault keys in selected
  paths.
- refimpl X25519 private keys implement `Zeroize` and `ZeroizeOnDrop` under the
  `stdcrypto` feature.

Evidence gap:

- KEM secrets, ML-DSA signing secrets, shared secrets, pending handshake
  records, serialized session snapshots, and qshield demo state are represented
  as `Vec`, array, or `String` values without a single documented lifecycle
  policy or comprehensive zeroization test coverage.
- qshield-cli persists deterministic demo `pq_init_ss_hex` and related session
  material in local state and relay-establish records. That is demo-boundary
  behavior and must not be turned into a public assurance claim.

Classification:

- MEDIUM / EVIDENCE_INCOMPLETE for comprehensive key/secret material lifecycle
  evidence.
- CLAIM_BOUNDARY_ONLY / BACKLOG_CANDIDATE for qshield deterministic demo state.
- No active high runtime risk was proven by this read-only audit.

## RNG lifecycle review

Runtime RNG evidence:

- qsc handshake, vault, session storage, contacts, attachments, and selected TUI
  paths use `OsRng` for cryptographic or token randomness.
- refimpl `StdCrypto` uses `OsRng` for ML-KEM keypair generation, ML-DSA seed
  generation, X25519 keypairs, KEM encapsulation, and random 12-byte nonces.

Deterministic/test evidence:

- qsc `QSC_QSP_SEED` fallback is gated by `QSC_ALLOW_SEED_FALLBACK` and is used
  in tests.
- qsc `QSC_HANDSHAKE_SEED` does not steer production handshake session IDs.
- refimpl fixed RNGs appear in unit tests and vector contexts.
- qshield-cli demo establishment derives deterministic material by design and
  labels that behavior as demo-only.

Evidence gap:

- `OsRng.fill_bytes` sites do not expose an injectable runtime RNG failure seam,
  so RNG failure behavior is not directly modeled or tested as a provider-error
  path.
- Existing formal models are deterministic state models and do not represent
  runtime RNG source health.

Classification:

- MEDIUM / EVIDENCE_INCOMPLETE for RNG failure and lifecycle modeling evidence.
- No high risk was identified because reviewed runtime cryptographic randomness
  sites use OS randomness or provider randomness, and deterministic RNGs are
  confined to tests/demo/vector contexts.

## Transcript / key schedule / session binding review

qsc evidence:

- qsc handshake binds session ID and suite context into `QSC.HS.PQ`,
  `QSC.HS.DHINIT`, confirm key, and confirm MAC derivations.
- qsc transcript MAC/hash covers A1 and B1 material.
- qsc suite-id parameter-block parsing rejects malformed, duplicate,
  downgraded, unknown critical, stripped, and mismatched context cases.
- qsc session insertion is gated on successful provider operations,
  transcript checks, identity checks, and session-state construction.
- The inherited `pq_decap_failed` test proves the decap reject does not create
  a session or mutate pending state; `pq_encap_failed` remains defensive branch
  documentation only.

refimpl evidence:

- Suite-2 establishment enforces session ID, DH, and PQ shared-secret lengths
  and derives root/header/chain keys from authenticated establishment inputs.
- Suite-2 header and body nonces bind labels, session ID, DH public key, and
  message counters.
- Suite-2 send/receive code stages state in cloned/new state and commits only
  on successful validation.

Classification:

- COVERED for bounded implementation/test evidence on transcript and session
  binding in the reviewed qsc and refimpl surfaces.
- SUPPORTING_ONLY for formal/vector evidence where it models abstract
  fail-closed behavior but not all runtime storage and provider details.

## Formal / vector / fuzz / test coverage review

Directly tested lifecycle properties:

- qsc provider-error `pq_decap_failed` no-mutation behavior.
- qsc handshake random session IDs are not driven by a test seed.
- qsc vault initialization/unlock/provider rejects are fail-closed and
  redacted.
- qsc session blobs are encrypted, tamper rejects do not mutate, and migration
  is idempotent.
- qsc ratchet retry/abort behavior avoids recomputed ciphertext reuse after
  abort.
- refimpl establishment, Suite-2 negotiation, replay, malformed input,
  chainkey, and receive reject paths are deterministic and no-mutation bounded.

Modeled properties:

- SCKA bounded replay/no-mutation properties.
- Suite-2 negotiation downgrade/reject no-mutation properties.
- qsc suite-id deterministic reject/no accepted-state mutation properties.

Vector-backed properties:

- Suite-2 establishment, transcript, KDF, SCKA, KEM, crash/restart, replay, and
  parse vectors exist under `inputs/suite2/**`.

Supporting-only or gap areas:

- No current formal model directly represents runtime RNG health, nonce
  uniqueness probability, vault/session encrypted-at-rest details, pending
  secret wipe semantics, or comprehensive zeroization coverage.
- qsc adversarial/fuzz evidence is strong for inherited provider-error and
  broader adversarial smoke, but it does not close the zeroization/RNG failure
  evidence gaps.

Classification:

- COVERED for bounded fail-closed/no-mutation properties already tested.
- SUPPORTING_ONLY for formal/model properties that do not map directly to
  runtime lifecycle implementation details.
- EVIDENCE_GAP for zeroization/wipe policy, RNG failure modeling, and
  qshield-demo claim boundary.

## Findings matrix and prioritization

| ID | Domain | Finding | Evidence | Classification | Risk | Suggested successor action | Exact future mutable paths if obvious | Public-claim implication | Goals affected |
|---|---|---|---|---|---|---|---|---|---|
| F-0441-01 | nonce | qsc/refimpl nonce and session-id generation use OS/provider randomness or deterministic derivation bound to session/DH/counter context, but no single cross-surface lifecycle proof covers uniqueness, replay, and storage together | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/src/protocol_state/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`; qsc nonce/replay tests | LOW / EVIDENCE_INCOMPLETE | Auditability gap, not active runtime blocker | Triage whether a nonce lifecycle policy or narrow evidence harness is needed | NA-0442 governance paths only unless later exact scope authorizes code/tests | No nonce-complete or crypto-complete claim | G1, G2, G4 |
| F-0441-02 | key | Key storage and parsing fail closed and encrypted-at-rest evidence is strong, but comprehensive zeroization/wipe expectations for KEM, PQ signature, shared secret, pending, and session snapshot material are incomplete | qsc identity/vault/session state code; refimpl `StdCrypto`; X25519 zeroize trait tests; qsc vault/session tests | MEDIUM / EVIDENCE_INCOMPLETE | Secret-material lifecycle evidence gap; no active exposure proven | Select findings triage to decide exact zeroization policy/test/documentation scope | NA-0442 governance paths first; possible future exact qsc/refimpl secret wrapper or test paths only after authorization | No side-channel-free, no perfect-crypto, no vulnerability-free claim | G1, G2, G3, G4 |
| F-0441-03 | RNG | Runtime cryptographic randomness sites use `OsRng` or provider randomness, while deterministic RNGs are test/demo/vector-bound; RNG failure behavior is not directly modeled or injectable | qsc `OsRng` sites; refimpl `StdCrypto`; qsc seed-fallback tests; fixed RNG test contexts | MEDIUM / EVIDENCE_INCOMPLETE | Provider/RNG failure evidence gap, not observed runtime failure | Triage whether RNG failure needs a provider seam, documentation caveat, or no-action record | NA-0442 governance paths first; future mutable implementation paths not authorized by NA-0441 | No RNG-complete or crypto-complete claim | G1, G2, G4 |
| F-0441-04 | transcript-binding | qsc and refimpl bind session IDs, suite context, transcript material, DH/PQ inputs, and counters before session acceptance; inherited provider-error caveats remain bounded | qsc handshake KDF/transcript code; qsc suite-id tests/vectors; refimpl Suite-2 establishment/ratchet code/tests | COVERED / SUPPORTING_ONLY | Residual risk is coverage mapping, not active blocker | Preserve as supporting evidence and consume in NA-0442 | None obvious for NA-0442 beyond governance paths | No transcript-complete or external-review-complete claim | G1, G2, G4 |
| F-0441-05 | coverage | Formal models and vectors support fail-closed/no-mutation discipline, but do not directly model nonce uniqueness, RNG failure, encrypted storage internals, or comprehensive zeroization | `formal/*.py`; `inputs/suite2/**`; recent governance evidence | EVIDENCE_GAP | Coverage overclaim risk | Record gap and triage highest leverage coverage/documentation option | NA-0442 governance paths first | No formal proof claim beyond bounded supporting evidence | G2, G4, G5 |
| F-0441-06 | claim-boundary | qshield-cli intentionally persists deterministic demo establishment material, including `pq_init_ss_hex`, in local demo state and relay records | `apps/qshield-cli/src/store.rs`; `apps/qshield-cli/src/commands/establish.rs`; qshield demo tests | CLAIM_BOUNDARY_ONLY / BACKLOG_CANDIDATE | Public/demo overclaim or misunderstood custody boundary | Keep qshield evidence internal/demo-boundary and triage documentation or future demo storage policy if prioritized | NA-0442 governance paths first; no qshield runtime path authorized by NA-0441 | No production-readiness, no public-readiness, no crypto-complete claim | G3, G4, G5 |

Prioritization result:

- No BLOCKER / ACTIVE_SECURITY_BLOCKER found.
- No HIGH / POTENTIAL_RUNTIME_RISK found.
- Meaningful MEDIUM evidence gaps exist for key/secret lifecycle and RNG
  failure modeling.
- Highest-priority successor is findings triage, not implementation.

## Authorization decision

Primary classification:

`NONCE_KEY_RNG_EVIDENCE_GAPS_FOUND`

D-0869 records:

- provider-error chain evidence consumed as background;
- findings matrix classification;
- highest-priority successor selected as NA-0442 findings triage;
- no implementation mutation in NA-0441;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation;
- no backup or restore;
- no public crypto-complete claim;
- no vulnerability-free or perfect-crypto claim;
- Level-1 steward template used;
- exactly one READY remains mandatory.

## Successor selection

Selected exact successor:

`NA-0442 -- QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Plan`

Reason:

- Key/secret material lifecycle and RNG failure modeling are meaningful
  evidence gaps.
- The exact future implementation shape is not yet authorized or low-risk
  enough to implement directly.
- Triage can decide whether the next action is a narrow policy/evidence lane,
  exact implementation authorization, or a documented no-action boundary.

Do not implement NA-0442 in NA-0441.

## Future path/scope bundle

Future allowed NA-0442 mutation paths for the selected findings-triage
successor:

- `docs/governance/evidence/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_plan.md`
- `tests/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0442 may inspect read-only:

- qsc lifecycle code/tests/fuzz metadata;
- reference implementation lifecycle code/tests;
- qshield demo/runtime source/tests;
- formal models;
- input vectors;
- recent governance evidence;
- Cargo manifests and locks;
- relevant scripts/workflows read-only.

Future forbidden unless a later exact scope authorizes it:

- runtime or crypto implementation changes;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs or website changes;
- qsl-server or qsl-attachments changes;
- backup/restore/qsl-backup changes;
- public claim expansion.

## Future validation/marker plan

Common NA-0442 markers:

- `NA0442_NONCE_KEY_RNG_FINDINGS_CONSUMED_OK`
- `NA0442_NEXT_SCOPE_SELECTED_OK`
- `NA0442_NO_RUNTIME_CHANGE_OK`
- `NA0442_NO_DEPENDENCY_CHANGE_OK`
- `NA0442_NO_WORKFLOW_CHANGE_OK`
- `NA0442_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0442_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0442_NO_SECRET_MATERIAL_OK`
- `NA0442_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0442_ONE_READY_INVARIANT_OK`
- `NA0442_LIFECYCLE_FINDINGS_TRIAGE_OK`

Expected NA-0442 validation:

- consume F-0441 findings;
- select exact successor from evidence;
- preserve no implementation mutation unless later exact scope authorizes it;
- preserve no public overclaim;
- keep root and nested dependency-health checks green;
- keep public-safety green before and after merge;
- keep exactly one READY item.

## Public claim/external review/website boundary

NA-0441 lifecycle findings are internal governance evidence only.

- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No public-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No bug-free claim is made.
- No vulnerability-free claim is made.
- No perfect-crypto claim is made.
- No public technical paper content is created.
- No README, START_HERE, public docs, or website update is made.
- Cargo audit green is dependency-health evidence only.
- Evidence gaps are called gaps, not completions.
- No secret-material lifecycle finding is turned into a public assurance claim.

## Rejected alternatives

Direct implementation now:

- Rejected because exact implementation scope for zeroization, RNG failure
  seams, or qshield demo storage changes is not yet selected.

No-action next-domain successor:

- Rejected because meaningful medium evidence gaps exist.

Blocker/high-risk successor:

- Rejected because this read-only audit did not identify an active blocker or
  high runtime risk.

Provider-error reopening:

- Rejected because provider-error evidence is consumed as background and no
  nonce/key/RNG finding requires reopening provider-error implementation.

Public documentation update:

- Rejected because NA-0441 is internal governance evidence and no public claim
  expansion is authorized.

## Backup-impact statement

NA-0441 did not run backup, restore, sudo, qwork, qstart, or qresume. NA-0441
did not mutate qsl-backup, backup status files, backup plan files, rollback
subtree paths, timers, fstab, source lists, backup scripts, or backup tree
paths. Backup evidence remains boundary evidence only, with no off-host backup
claim, no restore-proven claim, no disaster-recovery claim, no backup-complete
claim, and no source-of-truth index claim.

## Next recommendation

Close NA-0441 only after the evidence PR merges and post-merge public-safety is
green. Restore exactly:

`NA-0442 -- QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Plan`

NA-0442 should consume F-0441 findings, preserve provider-error caveats, and
choose the next exact follow-up without implementing runtime/crypto changes
unless a later directive authorizes precise scope.
