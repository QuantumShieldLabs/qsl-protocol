Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0450 QSL qsc RNG Failure Residual Surface Triage Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0450 consumes the NA-0447 through NA-0449 RNG failure evidence chain and
triages the qsc RNG failure surfaces that NA-0449 intentionally left residual.

Selected classification:

`RNG_RESIDUAL_TRIAGE_ROUTE_CONTACT_ATTACHMENT_NEXT`

Selected successor:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

Rationale:

- NA-0449 proved a bounded cfg-gated test-only seam for selected handshake
  session ID, vault, and session/protocol-state RNG failures.
- The remaining qsc-owned route/contact/attachment surfaces are cohesive:
  generated route tokens, contact route-token persistence, attachment IDs,
  attachment CEKs, and attachment nonce prefixes are all client-side
  randomness-dependent state or transfer-preparation boundaries.
- These surfaces appear suitable for a future authorization lane that decides
  exact test-only seam labels and no-partial-state assertions before any later
  implementation is allowed.
- Provider-dependent generation crosses into refimpl/provider contracts and
  should remain separate because it may require provider-boundary or runtime
  propagation authorization rather than a qsc-only test seam.
- Formal, fuzz, and vector work remain supporting/backlog until executable
  qsc residual behavior is scoped.

This directive performs no implementation mutation. It adds no tests, no test
seams, no runtime behavior, no crypto behavior, no dependency changes, no Cargo
manifest changes, no lockfile changes, no workflow changes, no fuzz target
changes, no vector changes, no formal model changes, no qsl-server changes, no
qsl-attachments changes, no qshield runtime changes, no qshield-cli changes, no
website changes, no README changes, and no START_HERE changes.

No RNG-failure-complete claim is made. Cargo audit green is dependency-health
evidence only.

## Live NA-0450 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0450 -- QSL qsc RNG Failure Residual Surface Triage Authorization Plan`

Status: READY.

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0450_qsl_qsc_rng_failure_residual_surface_triage_authorization_plan.md`
- `tests/NA-0450_qsl_qsc_rng_failure_residual_surface_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection included qsc source/tests/fuzz, refimpl source,
qshield-cli demo boundary files, formal models, inputs, governance evidence,
and relevant scripts/workflows.

Forbidden mutation scope:

- runtime source;
- crypto source;
- dependency metadata;
- Cargo manifests;
- lockfiles;
- workflows;
- executable tests;
- fuzz targets;
- vectors;
- formal models;
- qsl-server;
- qsl-attachments;
- qshield runtime;
- qshield-cli;
- website;
- public docs;
- README;
- START_HERE;
- qwork/qstart/qresume/qshell;
- qsl-backup;
- backup status or plan files;
- rollback subtree or `/backup/qsl`.

Acceptance criteria:

- residual qsc RNG failure surfaces are consumed;
- exact future scope is selected from evidence;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- inherited qsc RNG, zeroization, and provider-error tests remain green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- stale or inconsistent qwork proof;
- PR #1168 not merged;
- queue not READY NA-0450 at start;
- D-0886 absent or D-0887 present at start;
- root or nested cargo audit failure;
- residual surfaces cannot be safely classified;
- successor cannot be selected safely;
- public-safety red or missing;
- scope drift into forbidden paths;
- backup, restore, qwork, qstart, or qresume execution by Codex;
- public-claim expansion.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read proof files:

- `/srv/qbuild/work/NA-0450/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0450/.qwork/startup.qsl-protocol.json`

Proof values verified:

- `startup_result=OK`
- `lane=NA-0450`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0450/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0450`
- `requested_lane_status=READY`

Freshness proof:

- qwork proof head matched live `HEAD` before fetch.
- qwork proof `origin/main` matched live `origin/main` before fetch.
- Fetched `origin/main` remained `c3ba93d94564` and did not advance beyond
  the qwork proof.
- `origin/main` equals and descends from PR #1168 merge commit
  `c3ba93d94564`.

Proof root:

`/srv/qbuild/tmp/NA0450_qsc_rng_residual_surface_triage_20260609T150548Z`

## NA-0449 inheritance

NA-0449 implemented the D-0883-selected qsc RNG failure test-only seam for:

- handshake session ID RNG failure;
- selected vault RNG failure;
- selected session/protocol-state RNG failure.

Inherited passing proof:

- cfg-gated `rng_failure_behavior` test passed and emitted the NA-0449
  forced-failure markers.
- normal no-cfg `rng_failure_behavior` test passed and emitted the
  production-semantics marker.
- `key_lifecycle_zeroization` passed.
- `handshake_provider_error_no_mutation` passed.
- qsc adversarial script contains the inherited NA-0439 provider-error step.
- public-safety on current main is green.

Important boundary:

NA-0449 did not claim complete qsc RNG failure coverage. It did not prove
route/contact/attachment RNG failure behavior, provider-dependent RNG failure,
refimpl provider RNG failure, qshield-cli demo RNG behavior, formal/model RNG
behavior, fuzz RNG behavior, or vector RNG behavior.

Nuance recorded for route/default-route:

`qsl/qsl-client/qsc/src/vault/mod.rs` now has a cfg-gated label for default
route token generation, but the NA-0449 executable test forced only the vault
init salt label. Therefore route/default-route behavior remains residual for
scope triage and is not represented as completed route-token RNG failure
evidence.

## Applicable Stewardship Review

Level 1 stewardship review is active for this evidence lane.

Level 2 and Level 3 stewardship rollout remain future-gated.

No separate Directors are created. No steward has independent READY promotion
authority. No steward has independent merge authority. Lead Director final
authority is preserved.

### Crypto / Protocol Steward

Residual qsc RNG surfaces must be triaged without weakening the cfg seam or
normal production semantics. Route/contact/attachment RNG should be
distinguished from provider-dependent key and crypto generation. The
No RNG-failure-complete claim is made; the caveat is preserved.

Route/contact/attachment surfaces are qsc-owned and likely suitable for a
future bounded test-only seam authorization lane. Provider-dependent generation
is not the same shape because it crosses qsc/refimpl provider boundaries and may
require runtime or provider contract changes if actual RNG failure propagation
is required.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` is green.

Nested qsc fuzz lock `cargo audit --deny warnings --file
qsl/qsl-client/qsc/fuzz/Cargo.lock` is green.

Cfg RNG failure tests are green. Normal no-cfg RNG failure tests are green. qsc
key lifecycle and provider-error tests are green. qsc adversarial smoke is
green on current main. Public-safety is green on current main.

Cargo audit green is dependency-health evidence only.
No cargo audit output is used as public-readiness proof.
No cargo audit output is used as production-readiness proof.
No cargo audit output is used as public-internet readiness proof.
No cargo audit output is used as external-review proof.
No cargo audit output is used as crypto-complete proof.
No cargo audit output is used as RNG-failure-complete proof.
No cargo audit output is used as vulnerability-free proof.
No cargo audit output is used as bug-free proof.
No cargo audit output is used as perfect-crypto proof.
No cargo audit output is used as side-channel-free proof.

### Public Claims / External Review Steward

No RNG-failure-complete claim is made.

No crypto-complete claim is made.

No side-channel-free claim is made.

No vulnerability-free claim is made.

No bug-free claim is made.

No perfect-crypto claim is made.

No public-readiness claim is made.

No production-readiness claim is made.

No external-review-complete claim is made.

Residual triage is internal governance evidence only.

### Product / Demo / Service Boundary Steward

qshield-cli remains demo-local and out of scope for NA-0450 mutation.

qsl-server remains a service boundary and out of scope for NA-0450 mutation.

qsl-attachments remains a service boundary and out of scope for NA-0450
mutation.

No qshield, website, public-service, production, or public-internet readiness
claim is made.

### Local Ops / Backup / Restore Steward

No backup was run. No restore was run. No sudo was run.

qsl-backup proof is read-only boundary evidence only.

