Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0454 QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0454 consumes NA-0453 and selects the least invasive next strategy for
provider-dependent RNG failure evidence.

Primary classification:

`PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED`

Selected successor:

`NA-0455 -- QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan`

Reason: existing qsc and refimpl fakes are useful for generic provider-error
behavior, but they are not sufficient to force concrete provider RNG failure.
The qsc proof need is no-mutation at handshake/identity call sites; the refimpl
proof need is provider-boundary behavior around `OsRng`-backed helpers and
infallible traits. Combining those into direct implementation now would require
trait/API or cfg-only seam decisions across multiple source paths before the
future scope is exact enough.

NA-0454 is authorization-only. It changes no runtime behavior, crypto behavior,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qsl-backup, backup status, backup plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
Cargo audit green remains dependency-health evidence only.

## Live NA-0454 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0454.
- NA-0453 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0894.
- D-0893 exists once.
- D-0894 exists once.
- D-0895 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0454 mutation paths are exactly:

- `docs/governance/evidence/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
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
- NA-0453 provider RNG boundary inheritance is consumed;
- existing qsc and refimpl fake/test-seam infrastructure is inventoried;
- qsc/refimpl combined versus split strategy is decided;
- future NA-0455 scope and validation markers are selected;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, missing
public-safety, missing D-0894, D-0895 already present, failed root or nested
audit, unclassifiable provider RNG strategy, unsafe successor selection, backup
boundary regression, or any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0454/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0454/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0454`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0454/qsl-protocol`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0454`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`7009a0c29f0d`. Fetch did not advance `origin/main`.

PR #1176 was verified MERGED at `7009a0c29f0d`.

Proof root:

`/srv/qbuild/tmp/NA0454_provider_rng_fake_seam_strategy_20260610T141556Z`

A recovered startup verifier issue was recorded: the first aggregate
queue/decision parser assumed helper output listed all historical NA and
decision IDs. That was a command-shape issue, not repo drift. Direct
`NEXT_ACTIONS.md` and `DECISIONS.md` verification passed.

## NA-0453 inheritance

NA-0453 / D-0893 selected:

`PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`

Inherited facts:

- qsc KEM keypair, KEM encap, signature keypair, X25519 keypair, identity
  bootstrap, signing, verify, and decap provider boundaries were classified.
- refimpl ML-KEM keypair, ML-KEM encap/decap, ML-DSA keypair/sign/verify,
  X25519 keypair, `Rng12`, qsp handshake, and qsp ratchet provider boundaries
  were classified.
- Existing qsc provider-error no-mutation evidence covers `pq_decap_failed`,
  but does not force concrete provider RNG failure for keypair, encap,
  signature keypair, or X25519 keypair operations.
- Existing refimpl provider fakes can model generic provider errors, but
  concrete RNG failure remains hidden behind `OsRng` or infallible trait
  methods.
- qsc provider-dependent RNG and refimpl/provider RNG remain unimplemented.

NA-0454 consumes that inheritance only as strategy authorization evidence. It
does not implement provider fakes, seams, runtime code, crypto code, or new
executable tests.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- provider RNG fake/seam strategy must remain bounded;
- qsc call sites remain distinct from refimpl/provider internals;
- least invasive proof mechanism is preferred;
- production semantics and fail-closed behavior must be preserved;
- No RNG-failure-complete claim is made;
- No provider-RNG-complete claim is made.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green;
- nested qsc fuzz lock audit is green;
- cfg RNG failure tests are green;
- route/contact/attachment seam evidence is green;
- qsc key lifecycle and provider-error tests are green;
- refimpl `pqkem768` remains a required validation proof;
- qsc adversarial CI or accepted check shape remains required;
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
- Provider RNG fake/seam strategy is internal governance evidence only.

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

## Existing fake / test-seam inventory

