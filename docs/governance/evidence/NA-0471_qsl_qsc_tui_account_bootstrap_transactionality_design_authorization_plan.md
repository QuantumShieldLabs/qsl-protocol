Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0471 QSL qsc TUI Account Bootstrap Transactionality Design Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0471 consumes NA-0470 and D-0927, reviews the qsc TUI account bootstrap
write order, and authorizes the next exact lane.

Primary classification:

`TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTATION_READY`

Selected successor:

`NA-0472 -- QSL qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Harness`

The current TUI bootstrap order is:

1. `/init` or the init wizard calls `tui_try_vault_init(passphrase)`.
2. `initialize_account_after_init(alias, passphrase)` stores `profile_alias`.
3. `init_account_defaults_with_passphrase(passphrase)` writes account/default
   settings, account verification seed, relay endpoint/token fields, and TUI
   relay inbox token state.
4. `init_identity_with_passphrase(passphrase)` generates KEM and signature
   identity keypairs, writes both identity secrets, and writes the self public
   record.
5. The TUI emits setup-success output only after those steps complete.

That order cannot truthfully prove no partial account/bootstrap state when a
future identity provider RNG failure is forced at the current identity
generation point. NA-0471 selects pre-generation as the best-known bounded
successor: a future implementation should generate the TUI bootstrap identity
KEM/signature material before durable vault/account/default writes, then commit
durable account/default/identity state only after identity generation has
succeeded.

Pre-generation is selected over rollback because it avoids deleting already
written user state after a failure. It is selected over staged commit because
qsc has atomic per-file writes but no existing bootstrap-wide transaction
abstraction. It is selected over identity-only proof because identity-only
proof would be truthful but too weak after NA-0470 identified partial
account/default state as the actual blocker.

Markers recorded by this evidence:

- `NA0471_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0471_NA0470_CONSUMED_OK`
- `NA0471_D0927_CONTEXT_CONSUMED_OK`
- `NA0471_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0471_TUI_BOOTSTRAP_WRITE_ORDER_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_PREGENERATION_SELECTED_OK`
- `NA0471_NEXT_SCOPE_SELECTED_OK`
- `NA0471_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0471_NO_DEPENDENCY_CHANGE_OK`
- `NA0471_NO_WORKFLOW_CHANGE_OK`
- `NA0471_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0471_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0471_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0471_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0471_ASSURANCE_TRIGGER_HIGHER_PRIORITY_RESIDUAL_OK`
- `NA0471_ONE_READY_INVARIANT_OK`

NA-0471 mutates governance evidence only. It does not mutate runtime code,
crypto code, qsc source, qshield-cli source, refimpl source, dependencies,
Cargo manifests, lockfiles, workflows, executable tests, fuzz targets, vectors,
formal models, qsl-server, qsl-attachments, qshield runtime, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status files, backup plan files, rollback subtree paths, `/backup/qsl`, or
public technical paper content.

## Live NA-0471 scope

Startup proof showed:

- qwork proof files existed and were read without rerunning qwork.
- proof lane: NA-0471.
- proof repo: qsl-protocol.
- proof path: `/srv/qbuild/work/NA-0471/qsl-protocol`.
- proof HEAD and proof `origin/main`: `cace1ea0b693`.
- clean worktree, index, and untracked state.
- READY_COUNT 1.
- READY item: NA-0471.
- requested lane status: READY.

Allowed NA-0471 mutation paths are exactly:

- `docs/governance/evidence/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_plan.md`
- `tests/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inputs include:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_testplan.md`
- `tests/NA-0470_closeout_restore_na0471_testplan.md`
- D-0927, D-0928, and D-0929 in `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`

Forbidden mutation scope includes runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, refimpl,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback, backup tree paths, public technical paper
content, and public-claim expansion.

Acceptance criteria:

- NA-0470 evidence is consumed.
- qwork proof files are verified without rerunning qwork.
- TUI bootstrap write order and transactionality options are reviewed.
- one primary classification is selected.
- selected NA-0472 successor and future scope are recorded.
- assurance gap trigger classification is present.
- no implementation mutation occurs.
- no public overclaim is introduced.
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale or inconsistent
qwork proof, PR #1211 not merged, `origin/main` not equal to or descending from
`cace1ea0b693`, queue not READY NA-0471 at start, D-0929 absent, D-0930 present
at start, unconsumable D-0927 or NA-0470 evidence, unsafe design
classification, unsafe successor selection, omitted assurance trigger, failed
root or nested audit, qsl-backup source-list regression, public-safety red or
missing, more than one READY item, any forbidden mutation, or any prohibited
public claim.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0471/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0471/.qwork/startup.qsl-protocol.json`

Verified required proof values:

- `startup_result=OK`
- `lane=NA-0471`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0471/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0471`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch. Fetch
did not advance `origin/main`. PR #1211 was verified MERGED with merge commit
`cace1ea0b693`. Current main public-safety was completed success at startup.

Marker:

- `NA0471_QWORK_PROOF_FILE_VERIFIED_OK`

## NA-0470 inheritance

NA-0470 is consumed by NA-0471.

NA-0470 classified the TUI account bootstrap surface as:

`TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`

NA-0470 found:

- `/init` and the init wizard both call `tui_try_vault_init(passphrase)`, then
  `initialize_account_after_init(alias, passphrase)`, then setup-success
  output.
- `initialize_account_after_init` writes profile alias, account defaults, and
  then identity state.
- `init_account_defaults_with_passphrase` writes autolock, poll settings,
  receipt/file settings, verification seed, relay endpoint/token fields, and
  TUI relay inbox token state.
- `init_identity_with_passphrase` generates both KEM and signature identity
  keypairs, writes KEM/signature identity secrets, and writes the self public
  record.
- Direct TUI bootstrap implementation was rejected because durable
  account/default/verification/relay state is written before identity
  generation.

NA-0470 selected NA-0471 as the governance-only transactionality design
authorization lane. NA-0471 does not widen NA-0470 into implementation.

Marker:

- `NA0471_NA0470_CONSUMED_OK`

## Applicable Stewardship and Assurance Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

### Crypto / Protocol Steward

TUI bootstrap starts from no local TUI vault/account and combines vault
creation, alias/default settings, account verification seed, relay defaults,
TUI relay inbox token state, identity material, self public record, and
operator-visible setup output. This is different from completed CLI rotation,
lazy identity, and legacy/public-record provider RNG paths.

The selected design changes failure-order behavior for the TUI bootstrap
identity provider RNG path: identity material generation should occur before
durable account/default writes. The future implementation must preserve
successful bootstrap's durable outputs and must test no-cfg production behavior.
It must also record the new in-memory secret lifetime caveat introduced by
pre-generation.

### CI / Dependency / Release Health Steward

Current main public-safety was green before mutation. Root cargo audit and
nested qsc fuzz lock audit were green before mutation. Inherited qsc cfg/no-cfg
provider RNG tests, qsc zeroization, qsc provider-error no-mutation, refimpl
`pqkem768`, cargo tree dependency inventory, formal checks, and PR CI remain
validation gates for this governance lane and the future implementation lane.

Cargo audit green is dependency-health evidence only. No public-readiness
evidence comes from dependency-health results.

### Public Claims / External Review Steward

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
secret-material-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made.

### Product / Demo / Service Boundary Steward

qshield-cli remains a demo-local residual. qsl-server remains transport-only
outside this lane. qsl-attachments remains opaque ciphertext-only outside this
lane. No qshield runtime, qshield-cli, qsl-server, qsl-attachments, website, or
public-doc mutation is authorized.

### Local Ops / Backup / Restore Steward

No backup, restore, qsl-backup mutation, backup status mutation, backup plan
mutation, rollback subtree mutation, or `/backup/qsl` mutation is authorized.
qsl-backup hash and source-list checks are read-only boundary evidence only.
Same-host continuity is not off-host backup. Same-host continuity is not
disaster recovery. Same-host continuity is not restore proof. Same-host
continuity is not backup-complete evidence.