The qsl-backup script hash matched the required SHA, and the codex ops source
inclusion count was exactly one. qsl-backup, backup status files, backup plan
files, rollback paths, and `/backup/qsl` were not mutated.

## Residual qsc RNG surface inventory

| Surface | Path(s) | RNG role | Covered by NA-0449? | Existing API/test access | Likely failure behavior | Existing cfg seam reusable? | Future mutation category | Candidate future mutable paths | Risk / priority |
|---|---|---|---|---|---|---|---|---|---|
| Default route token during vault init | `qsl/qsl-client/qsc/src/vault/mod.rs` | Generates the default relay inbox route token stored in the vault payload | Partly source-labeled, but not proved by NA-0449 test | `vault init` path exists; forced route-token failure not directly tested | Should fail before vault payload/file commit if forced before payload construction | Yes, likely same cfg label pattern | Future test-only seam proof unless production RNG errors are made first-class | Future implementation candidate only if later authorized: `qsl/qsl-client/qsc/src/vault/mod.rs`, qsc RNG test path | Medium; high value because route token is persistent routing state |
| Contact route token generation | `qsl/qsl-client/qsc/src/contacts/mod.rs` | Generates route token when adding a contact without an explicit token | No | Contacts CLI/TUI paths exist; RNG failure cannot be forced now | Should fail before `contacts_entry_upsert` so no partial contact/device route-token state is written | Yes | Future test-only seam proof | Future implementation candidate only if later authorized: `qsl/qsl-client/qsc/src/contacts/mod.rs`, qsc RNG test path | Medium; high value because contact state is persistent |
| TUI account/contact bootstrap randomness | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`; `qsl/qsl-client/qsc/src/contacts/mod.rs` | Generates account verification seed and relay inbox route token during TUI init | No | TUI init path exists; no forced RNG failure access | Account verification seed failure may occur after earlier setup writes, so no-partial-state proof needs exact scoping | Partly; route token helper can share route seam, account seed may need separate label | Future authorization must decide test-only seam vs runtime transactional propagation | Future implementation candidate only if later authorized: `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`, `qsl/qsl-client/qsc/src/contacts/mod.rs`, qsc RNG test path | Medium; include in NA-0451 review, but maybe sub-scope/defer |
| Attachment ID generation | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Generates outbound attachment ID before staging | No | File send path exists; RNG failure cannot be forced now | Should fail before staging file, journal, descriptor, or send mutation | Yes | Future test-only seam proof | Future implementation candidate only if later authorized: `qsl/qsl-client/qsc/src/attachments/mod.rs`, qsc attachment RNG test path | Medium/high; high value because attachment state has staging/journal effects |
| Attachment CEK and nonce-prefix generation | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Generates content encryption key and nonce prefix before streaming encryption | No | File send path exists; RNG failure cannot be forced now | Should fail before staged ciphertext/journal/send mutation; may need cleanup proof if failure point follows ID creation | Yes | Future test-only seam proof; cleanup checks may be needed | Future implementation candidate only if later authorized: `qsl/qsl-client/qsc/src/attachments/mod.rs`, qsc attachment RNG test path | Medium/high; high value because CEK and nonce-prefix are encryption-critical |
| Provider-dependent qsc identity/key generation | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | Generates ML-KEM/ML-DSA identity keys, X25519 ephemeral keys, and KEM encapsulation randomness | No | Provider-error tests cover selected rejects, not RNG failure inside infallible provider helpers | Mixed: some provider APIs return `Result`; keypair helpers are infallible today | Not cleanly qsc-only | Provider/refimpl boundary authorization or runtime propagation if broadened | Future provider-boundary candidate only if later authorized: qsc identity/handshake plus refimpl provider paths | Medium/high; separate from route/contact/attachment |
| Deterministic qsc relay/meta simulation RNG | `qsl/qsl-client/qsc/src/relay/mod.rs`; `qsl/qsl-client/qsc/src/main.rs` | Seeded deterministic relay decisions and metadata padding selection | No, and not OS RNG failure | Existing deterministic tests can control values | Not an OS/provider RNG failure surface | Not needed for immediate RNG failure seam | Documentation/supporting only unless later scope says otherwise | No immediate source candidate | Low for RNG failure; claim-boundary only |

## Route / contact / attachment RNG triage

High-value follow-up surfaces:

- default route token generation during vault init;
- contact route token generation during contact add;
- TUI relay inbox route token generation during local bootstrap;
- attachment ID generation before staging;
- attachment CEK and nonce-prefix generation before encryption and staging.

These can be handled in one future authorization lane because they share the
same qsc-owned shape: generated random material is used to prepare persistent
client state, transfer metadata, or routing state, and a future test should
prove forced failure aborts before partial route/contact/attachment mutation.

They likely require future production source cfg seams if executable proof is
later authorized. NA-0450 does not authorize those source changes. The future
NA-0451 authorization lane should decide exact labels, exact no-partial-state
assertions, and whether a later implementation lane may reuse the NA-0449
`qsc_rng_failure_test_seam` pattern or should use a narrower route/contact/
attachment-specific seam.

No-partial-state proof appears straightforward for contact route-token
generation and attachment ID/CEK/prefix generation because randomness is drawn
before the main contact upsert or attachment staging/journal/send effects.

No-partial-state proof is less obvious for the TUI account verification seed
because that seed is generated after earlier local setup writes. NA-0451 should
explicitly classify that sub-surface as either in-scope with transactional
requirements, in-scope only for evidence of explicit failure, or deferred as an
account-bootstrap residual.

Classification:

`QSC_ROUTE_CONTACT_ATTACHMENT_RNG_SCOPE_NEXT`

## Provider-dependent RNG triage

qsc provider-dependent RNG surfaces are only partly covered by existing
provider-error tests.

Covered today:

- selected provider reject/no-mutation behavior such as decapsulation failure;
- key lifecycle and provider-error state preservation around bounded paths.

Not covered today:

- ML-KEM keypair RNG failure;
- ML-DSA seed/keypair RNG failure;
- X25519 ephemeral keypair RNG failure;
- KEM encapsulation RNG failure as an RNG-source failure rather than a provider
  reject;
- refimpl `StdRng::random_nonce12` failure as a first-class error.

RNG failure inside provider/refimpl is not cleanly controllable without
changing provider/refimpl contracts or adding exact provider-boundary test
seams. That belongs in a provider-boundary authorization lane, not the next
route/contact/attachment qsc residual lane.

Provider-dependent RNG is not selected as the immediate successor because
route/contact/attachment surfaces are qsc-owned, have clearer persistent-state
no-mutation questions, and can be scoped without first changing refimpl traits.

Classification:

`QSC_PROVIDER_DEPENDENT_RNG_BACKLOG`

## Formal / fuzz / vector RNG residual triage

Current formal models cover deterministic fail-closed state-machine behavior
for bounded SCKA, Suite-2 negotiation, and qsc suite-id admission. They do not
model RNG source health, entropy failure, provider RNG failure, nonce uniqueness
failure, or injectable RNG failure semantics.

Current qsc fuzz targets exercise parser and boundary behavior over input
bytes. They do not exercise RNG failure behavior.

Current vectors are deterministic conformance inputs. They do not encode
runtime RNG failure behavior.

Formal, fuzz, and vector work should not precede the qsc residual route/contact/
attachment authorization lane. They should follow once exact executable
behavior is scoped, or remain supporting/backlog if no exact behavior contract
is selected later.

Classifications:

- `RNG_RESIDUAL_FORMAL_BACKLOG`
- `RNG_RESIDUAL_FUZZ_BACKLOG`
- `RNG_RESIDUAL_VECTOR_BACKLOG`

## Triage matrix

| Residual surface | Path(s) | RNG role | Covered by NA-0449? | Existing API enough? | Existing cfg seam reusable? | Future mutable paths | Recommended lane type | Priority | Risk | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Route/default-route/relay token RNG residual | `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/src/contacts/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | Default route token, relay inbox route token, contact route token | No, except source label exists for default route token without NA-0449 executable proof | No, failure not forceable through public APIs | Yes, likely reusable after exact authorization | NA-0451 governance paths only; later implementation candidate paths are qsc route/contact/TUI source plus qsc RNG test path if later authorized | qsc route/contact/attachment authorization | High | Medium persistent-state risk | No RNG-failure-complete claim is made | G2, G4, G5 |
| Contact RNG residual | `qsl/qsl-client/qsc/src/contacts/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | Contact route-token generation and account verification seed | No | No | Yes for route token; account seed requires exact review | NA-0451 governance paths only; later implementation candidate paths are contacts/TUI source plus qsc RNG test path if later authorized | qsc route/contact/attachment authorization | High | Medium; account seed may need runtime/transactional scoping | No public-readiness claim is made | G2, G4, G5 |
| Attachment RNG residual | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Attachment ID, CEK, nonce prefix | No | No | Yes, likely reusable after exact authorization | NA-0451 governance paths only; later implementation candidate paths are attachments source plus qsc attachment RNG test path if later authorized | qsc route/contact/attachment authorization | High | Medium/high staging and encryption-material risk | No crypto-complete claim is made | G1, G2, G4, G5 |
| qsc provider-dependent RNG residual | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | qsc identity, X25519, KEM/signature provider generation | No | No | Not cleanly qsc-only | Future provider-boundary governance path only if later selected | provider-boundary backlog | Medium | Medium/high semantic and trait-contract risk | No vulnerability-free claim is made | G1, G2, G4 |
| refimpl provider RNG residual | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | ML-KEM, ML-DSA, X25519, `Rng12` randomness | No | No | No; may need provider contracts | Future provider-boundary governance path only if later selected | provider-boundary backlog | Medium | Medium/high because trait return shapes may change | No perfect-crypto claim is made | G1, G2, G4 |
| qshield-cli demo RNG residual | `apps/qshield-cli/**` | Demo-local token/session randomness and `/dev/urandom` fallback boundary | No | Not qsc scope | No immediate | No qshield-cli path authorized by NA-0450 | demo claim-boundary backlog | Low/medium | Claim-boundary risk | No production-readiness claim is made | G3, G4, G5 |
| formal/model RNG residual | `formal/**` | Deterministic models do not model RNG health or failure | No | No | No | No formal path authorized by NA-0450 | formal backlog | Medium after behavior is scoped | Evidence overclaim risk | No formal-complete claim is made | G2, G4 |
| fuzz/vector RNG residual | `qsl/qsl-client/qsc/fuzz/**`; `inputs/**` | Fuzz inputs and vectors do not model RNG failure | No | No | No | No fuzz/vector path authorized by NA-0450 | fuzz/vector backlog | Low until behavior API exists | Coverage overclaim risk | No fuzz-complete or vector-complete claim is made | G4 |

## Authorization decision

Primary classification:

`RNG_RESIDUAL_TRIAGE_ROUTE_CONTACT_ATTACHMENT_NEXT`

Selected highest-priority successor:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

This directive does not authorize implementation. It does not authorize runtime
changes, crypto changes, dependency changes, Cargo changes, lockfile changes,
workflow changes, executable test changes, fuzz target changes, vector changes,
formal model changes, qsl-server changes, qsl-attachments changes, qshield
runtime changes, qshield-cli changes, website changes, public-doc changes,
README changes, START_HERE changes, backup changes, restore changes, qsl-backup
changes, or public-claim expansion.

Exactly one READY successor remains mandatory.

## Successor selection

Selected NA-0451:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

NA-0451 should be authorization-only. It should decide exact future executable
scope for route/contact/attachment RNG failure behavior, including whether the
future implementation should be combined or split by surface. It should not
implement NA-0451 behavior.

## Future path/scope bundle

Future NA-0451 allowed mutation paths:

- `docs/governance/evidence/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_plan.md`
- `tests/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0451 may inspect read-only:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts/workflows read-only.

Future implementation candidate paths, not authorized by NA-0450 and not
authorized by NA-0451 unless a later exact implementation directive says so:

- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- qsc RNG failure test source path selected by a later directive.

Future forbidden unless exact later scope authorizes:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs or website changes;
- README or START_HERE changes;
- qsl-server or qsl-attachments changes;
- qshield runtime or qshield-cli changes;
- backup, restore, qsl-backup, status, plan, rollback, or `/backup/qsl`
  mutation;
- public claims.

## Future validation/marker plan

Common future NA-0451 markers:

- `NA0451_RNG_RESIDUAL_TRIAGE_CONSUMED_OK`
- `NA0451_NEXT_SCOPE_SELECTED_OK`
- `NA0451_NO_RUNTIME_CHANGE_OK`
- `NA0451_NO_DEPENDENCY_CHANGE_OK`
- `NA0451_NO_WORKFLOW_CHANGE_OK`
- `NA0451_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0451_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0451_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0451_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0451_ONE_READY_INVARIANT_OK`
- `NA0451_ROUTE_CONTACT_ATTACHMENT_SCOPE_AUTHORIZED_OK`

Recommended NA-0451 evidence questions:

- exact labels for default route token, contact route token, attachment ID,
  attachment CEK, and attachment nonce-prefix failures;
- whether TUI account verification seed belongs in the same implementation
  lane or should be deferred;
- whether future tests can prove no partial contact, route, attachment,
  staging, descriptor, journal, or send state is written after forced failure;
- whether any sub-surface needs runtime propagation authorization rather than a
  test-only seam.

## Public claim/external review/website boundary

Residual RNG triage is internal governance evidence only.

Residual RNG triage is not production readiness.

Residual RNG triage is not public-internet readiness.

Residual RNG triage is not crypto-complete proof.

Residual RNG triage is not side-channel-free proof.

Residual RNG triage is not RNG-failure-complete proof.

Residual RNG triage is not bug-free proof.

Residual RNG triage is not vulnerability-free proof.

Residual RNG triage is not perfect-crypto proof.

Residual RNG triage is not public technical paper content.

No README update is made.

No START_HERE update is made.

No public docs update is made.

No website update is made.

No public-readiness or public-security claim is made.

Cargo audit green is dependency-health evidence only.

Future tests, if later authorized, must be described as bounded evidence only.

## Rejected alternatives

`RNG_RESIDUAL_TRIAGE_PROVIDER_BOUNDARY_NEXT`

Rejected as the immediate successor because provider-dependent RNG failure
crosses qsc/refimpl contracts and may require runtime/provider API decisions.
It remains important but less cohesive than qsc-owned route/contact/attachment
residuals for the next authorization step.

`RNG_RESIDUAL_TRIAGE_ATTACHMENT_NEXT`

Rejected because attachment RNG is high value, but the route/contact surfaces
are closely related qsc-owned state-write boundaries and can be triaged in the
same authorization lane before implementation is allowed.

`RNG_RESIDUAL_TRIAGE_FORMAL_OR_FUZZ_NEXT`

Rejected because formal, fuzz, and vector evidence should follow exact
behavioral scoping. Current formal/fuzz/vector artifacts do not define RNG
source failure semantics.

`RNG_RESIDUAL_TRIAGE_DOCUMENTATION_ONLY`

Rejected because exact qsc route/contact/attachment failure behavior remains a
real bounded evidence gap, not merely a documentation wording gap.

`RNG_RESIDUAL_TRIAGE_BACKLOG_NO_ACTION`

Rejected because NA-0449 intentionally left route/contact/attachment residuals
and the next qsc-owned authorization step is coherent.

## Backup-impact statement

No backup was run.

No restore was run.

No sudo was run.

qsl-backup was not mutated.

Backup status files were not mutated.

Backup plan files were not mutated.

Rollback subtree paths were not mutated.

`/backup/qsl` was not mutated.

The qsl-backup SHA proof and source inclusion count are read-only boundary
evidence only.

## Next recommendation

After this PR merges and post-merge public-safety is green, close out NA-0450
and restore:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

NA-0451 should remain governance/authorization-only and should not implement
route/contact/attachment RNG failure behavior.