| Path | Purpose | Test-only | cfg-gated | Production behavior unchanged proof | Could force provider RNG failure? | Reusable for NA-0455? | Risks |
|---|---|---:|---:|---|---:|---:|---|
| `qsl/qsl-client/qsc/src/handshake/mod.rs` | Existing `qsc_rng_failure_test_seam` for qsc-local handshake session ID RNG | yes | yes | no-cfg `rng_failure_behavior` emits `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK` | no | partial | It reaches qsc-local `OsRng` for session IDs, not provider internals. |
| `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | Existing cfg seam for qsc session-store key/blob nonce RNG | yes | yes | no-cfg `rng_failure_behavior` proof | no | partial | It is local storage RNG, not KEM/signature/X25519 provider RNG. |
| `qsl/qsl-client/qsc/src/vault/mod.rs` | Existing cfg seam for vault salt/nonce/default route-token RNG | yes | yes | no-cfg `rng_failure_behavior` and `rng_failure_residual_surfaces` proof | no | partial | It should not be extended into provider semantics without exact scope. |
| `qsl/qsl-client/qsc/src/contacts/mod.rs` | Existing cfg seam for generated contact route-token RNG | yes | yes | no-cfg `rng_failure_residual_surfaces` proof | no | partial | It is local route-token RNG only. |
| `qsl/qsl-client/qsc/src/attachments/mod.rs` | Existing cfg seam for attachment ID, CEK, and nonce-prefix RNG | yes | yes | no-cfg `rng_failure_residual_surfaces` proof | no | partial | It is local attachment RNG only. |
| `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs` | qsc `pq_decap_failed` no-mutation proof | yes | no | normal test proof | no | partial | It proves a generic provider-error branch, not provider RNG failure. |
| `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` test module | Dummy KEM, signature, DH, and KT fakes for qsp handshake unit tests | yes | Rust `#[cfg(test)]` | test-only module | partial | partial | Fakes can return generic KEM/signature errors, but not concrete `OsRng` failure. |
| `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs` test module | Dummy DH/KEM and fixed RNG for qsp ratchet tests | yes | Rust `#[cfg(test)]` | test-only module | partial | partial | `Rng12` is infallible; fixed RNG is not a failure fake. |
| `tools/refimpl/quantumshield_refimpl/tests/na_0071_header_key_derivation.rs` | Dummy providers and fixed nonce helper for header tests | yes | no | test crate only | partial | partial | Models generic errors/fixed values, not provider RNG failure. |
| `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | Concrete provider wrappers using `OsRng` | no | feature-gated by provider features, not test seam | current tests exercise success and wrong-length errors | no | no without new strategy | Direct mutation here is crypto/provider boundary work and needs exact future scope. |
| `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | Provider traits for KEM, signature, X25519, and `Rng12` | no | feature/test derive only | current API shape | no | no without API decision | `X25519Dh::keypair` and `Rng12::random_nonce12` are infallible; changing them is an API/semantic decision. |

Gap summary:

- Existing qsc seams prove qsc-local RNG failures only.
- Existing provider fakes prove generic provider-error behavior only.
- Concrete provider RNG failure is not forceable through current public APIs.
- Provider RNG work needs a split-scope authorization lane before direct
  implementation.

## Strategy options review

Option 1 -- qsc-only fake provider strategy: rejected for direct implementation.
Evidence: qsc owns visible no-mutation call sites, but provider RNG failures
are hidden behind `StdCrypto`, `runtime_pq_kem_keypair()`,
`runtime_pq_sig_keypair()`, and `X25519Dh::keypair()`. Future exact paths are
not small enough without deciding refimpl/provider boundary shape. Stop if qsc
would need hidden provider fakes outside exact source paths. Public caveat:
No provider-RNG-complete claim is made.

Option 2 -- refimpl-only fake provider strategy: rejected for direct
implementation. Evidence: refimpl owns the provider traits/wrappers, but
refimpl-only proof would not establish qsc handshake/identity no-mutation.
Future exact paths may include `tools/refimpl/quantumshield_refimpl/src/crypto/`
and refimpl tests only after split-scope selection. Public caveat:
No RNG-failure-complete claim is made.

Option 3 -- combined qsc/refimpl fake strategy: rejected for immediate
implementation, selected for split-scope review. Evidence: it would be the
strongest evidence if exact, but current implementation would likely touch qsc
handshake, identity, main/TUI identity bootstrap call sites, refimpl traits, and
`StdCrypto`. That is too broad for direct authorization. Stop if the future
lane cannot split no-mutation and provider-boundary proof cleanly. Public
caveat: future tests would remain bounded evidence only.

