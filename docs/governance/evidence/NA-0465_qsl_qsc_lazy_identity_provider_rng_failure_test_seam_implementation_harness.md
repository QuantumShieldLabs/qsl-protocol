Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0465 QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0465 consumes the NA-0464 split-scope decision and implements only the
selected qsc lazy identity provider RNG failure test seam. The implementation is
confined to `qsl/qsl-client/qsc/src/identity/mod.rs` and the integration test
file `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`.

The seam is compiled only under `--cfg qsc_rng_failure_test_seam`. Normal
no-cfg builds do not read the selector. The no-cfg integration test sets
`QSC_RNG_FAILURE_TEST_SEAM=QSC.IDENTITY.LAZY.KEM_KEYPAIR` and proves normal lazy
identity creation plus A1 handshake output still occur.

Implemented lazy identity labels:

- `QSC.IDENTITY.LAZY.KEM_KEYPAIR`
- `QSC.IDENTITY.LAZY.SIG_KEYPAIR`

The lazy KEM path also preserves the inherited generic selector
`QSC.KEM.KEYPAIR` so NA-0458 KEM provider RNG background tests continue to prove
their original boundary.

The new cfg tests prove forced lazy identity KEM and signature key-generation
failure return sanitized `identity_secret_unavailable` / `rng_failure_forced`
behavior before identity secret, public record, selected identity, pending
handshake, session, or relay output writes.

This evidence covers lazy identity only. Legacy/public-record identity upgrade,
CLI identity rotation, TUI account bootstrap identity generation, X25519 /
ephemeral generation, refimpl provider RNG, qshield-cli demo RNG, formal/model
RNG, and fuzz/vector RNG remain residual.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
vulnerability-free claim is made. No perfect-crypto claim is made. Cargo audit
green remains dependency-health evidence only.

Required NA-0465 markers:

- `NA0465_LAZY_IDENTITY_PROVIDER_RNG_SEAM_IMPLEMENTED_OK`
- `NA0465_LAZY_IDENTITY_KEM_RNG_FAILURE_FORCED_OK`
- `NA0465_LAZY_IDENTITY_KEM_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0465_LAZY_IDENTITY_SIG_RNG_FAILURE_FORCED_OK`
- `NA0465_LAZY_IDENTITY_SIG_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0465_LAZY_IDENTITY_NO_IDENTITY_KEM_SECRET_WRITE_OK`
- `NA0465_LAZY_IDENTITY_NO_IDENTITY_SIG_SECRET_WRITE_OK`
- `NA0465_LAZY_IDENTITY_NO_SELF_PUBLIC_RECORD_WRITE_OK`
- `NA0465_LAZY_IDENTITY_NO_SELECTED_IDENTITY_WRITE_OK`
- `NA0465_LAZY_IDENTITY_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK`
- `NA0465_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0465_LEGACY_IDENTITY_UPGRADE_RESIDUAL_DEFERRED_OK`
- `NA0465_CLI_ROTATE_IDENTITY_RESIDUAL_DEFERRED_OK`
- `NA0465_TUI_BOOTSTRAP_IDENTITY_RESIDUAL_DEFERRED_OK`
- `NA0465_X25519_RESIDUAL_DEFERRED_OK`
- `NA0465_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`
- `NA0465_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0465_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0465_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`
- `NA0465_NO_DEPENDENCY_CHANGE_OK`
- `NA0465_NO_WORKFLOW_CHANGE_OK`
- `NA0465_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0465_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0465_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0465_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0465_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0465_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0465_ONE_READY_INVARIANT_OK`

## Live NA-0465 scope

Startup proof:

- qwork proof startup result: OK.
- qwork lane: NA-0465.
- qwork repo/path: qsl-protocol at `/srv/qbuild/work/NA-0465/qsl-protocol`.
- qwork proof HEAD and proof `origin/main`: `6b8774c6b190`.
- proof HEAD and proof `origin/main` matched live refs before fetch.
- fetch did not advance `origin/main`.
- PR #1198 was verified merged at `6b8774c6b190`.
- current branch before implementation was clean and equal to `origin/main`.

Queue and decision proof before mutation:

- READY_COUNT 1.
- READY item: NA-0465.
- NA-0464 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0916.
- D-0915 exists once.
- D-0916 exists once.
- D-0917 was absent before patch.
- Duplicate decision count was zero.