### Best-Known-Method Review

Classification: `BEST_KNOWN_METHOD_FOR_SCOPE`.

The best-known method for this scope is to force future identity provider RNG
failure before any durable bootstrap/account/default write. Pre-generation
does that with less cleanup risk than rollback and less architectural scope
than a vault-wide staged commit transaction. The future implementation remains
bounded to exact qsc TUI/identity source and one qsc test file.

### Hostile Cryptographer Review

Top concerns:

- pre-generation increases in-memory lifetime for KEM and signature identity
  secret material until durable commit completes;
- a future implementation must not represent TUI bootstrap proof as all
  identity/provider RNG proof;
- identity key generation order is not a formal model proof and does not close
  transcript-binding residuals;
- identity key generation order does not close side-channel residuals;
- identity key generation order does not close provider-RNG-complete residuals;
- identity key generation order does not close identity-complete residuals;
- all failure output must remain sanitized and must not imply setup success.

### Red-Team Review

Top concerns:

- rollback after durable writes can accidentally delete or disturb user state;
- identity-only proof would leave a known partial account/default state debt;
- staging without a real transaction boundary can create false confidence;
- future tests must inspect vault/account/default/identity/public-record state
  and success markers, not only process exit status.

### Production SRE Review

Top concerns:

- a future operator-facing bootstrap failure should leave a clear recovery
  story;
- success output must remain after full durable setup only;
- pre-generation should not introduce new persistent temp artifacts;
- the implementation must preserve successful bootstrap's durable state and
  no-cfg behavior while making identity-provider failure fail before durable
  account/default writes.

### Side-Channel Caveat

No side-channel-free claim. No constant-time proof. No memory-erasure
completeness proof. No all secret-material lifecycle proof. Pre-generation
adds an explicit future in-memory secret lifetime caveat.

### Formal-Model Mapping Residual

Classification: `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.

Existing formal models are supporting evidence only. TUI account bootstrap
transactionality and identity provider RNG failure are not directly modeled by
this lane.

### External-Review Readiness

Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.

NA-0471 improves internal design evidence and claim boundaries. It is not
external-review-complete and does not create a public review package.

### Release-Claim Boundary

No public-readiness claim. No production-readiness claim. No public-internet
readiness claim. No external-review-complete claim. No crypto-complete claim.
No KEM-complete claim. No signature-complete claim. No identity-complete claim.
No RNG-failure-complete claim. No provider-RNG-complete claim. No
secret-material-complete claim. No side-channel-free claim. No
vulnerability-free claim. No bug-free claim. No perfect-crypto claim. Cargo
audit green is dependency-health evidence only.

### Assurance Gap Review Trigger

Classification: `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.

Because NA-0471 selects an exact pre-generation implementation successor,
Assurance Gap Review is deferred until after the NA-0472 implementation lane
unless NA-0472 proves another higher-priority residual. If NA-0472 does not
merge or cannot preserve the selected invariants, the chain should stop rather
than claim assurance completion.

Marker:

- `NA0471_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0471_D0927_CONTEXT_CONSUMED_OK`

## TUI bootstrap write-order inventory

