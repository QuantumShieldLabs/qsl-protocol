Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0470 QSL qsc TUI Account Bootstrap Identity Provider RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0470 consumes D-0927 and NA-0469, inventories the qsc TUI account bootstrap
identity provider RNG surface, and classifies the next step.

Primary classification:

`TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`

Selected successor:

`NA-0471 -- QSL qsc TUI Account Bootstrap Transactionality Design Authorization Plan`

Read-only source review shows TUI account bootstrap generates both identity KEM
and signature keypairs, but it does so after the vault is created and after
alias, account defaults, verification seed, relay endpoint/token fields, and
TUI relay inbox token state have already been written. A future cfg-only
provider RNG seam in the identity generation call site could prove no partial
identity/public-record write, but it cannot truthfully prove no partial
account/bootstrap state under the current write order.

Therefore NA-0470 does not authorize an implementation lane. A future design
authorization lane must first decide whether TUI bootstrap should pre-generate
identity material before durable account/default writes, roll back durable
state after identity provider failure, or explicitly narrow the invariant to
identity-only evidence. That decision would alter bootstrap transactionality
semantics and must not be smuggled into a test-seam implementation.

NA-0470 mutates governance evidence only. It does not mutate runtime code,
crypto code, dependencies, Cargo manifests, lockfiles, workflows, executable
tests, fuzz targets, vectors, formal models, refimpl, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, backup/restore/local-ops paths,
qsl-backup, backup status files, backup plan files, rollback subtree paths, or
backup tree paths.

Markers recorded by this evidence:

- `NA0470_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0470_D0927_CONSUMED_OK`
- `NA0470_NA0469_INHERITANCE_CONSUMED_OK`
- `NA0470_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0470_TUI_BOOTSTRAP_TARGET_INVENTORY_OK`
- `NA0470_TUI_BOOTSTRAP_STATE_TIMING_REVIEW_OK`
- `NA0470_TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE_OK`
- `NA0470_SUCCESSOR_NA0471_TRANSACTIONALITY_DESIGN_SELECTED_OK`
- `NA0470_ASSURANCE_TRIGGER_HIGHER_PRIORITY_RESIDUAL_OK`
- `NA0470_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0470_NO_DEPENDENCY_CHANGE_OK`
- `NA0470_NO_WORKFLOW_CHANGE_OK`
- `NA0470_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0470_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0470_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0470_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0470_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0470_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0470_ONE_READY_INVARIANT_OK`

## Live NA-0470 scope

Startup proof showed:

- READY_COUNT 1.
- READY item: NA-0470.
- NA-0469 through NA-0435 DONE, except NA-0434 and NA-0429 BLOCKED.
- D-0925 exists once.
- D-0926 exists once.
- D-0927 exists once.
- D-0928 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0470 mutation paths are exactly:

- `docs/governance/evidence/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_testplan.md`
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
- D-0927 assurance recovery is consumed.
- NA-0469 inheritance is consumed.
- TUI account bootstrap identity provider RNG candidates are inventoried by
  exact path, provider operation, state timing, and future invariant.
