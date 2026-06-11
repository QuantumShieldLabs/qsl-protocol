Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0459 QSL qsc Signature / Identity Provider RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0459 consumes NA-0458 and classifies the remaining qsc signature / identity
provider RNG failure scope after the KEM-only provider RNG seam landed.

Primary classification:

`QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_NEEDED`

Selected successor:

`NA-0460 -- QSL qsc Signature / Identity Provider RNG Failure Split-Scope Authorization Plan`

Reason: the relevant qsc paths are identifiable, but they are not safe to
combine into one implementation lane. Signature signing and identity bootstrap
have different state-mutation timing and different evidence gaps. B1 signing is
before responder pending/session output mutation, but A2 signing occurs after
the initiator session store and pending clear. Identity bootstrap also spans
lazy identity creation, legacy/public-record upgrade, explicit identity rotate,
and TUI account initialization paths. Signature verification / `sig_invalid` is
useful background reject evidence but is not RNG-relevant. X25519/ephemeral
generation is qsc-visible but should remain a separate backlog surface, not
bundled with signature/identity.

The least invasive future mechanism remains a qsc cfg-only call-site seam, but
only after NA-0460 splits the exact signing, identity-bootstrap, and X25519
surfaces into separate future implementation candidates. Fake provider or trait
injection is rejected for the first next step because it would be broader than
the current qsc-local evidence need. Refimpl-first work is deferred because qsc
can still reason about qsc-local no-mutation boundaries without mutating
refimpl.

NA-0459 authorizes no implementation mutation. It changes no runtime behavior,
crypto behavior, dependencies, Cargo manifests, lockfiles, workflows,
executable tests, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
No signature-complete claim is made. No identity-complete claim is made. No crypto-complete claim is made.
Cargo audit green remains dependency-health evidence only.

Required markers recorded by this evidence:

- `NA0459_SIGNATURE_IDENTITY_SCOPE_AUTHORIZATION_CONSUMED_OK`
- `NA0459_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0459_NA0458_INHERITANCE_CONSUMED_OK`
- `NA0459_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0459_SIGNATURE_IDENTITY_TARGET_INVENTORY_OK`
- `NA0459_SIGNATURE_SIGNING_REVIEW_OK`
- `NA0459_IDENTITY_BOOTSTRAP_REVIEW_OK`
- `NA0459_SIGNATURE_VERIFY_BACKGROUND_ONLY_OK`
- `NA0459_X25519_BACKLOG_OK`
- `NA0459_SIGNATURE_IDENTITY_SPLIT_FURTHER_SELECTED_OK`
- `NA0459_SUCCESSOR_NA0460_SELECTED_OK`
- `NA0459_NO_RUNTIME_CHANGE_OK`
- `NA0459_NO_CRYPTO_CHANGE_OK`
- `NA0459_NO_DEPENDENCY_CHANGE_OK`
- `NA0459_NO_WORKFLOW_CHANGE_OK`
- `NA0459_NO_TEST_IMPLEMENTATION_OK`
- `NA0459_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0459_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0459_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0459_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0459_ONE_READY_INVARIANT_OK`

## Live NA-0459 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0459.
- NA-0458 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0904.
- D-0903 exists once.
- D-0904 exists once.
- D-0905 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0459 mutation paths are exactly:

- `docs/governance/evidence/NA-0459_qsl_qsc_signature_identity_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0459_qsl_qsc_signature_identity_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, refimpl,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback, and backup tree paths.

Acceptance criteria:

- qwork proof files are verified without rerunning qwork;
- NA-0458 KEM-only inheritance is consumed;
- exact qsc signature / identity candidate surfaces are inventoried;
- signing, identity bootstrap, verification, and X25519 reviews are completed;
- combined versus split decision is recorded;
- one primary classification and one NA-0460 successor are selected;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1186
not merged, missing public-safety, missing D-0904, D-0905 already present,
failed root or nested audit, unconsumable NA-0458 inheritance, unsafe
signature/identity scope classification, unsafe successor selection, backup
boundary regression, or any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0459/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0459/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0459`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0459/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0459`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, clean-state fields, READY count, queue top READY, and requested
lane status.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`a2712d05b1e2`. Fetch did not advance `origin/main`.

PR #1186 was verified merged at `a2712d05b1e2`.

Proof root:

`/srv/qbuild/tmp/NA0459_qsc_signature_identity_provider_rng_scope_20260611T035633Z`

## NA-0458 inheritance

NA-0458 / D-0903 implemented a qsc-local cfg-only KEM provider RNG failure
test seam for:

- `QSC.KEM.KEYPAIR`
- `QSC.KEM.ENCAP`

Inherited facts:

- KEM keypair forced failure is proven before selected identity, vault,
  pending, session, and A1 output mutation.
- KEM encap forced failure is proven before selected responder pending/session
  state and B1 output mutation.
- Normal no-cfg builds ignore the seam selector and preserve production
  semantics for the tested paths.
- `pq_decap_failed` remains generic provider-error background evidence, not
  RNG-specific proof.
- `pq_encap_failed` remains bounded forced-seam evidence, not an external
  triggerability claim.
- signature/identity provider RNG remains residual.
- X25519 provider RNG remains residual.
- refimpl provider RNG remains deferred.
- no KEM-complete claim exists;
- no RNG-failure-complete claim exists;
- no provider-RNG-complete claim exists;
- no crypto-complete claim exists;
- no public-readiness claim exists.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- signature/identity provider RNG scope must remain bounded;
- KEM and refimpl work must not be bundled into this qsc scope;
- qsc/refimpl separation remains preserved;
- future seams must be cfg-only/test-only unless later exact scope says
  otherwise;
- production semantics and public-claim caveats remain preserved.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green;
- nested qsc fuzz lock audit is green;
- KEM provider RNG seam tests are green;
- cfg RNG failure tests are green;
- qsc key lifecycle and provider-error tests are green;
- refimpl `pqkem768` is green;
- qsc adversarial CI is green on current main and local script syntax is green;
- public-safety is green on current main;
- cargo audit green is dependency-health evidence only, not vulnerability-free
  proof.

Public Claims / External Review Steward:

- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- qsc signature/identity scope authorization is internal governance evidence
  only.

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

## signature / identity provider RNG target inventory

| Candidate surface | Exact source path | Provider operation | Failure type | Current marker | Current state mutation risk | Existing tests | Current APIs enough? | Future seam/fake need | Refimpl mutation needed? | Priority |
|---|---|---|---|---|---|---|---|---|---|---|
| signing failure / B1 | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for B1 | provider signing error; current refimpl sign path does not show explicit RNG use | `sig_sign_failed` | lower; signing happens before responder pending store, session store, and B1 output | B1 tamper verifies `sig_invalid`, not forced signing failure | no | cfg-only qsc call-site seam | no for qsc-local proof | high |
| signing failure / A2 | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for A2 | provider signing error; current refimpl sign path does not show explicit RNG use | `sig_sign_failed` | high; A2 signing happens after initiator session store and pending clear | A2 tamper verifies `sig_invalid`, not forced signing failure | no | cfg-only qsc call-site seam plus separate state-timing decision | no for qsc-local proof | high |
| invalid signature | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::verify()` for B1 and A2 | invalid signature or provider verify error, not RNG-dependent | `sig_invalid` | low for tested tamper paths | `handshake_mvp.rs` B1/A2 signature tamper no-mutation tests | yes for invalid signature | no for RNG scope | no | background |
| lazy identity signature keypair | `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` through `runtime_pq_sig_keypair()` | provider RNG currently infallible from qsc perspective | none | medium/high; signature secret/public record writes follow generation | success and key lifecycle evidence only | no | cfg-only qsc keypair seam | no for qsc-local proof | high |
| legacy/public-record identity upgrade | `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` for legacy migration or missing `sig_pk` public record | provider RNG currently infallible from qsc perspective | none | medium/high; migration writes secrets and public record | success and key lifecycle evidence only | no | cfg-only qsc keypair seam | no for qsc-local proof | high |
| explicit identity rotate | `qsl/qsl-client/qsc/src/main.rs` | `hs_kem_keypair()` and `hs_sig_keypair()` | provider RNG currently infallible from qsc perspective | none for provider RNG | high; KEM and signature secret/public record writes occur in command path | identity success tests and key lifecycle evidence only | no | split-scope decision needed because this path is outside the narrow identity module surface | no for qsc-local proof | high |
| TUI account bootstrap identity | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | `hs_kem_keypair()` and `hs_sig_keypair()` | provider RNG currently infallible from qsc perspective | none for provider RNG | high; account defaults and identity writes share initialization flow | TUI account success/retired-provider tests only | no | split-scope decision needed because this path is a separate TUI/account surface | no for qsc-local proof | high |
| X25519/ephemeral generation | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::keypair()` in `hs_ephemeral_keypair()` | provider RNG currently infallible from qsc perspective | none | medium; initiator generation precedes pending store, responder generation precedes responder pending/session store | handshake success and tamper tests only | no | separate cfg-only qsc seam if later selected | no for qsc-local proof | backlog |

Inventory conclusion:

- qsc signing, identity, verification, and X25519 call sites are exact enough to
  discuss.
- current APIs cannot force signature keypair, signature signing, or X25519
  provider failure at qsc boundaries.
- identity scope is broader than the narrow `identity/mod.rs` path because
  explicit CLI rotate and TUI account bootstrap also generate identity keys.
- verification is not RNG-relevant and should remain background evidence.
- future implementation scope should be selected only after NA-0460 splits the
  exact surfaces.

## signature signing failure review

qsc signs handshake material in `qsl/qsl-client/qsc/src/handshake/mod.rs`:

- B1 responder signing uses `StdCrypto::sign()` and emits
  `sig_sign_failed` if signing fails before responder pending/session store and
  B1 relay output.
- A2 initiator signing uses `StdCrypto::sign()` and emits
  `sig_sign_failed` if signing fails after the initiator has stored the session
  and cleared pending state.

`sig_sign_failed` is present and sanitized. Current qsc APIs cannot force this
failure. A future cfg-only call-site seam could force it without provider trait
or refimpl mutation, but a single signing implementation lane would need to
split B1 from A2 or explicitly document the A2 already-mutated state boundary.

Classification:

- `SIGNATURE_SIGNING_PROVIDER_RNG_NEEDS_SPLIT`
- `SIGNATURE_SIGNING_PROVIDER_RNG_REQUIRES_CFG_SEAM`

## identity bootstrap / key generation review

qsc identity key material is generated through `hs_kem_keypair()` and
`hs_sig_keypair()` wrappers in `qsl/qsl-client/qsc/src/handshake/mod.rs`, which
delegate to refimpl runtime provider helpers. Signature key generation is
currently infallible from qsc's perspective even though refimpl fills ML-DSA
seed material from `OsRng`.

Exact identity generation paths discovered:

- `qsl/qsl-client/qsc/src/identity/mod.rs` lazy self identity creation;
- `qsl/qsl-client/qsc/src/identity/mod.rs` legacy migration and missing
  signature public-key upgrade;
- `qsl/qsl-client/qsc/src/main.rs` explicit `identity rotate`;
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` TUI account
  bootstrap identity initialization.