| Surface | Exact source path(s) | Provider/random operation | Initial state requirement | State/write timing | Existing marker/error | Existing test coverage | Existing APIs enough? | Future cfg-only seam enough? | Rollback/staging/pre-generation needed? | Future test path if selected | Truthful invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `/init` command entry | `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` lines 661-764 | dispatch only | locked state and no vault | validates inputs, calls vault init, account init, then success marker | `tui_cmd`, `tui_init_reject`, `tui_init` | `tui_locked_cmd_init_ux`; `tui_system_account_destroy` | yes for locating order | no for no-partial account proof under current order | pre-generation selected | `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs` | no success output unless full bootstrap succeeds | critical |
| init wizard entry | `locked.rs` lines 69-188 | dispatch only | locked flow init prompts | calls the same vault/account init sequence as `/init` | `tui_init_wizard`, `tui_init_reject`, `tui_init` | `tui_locked_cmd_init_ux`; command catalog coverage | yes | no under current order | pre-generation selected | same future test | wizard and slash command share invariant | critical |
| `tui_try_vault_init(passphrase)` | `locked.rs` lines 482-524; `vault/mod.rs` lines 440-582 | vault salt/nonce/default route token during child `vault init` | no existing vault | durable vault file written before account defaults and before identity generation | `vault_init`; `vault_init_failed` | vault RNG failure coverage; default route-token residual coverage | yes for current order | not identity seam covered | pre-generation must occur before this call for identity-provider failures | same future test | forced identity keygen failure leaves no vault/account/default state | critical |
| `initialize_account_after_init(alias, passphrase)` | `locked.rs` lines 614-621 | none directly | vault already created | writes alias, defaults, then identity | `alias_store_failed`, `settings_init_failed`, `identity_init_failed` | init UX/account tests | yes | not under current order | pre-generation should feed identity commit after defaults | same future test | no account/default writes after forced pre-generation failure | critical |
| `init_account_defaults_with_passphrase` | `locked.rs` lines 526-579 | verification seed via `OsRng`; relay inbox token via `generate_route_token*` | vault exists and passphrase opens it | writes account/default state before identity generation | `settings_init_failed` | `rng_failure_residual_surfaces` | yes | no for identity failure under current order | pre-generation selected; all-failure staging not selected | same future test plus later residuals if needed | defaults absent when TUI identity provider failure is forced before writes | critical |
| autolock write | `locked.rs` lines 527-529 | none | vault exists | before identity generation | `settings_init_failed` on failure | account/init coverage | yes | no under current order | pre-generation selected | same future test | no autolock default written on forced pre-generation failure | high |
| poll setting write | `locked.rs` lines 528-539 | none | vault exists | before identity generation | `settings_init_failed` on failure | account/init coverage | yes | no under current order | pre-generation selected | same future test | no poll defaults written on forced pre-generation failure | high |
| receipt/file setting write | `locked.rs` lines 540-559 | none | vault exists | before identity generation | `settings_init_failed` on failure | account/init coverage | yes | no under current order | pre-generation selected | same future test | no receipt/file defaults written on forced pre-generation failure | high |
| verification seed write | `locked.rs` lines 560-566 | `OsRng.fill_bytes` | vault exists | before identity generation | `settings_init_failed` on secret-write failure | NA-0452 residual notes; account init proof | yes for order | identity seam does not cover seed RNG | pre-generation for identity failure; separate seed failure lane only if later selected | same future test for identity failure only | no verification seed written on forced identity pre-generation failure | high |
| relay endpoint/token write | `locked.rs` lines 567-568 | none | vault exists | before identity generation | `settings_init_failed` on failure | relay config/account coverage | yes | no under current order | pre-generation selected | same future test | no relay endpoint/token defaults written on forced pre-generation failure | high |
| TUI relay inbox token state write | `locked.rs` lines 569-577; `vault/mod.rs` lines 462-474 | route token RNG | vault exists | before identity generation | `settings_init_failed`; `rng_failure_forced` in cfg seam | `rng_failure_residual_surfaces` lines 426-455 | yes for order | no for identity failure under current order | pre-generation selected; token-specific failure remains separate | same future test for identity failure only | no relay inbox token write on forced identity pre-generation failure | high |
| `init_identity_with_passphrase` | `locked.rs` lines 581-611 | KEM and signature keypair generation | currently after vault/default writes | generates keys, writes secrets, writes public record | `identity_init_failed` | no direct TUI identity RNG failure test | enough to refactor | yes if future TUI-specific labels are added | pre-generation selected | same future test | identity material generated before durable account/default writes | critical |
| identity KEM keypair generation | `locked.rs` line 596; `identity/mod.rs` lines 85-95 for rotate precedent | `hs_kem_keypair()` | currently after defaults | before KEM secret write | none in TUI today | CLI/lazy/KEM background only | yes | yes with future TUI label | pre-generation selected | same future test | forced KEM failure leaves no durable bootstrap state | critical |
| identity signature keypair generation | `locked.rs` line 597; `identity/mod.rs` lines 98-108 for rotate precedent | `hs_sig_keypair()` | currently after KEM generation and defaults | before signature secret write | none in TUI today | CLI/lazy/legacy/A2/B1 background only | yes | yes with future TUI label | pre-generation selected | same future test | forced signature failure leaves no durable bootstrap state | critical |
| identity KEM secret write | `locked.rs` lines 598-603; `identity/mod.rs` lines 126-159 | vault secret write | after key generation | after defaults under current order | `identity_secret_store`, `identity_secret_unavailable` precedent | CLI/lazy/legacy background only | yes | not enough alone | pre-generation selected | same future test | no KEM secret write on forced pre-generation failure | high |
| identity signature secret write | `locked.rs` lines 604-609; `identity/mod.rs` lines 130-217 | vault secret write | after key generation | after KEM secret write | `identity_secret_store`, `identity_secret_unavailable` precedent | CLI/lazy/legacy background only | yes | not enough alone | pre-generation selected | same future test | no signature secret write on forced pre-generation failure | high |
| self public record write | `locked.rs` line 610; `identity/mod.rs` lines 248-266 | atomic public record write | after secret writes | after both identity secrets | no TUI-specific marker | CLI/lazy/legacy background only | yes | yes after future labels | pre-generation selected | same future test | no self public record on forced pre-generation failure | high |
| selected identity write/update if any | `locked.rs` line 582 fixed `self`; `identity/mod.rs` self-label paths | none separate observed | fixed self identity convention | no separate selected-identity state observed in bootstrap | none | source search found self-label state, not selected write | yes | not applicable | no separate selected-identity rollback selected | same future test | selected identity write is absent; self public record is the durable identity surface | medium |
| setup success output/diagnostics | `locked.rs` lines 183-187 and 758-762 | none | after all bootstrap functions return | success marker after identity commit today | `event=tui_init ok=true` | `tui_locked_cmd_init_ux`; `tui_system_account_destroy` | yes | yes | pre-generation selected | same future test | no misleading success output on forced failure | critical |
| CLI rotation completed background | `qsl/qsl-client/qsc/src/main.rs`; `identity/mod.rs`; `tests/cli_identity_rotation_provider_rng_failure.rs` | `QSC.IDENTITY.ROTATE.*` | existing identity state | complete bounded path | NA-0469 markers | cfg/no-cfg green | done | done | none | none | bounded background only | closed |
| lazy identity completed background | `identity/mod.rs`; `tests/lazy_identity_provider_rng_failure.rs` | `QSC.IDENTITY.LAZY.*` | on-demand self identity | complete bounded path | NA-0465 markers | cfg/no-cfg green | done | done | none | none | bounded background only | closed |
| legacy/public-record completed background | `identity/mod.rs`; `tests/legacy_identity_public_record_provider_rng_failure.rs` | `QSC.IDENTITY.LEGACY_MIGRATE.*`; `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.*` | legacy/public-record state | complete bounded path | NA-0467 markers | cfg/no-cfg green | done | done | none | none | bounded background only | closed |
| X25519 / ephemeral excluded background | qsc handshake source read-only | ephemeral X25519 | handshake paths | outside bootstrap | none for this lane | provider-error and KEM background only | not in scope | no | future exact scope only | none | excluded residual | backlog |
| refimpl provider RNG excluded background | `tools/refimpl/` read-only | refimpl provider RNG | refimpl paths | outside qsc TUI bootstrap | `pqkem768` test | refimpl test green | not in scope | no | future exact scope only | none | excluded residual | deferred |

