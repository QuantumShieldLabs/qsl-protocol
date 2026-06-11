Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0460 QSL qsc Signature / Identity Provider RNG Failure Split-Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0460 consumes NA-0459 and selects the first exact implementation successor
for the qsc signature / identity provider RNG residuals.

Primary classification:

`SIGNATURE_IDENTITY_SPLIT_B1_SIGNING_NEXT`

Selected successor:

`NA-0461 -- QSL qsc B1 Signature Provider RNG Failure Test Seam Implementation Harness`

Reason: the B1 responder signing path is exact, qsc-visible, and has a clean
pre-mutation boundary. In `qsl/qsl-client/qsc/src/handshake/mod.rs`, B1 signing
with `StdCrypto::sign()` returns the sanitized `sig_sign_failed` marker before
responder pending state is stored, before responder session state is stored, and
before B1 output is emitted. A future cfg-only qsc call-site seam can force the
B1 signing failure path for bounded test evidence without refimpl, dependency,
workflow, Cargo, lockfile, fuzz, vector, formal, or public-surface mutation.

A2 signing is not selected for the first implementation successor because it
occurs after initiator session storage and pending clear. Its future invariant
must be different and truthful about already-mutated initiator state. Identity
generation is not selected because lazy identity creation, legacy/public-record
upgrade, CLI rotation, and TUI account bootstrap have different write timing and
partial-state surfaces. Signature verification / `sig_invalid` remains
background reject evidence and is not RNG-relevant. X25519 / ephemeral
generation remains a separate backlog surface.

NA-0460 authorizes no implementation mutation. It changes no runtime behavior,
crypto behavior, dependencies, Cargo manifests, lockfiles, workflows,
executable test source, fuzz targets, vectors, formal models, refimpl,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback subtree, or backup tree path.

No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
No signature-complete claim is made. No identity-complete claim is made.
No crypto-complete claim is made. Cargo audit green remains dependency-health
evidence only.

Required markers recorded by this evidence:

- `NA0460_SIGNATURE_IDENTITY_SPLIT_CONSUMED_OK`
- `NA0460_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0460_NA0459_INHERITANCE_CONSUMED_OK`
- `NA0460_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0460_SPLIT_TARGET_INVENTORY_OK`
- `NA0460_B1_SIGNING_IMPLEMENTATION_READY_OK`
- `NA0460_A2_SIGNING_DEFERRED_DIFFERENT_INVARIANT_OK`
- `NA0460_IDENTITY_GENERATION_SPLIT_BY_PATH_OK`
- `NA0460_VERIFY_BACKGROUND_ONLY_OK`
- `NA0460_X25519_BACKLOG_OK`
- `NA0460_SUCCESSOR_NA0461_B1_SIGNING_SELECTED_OK`
- `NA0460_NO_RUNTIME_CHANGE_OK`
- `NA0460_NO_CRYPTO_CHANGE_OK`
- `NA0460_NO_DEPENDENCY_CHANGE_OK`
- `NA0460_NO_WORKFLOW_CHANGE_OK`
- `NA0460_NO_TEST_IMPLEMENTATION_OK`
- `NA0460_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0460_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0460_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0460_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0460_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0460_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0460_ONE_READY_INVARIANT_OK`

## Live NA-0460 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0460.
- NA-0459 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0906.
- D-0905 exists once.
- D-0906 exists once.
- D-0907 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0460 mutation paths are exactly:

- `docs/governance/evidence/NA-0460_qsl_qsc_signature_identity_provider_rng_failure_split_scope_authorization_plan.md`
- `tests/NA-0460_qsl_qsc_signature_identity_provider_rng_failure_split_scope_authorization_testplan.md`
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
- NA-0459 inheritance is consumed;
- signature, identity, verification, and X25519 split candidates are inventoried;
- B1, A2, identity, verification, and X25519 reviews are completed;
- one primary classification and one NA-0461 successor are selected;
- exact future paths are recorded if implementation-ready;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1188
not merged, origin/main not equal to or descending from the expected merge,
missing D-0906, D-0907 already present, unconsumable NA-0459 inheritance,
unsafe signature/identity classification, unsafe successor selection, failed
root or nested audit, missing public-safety, backup boundary regression, or any
forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0460/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0460/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0460`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0460/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0460`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, clean-state fields, READY count, queue top READY, and requested
lane status.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`a89221dc8983`. Fetch did not advance `origin/main`.