Current APIs cannot force signature keypair/provider RNG failure. Future tests
would need to assert no partial public identity record, no partial KEM/signature
secret, no vault mutation when applicable, and no partial session/pending
state when identity creation is reached through handshake. Because identity
generation crosses CLI and TUI account surfaces, identity-only implementation is
not exact enough in NA-0459.

Classification:

- `IDENTITY_PROVIDER_RNG_NEEDS_SPLIT`
- `IDENTITY_PROVIDER_RNG_REQUIRES_CFG_SEAM`

## signature verify / invalid signature review

`hs_sig_verify()` maps both invalid signature and provider verify errors to
the sanitized `sig_invalid` marker. Existing B1 and A2 tamper tests cover
invalid-signature reject/no-mutation behavior.

Verification is not provider RNG-dependent in the current qsc/refimpl shape and
should not be included in a provider RNG implementation scope. It remains
background evidence for signature reject handling.

Classification:

- `SIGNATURE_VERIFY_BACKGROUND_ONLY`
- `SIGNATURE_VERIFY_NOT_RNG_RELEVANT`

## X25519 / ephemeral generation review

qsc-visible X25519/ephemeral generation is in
`qsl/qsl-client/qsc/src/handshake/mod.rs` through `hs_ephemeral_keypair()`,
which calls `StdCrypto::keypair()`. It is used by the initiator before pending
state and A1 output, and by the responder before responder pending/session
state and B1 output.

Current qsc APIs cannot force X25519 keypair provider RNG failure. A future
cfg-only seam could likely be local to qsc, but this surface is separate from
signature signing and identity bootstrap. It should not be bundled with the
NA-0460 signature/identity split authorization except as an explicit residual.

Classification:

- `X25519_BACKLOG`

## combined vs split signature / identity decision