Option 4 -- cfg-only provider RNG failure seam: selected as a candidate for
split-scope review, not implementation. Evidence: the existing qsc cfg seam is
proven inert in no-cfg builds, but provider boundary seams would need exact
placement and labels. Stop if the seam changes production provider semantics or
adds fallback-to-success behavior. Public caveat: No crypto-complete claim is
made.

Option 5 -- provider-error documentation only: rejected as the final strategy.
Evidence: documentation-only would preserve safety but leave a high-value
evidence gap for provider-dependent RNG failure. It remains a fallback if
split-scope cannot safely select implementation. Public caveat:
provider RNG evidence gaps remain gaps.

Option 6 -- move to next audit domain: rejected for now. Evidence: provider RNG
strategy is still high value after NA-0453, but direct implementation is not yet
exact. Candidate future domain remains `QSL KEM / Signature / Transcript
Binding Read-Only Audit Plan` if split-scope later rejects implementation.
Public caveat: no public security-readiness claim is created by deferring that
domain.

## qsc / refimpl split strategy decision

qsc and refimpl provider RNG failure strategy should be designed together at the
planning level, but qsc no-mutation proof and refimpl provider-boundary proof
should be split before implementation.

Answers:

- qsc and refimpl should be designed together: yes, as a coordinated strategy.
- qsc no-mutation proof should be separate from refimpl provider-boundary proof:
  yes.
- fake provider is preferable to cfg seam where generic provider errors are
  enough: yes.
- cfg-only seam may be needed for concrete `OsRng` failure: yes, but only after
  exact labels and source paths are authorized.
- exact traits/interfaces are already present: partial.
- implementation would require trait/API changes: partial to yes for
  infallible keypair and `Rng12` surfaces.
- implementation would be test-only: future proof must be test-only, but some
  source signatures may need exact authorization.
- production semantics can be proven unchanged only if the future lane keeps
  cfg/fake behavior unreachable from normal builds and runs no-cfg tests.
- smallest safe future scope is another split-scope authorization lane.

Strategy classifications:

- `PROVIDER_RNG_STRATEGY_QSC_ONLY`: rejected.
- `PROVIDER_RNG_STRATEGY_REFIMPL_ONLY`: rejected.
- `PROVIDER_RNG_STRATEGY_COMBINED_QSC_REFIMPL`: rejected for direct
  implementation; retained as a candidate after split-scope.
- `PROVIDER_RNG_STRATEGY_CFG_SEAM_ONLY`: rejected as sole strategy.
- `PROVIDER_RNG_STRATEGY_DOCUMENTATION_ONLY`: rejected as final strategy.
- `PROVIDER_RNG_STRATEGY_SPLIT_SCOPE_NEEDED`: selected.
- `PROVIDER_RNG_STRATEGY_AMBIGUOUS`: rejected because the next narrowing step is
  clear.

## Strategy authorization matrix

