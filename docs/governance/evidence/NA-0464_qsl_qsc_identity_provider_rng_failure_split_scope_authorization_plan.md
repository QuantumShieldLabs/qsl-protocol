Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0464 QSL qsc Identity Provider RNG Failure Split-Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0464 consumes NA-0463 and classifies qsc identity-provider RNG failure work
after the bounded KEM, B1 signing, and A2 signing provider-failure evidence.

Primary classification:

`IDENTITY_SPLIT_LAZY_IDENTITY_NEXT`

Selected successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

The first identity-provider RNG implementation target should be lazy identity
generation in `qsl/qsl-client/qsc/src/identity/mod.rs`. It is the smallest
identity path with an exact source location and a clear no-partial-write
invariant: if future test-only provider RNG failure is forced before lazy
identity writes, the selected test must prove no identity KEM secret, no
identity signature secret, no self public record, no selected-identity state
write, and no dependent handshake state/output for the selected path.

Legacy/public-record upgrade, CLI identity rotation, and TUI account bootstrap
identity generation are real identity-provider RNG surfaces, but they are not
selected for the first implementation lane. They have different existing-state,
write timing, and user-flow boundaries. They remain residual and should be
authorized separately after the lazy identity lane.

This directive is authorization-only. It makes no runtime, crypto, dependency,
Cargo, lockfile, workflow, executable-test, fuzz-target, vector, formal-model,
refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
README, START_HERE, public-doc, qwork/qstart/qresume/qshell, backup, restore,
qsl-backup, backup-status, backup-plan, rollback, or backup-tree mutation.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made. Cargo audit green remains dependency-health evidence only.

Required NA-0464 markers recorded by this evidence:

- `NA0464_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0464_NA0463_INHERITANCE_CONSUMED_OK`
- `NA0464_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0464_IDENTITY_SPLIT_TARGET_INVENTORY_OK`
- `NA0464_LAZY_IDENTITY_PROVIDER_RNG_IMPLEMENTATION_READY_OK`
- `NA0464_LEGACY_IDENTITY_UPGRADE_DEFER_AFTER_LAZY_OK`
- `NA0464_CLI_ROTATE_IDENTITY_DEFER_AFTER_LAZY_OK`
- `NA0464_TUI_BOOTSTRAP_IDENTITY_DEFER_AFTER_LAZY_OK`
- `NA0464_IDENTITY_SPLIT_LAZY_IDENTITY_NEXT_OK`
- `NA0464_SUCCESSOR_NA0465_SELECTED_OK`
- `NA0464_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0464_NO_DEPENDENCY_CHANGE_OK`
- `NA0464_NO_WORKFLOW_CHANGE_OK`
- `NA0464_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0464_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0464_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0464_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0464_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0464_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0464_ONE_READY_INVARIANT_OK`

## Live NA-0464 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0464.
- NA-0463 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0914.
- D-0913 exists once.
- D-0914 exists once.
- D-0915 was absent before this patch.
- D-0916 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0464 mutation paths are exactly:

- `docs/governance/evidence/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_plan.md`
- `tests/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, refimpl,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback, backup tree paths, and public technical paper
content.

Acceptance criteria:

- qwork proof files are verified without rerunning qwork;
- NA-0463 inheritance is consumed;
- identity provider RNG candidate surfaces are inventoried by exact path and
  write surface;
- one primary classification and one NA-0465 successor are selected;
- exact future paths are recorded because implementation is selected;
- public claim caveats remain explicit;
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1196
not merged, origin/main not equal to or descending from the expected merge,
missing D-0914, D-0915 already present at start, unconsumable NA-0463
inheritance, unsafe identity split classification, unsafe successor selection,
failed root or nested audit, backup boundary regression, public-safety red or
missing, more than one READY item, or any forbidden mutation/public overclaim.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0464/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0464/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0464`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0464/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0464`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`51de5511e055`. Fetch did not advance `origin/main`.

PR #1196 was verified merged at `51de5511e055`.

Proof root:

`/srv/qbuild/tmp/NA0464_qsc_identity_provider_rng_split_scope_20260612T031907Z`

Marker:

- `NA0464_QWORK_PROOF_FILE_VERIFIED_OK`

## NA-0463 inheritance

NA-0463 / D-0913 implemented the A2 signature provider RNG failure no-output
seam:

- cfg-only label: `QSC.SIG.A2`;
- forced failure takes the sanitized `sig_sign_failed` path;
- forced failure occurs after initiator session storage and effective pending
  clear;
- forced failure emits no A2 `handshake_send`;
- forced failure emits no relay A2;
- normal no-cfg production behavior is unchanged;
- A2 pre-mutation no-mutation is explicitly rejected.

D-0914 closed NA-0463 and restored NA-0464 as the sole READY item.

Inherited residuals:

- Identity provider RNG remains deferred across lazy identity,
  legacy/public-record upgrade, CLI rotation, and TUI account bootstrap.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- qshield-cli demo RNG remains demo-local residual.
- qsc KEM provider RNG evidence from NA-0458 remains background only.
- qsc B1 signing provider RNG evidence from NA-0461 remains background only.
- qsc A2 signing no-output evidence from NA-0463 remains background only.
- no identity-complete claim exists;
- no signature-complete claim exists;
- no RNG-failure-complete claim exists;
- no provider-RNG-complete claim exists;
- no crypto-complete claim exists;
- no public-readiness claim exists.

Marker:

- `NA0464_NA0463_INHERITANCE_CONSUMED_OK`

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- Split identity provider RNG paths by state/write surface.
- Do not bundle KEM, B1, A2, X25519, or refimpl work into the identity
  successor.
- Preserve production semantics and public-claim caveats.
- Identify the exact no-partial-identity-state invariant before any future
  implementation.

CI / Dependency / Release Health Steward:

- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock audit is green.
- A2/B1/KEM provider RNG seam tests are green.
- cfg RNG failure tests are green.
- qsc key lifecycle and provider-error tests are green.
- refimpl `pqkem768` validation is green and remains outside this lane's mutation
  scope.
- qsc adversarial script marker is present and public-safety on current main is
  green.
- Cargo audit green is dependency-health evidence only and is not vulnerability-free
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
- Identity split-scope authorization is internal governance evidence only.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local.
- qsl-server remains a service boundary.
- qsl-attachments remains a service boundary.
- No qshield, website, public-service, production, or public-internet readiness
  claim is made.

Local Ops / Backup / Restore Steward:

- Codex did not run backup or restore.
- Codex did not mutate local ops, qsl-backup, backup status, backup plan,
  rollback, or backup tree paths.
- qsl-backup proof remains boundary evidence only.

Marker:

- `NA0464_STEWARD_REVIEW_TEMPLATE_USED_OK`

## identity split target inventory

| Candidate | Exact source path(s) | Provider operation | State/write timing | Existing marker/error | Existing test coverage | Existing APIs enough? | Future cfg-only seam enough? | Fake provider/injection needed? | Refimpl changes needed? | Future test path if selected | Truthful no-partial-write invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| lazy identity key generation | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 306-339; call sites in `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1135-1145, 1449-1451, 1769-1775 | KEM keypair and signature keypair for a missing self identity | path existence checked first; keypairs generated before KEM secret, signature secret, public record, and dependent handshake state/output | existing forced KEM path emits `identity_secret_unavailable` with `rng_failure_forced`; future identity-specific markers needed | NA-0458 cfg test proves generic KEM keypair forced failure writes no identity/session state; identity success paths exist in CLI/handshake tests | yes for path observation and vault/public-record assertions | yes; future lane should add identity-specific cfg labels before lazy KEM and lazy signature keypair writes | no | no | `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs` | forced lazy identity provider RNG failure writes no `identity.kem_sk.<label>`, no `identity.sig_sk.<label>`, no `identities/self_<label>.json`, no selected identity state, and no dependent handshake pending/session/output for selected test path | selected |
| legacy/public-record identity upgrade | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 188-224 and lines 255-260 | signature keypair for existing legacy or public record without `sig_pk` | existing identity/public record already exists; migration stores secret(s), then updates public record | `identity_secret_migrate`; `identity_secret_unavailable` | legacy/read coverage exists indirectly; no forced provider RNG upgrade test | enough for read/write observation, not enough for current forced signature keypair failure | likely, but must be separate from lazy path because existing material is present | no | no | later separate upgrade test path | forced upgrade failure must not partially import signature secret or update public record while preserving existing KEM material truthfully | high residual after lazy |
| CLI identity rotation | `qsl/qsl-client/qsc/src/main.rs` lines 977-1040 | KEM keypair and signature keypair for explicit rotation | unlocked check and confirmation first; new secrets are stored, public record is written, optional peer reset follows | `identity_rotate`; `identity_secret_unavailable`; `identity_rotate_write_failed` | `qsl/qsl-client/qsc/tests/identity_ux.rs` covers confirm and success | enough for observation, not enough for current forced identity keypair failure | likely, but reset-peer and prior-identity preservation make it separate | no | no | later CLI rotate test path | forced rotation failure must preserve previous selected label public record/secrets and must not reset peers unless writes have succeeded | high residual after lazy |
| TUI account bootstrap identity generation | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` lines 526-623 plus init call sites lines 151-176 and 731-752 | account defaults RNG, route-token RNG, KEM keypair, signature keypair | vault init occurs first; profile alias and defaults are written before identity generation; identity secrets/public record are later | `tui_init_reject`; `account_init_failed`; `identity_init_failed` | `qsl/qsl-client/qsc/tests/tui_system_account_destroy.rs` covers successful init identity/public-record presence | enough for observation, not enough for combined account/identity no-partial proof | possible, but account-default writes and route-token seam make it separate | no | no | later TUI bootstrap test path | forced bootstrap identity failure must truthfully distinguish allowed prior account/default writes from forbidden partial identity writes and account selection state | high residual after lazy |
| public record write/update | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 167-185 | no RNG; writes public KEM/signature keys generated elsewhere | after successful key generation and secret store for lazy/rotate/bootstrap; after migration secret import for legacy | write failure maps to caller-specific markers | success and some failure paths covered indirectly | yes for file assertions | not itself a provider RNG seam; test as downstream write invariant | no | no | included in selected lazy test | public record must remain absent or unchanged when selected pre-write provider RNG failure is forced | selected downstream invariant |
| selected identity write/update | no separate selected-identity file observed; CLI uses `--as`; TUI bootstrap uses fixed `self`; TUI command dispatch reads `QSC_SELF_LABEL` | no direct provider operation | label selection is argument/env driven, not a separate write in selected lazy path | none | selection behavior covered indirectly by CLI/TUI tests | observation enough | not a seam target | no | no | included as negative assertion in selected lazy test | forced lazy failure must not create a separate selected identity state; if future implementation discovers one, it must assert unchanged | selected negative invariant |
| vault identity-secret write/update | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 53-79 and 111-137; TUI passphrase equivalents in `locked.rs` lines 598-609 | no RNG; stores generated KEM/signature secret material | downstream of provider keypair generation | `identity_secret_store`; `identity_secret_unavailable` | NA-0458 asserts absent KEM/signature secrets for generic KEM keypair failure | yes for mock-vault assertions | yes as downstream invariant | no | no | included in selected lazy test | forced pre-write provider RNG failure writes no KEM or signature secret | selected downstream invariant |
| X25519 / ephemeral backlog | `qsl/qsl-client/qsc/src/handshake/mod.rs` line 1145 and responder path around line 1730 | X25519 ephemeral keypair | handshake A1/B1 path-specific | none | normal handshake tests only | not enough for this lane | possible but separate audit/implementation lane | no | no | future X25519 test path | separate no-output/no-mutation invariant; not identity | excluded |
| refimpl provider RNG | `tools/refimpl/**` | refimpl provider operations | separate implementation boundary | not inventoried in qsc identity path | refimpl `pqkem768` validation only | not needed for qsc lazy identity | no for current qsc lane | no | no | future refimpl authorization path | separate refimpl boundary; must not drift qsc scope | excluded |

Marker:

- `NA0464_IDENTITY_SPLIT_TARGET_INVENTORY_OK`

## lazy identity review

Lazy identity generation is implemented by `identity_self_kem_keypair` in
`qsl/qsl-client/qsc/src/identity/mod.rs` lines 306-339. It is called from the
handshake path when qsc needs the local identity keypair and no existing self
identity has been loaded.

Write timing:

- The function validates the label and config directory.
- It checks whether `identities/self_<label>.json` already exists.
- If no public record exists, it generates KEM key material and signature key
  material.
- Only after key generation does it store the KEM secret, store the signature
  secret, and write the public record.

Existing evidence:

- NA-0458 already proves generic forced `QSC.KEM.KEYPAIR` failure through the
  handshake/lazy identity path writes no Alice public identity, no identity KEM
  secret, no identity signature secret, no pending state, no session state, and
  no A1 output.
- That evidence is background KEM evidence only. It does not cover lazy
  identity signature keypair failure.
- That evidence does not make an identity-complete claim.
- That evidence does not make an RNG-failure-complete claim.
- That evidence does not make a provider-RNG-complete claim.
- That evidence does not make a signature-complete claim.
- That evidence does not make a crypto-complete claim.

Future implementation readiness:

- The exact source path is clear: `qsl/qsl-client/qsc/src/identity/mod.rs`.
- The exact test path is clear:
  `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`.
- A future cfg-only test seam is enough; no fake provider, dependency change,
  Cargo change, workflow change, or refimpl change is needed.
- Future implementation should use identity-specific cfg labels for lazy KEM
  keypair and lazy signature keypair failure so the evidence does not overstate
  generic KEM provider coverage as all identity-provider coverage.
- The future no-partial-write invariant is clear: forced lazy identity provider
  RNG failure before identity writes must leave no identity KEM secret, no
  identity signature secret, no self public record, no selected identity state,
  and no dependent handshake pending/session/output for the selected trigger
  path.

Classification:

`LAZY_IDENTITY_PROVIDER_RNG_IMPLEMENTATION_READY`

Marker:

- `NA0464_LAZY_IDENTITY_PROVIDER_RNG_IMPLEMENTATION_READY_OK`

## legacy / public-record upgrade review

Legacy/public-record upgrade is separate from lazy identity generation.

Source evidence:

- Legacy migration is in `qsl/qsl-client/qsc/src/identity/mod.rs` lines
  188-224.
- Public-record-without-signature upgrade is in
  `qsl/qsl-client/qsc/src/identity/mod.rs` lines 255-260.

Why it is separate:

- The path starts with existing identity/public-record material.
- It may store or update secrets and then rewrite an existing public record.
- The truthful invariant is not simply "nothing exists"; it must preserve
  existing KEM material while proving no partial signature-secret or public
  record upgrade.

It can likely reuse the future identity signature keypair seam shape, but it
needs a separate lane and separate fixture because the starting state and
assertions differ from lazy identity.

Classification:

`LEGACY_IDENTITY_UPGRADE_DEFER_AFTER_LAZY`

Marker:

- `NA0464_LEGACY_IDENTITY_UPGRADE_DEFER_AFTER_LAZY_OK`

## CLI rotate identity review

CLI identity rotation is implemented in `qsl/qsl-client/qsc/src/main.rs` lines
977-1040.

Why it is separate:

- It is an explicit user state transition, not lazy bootstrap.
- It requires unlocked state and confirmation.
- It writes new identity KEM and signature secrets, then writes a new public
  record.
- It may reset peers when `reset_peers` is selected.
- A future no-partial-write test must preserve the previous identity and peer
  state on forced pre-write provider RNG failure.

CLI rotation is exact enough to inspect but too stateful to combine with lazy
identity. It should be deferred until the smaller lazy identity seam is merged.

Classification:

`CLI_ROTATE_IDENTITY_DEFER_AFTER_LAZY`

Marker:

- `NA0464_CLI_ROTATE_IDENTITY_DEFER_AFTER_LAZY_OK`

## TUI account bootstrap identity review

TUI account bootstrap identity generation is implemented in
`qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`.

Source evidence:

- Account defaults and account verification seed writes occur at lines 526-578.
- Identity key generation and identity writes occur at lines 581-611.
- Account initialization calls defaults before identity at lines 614-620.
- TUI init calls this account initializer at lines 151-176 and 731-752.

Why it is separate:

- TUI account bootstrap is a user-flow boundary.
- Vault initialization, alias, settings, account verification seed, relay
  settings, and inbox route token writes can occur before identity generation.
- A future test must distinguish allowed pre-identity account/default writes
  from forbidden partial identity writes.
- It should remain separate from qsc CLI identity rotation and lazy handshake
  identity work.

Classification:

`TUI_ACCOUNT_BOOTSTRAP_IDENTITY_DEFER_AFTER_LAZY`

Marker:

- `NA0464_TUI_BOOTSTRAP_IDENTITY_DEFER_AFTER_LAZY_OK`

## identity combined vs first implementation candidate decision

Option 1 - lazy identity implementation next:

- Selected.
- Evidence: exact path, pre-write generation timing, existing KEM background
  evidence, and a clear future no-partial-identity-state invariant.
- Future paths: `qsl/qsl-client/qsc/src/identity/mod.rs`;
  `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`;
  NA-0465 evidence/testplan; `DECISIONS.md`; `TRACEABILITY.md`; rolling journal.
- Future validation: cfg forced failure for lazy KEM and lazy signature keypair;
  normal no-cfg production-semantics control; inherited qsc/refimpl/audit/formal
  checks.
- Public-claim caveat: bounded lazy identity evidence only.

Option 2 - legacy/public-record upgrade next:

- Rejected for first implementation.
- Evidence: starts from existing identity material and needs preservation
  assertions that differ from lazy identity.
- Public-claim caveat: would still be bounded upgrade evidence only.

Option 3 - CLI rotate identity next:

- Rejected for first implementation.
- Evidence: explicit rotation has prior-state and optional peer-reset concerns.
- Public-claim caveat: would still be bounded CLI rotation evidence only.

Option 4 - TUI account bootstrap identity next:

- Rejected for first implementation.
- Evidence: account/default writes precede identity generation, so the invariant
  is a user-flow/account-state invariant rather than the smallest identity
  invariant.
- Public-claim caveat: would still be bounded TUI bootstrap evidence only.

Option 5 - identity split-further authorization:

- Rejected.
- Evidence: lazy identity is sufficiently exact for a future implementation
  lane.
- Public-claim caveat: not needed before the lazy identity lane.

Option 6 - combined identity implementation:

- Rejected.
- Evidence: lazy identity, legacy upgrade, CLI rotation, and TUI bootstrap do
  not share one uniform starting state or write invariant.
- Public-claim caveat: combined implementation has no supported identity-complete
  implication.

Option 7 - refimpl-first:

- Rejected.
- Evidence: qsc lazy identity can proceed without refimpl provider boundary
  mutation.
- Public-claim caveat: refimpl provider RNG remains residual.

Option 8 - documentation-only:

- Rejected.
- Evidence: a future bounded lazy identity test is feasible without public
  overclaim.
- Public-claim caveat: all future tests remain bounded evidence only.

Option 9 - next audit domain:

- Rejected for the immediate successor.
- Evidence: identity provider RNG still has a small exact qsc implementation
  target before moving to KEM/signature/transcript audit.
- Public-claim caveat: transcript audit remains future-gated.

## identity split-scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | State/write timing | Future mutation type | Production-semantics risk | No-partial-write invariant clarity | Evidence value | Scope size | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| lazy identity implementation | lazy self identity generation | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs` | missing identity; provider generation before identity secrets/public record and dependent handshake state/output | cfg-only source seam plus executable qsc test | low if cfg-only and inactive in normal builds | high | closes smallest identity-provider RNG gap | small | no | yes | exact path and invariant | bounded lazy evidence only; no identity-complete claim | G1-G5 |
| legacy/public-record upgrade implementation | legacy record import and missing `sig_pk` upgrade | `qsl/qsl-client/qsc/src/identity/mod.rs`; future upgrade test path | existing identity material; upgrade writes secrets/public record | cfg-only source seam plus fixture-heavy test | medium | medium | valuable after lazy | medium | yes, after lazy | no | different starting state | no identity-complete claim | G1-G5 |
| CLI rotate identity implementation | explicit CLI rotation | `qsl/qsl-client/qsc/src/main.rs`; future rotate test path | confirmed unlocked command; writes new secrets/public record; optional peer reset | cfg-only source seam plus state-preservation test | medium | medium | valuable after lazy | medium | yes, after lazy | no | stateful rotation and reset surface | no identity-complete claim | G1-G5 |
| TUI account bootstrap identity implementation | TUI init account identity | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`; future TUI bootstrap test path | vault/account/default writes before identity | cfg-only source seam plus account-state test | medium | medium-low until split | valuable after lazy/CLI | medium-large | yes | no | account-flow boundary differs | no identity-complete or product-readiness claim | G1-G5 |
| identity split-further authorization | path-specific identity narrowing | governance/testplan only | no implementation | docs/governance only | low | not needed for lazy | lower than implementation | small | no | no | lazy is exact enough | internal governance only | G4-G5 |
| combined identity implementation | all identity paths | identity/main/TUI plus tests | multiple non-uniform write timings | broad source/test mutation | high | low | high only if correct, but too broad | large | yes | no | non-uniform invariants | no supported identity-complete implication | G1-G5 |
| refimpl-first | refimpl provider RNG boundary | `tools/refimpl/**` future lane | separate implementation boundary | refimpl authorization | medium | separate | not needed for qsc lazy | medium | no for lazy | no | qsc can proceed first | refimpl residual remains | G4 |
| documentation-only | claim boundary only | docs/governance future lane | no implementation | docs only | low | no test evidence | low | small | no | no | bounded lazy implementation is feasible | no public claim expansion | G4-G5 |
| KEM/signature/transcript audit | next audit domain | future audit evidence/testplan | read-only audit | docs/read-only | low | audit-specific | valuable later | medium | no for this decision | no | identity still has exact first target | no crypto-complete or public-readiness claim | G1-G5 |

## authorization decision

Primary classification:

`IDENTITY_SPLIT_LAZY_IDENTITY_NEXT`

Selected successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

Exact future implementation paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

This directive does not implement NA-0465. It does not mutate runtime, crypto,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE, qwork/qstart/
qresume/qshell, qsl-backup, backup status, backup plan, rollback, or backup
tree paths.

Markers:

- `NA0464_IDENTITY_SPLIT_LAZY_IDENTITY_NEXT_OK`
- `NA0464_NO_IMPLEMENTATION_MUTATION_OK`

## successor selection

NA-0465 successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

Successor rationale:

- It is the smallest exact identity-provider RNG implementation target.
- It can preserve production semantics with cfg-only code.
- It does not need refimpl, dependency, workflow, Cargo, lockfile, fuzz target,
  vector, formal model, public docs, qsl-server, qsl-attachments, qshield-cli,
  backup, restore, or qsl-backup mutation.
- It does not imply identity-complete status.
- It does not imply signature-complete status.
- It does not imply RNG-failure-complete status.
- It does not imply provider-RNG-complete status.
- It does not imply crypto-complete status.
- It does not imply public readiness.
- It does not imply production readiness.
- It does not imply vulnerability-free status.
- It does not imply bug-free status.
- It does not imply perfect-crypto status.

Marker:

- `NA0464_SUCCESSOR_NA0465_SELECTED_OK`

## future path/scope bundle

Future NA-0465 allowed paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0465 allowed implementation shape:

- cfg-only lazy identity provider RNG failure seam in `identity/mod.rs`;
- forced failure before lazy identity secret/public-record writes;
- normal no-cfg build ignores `QSC_RNG_FAILURE_TEST_SEAM`;
- no fake provider or refimpl mutation by default;
- no dependency, Cargo, lockfile, workflow, vector, fuzz target, formal model,
  qsl-server, qsl-attachments, qshield-cli, public docs, website, README,
  START_HERE, backup, restore, qsl-backup, status, plan, rollback, or backup
  tree mutation.

Future forbidden unless later exact scope authorizes:

- runtime or crypto changes outside `qsl/qsl-client/qsc/src/identity/mod.rs`;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- executable test changes outside the exact NA-0465 test path;
- fuzz target source changes;
- vector changes;
- formal model changes;
- refimpl changes;
- public docs or website changes;
- qsl-server or qsl-attachments changes;
- qshield-cli changes;
- backup/restore/qsl-backup changes;
- public claims.

## future validation/marker plan

Common NA-0465 markers:

- `NA0465_IDENTITY_SPLIT_CONSUMED_OK`
- `NA0465_NEXT_SCOPE_SELECTED_OK`
- `NA0465_NO_DEPENDENCY_CHANGE_OK`
- `NA0465_NO_WORKFLOW_CHANGE_OK`
- `NA0465_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0465_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0465_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0465_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0465_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0465_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0465_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0465_ONE_READY_INVARIANT_OK`

Lazy identity implementation markers:

- `NA0465_LAZY_IDENTITY_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0465_LAZY_IDENTITY_PROVIDER_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0465_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Future validation:

- cfg `lazy_identity_provider_rng_failure` test proving forced failure and no
  partial identity state;
- no-cfg `lazy_identity_provider_rng_failure` test proving normal production
  semantics;
- inherited cfg/no-cfg A2, B1, KEM, residual RNG, key lifecycle, provider-error,
  qsc send_commit, refimpl pqkem768, root audit, nested fuzz audit, qsc
  adversarial, format, and formal checks as directed by NA-0465.

## public claim/external review/website boundary

qsc identity split-scope authorization is internal governance evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not public readiness.
- It is not external-review completion.
- It is not a public technical paper.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not RNG-failure-complete proof.
- It is not provider-RNG-complete proof.
- It is not signature-complete proof.
- It is not identity-complete proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.

No README, START_HERE, public-doc, website, public technical paper, qshield,
qsl-server, or qsl-attachments public claim is updated.

Cargo audit green is dependency-health evidence only, not public readiness, and
not vulnerability-free proof.

Future tests, if authorized, must be described as bounded evidence only.

## rejected alternatives

- Combined identity implementation was rejected because lazy identity,
  legacy/public-record upgrade, CLI rotation, and TUI account bootstrap do not
  share one uniform no-partial-write invariant.
- Legacy/public-record upgrade first was rejected because it starts from
  existing identity material and must preserve existing state differently.
- CLI identity rotation first was rejected because it is an explicit state
  transition with prior identity and optional peer-reset concerns.
- TUI account bootstrap first was rejected because account/default writes
  precede identity generation and require a separate account-state invariant.
- Refimpl-first was rejected because qsc lazy identity can proceed without
  refimpl provider boundary mutation.
- Documentation-only was rejected because a bounded future lazy identity test is
  feasible without public overclaim.
- KEM/signature/transcript audit next was rejected for the immediate successor
  because identity provider RNG still has an exact first qsc target.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths.

Read-only qsl-backup boundary proof:

- `/usr/local/sbin/qsl-backup` SHA matched
  `e9ecff3d22ed`.
- The qsl-backup source-list inclusion count for the Codex ops path was 1.

This lane changes only qsl-protocol governance/testplan/journal paths and has
no backup-plan update requirement.

## next recommendation

After this evidence PR merges and post-merge public-safety is green, close out
NA-0464 and restore exactly one READY successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

NA-0465 should implement only the lazy identity provider RNG failure test-seam
scope selected here and should not implement legacy/public-record upgrade, CLI
identity rotation, TUI account bootstrap, X25519 / ephemeral generation, refimpl
provider RNG, qshield-cli demo RNG, formal/model RNG, fuzz/vector RNG, public
docs, website, qsl-server, qsl-attachments, backup, restore, or public claims.
