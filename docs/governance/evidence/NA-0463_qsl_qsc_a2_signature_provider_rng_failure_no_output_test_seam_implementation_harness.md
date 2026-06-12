Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0463 QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0463 consumes the NA-0462 A2 scope authorization and implements a bounded
qsc A2 signature provider RNG failure test seam. The seam is compiled only
under `--cfg qsc_rng_failure_test_seam`, uses selector label `QSC.SIG.A2`,
and forces the existing sanitized `sig_sign_failed` reject path after the
known A2 initiator session-store and pending-clear mutations but before A2
`handshake_send` output and before relay A2 emission.

The new integration test proves:

- forced `QSC.SIG.A2` returns sanitized `sig_sign_failed`;
- forced A2 signing failure emits no A2 `handshake_send`;
- forced A2 signing failure emits no relay A2;
- the test explicitly acknowledges A2 post-mutation timing by requiring the
  initiator session write to exist and the effective pending clear to have
  occurred;
- normal no-cfg builds ignore `QSC_RNG_FAILURE_TEST_SEAM` and still emit A2.

This evidence is intentionally no-output-only for A2. It is not a pre-mutation
no-mutation proof. It does not implement identity provider RNG, X25519 /
ephemeral RNG, refimpl provider RNG, qshield-cli RNG, formal/model RNG, or
fuzz/vector RNG residuals.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
No vulnerability-free claim is made. No perfect-crypto claim is made. Cargo
audit green remains dependency-health evidence only.

## Live NA-0463 scope

Startup proof:

- qwork proof startup result: OK.
- qwork lane: NA-0463.
- qwork repo/path: qsl-protocol at `/srv/qbuild/work/NA-0463/qsl-protocol`.
- qwork proof HEAD and proof `origin/main`: `428322bb8094`.
- proof HEAD and proof `origin/main` matched live refs before fetch.
- fetch did not advance `origin/main`.
- PR #1194 was verified merged at `428322bb8094`.
- current branch before implementation was clean and equal to `origin/main`.

Queue and decision proof before mutation:

- READY_COUNT 1.
- READY item: NA-0463.
- NA-0462 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0912.
- D-0911 exists once.
- D-0912 exists once.
- D-0913 was absent before patch.
- Duplicate decision count was zero.

Allowed NA-0463 implementation paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`

Allowed NA-0463 governance paths:

- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included identity source, main CLI source, TUI command
source, refimpl, dependencies, Cargo manifests, lockfiles, workflows, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/
qshell, backup/restore/local-ops paths, qsl-backup, backup status files,
backup plan files, rollback subtree paths, and backup tree paths.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0463/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0463/.qwork/startup.qsl-protocol.json`

Verified proof values:

- `startup_result=OK`
- `lane=NA-0463`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0463/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0463`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, `origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof root:

`/srv/qbuild/tmp/NA0463_qsc_a2_signature_no_output_impl_20260612T001912Z`

## NA-0462 inheritance

NA-0462 / D-0911 selected:

`A2_SIGNATURE_PROVIDER_RNG_NO_OUTPUT_IMPLEMENTATION_READY`

D-0911 classified A2 state timing as:

`A2_STATE_TIMING_POST_MUTATION_PRE_OUTPUT`

NA-0462 explicitly rejected A2 pre-mutation no-mutation because the qsc source
stores the initiator session and clears initiator pending state before A2
signing. NA-0463 preserves that caveat. The inherited truthful invariant is
no A2 output plus sanitized fail-closed `sig_sign_failed`.

NA-0461 B1 signing evidence remains background only. NA-0458 KEM provider RNG
evidence remains background only. Identity provider RNG, X25519 / ephemeral
RNG, qshield-cli RNG, formal/model RNG, fuzz/vector RNG, and refimpl provider
RNG remain residual.

## A2 no-output-only implementation summary

Changed implementation path:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`

Changed test path:

- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`

