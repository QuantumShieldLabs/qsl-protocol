Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0469 QSL qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0469 consumes NA-0468 and implements the bounded qsc CLI identity rotation
provider RNG failure test seam selected by D-0923 and restored by D-0924.

Implemented cfg-only labels:

- `QSC.IDENTITY.ROTATE.KEM_KEYPAIR`
- `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`

Changed implementation paths:

- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`

Added test file:

- `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`

The seam is compiled only when `--cfg qsc_rng_failure_test_seam` is active.
Normal no-cfg builds do not read or honor `QSC_RNG_FAILURE_TEST_SEAM`.

The cfg tests force KEM and signature keypair failures before CLI rotation
writes and prove deterministic sanitized `identity_secret_unavailable` /
`rng_failure_forced` output, stable selected identity state, no new or partial
KEM or signature identity secret write, no partial self public-record
write/update, unchanged contact and legacy peer-reset state, no dependent
handshake/session output, and unchanged production semantics without cfg.

This is CLI-rotation-only internal qsc evidence. It is not identity-complete proof.
It is not signature-complete proof. It is not RNG-failure-complete proof.
It is not provider-RNG-complete proof. It is not crypto-complete proof.

## Live NA-0469 Scope

Startup proof showed:

- READY_COUNT 1.
- READY item: NA-0469.
- NA-0468 through NA-0435 DONE, except NA-0434 and NA-0429 BLOCKED.
- D-0923 exists once.
- D-0924 exists once.
- D-0925 was absent before this patch.
- Duplicate decision count was zero.

Allowed implementation scope was exactly `main.rs`, `identity/mod.rs`, and the
new CLI rotation integration test file named above. Allowed governance scope was
this evidence doc, the NA-0469 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
the rolling journal.

Forbidden mutation scope included dependencies, Cargo manifests, lockfiles,
workflows, handshake source, TUI commands, qshield-cli, refimpl, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
fuzz targets, vectors, formal models, qwork/qstart/qresume/qshell,
backup/restore/local-ops files, qsl-backup, backup status, backup plan,
rollback subtree paths, and `/backup/qsl` mutation.

## qwork Proof-File Verification

Codex read but did not run `qwork`, `qstart`, or `qresume`.

Verified qwork proof files:

- `/srv/qbuild/work/NA-0469/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0469/.qwork/startup.qsl-protocol.json`

Required proof values were present and mirrored:

- `startup_result=OK`
- `lane=NA-0469`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0469/qsl-protocol`
- proof HEAD and proof `origin/main`: `b96e220baec0`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0469`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live refs before fetch. Fetch did
not advance `origin/main`. PR #1206 was verified merged at `b96e220baec0`.

Proof root:

`/srv/qbuild/tmp/NA0469_qsc_cli_identity_rotation_provider_rng_impl_20260613T023954Z`

## NA-0468 Inheritance

NA-0468 selected `CLI_ROTATE_IMPLEMENTATION_READY`. It found that CLI
`identity rotate` is provider-RNG relevant for both KEM and signature keypair
generation and that both happen before vault identity secret writes, self public
record writes, and optional peer reset/contact mutation.

NA-0468 required future NA-0469 evidence to prove:

- selected identity/public-record state stability;
- no new or partial KEM or signature identity secret write;
- no partial self public-record write/update;
- optional peer-reset/contact state unchanged;
- no dependent handshake/session success output;
- normal no-cfg production semantics unchanged.

NA-0468 also preserved lazy identity and legacy/public-record evidence as
completed bounded background, not as CLI rotation proof.

## CLI Rotation Implementation Summary

`identity/mod.rs` now defines cfg-only rotation failure labels and wrappers:

- `identity_rotate_kem_keypair()`
- `identity_rotate_sig_keypair()`

When `--cfg qsc_rng_failure_test_seam` is active, these wrappers inspect
`QSC_RNG_FAILURE_TEST_SEAM` for the exact rotation label or `all`. On forced
failure they return `rng_failure_forced`. Without the cfg, the wrappers simply
call the normal provider keypair functions and do not read the environment.

`main.rs` now calls the wrappers at the start of confirmed CLI
`identity rotate`. On forced failure, it emits sanitized
`identity_secret_unavailable` evidence with `reason=rng_failure_forced` and
exits before any vault secret write, public-record write, or peer-reset path.

## cfg Seam Labels and Normal-Build Boundary

Implemented labels:

- KEM: `QSC.IDENTITY.ROTATE.KEM_KEYPAIR`
- Signature: `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`

The no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM` to the KEM label and proves a
normal CLI rotation succeeds, writes the expected identity public record and
vault identity secrets, emits `identity_fp=...`, and does not emit
`rng_failure_forced` or `identity_secret_unavailable`.