Marker:

- `NA0471_TUI_BOOTSTRAP_WRITE_ORDER_REVIEW_OK`

## Transactionality design options review

| Option | Select/reject | Evidence | Future exact paths | Future tests/markers | Production-semantics risk | Public-claim caveat |
|---|---|---|---|---|---|---|
| 1 - Pre-generate identity material before durable account/default writes | select | It forces identity provider RNG failure before `tui_try_vault_init`, alias/default writes, verification seed, relay fields, relay inbox token, identity secret writes, and public record writes. It is narrower than staged commit and safer than rollback. | `locked.rs`; `identity/mod.rs`; `tests/tui_account_bootstrap_transactionality.rs`; NA-0472 evidence/testplan; `DECISIONS.md`; `TRACEABILITY.md`; rolling journal | `NA0472_TUI_BOOTSTRAP_PROVIDER_RNG_FAILURE_FORCED_OK`; `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK`; `NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK` | increases in-memory secret lifetime; success outputs should remain semantically unchanged | internal evidence only; no identity-complete or provider-RNG-complete claim |
| 2 - Stage all bootstrap writes, then atomic commit | reject for immediate successor | qsc has per-file atomic writes and temp/rename helpers, but no bootstrap-wide staged transaction support across vault secrets and identity public record. | not selected; would require `vault/mod.rs` only if future proof shows need | not selected | broader architecture and transaction semantics risk | no production-readiness claim |
| 3 - Roll back durable account/default writes after identity provider failure | reject | Rollback after vault/default writes could delete or disturb user state and is harder to prove complete than preventing writes before failure. Existing best-effort wipe removes broad local state and is not a precise bootstrap transaction. | not selected | not selected | accidental deletion and partial cleanup risk | no no-partial-account claim via rollback |
| 4 - Narrow future invariant to identity/public-record-only | reject | Truthful but too weak. It would preserve known partial account/default state debt from NA-0470. | not selected | identity-only caveat markers not selected | low implementation risk but high evidence gap | explicitly not selected |
| 5 - Path-specific split | reject for immediate successor | Defaults, verification seed, relay token, and identity generation are separate surfaces, but the direct identity-provider failure blocker has a bounded pre-generation design. | not selected | not selected | could fragment the direct fix | future non-identity failures may still get split lanes |
| 6 - Documentation-only | reject | Documentation-only would preserve the known behavior but leave an actionable design gap unresolved. | not selected | not selected | low code risk, insufficient safety gain | no implementation evidence |
| 7 - Assurance Gap Review now | reject for immediate successor | D-0927 default review remains important, but NA-0471 has a precise implementation successor. | future after NA-0472 unless superseded | assurance-review markers later | no code risk now, but defers direct residual | no external-review-complete claim |