The implementation wraps only the existing A2 `StdCrypto::sign()` call site.
Under `qsc_rng_failure_test_seam`, selector `QSC.SIG.A2` returns `Err(())`
through the existing sanitized signature-sign failure handling. Without the cfg,
the code calls `c.sign(&sig_sk, &a2_sig_msg).map_err(|_| ())` directly and does
not read `QSC_RNG_FAILURE_TEST_SEAM`.

The forced-failure point is after:

- `qsp_session_store(peer, &st)`;
- `hs_pending_clear(self_label, peer)`;
- suite-admission accept marker emission.

The forced-failure point is before:

- `sig_status reason=a2_sign`;
- `handshake_send msg=A2`;
- `transport::relay_inbox_push(..., &cbytes)`;
- initiator `handshake_complete`.

## cfg seam label and normal-build boundary

cfg selector label:

`QSC.SIG.A2`

The selector helper remains behind:

`#[cfg(qsc_rng_failure_test_seam)]`

Normal no-cfg builds compile the direct provider-sign path and do not compile
the A2 selector branch. The no-cfg integration test sets
`QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.A2` and still observes normal A2 signing,
normal A2 output, and relay A2 emission.

## A2 signing forced-failure proof

Command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Result: PASS.

Markers:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK`

## sig_sign_failed proof

Forced `QSC.SIG.A2` takes the existing sanitized reject path:

- output contains `event=handshake_reject`;
- output contains `sig_sign_failed`;
- output does not expose a provider-internal error;
- output does not contain test route tokens, test passphrase, panic text, stack
  backtrace text, or thread panic text.

Marker:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK`

## no A2 output proof

The forced cfg test asserts the A2 failure command output does not contain:

- `reason=a2_sign`;
- `event=handshake_send`;
- `msg=A2`;
- `event=handshake_complete`.

Marker:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_A2_OUTPUT_OK`

## no relay A2 proof

The forced cfg test drains Bob's relay channel after the forced A2 signing
failure and requires it to be empty. This proves no relay A2 was emitted after
the forced A2 signing failure point.

Marker:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RELAY_A2_OK`

## no false no-mutation claim proof

NA-0463 does not claim A2 pre-mutation no-mutation.

The forced cfg test explicitly preserves the post-mutation caveat:

- before A2 processing, Alice has non-empty initiator pending state and no
  session blob for Bob;
- after forced A2 signing failure, Alice has a session blob for Bob;
- after forced A2 signing failure, Alice pending state is effectively cleared
  by the existing `hs_pending_clear` semantics, where empty-or-absent pending
  loads as no pending.

Markers:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_FALSE_NO_MUTATION_CLAIM_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_POST_MUTATION_TIMING_ACKNOWLEDGED_OK`

## production semantics unchanged proof

Command:

```bash
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Result: PASS.

The normal no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.A2` and proves:

- `sig_status reason=a2_sign` appears;
- `handshake_send msg=A2` appears;
- `handshake_complete` appears;
- no `sig_sign_failed` appears;
- no `rng_failure_forced` appears;
- Bob's relay channel receives A2.

Marker:

- `NA0463_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## B1 background preservation

Commands:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Result: PASS.

Marker:

- `NA0463_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`

## identity residual deferral

NA-0463 does not mutate identity source and does not implement lazy identity,
legacy/public-record upgrade, CLI rotation, or TUI account bootstrap identity
provider RNG failure coverage.

Marker:

- `NA0463_IDENTITY_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## X25519 residual deferral

NA-0463 does not implement X25519 / ephemeral RNG failure coverage.

Marker:

- `NA0463_X25519_RESIDUAL_DEFERRED_OK`

## refimpl residual deferral

NA-0463 does not mutate refimpl and does not implement refimpl provider RNG
failure coverage.

Marker:

- `NA0463_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## KEM background preservation

Commands:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Result: PASS.

Marker:

- `NA0463_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`

## no dependency/workflow/refimpl mutation proof

No Cargo manifest changed. No lockfile changed. No workflow changed. No refimpl
path changed. No qsl-server or qsl-attachments path changed. No qshield runtime
or qshield-cli path changed. No fuzz target, vector, or formal model path
changed.

