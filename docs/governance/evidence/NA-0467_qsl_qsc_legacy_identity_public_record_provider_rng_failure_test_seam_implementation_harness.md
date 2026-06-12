Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0467 QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0467 consumes NA-0466 and implements the bounded qsc legacy/public-record
identity provider RNG failure test seam selected by D-0919 and restored by
D-0920.

The implementation is confined to:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`

The seam is compiled only under `--cfg qsc_rng_failure_test_seam`. Normal builds
do not read the seam selector. The no-cfg integration test sets the selector and
proves production semantics are unchanged for the selected public-record upgrade
fixture.

Implemented cfg-only labels:

- `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR`
- `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`

The cfg tests prove forced signature-provider RNG failure returns deterministic
sanitized `identity_secret_unavailable` / `rng_failure_forced` output before
legacy migration or public-record upgrade writes. Existing identity/public-record
state remains stable, no signature secret is written, no partial self public
record is written, selected identity fingerprint remains stable, and no
dependent handshake pending state, session, or relay output is produced.

This is legacy/public-record-only internal evidence. It is not identity-wide,
signature-wide, RNG-wide, provider-RNG-wide, or crypto-wide evidence.

## Live NA-0467 scope

Allowed implementation paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included dependencies, Cargo manifests, lockfiles,
workflows, handshake source, main CLI source, TUI command source, refimpl,
fuzz targets, vectors, formal models, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status, backup
plan, rollback, and backup tree paths.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0467/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0467/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0467`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0467/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0467`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live refs before fetch at
`b986504368ec`. Fetch did not advance `origin/main`. PR #1202 was verified
merged at `b986504368ec`.

## NA-0466 inheritance

NA-0466 / D-0919 classified legacy/public-record identity provider RNG failure
scope as `LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY` and
`LEGACY_PUBLIC_RECORD_PROVIDER_RNG_IMPLEMENTATION_READY`.

Inherited future invariant:

- legacy migration signature keypair failure must be forced before migration
  writes;
- public-record upgrade signature keypair failure must be forced before
  signature-secret and public-record update writes;
- existing identity/public-record state must remain stable;
- no signature secret may be written;
- no partial self public record may be written;
- selected identity state must remain stable;
- no dependent handshake state or relay output may be produced;
- normal no-cfg production semantics must remain unchanged.

NA-0466 also preserved the lazy identity boundary, CLI identity rotation
residual, TUI account bootstrap residual, X25519 residual, refimpl residual,
qshield-cli demo-local residual, formal/model residual, and fuzz/vector
residual.

## legacy/public-record implementation summary

`identity/mod.rs` adds cfg-only helper labels for the two selected signature
keypair generation points:

- `identity_legacy_migrate_sig_keypair()`
- `identity_public_record_upgrade_sig_keypair()`

Both helpers exist only under `qsc_rng_failure_test_seam`. Without that cfg,
the existing direct `hs_sig_keypair()` production path remains the compiled
path and the selector environment variable is not read.

## cfg seam labels and normal-build boundary

Cfg-only labels:

- `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR`
- `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`

Normal build proof:

- `cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture`
- marker: `NA0467_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## legacy migration forced-failure proof

Cfg test:

- `legacy_identity_migrate_sig_rng_failure_writes_no_partial_upgrade_state`

The test builds a temporary legacy identity record from real qsc-generated KEM
identity material, removes the temporary Alice identity secrets from the mock
vault, records byte snapshots, and forces
`QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR` during `handshake init`.

Proof markers:

- `NA0467_LEGACY_MIGRATE_SIG_RNG_FAILURE_FORCED_OK`
- `NA0467_LEGACY_MIGRATE_SIG_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`

## public-record upgrade forced-failure proof

Cfg test:

- `public_record_upgrade_sig_rng_failure_writes_no_partial_upgrade_state`

The test builds a temporary self public record with existing KEM public material
and an empty `sig_pk`, keeps the KEM secret in the mock vault, removes the
signature secret, records byte snapshots, and forces
`QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR` during `handshake init`.

Proof markers:

- `NA0467_PUBLIC_RECORD_UPGRADE_SIG_RNG_FAILURE_FORCED_OK`
- `NA0467_PUBLIC_RECORD_UPGRADE_SIG_RNG_FAILURE_NO_PARTIAL_UPGRADE_STATE_OK`

## existing identity state stability proof

Both cfg tests prove byte-for-byte stability of the selected identity file and
mock vault after forced failure, and prove the `identity_fp` observed before the
failure is the same after the failure.

Marker:

- `NA0467_EXISTING_IDENTITY_STATE_STABLE_OK`

## no new signature secret write proof

Both cfg tests decrypt the temporary mock vault after forced failure and prove
`identity.sig_sk.alice` is absent.

Marker:

- `NA0467_NO_NEW_SIGNATURE_SECRET_WRITE_OK`

## no partial self public-record write/update proof

Both cfg tests record `identities/self_alice.json` before forced failure and
assert byte-for-byte equality after forced failure.

Marker:

- `NA0467_NO_PARTIAL_SELF_PUBLIC_RECORD_WRITE_OK`