- one primary classification and one NA-0471 successor are selected.
- exact future design scope is recorded because implementation is not selected.
- public claim caveats remain explicit.
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1209
not merged, origin/main not equal to or descending from `4f1d334826b4`, missing
D-0927, D-0928 already present at start, unconsumable D-0927 or NA-0469
inheritance, unsafe TUI bootstrap classification, unsafe successor selection,
failed root or nested audit, qsl-backup source-list regression, public-safety
red or missing, more than one READY item, or any forbidden mutation/public
overclaim.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0470/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0470/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0470`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0470/qsl-protocol`
- proof HEAD and proof `origin/main`: `4f1d334826b4`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0470`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch. Fetch
did not advance `origin/main`. PR #1209 was verified merged at `4f1d334826b4`.

Marker:

- `NA0470_QWORK_PROOF_FILE_VERIFIED_OK`

## D0927 assurance recovery consumption

D-0927 is consumed by NA-0470.

D-0927 recovered the D328 assurance addendum after NA-0469 implementation PR
#1207 and closeout PR #1208 had already merged. It required NA-0470 to consume
best-known-method, hostile cryptographer, red-team, production SRE,
side-channel, formal-model residual, external-review readiness, release-claim,
and assurance-gap trigger evidence before executing this lane.

Inherited classifications:

- `BEST_KNOWN_METHOD_FOR_SCOPE`
- `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`
- `EXTERNAL_REVIEW_READINESS_INCREMENTAL`
- `ASSURANCE_GAP_REVIEW_REQUIRED_AFTER_CURRENT_CHAIN`

NA-0470 consumes that recovery and does not widen NA-0469. The D-0927 default
next-lane pressure toward Assurance Gap Review is superseded here only by the
newly proven, higher-priority TUI bootstrap transactionality design residual.

Marker:

- `NA0470_D0927_CONSUMED_OK`

## NA-0469 inheritance

NA-0469 / D-0925 implemented bounded qsc CLI identity rotation provider RNG
failure labels:

- `QSC.IDENTITY.ROTATE.KEM_KEYPAIR`
- `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`

NA-0469 proved forced CLI rotation provider failures return sanitized
`identity_secret_unavailable` / `rng_failure_forced` before durable rotation
writes and preserve selected identity state, KEM/signature identity secrets,
self public record, contact/peer-reset state, dependent handshake/session
absence, and no-cfg production semantics.

Completed background boundaries:

- CLI identity rotation provider RNG seam is complete for the bounded CLI path.
- Lazy identity provider RNG seam is complete for its bounded path.
- Legacy/public-record provider RNG seam is complete for its bounded path.
- KEM provider RNG seam is complete for its bounded path.
- B1 signature provider RNG seam is complete for its bounded path.
- A2 signature provider RNG no-output seam is complete for its bounded path.

Residual boundaries:

- TUI account bootstrap identity provider RNG remains separate.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- qshield-cli demo RNG remains demo-local residual.
- formal/model RNG and fuzz/vector RNG remain residual.

No identity-complete claim exists.
No signature-complete claim exists.
No RNG-failure-complete claim exists.
No provider-RNG-complete claim exists.
No crypto-complete claim exists.
No side-channel-free claim exists.
No external-review-complete claim exists.
No production-ready claim exists.
No public-readiness claim exists.

Marker:

- `NA0470_NA0469_INHERITANCE_CONSUMED_OK`

## Applicable Stewardship and Assurance Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

### Crypto / Protocol Steward

- TUI bootstrap differs from CLI identity rotation, lazy identity, and
  legacy/public-record upgrade. TUI bootstrap starts from no TUI vault/account
  and combines vault creation, default account settings, verification seed,
  relay defaults, identity material, self public record, selected identity
  behavior, and user-visible setup output.
- Current code generates both KEM and signature identity keypairs in
  `init_identity_with_passphrase`, but only after earlier account/default
  writes. A truthful no-partial-account/bootstrap invariant is not available
  without transactionality design work.
- Protected surfaces are account-state, default route/account config, identity
  material, selected identity, self public record, setup outputs, and
  diagnostics.
- Production semantics and public-claim caveats must remain unchanged.

### CI / Dependency / Release Health Steward

- Current main public-safety was green before mutation.
- Root `cargo audit --deny warnings` was green before mutation.
- Nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` was green before mutation.
- CLI rotation, legacy/public-record, lazy identity, A2, B1, and KEM provider
  RNG seam tests are inherited green checks.
- qsc key lifecycle and provider-error tests are inherited green checks.
- refimpl `pqkem768` is an inherited green check.
- qsc adversarial script marker and syntax are validation checks; PR CI
  qsc-adversarial-smoke remains authoritative for the attached CI shape.
- Cargo audit green is dependency-health evidence only.

### Public Claims / External Review Steward