Markers:

- `NA0463_NO_DEPENDENCY_CHANGE_OK`
- `NA0463_NO_WORKFLOW_CHANGE_OK`

## public claim boundary

NA-0463 is bounded internal qsc forced-seam evidence only.

- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- Cargo audit green is dependency-health evidence only.

Markers:

- `NA0463_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0463_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0463_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0463_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0463_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0463_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`

## validation

Local validation before governance patch:

- `cargo fmt --check`: PASS.
- cfg `a2_signature_provider_rng_failure`: PASS.
- no-cfg `a2_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `b1_signature_provider_rng_failure`: PASS.
- cfg/no-cfg `kem_provider_rng_failure`: PASS.
- cfg/no-cfg `rng_failure_residual_surfaces`: PASS.
- cfg/no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- stable `send_commit`: PASS.
- refimpl `pqkem768`: PASS.
- qsc adversarial script syntax: PASS.
- qsc adversarial local stable phases: PASS; local cargo-fuzz phase unavailable
  because `cargo fuzz` is not installed, so PR CI qsc-adversarial-smoke remains
  the required adversarial smoke proof.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- `rustls-webpki` and `ml-kem` dependency probes: PASS.
- `pqcrypto-*` inverse dependency probes: expected zero-match inventory output.
- `formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `formal/run_model_checks.py`: PASS.

Recovered failures:

- The first broad backup source-count command counted status/plan prose
  references rather than the qsl-backup source list. Classification:
  recoverable command-shape issue. Corrective action: inspect the installed
  qsl-backup source list directly. Final result: script-local source inclusion
  count is 1 and qsl-backup SHA matches the expected value.
- The first cfg A2 test assertion rejected all `sig_status` output, but A2
  processing truthfully emits B1 verification status before forced A2 signing
  failure. Classification: recoverable in-scope validation failure with
  understood test assertion cause. Corrective action: forbid `reason=a2_sign`
  while preserving no A2 output and no relay A2 checks. Final result: PASS.
- The second cfg A2 test assertion expected the pending vault key to be absent,
  but `hs_pending_clear` writes an empty value that `hs_pending_load` treats as
  no pending. Classification: recoverable in-scope validation failure with
  understood test assertion cause. Corrective action: assert non-empty pending
  before A2 and empty-or-absent pending after forced A2 signing failure. Final
  result: PASS.
- The first added-line overclaim scan found wrapped claim-boundary lines whose
  negation was on the previous line. Classification: recoverable wording/scan
  shape issue. Corrective action: rewrapped claim-boundary text so each
  sensitive phrase carries same-line negation. Final result: PASS.

## scope guard

Expected changed paths for the implementation PR:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No dependency, Cargo, lockfile, workflow, refimpl, identity source, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, public docs, README, START_HERE,
fuzz target, vector, formal model, backup, restore, qsl-backup, status/plan, or
rollback path is in scope.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, source lists,
retention, timers, fstab, rollback subtree paths, or backup tree paths.

Read-only boundary proof:

- installed qsl-backup SHA matched the expected value;
- installed qsl-backup source list includes the Codex ops source exactly once.

This is same-host continuity boundary evidence only.

- No off-host backup claim is made.
- No disaster-recovery claim is made.
- No restore-proven claim is made.
- No backup-complete claim is made.

## successor selection

Selected successor:

`NA-0464 -- QSL qsc Identity Provider RNG Failure Split-Scope Authorization Plan`

Rationale:

- NA-0463 implements A2 no-output only.
- B1 and A2 signing are now both covered in bounded ways.
- NA-0460 and NA-0462 left identity provider RNG as the next qsc
  signature/identity residual.
- Identity paths are not uniform and need split-scope authorization before any
  implementation lane.
- X25519 and refimpl provider RNG remain separate residuals.

## next recommendation

After the NA-0463 implementation PR merges and post-merge public-safety is
green, close out NA-0463 and restore NA-0464 as the sole READY item. Do not
implement NA-0464 inside the NA-0463 implementation PR.
