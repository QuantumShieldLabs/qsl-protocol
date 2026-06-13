Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0472 qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0472 implementation lane for qsc TUI account bootstrap
pre-generation transactionality. The lane must implement the NA-0471 selected
behavior, prove forced TUI bootstrap KEM and signature identity provider RNG
failures happen before durable account/default/identity writes, and preserve
normal no-cfg production behavior.

## Protected invariants

- NA-0472 consumes NA-0471.
- Exactly one READY item remains before optional closeout.
- TUI bootstrap identity KEM/signature material is generated before
  `tui_try_vault_init` and before account/default writes.
- Forced TUI bootstrap KEM identity provider failure writes no partial
  account/default/identity state.
- Forced TUI bootstrap signature identity provider failure writes no partial
  account/default/identity state.
- Forced failure emits sanitized failure output and no setup-success output.
- Normal no-cfg TUI bootstrap ignores `QSC_RNG_FAILURE_TEST_SEAM`.
- The in-memory secret lifetime caveat introduced by pre-generation is
  preserved.
- No dependency, Cargo manifest, lockfile, workflow, refimpl, qshield-cli,
  qsl-server, qsl-attachments, fuzz target, vector, formal model, public doc,
  README, START_HERE, qwork/qstart/qresume/qshell, backup/restore, qsl-backup,
  status, plan, rollback, or `/backup/qsl` mutation occurs.

## Allowed scope

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs`
- `docs/governance/evidence/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- No dependency mutation.
- No Cargo manifest mutation.
- No lockfile mutation.
- No workflow mutation.
- No `vault/mod.rs` mutation.
- No `main.rs` mutation.
- No `handshake/mod.rs` mutation.
- No unrelated TUI command mutation.
- No qshield-cli mutation.
- No refimpl mutation.
- No qsl-server mutation.
- No qsl-attachments mutation.
- No fuzz target mutation.
- No vector mutation.
- No formal model mutation.
- No website/public-doc/README/START_HERE mutation.
- No qwork/qstart/qresume/qshell mutation.
- No backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  or `/backup/qsl` mutation.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
```

Required tests:

- `tui_bootstrap_kem_rng_failure_writes_no_partial_account_state`
- `tui_bootstrap_sig_rng_failure_writes_no_partial_account_state`

Required labels:

- `QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR`
- `QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR`

Required output markers:

- `NA0472_TUI_TRANSACTIONALITY_DESIGN_CONSUMED_OK`
- `NA0472_TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTED_OK`
- `NA0472_TUI_BOOTSTRAP_KEM_RNG_FAILURE_FORCED_OK`
- `NA0472_TUI_BOOTSTRAP_SIG_RNG_FAILURE_FORCED_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_DEFAULT_CONFIG_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_IDENTITY_KEM_SECRET_WRITE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_IDENTITY_SIG_SECRET_WRITE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_SELF_PUBLIC_RECORD_WRITE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK`
- `NA0472_IN_MEMORY_SECRET_LIFETIME_CAVEAT_PRESERVED_OK`
- `NA0472_CLI_ROTATION_BACKGROUND_PRESERVED_OK`
- `NA0472_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK`
- `NA0472_LEGACY_PUBLIC_RECORD_BACKGROUND_PRESERVED_OK`
- `NA0472_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0472_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0472_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`
- `NA0472_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`
- `NA0472_X25519_RESIDUAL_DEFERRED_OK`
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

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
```

Required test:

- `tui_bootstrap_pregeneration_seam_inactive_without_cfg`

Required marker:

- `NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Expected result:

- setting `QSC_RNG_FAILURE_TEST_SEAM` in a no-cfg build does not force failure;
- normal TUI bootstrap writes the vault, account defaults, identity secrets,
  and self public record.

## no partial account/default assertions

For forced KEM and signature failures, tests must assert:

- no `vault.qsv`;
- no `vault.qsv.tmp`;
- no `profile_alias`;
- no TUI autolock default;
- no TUI poll defaults;
- no receipt/file defaults;
- no account verification seed;
- no relay endpoint/token defaults;
- no TUI relay inbox token default.

No vault file is the proof mechanism for vault-backed defaults.

## no partial identity assertions

For forced KEM and signature failures, tests must assert:

- no `identity.kem_sk.self`;
- no `identity.sig_sk.self`;
- no `identities/self_self.json`;
- any `identities/` directory created by TUI startup is absent or empty.

## no success-output assertions

For forced KEM and signature failures, tests must assert:

- output includes `identity_secret_unavailable`;
- output includes `rng_failure_forced`;
- output includes `event=tui_init_reject`;
- output includes `reason=identity_init_failed`;
- output does not include `event=tui_init ok=true`;
- output does not include `alias=stored_local_only`;
- output does not include `identity_fp=`;
- output does not include identity secret names, passphrases, panic text, or
  stack traces.

## in-memory secret lifetime caveat

Evidence must state that pre-generation increases the lifetime of generated
identity secret material in memory during the bootstrap commit sequence.

The implementation should zeroize new pre-generated secret vectors and
temporary hex strings at the new storage boundary where practical, but no
side-channel-free claim is made and no secret-material-complete claim is made.

## inherited tests

Run after implementation:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Expected result: all pass.

## root audit

Run:

```bash
cargo audit --deny warnings
```

Expected result: PASS.

Cargo audit green is dependency-health evidence only.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Expected result: PASS.

## qsc adversarial check

Run syntax checks:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run:

```bash
scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected result: PASS.

These checks preserve formal-model health but do not map the TUI bootstrap
pre-generation invariant into a formal model in this lane.

## assurance sections

Evidence must include:

- Best-Known-Method Review.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Side-Channel Caveat.
- Formal-Model Mapping Residual.
- External-Review Readiness.
- Release-Claim Boundary.
- Assurance Gap Review Trigger.

## scope guard

Before PR, run a scope guard that permits only the allowed NA-0472
implementation/governance paths.

Expected result:

- no `vault/mod.rs` mutation;
- no `main.rs` mutation;
- no `handshake/mod.rs` mutation;
- no unrelated TUI command mutation;
- no qshield-cli mutation;
- no refimpl mutation;
- no dependency/Cargo/lockfile/workflow/fuzz-target/vector/formal mutation;
- no qsl-server or qsl-attachments mutation;
- no public docs, README, or START_HERE mutation.

## public claim boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
secret-material-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made.

## closeout prerequisites

Optional closeout to NA-0473 may run only if:

- implementation PR merges;
- post-merge public-safety is green;
- D-0932 exists once on main;
- READY remains NA-0472 before closeout;
- no higher-priority residual supersedes Assurance Gap Review.
