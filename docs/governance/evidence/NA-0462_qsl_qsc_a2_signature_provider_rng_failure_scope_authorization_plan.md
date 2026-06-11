Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0462 QSL qsc A2 Signature Provider RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0462 consumes the NA-0461 B1 signature provider RNG failure implementation
evidence and authorizes the next exact A2 signature provider RNG failure scope.

Primary classification:

`A2_SIGNATURE_PROVIDER_RNG_NO_OUTPUT_IMPLEMENTATION_READY`

Selected successor:

`NA-0463 -- QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness`

The qsc source shows A2 signing occurs after the initiator session store and
after initiator pending clear, but before A2 relay output. Therefore the B1
pre-mutation no-mutation invariant is not truthful for A2. The future A2 lane
is implementation-ready only for a cfg-only, test-only no-output seam that
forces A2 signing failure at the `StdCrypto::sign()` call site, proves the
sanitized `sig_sign_failed` reject path, proves no A2 `handshake_send` or relay
output, and explicitly avoids any false A2 pre-mutation no-mutation claim.

NA-0462 is authorization-only. It makes no runtime, crypto, dependency, Cargo,
lockfile, workflow, executable-test, fuzz-target, vector, formal-model,
refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
README, START_HERE, public-doc, qwork/qstart/qresume/qshell, backup, restore,
qsl-backup, backup-status, backup-plan, rollback, or backup-tree mutation.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
vulnerability-free claim is made. No perfect-crypto claim is made. Cargo audit
green remains dependency-health evidence only.

Required markers recorded by this evidence:

- `NA0462_A2_SIGNATURE_SCOPE_CONSUMED_OK`
- `NA0462_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0462_NA0461_INHERITANCE_CONSUMED_OK`
- `NA0462_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0462_A2_SIGNING_TARGET_INVENTORY_OK`
- `NA0462_A2_STATE_TIMING_POST_MUTATION_PRE_OUTPUT_OK`
- `NA0462_A2_NO_OUTPUT_IMPLEMENTATION_SELECTED_OK`
- `NA0462_A2_FALSE_NO_MUTATION_CLAIM_REJECTED_OK`
- `NA0462_SUCCESSOR_NA0463_SELECTED_OK`
- `NA0462_NO_RUNTIME_CHANGE_OK`
- `NA0462_NO_CRYPTO_CHANGE_OK`
- `NA0462_NO_DEPENDENCY_CHANGE_OK`
- `NA0462_NO_WORKFLOW_CHANGE_OK`
- `NA0462_NO_TEST_IMPLEMENTATION_OK`
- `NA0462_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0462_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0462_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0462_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0462_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0462_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0462_ONE_READY_INVARIANT_OK`

## Live NA-0462 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0462.
- NA-0461 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0910.
- D-0909 exists once.
- D-0910 exists once.
- D-0911 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0462 mutation paths are exactly:

- `docs/governance/evidence/NA-0462_qsl_qsc_a2_signature_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0462_qsl_qsc_a2_signature_provider_rng_failure_scope_authorization_testplan.md`
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
- NA-0461 inheritance is consumed;
- A2 signing call site, state timing, output timing, and existing failure marker
  are inventoried;
- A2 invariant options are selected or rejected with evidence;
- one primary classification and one NA-0463 successor are selected;
- exact future paths are recorded because implementation is selected;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1192
not merged, origin/main not equal to or descending from the expected merge,
missing D-0910, D-0911 already present, unconsumable NA-0461 inheritance,
unsafe A2 classification, unsafe successor selection, failed root or nested
audit, backup boundary regression, or any forbidden mutation.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0462/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0462/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0462`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0462/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0462`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, clean-state fields, READY count, queue top READY, and requested
lane status.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`14f930a4dca`. Fetch did not advance `origin/main`.

PR #1192 was verified merged at `14f930a4dca`.

Proof root:

`/srv/qbuild/tmp/NA0462_qsc_a2_signature_scope_20260611T171611Z`

## NA-0461 inheritance

NA-0461 / D-0909 implemented the B1 signing-only qsc cfg seam:

- cfg-only label: `QSC.SIG.B1`;
- forced failure takes the sanitized `sig_sign_failed` path;
- forced failure occurs before selected responder pending/session storage;
- forced failure emits no B1 output;
- normal no-cfg production behavior is unchanged.

D-0910 closed NA-0461 and restored NA-0462 as the sole READY item.

Inherited residuals:

- A2 signing remains unimplemented and requires a post-mutation invariant.
- Identity provider RNG remains deferred across lazy identity,
  legacy/public-record upgrade, CLI rotation, and TUI account bootstrap.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- qsc KEM provider RNG seam evidence from NA-0458 remains background preserved
  only.
- no signature-complete claim exists;
- no identity-complete claim exists;
- no RNG-failure-complete claim exists;
- no provider-RNG-complete claim exists;
- no crypto-complete claim exists;
- no public-readiness claim exists.

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- A2 signature provider RNG scope must define a truthful post-mutation
  invariant.
- The B1 no-mutation invariant must not be reused for A2.
- Identity, X25519, KEM, and refimpl work must not be bundled into this A2
  successor.
- Future A2 work must preserve production semantics and public-claim caveats.

CI / Dependency / Release Health Steward:

- root `cargo audit --deny warnings` is green;
- nested qsc fuzz lock audit is green;
- B1 provider RNG seam tests are green;
- KEM provider RNG seam tests are green;
- cfg RNG failure tests are green;
- qsc key lifecycle and provider-error tests are green;
- refimpl `pqkem768` is green;
- qsc adversarial script marker is present and PR CI public-safety on current
  main is green;
- cargo audit green is dependency-health evidence only, not
  vulnerability-free proof.

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
- A2 scope authorization is internal governance evidence only.

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

## A2 signing target inventory

| Candidate | Exact source path | Provider operation | State mutation timing | Current error marker | Existing test coverage | Existing APIs enough? | Future cfg-only seam enough? | Fake provider/injection needed? | Refimpl changes needed? | Future test path if selected | Truthful invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A2 signing call site | `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1452-1463 | initiator `StdCrypto::sign()` over `hs_sig_msg_a2(...)` | after `qsp_session_store(peer, &st)` at line 1429 and after `hs_pending_clear(self_label, peer)` at line 1431 | `sig_sign_failed` | A2 signature tamper covers `sig_invalid`, not forced sign failure | no direct failure injection today | yes, if limited to a cfg-only A2 signing label | no | no | `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs` | no A2 output and sanitized fail-closed error; no false pre-mutation no-mutation claim | selected |
| A2 output / relay emission | `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1475-1482 | encode A2, emit `handshake_send msg=A2`, relay push | after successful A2 signing | none on success | normal handshake tests observe A2 output | yes for observing output | yes | no | no | same A2 future test | forced A2 signing failure must emit no A2 `handshake_send` and no relay A2 | selected |
| initiator pending clear timing | `qsl/qsl-client/qsc/src/handshake/mod.rs` line 1431 | `hs_pending_clear(self_label, peer)` | before A2 signing | none | key lifecycle tests cover normal clear behavior | yes for observing final state | yes, but not as no-mutation proof | no | no | same A2 future test | pending is already cleared before the forced A2 signing failure point | selected caveat |
| initiator session storage timing | `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1410-1430 | `hs_build_session(...)`, then `qsp_session_store(peer, &st)` | before A2 signing | `handshake_session_store_failed` if storage fails | session-store RNG failure coverage exists for a different seam | yes for observing final state | yes, but not as no-mutation proof | no | no | same A2 future test | initiator session is already stored before the forced A2 signing failure point | selected caveat |
| existing B1 signing seam comparison | `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1778-1792 | responder B1 `StdCrypto::sign()` with cfg-only `QSC.SIG.B1` | before responder pending/session storage and before B1 output | `sig_sign_failed` | cfg/no-cfg `b1_signature_provider_rng_failure` | yes for B1 only | already implemented for B1 | no | no | not selected for A2 | B1 pre-mutation invariant is not portable to A2 | background |
| identity residual boundary | `qsl/qsl-client/qsc/src/identity/mod.rs` plus CLI/TUI identity paths | signature key generation and identity writes | path-specific and not uniform | no provider-RNG marker today | key lifecycle and success paths only | not for direct failure injection today | likely, but path-specific | no | no | future identity split test(s) | separate no-partial-write invariant per identity path | excluded |
| X25519 residual boundary | `qsl/qsl-client/qsc/src/handshake/mod.rs` `hs_ephemeral_keypair()` | `StdCrypto::keypair()` | A1/B1 path-specific | none today | success/tamper only | no direct failure injection today | possible, separate lane | no | no | future X25519 test | separate no-output/no-mutation invariant | excluded |

