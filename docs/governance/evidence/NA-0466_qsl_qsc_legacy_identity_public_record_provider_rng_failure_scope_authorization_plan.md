Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0466 QSL qsc Legacy Identity Public-Record Provider RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0466 consumes the NA-0465 lazy identity implementation result and authorizes
the next exact qsc identity-provider RNG failure lane.

Primary classification:

`LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY`

Selected successor:

`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness`

The legacy/public-record identity upgrade path is RNG-bearing because current
`qsl/qsl-client/qsc/src/identity/mod.rs` generates a signature keypair when it
upgrades either a legacy plaintext identity record or a public record with
missing `sig_pk`. This is not the same state surface as lazy identity. Lazy
identity starts from no self identity and proves no new state exists after
forced failure. Legacy/public-record upgrade starts from existing identity
material and must prove existing state remains stable while no partial signature
secret or public-record upgrade is written.

The future implementation can be bounded to:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz-target, vector, formal-model, refimpl, qsl-server, qsl-attachments,
qshield runtime, qshield-cli, website, public-doc, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup-status,
backup-plan, rollback, or backup-tree mutation is performed by NA-0466.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made. Cargo audit green is
dependency-health evidence only.

Required NA-0466 markers recorded by this evidence:

- `NA0466_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0466_NA0465_INHERITANCE_CONSUMED_OK`
- `NA0466_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0466_LEGACY_PUBLIC_RECORD_TARGET_INVENTORY_OK`
- `NA0466_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_RELEVANT_OK`
- `NA0466_LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY_OK`
- `NA0466_SUCCESSOR_NA0467_SELECTED_OK`
- `NA0466_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0466_NO_DEPENDENCY_CHANGE_OK`
- `NA0466_NO_WORKFLOW_CHANGE_OK`
- `NA0466_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0466_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0466_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0466_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0466_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0466_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0466_ONE_READY_INVARIANT_OK`

## Live NA-0466 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0466.
- NA-0465 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0918.
- D-0917 exists once.
- D-0918 exists once.
- D-0919 was absent before this patch.
- D-0920 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0466 mutation paths are exactly:

- `docs/governance/evidence/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, refimpl,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback, backup tree paths, public technical paper
content, and public-claim expansion.

Acceptance criteria:

- qwork proof files are verified without rerunning qwork.
- NA-0465 inheritance is consumed.
- legacy/public-record identity provider RNG candidates are inventoried by
  exact path, operation, state timing, and future invariant.
- one primary classification and one NA-0467 successor are selected.
- exact future paths are recorded because implementation is selected.
- public claim caveats remain explicit.
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1200
not merged, origin/main not equal to or descending from `c6addc0ce436`,
missing D-0918, D-0919 already present at start, unconsumable NA-0465
inheritance, unsafe legacy/public-record classification, unsafe successor
selection, failed root or nested audit, qsl-backup source-list regression,
public-safety red or missing, more than one READY item, or any forbidden
mutation/public overclaim.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0466/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0466/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0466`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0466/qsl-protocol`
- proof HEAD: `c6addc0ce436`
- proof `origin/main`: `c6addc0ce436`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0466`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`c6addc0ce436`. Fetch did not advance `origin/main`. PR #1200 was verified
merged at `c6addc0ce436`.

Proof root:

`/srv/qbuild/tmp/NA0466_qsc_legacy_identity_public_record_scope_20260612T131532Z`

Marker:

- `NA0466_QWORK_PROOF_FILE_VERIFIED_OK`

## NA-0465 inheritance

NA-0465 / D-0917 implemented the bounded lazy identity provider RNG failure
test seam:

- cfg-only lazy KEM label: `QSC.IDENTITY.LAZY.KEM_KEYPAIR`;
- cfg-only lazy signature label: `QSC.IDENTITY.LAZY.SIG_KEYPAIR`;
- inherited generic KEM label preserved: `QSC.KEM.KEYPAIR`;
- forced lazy KEM failure returns sanitized `identity_secret_unavailable` /
  `rng_failure_forced` before identity secret, self public record, selected
  identity, pending handshake, session, or relay output writes;