No RNG-failure-complete claim.
No provider-RNG-complete claim.
No signature-complete claim.
No identity-complete claim.
No crypto-complete claim.
No side-channel-free claim.
No vulnerability-free claim.
No bug-free claim.
No perfect-crypto claim.
No public-readiness claim.
No production-readiness claim.
No external-review-complete claim.
TUI account bootstrap authorization is internal governance evidence only.

### Product / Demo / Service Boundary Steward

qshield-cli remains demo-local. qsl-server and qsl-attachments remain service
boundaries. This lane makes no qshield claim. This lane makes no website claim.
This lane makes no public-service claim. This lane makes no production claim.
This lane makes no public-readiness claim.

### Local Ops / Backup / Restore Steward

No backup, restore, or local-ops mutation is authorized. qsl-backup hash and
source-list proof are boundary evidence only. Same-host continuity remains
separate from disaster recovery.

### Best-Known-Method Review

Classification: `BEST_KNOWN_METHOD_FOR_SCOPE`.

The best-known method for this authorization lane is read-only source inventory
plus explicit state-timing review before selecting implementation. Because the
timing proves earlier durable writes precede identity key generation, the best
method is to reject implementation and select a design authorization lane.

### Hostile Cryptographer Review

Top concerns:

- overclaim risk: bounded TUI bootstrap analysis must not be represented as all
  identity-provider RNG coverage.
- qsc/refimpl boundary risk: this lane inspects qsc only and does not move
  refimpl provider RNG evidence.
- transcript/identity-binding/formal mapping residual: bootstrap identity
  creation is not a formal transcript-binding proof and does not model peer
  identity binding.

### Red-Team Review

Top concerns:

- account-bootstrap partial state: current write order leaves vault/default
  state before identity generation, so simple forced identity RNG failure would
  produce partial bootstrap state.
- stale identity/public record and relay/server observation: identity failure
  must not leave misleading success output or relay-visible setup state.
- rollback/replay, peer-reset, or contact confusion: bootstrap failures must
  not be conflated with later contact or peer reset behavior.

### Production SRE Review

Top concerns:

- bootstrap failure incident handling: current UX can fail after earlier local
  writes, which needs a designed recovery story before release claims.
- logs/diagnostics/user confusion: failure output must be sanitized and must not
  imply setup completed.
- missing recovery playbook: production or public claims require an operator
  recovery playbook and explicit rollback/cleanup semantics.

### Side-Channel Caveat

No side-channel-free claim. No constant-time proof. No memory-erasure
completeness proof. No all secret-material lifecycle proof.

### Formal-Model Mapping Residual

Classification: `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.

Existing formal models are supporting evidence only. TUI account bootstrap
identity provider RNG failure and bootstrap transactionality are not directly
modeled by this lane.

### External-Review Readiness

Classification: `EXTERNAL_REVIEW_READINESS_UNCHANGED`.

NA-0470 improves internal scoping evidence but does not create an external
review package or close review prerequisites.

### Release-Claim Boundary

No public-readiness claim. No production-readiness claim. No public-internet-readiness claim. No external-review-complete claim. No crypto-complete claim. No KEM-complete claim. No signature-complete claim. No identity-complete claim. No RNG-failure-complete claim. No provider-RNG-complete claim. No secret-material-complete claim. No side-channel-free claim. No vulnerability-free claim. No bug-free claim. No perfect-crypto claim. Cargo audit green is dependency-health evidence only.

### Assurance Gap Review Trigger

Classification: `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.

D-0927's default Assurance Gap Review remains important. NA-0470 proves a
higher-priority direct residual first: the TUI bootstrap transactionality design
question must be settled before a truthful TUI identity provider RNG
implementation lane can exist. Assurance Gap Review should follow after that
direct residual chain closes unless a later directive proves another
higher-priority residual.

Marker:

- `NA0470_STEWARD_REVIEW_TEMPLATE_USED_OK`

## TUI account bootstrap target inventory