## A2 state timing review

Classification:

`A2_STATE_TIMING_POST_MUTATION_PRE_OUTPUT`

Source evidence:

- A2 initiator session storage occurs at `handshake/mod.rs` line 1429.
- Initiator pending clear occurs at `handshake/mod.rs` line 1431.
- A2 signing occurs at `handshake/mod.rs` line 1453.
- Existing A2 signing failure emits `handshake_reject reason=sig_sign_failed`
  at lines 1456-1460 and returns at line 1461.
- A2 `handshake_send msg=A2` is emitted at lines 1477-1480.
- A2 relay output is pushed at line 1482.

Answers:

- A2 signing occurs after initiator session store write.
- A2 signing occurs after pending state clear.
- A2 signing occurs before A2 relay output.
- Existing signing failure behavior is fail-closed, sanitized
  `sig_sign_failed`, and returns before A2 output.
- It is not truthful to assert no mutation for A2 signing failure.
- The truthful invariant is: selected A2 provider signing failure is
  post-mutation/pre-output, returns sanitized `sig_sign_failed`, and emits no
  A2 output.
- Future tests can safely assert no A2 output and can describe observed
  post-state without turning that observation into a pre-mutation no-mutation
  claim.
- This is a useful implementation lane if it is scoped to no-output evidence.

## A2 invariant options review

| Option | Decision | Evidence | Future paths | Future markers | Public-claim caveat |
|---|---|---|---|---|---|
| Option 1 - no mutation | rejected | A2 session store and pending clear precede A2 signing | none | `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_FALSE_NO_MUTATION_CLAIM_OK` | no signature-complete, provider-RNG-complete, RNG-failure-complete, or crypto-complete claim |
| Option 2 - no additional mutation after signing failure | rejected as primary | exact source state at the signing point is post-session-store and post-pending-clear; a future test can observe final state but cannot honestly call the path pre-mutation | not selected as primary | use only as bounded observation if later exact scope names snapshots | no false no-mutation claim |
| Option 3 - no A2 output / no relay emission | selected | signing failure returns at line 1461 before `handshake_send msg=A2` and relay push at lines 1477-1482 | `handshake/mod.rs`; `qsc/tests/a2_signature_provider_rng_failure.rs` | `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_A2_OUTPUT_OK` | bounded internal qsc evidence only |
| Option 4 - fail-closed sanitized error only | selected as required but insufficient alone | existing failure marker is `sig_sign_failed` | same future paths | `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK` if future lane adopts it | not provider-RNG-complete proof |
| Option 5 - rollback semantics | rejected | rollback would alter current state semantics and is not authorized | none | none | would require a separate design/change lane |
| Option 6 - documentation-only | rejected | no-output invariant is exact and testable without overclaim | not selected | none | documentation-only is unnecessary for the next lane |

## A2 implementation readiness review

Classification:

`A2_SIGNATURE_PROVIDER_RNG_NO_OUTPUT_IMPLEMENTATION_READY`

Readiness answers:

- `qsl/qsl-client/qsc/src/handshake/mod.rs` is enough for a future A2 cfg-only
  seam.
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs` is enough to
  prove the selected no-output invariant.
- The future implementation does not require source changes outside
  `handshake/mod.rs`.
- The future implementation does not require refimpl mutation.
- The future implementation does not require dependency, Cargo, lockfile, or
  workflow mutation.
- Normal production semantics can be proven unchanged with a no-cfg test that
  sets the future selector and still observes normal A2 output.
- The selected future test avoids overclaiming no-mutation by asserting no A2
  output and same-line caveating the post-mutation timing.
- Exact markers and invariant language are clear enough for an implementation
  successor.

## A2 vs identity / next domain decision

| Option | Decision | Evidence | Future exact paths if known | Future validation | Public-claim caveat |
|---|---|---|---|---|---|
| A2 implementation next | selected | A2 signing call site and no-output invariant are exact | `handshake/mod.rs`; `qsc/tests/a2_signature_provider_rng_failure.rs`; NA-0463 evidence/testplan; `DECISIONS.md`; `TRACEABILITY.md`; rolling journal | cfg/no-cfg A2 tests plus inherited qsc/refimpl/formal/audit checks | bounded internal qsc evidence only |
| A2 documentation-only next | rejected | no-output proof has real evidence value without claiming no mutation | none | none | no public claim expansion |
| identity split-scope next | rejected for immediate successor | identity remains important but A2 no-output is smaller and exact | future identity governance paths only | future split-scope review | identity remains residual |
| KEM / signature / transcript binding audit next | rejected for immediate successor | A2 no-output implementation remains bounded and useful | future audit paths not selected | future read-only audit if later promoted | no crypto-complete claim |
| refimpl provider RNG boundary next | rejected for immediate successor | qsc A2 can progress without refimpl mutation | future refimpl governance paths not selected | future refimpl boundary review | refimpl provider RNG remains residual |

## A2 scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | State timing | Future mutation type | Production-semantics risk | Truthful invariant clarity | Evidence value | Scope size | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A2 full no-mutation implementation | A2 signing plus initiator state | none selected | post-mutation | would require rollback/design change | high | false for current code | misleading | broad | yes | no | session store and pending clear occur before signing | no claim expansion; false no-mutation must not appear | G1-G5 |
| A2 no-output implementation | A2 signing and A2 relay output | `handshake/mod.rs`; `qsc/tests/a2_signature_provider_rng_failure.rs` | post-mutation/pre-output | cfg-only test seam plus integration test | low if cfg-only | high | high | small | no | yes | proves fail-closed no-A2-output behavior without false no-mutation | bounded internal evidence only | G1-G5 |
| A2 stable-post-state implementation | A2 signing plus observed final state | same paths if later narrowed | post-mutation/pre-output | cfg-only seam plus snapshot assertions | medium | medium | medium | small-medium | yes | no | snapshot wording could imply stronger rollback/no-mutation than source supports | no false no-mutation claim | G1-G5 |
| A2 fail-closed-only documentation | A2 signing failure marker | governance paths only | post-mutation/pre-output | docs only | low | high but weak | low-medium | small | no | no | no-output is exact enough to implement next | no public claim expansion | G1-G5 |
| identity split-scope | lazy, upgrade, rotate, TUI identity | future identity evidence/testplan | path-specific | authorization only | low | medium | high later | medium | yes | no | identity needs separate path split after A2 | identity remains residual | G1-G5 |
| refimpl provider boundary | refimpl provider RNG | future refimpl evidence/testplan | separate boundary | authorization only | low | medium | medium | medium | yes | no | qsc A2 does not need refimpl mutation | refimpl remains residual | G1-G5 |
| KEM/signature/transcript audit | read-only audit | future audit evidence/testplan | cross-domain | docs/read-only | low | medium | medium | medium | yes | no | A2 exact work has not reached diminishing returns | no crypto-complete claim | G1-G5 |

## authorization decision

Primary classification:

`A2_SIGNATURE_PROVIDER_RNG_NO_OUTPUT_IMPLEMENTATION_READY`

NA-0462 authorizes a future exact A2 no-output implementation lane and does not
implement it here.

Exact future implementation paths if restored:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No implementation mutation occurs in NA-0462. No runtime, crypto, dependency,
Cargo, lockfile, workflow, executable-test, fuzz-target, vector, formal-model,
refimpl, service, public-surface, backup, restore, qsl-backup, status, plan,
rollback, or qwork mutation occurs in NA-0462.

## successor selection

Selected successor:

`NA-0463 -- QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness`

Default recommendation followed: A2 no-output implementation is exact and
bounded, while stable-post-state is not selected as primary because it risks
implying a stronger no-mutation property than the current source supports.

## future path/scope bundle

Future NA-0463 allowed paths if restored:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope unless a later exact directive changes it:

- runtime/crypto implementation changes outside `handshake/mod.rs`;
- dependencies, Cargo files, lockfiles, or workflows;
- test source changes outside the exact A2 test path;
- fuzz targets, vectors, formal models, or refimpl;
- public docs, website, qsl-server, qsl-attachments, qshield runtime,
  qshield-cli, README, or START_HERE;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree paths;
- any public claim expansion;
- any false A2 pre-mutation no-mutation claim.

## future validation/marker plan

Common NA-0463 markers:

- `NA0463_A2_SIGNATURE_SCOPE_CONSUMED_OK`
- `NA0463_NEXT_SCOPE_SELECTED_OK`
- `NA0463_NO_DEPENDENCY_CHANGE_OK`
- `NA0463_NO_WORKFLOW_CHANGE_OK`
- `NA0463_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0463_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0463_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0463_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0463_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0463_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0463_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0463_ONE_READY_INVARIANT_OK`

A2 no-output implementation successor markers:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_A2_OUTPUT_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_FALSE_NO_MUTATION_CLAIM_OK`
- `NA0463_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Recommended additional implementation marker if future NA-0463 adopts it:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK`