| Surface | Current path(s) | Provider/RNG operation | Current fake/seam availability | Future fake/seam strategy | Future mutable paths if selected | Existing API enough? | Requires trait/API change? | Requires cfg-only seam? | Production semantics risk | qsc no-mutation proof possible? | refimpl provider-boundary proof possible? | Priority | Selected for successor? | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| qsc KEM provider boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs`; call sites also exist in qsc main/TUI identity bootstrap | ML-KEM keypair and encap through refimpl helpers/`StdCrypto` | generic provider-error branch only | split-scope must decide qsc call-site proof separately from provider helper proof | not authorized by NA-0454 | partial | partial/yes for keypair fallibility | partial | medium | yes after exact qsc path selection | partial via refimpl | high | yes, for NA-0455 split-scope only | No provider-RNG-complete claim is made. | G1, G2, G3, G4 |
| qsc signature/identity provider boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs`; qsc main/TUI identity bootstrap | ML-DSA keypair and sign/verify | generic signature error handling only | split qsc identity write-order proof from refimpl signing/keypair proof | not authorized by NA-0454 | partial | partial/yes for keypair fallibility | partial | medium | yes after exact qsc path selection | partial via refimpl | high | yes, for NA-0455 split-scope only | No crypto-complete claim is made. | G1, G2, G3, G4 |
| qsc handshake provider RNG boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` | X25519 ephemeral keypair, KEM encap, sign | qsc cfg seam reaches session ID only; provider-error branches exist | split-scope must select whether provider fake or cfg seam is least invasive | not authorized by NA-0454 | partial | yes for infallible X25519 keypair if made fallible | partial/yes | medium | yes after exact labels/call sites | partial | high | yes, for NA-0455 split-scope only | No RNG-failure-complete claim is made. | G1, G2, G3, G4 |
| refimpl ML-KEM provider RNG boundary | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `traits.rs`; `tests/pqkem768.rs` | ML-KEM keypair and encap use `OsRng` | generic `PqKem768` fakes; no concrete RNG failure seam | split-scope must decide fake provider versus cfg seam in provider wrappers | not authorized by NA-0454 | partial | partial/yes for keypair helper | partial/yes | medium | no, refimpl-only | yes after exact path selection | high | yes, for NA-0455 split-scope only | Cargo audit green remains dependency-health evidence only. | G1, G2, G4 |
| refimpl ML-DSA provider RNG boundary | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `traits.rs` | ML-DSA keypair uses `OsRng`; sign/verify are Result-based | generic `PqSigMldsa65` fakes | split-scope must decide whether keypair helper can be made fallible or cfg-only | not authorized by NA-0454 | partial | partial/yes for keypair helper | partial/yes | medium | no, refimpl-only | yes after exact path selection | high | yes, for NA-0455 split-scope only | No vulnerability-free claim is made. | G1, G2, G4 |
| refimpl X25519 provider randomness boundary | `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `stdcrypto.rs` | `X25519Dh::keypair()` uses `OsRng` | dummy DH fixed keypair fakes, infallible | split-scope must decide if a fallible test-only wrapper is safer than trait change | not authorized by NA-0454 | no | yes if trait is changed | partial/yes | medium | partial through qsc call sites | yes after exact path selection | high | yes, for NA-0455 split-scope only | No perfect-crypto claim is made. | G1, G2, G4 |
| refimpl Rng12/random helper boundary | `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; `stdcrypto.rs`; qsp ratchet tests | `Rng12::random_nonce12()` uses `OsRng` | fixed RNG fakes only, infallible | split-scope must decide documentation, trait change, or cfg seam | not authorized by NA-0454 | no | yes if made fallible | partial/yes | medium | no | yes after exact path selection | medium | yes, for NA-0455 split-scope only | No RNG-failure-complete claim is made. | G2, G4 |
| qshield-cli demo RNG boundary | `apps/qshield-cli/` | demo-local random helpers | not inspected for mutation | keep out of provider RNG implementation lane | none | no | unknown | unknown | high if included | no | no | low | no | qshield-cli remains demo-local; No production-readiness claim is made. | G4, G5 |
| formal/model provider RNG residual | `formal/` | model abstraction residual | no provider RNG execution | documentation/backlog only unless future exact formal lane | none | no | no | no | low | no | no | low | no | No formal completeness claim is made. | G4 |
| fuzz/vector provider RNG residual | `qsl/qsl-client/qsc/fuzz/`; `inputs/` | fuzz/vector residual | fuzz lock healthy; no provider RNG seam | backlog/supporting only unless future exact lane | none | no | no | no | low | no | no | low | no | No public-readiness claim is made. | G4 |

## Authorization decision

Primary classification:

`PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED`

Selected successor:

`NA-0455 -- QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan`

Rationale:

- Existing provider fakes are insufficient for concrete provider RNG failure.
- Existing qsc cfg seams are useful precedent but do not reach provider
  internals.
- Combined qsc/refimpl implementation is not yet exact enough because it would
  likely require qsc source, refimpl traits, provider wrapper, and call-site
  decisions together.
- Documentation-only would leave the evidence gap without testing whether a
  bounded future implementation can be selected.
- Moving to the next audit domain is premature because provider RNG strategy is
  still a meaningful G4 verification gap.

This directive authorizes no implementation mutation.

This directive authorizes no runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, service, public,
backup, restore, or qwork tooling mutation.

## Successor selection

Selected exact NA-0455 successor:

`NA-0455 -- QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan`

NA-0455 must remain authorization-only unless a later directive authorizes exact
implementation paths. It must decide whether qsc no-mutation proof, refimpl
provider-boundary proof, or both can move to exact implementation lanes, and it
must keep exactly one READY item.

## Future path/scope bundle

Future allowed NA-0455 paths for the split-scope successor:

- `docs/governance/evidence/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_plan.md`
- `tests/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_testplan.md`
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

Future forbidden unless exact later scope authorizes:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- test source changes outside exact later paths;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs/website;
- qsl-server/qsl-attachments changes;
- backup/restore/qsl-backup changes;
- public claims.

If a later implementation successor is selected, it must list exact source and
test paths before mutation. Candidate implementation paths remain candidates
only and are not authorized by NA-0454.

## Future validation/marker plan

Common NA-0455 markers:

- `NA0455_PROVIDER_RNG_STRATEGY_CONSUMED_OK`
- `NA0455_NEXT_SCOPE_SELECTED_OK`
- `NA0455_NO_DEPENDENCY_CHANGE_OK`
- `NA0455_NO_WORKFLOW_CHANGE_OK`
- `NA0455_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0455_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0455_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0455_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0455_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0455_ONE_READY_INVARIANT_OK`
- `NA0455_PROVIDER_RNG_SPLIT_SCOPE_SELECTED_OK`

If NA-0455 selects a later qsc implementation successor, it must add markers
for qsc provider RNG fake/seam implementation and production semantics
unchanged.

If NA-0455 selects a later refimpl implementation successor, it must add markers
for refimpl provider RNG fake/seam implementation and production semantics
unchanged.

If NA-0455 selects a later combined implementation successor, it must add
markers for both qsc and refimpl provider RNG failure forced by test-only
boundaries and production semantics unchanged.

If NA-0455 selects documentation-only, it must emit
`NA0455_PROVIDER_RNG_DOCUMENTATION_ONLY_SELECTED_OK`.

## Public claim/external review/website boundary

Provider RNG fake/seam strategy authorization is internal governance evidence
only.

Provider RNG fake/seam strategy authorization is not production readiness.

Provider RNG fake/seam strategy authorization is not public-internet readiness.

Provider RNG fake/seam strategy authorization is not crypto-complete proof.

Provider RNG fake/seam strategy authorization is not side-channel-free proof.

Provider RNG fake/seam strategy authorization is not RNG-failure-complete proof.

Provider RNG fake/seam strategy authorization is not provider-RNG-complete
proof.

Provider RNG fake/seam strategy authorization is not bug-free proof.

Provider RNG fake/seam strategy authorization is not vulnerability-free proof.

Provider RNG fake/seam strategy authorization is not perfect-crypto proof.

Provider RNG fake/seam strategy authorization is not a public technical paper.

No README, START_HERE, public docs, or website update is made.

No public-readiness claim is made. No public-security claim is made.

Cargo audit green is dependency-health evidence, not vulnerability-free proof.

Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

- Direct combined implementation in NA-0455: rejected because source/test paths
  are not exact enough and may require trait/API changes.
- qsc-only implementation in NA-0455: rejected because refimpl/provider RNG
  boundary decisions would still be unresolved.
- refimpl-only implementation in NA-0455: rejected because qsc visible
  no-mutation proof would still be unresolved.
- cfg-only seam as the entire strategy: rejected because generic provider fakes
  may be less invasive for Result-returning provider operations.
- documentation-only now: rejected because split-scope can still narrow a safe
  bounded implementation path.
- next audit domain now: rejected because provider RNG remains a high-value
  verification gap after NA-0453.

## Backup-impact statement

No backup was run. No restore was run. No sudo command was run. No qsl-backup,
backup status, backup plan, rollback subtree, timer, fstab, retention,
source-list, or backup tree path was mutated.

Read-only qsl-backup boundary proof matched expected short SHA
`e9ecff3d22ed`, and the codex ops source inclusion count remained one.

## Next recommendation

Close NA-0454 after the evidence PR merges and public-safety is green, then
restore NA-0455 as the sole READY item:

`QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan`

NA-0455 should preserve no-runtime/no-crypto/no-dependency/no-workflow and
no-public-overclaim boundaries while selecting exact later implementation scope
or rejecting implementation with documentation-only rationale.