Allowed implementation paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included dependencies, Cargo manifests, lockfiles,
workflows, handshake source, main CLI source, TUI command source, refimpl, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status, backup
plan, rollback, and backup tree paths.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0465/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0465/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0465`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0465/qsl-protocol`
- clean worktree, index, and untracked state
- `head_equals_origin_main=yes`
- `ready_count=1`
- `queue_top_ready=NA-0465`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof root:

`/srv/qbuild/tmp/NA0465_qsc_lazy_identity_provider_rng_impl_20260612T100436Z`

## NA-0464 inheritance

NA-0464 selected `IDENTITY_SPLIT_LAZY_IDENTITY_NEXT` because
`identity_self_kem_keypair` has a clear generation-before-write boundary. The
future invariant selected by NA-0464 was:

- forced provider RNG failure before writes leaves no identity KEM secret;
- no identity signature secret;
- no self public record;
- no selected identity write;
- no dependent handshake state/output for the selected trigger path.

NA-0464 explicitly deferred legacy/public-record upgrade, CLI identity rotation,
TUI account bootstrap identity generation, X25519 / ephemeral generation,
qshield-cli RNG, formal/model RNG, fuzz/vector RNG, and refimpl provider RNG.
Those residuals remain deferred by NA-0465.

## lazy identity-only implementation summary

`identity_self_kem_keypair` now uses cfg-only lazy identity helper functions
before any identity secret or public-record write:

- `identity_lazy_kem_keypair()` gates KEM keypair generation with
  `QSC.IDENTITY.LAZY.KEM_KEYPAIR` and preserves inherited `QSC.KEM.KEYPAIR`
  behavior by delegating to the existing KEM helper.
- `identity_lazy_sig_keypair()` gates signature keypair generation with
  `QSC.IDENTITY.LAZY.SIG_KEYPAIR`.

Both helpers exist only under `qsc_rng_failure_test_seam`. The no-cfg code path
continues to call `hs_kem_keypair()` and `hs_sig_keypair()` directly and does
not read `QSC_RNG_FAILURE_TEST_SEAM`.

## cfg seam labels and normal-build boundary

Cfg-only labels:

- `QSC.IDENTITY.LAZY.KEM_KEYPAIR`
- `QSC.IDENTITY.LAZY.SIG_KEYPAIR`

Compatibility label preserved:

- `QSC.KEM.KEYPAIR`

Normal-build proof:

- Test: `lazy_identity_rng_failure_seam_inactive_without_cfg`.
- Command: `cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture`.
- Result: PASS.
- Marker: `NA0465_PRODUCTION_SEMANTICS_UNCHANGED_OK`.

The normal-build test sets the lazy KEM selector, then proves identity public
record, identity KEM secret, identity signature secret, dependent pending
handshake state, and relay A1 output are produced normally.

## lazy identity KEM forced-failure proof

Cfg test:

- Test: `lazy_identity_kem_rng_failure_writes_no_identity_state`.
- Selector: `QSC.IDENTITY.LAZY.KEM_KEYPAIR`.
- Command: `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture`.
- Result: PASS.

Proof details:

- The selected trigger path is `handshake init --as alice --peer bob`, where
  Alice has no pre-existing identity and Bob is pre-seeded only as a contact.
- The forced lazy KEM failure returns nonzero command status.
- Output contains sanitized `identity_secret_unavailable` and
  `rng_failure_forced`.
- Output does not contain `handshake_send`, `handshake_complete`, or
  `identity_fp=`.
- Test output is scanned for secret-bearing fragments.

Markers:

- `NA0465_LAZY_IDENTITY_KEM_RNG_FAILURE_FORCED_OK`
- `NA0465_LAZY_IDENTITY_KEM_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`

## lazy identity signature forced-failure proof

Cfg test:

- Test: `lazy_identity_sig_rng_failure_writes_no_identity_state`.
- Selector: `QSC.IDENTITY.LAZY.SIG_KEYPAIR`.
- Command: `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture`.
- Result: PASS.

Proof details:

- The signature failure point occurs after in-memory lazy KEM keypair generation
  but before any identity secret, public-record, or handshake-state write.