| Candidate | Exact source path(s) | Provider operation | Initial state | State/write timing | Existing marker/error | Existing coverage | APIs enough? | Future cfg-only seam enough? | Injection needed? | refimpl needed? | Future test path if selected | Truthful invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TUI account setup/bootstrap command entry point | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | dispatches `/init` and init wizard | no vault for `/init`; passphrase and alias accepted | calls `tui_try_vault_init`, then `initialize_account_after_init`, then emits `tui_init ok=true` | `tui_init_reject`, `tui_init` | `tui_locked_cmd_init_ux`, `tui_system_account_destroy`, residual tests | enough for read-only timing | not for no-partial account invariant | no fake provider; design needed | no | future design lane only | no success output unless full bootstrap succeeds | high |
| identity KEM keypair generation during bootstrap | `locked.rs` `init_identity_with_passphrase`; `identity/mod.rs`; handshake keypair function read-only | KEM keypair via `hs_kem_keypair()` | after vault/default writes | before KEM secret write and public record | none specific today | no direct TUI identity RNG failure test | enough to locate call | enough only for identity-only failure | no fake provider if wrapper is later authorized | no | not selected | no KEM secret/public-record write on forced KEM failure | high |
| identity signature keypair generation during bootstrap | `locked.rs` `init_identity_with_passphrase`; `identity/mod.rs` | signature keypair via `hs_sig_keypair()` | after KEM generation and after vault/default writes | before signature secret write and public record | none specific today | no direct TUI identity RNG failure test | enough to locate call | enough only for identity-only failure | no fake provider if wrapper is later authorized | no | not selected | no signature secret/public-record write on forced signature failure | high |
| default account/vault writes | `locked.rs` `initialize_account_after_init`; `init_account_defaults_with_passphrase`; `vault/mod.rs` | vault nonce/salt/default token; account secret writes | no vault then passphrase-created vault | occurs before identity key generation | `vault_init`, `settings_init_failed` | residual route-token test proves partial retained default token | APIs enough for proof | no, because writes already happen before identity | design rollback or reordering needed | no | design lane | no partial account/bootstrap state requires design | critical |
| selected identity write/update | `locked.rs`; `identity/mod.rs` identity naming | selected `self` identity convention | no prior self public record expected | identity writes after key generation | none specific | lazy/CLI/legacy background only | enough | enough only after design settles | no | no | not selected | selected identity unchanged or absent | high |
| self public-record write/update | `locked.rs`; `identity/mod.rs` `identity_write_public_record` | writes self public record | identity directory after account setup | after KEM and signature secrets | none specific | lazy/CLI/legacy background only | enough | yes for identity-only surface | no | no | not selected | no partial self public record | high |
| default route/account config writes | `locked.rs`; `vault/mod.rs` | route token, endpoint/token blanks, relay inbox token | no vault | before identity generation | `settings_init_failed`; retained default token in residual test | `rng_failure_residual_surfaces` | enough to prove timing | no | design needed | no | design lane | config unchanged/absent requires design | critical |
| verification seed/account probe surfaces | `locked.rs`; state cache in `state/ownership.rs` | `OsRng.fill_bytes` seed | no account cache | before identity generation | account cache refresh | `NA0452_TUI_ACCOUNT_VERIFICATION_SEED_DEFERRED_OK` | enough to locate | no identity seam does not cover this RNG | separate seam or split needed | no | design lane | seed generation should not create partial setup | high |
| user-visible setup output/diagnostics | `locked.rs` | markers and TUI errors | init command/wizard | failure path emits reject; success emits `tui_init ok=true` | `tui_init_reject`, `tui_init` | TUI init UX tests | enough | yes after design | no | no | design lane | no misleading success output | high |
| CLI rotation completed background | `main.rs`; `identity/mod.rs`; CLI rotation test | KEM/signature keypairs | existing unlocked identity | complete bounded implementation | NA0469 markers | cfg/no-cfg green | done | done | no | no | none | bounded background only | closed |
| lazy identity closed background | `identity/mod.rs`; lazy identity test | KEM/signature keypairs | no self identity on demand | bounded implementation already closed | NA0465 markers | cfg/no-cfg green | done | done | no | no | none | bounded background only | closed |
| legacy/public-record closed background | `identity/mod.rs`; legacy/public-record test | signature keypair | existing legacy/public record | bounded implementation already closed | NA0467 markers | cfg/no-cfg green | done | done | no | no | none | bounded background only | closed |
| X25519 / ephemeral excluded background | handshake source read-only | ephemeral X25519 | handshake paths | outside bootstrap | none for this lane | provider-error background only | no | no | future exact scope | not in this lane | none | excluded residual | backlog |
| refimpl provider RNG excluded background | `tools/refimpl/` read-only | refimpl provider RNG | refimpl paths | outside qsc TUI bootstrap | pqkem768 | refimpl test green | no | no | future exact scope | yes if selected later | none | excluded residual | deferred |