Future validation should include cfg/no-cfg A2 tests, inherited B1/KEM/RNG
seam tests, key lifecycle and provider-error tests, qsc `send_commit`, refimpl
`pqkem768`, root and nested audits, formal checks, qsc adversarial smoke, scope
guard, link check, leak scan, overclaim scan, PR body preflight, goal-lint,
public-safety, and exactly one READY proof.

## public claim/external review/website boundary

A2 signature provider RNG scope authorization is internal governance evidence
only. It is not production readiness, not public-internet readiness, not
crypto-complete proof, not side-channel-free proof, not RNG-failure-complete
proof, not provider-RNG-complete proof, not signature-complete proof, not
identity-complete proof, not bug-free proof, not vulnerability-free proof, not
perfect-crypto proof, and not public technical paper content.

No README, START_HERE, public docs, or website update is made. No
public-readiness or public-security claim is made. Cargo audit green is
dependency-health evidence only. Future tests, if authorized, must be described
as bounded internal qsc evidence only.

## rejected alternatives

- A2 full no-mutation implementation: rejected because source timing is
  post-session-store and post-pending-clear.
- A2 stable-post-state implementation as primary: rejected because useful
  observations are possible, but the primary invariant could mislead unless a
  later exact directive defines snapshots precisely.
- A2 documentation-only: rejected because no-output is exact and useful.
- Rollback semantics: rejected because rollback would be a behavior/design
  change and is not authorized here.
- Identity split-scope next: rejected for immediate successor; identity remains
  residual and should be separately scoped later.
- Refimpl provider RNG next: rejected for immediate successor; qsc A2 no-output
  can progress without refimpl mutation.
- KEM/signature/transcript audit next: rejected for immediate successor; A2
  bounded no-output evidence is still a useful next step.

## backup-impact statement

NA-0462 has no backup impact. Codex did not run backup or restore. Codex did
not mutate `/usr/local/sbin/qsl-backup`, backup status files, backup plan files,
rollback subtree paths, systemd units, timers, fstab, source lists, retention,
backup scripts, or `/backup/qsl`.

Read-only boundary evidence:

- `/usr/local/sbin/qsl-backup` sha256:
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- script-local `/home/victor/work/qsl/codex/ops` source inclusion count: 1.

## next recommendation

After this evidence PR merges and post-merge public-safety is green, close out
NA-0462 and restore exactly one READY successor:

`NA-0463 -- QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness`

Do not implement NA-0463 inside the closeout.
