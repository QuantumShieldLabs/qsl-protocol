Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0468 QSL qsc CLI Identity Rotation Provider RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0468 consumes NA-0467 and authorizes the next exact qsc identity provider
RNG failure lane.

Primary classification:

`CLI_ROTATE_IMPLEMENTATION_READY`

Selected successor:

`NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness`

Read-only source review shows the qsc CLI `identity rotate` command in
`qsl/qsl-client/qsc/src/main.rs` generates both a KEM keypair and a signature
keypair before it writes vault identity secrets, writes the self public record,
or optionally resets peer/contact state. This timing is exact enough for a
future cfg-only test seam and one future qsc integration test file to prove a
bounded no-partial-rotation-state invariant.

Future implementation can be bounded to:

- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`

Future implementation must prove forced CLI rotation provider RNG failure
leaves selected identity/public record state stable, writes no new KEM or
signature identity secret, performs no partial self public-record write/update,
does not reset or mutate peer state, and emits no dependent handshake/session
success output. Normal no-cfg production semantics must remain unchanged.

NA-0468 does not implement the seam and does not mutate runtime code, crypto
code, dependencies, Cargo manifests, lockfiles, workflows, executable tests,
fuzz targets, vectors, formal models, refimpl, qsl-server, qsl-attachments,
qshield runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup/restore/local-ops paths, qsl-backup,
backup status files, backup plan files, rollback subtree paths, or backup tree
paths.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No signature-complete claim is made. No
identity-complete claim is made. No RNG-failure-complete claim is made. No
provider-RNG-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made. Cargo audit green is dependency-health evidence only.

Markers recorded by this evidence:

- `NA0468_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0468_NA0467_INHERITANCE_CONSUMED_OK`
- `NA0468_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0468_CLI_ROTATE_TARGET_INVENTORY_OK`
- `NA0468_CLI_ROTATE_PROVIDER_RNG_RELEVANT_OK`
- `NA0468_CLI_ROTATE_STATE_TIMING_REVIEW_OK`
- `NA0468_CLI_ROTATE_IMPLEMENTATION_READY_OK`
- `NA0468_SUCCESSOR_NA0469_SELECTED_OK`
- `NA0468_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0468_NO_DEPENDENCY_CHANGE_OK`
- `NA0468_NO_WORKFLOW_CHANGE_OK`
- `NA0468_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0468_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0468_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0468_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0468_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0468_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0468_ONE_READY_INVARIANT_OK`

## Live NA-0468 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0468.
- NA-0467 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0922.
- D-0921 exists once.
- D-0922 exists once.
- D-0923 was absent before this patch.
- D-0924 was absent before this patch.
- Duplicate decision count was zero.

Allowed NA-0468 mutation paths are exactly:

- `docs/governance/evidence/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_testplan.md`
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
- NA-0467 inheritance is consumed.
- CLI identity rotation provider RNG candidates are inventoried by exact path,
  provider operation, state timing, and future invariant.
- one primary classification and one NA-0469 successor are selected.
- exact future paths are recorded because implementation is selected.
- public claim caveats remain explicit.
- exactly one READY item remains mandatory.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1204
not merged, origin/main not equal to or descending from `a557440e8ab2`,
missing D-0922, D-0923 already present at start, unconsumable NA-0467
inheritance, unsafe CLI rotation classification, unsafe successor selection,
failed root or nested audit, qsl-backup source-list regression, public-safety
red or missing, more than one READY item, or any forbidden mutation/public
overclaim.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0468/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0468/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0468`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0468/qsl-protocol`
- proof HEAD: `a557440e8ab2`
- proof `origin/main`: `a557440e8ab2`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0468`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`a557440e8ab2`. Fetch did not advance `origin/main`. PR #1204 was verified
merged at `a557440e8ab2`.

Proof root:

`/srv/qbuild/tmp/NA0468_qsc_cli_identity_rotation_scope_20260613T012330Z`

Marker:

- `NA0468_QWORK_PROOF_FILE_VERIFIED_OK`

## NA-0467 inheritance

NA-0467 / D-0921 implemented the bounded legacy/public-record identity provider
RNG failure test seam:

- cfg-only legacy migration signature label:
  `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR`;
- cfg-only public-record upgrade signature label:
  `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`;
- forced legacy migration signature failure returns sanitized
  `identity_secret_unavailable` / `rng_failure_forced` before legacy migration
  writes;
- forced public-record upgrade signature failure returns sanitized
  `identity_secret_unavailable` / `rng_failure_forced` before signature-secret
  and self public-record upgrade writes;
- existing identity state remains stable;
- no new signature secret is written;
- no partial self public record is written;
- selected identity fingerprint remains stable;
- no dependent handshake state or relay output is produced;
- normal no-cfg behavior is unchanged.

D-0922 closed NA-0467 and restored NA-0468 as the sole READY item.

Inherited residuals:

- CLI identity rotation remains deferred until this authorization.
- TUI account bootstrap remains deferred.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- qshield-cli demo RNG remains demo-local residual.
- qsc KEM, B1 signing, A2 signing, lazy identity, legacy/public-record,
  route/contact/attachment, base RNG seam, key lifecycle, and provider-error
  evidence remain bounded background evidence only.
- no identity-complete claim exists.
- no signature-complete claim exists.
- no RNG-failure-complete claim exists.
- no provider-RNG-complete claim exists.
- no crypto-complete claim exists.
- no public-readiness claim exists.

Marker:

- `NA0468_NA0467_INHERITANCE_CONSUMED_OK`

## Applicable Stewardship Review

Level 1 stewardship review is active. Level 2 and Level 3 remain future-gated.
There are no separate Directors, no independent READY promotion authority, and
no independent merge authority. Lead Director final authority is preserved.

Crypto / Protocol Steward:

- CLI identity rotation is a separate explicit identity state transition from
  lazy identity creation and legacy/public-record upgrade. Lazy identity starts
  from no self record; legacy/public-record upgrade starts from existing
  identity material; CLI rotation starts from an unlocked profile and attempts
  to replace the active self identity material.
- CLI rotation is provider-RNG relevant for both KEM keypair generation and
  signature keypair generation.
- The future no-partial-rotation invariant must be defined before
  implementation: forced provider RNG failure must leave selected self identity
  state stable, write no new KEM or signature identity secret, perform no
  partial self public-record update, leave peer/contact reset state unchanged,
  and produce no dependent handshake/session success output.
- TUI account bootstrap, X25519 / ephemeral generation, refimpl provider RNG,
  qshield-cli demo RNG, formal/model RNG, and fuzz/vector RNG remain separate
  residuals.
- Production semantics and public-claim caveats must remain unchanged.

CI / Dependency / Release Health Steward:

- Public-safety on current `main` was green before mutation.
- Root `cargo audit --deny warnings` was green before mutation.
- Nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` was green before mutation.
- Legacy/public-record and lazy identity cfg/no-cfg tests were green before
  mutation.
- A2, B1, and KEM provider RNG cfg/no-cfg tests were green before mutation.
- Base cfg RNG failure tests and route/contact/attachment residual RNG tests
  were green before mutation.
- qsc key lifecycle and provider-error no-mutation tests were green before
  mutation.
- refimpl pqkem768, formal model checks, and qsc adversarial checks remain
  required validation for the evidence PR.
- Cargo audit green is dependency-health evidence only. No vulnerability-free
  claim is made.

Public Claims / External Review Steward:

- CLI identity rotation authorization is internal governance evidence only.
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
- Future tests, if implemented, are bounded evidence only and must not be
  described as completing identity, signature, RNG failure, provider RNG, or
  crypto work.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local and outside NA-0468 and the selected NA-0469
  implementation scope.
- qsl-server and qsl-attachments remain service boundaries and are not mutated
  or reclassified by this lane.
- No qshield, website, public-service, production, or public-internet readiness
  claim is made.

Local Ops / Backup / Restore Steward:

- No backup, restore, sudo, local-ops mutation, backup status mutation, backup
  plan mutation, qsl-backup mutation, rollback subtree mutation, timer mutation,
  fstab mutation, or backup tree mutation occurs.
- qsl-backup SHA/source-list proof remains read-only boundary evidence only.
- Same-host continuity, off-host backup, restore, key custody, and durable
  Director State Index residuals are not advanced by NA-0468.

Marker:

- `NA0468_STEWARD_REVIEW_TEMPLATE_USED_OK`

## CLI identity rotation target inventory

| Candidate | Exact source path(s) | Provider operation | Initial state requirement | State/write timing | Existing marker/error | Existing coverage | Existing APIs enough? | Future cfg-only seam enough? | Fake provider/injection needed? | refimpl change needed? | Future test path | Truthful no-partial-rotation invariant | Priority |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CLI rotate command entry point | `qsl/qsl-client/qsc/src/main.rs` | command dispatch calls `identity_rotate(&as_label, confirm, reset_peers)` | qsc profile unlocked; selected self label; normal successful test should use explicit confirm | no writes at dispatch | `identity_rotate`, `identity_rotate_confirm_required`, `vault_locked` | command behavior indirectly covered by existing qsc tests; no provider-RNG rotation test yet | yes | yes | no | no | `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs` | forced provider RNG failure must exit before state writes | P1 |
| identity keypair generation during rotation | `qsl/qsl-client/qsc/src/main.rs` | `hs_kem_keypair()` then `hs_sig_keypair()` | unlocked profile; confirm accepted; pre-existing identity/public record for no-partial comparison | both provider RNG operations occur before vault/public-record/peer-reset writes | none path-specific today | no rotation provider-RNG forced-failure coverage today | yes | yes, with labels such as `QSC.IDENTITY.ROTATE.KEM_KEYPAIR` and `QSC.IDENTITY.ROTATE.SIG_KEYPAIR` | no | no | same future test file | KEM failure writes nothing; signature failure after in-memory KEM generation still writes nothing durable | P1 |
| selected identity write/update | `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | no provider operation; state affected through public record and fingerprint | existing selected self identity | occurs after both keypair operations via public-record write | `identity_rotate_write_failed` on write error | legacy/public-record tests prove selected fingerprint stability for another path | yes | yes | no | no | same future test file | selected identity fingerprint and self record bytes remain unchanged on forced failure | P1 |
| vault identity secret write/update | `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | stores KEM secret then signature secret | unlocked vault required for normal rotation | occurs after both keypair operations | `identity_secret_unavailable` / `vault_missing_or_locked` on store error | lazy and legacy/public-record tests cover other identity secret surfaces | yes | yes | no | no | same future test file | no new or replacement `identity.kem_sk.<label>` or `identity.sig_sk.<label>` value is written on forced failure | P1 |
| self public-record write/update | `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/identity/mod.rs` | no provider operation; writes generated public keys | identity directory available | occurs after both keypair operations and secret stores | `identity_rotate_write_failed` | lazy and legacy/public-record tests cover other self public-record no-partial surfaces | yes | yes | no | no | same future test file | `identities/self_<label>.json` remains byte-for-byte unchanged on forced failure | P1 |
| peer reset / peer state implications | `qsl/qsl-client/qsc/src/main.rs`; contact store helpers | no provider operation; optional `--reset-peers` clears contacts and peer pin files | rotation invoked with reset-peers flag for the peer-reset invariant case | occurs after public-record write | none path-specific beyond final rotate success marker | no rotation provider-RNG peer-reset coverage today | yes | yes | no | no | same future test file | forced provider RNG failure leaves contacts and `peer_*.fp` files unchanged even when reset-peers is requested | P1 |
| dependent handshake/session output implications | `qsl/qsl-client/qsc/src/main.rs`; existing handshake/session stores read-only | no direct provider operation inside rotate after key generation | existing or absent handshake/session baseline recorded by test | no dependent output should happen on provider generation failure | final `identity_rotate` success marker must be absent | legacy/public-record tests prove no dependent handshake output for another path | yes | yes | no | no | same future test file | no `handshake_send`, `handshake_complete`, session blob, or rotate success output after forced failure | P2 |
| lazy identity completed background | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs` | lazy KEM and signature keypair labels | no self identity | already implemented in NA-0465 | NA0465 markers | cfg/no-cfg green | n/a | n/a | no | no | none in NA-0469 | background only; do not reuse lazy labels for rotation | P3 |
| legacy/public-record completed background | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs` | legacy/public-record signature keypair labels | existing legacy/public-record state | already implemented in NA-0467 | NA0467 markers | cfg/no-cfg green | n/a | n/a | no | no | none in NA-0469 | background only; do not reuse legacy/public-record labels for rotation | P3 |
| TUI account bootstrap excluded background | qsc TUI sources read-only | possible future identity generation, not CLI rotate | TUI bootstrap flow | excluded | none selected | deferred | no for NA-0469 | not selected | no | no | none | remains residual | P4 |
| X25519 / ephemeral backlog excluded background | qsc handshake sources read-only | X25519 / ephemeral generation | handshake-specific | excluded | none selected | deferred | no for NA-0469 | not selected | no | no | none | remains backlog | P4 |
| refimpl provider RNG excluded background | `tools/refimpl/` read-only | refimpl provider RNG | refimpl tests | excluded | none selected | deferred | no for NA-0469 | not selected | no | no | none | remains residual | P4 |

Markers:

- `NA0468_CLI_ROTATE_TARGET_INVENTORY_OK`
- `NA0468_CLI_ROTATE_PROVIDER_RNG_RELEVANT_OK`

## CLI rotation state timing review

Initial state required for the selected future implementation lane:

- qsc profile is unlocked;
- the command is invoked as the current self label with explicit confirmation;
- existing self public record and vault identity secrets are present for the
  stable-state assertions;
- peer/contact state exists when the future test exercises `reset_peers=true`;
- any existing pending/session state baseline is recorded before forced
  failure.

State timing facts from read-only source review:

- `identity_rotate` calls `require_unlocked("identity_rotate")` before provider
  key generation.
- `print_error_marker(...)` exits the process, so existing confirm and error
  paths do not continue after printing an error marker.
- with confirm accepted, rotation calls `hs_kem_keypair()` and then
  `hs_sig_keypair()`;
- KEM and signature keypair generation both occur before
  `identity_secret_store`, `identity_sig_secret_store`,
  `identity_write_public_record`, and the optional peer reset block;
- no durable identity, vault, public-record, peer, handshake, or session write
  is visible before provider key generation in the selected confirmed path.

Answers:

- Rotation generates new provider key material before selected
  identity/vault/public-record writes.
- KEM and signature keypair operations are both part of rotation.
- No selected durable state mutation is visible before provider key generation
  in the confirmed, unlocked path.
- Forced provider RNG failure can be applied before writes.
- Future tests can prove selected identity unchanged by snapshotting
  `identity_show` output and self record bytes.
- Future tests can prove no new identity secret/write by snapshotting or
  decrypting the temporary mock vault before and after forced failure.
- Future tests can prove no partial self public record write/update by
  byte-for-byte comparison.
- Future tests can prove peer/reset state unchanged by populating contacts and
  `peer_*.fp` files before invoking rotation with reset-peers and then
  asserting unchanged state after forced failure.
- Future tests can prove no dependent handshake/session/output by asserting
  absence of rotation success, handshake send/complete markers, session blobs,
  and relay output.

State timing classification:

`CLI_ROTATE_PROVIDER_RNG_IMPLEMENTATION_READY`

Marker:

- `NA0468_CLI_ROTATE_STATE_TIMING_REVIEW_OK`

## rotation invariant options review

| Option | Select/reject | Evidence | Future exact paths | Future validation markers | Public-claim caveat |
|---|---|---|---|---|---|
| Option 1 - no partial selected-identity change | Select | self public record and fingerprint are only affected after keypair generation | `main.rs`; `identity/mod.rs`; future test file | `NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK` | bounded CLI rotation evidence only |
| Option 2 - no new/partial identity secret write | Select | KEM and signature secret stores occur after both keypair operations | same | `NA0469_CLI_ROTATE_NO_PARTIAL_SECRET_WRITE_OK` | no identity-complete claim is made |
| Option 3 - no partial public-record write/update | Select | self public-record write occurs after both keypair operations and secret stores | same | `NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK` | no public-readiness claim is made |
| Option 4 - peer reset/state unchanged | Select | optional peer reset block is after public-record write, which is after provider key generation | same | `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK` | peer reset evidence is bounded to forced provider failure before reset |
| Option 5 - no dependent handshake/session/output | Select | rotation should not produce dependent handshake/session success output after keypair failure | same | `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK` | no protocol-complete or crypto-complete claim is made |
| Option 6 - design-change/rollback semantics | Reject | not needed for current timing; would alter runtime transactionality outside NA-0468 | none | none | design change would need a separate lane |
| Option 7 - documentation-only | Reject | source timing is exact and testable without public or runtime scope expansion in NA-0468 | none | none | internal evidence remains sufficient for implementation authorization |

## implementation readiness review

Readiness questions:

- `main.rs` plus `identity/mod.rs` is enough for future CLI rotation
  implementation because the command entry point and keypair/write timing are
  in `main.rs`, while the vault/public-record helpers and existing identity
  seam pattern are in `identity/mod.rs`.
- One future qsc integration test file is enough to prove the selected
  invariant, following the existing NA-0465 and NA-0467 pattern.
- Future implementation does not require TUI command mutation.
- Future implementation does not require qshield-cli mutation.
- Future implementation does not require refimpl mutation.
- Future implementation does not require dependency, Cargo, lockfile, or
  workflow changes.
- Normal production semantics can be proven unchanged by a no-cfg test that
  sets the future seam selector and verifies rotation still succeeds.
- The selected future test can avoid identity-complete overclaim by stating the
  evidence is CLI-rotation-only and not identity-wide.
- Exact labels and invariants are clear enough. Recommended future labels:
  `QSC.IDENTITY.ROTATE.KEM_KEYPAIR` and
  `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`.

Implementation readiness classification:

`CLI_ROTATE_IMPLEMENTATION_READY`

Marker:

- `NA0468_CLI_ROTATE_IMPLEMENTATION_READY_OK`

## CLI rotation vs TUI / next domain decision

| Option | Select/reject | Evidence | Future exact paths if known | Future validation | Public-claim caveat |
|---|---|---|---|---|---|
| CLI rotation implementation next | Select | keypair generation and write timing are exact, bounded, and testable | `main.rs`; `identity/mod.rs`; `cli_identity_rotation_provider_rng_failure.rs`; NA-0469 evidence/testplan; governance paths | cfg/no-cfg CLI rotation provider RNG tests plus inherited checks | bounded internal evidence only |
| CLI rotation path-specific split next | Reject | KEM/signature/peer-reset surfaces are related to one command and can be covered in one test file without widening scope | none | none | no split needed |
| CLI rotation documentation-only next | Reject | implementation path is exact and testable | none | none | not needed |
| TUI account bootstrap scope next | Reject for immediate successor | TUI remains residual but CLI rotation is current READY and exact | none | later scope plan if selected by future directive | no TUI readiness claim |
| KEM / Signature / Transcript Binding Read-Only Audit next | Reject for immediate successor | broader audit may be valuable later but does not outrank exact CLI rotation provider-RNG gap | none | later audit plan if selected | no crypto-complete claim |
| refimpl provider RNG boundary next | Reject for immediate successor | qsc CLI rotation can progress without refimpl mutation | none | later boundary plan if selected | no refimpl coverage claim |

## CLI rotation scope matrix

| Candidate next lane | Surface(s) | Exact candidate paths | Initial state | State/write timing | Future mutation type | Production-semantics risk | Truthful invariant clarity | Evidence value | Scope size | RNG relevance | Needs further triage? | Selected? | Reason | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CLI rotation implementation | CLI rotate KEM/signature generation, vault, public record, peer reset | `main.rs`; `identity/mod.rs`; `cli_identity_rotation_provider_rng_failure.rs` | unlocked, confirmed, existing self identity; optional peer state | keypairs before writes | cfg-only seam plus test | low if seam is cfg-only and no-cfg test proves ignored selector | high | closes current exact qsc identity residual | small | high for KEM and signature provider RNG | no | yes | exact and bounded | no public readiness or completion claim | G1-G5 |
| CLI rotation path-specific split | KEM vs signature vs peer reset | governance-only future split docs | same | same | no implementation | low | medium | lower than direct implementation | small | high | no | no | split not needed because one command timing is clear | no completion claim | G1-G5 |
| CLI rotation documentation-only | claim boundary only | governance docs only | n/a | n/a | docs only | low | medium | low | small | medium | no | no | source path is testable | no implementation claim | G4, G5 |
| TUI account bootstrap identity scope | TUI/bootstrap identity generation | future TUI scope docs; exact source paths not selected here | TUI bootstrap state | not inventoried in NA-0468 | authorization later | unknown | lower than CLI rotation today | useful later | unknown | likely identity RNG relevant | yes | no | CLI rotation is exact current READY | no TUI readiness claim | G1-G5 |
| refimpl provider boundary | refimpl provider RNG | `tools/refimpl/` future boundary docs | refimpl tests | not inventoried in NA-0468 | authorization later | unknown | separate | useful later | medium | high | yes | no | qsc CLI can progress without refimpl | no refimpl completion claim | G1-G5 |
| KEM/signature/transcript audit | broader audit domain | future audit evidence/testplan | read-only repo audit | n/a | docs/audit only | low | broad | useful after identity RNG chain | medium | indirect | yes | no | broader than current exact gap | no crypto-complete claim | G1-G5 |
| identity provider RNG chain complete enough / no action | no new lane | none | n/a | n/a | none | low | false | poor | none | ignores CLI residual | yes | no | CLI rotation remains exact residual | would risk overclaim if selected | G4, G5 |

## authorization decision

Primary classification:

`CLI_ROTATE_IMPLEMENTATION_READY`

Selected successor:

`NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness`

Decision summary:

- NA-0467 is consumed.
- CLI identity rotation provider RNG scope is classified as implementation-ready.
- Rotation uses provider RNG for both signature keypair and KEM keypair
  generation.
- Rotation has a clear future no-partial-rotation invariant around selected
  identity/public record, vault identity secrets, optional peer reset state,
  and dependent handshake/session output.
- Future implementation can be bounded to qsc source plus one future qsc test
  file.
- Future implementation can avoid qshield-cli, qsl-server, qsl-attachments,
  refimpl, workflows, dependencies, Cargo manifests, lockfiles, fuzz targets,
  vectors, and formal model changes.
- No implementation mutation occurs in NA-0468.
- Exactly one READY successor remains mandatory.

## successor selection

Selected exact NA-0469 successor:

`NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness`

Rejected successors:

- `NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Path-Specific Authorization Plan`
- `NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Claim Boundary Documentation Plan`
- `NA-0469 -- QSL qsc CLI Identity Rotation Transactionality Design Authorization Plan`
- `NA-0469 -- QSL qsc TUI Account Bootstrap Identity Provider RNG Failure Scope Authorization Plan`
- `NA-0469 -- QSL KEM / Signature / Transcript Binding Read-Only Audit Plan`
- `NA-0469 -- QSL refimpl Provider RNG Failure Boundary Scope Authorization Plan`
- `NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Ambiguity Resolution Plan`

Marker:

- `NA0468_SUCCESSOR_NA0469_SELECTED_OK`

## future path/scope bundle

Allowed future NA-0469 implementation paths, if restored by closeout:

- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless later exact scope authorizes:

- runtime/crypto implementation changes outside exact paths;
- dependency changes;
- Cargo/lockfile changes;
- workflow changes;
- test source changes outside exact paths;
- fuzz target source changes;
- vector changes;
- formal model changes;
- refimpl changes;
- public docs/website changes;
- qsl-server/qsl-attachments changes;
- qshield-cli changes;
- backup/restore/qsl-backup changes;
- backup status or backup plan changes;
- public claims.

## future validation/marker plan

Common NA-0469 markers:

- `NA0469_CLI_ROTATION_SCOPE_CONSUMED_OK`
- `NA0469_NEXT_SCOPE_SELECTED_OK`
- `NA0469_NO_DEPENDENCY_CHANGE_OK`
- `NA0469_NO_WORKFLOW_CHANGE_OK`
- `NA0469_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0469_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0469_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0469_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0469_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0469_ONE_READY_INVARIANT_OK`

CLI rotation implementation markers:

- `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK`
- `NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK`
- `NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK`
- `NA0469_CLI_ROTATE_NO_PARTIAL_SECRET_WRITE_OK`
- `NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Recommended future cfg-only labels:

- `QSC.IDENTITY.ROTATE.KEM_KEYPAIR`
- `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`

Future validation should include:

- cfg CLI rotation forced KEM failure test;
- cfg CLI rotation forced signature failure test;
- no-cfg production semantics test proving selector ignored;
- scope guard for exact future paths;
- inherited qsc provider RNG tests;
- qsc key lifecycle and provider-error tests;
- qsc send_commit;
- refimpl pqkem768;
- root cargo audit;
- nested qsc fuzz lock audit;
- cargo fmt check;
- formal model checks;
- public-safety and qsc-adversarial-smoke.

## public claim/external review/website boundary

qsc CLI identity rotation scope authorization is internal governance evidence
only.

qsc CLI identity rotation scope authorization is not production readiness.
qsc CLI identity rotation scope authorization is not public-internet readiness.
qsc CLI identity rotation scope authorization is not crypto-complete proof.
qsc CLI identity rotation scope authorization is not side-channel-free proof.
qsc CLI identity rotation scope authorization is not RNG-failure-complete proof.
qsc CLI identity rotation scope authorization is not provider-RNG-complete
proof. qsc CLI identity rotation scope authorization is not signature-complete
proof. qsc CLI identity rotation scope authorization is not identity-complete
proof. qsc CLI identity rotation scope authorization is not bug-free proof. qsc
CLI identity rotation scope authorization is not vulnerability-free proof. qsc
CLI identity rotation scope authorization is not perfect-crypto proof. qsc CLI
identity rotation scope authorization is not public technical paper content.