- forced lazy signature failure returns sanitized `identity_secret_unavailable`
  / `rng_failure_forced` after in-memory lazy KEM generation but before identity
  secret, self public record, selected identity, pending handshake, session, or
  relay output writes;
- normal no-cfg behavior is unchanged.

D-0918 closed NA-0465 and restored NA-0466 as the sole READY item.

Inherited residuals:

- Legacy/public-record identity upgrade remains deferred until this
  authorization.
- CLI identity rotation remains deferred.
- TUI account bootstrap remains deferred.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- qshield-cli demo RNG remains demo-local residual.
- qsc KEM, B1 signing, A2 signing, lazy identity, route/contact/attachment,
  base RNG seam, key lifecycle, and provider-error evidence remain bounded
  background evidence only.
- no identity-complete claim exists.
- no signature-complete claim exists.
- no RNG-failure-complete claim exists.
- no provider-RNG-complete claim exists.
- no crypto-complete claim exists.
- no public-readiness claim exists.

Marker:

- `NA0466_NA0465_INHERITANCE_CONSUMED_OK`

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- Distinguish legacy/public-record upgrade from lazy identity. Lazy identity
  starts with no self record; legacy/public-record upgrade starts with existing
  KEM public material and, depending on the subpath, existing plaintext legacy
  KEM secret or an existing vault KEM secret.
- Define the future no-partial-upgrade invariant before implementation: forced
  signature-provider RNG failure must leave the existing public record stable,
  leave existing KEM material stable, write no new signature secret, and emit no
  upgraded public record.
- Do not bundle CLI rotation, TUI account bootstrap, X25519 / ephemeral, or
  refimpl work.
- Preserve production semantics and public-claim caveats.

CI / Dependency / Release Health Steward:

- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock audit is green.
- Lazy identity provider RNG cfg/no-cfg tests are green.
- A2, B1, and KEM provider RNG seam cfg/no-cfg tests are green.
- Base cfg/no-cfg RNG failure tests are green.
- qsc key lifecycle and provider-error tests are green.
- refimpl `pqkem768` validation remains required background validation.
- qsc adversarial CI is green on current main; the local script marker is
  present.
- Public-safety is green on current main.
- Cargo audit green is dependency-health evidence only; it is not vulnerability-free proof.

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
- Legacy/public-record authorization is internal governance evidence only.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local and is not promoted by this lane.
- qsl-server and qsl-attachments remain service boundaries.
- No qshield, website, or public-service readiness claim is made.

Local Ops / Backup / Restore Steward:

- No backup, restore, qsl-backup, backup-status, backup-plan, rollback, or
  backup-tree mutation is performed.
- qsl-backup SHA/source-list proof remains read-only boundary evidence only.

Marker:

- `NA0466_STEWARD_REVIEW_TEMPLATE_USED_OK`

## legacy / public-record identity target inventory

