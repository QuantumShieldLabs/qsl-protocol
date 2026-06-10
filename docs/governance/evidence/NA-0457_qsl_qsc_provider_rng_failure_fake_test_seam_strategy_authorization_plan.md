Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0457 QSL qsc Provider RNG Failure Fake / Test Seam Strategy Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0457 consumes NA-0456 and chooses the first qsc provider-dependent RNG
failure fake/test-seam strategy.

Primary classification:

`QSC_PROVIDER_RNG_KEM_FAKE_SEAM_IMPLEMENTATION_READY`

Selected successor:

`NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`

Reason: the qsc KEM keypair and responder KEM encapsulation surfaces are exact
enough for a small cfg-only call-site seam. The future seam can force bounded
KEM provider RNG failure at qsc-visible boundaries without refimpl mutation,
trait/API mutation, dependency mutation, workflow mutation, or production
semantic change when the cfg is inactive. Existing qsc no-mutation harnesses
can be reused for vault, pending, session, and relay-output assertions.

Combined KEM/signature/identity implementation is rejected for the next lane.
Signature and broader identity work still need separate authorization because
forced `sig_sign_failed` is not obviously no-mutation in all qsc paths today;
the initiator A2 signing branch is reached after session store and pending
clear. Refimpl provider RNG remains deferred and is not required before the
KEM-only qsc implementation can produce bounded qsc no-mutation evidence.

NA-0457 authorizes no implementation mutation. It changes no runtime behavior,
crypto behavior, dependencies, Cargo manifests, lockfiles, workflows,
executable tests, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
Cargo audit green remains dependency-health evidence only.

Required markers recorded by this evidence:

- `NA0457_QSC_PROVIDER_RNG_STRATEGY_CONSUMED_OK`
- `NA0457_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0457_NA0456_INHERITANCE_CONSUMED_OK`
- `NA0457_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0457_QSC_FAKE_SEAM_INVENTORY_OK`
- `NA0457_STRATEGY_MECHANISM_REVIEW_OK`
- `NA0457_QSC_KEM_PROVIDER_CFG_SEAM_READY_OK`
- `NA0457_QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_OK`
- `NA0457_COMBINED_REJECTED_KEM_ONLY_SELECTED_OK`
- `NA0457_QSC_STRATEGY_MATRIX_OK`
- `NA0457_QSC_KEM_IMPLEMENTATION_READY_SELECTED_OK`
- `NA0457_SUCCESSOR_NA0458_SELECTED_OK`
- `NA0457_NO_RUNTIME_CHANGE_OK`
- `NA0457_NO_CRYPTO_CHANGE_OK`
- `NA0457_NO_DEPENDENCY_CHANGE_OK`
- `NA0457_NO_WORKFLOW_CHANGE_OK`
- `NA0457_NO_TEST_IMPLEMENTATION_OK`
- `NA0457_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0457_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0457_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0457_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0457_ONE_READY_INVARIANT_OK`

## Live NA-0457 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0457.
- NA-0456 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0900.
- D-0899 exists once.
- D-0900 exists once.
- D-0901 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0457 mutation paths are exactly:

- `docs/governance/evidence/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
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
- NA-0456 qsc fake/seam strategy inheritance is consumed;
- qsc fake/test-seam candidates are inventoried;
- strategy mechanism, KEM, signature/identity, and combined/split reviews are
  completed;
- one primary classification and one NA-0458 successor are selected;
- exact future scope is recorded if implementation-ready;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1182
not merged, missing public-safety, missing D-0900, D-0901 already present,
failed root or nested audit, unconsumable NA-0456 inheritance, unclassifiable
fake/seam strategy, unsafe successor selection, backup boundary regression, or
any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0457/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0457/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0457`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0457/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0457`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, clean-state fields, READY count, queue top READY, and requested
lane status.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`7b04809fac1d`. Fetch did not advance `origin/main`.

PR #1182 was verified merged at `7b04809fac1d`.

Proof root:

`/srv/qbuild/tmp/NA0457_qsc_provider_rng_fake_seam_strategy_20260610T231832Z`

## NA-0456 inheritance

NA-0456 / D-0899 selected:

`QSC_PROVIDER_RNG_NO_MUTATION_REQUIRES_FAKE_SEAM_STRATEGY`

Inherited facts:

- qsc KEM decap / `pq_decap_failed` is already covered generically by
  provider-error no-mutation evidence and is not RNG-specific proof.
- qsc KEM encap / `pq_encap_failed` remains defensive and not forceable through
  current APIs.
- qsc KEM keypair/provider generation remains a high-priority residual.
- qsc signature signing / `sig_sign_failed` remains a high-priority residual.
- qsc signature verification / `sig_invalid` has useful invalid-signature
  evidence but lower RNG relevance.
- qsc identity bootstrap/provider generation remains a high-priority residual.
- qsc X25519/ephemeral generation remains residual.
- refimpl provider RNG remains deferred.
- implementation is not authorized until the fake/seam strategy is chosen.

NA-0457 consumes that inheritance only as strategy authorization evidence. It
does not implement provider fakes, cfg seams, runtime code, crypto code, or new
executable tests.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- choose the least-invasive qsc provider RNG fake/test-seam strategy;
- preserve qsc/refimpl separation;
- preserve production semantics when the cfg seam is inactive;
- separate provider-error surface coverage from provider RNG failure evidence;
- do not convert the qsc KEM seam strategy into provider-RNG-complete proof.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green;
- nested qsc fuzz lock audit is green;
- cfg RNG failure tests are green;
- route/contact/attachment seam evidence is green;
- qsc key lifecycle and provider-error tests are green;
- refimpl `pqkem768` is green;
- qsc adversarial CI is green on current main and local script syntax is green;
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
- qsc provider RNG fake/seam strategy authorization is internal governance
  evidence only.

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

## qsc fake / test-seam strategy inventory

| Candidate | Exact path | Provider operation | Fake/seam candidate | Existing tests usable? | Production semantics risk | Requires qsc source change? | Requires refimpl change? | Requires trait/API change? | cfg-only? | test-only? | Can prove no-mutation? | Recommendation |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| KEM keypair | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `runtime_pq_kem_keypair()` via `hs_kem_keypair()` during identity bootstrap / handshake identity load | cfg-only qsc call-site failure label before provider call | partial; qsc vault/session harness patterns reusable | low if cfg-inactive normal tests prove unchanged | yes, future only | no | no | yes | yes | yes, at identity/vault/pending boundaries | selected for KEM-only successor |
| KEM encap | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::encap()` in responder poll | cfg-only qsc call-site failure label returning `pq_encap_failed` before DH/session/pending writes | partial; provider-error and handshake no-mutation harness patterns reusable | low if cfg-inactive normal tests prove unchanged | yes, future only | no | no | yes | yes | yes, around responder pending/session/relay outputs | selected for KEM-only successor |
| KEM decap | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::decap()` in initiator finalize | reuse existing generic provider-error proof | yes | low | no for current generic proof | no | no | n/a | yes | already for generic provider error | not selected for RNG-specific KEM scope |
| Signature signing | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for B1 and A2 | cfg-only qsc call-site failure label | partial | medium/high; initiator A2 sign occurs after session store and pending clear | yes, future only | no | no | yes | yes | not safely for all paths without more triage | reject for first successor; split later |
| Signature verification | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::verify()` for B1/A2 | reuse invalid-signature and optional provider-error seam later | yes for `sig_invalid` | low | maybe later | no | no | partial | yes | yes for invalid signature, lower RNG relevance | reject for first successor |
| Signature keypair | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | `runtime_pq_sig_keypair()` via `hs_sig_keypair()` | cfg-only qsc call-site failure label | partial | medium; identity migration/bootstrap ordering needs separate proof | yes, future only | no | no | yes | yes | likely, but needs separate signature/identity scope | split later |
| Identity bootstrap | `qsl/qsl-client/qsc/src/identity/mod.rs` | generated KEM and signature identity keys plus secret/public writes | KEM-only cfg label now; signature labels later | partial | medium; broad identity scope mixes KEM and signature | yes, future only | no | no | yes | yes | yes for KEM keypair only | selected only for KEM keypair failure |
| X25519 ephemeral | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::keypair()` through `hs_ephemeral_keypair()` | cfg-only qsc call-site failure label or later refimpl boundary | partial | medium | yes, future only | no for qsc seam; yes for provider internals | no for qsc seam | yes | yes | yes, but outside KEM-only scope | defer |
| Fake provider injection | qsc handshake/identity plus provider construction sites | injectable provider object for KEM/signature/DH | fake provider trait/constructor injection | no | medium/high; broad API refactor risk | yes | maybe | yes | no | yes | yes if built, but too broad | reject for first successor |
| Refimpl provider fake | `tools/refimpl/quantumshield_refimpl/src/crypto/**` | provider wrappers / traits | refimpl-level fake or fallible RNG boundary | partial refimpl fakes only | medium; provider/trait semantics | no qsc first | yes | maybe | partial | yes | provider-boundary only, not qsc state | defer |

Inventory conclusion:

- A qsc cfg-only KEM seam is the least invasive implementation-ready strategy.
- A fake provider or trait injection strategy would touch broader APIs and is
  not needed before bounded qsc KEM no-mutation evidence.
- Existing provider-error harnesses remain useful background evidence but are
  insufficient for concrete provider RNG failure.
- Documentation-only handling is too weak for the next lane because KEM
  call-site forcing is now exact enough.

## Strategy mechanism review

Option 1 -- qsc cfg-only provider failure labels: selected for KEM-only
implementation. Evidence: existing `qsc_rng_failure_test_seam` infrastructure
already proves cfg-inactive normal behavior for local qsc RNG failures. Future
KEM labels can be placed at exact qsc-visible provider boundaries:
`hs_kem_keypair()` / identity bootstrap and responder `StdCrypto::encap()`.
Future paths: `qsl/qsl-client/qsc/src/handshake/mod.rs`,
`qsl/qsl-client/qsc/src/identity/mod.rs`, and
`qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`. Validation must run
cfg and no-cfg qsc tests and prove production semantics unchanged. Public
caveat: bounded qsc KEM evidence only, not provider-RNG-complete proof.

Option 2 -- qsc fake provider injection: rejected for the next lane. Evidence:
injecting a fake provider would require constructor/trait/API decisions across
KEM, signature, DH, and identity code that are broader than the KEM proof need.
Future exact paths are not selected. Validation would need broader qsc/refimpl
contract coverage. Public caveat: fake-provider tests would still be bounded
evidence only.

Option 3 -- reuse existing generic provider-error paths: rejected as sufficient
strategy. Evidence: `pq_decap_failed` no-mutation is covered, but KEM keypair
and KEM encap RNG failure are not forceable today. Future validation may keep
the existing test as inherited evidence. Public caveat: generic provider-error
coverage is not RNG-specific proof.

Option 4 -- refimpl-backed fake later: rejected as a prerequisite for the KEM
qsc implementation. Evidence: qsc can prove bounded no-mutation at exact KEM
call sites with a cfg-only seam while preserving qsc/refimpl separation.
Refimpl provider-boundary proof remains valuable future work. Public caveat:
qsc KEM no-mutation evidence will not prove provider internals.

Option 5 -- documentation-only: rejected. Evidence: KEM keypair and encap
call-site forcing is exact enough for a bounded future implementation; stopping
at documentation would leave a high-value evidence gap unnecessarily. Public
caveat: all remaining gaps must stay explicit.

Option 6 -- split KEM/signature/identity strategy before implementation:
selected in the narrower KEM-only form. Evidence: KEM can be implemented next;
signature/identity and X25519 should not be bundled because they need separate
state-ordering and provider-boundary review. Public caveat: KEM-only evidence
does not complete qsc provider RNG evidence.

## KEM strategy review

KEM provider failure can be forced safely with a qsc cfg-only label at the call
site for the first future implementation. The exact qsc paths are clear:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`

KEM provider failure can also be modeled with a fake provider in principle, but
that would require broader injection/API work and is rejected for the first
implementation.

KEM keypair generation is provider-backed in refimpl, but qsc owns the visible
state boundary where identity/vault/public-record mutations begin. A qsc
cfg-only seam can therefore prove bounded qsc no-mutation for forced KEM
keypair failure without claiming provider-internal RNG proof.

KEM should be the first implementation scope because:

- KEM keypair and encap are randomness-dependent provider operations;
- responder encap failure occurs before responder pending/session writes;
- KEM keypair failure can be forced before identity secret/public writes;
- future source and test paths are exact;
- the scope is smaller than combined KEM/signature/identity work.

Classification:

`QSC_KEM_PROVIDER_CFG_SEAM_READY`

Not selected for this lane:

- `QSC_KEM_PROVIDER_FAKE_SEAM_READY` because fake provider injection is broader.
- `QSC_KEM_PROVIDER_REUSE_GENERIC_ERROR_ONLY` because existing proof is not
  enough.
- `QSC_KEM_PROVIDER_NEEDS_REFIMPL_FIRST` because qsc call-site proof can proceed.
- `QSC_KEM_PROVIDER_SPLIT_FURTHER` because KEM-only is exact enough.
- `QSC_KEM_PROVIDER_DOCUMENTATION_ONLY` because implementation-ready evidence
  exists.
- `QSC_KEM_PROVIDER_AMBIGUOUS` because exact paths and mechanism are clear.

## Signature / identity strategy review

Signature signing failure can be forced with a qsc cfg-only label in principle,
but it should not be bundled into the first implementation. The B1 signing path
appears before responder pending write, while the A2 signing path is reached
after initiator session store and pending clear. That means a forced
`sig_sign_failed` no-mutation claim needs separate state-ordering review before
implementation.

Identity provider generation failure can be forced with qsc cfg-only labels in
principle, but identity bootstrap mixes KEM keypair, signature keypair, vault
secret stores, and public-record writes. NA-0458 may cover only KEM keypair
failure in that path. Signature keypair failure and legacy migration should be
separate future signature/identity scope.

Classification:

`QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER`

Rejected classifications:

- `QSC_SIGNATURE_IDENTITY_FAKE_SEAM_READY` because fake provider injection is
  too broad.
- `QSC_SIGNATURE_IDENTITY_CFG_SEAM_READY` for immediate implementation because
  state-ordering around `sig_sign_failed` needs separate review.
- `QSC_SIGNATURE_IDENTITY_REUSE_GENERIC_ERROR_ONLY` because existing
  `sig_invalid` proof is not signing failure proof.
- `QSC_SIGNATURE_IDENTITY_NEEDS_REFIMPL_FIRST` because qsc call-site strategy
  remains plausible later.
- `QSC_SIGNATURE_IDENTITY_DOCUMENTATION_ONLY` because the residual is actionable
  but not first.
- `QSC_SIGNATURE_IDENTITY_AMBIGUOUS` because the reason to split is concrete.

## Combined vs split strategy decision

Option 1 -- combined qsc provider fake/seam implementation next: rejected.
Evidence: KEM, signature, identity, and X25519 do not share one equally safe
small mechanism. Future validation would be too broad. Public caveat: combined
scope would still not prove provider-RNG-complete behavior.

Option 2 -- KEM-only qsc provider fake/seam implementation next: selected.
Evidence: KEM keypair and responder encap have exact call sites and a small
cfg-only forcing mechanism. Future paths are known. Validation must prove
forced failure no-mutation and cfg-inactive normal behavior. Public caveat:
bounded qsc KEM evidence only.

Option 3 -- signature/identity qsc provider fake/seam implementation next:
rejected for the first successor. Evidence: signing and identity-generation
state ordering need separate review, especially A2 signing after state writes.
Public caveat: signature/identity residual remains.

Option 4 -- qsc provider fake/seam strategy split further: rejected as the
primary next lane because KEM-only is already exact enough. Signature/identity
may become a later split lane. Public caveat: KEM-only does not complete the
provider RNG strategy.

Option 5 -- refimpl-first: rejected for the first successor. Evidence: qsc KEM
state no-mutation can be proven without refimpl mutation. Public caveat:
refimpl provider RNG remains residual.

Option 6 -- documentation-only: rejected because KEM-only implementation is
available and bounded. Public caveat: all gaps remain explicit.

Option 7 -- next audit domain: rejected because provider RNG KEM no-mutation is
more direct and exact for the next lane. Public caveat: KEM/signature/transcript
binding audit remains a possible later domain.

## qsc fake/seam strategy matrix

| Candidate strategy | Surface(s) | Exact candidate paths | Mechanism | Existing tests/fakes | Future mutation type | Production-semantics risk | Evidence value | Scope size | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---:|---:|---|---|---|
| combined qsc KEM/signature/identity cfg seam | KEM, signing, identity | `handshake/mod.rs`; `identity/mod.rs`; qsc tests | cfg labels at all provider call sites | partial qsc seam harnesses | qsc source + tests | medium | high but broad | large | yes | no | A2 signing state ordering needs separate review | not provider-RNG-complete proof | G1-G5 |
| combined qsc fake provider injection | KEM, signing, identity, DH | qsc provider construction paths plus tests | fake provider/trait injection | no direct qsc fake | qsc/refimpl API shape | high | high if complete | large | yes | no | too broad for first implementation | bounded fake evidence only | G1-G5 |
| KEM-only cfg seam | KEM keypair and encap | `handshake/mod.rs`; `identity/mod.rs`; `tests/kem_provider_rng_failure.rs` | cfg-only qsc failure labels | reusable qsc no-mutation harnesses | qsc source + test | low when cfg inactive | high for qsc KEM no-mutation | small | no | yes | exact call sites and no refimpl/API mutation | bounded qsc KEM evidence only | G1-G5 |
| KEM-only fake provider | KEM keypair and encap | qsc/refimpl provider boundaries plus qsc tests | fake provider injection | partial refimpl fakes | API/trait source + tests | medium | high | medium | yes | no | cfg-only is smaller | bounded fake evidence only | G1-G5 |
| signature/identity cfg seam | signing and identity keypairs | `handshake/mod.rs`; `identity/mod.rs`; qsc tests | cfg labels | invalid-signature tests only | qsc source + tests | medium | high later | medium | yes | no | A2 sign failure ordering needs review | not RNG-failure-complete proof | G1-G5 |
| signature/identity fake provider | signing and identity keypairs | qsc/refimpl provider paths plus tests | fake provider injection | no direct qsc fake | API/trait source + tests | high | high later | large | yes | no | too broad | bounded fake evidence only | G1-G5 |
| identity-bootstrap-only seam | identity KEM/signature keypair generation | `identity/mod.rs`; qsc identity tests | cfg labels before stores | partial vault harnesses | qsc source + tests | medium | medium/high | medium | yes | no | KEM-only can cover first identity KEM failure; signature identity needs later lane | not provider-RNG-complete proof | G1-G5 |
| reuse generic provider-error evidence | decap and invalid signature | existing qsc tests | no new seam | yes | none | low | medium | small | no | no | insufficient for KEM keypair/encap RNG | generic provider-error only | G1-G5 |
| refimpl-first | provider internals | `tools/refimpl/**` | provider fake/fallible RNG boundary | partial | refimpl source + tests | medium | provider-boundary value | medium | yes | no | not needed before qsc KEM state proof | not qsc no-mutation completion | G1-G5 |
| documentation-only | all residuals | governance docs only | record gap | n/a | docs only | low | low | small | no | no | KEM-only is actionable | gaps remain gaps | G1-G5 |
| next audit domain | KEM/signature/transcript | docs/tests later | read-only audit | existing tests | docs only | low | medium later | medium | no | no | provider RNG KEM is more direct now | no public-readiness claim | G1-G5 |

## Authorization decision

Primary classification:

`QSC_PROVIDER_RNG_KEM_FAKE_SEAM_IMPLEMENTATION_READY`

Selected implementation mechanism:

- qsc cfg-only KEM provider RNG failure labels.
- No fake provider injection in the first implementation.
- No trait/API mutation.
- No refimpl mutation.
- No dependency, Cargo, lockfile, workflow, fuzz target, vector, or formal model
  mutation.

Exact future implementation paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future implementation must limit qsc source mutation to test-only cfg-gated
KEM provider failure forcing. The seam must be inert when
`qsc_rng_failure_test_seam` is not configured. It must not change production
wire, crypto, auth, state-machine, or persistence semantics.

Refimpl does not need to be touched before this qsc KEM implementation can be
meaningful. The future proof will be bounded qsc state no-mutation evidence at
the qsc call-site boundary; it will not prove provider-internal RNG behavior.

## Successor selection

Selected exact successor:

`NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`

NA-0458 must implement only the KEM-only qsc cfg seam strategy selected by
NA-0457. It must not implement signature/identity signing seams, X25519 seams,
refimpl provider RNG fakes, dependency changes, workflow changes, public docs,
public website content, backup/restore changes, or public-claim expansion.

## Future path/scope bundle

Future NA-0458 allowed paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0458 forbidden scope unless a later exact directive changes it:

- dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model,
  qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore,
  qsl-backup, backup status, backup plan, rollback, and backup tree mutation;
- unrelated qsc/runtime/crypto paths outside the exact KEM source/test scope;
- no public-readiness claim, no production-readiness claim, no
  public-internet-readiness claim, no external-review-complete claim, no
  crypto-complete claim, no side-channel-free claim, no RNG-failure-complete
  claim, no provider-RNG-complete claim, no vulnerability-free claim, no
  bug-free claim, and no perfect-crypto claim.

## Future validation/marker plan

Common NA-0458 markers:

- `NA0458_QSC_PROVIDER_RNG_STRATEGY_CONSUMED_OK`
- `NA0458_NEXT_SCOPE_SELECTED_OK`
- `NA0458_NO_DEPENDENCY_CHANGE_OK`
- `NA0458_NO_WORKFLOW_CHANGE_OK`
- `NA0458_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0458_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0458_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0458_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0458_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0458_ONE_READY_INVARIANT_OK`
- `NA0458_QSC_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0458_QSC_PROVIDER_RNG_FAILURE_NO_MUTATION_OK`
- `NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0458_QSC_KEM_PROVIDER_RNG_FAKE_SEAM_IMPLEMENTED_OK`

Required future validation:

- cfg-gated qsc KEM provider RNG failure test;
- no-cfg qsc KEM provider RNG failure test proving seam inactivity;
- inherited qsc `rng_failure_residual_surfaces` cfg/no-cfg tests;
- inherited qsc `rng_failure_behavior` cfg/no-cfg tests;
- qsc `key_lifecycle_zeroization`;
- qsc `handshake_provider_error_no_mutation`;
- qsc `send_commit`;
- refimpl `pqkem768`;
- root cargo audit and nested qsc fuzz lock audit;
- qsc adversarial script syntax and required qsc adversarial CI;
- formal checks if unchanged but required by directive.

## Public claim/external review/website boundary

qsc provider RNG fake/seam strategy authorization is internal governance
evidence only.

qsc provider RNG fake/seam strategy authorization is not production readiness.

qsc provider RNG fake/seam strategy authorization is not public-internet
readiness.

qsc provider RNG fake/seam strategy authorization is not crypto-complete proof.

qsc provider RNG fake/seam strategy authorization is not side-channel-free
proof.

qsc provider RNG fake/seam strategy authorization is not RNG-failure-complete
proof.

qsc provider RNG fake/seam strategy authorization is not provider-RNG-complete
proof.

qsc provider RNG fake/seam strategy authorization is not bug-free proof.

qsc provider RNG fake/seam strategy authorization is not vulnerability-free
proof.

qsc provider RNG fake/seam strategy authorization is not perfect-crypto proof.

qsc provider RNG fake/seam strategy authorization is not a public technical
paper.

No README, START_HERE, public docs, or website update is made.

No public-readiness or public-security claim is made.

Cargo audit green is dependency-health evidence, not vulnerability-free proof.

Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

Combined qsc KEM/signature/identity cfg seam was rejected because signature
signing and identity signature generation need separate state-ordering review.

Combined qsc fake provider injection was rejected because it would require
broader trait/API or constructor changes than needed for KEM no-mutation proof.

Refimpl-first was rejected as a prerequisite because qsc KEM state-boundary
proof can proceed without refimpl mutation.

Documentation-only was rejected because KEM-only implementation is exact enough
and higher evidence value.

Next audit domain was rejected because qsc KEM provider RNG no-mutation is the
more direct next blocker.

## Backup-impact statement

No backup or restore was run. No qsl-backup, backup status, backup plan,
rollback subtree, backup tree, timer, fstab, systemd, source-list, retention, or
local-ops path was mutated. The qsl-backup SHA matched the required boundary
value and the script-local ops source inclusion count remained 1.

## Next recommendation

Close NA-0457 after merge and restore:

`NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`

The future lane should implement only the KEM-only cfg seam selected here and
should leave signature/identity, X25519, refimpl provider RNG, and public claim
work as residuals.