## no selected identity change proof

Both cfg tests prove the selected Alice identity fingerprint before forced
failure equals the fingerprint after forced failure.

Marker:

- `NA0467_NO_SELECTED_IDENTITY_CHANGE_OK`

## no dependent handshake state/output proof

Both cfg tests prove no pending handshake vault secret, no legacy pending file,
no session blob, no `handshake_send`, no `handshake_complete`, and no relay A1
output after forced failure.

Marker:

- `NA0467_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK`

## production semantics unchanged proof

The no-cfg test sets the public-record upgrade selector and proves the selector
is ignored by a normal build: the public record is upgraded with a signature
public key, a signature secret is written, pending handshake state is created,
and A1 relay output is emitted.

Marker:

- `NA0467_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## lazy identity background preservation

Inherited cfg/no-cfg `lazy_identity_provider_rng_failure` remains green. Lazy
identity labels remain lazy-only:

- `QSC.IDENTITY.LAZY.KEM_KEYPAIR`
- `QSC.IDENTITY.LAZY.SIG_KEYPAIR`

Marker:

- `NA0467_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK`

## CLI identity rotation residual deferral

CLI identity rotation remains a separate explicit identity state transition and
is not implemented by NA-0467.

Marker:

- `NA0467_CLI_ROTATE_IDENTITY_RESIDUAL_DEFERRED_OK`

## TUI account bootstrap residual deferral

TUI account bootstrap identity generation remains separate and is not
implemented by NA-0467.

Marker:

- `NA0467_TUI_BOOTSTRAP_IDENTITY_RESIDUAL_DEFERRED_OK`

## X25519 residual deferral

X25519 / ephemeral generation remains residual. Handshake source is not mutated
by NA-0467.

Marker:

- `NA0467_X25519_RESIDUAL_DEFERRED_OK`

## refimpl residual deferral

refimpl provider RNG remains residual. refimpl source is not mutated by NA-0467.

Marker:

- `NA0467_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## A2/B1/KEM background preservation

Inherited cfg/no-cfg A2 signature, B1 signature, and KEM provider RNG tests
remain green. These remain bounded background checks only.

Markers:

- `NA0467_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0467_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0467_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`

## no dependency/workflow/refimpl mutation proof

No dependency, Cargo manifest, lockfile, workflow, fuzz target, vector, formal
model, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup,
restore, qsl-backup, backup status, backup plan, rollback, or backup tree path
is intentionally mutated by NA-0467.

Markers:

- `NA0467_NO_DEPENDENCY_CHANGE_OK`
- `NA0467_NO_WORKFLOW_CHANGE_OK`

## public claim boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No signature-complete claim is made. No
identity-complete claim is made. No RNG-failure-complete claim is made. No
provider-RNG-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made. Cargo audit green remains dependency-health evidence only.

Markers:

- `NA0467_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0467_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0467_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0467_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0467_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0467_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`

## validation

Post-implementation validation passed:

- `cargo fmt --check`
- cfg/no-cfg `legacy_identity_public_record_provider_rng_failure`
- cfg/no-cfg `lazy_identity_provider_rng_failure`
- cfg/no-cfg `a2_signature_provider_rng_failure`
- cfg/no-cfg `b1_signature_provider_rng_failure`
- cfg/no-cfg `kem_provider_rng_failure`
- cfg/no-cfg `rng_failure_residual_surfaces`
- cfg/no-cfg `rng_failure_behavior`
- `key_lifecycle_zeroization`
- `handshake_provider_error_no_mutation`
- stable `send_commit`
- refimpl `pqkem768`
- qsc adversarial shell syntax checks
- root `cargo audit --deny warnings`
- nested qsc fuzz lock `cargo audit --deny warnings`
- dependency inverse probes
- formal model checks

Implementation recovery notes:

- Formatting drift in the new Rust test file was recovered by one `cargo fmt`
  run and a clean `cargo fmt --check`.
- The first cfg test compile failed because the test attempted to use an
  undeclared `hex` crate. This was recovered by replacing it with a local test
  hex decoder, preserving the no-dependency invariant.

## scope guard

Changed implementation/test paths are bounded to:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`

Governance changes are bounded to the NA-0467 evidence/testplan, D-0921,
TRACEABILITY, and rolling journal.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths. qsl-backup proof was checked read-only and matched the expected boundary.

## successor selection

Selected successor after NA-0467 implementation PR merge and post-merge
public-safety green:

`NA-0468 -- QSL qsc CLI Identity Rotation Provider RNG Failure Scope Authorization Plan`

Rationale: legacy/public-record identity provider RNG failure is now the
selected implemented qsc identity path. CLI identity rotation is the next
explicit identity state transition with selected identity, vault, public-record,
and optional peer-reset concerns. TUI bootstrap, X25519, refimpl, qshield-cli,
formal/model, and fuzz/vector residuals remain separate.

## next recommendation

Merge the NA-0467 implementation PR only after required checks and
public-safety pass. If post-merge public-safety is green, perform a separate
closeout directive/PR that marks NA-0467 DONE and restores NA-0468 READY without
implementing NA-0468.