Marker:

- `NA0470_TUI_BOOTSTRAP_TARGET_INVENTORY_OK`

## TUI bootstrap state timing review

Initial state:

- TUI `/init` is accepted only when `state.has_vault()` is false.
- The operator provides alias, passphrase, confirmation, and an explicit
  destructive-action decision.
- Both command and wizard flows call the same durable initialization sequence.

Observed timing:

1. `tui_try_vault_init(passphrase)` creates the encrypted vault file.
2. `initialize_account_after_init(alias, passphrase)` writes `profile_alias`.
3. `init_account_defaults_with_passphrase(passphrase)` writes autolock, polling,
   receipt, file confirmation, verification seed, relay endpoint/token blanks,
   and TUI relay inbox token state.
4. `init_identity_with_passphrase(passphrase)` ensures the identity directory,
   generates a KEM keypair, generates a signature keypair, writes KEM and
   signature identity secrets, then writes the self public record.
5. Only after all of that does TUI emit `event=tui_init ok=true`.

Answers:

- Bootstrap generates both KEM and signature keypairs.
- Identity generation occurs after account/default/vault/bootstrap writes.
- Account/default/vault mutation occurs before provider identity generation.
- Verification seed generation occurs before identity generation.
- Forced identity provider RNG failure can be applied before identity secret and
  public-record writes, but not before earlier account/default writes.
- Future tests can prove selected identity is absent or unchanged, no new
  identity secret, no partial self public record, and no misleading success
  output for identity failure.
- Future tests cannot prove default route/account config unchanged or absent
  under current ordering.
- This path is not implementation-ready for the directive's no-partial
  account/bootstrap invariant.

Classification:

`TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`

Marker:

- `NA0470_TUI_BOOTSTRAP_STATE_TIMING_REVIEW_OK`

## TUI bootstrap invariant options review

| Option | Select/reject | Evidence | Future exact paths | Future validation markers | Public-claim caveat |
|---|---|---|---|---|---|
| 1 - no partial account/bootstrap state | select as required design target | current ordering violates it for simple identity seam | design evidence/testplan; read-only `locked.rs`, `vault/mod.rs`, `identity/mod.rs` | `NA0471_TUI_BOOTSTRAP_TRANSACTIONALITY_DESIGN_SELECTED_OK` | bounded internal evidence only |
| 2 - no selected identity write/update | select as implementation sub-invariant after design | identity writes happen after provider key generation | later exact implementation paths only if design authorizes | `NA0471_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK` | not identity-complete |
| 3 - no new/partial identity secret write | select as implementation sub-invariant after design | secret writes follow KEM/signature generation | later exact implementation paths only if design authorizes | `NA0471_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK` | not secret-material-complete |
| 4 - no partial public-record write/update | select as implementation sub-invariant after design | public record follows secret writes | later exact implementation paths only if design authorizes | `NA0471_TUI_BOOTSTRAP_NO_PARTIAL_PUBLIC_RECORD_OK` | not provider-RNG-complete |
| 5 - default route/account config unchanged/absent | select as design blocker | config writes precede identity generation | design lane | `NA0471_TUI_BOOTSTRAP_DEFAULT_CONFIG_TIMING_REVIEW_OK` | not production readiness |
| 6 - no misleading success output/diagnostics | select | success marker is after full sequence; failure path must stay sanitized | design lane, later implementation if authorized | `NA0471_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_PLAN_OK` | no public claim |
| 7 - verification seed/probe split | select | verification seed precedes identity generation and is separate RNG | design lane | `NA0471_TUI_BOOTSTRAP_VERIFICATION_SEED_SPLIT_REVIEW_OK` | not RNG-failure-complete |
| 8 - design-change/rollback semantics | select | required to satisfy option 1 | design lane | `NA0471_TUI_BOOTSTRAP_TRANSACTIONALITY_DESIGN_SELECTED_OK` | no runtime change until authorized |
| 9 - documentation-only | reject | evidence is exact enough to select design, not just documentation | none | n/a | n/a |