Marker:

- `NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## CLI Rotate KEM Forced-Failure Proof

Test:

- `cli_identity_rotate_kem_rng_failure_writes_no_partial_rotation_state`

The test seeds an existing Alice identity, a Bob contact, and a legacy peer pin
file, snapshots the public record, selected identity fingerprint, vault bytes,
identity secrets, contacts output, and peer pin bytes, then runs CLI rotation
twice with `--reset-peers` and `QSC.IDENTITY.ROTATE.KEM_KEYPAIR` forced.

Markers:

- `NA0469_CLI_ROTATE_KEM_RNG_FAILURE_FORCED_OK`
- `NA0469_CLI_ROTATE_KEM_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK`

## CLI Rotate Signature Forced-Failure Proof

Test:

- `cli_identity_rotate_sig_rng_failure_writes_no_partial_rotation_state`

The test uses the same state snapshot and double-run deterministic failure
shape, forcing `QSC.IDENTITY.ROTATE.SIG_KEYPAIR`. This proves signature keypair
failure occurs after KEM generation but before any durable rotation write.

Markers:

- `NA0469_CLI_ROTATE_SIG_RNG_FAILURE_FORCED_OK`
- `NA0469_CLI_ROTATE_SIG_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK`

## Selected Identity Stability Proof

Both forced-failure tests prove:

- `identities/self_alice.json` remains byte-for-byte unchanged;
- `identity show --as alice` returns the same selected fingerprint after each
  forced failure;
- forced failure output contains no replacement `identity_fp=...` success line.

Marker:

- `NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK`

## No Partial KEM/Signature Secret Write Proof

Both forced-failure tests decrypt the temporary mock vault and prove:

- `identity.kem_sk.alice` remains exactly the preimage secret;
- `identity.sig_sk.alice` remains exactly the preimage secret;
- `vault.qsv` remains byte-for-byte unchanged after each forced failure.

Markers:

- `NA0469_CLI_ROTATE_NO_PARTIAL_KEM_SECRET_WRITE_OK`
- `NA0469_CLI_ROTATE_NO_PARTIAL_SIG_SECRET_WRITE_OK`

## No Partial Self Public-Record Write/Update Proof

Both forced-failure tests compare `identities/self_alice.json` before and after
forced failure and prove it is byte-for-byte unchanged.

Marker:

- `NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK`

## Peer-Reset/Contact State Unchanged Proof

Both forced-failure tests invoke CLI rotation with `--reset-peers`. They prove
the contact list output remains unchanged and a seeded legacy `peer_bob.fp` file
remains byte-for-byte unchanged. This proves the forced failure returned before
the optional peer reset/contact cleanup path.

Marker:

- `NA0469_CLI_ROTATE_PEER_RESET_STATE_UNCHANGED_OK`

## No Dependent Handshake/Session Output Proof

Both forced-failure tests prove:

- no `event=handshake_send` output;
- no `event=handshake_complete` output;
- no `event=qsp_session_store` output;
- no pending handshake vault secret;
- no legacy pending handshake file;
- no `qsp_sessions/bob.qsv` session blob.

Marker:

- `NA0469_CLI_ROTATE_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK`

## Production Semantics Unchanged Proof

The no-cfg test proves a normal build ignores `QSC_RNG_FAILURE_TEST_SEAM` and
performs a normal CLI identity rotation. The seam does not add a Cargo feature,
dependency, workflow, or runtime fallback.

Marker:

- `NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## Lazy Identity Background Preservation

Post-implementation cfg/no-cfg `lazy_identity_provider_rng_failure` passed.
Lazy identity remains bounded background evidence only.

Marker:

- `NA0469_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK`

## Legacy/Public-Record Background Preservation

Post-implementation cfg/no-cfg
`legacy_identity_public_record_provider_rng_failure` passed.
Legacy/public-record evidence remains bounded background evidence only.