No README, START_HERE, public docs, docs-public, or website update is performed.
No public-readiness or public-security claim is made. Cargo audit green is
dependency-health evidence only. Future tests, if authorized, must be described
as bounded evidence only.

Markers:

- `NA0468_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0468_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0468_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0468_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0468_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0468_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`

## rejected alternatives

Path-specific split rejected:

- KEM failure, signature failure, and optional peer reset are all part of one
  CLI command whose current source timing is clear.
- A split would add governance delay without reducing implementation risk.

Documentation-only rejected:

- The path is exact and testable with the existing cfg-only seam pattern.

Design-change lane rejected:

- Current source timing is sufficient for forced provider RNG failure before
  durable writes.
- Any transactionality redesign would be separate and is not needed to prove
  the selected bounded invariant.

TUI bootstrap next rejected:

- TUI account bootstrap remains residual, but the current READY item is CLI
  rotation and that path is implementation-ready.

Broader KEM/signature/transcript audit next rejected:

- Broader audit work remains valuable later, but it does not outrank the exact
  CLI identity rotation provider-RNG gap.

refimpl provider RNG next rejected:

- qsc CLI rotation can progress without refimpl mutation.

No action rejected:

- CLI identity rotation remains an exact identity provider-RNG residual.

## backup-impact statement

Backup impact: none.

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths.

Read-only qsl-backup boundary proof:

- `/usr/local/sbin/qsl-backup` SHA matched the expected boundary value.
- the `/home/victor/work/qsl/codex/ops` source-list inclusion count was exactly
  1.

This evidence does not prove off-host backup completion.
This evidence does not prove disaster recovery completion.
This evidence is not restore proof.
This evidence does not prove backup completion.
This evidence does not prove key custody completion.
This evidence does not prove durable Director State Index readiness.

## next recommendation

After this evidence PR merges and post-merge public-safety is green, close out
NA-0468 and restore:

`NA-0469 -- QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness`

NA-0469 should implement only the exact future path/scope bundle above and
should preserve no public overclaim, no dependency/workflow/Cargo/lockfile
mutation, no refimpl/qshield-cli/service mutation, and exactly one READY item.

Markers:

- `NA0468_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0468_NO_DEPENDENCY_CHANGE_OK`
- `NA0468_NO_WORKFLOW_CHANGE_OK`
- `NA0468_ONE_READY_INVARIANT_OK`