PR #1188 was verified merged at `a89221dc8983`.

Proof root:

`/srv/qbuild/tmp/NA0460_qsc_signature_identity_split_scope_20260611T102442Z`

## NA-0459 inheritance

NA-0459 / D-0905 classified the signature / identity residual as:

`QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_NEEDED`

Inherited facts:

- qsc KEM provider RNG forced-seam evidence from NA-0458 is complete and green.
- B1 signing remains residual, with `sig_sign_failed` before responder pending,
  session, and B1 output mutation.
- A2 signing remains residual, with `sig_sign_failed` after initiator session
  storage and pending clear.
- identity provider RNG remains residual across lazy identity creation,
  legacy/public-record upgrade, CLI rotation, and TUI account bootstrap.
- X25519 / ephemeral generation remains backlog.
- verification / `sig_invalid` remains background only and not RNG-relevant.
- refimpl provider RNG remains deferred.
- qsc KEM evidence is bounded and is not KEM-complete proof.
- no RNG-failure-complete claim exists;
- no provider-RNG-complete claim exists;
- no signature-complete claim exists;
- no identity-complete claim exists;
- no crypto-complete claim exists;
- no public-readiness claim exists.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- split signature and identity into the least invasive future scope;
- do not bundle KEM, X25519, or refimpl work into the first successor;
- preserve fail-closed behavior and production semantics;
- preserve public-claim caveats;
- account for the state-mutation order difference between B1 signing, A2
  signing, and identity paths.

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
- qsc signature/identity split-scope authorization is internal governance
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

## signature / identity split target inventory

| Candidate | Exact source path(s) | Provider operation | State mutation timing | Error marker | Existing coverage | Existing APIs enough? | Future cfg-only seam enough? | Fake provider/injection needed? | Refimpl changes needed? | Future test path if selected | No-mutation invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| B1 signing failure | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for responder B1 | before responder pending store, responder session store, and B1 output | `sig_sign_failed` | B1 signature tamper covers `sig_invalid`, not forced sign failure | no | yes | no | no | `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs` | no responder pending/session state and no B1 output | first |
| A2 signing failure | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::sign()` for initiator A2 | after initiator session store and pending clear, before A2 output | `sig_sign_failed` | A2 signature tamper covers `sig_invalid`, not forced sign failure | no | yes, but with different invariant | no | no | future A2-specific test | truthful invariant must allow already-stored initiator session and cleared pending state; no A2 output | deferred after B1 |
| lazy identity key generation | `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` after KEM keypair in lazy identity creation | before KEM/signature secret writes and public record write, but after KEM generation | no provider-RNG marker today | success and key lifecycle evidence only | no | likely | no | no | future identity-lazy test | no partial identity secrets or public record for selected path | split by path |
| legacy/public-record identity upgrade | `qsl/qsl-client/qsc/src/identity/mod.rs` | `hs_sig_keypair()` for legacy migration or missing `sig_pk` upgrade | legacy migration generates signature before KEM secret write; missing `sig_pk` upgrade generates before signature secret and public record rewrite | no provider-RNG marker today | success and migration/identity evidence only | no | likely | no | no | future identity-upgrade test | no partial vault/public record update for selected path | split by path |
| CLI identity rotation | `qsl/qsl-client/qsc/src/main.rs` | `hs_kem_keypair()` and `hs_sig_keypair()` in `identity_rotate` | generates both keypairs before KEM secret, signature secret, public record, and optional peer reset writes | no provider-RNG marker today | success/key lifecycle evidence only | no | likely, but CLI-specific | no | no | future CLI rotate identity test | no partial rotated identity or peer reset side effects | split by path |
| TUI account bootstrap identity generation | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | `hs_kem_keypair()` and `hs_sig_keypair()` in account bootstrap identity init | after account defaults and route token writes, before identity secret/public writes | no provider-RNG marker today | TUI account success evidence only | no | likely, but TUI/account-specific | no | no | future TUI bootstrap identity test | no partial identity write; account-default prior writes need separate truthful treatment | split by path |
| invalid signature / `sig_invalid` background | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/tests/handshake_mvp.rs` | `StdCrypto::verify()` for B1/A2 | reject path; not RNG-dependent | `sig_invalid` | B1/A2 tamper no-mutation tests | yes for invalid signature | no for RNG scope | no | no | none for provider RNG | background reject evidence only | background |
| X25519 / ephemeral generation backlog | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `StdCrypto::keypair()` in `hs_ephemeral_keypair()` | initiator generation before pending store/A1 output; responder generation before responder pending/session/B1 output | none | success/tamper only | no | possible, but separate | no | no | future X25519-specific test | separate no-mutation/output invariant | backlog |