Marker:

- `NA0469_LEGACY_PUBLIC_RECORD_BACKGROUND_PRESERVED_OK`

## TUI Bootstrap Residual Deferral

TUI account bootstrap identity provider RNG failure remains deferred to the
selected successor NA-0470. No TUI command path was mutated.

Marker:

- `NA0469_TUI_BOOTSTRAP_IDENTITY_RESIDUAL_DEFERRED_OK`

## X25519 Residual Deferral

X25519 / ephemeral generation remains a separate residual. No handshake source
was mutated.

Marker:

- `NA0469_X25519_RESIDUAL_DEFERRED_OK`

## refimpl Residual Deferral

refimpl provider RNG remains separate. No refimpl source was mutated.

Marker:

- `NA0469_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## A2/B1/KEM Background Preservation

Post-implementation cfg/no-cfg A2 signature, B1 signature, and KEM provider RNG
tests passed. These remain bounded background checks only.

Markers:

- `NA0469_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0469_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0469_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`

## No Dependency/Workflow/refimpl/qshield-cli Mutation Proof

No dependency, Cargo manifest, lockfile, workflow, refimpl, or qshield-cli path
was changed.

Markers:

- `NA0469_NO_DEPENDENCY_CHANGE_OK`
- `NA0469_NO_WORKFLOW_CHANGE_OK`

## Public Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made. Cargo audit green is
dependency-health evidence only.

Markers:

- `NA0469_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0469_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0469_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0469_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`

## Validation

Local validation after implementation:

- `cargo fmt --check`: PASS after one allowed `cargo fmt` recovery.
- cfg `cli_identity_rotation_provider_rng_failure`: PASS.
- no-cfg `cli_identity_rotation_provider_rng_failure`: PASS.
- cfg/no-cfg `legacy_identity_public_record_provider_rng_failure`: PASS.
- cfg/no-cfg `lazy_identity_provider_rng_failure`: PASS.
- cfg/no-cfg `a2_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `b1_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `kem_provider_rng_failure`: PASS.
- cfg/no-cfg `rng_failure_residual_surfaces`: PASS.
- cfg/no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- stable `send_commit`: PASS.
- refimpl `pqkem768`: PASS.
- `sh -n` and `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock `cargo audit --deny warnings --file`: PASS.
- `cargo tree -i rustls-webpki --locked`: PASS.
- `cargo tree -i ml-kem --locked`: PASS.
- `pqcrypto-*` inverse probes: expected zero-match package-ID absence under
  directive-authorized `|| true`.
- formal model checks: PASS.
- local qsc adversarial script: Rust adversarial properties, Miri-style tests,
  and provider-error step PASS; local `cargo fuzz` command unavailable, so PR CI
  qsc-adversarial-smoke remains the required fuzz-smoke evidence.

## Scope Guard

Expected implementation/evidence changed paths:

- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No handshake source, TUI command source, qshield-cli, refimpl, dependency,
Cargo, lockfile, workflow, fuzz target, vector, formal model, qsl-server,
qsl-attachments, qshield runtime, website, public doc, README, START_HERE,
backup, qwork/qstart/qresume/qshell, or rollback path is intentionally mutated.

## Backup-Impact Statement

Backup impact: none. Codex did not run backup or restore. Codex did not run
sudo. Codex did not mutate qsl-backup, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, backup scripts,
or `/backup/qsl`.

Read-only qsl-backup proof:

- `/usr/local/sbin/qsl-backup` SHA matched the required boundary value.
- latest same-host manifest included the Codex ops source entry exactly once.

## Successor Selection

Selected successor after NA-0469 merge:

`NA-0470 -- QSL qsc TUI Account Bootstrap Identity Provider RNG Failure Scope Authorization Plan`

Rationale:

- NA-0469 implements CLI rotation only.
- NA-0464 and NA-0468 deferred TUI account bootstrap after lazy,
  legacy/public-record, and CLI identity work.
- TUI account bootstrap is a separate user-flow boundary with account-state and
  bootstrap writes.
- X25519 and refimpl provider RNG remain separate residuals.

## Next Recommendation

Merge NA-0469 only after required checks and public-safety pass. If the
post-merge public-safety proof is green, close out NA-0469 and restore the
selected NA-0470 scope authorization lane without implementing NA-0470.
