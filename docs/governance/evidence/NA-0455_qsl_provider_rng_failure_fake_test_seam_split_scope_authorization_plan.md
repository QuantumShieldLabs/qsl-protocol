Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0455 QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0455 consumes NA-0454 and splits provider-dependent RNG failure fake/test-seam
work into the smallest safe next authorization scope.

Primary classification:

`PROVIDER_RNG_SPLIT_QSC_FIRST`

Selected successor:

`NA-0456 -- QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan`

Reason: qsc owns the visible state mutation and no-mutation proof surfaces for
handshake and identity provider calls. Those qsc paths are exact enough for a
next authorization lane, while refimpl/provider internals still require more
care around infallible `OsRng`-backed helpers and trait shapes. The next lane
therefore should authorize or reject exact qsc-side no-mutation scope first,
without implementing provider fakes, test seams, tests, runtime code, or crypto
code in NA-0455.

This directive authorizes no implementation mutation. It changes no runtime
behavior, crypto behavior, dependencies, Cargo manifests, lockfiles, workflows,
executable tests, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
Cargo audit green remains dependency-health evidence only.

Required markers recorded by this evidence:

- `NA0455_PROVIDER_RNG_SPLIT_SCOPE_CONSUMED_OK`
- `NA0455_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0455_NA0454_INHERITANCE_CONSUMED_OK`
- `NA0455_SPLIT_SCOPE_INVENTORY_OK`
- `NA0455_QSC_FIRST_REVIEW_OK`
- `NA0455_REFIMPL_FIRST_REVIEW_OK`
- `NA0455_QSC_FIRST_SELECTED_OK`
- `NA0455_SUCCESSOR_NA0456_SELECTED_OK`
- `NA0455_NO_RUNTIME_CHANGE_OK`
- `NA0455_NO_CRYPTO_CHANGE_OK`
- `NA0455_NO_DEPENDENCY_CHANGE_OK`
- `NA0455_NO_WORKFLOW_CHANGE_OK`
- `NA0455_NO_TEST_IMPLEMENTATION_OK`
- `NA0455_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0455_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0455_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0455_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0455_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0455_ONE_READY_INVARIANT_OK`

## Live NA-0455 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0455.
- NA-0454 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0896.
- D-0895 exists once.
- D-0896 exists once.
- D-0897 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0455 mutation paths are exactly:

- `docs/governance/evidence/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_plan.md`
- `tests/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
rollback, and backup tree paths.

Acceptance criteria:

- qwork proof files are verified without rerunning qwork;
- NA-0454 split-scope inheritance is consumed;
- qsc and refimpl candidate provider RNG split targets are inventoried;
- qsc-first and refimpl-first strategies are classified;
- combined versus sequential strategy is decided;
- successor NA-0456 is selected with exact governance scope;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, missing
public-safety, missing D-0896, D-0897 already present, failed root or nested
audit, unconsumable NA-0454 inheritance, unclassifiable split scope, unsafe
successor selection, backup boundary regression, or any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0455/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0455/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0455`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0455/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0455`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`c6dca3e2415`. Fetch did not advance `origin/main`.

PR #1178 was verified merged at `c6dca3e2415` through the GitHub connector. The
required `gh pr view` command returned GraphQL HTTP 401 twice despite
`gh auth status` reporting a logged-in account; this was recorded as a
recoverable GitHub CLI authentication failure and no credentials were changed.

Proof root:

`/srv/qbuild/tmp/NA0455_provider_rng_fake_seam_split_scope_20260610T152352Z`

## NA-0454 inheritance

NA-0454 / D-0895 selected:

`PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED`

Inherited facts:

- Existing provider fakes are insufficient for concrete provider RNG failure.
- Existing qsc cfg seams prove qsc-local RNG failure only.
- qsc provider-error no-mutation proof covers a generic `pq_decap_failed`
  branch, not provider RNG failure.
- refimpl provider fakes model generic provider errors, but concrete RNG
  failure remains behind `OsRng` or infallible trait/helper methods.
- qsc no-mutation proof and refimpl provider-boundary proof should be split
  before implementation.
- NA-0454 did not authorize direct implementation.

NA-0455 consumes that inheritance only as split-scope authorization evidence.
It does not implement provider fakes, test seams, runtime code, crypto code, or
new executable tests.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- Split provider RNG fake/seam work into the least invasive future scope.
- Keep qsc no-mutation proof separate from refimpl provider-boundary proof unless
  evidence supports a combined exact lane.
- Prefer exact read-only authorization before runtime or test mutation.
- Preserve production semantics; no RNG-failure-complete claim and no provider-RNG-complete claim remain caveats.

CI / Dependency / Release Health Steward:

- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock audit is green.
- cfg RNG failure tests are green.
- route/contact/attachment seam evidence is green.
- qsc key lifecycle and provider-error tests are green.
- refimpl `pqkem768` is green.
- qsc adversarial script marker and syntax are green locally; local full smoke
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
- Provider RNG split-scope authorization is internal governance evidence only.

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

## Split-scope target inventory

### Candidate qsc scope

Candidate qsc surfaces:

- qsc KEM provider boundary.
- qsc signature/identity provider boundary.
- qsc handshake provider RNG boundary.
- qsc no-mutation behavior around provider failure.

Exact candidate paths for future authorization review:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/`