- The forced lazy signature failure returns nonzero command status.
- Output contains sanitized `identity_secret_unavailable` and
  `rng_failure_forced`.
- Output does not contain `handshake_send`, `handshake_complete`, or
  `identity_fp=`.
- Test output is scanned for secret-bearing fragments.

Markers:

- `NA0465_LAZY_IDENTITY_SIG_RNG_FAILURE_FORCED_OK`
- `NA0465_LAZY_IDENTITY_SIG_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`

## no partial identity state proof

Both cfg forced-failure tests assert:

- Alice's vault bytes are unchanged after forced failure.
- Alice's self public record path remains absent.
- Alice's identity KEM secret remains absent.
- Alice's identity signature secret remains absent.
- Alice's pending handshake secret remains absent.
- Alice's legacy pending handshake path remains absent.
- Alice's session blob for Bob remains absent.
- Bob's relay channel remains empty.

Marker:

- `NA0465_LAZY_IDENTITY_KEM_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0465_LAZY_IDENTITY_SIG_RNG_FAILURE_NO_PARTIAL_IDENTITY_STATE_OK`

## no identity KEM secret write proof

Both cfg forced-failure tests decrypt the mock vault and assert
`identity.kem_sk.alice` is absent after forced failure.

Marker:

- `NA0465_LAZY_IDENTITY_NO_IDENTITY_KEM_SECRET_WRITE_OK`

## no identity signature secret write proof

Both cfg forced-failure tests decrypt the mock vault and assert
`identity.sig_sk.alice` is absent after forced failure.

Marker:

- `NA0465_LAZY_IDENTITY_NO_IDENTITY_SIG_SECRET_WRITE_OK`

## no self public record write proof

Both cfg forced-failure tests assert
`identities/self_alice.json` remains absent after forced failure.

Marker:

- `NA0465_LAZY_IDENTITY_NO_SELF_PUBLIC_RECORD_WRITE_OK`

## no selected identity write proof

The selected lazy identity trigger path has no separate selected-identity state
write before the identity public record. The tests assert the self public record
is absent, identity secrets are absent, vault bytes are unchanged, and no
`identity_fp=` output is emitted.

Marker:

- `NA0465_LAZY_IDENTITY_NO_SELECTED_IDENTITY_WRITE_OK`

## no dependent handshake state/output proof

Both cfg forced-failure tests assert:

- `handshake.pending.alice.bob` is absent from the mock vault;
- legacy pending path `handshake_pending_alice_bob.json` is absent;
- session path `qsp_sessions/bob.qsv` is absent;
- output contains no `handshake_send` or `handshake_complete`;
- the relay channel for Bob is empty.

Marker:

- `NA0465_LAZY_IDENTITY_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK`

## production semantics unchanged proof

The no-cfg integration test sets `QSC_RNG_FAILURE_TEST_SEAM` to the lazy KEM
label and proves normal lazy identity and handshake-init behavior:

- identity public record exists;
- identity KEM secret exists;
- identity signature secret exists;
- pending handshake state exists;
- relay A1 output exists;
- output contains `event=handshake_send` and `msg=A1`;
- output does not contain `rng_failure_forced` or
  `identity_secret_unavailable`.

Marker:

- `NA0465_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## legacy/public-record upgrade residual deferral

Legacy/public-record identity upgrade remains a separate residual because it
starts from existing public-record state and has a different update/migration
boundary.

Marker:

- `NA0465_LEGACY_IDENTITY_UPGRADE_RESIDUAL_DEFERRED_OK`

## CLI identity rotation residual deferral

CLI identity rotation remains a separate residual because it is an explicit
state transition outside the lazy identity trigger path and is implemented in a
different source path.

Marker:

- `NA0465_CLI_ROTATE_IDENTITY_RESIDUAL_DEFERRED_OK`

## TUI account bootstrap residual deferral

TUI account bootstrap identity generation remains a separate residual because it
is part of TUI account bootstrap state, not the qsc CLI lazy identity path.

Marker:

- `NA0465_TUI_BOOTSTRAP_IDENTITY_RESIDUAL_DEFERRED_OK`

## X25519 residual deferral

X25519 / ephemeral generation remains residual. NA-0465 does not mutate the
handshake source path or implement X25519 provider-failure coverage.

Marker:

- `NA0465_X25519_RESIDUAL_DEFERRED_OK`

## refimpl residual deferral

refimpl provider RNG remains residual. NA-0465 does not mutate refimpl.

Marker:

- `NA0465_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## A2/B1/KEM background preservation