## Implementation readiness review

Questions answered:

- `locked.rs` plus `identity/mod.rs` are enough to locate identity generation,
  but not enough to implement no-partial account/bootstrap behavior without a
  design decision.
- `vault/mod.rs` is needed as read-only evidence for vault creation, default
  route token, secret-write, and atomic-write behavior. It is not authorized
  for mutation by NA-0470.
- One future qsc test file would be enough only after transactionality design
  authorizes the exact invariant and implementation paths.
- Future implementation should not require qshield-cli mutation.
- Future implementation should not require refimpl mutation.
- Future implementation should not require dependency or workflow changes.
- Future implementation would require account-state design or rollback/reorder
  semantics if it must prove no partial account/bootstrap state.
- Normal production semantics cannot be claimed unchanged until the design lane
  selects the exact semantics and an implementation lane tests no-cfg behavior.
- Future test labels and invariants are not clear enough for implementation
  until the transactionality design choice is made.

Classification:

`TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`

Marker:

- `NA0470_TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE_OK`

## TUI bootstrap vs Assurance Gap Review / next-domain decision

| Option | Select/reject | Evidence | Future exact paths if known | Future validation | Public-claim caveat |
|---|---|---|---|---|---|
| TUI bootstrap implementation next | reject | no-partial account invariant cannot hold under current write order | not selected | n/a | no implementation claim |
| TUI bootstrap path-specific split next | reject | timing is already exact; the missing decision is transactionality semantics | not selected | n/a | no claim expansion |
| TUI bootstrap documentation-only next | reject | evidence supports a precise design authorization lane | not selected | n/a | no claim expansion |
| Assurance Gap Review next | reject for immediate successor | D-0927 default is superseded by a higher-priority TUI transactionality residual | future after direct residual chain | assurance review markers later | no external-review-complete claim |
| KEM/signature/transcript audit next | reject | valuable, but does not supersede the direct TUI bootstrap blocker | not selected | n/a | no crypto-complete claim |
| refimpl provider RNG boundary next | reject | qsc TUI blocker does not require refimpl first | not selected | n/a | refimpl residual remains |

## TUI bootstrap scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | Initial state | State/write timing | Future mutation type | Production-semantics risk | Truthful invariant clarity | Evidence value | Scope size | RNG relevance | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| TUI bootstrap implementation | TUI init identity generation | `locked.rs`, `identity/mod.rs`, future qsc test | no vault/account | identity after defaults | runtime/test | high without design | unclear for account state | high | medium | high | yes | no | would alter transactionality without design | no public claim | G1-G5 |
| TUI bootstrap path-specific split | defaults, seed, identity, outputs | governance only, read-only `locked.rs`/`vault.mod.rs`/`identity.mod.rs` | no vault/account | mixed | governance | low | clear enough already | medium | small | high | no | no | design, not discovery, is blocker | no public claim | G1-G5 |
| TUI bootstrap documentation-only | claim boundary | governance docs | no vault/account | n/a | governance | low | too weak | low | small | medium | no | no | direct design residual is actionable | no public claim | G4-G5 |
| Assurance Gap Review | identity/provider RNG chain | governance evidence/testplan | n/a | n/a | governance | low | clear later | high | medium | medium | no | no | deferred by higher-priority direct residual | no external-review-complete claim | G1-G5 |
| KEM/signature/transcript audit | protocol binding | governance evidence/testplan | n/a | n/a | governance | low | clear later | high | medium | indirect | no | no | does not resolve TUI partial bootstrap | no crypto-complete claim | G1-G5 |
| refimpl provider boundary | refimpl RNG | governance evidence/testplan | n/a | n/a | governance/read-only | low | clear later | medium | medium | high | no | no | qsc can decide TUI design first | no provider-RNG-complete claim | G1-G5 |
| identity/provider RNG chain complete enough / no action | none | none | n/a | n/a | none | high overclaim risk | false | low | none | high | n/a | no | TUI design residual remains | would overclaim | G1-G5 |
| TUI bootstrap transactionality design | bootstrap transactionality | future design evidence/testplan; read-only `locked.rs`, `identity/mod.rs`, `vault/mod.rs`, tests | no vault/account | defaults before identity today | governance design authorization | low in design lane | clear | high | small | high | no | yes | required before truthful implementation | no public claim | G1-G5 |

