Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0456 QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0456 consumes NA-0455 and classifies the qsc-side provider RNG failure
no-mutation scope.

Primary classification:

`QSC_PROVIDER_RNG_NO_MUTATION_REQUIRES_FAKE_SEAM_STRATEGY`

Selected successor:

`NA-0457 -- QSL qsc Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

Reason: the qsc target inventory is exact enough to name the relevant call
sites, but it is not implementation-ready. The current qsc KEM keypair,
signature keypair, identity bootstrap, and X25519/ephemeral generation paths
are infallible from qsc's perspective. Existing qsc tests prove local RNG
failure seams and generic provider-error no-mutation behavior, including
`pq_decap_failed`, but they do not force concrete provider RNG failure at the
provider boundary. A future qsc fake/test-seam strategy lane must choose the
least invasive cfg-only/fake/injection shape before any implementation lane.

NA-0456 authorizes no implementation mutation. It changes no runtime behavior,
crypto behavior, dependencies, Cargo manifests, lockfiles, workflows,
executable tests, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
Cargo audit green remains dependency-health evidence only.

Required markers recorded by this evidence:

- `NA0456_QSC_PROVIDER_RNG_SCOPE_CONSUMED_OK`
- `NA0456_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0456_NA0455_INHERITANCE_CONSUMED_OK`
- `NA0456_QSC_TARGET_INVENTORY_OK`
- `NA0456_QSC_KEM_REVIEW_OK`
- `NA0456_QSC_SIGNATURE_IDENTITY_REVIEW_OK`
- `NA0456_QSC_HANDSHAKE_IDENTITY_SPLIT_DECISION_OK`
- `NA0456_QSC_AUTHORIZATION_MATRIX_OK`
- `NA0456_REQUIRES_FAKE_SEAM_STRATEGY_SELECTED_OK`
- `NA0456_SUCCESSOR_NA0457_SELECTED_OK`
- `NA0456_NO_RUNTIME_CHANGE_OK`
- `NA0456_NO_CRYPTO_CHANGE_OK`
- `NA0456_NO_DEPENDENCY_CHANGE_OK`
- `NA0456_NO_WORKFLOW_CHANGE_OK`
- `NA0456_NO_TEST_IMPLEMENTATION_OK`
- `NA0456_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0456_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0456_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0456_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0456_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0456_ONE_READY_INVARIANT_OK`

## Live NA-0456 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0456.
- NA-0455 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0898.
- D-0897 exists once.
- D-0898 exists once.
- D-0899 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0456 mutation paths are exactly:

- `docs/governance/evidence/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_plan.md`
- `tests/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback, and backup tree paths.

Acceptance criteria:

- qwork proof files are verified without rerunning qwork;
- NA-0455 qsc-first inheritance is consumed;
- exact qsc target call sites are inventoried;
- KEM, signature/identity, handshake, and X25519-related qsc surfaces are
  classified as combined, split, documentation-only, or fake/seam strategy;
- future NA-0457 successor is selected;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, missing
public-safety, missing D-0898, D-0899 already present, failed root or nested
audit, unconsumable NA-0455 inheritance, unclassifiable qsc provider RNG
no-mutation scope, unsafe successor selection, backup boundary regression, or
any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0456/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0456/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0456`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0456/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0456`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`e9671c244a5f`. Fetch did not advance `origin/main`.

PR #1180 was verified merged at `e9671c244a5f`.

Proof root:

`/srv/qbuild/tmp/NA0456_qsc_provider_rng_no_mutation_scope_20260610T174046Z`

## NA-0455 inheritance

NA-0455 / D-0897 selected:

`PROVIDER_RNG_SPLIT_QSC_FIRST`

Inherited facts:

- qsc owns visible no-mutation call sites in `qsl/qsl-client/qsc/src/handshake/mod.rs`
  and `qsl/qsl-client/qsc/src/identity/mod.rs`.
- Existing qsc cfg RNG seams prove qsc-local RNG failures only.
- Existing qsc provider-error no-mutation proof covers generic
  `pq_decap_failed`, not concrete provider RNG failure.
- refimpl/provider boundary proof remains important but should not be bundled
  into the first qsc no-mutation scope.
- NA-0455 did not authorize implementation paths, fake providers, test seams,
  runtime code, crypto code, or new executable tests.

