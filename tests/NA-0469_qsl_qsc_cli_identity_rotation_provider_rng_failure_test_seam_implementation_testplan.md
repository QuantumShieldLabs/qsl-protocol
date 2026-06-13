Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0469 qsc CLI Identity Rotation Provider RNG Failure Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded NA-0469 implementation lane for qsc CLI identity rotation
provider RNG failure. The lane must implement cfg-only KEM and signature
rotation failure labels, prove no partial rotation state, preserve no-cfg
production semantics, and avoid public overclaim.

## Protected Invariants

- Exactly one READY item remains before optional closeout.
- D-0924 exists once before patch.
- D-0925 is absent before patch and exists once after patch.
- D-0926 remains absent before optional closeout.
- CLI rotation forced failure returns deterministic sanitized errors.
- CLI rotation forced failure occurs before selected identity, vault,
  public-record, contact, peer-reset, handshake, or session writes.
- Lazy identity and legacy/public-record seams remain bounded background.
- TUI account bootstrap, X25519 / ephemeral, refimpl provider RNG, qshield-cli
  demo RNG, formal/model RNG, and fuzz/vector RNG remain residual.
- No dependency, Cargo manifest, lockfile, workflow, qsl-server,
  qsl-attachments, qshield runtime, qshield-cli, refimpl, public doc, website,
  README, START_HERE, fuzz target, vector, formal model, backup, restore,
  qsl-backup, backup status, backup plan, rollback, or qwork mutation occurs.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## Allowed Scope

- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/cli_identity_rotation_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to handshake source, TUI command source, qshield-cli, refimpl,
dependencies, Cargo manifests, lockfiles, workflows, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
fuzz target source, vectors, formal models, qwork/qstart/qresume/qshell,
backup/restore/local-ops paths, qsl-backup, backup status, backup plan,
rollback subtree paths, or `/backup/qsl`.

## cfg Seam Tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `cli_identity_rotate_kem_rng_failure_writes_no_partial_rotation_state` PASS.
- `cli_identity_rotate_sig_rng_failure_writes_no_partial_rotation_state` PASS.
- `common_na0469_markers` PASS.
- required NA0469 cfg markers appear.

## Normal-Build Tests

Run:

```bash
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `cli_identity_rotation_rng_failure_seam_inactive_without_cfg` PASS.
- `QSC_RNG_FAILURE_TEST_SEAM` is ignored by a normal build.
- `NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK` appears.

## Selected Identity Stability Assertions

Forced KEM and signature tests must prove:

- `identity show --as alice` returns the same fingerprint before and after
  forced failure.
- `identities/self_alice.json` remains byte-for-byte unchanged.
- forced-failure output contains no replacement `identity_fp=...` line.

## No Partial Secret Write Assertions

Forced KEM and signature tests must prove:

- `vault.qsv` remains byte-for-byte unchanged.
- `identity.kem_sk.alice` remains unchanged.
- `identity.sig_sk.alice` remains unchanged.
- no secret material appears in command output.

## Public-Record Partial Write Assertions

Forced KEM and signature tests must prove `identities/self_alice.json` remains
byte-for-byte unchanged after each forced failure attempt.

## Peer-Reset/Contact State Assertions

Forced KEM and signature tests must run rotation with `--reset-peers` and prove:

- contact list output remains unchanged;
- seeded legacy `peer_bob.fp` remains byte-for-byte unchanged;
- the reset path was not reached.

## Dependent Handshake/Session Output Absence Assertions

Forced KEM and signature tests must prove:

- no `event=handshake_send`;
- no `event=handshake_complete`;
- no `event=qsp_session_store`;
- no pending handshake vault secret;
- no legacy pending handshake file;
- no `qsp_sessions/bob.qsv` session blob.

## Inherited Lazy and Legacy/Public-Record Tests

Run cfg/no-cfg:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
```

Required: PASS. These remain inherited background checks only.

## Inherited A2/B1/KEM Provider RNG Tests

Run cfg/no-cfg:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required: PASS. These remain bounded background checks only.

## Inherited RNG Residual Tests

Run cfg/no-cfg:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required: PASS.

## Inherited Provider-Error/Key-Lifecycle Tests

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Required: PASS.

## Root Audit

Run:

```bash
cargo audit --deny warnings
```

Required: PASS. This is dependency-health evidence only.

## Nested Fuzz Lock Audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: audit PASS and pqcrypto residual scan zero-match.

## qsc Adversarial Check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible locally, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

Required: syntax PASS. If local `cargo fuzz` is unavailable, record exact
output and rely on PR CI `qsc-adversarial-smoke`.

## Formal Checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required: PASS.

## Scope Guard

Run `git diff --name-only` plus untracked inventory. Required changed paths are
limited to the eight allowed NA-0469 implementation/governance paths.

Run `git diff --check`. Required: PASS.

## Public Claim Boundary

Run leak and overclaim scans against added lines. Required:

- no secret/token/auth/header/long-hex evidence leak;
- no affirmative public-readiness claim;
- no affirmative production-readiness claim;
- no affirmative external-review-complete claim;
- no affirmative crypto-complete claim;
- no affirmative signature-complete claim;
- no affirmative identity-complete claim;
- no affirmative RNG-failure-complete claim;
- no affirmative provider-RNG-complete claim;
- no affirmative side-channel-free claim;
- no affirmative vulnerability-free claim;
- no affirmative bug-free claim;
- no affirmative perfect-crypto claim.

## Closeout Prerequisites

Do not close out NA-0469 unless:

- implementation PR merges;
- required checks are green or accepted skipped/neutral by repo policy;
- post-merge public-safety is green;
- queue still has exactly one READY item;
- D-0925 exists once on main;
- no forbidden mutation or public overclaim is introduced.