## Authorization decision

Primary classification:

`TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`

Decision:

- D-0927 consumed.
- NA-0469 consumed.
- TUI account bootstrap identity provider RNG scope classified.
- TUI bootstrap is exact enough to identify the blocker, but not exact enough
  to authorize implementation because current ordering writes durable
  account/default/bootstrap state before identity provider key generation.
- selected successor is `NA-0471 -- QSL qsc TUI Account Bootstrap
  Transactionality Design Authorization Plan`.
- assurance gap trigger classification is
  `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.
- no implementation mutation occurs in NA-0470.
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation occurs in NA-0470.
- no backup or restore occurs.
- no public claim expands.
- exactly one READY successor remains mandatory.

## Successor selection

Selected NA-0471:

`NA-0471 -- QSL qsc TUI Account Bootstrap Transactionality Design Authorization Plan`

Reason:

TUI bootstrap implementation would need to prove no partial account/bootstrap
state. Current source order cannot support that proof with a cfg-only identity
provider RNG seam. The next lane must authorize the transactionality decision
before implementation.

Marker:

- `NA0470_SUCCESSOR_NA0471_TRANSACTIONALITY_DESIGN_SELECTED_OK`

## Future path/scope bundle

Future NA-0471 transactionality design authorization scope:

- `docs/governance/evidence/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_plan.md`
- `tests/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0471 read-only source/evidence inputs:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_plan.md`

Future NA-0471 must not implement runtime changes. It must select one of:

- pre-generate identity material before durable account/default writes;
- rollback durable account/default/bootstrap state after identity provider
  failure;
- narrow the future implementation invariant to identity-only evidence and
  explicitly reject no-partial-account/bootstrap proof;
- defer TUI bootstrap implementation and select Assurance Gap Review.

Future forbidden unless later exact scope authorizes:

- runtime/crypto implementation changes outside exact paths;
- dependency changes;
- Cargo/lockfile changes;
- workflow changes;
- executable test changes outside exact paths;
- fuzz target source changes;
- vector changes;
- formal model changes;
- refimpl changes;
- qshield-cli changes;
- public docs/website;
- qsl-server/qsl-attachments changes;
- backup/restore/qsl-backup changes;
- public claims.

## Future validation/marker plan

Common NA-0471 markers:

- `NA0471_TUI_BOOTSTRAP_SCOPE_CONSUMED_OK`
- `NA0471_NEXT_SCOPE_SELECTED_OK`
- `NA0471_NO_DEPENDENCY_CHANGE_OK`
- `NA0471_NO_WORKFLOW_CHANGE_OK`
- `NA0471_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0471_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0471_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0471_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0471_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0471_ONE_READY_INVARIANT_OK`

Design-successor markers:

- `NA0471_TUI_BOOTSTRAP_TRANSACTIONALITY_DESIGN_SELECTED_OK`
- `NA0471_TUI_BOOTSTRAP_WRITE_ORDER_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_DEFAULT_CONFIG_TIMING_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_VERIFICATION_SEED_SPLIT_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_OUTPUT_DIAGNOSTIC_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_IMPLEMENTATION_SCOPE_SELECTED_OR_REJECTED_OK`
- `NA0471_ASSURANCE_REVIEW_TRIGGER_REEVALUATED_OK`

If a later implementation is authorized after design, expected markers include:

- `NA0471_OR_LATER_TUI_BOOTSTRAP_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0471_OR_LATER_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK`
- `NA0471_OR_LATER_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0471_OR_LATER_TUI_BOOTSTRAP_DEFAULT_CONFIG_STABLE_OK`
- `NA0471_OR_LATER_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK`
- `NA0471_OR_LATER_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## Public claim/external review/website boundary