Post-implementation validation reran:

- cfg/no-cfg `a2_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `b1_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `kem_provider_rng_failure`: PASS.

Markers:

- `NA0465_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0465_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0465_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`

These remain bounded background checks only.

## no dependency/workflow/refimpl mutation proof

NA-0465 does not mutate dependencies, Cargo manifests, lockfiles, workflows,
refimpl, fuzz targets, vectors, formal models, qsl-server, qsl-attachments,
qshield runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup status, backup plan, rollback, or backup
tree paths.

Markers:

- `NA0465_NO_DEPENDENCY_CHANGE_OK`
- `NA0465_NO_WORKFLOW_CHANGE_OK`

## public claim boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No signature-complete claim is made. No
identity-complete claim is made. No RNG-failure-complete claim is made. No
provider-RNG-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made. Cargo audit green remains dependency-health evidence only.

## validation

Post-implementation local validation:

- `cargo fmt --check`: PASS.
- cfg `lazy_identity_provider_rng_failure`: PASS.
- no-cfg `lazy_identity_provider_rng_failure`: PASS.
- cfg/no-cfg `a2_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `b1_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `kem_provider_rng_failure`: PASS.
- cfg/no-cfg `rng_failure_residual_surfaces`: PASS.
- cfg/no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- stable `send_commit`: PASS.
- refimpl `pqkem768`: PASS.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- `sh scripts/ci/qsc_adversarial.sh`: Rust adversarial phases PASS; local
  cargo-fuzz command unavailable, exact output recorded under proof root.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- `rustls-webpki` and `ml-kem` inverse trees: PASS.
- pqcrypto inverse probes: expected zero-match inventory.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.

Recovered failure:

- Failing command: read-only qsc adversarial marker scan wrapped by the startup
  helper attempted to write its log under a non-existent nested proof directory.
- Classification: recoverable command-shape/log-path mistake; provider-error
  startup baseline had already passed.
- Corrective action: reran only the marker scan with an existing proof log path.
- Final result: corrected marker scan exited 0 and found the required marker
  reference.
- Failing command: manual staged scope guard in the pre-PR governance script.
- Classification: recoverable local proof-script command-shape issue; a
  single-quoted heredoc passed a literal proof-file variable into Python.
- Corrective action: reran the manual scope guard, exact queue/decision checks,
  and added-line overclaim scan with a literal proof path.
- Final result: corrected staged scope guard and exact queue/decision checks
  exited 0.
- Failing command: added-line overclaim scan over the staged diff.
- Classification: recoverable in-scope governance validation failure; wrapped
  testplan claim-boundary continuation lines carried sensitive claim terms
  without same-line negation.
- Corrective action: reworded the NA-0465 testplan public-claim boundary into
  one negated bullet per sensitive claim term.
- Final result: added-line overclaim rerun exited 0.
- Failing command: direct `scripts/ci/qsc_adversarial.sh` invocation.
- Classification: recoverable invocation-shape issue; the script was not
  executable and the directive specifies using `sh` when not executable.
- Corrective action: reran as `sh scripts/ci/qsc_adversarial.sh`.
- Final result: Rust adversarial phases passed; local cargo-fuzz command was
  unavailable and PR CI qsc-adversarial-smoke remains required evidence.

## scope guard

Expected changed paths for the NA-0465 implementation PR:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation paths remain absent from the diff.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths. qsl-backup SHA/source-list proof was checked read-only during startup.

## successor selection

Selected successor for closeout:

`NA-0466 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Scope Authorization Plan`

Rationale:

- NA-0465 implements lazy identity only.
- NA-0464 explicitly deferred legacy/public-record upgrade after lazy identity.
- CLI identity rotation and TUI account bootstrap remain separate state
  transitions.
- X25519 and refimpl provider RNG remain separate residuals.

## next recommendation

After the NA-0465 implementation PR is merged and post-merge public-safety is
green, close out NA-0465 and restore NA-0466 as the sole READY item without
implementing NA-0466 in the closeout.