NA-0456 consumes that inheritance only as qsc no-mutation scope authorization
evidence.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- qsc provider RNG no-mutation scope must remain bounded.
- qsc no-mutation proof is distinct from refimpl/provider RNG proof.
- exact qsc call-site authorization should precede runtime or test mutation.
- production semantics, fail-closed behavior, no RNG-failure-complete claim,
  and no provider-RNG-complete claim remain required caveats.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green.
- nested qsc fuzz lock audit is green.
- cfg RNG failure tests are green.
- route/contact/attachment seam evidence is green.
- qsc key lifecycle and provider-error tests are green.
- refimpl `pqkem768` is green.
- qsc adversarial marker/syntax evidence is green locally; local full smoke
  remains subject to local cargo-fuzz availability.
- public-safety is green on current main.
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
- qsc provider RNG no-mutation authorization is internal governance evidence only.

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

## qsc provider RNG no-mutation target inventory

| Candidate | Exact source path | Provider operation | Failure type | Current marker | Current mutation risk | Existing tests | Current APIs enough? | Future fake/seam need | Refimpl mutation needed? | Priority |
|---|---|---|---|---|---|---|---|---|---|---|
| KEM decap | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::decap()` in initiator finalize | provider-error, no RNG role expected | `pq_decap_failed` | low; reject returns before session store and pending clear | `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs` | yes for generic provider-error; no for RNG-specific proof | no for decap; documentation caveat only | no | medium |
| KEM encap | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::encap()` in responder path | Result provider failure; concrete RNG failure hidden in provider | `pq_encap_failed` | low/medium; branch is before pending store, but not forceable today | defensive marker only; no forced qsc test | partial | yes, a cfg-only/fake provider failure selector or injection strategy | no for qsc-only generic failure; maybe for concrete provider RNG | high |
| KEM keypair | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_kem_keypair()` via `runtime_pq_kem_keypair()` | concrete provider RNG failure currently infallible | none | high in identity bootstrap because key generation precedes vault/public writes | success paths and key lifecycle coverage only | no | yes, because qsc sees an infallible tuple | not for a qsc cfg seam; yes if proving provider internals | high |
| Signature keypair | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` via `runtime_pq_sig_keypair()` | concrete provider RNG failure currently infallible | none | high in identity bootstrap and legacy migration ordering | success paths only | no | yes, because qsc sees an infallible tuple | not for a qsc cfg seam; yes if proving provider internals | high |
| Signature signing | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for B1/A2 | Result provider failure; no confirmed RNG role after key generation | `sig_sign_failed` | medium; A2 branch currently stores session before signing | signature tamper tests cover `sig_invalid`, not forced sign failure | partial | yes for forced qsc provider-error/no-mutation proof; RNG-specific caveat remains | no for qsc generic failure; maybe for concrete provider RNG | high |
| Signature verification | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::verify()` for B1/A2 | Result/invalid signature; no RNG role | `sig_invalid` | low; reject before completion in tested tamper paths | handshake tamper no-mutation tests | yes for invalid signature; partial for provider-error | maybe only if future generic provider-error parity requires it | no | low |
| Identity bootstrap | `qsl/qsl-client/qsc/src/identity/mod.rs` | generated KEM and signature identity keypairs | concrete provider RNG failure currently infallible | none | high; secret-store and public-record ordering must stay fail-closed if made fallible | success paths and zeroization coverage only | no | yes, exact strategy needed before changing error propagation | not for a qsc cfg seam; yes if proving provider internals | high |
| X25519/ephemeral generation | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `hs_ephemeral_keypair()` via `StdCrypto::keypair()` | concrete provider RNG failure currently infallible | none | medium; initiator generation occurs before pending write; responder generation occurs before pending write | success paths only | no | yes, because qsc sees an infallible tuple | not for a qsc cfg seam; yes if proving provider internals | high |

Inventory conclusion:

- qsc call sites are exact.
- qsc can observe no-mutation around current API boundaries only where a failure
  is already forceable.
- qsc cannot currently force concrete provider RNG failure for keypair or
  X25519 paths.
- future qsc proof needs a test-only fake/seam strategy before implementation.

## qsc KEM provider no-mutation review

`pq_decap_failed` is already sufficiently covered for generic provider-error
no-mutation by NA-0436/NA-0439 evidence. It is not an RNG-specific proof
because decapsulation is not a randomness-dependent operation in the current
path.

`pq_encap_failed` remains a defensive branch with a sanitized marker. The
branch is useful, but it is not forceable with current qsc APIs against the
concrete provider RNG path. A future qsc provider RNG scope should not focus on
decap. It should focus on KEM keypair and encap failure if the future seam can
force those failures before qsc writes pending/session state.

Classifications:

- `QSC_KEM_PROVIDER_NO_MUTATION_ALREADY_COVERED` for generic
  `pq_decap_failed`.
- `QSC_KEM_PROVIDER_ENCAP_SCOPE_DOCUMENTATION_ONLY` until a fake/seam is
  selected for `pq_encap_failed`.
- `QSC_KEM_PROVIDER_RNG_REQUIRES_FAKE_SEAM` for KEM keypair and concrete KEM
  encap RNG failure.

KEM scope is not implementation-ready in NA-0456 because the exact seam
mechanism is not selected.

## qsc signature / identity provider no-mutation review

`sig_sign_failed` and `sig_invalid` paths are present and sanitized. Existing
tamper tests exercise signature-invalid reject paths, but qsc does not
currently force a provider signing failure. Signing failures are mostly before
new pending-state writes on the responder B1 path, but the initiator A2 path
stores a session before signing A2; any future no-mutation proof must decide
whether that current ordering is acceptable evidence, a residual, or a target
for a later exact implementation lane.

Identity key generation failures are not forceable today because
`hs_kem_keypair()` and `hs_sig_keypair()` return infallible tuples from qsc's
perspective. qsc does not expose an existing fake/signature provider path for
concrete RNG failure. Exact source paths are clear, but the fake/seam mechanism
is not.

Classifications:

- `QSC_SIGNATURE_PROVIDER_REQUIRES_FAKE_SEAM`
- `QSC_SIGNATURE_PROVIDER_DOCUMENTATION_ONLY` for `sig_invalid` RNG-specific
  claims, because verification has no RNG role.

Signature/identity scope is not implementation-ready in NA-0456 because the
future strategy must first decide how to force provider signing/keypair failure
without changing production semantics.

## qsc handshake / identity split decision

NA-0456 rejects a direct combined implementation lane. KEM, signature/identity,
and X25519 generation have distinct failure mechanics:

- KEM decap already has generic provider-error no-mutation evidence.
- KEM encap and signing have `Result`-returning provider calls but no current
  forced concrete provider RNG failure harness.
- KEM keypair, signature keypair, and X25519 keypair are infallible from qsc's
  perspective and require strategy before implementation.
- Identity bootstrap has higher partial-write risk than handshake-only encap
  because key generation feeds vault secret writes and public-record writes.

The smallest safe successor is not KEM-only, signature/identity-only, combined
implementation, documentation-only, refimpl-first, or the next audit domain.
The smallest safe successor is a qsc fake/test-seam strategy authorization lane
that chooses the exact test-only mechanism and then may split implementation
after the mechanism is bounded.

## qsc no-mutation authorization matrix

| Candidate qsc surface | Exact path(s) | Provider operation | Existing error marker | Existing coverage | Future fake/seam need | Existing APIs enough? | Refimpl dependency? | Production-semantics risk | Future test path if selected | Priority | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---:|---|---|---|---|
| KEM decap / `pq_decap_failed` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::decap()` | `pq_decap_failed` | NA-0436 no-mutation test and NA-0439 adversarial step | no for generic provider-error | yes for generic provider-error | no | low | none; keep existing test | medium | no | already covered for generic provider-error; not RNG-specific | No RNG-failure-complete claim | G1-G5 |
| KEM encap / `pq_encap_failed` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::encap()` | `pq_encap_failed` | defensive marker only | yes | partial | partial | medium | candidate future qsc provider RNG strategy test | high | no direct implementation | failure is not forceable through current qsc APIs | No provider-RNG-complete claim | G1-G5 |
| KEM keypair/provider generation | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_kem_keypair()` | none | success paths only | yes | no | partial | medium/high | candidate future qsc provider RNG strategy test | high | no direct implementation | infallible qsc surface needs strategy | No provider-RNG-complete claim | G1-G5 |
| Signature signing failure / `sig_sign_failed` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` | `sig_sign_failed` | marker path present; no forced provider failure | yes | partial | partial | medium | candidate future qsc signature provider strategy test | high | no direct implementation | A2 store/sign ordering needs exact review | No provider-RNG-complete claim | G1-G5 |
| Signature verification failure / `sig_invalid` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::verify()` | `sig_invalid` | tamper no-mutation tests | maybe for generic provider-error parity | partial | no | low | likely no RNG-specific test | low | no | verification has no RNG role | No RNG-failure-complete claim | G1-G5 |
| Identity bootstrap/provider generation | `qsl/qsl-client/qsc/src/identity/mod.rs` | KEM and signature identity keypair helpers | none | success paths and zeroization coverage | yes | no | partial | high | candidate future qsc identity provider RNG strategy test | high | no direct implementation | partial-write ordering needs exact seam strategy | No provider-RNG-complete claim | G1-G5 |
| X25519/ephemeral provider generation | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `hs_ephemeral_keypair()` | none | success paths only | yes | no | partial | medium | candidate future qsc handshake provider RNG strategy test | high | no direct implementation | infallible qsc surface needs strategy | No provider-RNG-complete claim | G1-G5 |
| qsc combined KEM/signature/identity scope | handshake and identity paths above | mixed | mixed | partial | yes | no/partial | partial | high | candidate only after strategy | high | no | too broad before seam strategy | No RNG-failure-complete claim | G1-G5 |
| documentation-only | governance paths only | none | none | possible caveats | no | yes | no | low | none | medium | no | insufficient because qsc no-mutation evidence remains valuable | No provider-RNG-complete claim | G1-G5 |
| refimpl-first | `tools/refimpl/` future candidate paths | provider internals | mixed | partial | yes | yes for provider boundary only | yes | medium | refimpl boundary strategy test | medium | no | NA-0455 selected qsc-first; refimpl remains residual | No provider-RNG-complete claim | G1-G5 |
| next audit domain | future KEM/signature/transcript audit paths | transcript binding audit | none | existing formal/model evidence | no | yes | no | low | future audit testplan only | medium | no | qsc provider RNG mechanism is the immediate blocker | No crypto-complete claim | G1-G5 |