| Candidate | Exact source path(s) | Provider operation | Initial state requirement | State/write timing | Existing marker/error | Existing test coverage | Existing APIs enough? | Future cfg-only seam enough? | Fake provider/injection needed? | Refimpl changes needed? | Future test path if selected | Truthful no-partial-upgrade invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| existing identity with missing self public record | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 271-305 and 335-349 | none if self public record is absent; lazy path handles missing record | no `identities/self_<label>.json` exists | path returns `None`, then lazy identity generation occurs | `identity_secret_unavailable` only on downstream lazy failure | NA-0465 lazy identity tests cover the missing-record path | yes | already covered by NA-0465 labels | no | no | not selected by NA-0466 | no self-record state exists; lazy invariant remains NA-0465-only | covered background |
| public-record upgrade / regeneration path | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 284-290 | signature keypair generation with `hs_sig_keypair()` when `rec.sig_pk` is empty | `IdentityPublicRecord` exists with KEM public key and empty `sig_pk`; existing vault KEM secret exists | loads existing KEM secret, generates signature keypair, stores signature secret, then rewrites public record | `identity_secret_store`; `identity_secret_unavailable` | indirect legacy/read coverage; no forced provider RNG upgrade test | yes for file/vault assertions | yes, with a separate public-record-upgrade label | no | no | `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs` | forced signature RNG failure leaves existing empty-`sig_pk` public record unchanged, leaves existing KEM secret unchanged, writes no `identity.sig_sk.<label>`, and emits no dependent handshake output | selected |
| identity KEM keypair generation in upgrade path | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 217-305 | none in legacy/public-record upgrade; existing KEM public/secret material is reused | existing legacy or public-record state | no KEM keypair generation occurs in selected upgrade branches | not applicable | NA-0465 covers lazy KEM generation; NA-0458 covers generic KEM background | yes | not a seam target for this lane | no | no | not selected | KEM material must remain stable, not regenerated | downstream invariant |
| identity signature keypair generation in upgrade path | `qsl/qsl-client/qsc/src/identity/mod.rs` line 223 and lines 286-290 | signature keypair generation with `hs_sig_keypair()` | legacy record or public record missing `sig_pk` | signature keypair is generated before signature secret store and public-record rewrite | `identity_secret_migrate`; `identity_secret_unavailable` | no forced provider RNG upgrade test | yes | yes, with separate labels for legacy migration and public-record upgrade | no | no | `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs` | forced failure leaves existing state stable and writes no partial signature upgrade | selected |
| public record write/update | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 196-215, 248-253, and 288-289 | no RNG; writes public KEM/signature keys generated elsewhere | generated signature public key available | after successful signature keypair generation and signature secret store | write errors map through caller | success and failure paths covered indirectly | yes | not a seam target; downstream invariant only | no | no | selected future test | public record must remain byte-for-byte unchanged when forced failure occurs before upgrade writes | selected downstream invariant |
| identity secret write/update | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 82-107, 140-165, 224-247, and 287-289 | no RNG; stores existing KEM secret or generated signature secret | vault available and generated/legacy secret available | legacy migration stores KEM secret then signature secret; public-record upgrade stores signature secret before public-record rewrite | `identity_secret_store`; `identity_secret_migrate`; `identity_secret_unavailable` | NA-0465 tests assert absent identity secrets for lazy failures | yes | downstream assertion after forced signature RNG failure | no | no | selected future test | forced failure before signature keypair success writes no signature secret and does not import legacy KEM secret | selected downstream invariant |
| selected identity write/update if applicable | no separate selected-identity file observed; CLI uses `--as`; TUI uses fixed/self or env-driven labels | no direct provider operation | label chosen by argument or TUI/account context | no separate selected-identity write in selected qsc identity module path | none | selection behavior covered indirectly | observation enough | not a seam target | no | no | future test must assert no unexpected selected-state write if discovered | no selected-identity state changes on forced upgrade failure | negative invariant |
| lazy identity seam as completed background | `qsl/qsl-client/qsc/src/identity/mod.rs` lines 30-57 and 335-380; `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs` | KEM and signature keypair for missing self identity | no self public record exists | provider keypairs generated before any identity writes | `identity_secret_unavailable` / `rng_failure_forced` | NA-0465 cfg/no-cfg tests are green | yes | already implemented | no | no | none for NA-0466 | bounded lazy no-partial-state evidence remains unchanged | completed background |
| CLI identity rotation as excluded background | `qsl/qsl-client/qsc/src/main.rs` lines 977-1040 | KEM and signature keypair for explicit rotation | unlocked CLI identity rotation with confirmation | generates new secrets, writes public record, optional peer reset follows | `identity_rotate`; `identity_secret_unavailable`; `identity_rotate_write_failed` | `identity_ux.rs` and handshake tests cover success/confirm paths | enough for future observation | likely, but separate state transition | no | no | future CLI rotation lane | previous identity and peer state must remain stable on forced pre-write failure | excluded |
| TUI account bootstrap as excluded background | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` lines 151-176, 526-623, and 731-752 | account defaults RNG, route-token RNG, KEM keypair, signature keypair | locked TUI init/bootstrap flow | account/default writes can precede identity generation | `tui_init_reject`; `account_init_failed`; `identity_init_failed` | `tui_system_account_destroy.rs` covers successful init identity/public-record presence | enough for future observation | possible, but separate account-flow invariant | no | no | future TUI bootstrap lane | must distinguish allowed account/default writes from forbidden partial identity writes | excluded |
| X25519 / ephemeral backlog as excluded background | `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 689-717, 1145, and 1738 | X25519 ephemeral keypair | handshake A1/B1 path-specific | ephemeral generated in handshake source, not identity module | none for this lane | normal handshake tests only | not enough for this lane | separate seam required | no for NA-0466 | no | future X25519 lane | separate no-output/no-mutation invariant, not identity | excluded |
| refimpl provider RNG as excluded background | `tools/refimpl/**` | refimpl provider operations | separate implementation boundary | separate from qsc identity module | not applicable | refimpl `pqkem768` validation only | not needed for qsc legacy/public-record | no | no | future refimpl authorization path | separate refimpl boundary; must not drift qsc scope | excluded |

