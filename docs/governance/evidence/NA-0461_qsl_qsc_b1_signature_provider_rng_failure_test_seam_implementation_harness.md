Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0461 QSL qsc B1 Signature Provider RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0461 consumes the NA-0460 split-scope authorization and implements only the
selected qsc B1 responder signing provider-failure test seam.

The seam is compiled only when Rust is invoked with:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam'
```

Implemented label:

- `QSC.SIG.B1`

The forced path takes the existing sanitized `sig_sign_failed` B1 responder
reject branch before responder pending state is stored, before responder
session state is stored, and before B1 relay output is emitted. Normal no-cfg
builds do not compile the selector check and ignore `QSC_RNG_FAILURE_TEST_SEAM`
for this path.

This lane does not implement A2 signing provider RNG, identity provider RNG,
X25519/ephemeral provider RNG, refimpl provider RNG, qshield-cli RNG, formal
RNG, fuzz RNG, vector RNG, service work, public docs, dependency changes, or
workflow changes. Existing qsc KEM provider RNG seam evidence from NA-0458
remains background preserved evidence only.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
vulnerability-free claim is made. No perfect-crypto claim is made. Cargo audit
green remains dependency-health evidence only.

Required NA-0461 markers:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RESPONDER_MUTATION_OK`
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_B1_OUTPUT_OK`
- `NA0461_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0461_A2_SIGNATURE_RESIDUAL_DEFERRED_OK`
- `NA0461_IDENTITY_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`
- `NA0461_X25519_RESIDUAL_DEFERRED_OK`
- `NA0461_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`
- `NA0461_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`
- `NA0461_NO_DEPENDENCY_CHANGE_OK`
- `NA0461_NO_WORKFLOW_CHANGE_OK`
- `NA0461_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0461_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0461_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0461_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0461_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0461_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0461_ONE_READY_INVARIANT_OK`

## Live NA-0461 scope

Startup proof established READY_COUNT 1 and READY NA-0461. NA-0460 through
NA-0435 are DONE. NA-0434 and NA-0429 are BLOCKED. Latest decision before patch
was D-0908. D-0907 exists once, D-0908 exists once, D-0909 was absent before
this patch, and duplicate decision count was zero.

Allowed implementation paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included dependencies, Cargo files, lockfiles,
workflows, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
public docs, README, START_HERE, fuzz targets, vectors, formal model files,
refimpl, qwork/qstart/qresume/qshell, backup/restore/local-ops files, and
backup tree paths.

Acceptance criteria were the directive criteria: qwork proof-file verification,
NA-0460 inheritance, cfg-only B1 signature seam implementation, forced
`sig_sign_failed`, no responder mutation, no B1 output, production semantics
unchanged without cfg, residual preservation, exact path scope, local validation,
public-safety protection, and exactly one READY item.

Stop conditions were the directive stop conditions: stale qwork proof, PR #1190
not merged, origin/main mismatch, queue not READY NA-0461, missing D-0908,
D-0909 present at start, insufficient implementation paths, inability to prove
`sig_sign_failed`, responder no-mutation, no B1 output, or no-cfg production
semantics, any forbidden mutation, failed required audit/check, backup boundary
regression, or any public overclaim.

## qwork proof-file verification

Codex read, but did not run or regenerate, these proof files:

- `/srv/qbuild/work/NA-0461/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0461/.qwork/startup.qsl-protocol.json`

Verified values:

- `startup_result=OK`
- `lane=NA-0461`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0461/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0461`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`7b4c9b109c7`. Fetch did not advance `origin/main`.

PR #1190 was verified merged at `7b4c9b109c7`.

Proof root:

`/srv/qbuild/tmp/NA0461_qsc_b1_signature_provider_rng_seam_impl_20260611T122247Z`

## NA-0460 inheritance

NA-0460 selected:

`SIGNATURE_IDENTITY_SPLIT_B1_SIGNING_NEXT`

Inherited facts:

- B1 responder signing is the first implementation target.
- B1 responder signing failure occurs before responder pending/session
  insertion and before B1 output.
- A2 signing remains deferred because it occurs after initiator session storage
  and pending clear.
- Identity provider RNG remains deferred across lazy identity creation,
  legacy/public-record upgrade, CLI rotation, and TUI account bootstrap.
- Verification / `sig_invalid` remains background only and not RNG-relevant.
- X25519 / ephemeral generation remains backlog.
- qsc KEM provider RNG seam evidence from NA-0458 is complete and green, but
  remains bounded background evidence only.
- refimpl provider RNG remains deferred.
- No RNG-failure-complete claim exists.
- No provider-RNG-complete claim exists.
- No signature-complete claim exists.
- No identity-complete claim exists.
- No crypto-complete claim exists.
- No public-readiness claim exists.

## B1 signing-only implementation summary

Implementation paths changed:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs`

`handshake/mod.rs` adds one cfg-gated check immediately before the responder B1
`StdCrypto::sign()` call. When the cfg is active and
`QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.B1`, the path returns the existing sanitized
`sig_sign_failed` reject marker and continues without pending/session storage or
B1 relay output.

The new integration test file uses the existing qsc mock vault and mock relay
test style. It proves the forced cfg path and the normal no-cfg boundary.

## cfg seam label and normal-build boundary

The seam label is:

- `QSC.SIG.B1`

The check is under `#[cfg(qsc_rng_failure_test_seam)]`. Normal no-cfg builds do
not compile the environment-variable read at the B1 signing site.

Normal-build proof command:

```bash
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

The no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.B1` while exercising the
B1 signing path. The handshake still emits normal B1 `sig_status` /
`handshake_send` output and emits:

- `NA0461_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## B1 signing forced-failure proof

Cfg proof command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Test:

`b1_signature_rng_failure_writes_no_responder_state_or_b1`

Evidence:

- forced label: `QSC.SIG.B1`;
- qsc emitted `event=handshake_reject`;
- qsc emitted sanitized reason `sig_sign_failed`;
- no B1 `sig_status` marker was emitted;
- no B1 `handshake_send` marker was emitted;
- no B1 relay output was present on the initiator channel.

Marker:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK`

## sig_sign_failed proof

Forced B1 signing failure uses the existing sanitized reject reason:

- `sig_sign_failed`

The integration test asserts the marker appears and emits:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK`

No provider internals, secret bytes, route tokens, passphrases, stack traces, or
panic output are accepted in test output.

## no responder mutation proof

The forced B1 signing failure test snapshots Bob's vault bytes before the
forced poll and asserts after the poll:

- Bob vault bytes are unchanged.
- `handshake.pending.bob.alice` is absent from the mock vault.
- legacy `handshake_pending_bob_alice.json` is absent.
- `qsp_sessions/alice.qsv` is absent.

Marker:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RESPONDER_MUTATION_OK`

## no B1 output proof

The forced B1 signing failure test asserts:

- command output does not contain `event=handshake_send`;
- command output does not contain `msg=B1`;
- the initiator relay channel is empty after Bob's forced-failure poll.

Marker:

- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_B1_OUTPUT_OK`

## production semantics unchanged proof

The no-cfg test `b1_signature_rng_failure_seam_inactive_without_cfg` sets the
same selector and proves the normal B1 path still succeeds:

- output contains `event=sig_status`;
- output contains `reason=b1_sign`;
- output contains `event=handshake_send`;
- output contains `msg=B1`;
- output does not contain `sig_sign_failed`;
- output does not contain `rng_failure_forced`;
- B1 relay output is queued.

Marker:

- `NA0461_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## A2 signing residual deferral

A2 signing remains deferred. NA-0461 does not add an A2 signing seam because the
A2 signing point occurs after initiator session storage and pending clear. A
future lane must define a different truthful invariant for that post-mutation
timing.

Marker:

- `NA0461_A2_SIGNATURE_RESIDUAL_DEFERRED_OK`

## identity residual deferral

Identity provider RNG remains deferred across lazy identity creation,
legacy/public-record upgrade, CLI rotation, and TUI account bootstrap. NA-0461
does not mutate `identity/mod.rs`, `main.rs`, or TUI command paths.

Marker:

- `NA0461_IDENTITY_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## X25519 residual deferral

X25519 / ephemeral generation remains a separate backlog surface. NA-0461 does
not add X25519 seams or tests.

Marker:

- `NA0461_X25519_RESIDUAL_DEFERRED_OK`

## refimpl residual deferral

refimpl provider RNG remains deferred. NA-0461 does not mutate `tools/refimpl/**`
or claim refimpl provider RNG coverage.

Marker:

- `NA0461_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`

## KEM background preservation

NA-0458 KEM provider RNG seam evidence remains green and is background
preserved. NA-0461 does not weaken or broaden the KEM seam.

Post-implementation validation reran:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Marker:

- `NA0461_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`

## no dependency/workflow/refimpl mutation proof

NA-0461 did not mutate:

- `Cargo.toml`
- `Cargo.lock`
- `qsl/qsl-client/qsc/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `.github/workflows/**`
- `tools/refimpl/**`

Markers:

- `NA0461_NO_DEPENDENCY_CHANGE_OK`
- `NA0461_NO_WORKFLOW_CHANGE_OK`

## public claim boundary

NA-0461 is bounded internal qsc evidence only. No public-readiness claim is
made. No production-readiness claim is made. No public-internet-readiness claim
is made. No external-review-complete claim is made. No crypto-complete claim is
made. No signature-complete claim is made. No identity-complete claim is made.
No RNG-failure-complete claim is made. No provider-RNG-complete claim is made.
No side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made.

## validation

Post-implementation validation passed:

```bash
cargo fmt --check
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

The pqcrypto inverse probes returned expected zero-match package-ID results
under `|| true`.

## scope guard

Changed paths are limited to the allowed NA-0461 implementation and governance
paths. No qsl-server, qsl-attachments, qshield runtime, qshield-cli, refimpl,
dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model,
public docs, website, README, START_HERE, qwork/qstart/qresume/qshell, backup,
restore, qsl-backup, status, plan, rollback, or backup tree path is mutated.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, timers, fstab, source lists, retention, backup scripts, or backup tree
paths.

Read-only boundary proof:

- `/usr/local/sbin/qsl-backup` SHA matched the required value.
- the script-local Codex ops source-list inclusion count was 1.

This is same-host continuity boundary evidence only. No off-host backup,
disaster-recovery, restore-proven, backup-complete, or key-custody claim is
made.

## successor selection

Selected successor:

`NA-0462 -- QSL qsc A2 Signature Provider RNG Failure Scope Authorization Plan`

Rationale:

- NA-0461 implements B1 signing only.
- NA-0460 explicitly deferred A2 because it needs a different truthful
  post-mutation invariant.
- Identity generation remains deferred until A2 signing is scoped or explicitly
  superseded.
- X25519 and refimpl provider RNG remain separate residuals.

## next recommendation

After NA-0461 merges and post-merge public-safety is green, close out NA-0461
and restore NA-0462 as the sole READY item. Do not implement NA-0462 inside the
closeout.