## authorization decision

Primary classification:

`QSC_PROVIDER_RNG_NO_MUTATION_REQUIRES_FAKE_SEAM_STRATEGY`

Selected successor:

`NA-0457 -- QSL qsc Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

NA-0456 does not select an implementation-ready scope. It selects a strategy
lane because exact qsc source paths are known but the least invasive mechanism
is not. Future implementation must not start until NA-0457 decides whether the
qsc mechanism is cfg-only seam, provider fake, trait injection, documentation
boundary, split KEM/signature implementation, or another bounded option.

## successor selection

Selected exact NA-0457 successor:

`NA-0457 -- QSL qsc Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`

Rationale:

- combined qsc implementation is too broad before seam strategy;
- KEM-only implementation would leave signature/identity and X25519 mechanism
  ambiguity unresolved;
- signature/identity-only implementation would leave KEM encap/keypair
  ambiguity unresolved;
- documentation-only would underuse the exact qsc no-mutation call-site
  inventory;
- refimpl-first contradicts the qsc-first split selected by D-0897.

## future path/scope bundle

Future NA-0457 allowed qsl-protocol mutation paths should be limited to:

- `docs/governance/evidence/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0457 read-only candidate paths should include:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `tools/refimpl/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts/workflows read-only.

Future NA-0457 must not implement qsc source changes, executable tests, fuzz
targets, vectors, formal models, runtime behavior, crypto behavior,
dependencies, Cargo manifests, lockfiles, workflows, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, backup/restore/local-ops paths, or
public claim expansions unless a later exact implementation directive authorizes
specific paths.

If NA-0457 later selects implementation, likely candidate implementation paths
for a later lane must be reauthorized exactly and may include only the selected
qsc test path plus selected source paths from `handshake/mod.rs` and/or
`identity/mod.rs`.

## future validation/marker plan

Common NA-0457 markers:

- `NA0457_QSC_PROVIDER_RNG_SCOPE_CONSUMED_OK`
- `NA0457_NEXT_SCOPE_SELECTED_OK`
- `NA0457_NO_DEPENDENCY_CHANGE_OK`
- `NA0457_NO_WORKFLOW_CHANGE_OK`
- `NA0457_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0457_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0457_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0457_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0457_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0457_ONE_READY_INVARIANT_OK`
- `NA0457_QSC_PROVIDER_RNG_FAKE_SEAM_STRATEGY_SELECTED_OK`

If NA-0457 selects a future implementation successor, that later successor
should require:

- `NA0457_QSC_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0457_QSC_PROVIDER_RNG_FAILURE_NO_MUTATION_OK`
- `NA0457_PRODUCTION_SEMANTICS_UNCHANGED_OK`

If NA-0457 splits scope, that later successor should require:

- `NA0457_QSC_KEM_SIGNATURE_SPLIT_SCOPE_SELECTED_OK`

If NA-0457 moves to another audit domain, that later successor should require:

- `NA0457_KEM_SIGNATURE_TRANSCRIPT_AUDIT_SELECTED_OK`

## public claim/external review/website boundary

qsc provider RNG no-mutation scope authorization is internal governance
evidence only.

qsc provider RNG no-mutation scope authorization is not production readiness.

qsc provider RNG no-mutation scope authorization is not public-internet readiness.

qsc provider RNG no-mutation scope authorization is not crypto-complete proof.

qsc provider RNG no-mutation scope authorization is not side-channel-free proof.

qsc provider RNG no-mutation scope authorization is not RNG-failure-complete
proof.

qsc provider RNG no-mutation scope authorization is not provider-RNG-complete
proof.

qsc provider RNG no-mutation scope authorization is not bug-free proof.

qsc provider RNG no-mutation scope authorization is not vulnerability-free proof.

qsc provider RNG no-mutation scope authorization is not perfect-crypto proof.

qsc provider RNG no-mutation scope authorization is not public technical paper
content.

No README, START_HERE, docs-public, or website update is made.

No public-readiness or public-security claim is made.

Cargo audit green is dependency-health evidence only, not vulnerability-free
proof.

Future tests, if authorized later, must be described as bounded evidence only.

## rejected alternatives

- `QSC_PROVIDER_RNG_NO_MUTATION_SCOPE_IMPLEMENTATION_READY` rejected because
  infallible keypair/X25519 surfaces need a mechanism decision first.
- `QSC_PROVIDER_RNG_NO_MUTATION_KEM_SCOPE_READY` rejected because KEM decap is
  already covered generically and KEM encap/keypair still need a seam strategy.
- `QSC_PROVIDER_RNG_NO_MUTATION_SIGNATURE_IDENTITY_SCOPE_READY` rejected because
  signing/keypair/identity bootstrap failure forcing is not yet exact.
- `QSC_PROVIDER_RNG_NO_MUTATION_SPLIT_KEM_SIGNATURE_NEEDED` rejected for this
  lane because splitting before selecting the fake/seam mechanism would still
  leave each split implementation ambiguous.
- `QSC_PROVIDER_RNG_NO_MUTATION_REQUIRES_REFIMPL_FIRST` rejected because
  NA-0455 selected qsc-first and qsc no-mutation remains valuable.
- `QSC_PROVIDER_RNG_NO_MUTATION_DOCUMENTATION_ONLY` rejected because exact qsc
  call sites exist and merit a future strategy selection.
- `NEXT_AUDIT_DOMAIN_KEM_SIGNATURE_TRANSCRIPT` rejected because the provider RNG
  fake/seam mechanism is the narrower immediate blocker.
- `QSC_PROVIDER_RNG_NO_MUTATION_AMBIGUOUS` rejected because the target inventory
  and successor are clear enough.

## backup-impact statement

Backup impact: none. Codex did not run backup or restore. Codex did not run
sudo. Codex did not mutate qsl-backup, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, backup scripts,
or backup tree paths.

The qsl-backup SHA matched the expected boundary hash and the Codex ops source
inclusion count was exactly one.

## next recommendation

Restore `NA-0457 -- QSL qsc Provider RNG Failure Fake / Test Seam Strategy
Authorization Plan` as the sole READY successor after NA-0456 closes. NA-0457
should select the exact qsc fake/test-seam mechanism or reject implementation
with evidence before any qsc source/test mutation is authorized.