Marker:

- `NA0466_LEGACY_PUBLIC_RECORD_TARGET_INVENTORY_OK`

## legacy / public-record upgrade state timing review

Initial states that trigger the selected upgrade:

- Legacy migration: `identity_read_self_kem_keypair` reads a
  `IdentityLegacyRecord` containing `kem_pk` and `kem_sk`, then calls
  `identity_migrate_legacy`.
- Public-record upgrade: `identity_read_self_kem_keypair` reads an
  `IdentityPublicRecord` with an empty `sig_pk`, loads the existing KEM secret
  from the vault, then upgrades by generating a signature keypair.

Provider RNG relevance:

- Legacy migration generates new signature key material at
  `qsl/qsl-client/qsc/src/identity/mod.rs` line 223.
- Public-record upgrade generates new signature key material at
  `qsl/qsl-client/qsc/src/identity/mod.rs` line 287.
- Neither selected upgrade branch generates new KEM key material. KEM material
  is pre-existing and must remain stable.

Write timing:

- In legacy migration, signature keypair generation occurs before legacy KEM
  secret import, signature secret store, and public-record rewrite.
- In public-record upgrade, existing KEM secret load occurs first. Signature
  keypair generation then occurs before signature secret store and
  public-record rewrite.
- Forced provider RNG failure can be applied at each signature-generation point
  before upgrade writes.

Lazy separation:

- The lazy identity labels must not be reused. They intentionally describe a
  no-self-identity starting state.
- Future NA-0467 needs separate labels, recommended as:
  `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR` and
  `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`.

Future tests can prove:

- legacy migration forced failure leaves the legacy file unchanged, writes no
  vault KEM secret import, writes no signature secret, and emits no upgraded
  public record;
- public-record upgrade forced failure leaves the empty-`sig_pk` public record
  unchanged, preserves the existing vault KEM secret, writes no signature
  secret, and emits no dependent handshake output;
- normal no-cfg behavior remains unchanged.

Classification:

`LEGACY_PUBLIC_RECORD_PROVIDER_RNG_IMPLEMENTATION_READY`

Marker:

- `NA0466_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_RELEVANT_OK`

## invariant options review

Option 1 - no partial public-record write:

- Select.
- Evidence: both upgrade branches write the public record only after signature
  keypair generation succeeds.
- Future exact paths: `identity/mod.rs`; future
  `legacy_identity_public_record_provider_rng_failure.rs`.
- Future validation markers:
  `NA0467_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`.
- Public-claim caveat: bounded internal qsc evidence only; no identity-complete claim is made.

Option 2 - existing identity state stable:

- Select.
- Evidence: the selected upgrade path starts from existing legacy/public-record
  state rather than no state.
- Future exact paths: `identity/mod.rs`; future
  `legacy_identity_public_record_provider_rng_failure.rs`.
- Future validation markers:
  `NA0467_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`.
- Public-claim caveat: bounded state-stability evidence only; no provider-RNG-complete claim is made.

Option 3 - no dependent handshake state/output:

- Select as downstream evidence if the trigger path uses handshake or another
  command that would otherwise continue after identity upgrade.
- Evidence: NA-0465 already uses command/output assertions for dependent
  handshake state; legacy/public-record upgrade can reuse that assertion shape.
- Future exact paths: same qsc identity source/test scope.
- Future validation markers:
  `NA0467_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`.
- Public-claim caveat: no RNG-failure-complete claim is made.

Option 4 - not RNG-relevant documentation:

- Reject.
- Evidence: both selected upgrade branches call `hs_sig_keypair()`.
- Future exact paths: not applicable.
- Future validation markers: not applicable.
- Public-claim caveat: do not claim the path is non-RNG-bearing.

Option 5 - design-change/rollback semantics:

- Reject for NA-0467.
- Evidence: future implementation can add cfg-only test seam labels and tests
  without changing no-cfg production semantics.
- Future exact paths: not selected.
- Future validation markers: not applicable.
- Public-claim caveat: any future transactional design change would require a
  separate authorization lane.

## implementation readiness review

Implementation readiness classification:

`LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY`

Readiness answers:

- `identity/mod.rs` is enough for future implementation because both signature
  keypair generation points are in that file.
- One future qsc test file is enough to prove both selected subpaths and normal
  no-cfg production semantics.
- Future implementation does not require source changes outside
  `identity/mod.rs`.
- Future implementation does not require refimpl mutation.
- Future implementation does not require dependency, Cargo, lockfile, or
  workflow mutation.
- Normal production semantics can be proven unchanged with a no-cfg test that
  sets the future selectors and observes successful upgrade.
- The selected future test must not make an identity-complete claim; it asserts only legacy/public-record upgrade behavior.
- Exact labels, paths, and invariants are clear enough for implementation.

Marker:

- `NA0466_LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY_OK`

## legacy vs CLI / TUI / next domain decision

Option 1 - legacy/public-record implementation next:

- Select.
- Evidence: exact source paths, RNG relevance, pre-upgrade-write timing, and a
  truthful no-partial-upgrade invariant exist.
- Future exact paths:
  `qsl/qsl-client/qsc/src/identity/mod.rs` and
  `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`.
- Future validation: cfg forced-failure tests for legacy migration and
  public-record upgrade plus no-cfg production-semantics control.
- Public-claim caveat: bounded qsc legacy/public-record evidence only.

Option 2 - legacy/public-record documentation-only next:

- Reject.
- Evidence: the path is RNG-bearing and testable without public overclaim.
- Future exact paths: not selected.
- Future validation: not applicable.
- Public-claim caveat: no documentation-only claim is needed.

Option 3 - CLI identity rotation scope next:

- Reject for immediate successor.
- Evidence: CLI rotation remains valuable but is a separate explicit user state
  transition with peer-reset considerations.
- Future exact paths if later selected:
  `qsl/qsl-client/qsc/src/main.rs` and a future CLI rotation test file.
- Future validation: preserve previous identity and peer state on forced
  pre-write provider failure.
- Public-claim caveat: bounded CLI rotation evidence only.

Option 4 - TUI account bootstrap scope next:

- Reject for immediate successor.
- Evidence: TUI bootstrap mixes account/default writes and identity generation,
  so it needs a separate account-flow invariant.
- Future exact paths if later selected:
  `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` and a future TUI
  bootstrap test file.
- Future validation: distinguish allowed account/default writes from forbidden
  partial identity writes.
- Public-claim caveat: bounded TUI bootstrap evidence only.