Inventory conclusion:

- B1 signing has the cleanest exact future implementation scope.
- A2 signing must remain separate because its state timing differs.
- identity generation must be split by path before implementation.
- verification is not RNG-relevant and remains background only.
- X25519 remains separate backlog.

## B1 signing split review

B1 responder signing occurs in `qsl/qsl-client/qsc/src/handshake/mod.rs` after
responder KEM encapsulation, X25519 generation, session construction, identity
load, transcript MAC/hash construction, and B1 signature message construction.
The `StdCrypto::sign()` call returns `sig_sign_failed` on error.

The key ordering is:

- B1 signature message is built;
- `StdCrypto::sign()` is called;
- on failure, qsc emits `handshake_reject reason=sig_sign_failed` and
  continues;
- responder pending state is stored only after signing succeeds;
- B1 output is encoded and pushed only after pending state is stored.

Answers:

- B1 signing happens before responder pending/session insertion for selected
  persistent state and before B1 output.
- A future qsc cfg-only seam can force signing failure at the B1 signing call
  site.
- Future tests can assert no responder pending state, no responder session, and
  no B1 relay output.
- Exact future source/test paths are small.
- B1 signing is the safest first implementation target.

Classification:

`B1_SIGNING_PROVIDER_RNG_IMPLEMENTATION_READY`

## A2 signing split review

A2 initiator signing occurs in `qsl/qsl-client/qsc/src/handshake/mod.rs` after
the initiator builds and stores the session, clears initiator pending state, and
derives the confirm key and MAC. The `StdCrypto::sign()` call returns
`sig_sign_failed` on error before A2 output is emitted.

Answers:

- A2 signing occurs after initiator state mutation and pending clear.
- A truthful future invariant cannot claim no initiator session mutation.
- A future A2 lane must assert a different invariant, such as no A2 output and
  no additional mutation beyond the already-stored initiator session and cleared
  pending state, if that remains the selected design.
- A2 should be a separate lane after B1.
- A2 is not selected as the first implementation successor now.

Classification:

`A2_SIGNING_PROVIDER_RNG_DEFER_AFTER_B1`

Secondary classification:

`A2_SIGNING_PROVIDER_RNG_REQUIRES_DIFFERENT_INVARIANT`

## identity generation split review

Identity-generation paths are not equivalent enough to bundle:

- lazy identity creation in `identity/mod.rs` creates KEM and signature keys,
  then writes KEM secret, signature secret, and public record;
- legacy migration in `identity/mod.rs` generates a signature keypair, then
  writes migrated KEM secret, signature secret, and upgraded public record;
- missing `sig_pk` public-record upgrade in `identity/mod.rs` generates a
  signature keypair, then writes signature secret and public record;