## Best-known-method design decision

Selected classification:

`TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTATION_READY`

Decision:

- TUI bootstrap should be redesigned so identity KEM/signature material is
  generated before durable account/default writes.
- TUI bootstrap should not use rollback/cleanup as the primary answer to
  identity provider failure.
- TUI bootstrap should not narrow the next implementation to
  identity/public-record-only proof.
- TUI bootstrap should not split verification seed/default config/relay
  token/identity generation before the identity pre-generation implementation
  lane.
- TUI bootstrap should not defer to Assurance Gap Review immediately.
- Future implementation can remain bounded to exact qsc source/test paths:
  `locked.rs`, `identity/mod.rs`, and
  `tests/tui_account_bootstrap_transactionality.rs`.
- Future implementation should not require vault transactionality, new staging
  infrastructure, new temp-file protocols, or state-machine redesign for the
  selected identity-provider failure invariant.
- Future implementation must preserve successful production behavior when the
  test seam is inactive, while caveating the intentional failure-order change
  and longer in-memory secret lifetime introduced by pre-generation.
- Assurance Gap Review should be the default next lane after NA-0472 closes,
  unless NA-0472 proves another higher-priority residual.

Marker:

- `NA0471_TUI_BOOTSTRAP_PREGENERATION_SELECTED_OK`

## Future scope bundle

Selected NA-0472 title:

`QSL qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Harness`

Candidate future paths:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs`
- `docs/governance/evidence/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_harness.md`
- `tests/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0472 must not mutate dependencies, Cargo manifests, lockfiles,
workflows, qsl-server, qsl-attachments, qshield runtime, qshield-cli runtime,
website, public docs, README, START_HERE, refimpl, fuzz targets, vectors,
formal models, qwork/qstart/qresume/qshell, backup, restore, qsl-backup,
backup status files, backup plan files, rollback subtree paths, `/backup/qsl`,
or public technical paper content unless a later exact directive changes scope.