Option 5 - KEM / Signature / Transcript Binding Read-Only Audit next:

- Reject for immediate successor.
- Evidence: identity-provider RNG work still has one exact, valuable qsc
  implementation lane before broader audit work.
- Future exact paths: read-only audit paths would be selected by a later lane.
- Future validation: audit evidence, not implementation tests.
- Public-claim caveat: no crypto-complete claim is made.

Option 6 - refimpl provider RNG boundary next:

- Reject for immediate successor.
- Evidence: qsc legacy/public-record scope can progress without refimpl
  mutation.
- Future exact paths: `tools/refimpl/**` only under a separate authorization.
- Future validation: refimpl provider boundary tests.
- Public-claim caveat: refimpl provider RNG remains residual.

## legacy / public-record scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | Initial state | State/write timing | Future mutation type | Production-semantics risk | Truthful invariant clarity | Evidence value | Scope size | RNG relevance | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| legacy/public-record implementation | legacy migration and empty-`sig_pk` public-record upgrade | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs` | existing legacy/public-record identity state | signature keypair generated before upgrade writes | cfg-only source seam plus integration test | low if cfg-only and no-cfg test passes | high | high | small | yes | no | yes | exact RNG-bearing path and stable-state invariant | no identity-complete or provider-RNG-complete claim | G1-G5 |
| legacy/public-record documentation-only | claim boundary around legacy/public-record path | NA-0467 docs/testplan only | existing identity state | documentation only | governance only | low | medium | medium | small | yes, so docs-only is insufficient | no | no | implementation-ready path exists | no broader claim | G1-G5 |
| CLI rotate identity scope | explicit CLI identity rotation | `qsl/qsl-client/qsc/src/main.rs`; future CLI test | existing unlocked CLI state | new secrets/public record, optional peer reset | future separate seam/test | medium | medium | high later | medium | yes | no, but separate | no | more stateful than selected qsc identity module path | no identity-complete claim | G1-G5 |
| TUI account bootstrap identity scope | TUI init/bootstrap identity | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`; future TUI test | account bootstrap flow | account/default writes can precede identity writes | future separate seam/test | medium | medium-low until split | high later | medium-large | yes | yes for exact split | no | account-flow invariant differs | no product-readiness claim | G1-G5 |
| refimpl provider boundary | refimpl provider operations | `tools/refimpl/**` future lane | separate implementation | separate provider boundary | future authorization | medium | separate | medium later | medium | yes outside qsc | yes | no | qsc can progress without refimpl | refimpl residual remains | G4 |
| KEM/signature/transcript audit | broader read-only audit | canonical specs, qsc/refimpl/formal/inputs read-only | audit state | no mutation | read-only evidence | low | audit-specific | high later | medium | indirect | no | no | identity path still has exact implementation value | no crypto-complete claim | G1-G5 |
| identity work complete enough / no action | no successor | none | no work | no mutation | none | low | low | low | none | no | yes | no | residual exact RNG-bearing path exists | no completion claim supported | G1-G5 |

## authorization decision

Primary classification:

`LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY`

RNG relevance classification:

`LEGACY_PUBLIC_RECORD_PROVIDER_RNG_IMPLEMENTATION_READY`

Selected successor:

`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness`

Exact future implementation paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`

Future recommended cfg labels:

- `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR`
- `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`

NA-0466 performs no implementation mutation. NA-0466 performs no runtime,
crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz-target,
vector, formal-model, refimpl, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public-doc, README, START_HERE, qwork/qstart/qresume/
qshell, backup, restore, qsl-backup, backup-status, backup-plan, rollback, or
backup-tree mutation. Exactly one READY successor remains mandatory.

## successor selection

Selected:

`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness`

Rationale:

- The path is RNG-bearing because it generates signature key material.
- The path is exact enough for implementation.
- The future mutation can be bounded to `identity/mod.rs` plus one qsc test
  file.
- CLI rotation and TUI bootstrap remain deferred because they have different
  user-flow and state-write boundaries.
- refimpl remains deferred because qsc can progress without refimpl mutation.

Marker:

- `NA0466_SUCCESSOR_NA0467_SELECTED_OK`

## future path/scope bundle

If NA-0467 is restored, allowed qsl-protocol paths should be:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless a later exact scope authorizes it:

- runtime/crypto implementation changes outside exact paths;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- test source changes outside exact paths;
- fuzz target source changes;
- vector changes;
- formal model changes;
- refimpl changes;
- public docs or website changes;
- qsl-server or qsl-attachments changes;
- backup, restore, or qsl-backup changes;
- public claim expansion.

## future validation/marker plan

Common NA-0467 markers:

- `NA0467_LEGACY_PUBLIC_RECORD_SCOPE_CONSUMED_OK`
- `NA0467_NEXT_SCOPE_SELECTED_OK`
- `NA0467_NO_DEPENDENCY_CHANGE_OK`
- `NA0467_NO_WORKFLOW_CHANGE_OK`
- `NA0467_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0467_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0467_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0467_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0467_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0467_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0467_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0467_ONE_READY_INVARIANT_OK`

Legacy/public-record implementation markers:

- `NA0467_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0467_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`
- `NA0467_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Recommended subpath markers:

- `NA0467_LEGACY_MIGRATE_SIG_RNG_FAILURE_FORCED_OK`
- `NA0467_LEGACY_MIGRATE_SIG_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`
- `NA0467_PUBLIC_RECORD_UPGRADE_SIG_RNG_FAILURE_FORCED_OK`
- `NA0467_PUBLIC_RECORD_UPGRADE_SIG_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`

Required validation shape:

- cfg test forces `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR` and proves no
  partial legacy migration;
- cfg test forces `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR` and proves
  no partial public-record upgrade;
- no-cfg test sets both selectors and proves normal upgrade semantics still
  occur;
- inherited qsc provider RNG, base RNG failure, key lifecycle, provider-error,
  send_commit, refimpl, audit, format, and formal checks remain green.

## public claim/external review/website boundary

This authorization is internal governance evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not external-review completion.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not RNG-failure-complete proof.
- It is not provider-RNG-complete proof.
- It is not signature-complete proof.
- It is not identity-complete proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.
- It is not public technical paper content.
- No README, START_HERE, public docs, or website update is made.
- No public-readiness or public-security claim is made.
- Cargo audit green is dependency-health evidence only.
- Future tests, if restored and implemented, must be described as bounded qsc
  legacy/public-record evidence only.

## rejected alternatives

- Reusing lazy labels is rejected because lazy identity starts from no existing
  self identity, while legacy/public-record upgrade starts from existing state.
- Documentation-only is rejected because the path is RNG-bearing and testable.
- CLI rotation next is rejected for the immediate successor because it is a
  separate explicit user state transition.
- TUI bootstrap next is rejected for the immediate successor because account
  defaults and route-token work can precede identity generation.
- refimpl-first is rejected because qsc legacy/public-record scope can progress
  without refimpl mutation.
- KEM/signature/transcript audit next is rejected because this identity
  provider RNG residual still has exact implementation value.
- No action is rejected because residual RNG-bearing identity upgrade evidence
  remains.

## backup-impact statement

NA-0466 does not run backup or restore. NA-0466 does not mutate qsl-backup,
backup status files, backup plan files, rollback subtree paths, systemd, timers,
fstab, source lists, retention, backup scripts, or `/backup/qsl`.

Read-only qsl-backup boundary proof:

- `/usr/local/sbin/qsl-backup` SHA matched
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- source inclusion count for the codex ops path was exactly 1.

## next recommendation

After this evidence PR merges and post-merge public-safety is green, close out
NA-0466 and restore:

`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness`

The closeout must not implement NA-0467. Future NA-0467 should implement only
the selected `identity/mod.rs` cfg-only signature keypair test seam and the one
selected qsc test file, with no public claim expansion.