- CLI rotation in `main.rs` generates KEM and signature keys before secret,
  public-record, and optional peer-reset writes;
- TUI account bootstrap in `locked.rs` generates identity keys after account
  defaults and route-token writes, so the truthful invariant must distinguish
  prior account state from identity writes.

Answers:

- The identity paths are not uniform enough to bundle.
- The smallest identity invariant is likely lazy identity creation, but even
  that path mixes KEM generation with signature generation and identity writes.
- Future tests must distinguish vault write, public-record write, selected
  identity write, and TUI account state.
- Identity generation should be split into a separate authorization after B1
  signing.
- Identity implementation is not selected now.

Classification:

`IDENTITY_GENERATION_SPLIT_BY_PATH_NEEDED`

Secondary classification:

`IDENTITY_GENERATION_DEFER_AFTER_SIGNING`

## verify / X25519 background review

Verification / `sig_invalid`:

- `hs_sig_verify()` maps invalid B1/A2 signatures and provider verify errors to
  sanitized `sig_invalid`.
- Existing B1 and A2 tamper tests exercise useful no-mutation reject behavior.
- The current verification path is not provider RNG-dependent.
- Verification remains background only.

Classification:

- `VERIFY_BACKGROUND_ONLY`
- `VERIFY_NOT_RNG_RELEVANT`

X25519 / ephemeral generation:

- `hs_ephemeral_keypair()` calls `StdCrypto::keypair()`.
- The surface is qsc-visible but distinct from signature and identity.
- It should not be included in this signature/identity successor.
- X25519 remains separate backlog.

Classification:

- `X25519_BACKLOG`
- `X25519_SEPARATE_SCOPE_NEEDED`

## combined vs first implementation candidate decision

Option 1 - B1 signing implementation next: selected. B1 signing has exact paths
and a clean pre-mutation/no-B1-output invariant.

Option 2 - A2 signing implementation next: rejected for the first successor.
A2 has exact paths, but the invariant is different because signing happens
after initiator session storage and pending clear.

Option 3 - identity-only implementation next: rejected. Identity paths are
stateful and non-uniform; no single identity path is selected as higher-value
than B1 for this lane.

Option 4 - signing split-further authorization: rejected. B1 is exact enough to
implement next; A2 can remain deferred without another signing-only strategy
lane.

Option 5 - identity split-further authorization: rejected as the primary next
lane. It remains a future need after B1.

Option 6 - combined signing/identity implementation: rejected. The paths and
invariants are not small or uniform.

Option 7 - documentation-only: rejected. B1 is exact enough for bounded future
implementation evidence.

Option 8 - next audit domain: rejected. Signature/identity provider RNG work
still has a small, exact B1 implementation target.

Public-claim caveat for all options: no option creates signature-complete proof,
no option creates identity-complete proof, no option creates RNG-failure-complete
proof, no option creates provider-RNG-complete proof, no option creates
crypto-complete proof, no option creates vulnerability-free proof, no option
creates bug-free proof, and no option creates perfect-crypto proof.