Evidence gap:

- qsc state changes happen after provider call sites, but current tests cannot
  force concrete provider RNG failure for KEM keypair, ML-KEM encap, ML-DSA
  keypair, or X25519 keypair without a new qsc-side fake/seam strategy.

Future implementation can be small only if a later lane limits qsc mutation to
cfg/test-only wrappers or fakes around exact call sites. Existing provider fakes
do not directly force concrete provider RNG failure in qsc. Production semantics
can be proven unchanged only with normal no-cfg tests. No-mutation evidence can
be tested at qsc state boundaries. Provider-boundary behavior cannot be fully
proven by qsc-only tests without a separate refimpl/provider proof.

Another narrowing lane is required before implementation: NA-0456 qsc
no-mutation scope authorization.

### Candidate refimpl scope

Candidate refimpl surfaces:

- refimpl ML-KEM provider RNG boundary.
- refimpl ML-DSA provider RNG boundary.
- refimpl X25519 randomness boundary.
- refimpl `Rng12`/random helper boundary.

Exact candidate paths for future authorization review:

- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/mod.rs`
- `tools/refimpl/quantumshield_refimpl/tests/`

Evidence gap:

- `runtime_pq_kem_keypair()`, `runtime_pq_sig_keypair()`,
  `X25519Dh::keypair()`, and `Rng12::random_nonce12()` are infallible from the
  caller perspective.
- `PqKem768::encap()` is fallible and uses `OsRng`, but concrete RNG failure is
  not forceable through current public APIs.

Future implementation may require cfg seams, helper wrappers, fake provider
types, or trait/API changes. That makes refimpl-first less exact than qsc-first
for the immediate successor. Production semantics can be proven unchanged only
if future proof is test-only/cfg-only and normal provider tests remain green.
Provider-boundary behavior can be tested after exact helper or fake strategy is
selected.

Another narrowing lane is required before refimpl implementation.

### Candidate documentation-only scope

Candidate path family:

- provider RNG claim-boundary documentation under governance evidence only.

Evidence gap:

- Documentation-only would preserve safety but would not advance bounded
  no-mutation evidence for qsc state surfaces.

Another documentation-only lane is not selected because qsc-first authorization
is exact enough and higher value.

### Candidate next audit domain

Candidate future domain:

- KEM / Signature / Transcript Binding Read-Only Audit Plan.

Evidence gap:

- This remains valuable, but provider RNG split scope is still actionable as a
  qsc-first authorization lane. Moving domains now would leave the provider RNG
  residual without using the NA-0454 split-scope conclusion.

## qsc-first strategy review

Questions and answers:

- Can qsc test no-mutation behavior around provider RNG failure without changing
  refimpl provider internals? Yes, but only as qsc-side forced provider-failure
  simulation through exact cfg/test-only call-site seams; this would not prove
  concrete refimpl/provider RNG failure.
- Can qsc use an existing provider fake/test double? No. Existing qsc seams cover
  qsc-local RNG labels, and existing provider fakes are not wired into qsc
  provider internals.
- Would qsc require a new provider injection seam? Yes, for any future
  implementation. The seam should be cfg/test-only and should fail before state
  mutation at selected qsc call sites.
- Which exact qsc source/test paths would be touched by a future implementation?
  Candidate paths are `qsl/qsl-client/qsc/src/handshake/mod.rs`,
  `qsl/qsl-client/qsc/src/identity/mod.rs`, and an exact future qsc test path
  under `qsl/qsl-client/qsc/tests/`. These are candidates only; NA-0455 does not
  authorize them.
- Would future changes be cfg/test-only? They must be, unless a later directive
  explicitly authorizes otherwise. NA-0455 does not authorize non-cfg runtime or
  crypto changes.
- Would future changes risk production semantics? Medium if not cfg/test-only;
  low only if the future seam is proven inert in normal no-cfg builds.
- Would qsc-only evidence be meaningful without refimpl provider-boundary proof?
  Yes, as bounded no-mutation evidence for qsc state behavior. It would not be
  provider-boundary-complete.
- Is qsc-only scope smaller and safer than refimpl-first? Yes for the next
  authorization lane, because qsc call sites and state boundaries are exact and
  refimpl trait/helper decisions remain broader.

Classification:

`QSC_PROVIDER_RNG_SPLIT_SCOPE_READY`

## refimpl-first strategy review

Questions and answers:

- Can refimpl provider RNG failure be forced without qsc changes? Not for
  concrete provider RNG failure through current public APIs.
- Do existing traits allow a fake provider or failure injection? Partially.
  `PqKem768` and `PqSigMldsa65` can fake generic provider errors, but
  `X25519Dh::keypair()` and `Rng12::random_nonce12()` are infallible, and
  keypair helpers are infallible free functions.
- Would refimpl need a cfg seam, trait change, fake provider type, or wrapper?
  Yes. The exact shape remains broader than qsc-first no-mutation authorization.
- Which exact refimpl source/test paths would be touched by a future
  implementation? Candidate paths are
  `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`,
  `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`,
  `tools/refimpl/quantumshield_refimpl/src/crypto/mod.rs`, and an exact future
  refimpl test path under `tools/refimpl/quantumshield_refimpl/tests/`. These are
  candidates only; NA-0455 does not authorize them.
- Would future changes be test-only or production-visible? That is not yet exact.
  Trait/API changes could be production-visible, while cfg-only helper seams
  could remain test-only.
- Would refimpl-only evidence be meaningful without qsc no-mutation proof? Yes
  as provider-boundary evidence, but it would not prove qsc state no-mutation.
- Is refimpl-first scope smaller and safer than qsc-first? No for the immediate
  successor, because infallible helper and trait boundaries must be resolved
  first.

Classification:

`REFIMPL_PROVIDER_RNG_NEEDS_MORE_TRIAGE`

## Combined vs sequential decision

Option 1 -- qsc-first authorization: selected.

Evidence: qsc no-mutation call sites are exact and valuable before refimpl
provider-boundary implementation. Candidate future qsc paths are
`qsl/qsl-client/qsc/src/handshake/mod.rs`,
`qsl/qsl-client/qsc/src/identity/mod.rs`, and an exact future qsc test path
under `qsl/qsl-client/qsc/tests/`. Future validation must include cfg and no-cfg
qsc RNG tests, key lifecycle, provider-error no-mutation, public-safety, root
and nested audits, and claim scans. Public caveat: qsc-first evidence would be
bounded no-mutation evidence only.

Option 2 -- refimpl-first authorization: rejected for first successor.

Evidence: refimpl provider-boundary proof is valuable, but immediate
implementation shape may require trait/helper/cfg decisions across `traits.rs`
and `stdcrypto.rs`. Public caveat: refimpl-first evidence would not prove qsc
state no-mutation.

Option 3 -- combined qsc/refimpl implementation authorization: rejected.

Evidence: exact combined source/test paths and seam shape are not small enough.
Combined implementation would risk mixing qsc no-mutation and refimpl
provider-boundary proof before either side has exact authorization. Public
caveat: no provider-RNG-complete claim is made.

Option 4 -- another split-scope lane: rejected as a separate KEM/signature/X25519
triage lane for now.

Evidence: qsc-first authorization is already a smaller split than a combined
implementation lane. NA-0456 can decide whether qsc must split further by KEM,
signature/identity, or X25519 before implementation. Public caveat: no RNG-failure-complete claim is made.

Option 5 -- documentation-only: rejected.

Evidence: documentation-only would leave qsc no-mutation evidence unplanned even
though exact qsc call-site surfaces are clear. Public caveat: provider RNG gaps
remain gaps.

Option 6 -- move to next audit domain: rejected.

Evidence: KEM/signature/transcript binding audit remains useful, but provider
RNG split scope is still actionable and directly follows NA-0454. Public caveat:
no public-readiness or public-security claim is made.

## Split-scope authorization matrix

| Candidate first lane | Surface(s) | Exact candidate paths | Existing tests/fakes | Future mutation type | Production-semantics risk | Evidence value | Scope size | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| qsc KEM provider no-mutation scope | KEM identity keypair, responder encap, initiator decap state boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs`; future exact qsc test under `qsl/qsl-client/qsc/tests/` | `handshake_provider_error_no_mutation`; qsc cfg RNG seam precedent | future cfg/test-only qsc seam or fake, not authorized here | low only if no-cfg inert proof is required; otherwise medium | high qsc state no-mutation value | small to medium | no for authorization, yes before implementation | yes | qsc state surfaces are exact and safer than refimpl trait/helper changes | bounded internal evidence only; no provider-RNG-complete claim | G1, G2, G3, G4 |
| qsc signature/identity provider no-mutation scope | ML-DSA keypair and signing state/write order | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs`; future exact qsc test under `qsl/qsl-client/qsc/tests/` | signature error branches; key lifecycle write-order background | future cfg/test-only qsc seam or fake, not authorized here | medium unless write ordering remains exact | high for identity/handshake no-mutation | small to medium | no for authorization, yes before implementation | yes as part of qsc-first | identity writes are qsc-visible and must fail before partial state | no crypto-complete claim | G1, G2, G3, G4 |
| qsc handshake provider RNG boundary | X25519 keypair, KEM encap, signing, session state commit | `qsl/qsl-client/qsc/src/handshake/mod.rs`; future exact qsc test under `qsl/qsl-client/qsc/tests/` | qsc session ID seam; provider-error no-mutation | future cfg/test-only qsc seam or fake, not authorized here | low only with normal build proof | high | small | no for authorization, yes before implementation | yes as part of qsc-first | handshake state mutation boundary is exact | no RNG-failure-complete claim | G1, G2, G3, G4 |
| refimpl ML-KEM provider RNG scope | ML-KEM keypair and encap | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `tools/refimpl/quantumshield_refimpl/tests/` | `pqkem768`; generic KEM fakes in qsp tests | possible cfg seam, wrapper, or helper change | medium | high provider-boundary value | medium | yes | no | keypair/helper infallibility needs separate exact shape | dependency health only; no vulnerability-free claim | G1, G2, G4 |
| refimpl ML-DSA provider RNG scope | ML-DSA keypair and sign/verify | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `tools/refimpl/quantumshield_refimpl/tests/` | generic signature fakes in qsp/kt tests | possible cfg seam, wrapper, or helper change | medium | high | medium | yes | no | keypair helper uses `OsRng` and is infallible | no crypto-complete claim | G1, G2, G4 |
| refimpl X25519 randomness scope | X25519 keypair | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `tools/refimpl/quantumshield_refimpl/tests/` | dummy DH fakes, infallible | possible trait/API or cfg wrapper | medium | high | medium | yes | no | trait is infallible and must not be changed casually | no perfect-crypto claim | G1, G2, G4 |
| refimpl Rng12/random helper scope | nonce helper | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; qsp ratchet tests | fixed RNG fakes only | possible trait/API or cfg wrapper | medium | medium | medium | yes | no | infallible `Rng12` shape needs a distinct decision | no RNG-failure-complete claim | G2, G4 |
| combined qsc/refimpl scope | qsc no-mutation plus refimpl provider boundary | qsc paths plus refimpl crypto paths above | partial fakes and qsc local seams | broad cfg/fake/trait/test changes | medium to high | highest if exact | large | yes | no | too broad for first successor | no provider-RNG-complete claim | G1, G2, G3, G4 |
| documentation-only scope | claim-boundary documentation | governance evidence/testplan only | existing governance caveats | docs only | low | low to medium | small | no | no | lower value than qsc-first authorization | gaps remain gaps | G4, G5 |
| next audit domain | KEM / signature / transcript binding audit | future governance evidence/testplan only | existing formal/model checks | docs/read-only audit | low | medium | small | no | no | useful later but provider RNG split scope is still actionable | no public-readiness claim | G1, G3, G4 |

## Authorization decision

Primary classification:

`PROVIDER_RNG_SPLIT_QSC_FIRST`

Selected successor:

`NA-0456 -- QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan`

Decision summary:

- qsc-first is selected for the next authorization lane.
- refimpl-first is deferred because provider helper/trait shape is not exact
  enough for first successor.
- combined qsc/refimpl implementation is rejected for now.
- another KEM/signature/X25519 split-scope lane is rejected for now because
  qsc-first is already a smaller split; NA-0456 may split further if required.
- documentation-only is rejected as the immediate successor.
- moving to the KEM/signature/transcript audit domain is rejected for now.

No implementation-ready lane is selected by NA-0455. Exact implementation paths
remain candidates only and are not authorized for mutation by this directive.

## Successor selection

Selected exact NA-0456 successor:

`NA-0456 -- QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan`

NA-0456 must remain authorization-only unless a later directive authorizes exact
implementation paths. It must decide whether qsc provider-dependent RNG failure
no-mutation scope can move to an exact implementation lane, whether it must
split further by KEM/signature/X25519, or whether refimpl/provider-boundary
authorization must come first.

Exactly one READY item remains mandatory.

## Future path/scope bundle

Future allowed NA-0456 paths for the qsc-first authorization successor:

- `docs/governance/evidence/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_plan.md`
- `tests/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future read-only inspection may include:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `tools/refimpl/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts/workflows read-only.

Candidate implementation paths for a later implementation lane, not authorized
by NA-0455 and not automatically authorized by NA-0456:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- one exact future qsc test path under `qsl/qsl-client/qsc/tests/`

Future forbidden unless exact later scope authorizes:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs/website;
- qsl-server/qsl-attachments changes;
- qshield runtime or qshield-cli changes;
- backup/restore/qsl-backup changes;
- public claims.

## Future validation/marker plan

Common NA-0456 markers:

- `NA0456_PROVIDER_RNG_SPLIT_SCOPE_CONSUMED_OK`
- `NA0456_NEXT_SCOPE_SELECTED_OK`
- `NA0456_NO_DEPENDENCY_CHANGE_OK`
- `NA0456_NO_WORKFLOW_CHANGE_OK`
- `NA0456_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0456_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0456_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0456_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0456_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0456_ONE_READY_INVARIANT_OK`
- `NA0456_QSC_PROVIDER_RNG_SCOPE_SELECTED_OK`

Future validation required by NA-0456:

- qwork proof-file verification without running qwork;
- queue and decision proof;
- qsc/refimpl surface inventory;
- root cargo audit;
- nested qsc fuzz lock audit;
- cfg and no-cfg `rng_failure_residual_surfaces`;
- cfg and no-cfg `rng_failure_behavior`;
- qsc key lifecycle zeroization;
- qsc provider-error no-mutation;
- qsc `send_commit`;
- refimpl `pqkem768`;
- qsc adversarial script marker and syntax;
- formal model checks;
- public-safety;
- link check;
- leak scan;
- overclaim scan;
- scope guard.

Future implementation validation, if a later implementation lane is selected,
must additionally prove normal no-cfg production semantics unchanged and must
describe all provider RNG tests as bounded evidence only.

## Public claim/external review/website boundary

Provider RNG split-scope authorization is internal governance evidence only.

Provider RNG split-scope authorization is not production readiness.

Provider RNG split-scope authorization is not public-internet readiness.

Provider RNG split-scope authorization is not crypto-complete proof.

Provider RNG split-scope authorization is not side-channel-free proof.

Provider RNG split-scope authorization is not RNG-failure-complete proof.

Provider RNG split-scope authorization is not provider-RNG-complete proof.

Provider RNG split-scope authorization is not bug-free proof.

Provider RNG split-scope authorization is not vulnerability-free proof.

Provider RNG split-scope authorization is not perfect-crypto proof.

Provider RNG split-scope authorization is not a public technical paper.

No README, START_HERE, public docs, or website update is made.

No public-readiness claim is made. No public-security claim is made.

Cargo audit green is dependency-health evidence, not vulnerability-free proof.

Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

- Refimpl-first as NA-0456: rejected because helper/trait decisions are broader
  than qsc no-mutation authorization.
- Combined qsc/refimpl implementation: rejected because exact paths and seam
  shape are not small enough and production semantics cannot be proven unchanged
  before separate authorization.
- Further KEM/signature/X25519 triage as NA-0456: rejected for now because
  qsc-first authorization is already a smaller split and can decide whether more
  subdivision is needed.
- Documentation-only: rejected because qsc no-mutation scope is exact enough to
  authorize the next narrowing lane.
- Move to KEM/signature/transcript audit: rejected for now because provider RNG
  split scope remains directly actionable.

## Backup-impact statement

No backup was run. No restore was run. No sudo command was run. No qsl-backup,
backup status, backup plan, rollback subtree, timer, fstab, retention,
source-list, or backup tree path was mutated.

Read-only qsl-backup boundary proof matched expected SHA
`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`, and the
codex ops source inclusion count remained one.

## Next recommendation

After the NA-0455 evidence PR merges and public-safety is green, close NA-0455
and restore NA-0456 as the sole READY item:

`QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan`

NA-0456 should preserve no-runtime/no-crypto/no-dependency/no-workflow and
no-public-overclaim boundaries while selecting exact qsc no-mutation
implementation scope, splitting further, or deferring to refimpl provider-boundary
authorization if qsc-first proves too intrusive.