Future NA-0472 implementation objective:

- add TUI-bootstrap-specific identity provider RNG failure labels or equivalent
  cfg-only test seam;
- generate identity KEM/signature material before `tui_try_vault_init` and
  before account/default writes;
- commit vault/account/default/identity state only after pre-generation
  succeeds;
- prove forced KEM and signature pre-generation failures write no vault,
  account defaults, verification seed, relay defaults, TUI relay inbox token,
  identity secrets, or self public record;
- prove no misleading setup success output;
- prove no-cfg production success behavior remains unchanged.

Marker:

- `NA0471_NEXT_SCOPE_SELECTED_OK`

## Future validation/marker plan

Common NA-0472 markers:

- `NA0472_TUI_TRANSACTIONALITY_DESIGN_CONSUMED_OK`
- `NA0472_NEXT_SCOPE_SELECTED_OK`
- `NA0472_NO_DEPENDENCY_CHANGE_OK`
- `NA0472_NO_WORKFLOW_CHANGE_OK`
- `NA0472_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0472_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0472_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0472_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0472_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0472_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0472_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0472_ONE_READY_INVARIANT_OK`

Selected pre-generation implementation markers:

- `NA0472_TUI_BOOTSTRAP_PROVIDER_RNG_FAILURE_FORCED_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_DEFAULT_CONFIG_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK`
- `NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Future validation commands should include cfg and no-cfg
`tui_account_bootstrap_transactionality`, inherited qsc provider RNG tests,
key lifecycle zeroization, provider-error no-mutation, `send_commit`, refimpl
`pqkem768`, root and nested cargo audits, cargo fmt check, formal model checks,
qsc adversarial script syntax, and PR CI public-safety.

## Public claim/external review/website boundary

This is internal governance design authorization only. It creates no website
content and no public technical paper content.

No public-readiness claim. No production-readiness claim. No public-internet
readiness claim. No external-review-complete claim. No crypto-complete claim.
No KEM-complete claim. No signature-complete claim. No identity-complete claim.
No RNG-failure-complete claim. No provider-RNG-complete claim. No
secret-material-complete claim. No side-channel-free claim. No
vulnerability-free claim. No bug-free claim. No perfect-crypto claim.

## Assurance Gap Review trigger

Classification:

`HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`

Reason:

NA-0471 selects one more exact implementation lane, NA-0472. Assurance Gap
Review remains the default after that implementation lane closes unless NA-0472
proves a higher-priority residual.

## Rejected alternatives

- Staged commit is rejected as too broad for the next lane because there is no
  existing bootstrap-wide transaction abstraction.
- Rollback is rejected as riskier than pre-generation because cleanup can be
  incomplete or over-delete.
- Identity-only invariant is rejected as too weak because it would preserve
  known partial account/default state debt.
- Path-specific split is rejected as unnecessary before the selected
  pre-generation implementation.
- Documentation-only is rejected as insufficient because the design gap is
  actionable.
- Assurance Gap Review now is rejected because one more direct residual is
  selected.

## Backup-impact statement

No backup was run. No restore was run. No qsl-backup mutation occurred. No
backup status file mutation occurred. No backup plan mutation occurred. No
rollback subtree mutation occurred. No `/backup/qsl` mutation occurred.

qsl-backup SHA remained the expected read-only boundary value during startup
proof. The script-local source inclusion for `/home/victor/work/qsl/codex/ops`
remained exactly one. This is same-host continuity evidence only. It is not
off-host backup. It is not disaster recovery. It is not restore proof. It is
not backup-complete evidence. It is not public-readiness evidence.

## Next recommendation

Open and merge the NA-0471 evidence PR first. If public-safety is green after
that merge, close out NA-0471 and restore NA-0472 as:

`QSL qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Harness`

NA-0472 should implement only the selected pre-generation scope and should
leave Assurance Gap Review as the default follow-up after the implementation
chain closes.