## split-scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | State timing | Future mutation type | Production-semantics risk | No-mutation invariant clarity | Evidence value | Scope size | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| B1 signing implementation | responder B1 signing | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs`; NA-0461 governance paths | before responder pending/session store and B1 output | cfg-only qsc call-site seam plus executable qsc test | low if seam is cfg-only and no-cfg test proves unchanged | high | high | small | no | yes | exact and lowest-risk first implementation | bounded evidence only; no signature-complete claim | G1, G2, G3, G4, G5 |
| A2 signing implementation | initiator A2 signing | `qsl/qsl-client/qsc/src/handshake/mod.rs`; future A2 test | after initiator session store and pending clear, before A2 output | cfg-only qsc call-site seam plus different state assertions | medium | medium | high | small/medium | yes | no | invariant differs from B1 | bounded evidence only; no signature-complete claim | G1, G2, G3, G4, G5 |
| signing split-further authorization | B1 and A2 signing | NA-0461 signing split governance paths | mixed | governance only | low | medium | medium | small | no for B1, yes for A2 | no | B1 is already exact enough | no signature-complete claim | G1, G2, G3, G4, G5 |
| lazy identity implementation | lazy identity creation | `qsl/qsl-client/qsc/src/identity/mod.rs`; future lazy identity test | before identity secret/public writes, after KEM generation | cfg-only qsc identity/signature keypair seam plus test | medium | medium | high | medium | yes | no | identity path still needs split | no identity-complete claim | G1, G2, G3, G4, G5 |
| CLI rotate identity implementation | explicit identity rotation | `qsl/qsl-client/qsc/src/main.rs`; future CLI identity test | before rotated identity writes and optional peer reset | cfg-only qsc seam plus CLI test | medium/high | medium | medium | medium | yes | no | state side effects differ from lazy identity | no identity-complete claim | G1, G2, G3, G4, G5 |
| TUI bootstrap identity implementation | TUI account bootstrap identity | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`; future TUI test | after account defaults, before identity writes | cfg-only qsc seam plus TUI/account test | medium/high | lower unless prior account writes are explicitly scoped | medium | medium/high | yes | no | account defaults and identity writes must be separated | no identity-complete claim | G1, G2, G3, G4, G5 |
| identity split-further authorization | identity lazy, upgrade, CLI, TUI | future identity split governance paths | mixed | governance only | low | medium | medium | small | yes | no | future need, but lower priority than B1 | no identity-complete claim | G1, G2, G3, G4, G5 |
| combined signing/identity implementation | signing and identity | multiple qsc source/test paths | mixed | multiple cfg seams and tests | high | low | high but diffuse | large | yes | no | too broad and non-uniform | no crypto-complete claim | G1, G2, G3, G4, G5 |
| verify/sig_invalid background | B1/A2 verification | `qsl/qsl-client/qsc/src/handshake/mod.rs`; existing tamper tests | reject path, not RNG-dependent | none for provider RNG | low | high for invalid signature, not RNG | background | none | no | no | not RNG-relevant | no provider-RNG-complete claim | G1, G2, G3, G4, G5 |
| X25519 backlog | ephemeral keypair | `qsl/qsl-client/qsc/src/handshake/mod.rs`; future X25519 test | before pending/output in selected roles | separate cfg-only seam and test | medium | medium/high by role | medium | small/medium | yes | no | not signature/identity | no RNG-failure-complete claim | G1, G2, G3, G4, G5 |
| next audit domain | KEM/signature/transcript binding audit | future read-only audit governance paths | n/a | governance/read-only audit | low | n/a | medium | small | no | no | B1 residual is ready first | no crypto-complete claim | G1, G2, G3, G4, G5 |

## authorization decision

Primary classification:

`SIGNATURE_IDENTITY_SPLIT_B1_SIGNING_NEXT`

NA-0460 selects B1 signing as the next exact implementation successor because
the current source ordering supports a truthful bounded invariant:

- forced B1 signing failure can occur before responder pending state is stored;
- forced B1 signing failure can occur before responder session state is stored;
- forced B1 signing failure can occur before B1 output is emitted;
- the failure marker is already sanitized as `sig_sign_failed`;
- future changes can be limited to a qsc cfg-only call-site seam, one B1 test,
  and governance evidence.

No implementation mutation occurs in NA-0460.

## successor selection

Selected NA-0461:

`NA-0461 -- QSL qsc B1 Signature Provider RNG Failure Test Seam Implementation Harness`

This successor preserves the one-READY invariant and does not implement NA-0461
inside NA-0460.

## future path/scope bundle

Future NA-0461 allowed implementation paths, if restored by closeout:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs`

Future NA-0461 governance paths:

- `docs/governance/evidence/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0461 forbidden unless another exact scope authorizes:

- A2 signing implementation;
- identity implementation;
- X25519 implementation;
- refimpl mutation;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs or website changes;
- qsl-server or qsl-attachments changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree changes;
- public claim expansion.

Future B1 validation must include:

- cfg-forced B1 signing failure emits `sig_sign_failed`;
- cfg-forced B1 signing failure writes no responder pending state;
- cfg-forced B1 signing failure writes no responder session state;
- cfg-forced B1 signing failure emits no B1 output;
- no-cfg build ignores the seam selector and preserves production semantics;
- no dependency, workflow, Cargo, lockfile, fuzz, vector, formal, refimpl,
  service, public-surface, backup, restore, or qsl-backup mutation occurs.

## future validation/marker plan

Common NA-0461 markers:

- `NA0461_SIGNATURE_IDENTITY_SPLIT_CONSUMED_OK`
- `NA0461_NEXT_SCOPE_SELECTED_OK`
- `NA0461_NO_DEPENDENCY_CHANGE_OK`
- `NA0461_NO_WORKFLOW_CHANGE_OK`
- `NA0461_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0461_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0461_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0461_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0461_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0461_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0461_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0461_ONE_READY_INVARIANT_OK`

B1 implementation successor markers:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RESPONDER_MUTATION_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_B1_OUTPUT_OK`
- `NA0461_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Expected validation commands for NA-0461 should include root audit, nested qsc
fuzz lock audit, cfg/no-cfg qsc tests for the new B1 seam, inherited qsc RNG
failure tests, key lifecycle zeroization, provider-error no-mutation,
`send_commit`, refimpl `pqkem768`, qsc adversarial script syntax and CI smoke,
formal model scripts, link check, leak scan, overclaim scan, PR body preflight,
goal-lint, and exact scope guard.

## public claim/external review/website boundary

qsc signature/identity split-scope authorization is internal governance
evidence only. It is not production readiness. It is not public-internet
readiness. It is not crypto-complete proof. It is not side-channel-free proof.
It is not RNG-failure-complete proof. It is not provider-RNG-complete proof.
It is not signature-complete proof. It is not identity-complete proof.
It is not bug-free proof. It is not vulnerability-free proof.
It is not perfect-crypto proof. It is not public technical paper content.
No README, START_HERE, public docs, or website update is made. Cargo audit
green is dependency-health evidence only. Future tests, if authorized, must be
described as bounded qsc evidence only.

## rejected alternatives

- A2 signing implementation first: rejected because A2 signing occurs after
  initiator session storage and pending clear.
- Identity implementation first: rejected because identity generation spans
  lazy identity, legacy/public-record upgrade, CLI rotation, and TUI account
  bootstrap with different state timing.
- Signing split-further authorization: rejected because B1 is exact enough for
  the next implementation lane.
- Identity split-further authorization as the next lane: rejected because B1 is
  cleaner and higher confidence for the first implementation successor.
- Combined signature/identity implementation: rejected because paths and
  invariants are too broad and non-uniform.
- Documentation-only: rejected because B1 has an exact bounded implementation
  target.
- Next audit domain: rejected because B1 provider RNG failure evidence is ready
  before moving to KEM/signature/transcript binding audit.
- Refimpl-first: rejected because qsc-local bounded B1 evidence can proceed
  without refimpl mutation.

## backup-impact statement

No backup was run. No restore was run. No qsl-backup, backup status, backup
plan, rollback subtree, or `/backup/qsl` path was mutated. The qsl-backup SHA
and script-local source-list count remain boundary evidence only.

## next recommendation

After the NA-0460 evidence PR merges and post-merge public-safety is green,
close out NA-0460 and restore:

`NA-0461 -- QSL qsc B1 Signature Provider RNG Failure Test Seam Implementation Harness`

NA-0461 should implement only the selected B1 signing forced-failure seam and
bounded qsc test evidence. A2 signing, identity generation, X25519, and refimpl
provider RNG should remain residual unless later exact scope authorizes them.