Option 1 -- combined signature / identity implementation next: rejected.
Signing and identity generation do not share one safe state-timing profile, and
identity also spans CLI/TUI bootstrap paths.

Option 2 -- signing-only implementation next: rejected for NA-0460. Signing is
important, but B1 and A2 need separate state-boundary handling before an
implementation lane can be exact.

Option 3 -- identity-bootstrap-only implementation next: rejected for NA-0460.
Identity generation is important, but exact paths include `identity/mod.rs`,
`main.rs`, and TUI locked account bootstrap.

Option 4 -- signature / identity split-further authorization: selected. This is
the least invasive truthful next step and preserves exact future scope.

Option 5 -- refimpl-first: rejected. qsc-local cfg seams can still produce
bounded qsc no-mutation evidence without refimpl mutation once split.

Option 6 -- documentation-only: rejected. There are exact qsc-visible surfaces;
the blocker is split granularity, not absence of future work.

Option 7 -- next audit domain: rejected. Signature/identity residuals remain
active and should be split before moving to KEM/signature/transcript binding.

## signature / identity scope matrix

| Candidate surface | Exact path(s) | Provider/RNG operation | Existing marker | Existing coverage | Future seam/fake need | Existing APIs enough? | Refimpl dependency? | Production-semantics risk | Future test path if selected | Priority | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| signing failure / `sig_sign_failed` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | B1/A2 `StdCrypto::sign()` | `sig_sign_failed` | no forced sign-failure coverage | cfg-only qsc seam | no | no for qsc-local proof | medium/high due A2 state timing | future signing-specific test after split | high | no | needs B1/A2 split | No signature-complete claim. | G1, G2, G3, G4, G5 |
| invalid signature / `sig_invalid` | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/tests/handshake_mvp.rs` | `StdCrypto::verify()` | `sig_invalid` | B1/A2 tamper no-mutation tests | none for RNG scope | yes for invalid signature | no | low | background only | background | no | not RNG-relevant | No provider-RNG-complete claim. | G1, G2, G3, G4, G5 |
| identity bootstrap/provider key generation | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | `hs_sig_keypair()` and identity key writes | none for provider RNG | success/key lifecycle only | cfg-only qsc seam | no | no for qsc-local proof | medium/high due public/secret write ordering | future identity-specific test after split | high | no | exact identity surfaces need split | No identity-complete claim. | G1, G2, G3, G4, G5 |
| X25519/ephemeral generation | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::keypair()` | none | success/tamper only | separate cfg-only qsc seam | no | no for qsc-local proof | medium | future X25519-specific test if selected later | backlog | no | not signature/identity | No RNG-failure-complete claim. | G1, G2, G3, G4, G5 |
| combined signature/identity implementation | multiple qsc source paths | signing plus identity key generation | mixed | partial | cfg seams would differ | no | no for qsc-local proof | high | not selected | high | no | too broad | No crypto-complete claim. | G1, G2, G3, G4, G5 |
| signing-only | `handshake/mod.rs` | B1/A2 signing | `sig_sign_failed` | partial | cfg seam plus B1/A2 decision | no | no | medium/high | not selected until split | high | no | A2 already-mutated state boundary needs authorization | No signature-complete claim. | G1, G2, G3, G4, G5 |
| identity-only | identity, CLI rotate, TUI bootstrap paths | identity signature/keypair generation | none | partial | cfg seam plus exact path selection | no | no | medium/high | not selected until split | high | no | broader than narrow identity module | No identity-complete claim. | G1, G2, G3, G4, G5 |
| refimpl-first | `tools/refimpl/**` | provider boundary changes | n/a | refimpl tests green | broader provider work | partial | yes | high | not selected | deferred | no | qsc-local split can proceed first | No provider-RNG-complete claim. | G1, G2, G3, G4, G5 |
| documentation-only | governance docs only | none | n/a | n/a | none | n/a | no | low | not selected | low | no | exact qsc surfaces exist | No public-readiness claim. | G1, G2, G3, G4, G5 |
| next audit domain | read-only KEM/signature/transcript audit | none | n/a | KEM seam green | none | n/a | no | low | not selected | medium | no | signature/identity residual should be split first | No crypto-complete claim. | G1, G2, G3, G4, G5 |

## authorization decision

Primary classification:

`QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_NEEDED`

NA-0459 selects a split-further authorization successor because exact
implementation paths are not yet narrow enough. Future implementation should
not be selected until NA-0460 decides whether to authorize:

- B1 signing only;
- A2 signing only or A2 with explicit already-mutated caveat;
- identity signature keypair / lazy identity creation only;
- identity CLI rotate / TUI account bootstrap separately;
- X25519 as a separate backlog lane.

No implementation mutation occurs in NA-0459.

## successor selection

Selected NA-0460:

`NA-0460 -- QSL qsc Signature / Identity Provider RNG Failure Split-Scope Authorization Plan`

This successor preserves the one-READY invariant and does not implement NA-0460.

## future path/scope bundle

Future NA-0460 allowed mutation paths:

- `docs/governance/evidence/NA-0460_qsl_qsc_signature_identity_provider_rng_failure_split_scope_authorization_plan.md`
- `tests/NA-0460_qsl_qsc_signature_identity_provider_rng_failure_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0460 read-only inspection may include:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/tests/`
- `tools/refimpl/`
- prior governance evidence and testplans
- qsc fuzz, formal, inputs, scripts, and workflows read-only

Future forbidden unless a later exact scope authorizes it:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo/lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs/website changes;
- qsl-server/qsl-attachments changes;
- backup/restore/qsl-backup changes;
- public claims.

## future validation/marker plan

Common NA-0460 markers:

- `NA0460_SIGNATURE_IDENTITY_SCOPE_CONSUMED_OK`
- `NA0460_NEXT_SCOPE_SELECTED_OK`
- `NA0460_NO_DEPENDENCY_CHANGE_OK`
- `NA0460_NO_WORKFLOW_CHANGE_OK`
- `NA0460_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0460_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0460_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0460_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0460_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0460_ONE_READY_INVARIANT_OK`
- `NA0460_SIGNATURE_IDENTITY_SPLIT_SCOPE_SELECTED_OK`

Future validation should include:

- root cargo audit;
- nested qsc fuzz lock audit;
- qsc cfg/no-cfg KEM provider RNG tests;
- qsc cfg/no-cfg residual RNG tests;
- qsc key lifecycle test;
- qsc provider-error no-mutation test;
- qsc send_commit test;
- refimpl `pqkem768`;
- qsc adversarial script syntax and PR CI qsc-adversarial-smoke;
- formal model scripts;
- link check, leak scan, overclaim scan, PR body preflight, goal-lint, and
  exact scope guard.

If a later implementation successor is selected, it must add marker-specific
forced-failure and no-mutation evidence only for the exact selected surface.

## public claim/external review/website boundary

qsc signature/identity provider RNG scope authorization is internal governance
evidence only. It is not production readiness. It is not public-internet
readiness. It is not crypto-complete proof. It is not side-channel-free proof.
It is not RNG-failure-complete proof. It is not provider-RNG-complete proof. It
is not signature-complete proof. It is not identity-complete proof. It is not
bug-free proof. It is not vulnerability-free proof. It is not perfect-crypto
proof. It is not public technical paper content. No README, START_HERE, public
docs, or website update is made. Cargo audit green is dependency-health
evidence only. Future tests, if authorized, must be described as bounded qsc
evidence only.

## rejected alternatives

- Combined signature/identity implementation: rejected because state timing and
  exact paths differ.
- Signing-only implementation: rejected because B1 and A2 signing need separate
  state-boundary handling.
- Identity-only implementation: rejected because identity generation spans lazy
  identity, migration/upgrade, CLI rotate, and TUI account bootstrap.
- Refimpl-first: rejected because qsc-local split authorization can proceed
  without refimpl mutation.
- Documentation-only: rejected because exact qsc-visible surfaces exist.
- Next audit domain: rejected because signature/identity residuals remain
  unresolved and should be split first.

## backup-impact statement

No backup was run. No restore was run. No qsl-backup, backup status, backup
plan, rollback subtree, or `/backup/qsl` path was mutated. The qsl-backup SHA
and source-list proof are boundary evidence only.

## next recommendation

Close NA-0459 after this evidence PR merges and post-merge public-safety is
green, then restore `NA-0460 -- QSL qsc Signature / Identity Provider RNG
Failure Split-Scope Authorization Plan` as the sole READY item. NA-0460 should
remain authorization-only and select the exact future implementation or
defer/no-action path without mutating qsc runtime, crypto, dependency,
workflow, executable test, fuzz, vector, formal, service, backup, or public
surface paths.