qsc TUI account bootstrap scope authorization is internal governance evidence
only. qsc TUI account bootstrap scope authorization is not production readiness.
qsc TUI account bootstrap scope authorization is not public-internet readiness.
qsc TUI account bootstrap scope authorization is not crypto-complete proof.
qsc TUI account bootstrap scope authorization is not side-channel-free proof.
qsc TUI account bootstrap scope authorization is not RNG-failure-complete proof.
qsc TUI account bootstrap scope authorization is not provider-RNG-complete
proof. qsc TUI account bootstrap scope authorization is not signature-complete
proof. qsc TUI account bootstrap scope authorization is not identity-complete
proof. qsc TUI account bootstrap scope authorization is not bug-free proof.
qsc TUI account bootstrap scope authorization is not vulnerability-free proof.
qsc TUI account bootstrap scope authorization is not perfect-crypto proof. qsc
TUI account bootstrap scope authorization is not public technical paper.

No README, START_HERE, public docs, or website update is made.
No public-readiness claim is made.
No public-security claim is made.
Cargo audit green is dependency-health evidence only.
Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

- `TUI_BOOTSTRAP_IMPLEMENTATION_READY` rejected because current write order
  prevents a truthful no-partial account/bootstrap invariant.
- `TUI_BOOTSTRAP_REQUIRES_PATH_SPECIFIC_SPLIT` rejected because the paths and
  timing are already specific enough; transactionality semantics are the
  blocker.
- `TUI_BOOTSTRAP_DOCUMENTATION_ONLY` rejected because a concrete design
  authorization successor is more useful than documenting without action.
- `TUI_BOOTSTRAP_REQUIRES_REFIMPL_FIRST` rejected because refimpl is not needed
  to decide qsc TUI bootstrap transactionality.
- `ASSURANCE_GAP_REVIEW_REQUIRED_NOW` rejected for immediate successor because
  the transactionality design residual is higher priority.
- `NEXT_AUDIT_DOMAIN_KEM_SIGNATURE_TRANSCRIPT` rejected because it does not
  resolve the direct TUI bootstrap partial-state blocker.
- `REFIMPL_PROVIDER_RNG_BOUNDARY_NEXT` rejected because qsc can settle TUI
  transactionality first.
- `TUI_BOOTSTRAP_AMBIGUOUS` rejected because the source timing is clear.

## Assurance gap review trigger

Current classification:

`HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`

Assurance Gap Review remains required after the current direct residual chain
unless a later directive proves another higher-priority residual. The current
direct residual is not implementation; it is transactionality design
authorization for TUI bootstrap.

Marker:

- `NA0470_ASSURANCE_TRIGGER_HIGHER_PRIORITY_RESIDUAL_OK`

## Backup-impact statement

No backup was run. No restore was run. No qsl-backup file, backup status file,
backup plan file, rollback subtree path, timer, fstab, source list, retention
setting, backup script, or `/backup/qsl` path was mutated.

The qsl-backup proof remains boundary evidence only. The allowed NA-0470
governance files live under the qsl-protocol worktree and do not alter backup
source-list policy.

## Next recommendation

Close NA-0470 only after this evidence PR merges and post-merge public-safety
is green. Restore:

`NA-0471 -- QSL qsc TUI Account Bootstrap Transactionality Design Authorization Plan`

Do not implement NA-0471 inside NA-0470.
