Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0472 QSL qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0472 consumes NA-0471 and implements the selected qsc TUI account bootstrap
pre-generation transactionality lane.

Implemented behavior:

- TUI `/init` and the init wizard now generate the bootstrap identity KEM and
  signature keypairs before `tui_try_vault_init` and before durable
  account/default writes.
- Forced TUI bootstrap KEM provider RNG failure under
  `QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR` returns sanitized
  `identity_secret_unavailable` / `rng_failure_forced` output before vault,
  account/default, identity secret, and self public-record writes.
- Forced TUI bootstrap signature provider RNG failure under
  `QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR` returns the same sanitized failure
  shape before those durable writes.
- Normal builds do not compile or read the seam selector and preserve normal TUI
  bootstrap behavior.

Changed implementation paths:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`

Added qsc test file:

- `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs`

This evidence is bounded internal qsc evidence only. No identity-complete claim
is made. No signature-complete claim is made. No RNG-failure-complete claim is
made. No provider-RNG-complete claim is made. No crypto-complete claim is made.
No side-channel-free claim is made. No public-readiness claim is made. Cargo
audit green is dependency-health evidence only.

## Live NA-0472 scope

qwork proof files were read without rerunning qwork:

- `/srv/qbuild/work/NA-0472/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0472/.qwork/startup.qsl-protocol.json`

Verified startup facts:

- `startup_result=OK`
- lane `NA-0472`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0472/qsl-protocol`
- proof HEAD and proof `origin/main`: `eb93fc18c3cd`
- clean worktree, index, and untracked state before mutation
- READY_COUNT 1
- sole READY item: NA-0472
- requested lane status: READY

Allowed implementation paths:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs`

Allowed governance paths:

- this evidence doc
- `tests/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope was preserved for dependencies, Cargo manifests,
lockfiles, workflows, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, refimpl, fuzz targets, vectors, formal models, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, backup/restore paths,
qsl-backup, backup status files, backup plan files, rollback subtree paths,
`/backup/qsl`, and public technical paper content.

## qwork proof-file verification

The `.kv` and `.json` qwork proof files were present, valid, and mutually
consistent for lane, repo, path, HEAD, `origin/main`, READY count, queue top
READY, requested lane status, and clean-state fields.

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- local `main` was reset to verified `origin/main`;
- PR #1213 was verified MERGED at `eb93fc18c3cd`;
- current main public-safety completed success.

Codex did not run `qwork`, `qstart`, or `qresume`.

## NA-0471 inheritance

NA-0471 selected:

`TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTATION_READY`

Controlling inherited finding:

- previous TUI bootstrap order wrote vault/account/default state before
  identity KEM/signature generation;
- a simple identity seam at the old generation point could not prove no partial
  account/default state;
- NA-0471 rejected rollback and broad staged-commit infrastructure for this
  immediate successor;
- NA-0471 selected pre-generation before durable account/default writes and
  recorded the in-memory secret lifetime caveat.

NA-0472 consumes that design and implements only the selected bounded
pre-generation behavior.

Marker:

- `NA0472_TUI_TRANSACTIONALITY_DESIGN_CONSUMED_OK`

## Pre-generation implementation summary

`identity/mod.rs` now provides the cfg-only TUI bootstrap identity keypair seam:

- `QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR`
- `QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR`

`locked.rs` now calls the TUI bootstrap identity pre-generation helper before
both TUI bootstrap entry points call `tui_try_vault_init`:

- locked init wizard confirmation path;
- locked `/init <alias> <passphrase> <confirm> <decision>` path.

If pre-generation succeeds, the generated identity material is passed into the
existing account/default/identity commit sequence. The identity secret vectors
and hex-encoded temporary strings are zeroized after use where this lane
introduced the longer pre-generation lifetime.

Marker:

- `NA0472_TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTED_OK`

## cfg seam labels and normal-build boundary

The forced-failure labels are compiled only under:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam'
```

Normal builds compile the no-cfg keypair path and do not read
`QSC_RNG_FAILURE_TEST_SEAM` for TUI bootstrap identity keypair generation.

The no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM` to the TUI KEM label and proves
normal TUI bootstrap still succeeds.

Marker:

- `NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## TUI bootstrap KEM forced-failure proof

Test:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
```

The test `tui_bootstrap_kem_rng_failure_writes_no_partial_account_state` forces
`QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR`.

Observed invariant:

- output includes `identity_secret_unavailable`;
- output includes `rng_failure_forced`;
- output includes `event=tui_init_reject`;
- output does not include `event=tui_init ok=true`;
- no `vault.qsv` or `vault.qsv.tmp` is written;
- no self public record is written;
- no identity secret can be written because no vault is created.

Marker:

- `NA0472_TUI_BOOTSTRAP_KEM_RNG_FAILURE_FORCED_OK`

## TUI bootstrap signature forced-failure proof

The test `tui_bootstrap_sig_rng_failure_writes_no_partial_account_state` forces
`QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR`.

Observed invariant:

- output includes `identity_secret_unavailable`;
- output includes `rng_failure_forced`;
- output includes `event=tui_init_reject`;
- output does not include `event=tui_init ok=true`;
- no `vault.qsv` or `vault.qsv.tmp` is written;
- no self public record is written;
- no identity secret can be written because no vault is created.

Marker:

- `NA0472_TUI_BOOTSTRAP_SIG_RNG_FAILURE_FORCED_OK`

## No partial account/default state proof

Forced KEM and signature failures occur before `tui_try_vault_init` and before
`initialize_account_after_init`.

The cfg tests prove:

- no vault file is written;
- no temporary vault file is left;
- no `profile_alias` can be written;
- no TUI autolock, polling, receipt, file-confirm, verification seed, relay
  endpoint, relay token, or relay inbox token default can be written because
  the vault does not exist.

Markers:

- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_DEFAULT_CONFIG_OK`

## No partial identity state proof

The cfg tests prove:

- no identity KEM secret write;
- no identity signature secret write;
- no self public record write;
- if TUI startup creates an empty `identities/` directory, it remains empty and
  contains no identity record.

Markers:

- `NA0472_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_IDENTITY_KEM_SECRET_WRITE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_IDENTITY_SIG_SECRET_WRITE_OK`
- `NA0472_TUI_BOOTSTRAP_NO_SELF_PUBLIC_RECORD_WRITE_OK`

## No misleading setup-success output proof

Forced KEM and signature failures emit sanitized failure markers and do not emit
successful setup output.

The cfg tests assert:

- output contains `event=tui_init_reject`;
- output contains `reason=identity_init_failed`;
- output does not contain `event=tui_init ok=true`;
- output does not contain `alias=stored_local_only`;
- output does not contain `identity_fp=`;
- output does not contain identity secret names or the TUI passphrase.

Marker:

- `NA0472_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK`

## Production semantics unchanged proof

The no-cfg test `tui_bootstrap_pregeneration_seam_inactive_without_cfg` sets
`QSC_RNG_FAILURE_TEST_SEAM=QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR` and runs
normal headless TUI bootstrap.

The test proves:

- normal TUI bootstrap emits `event=tui_init ok=true`;
- normal TUI bootstrap writes `vault.qsv`;
- normal TUI bootstrap writes `identities/self_self.json`;
- normal TUI bootstrap stores `profile_alias`;
- normal TUI bootstrap stores expected account defaults;
- normal TUI bootstrap stores `identity.kem_sk.self`;
- normal TUI bootstrap stores `identity.sig_sk.self`;
- forced-failure markers are absent.

Marker:

- `NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## In-memory secret lifetime caveat

Pre-generation intentionally moves identity KEM/signature generation before
vault/account/default writes. That means generated identity secret material can
exist in memory while `tui_try_vault_init` and account/default commit steps run.

The implementation zeroizes the pre-generated identity secret vectors and the
hex-encoded temporary strings at the new storage boundary, and zeroizes the
pre-generated secrets on early account/default failure paths introduced by this
lane. This is still not a side-channel-free claim, not a complete
secret-material lifecycle claim, and not a memory-erasure completeness claim.

Marker:

- `NA0472_IN_MEMORY_SECRET_LIFETIME_CAVEAT_PRESERVED_OK`

## Background / residual preservation

Post-implementation inherited tests remained green:

- CLI identity rotation provider RNG cfg/no-cfg tests;
- lazy identity provider RNG cfg/no-cfg tests;
- legacy/public-record provider RNG cfg/no-cfg tests;
- A2 signature provider RNG cfg/no-cfg tests;
- B1 signature provider RNG cfg/no-cfg tests;
- KEM provider RNG cfg/no-cfg tests;
- qsc key lifecycle zeroization test;
- qsc provider-error no-mutation test.

Residuals remain:

- X25519 / ephemeral generation remains deferred.
- refimpl provider RNG remains deferred.
- qshield-cli demo RNG remains demo-local residual.
- formal/model RNG and fuzz/vector RNG remain residual.

Markers:

- `NA0472_CLI_ROTATION_BACKGROUND_PRESERVED_OK`
- `NA0472_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK`
- `NA0472_LEGACY_PUBLIC_RECORD_BACKGROUND_PRESERVED_OK`
- `NA0472_A2_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0472_B1_SIGNATURE_BACKGROUND_PRESERVED_OK`
- `NA0472_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK`
- `NA0472_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK`
- `NA0472_X25519_RESIDUAL_DEFERRED_OK`

## No dependency/workflow/refimpl/qshield-cli mutation proof

Changed paths are limited to the allowed NA-0472 qsc implementation/test paths
and governance paths.

No dependency change is made. No Cargo manifest change is made. No lockfile
change is made. No workflow change is made. No refimpl mutation is made. No
qshield-cli mutation is made. No qsl-server mutation is made. No
qsl-attachments mutation is made.

Markers:

- `NA0472_NO_DEPENDENCY_CHANGE_OK`
- `NA0472_NO_WORKFLOW_CHANGE_OK`

## Best-Known-Method Review

Classification: `BEST_KNOWN_METHOD_FOR_SCOPE`.

Pre-generation is the best-known bounded method for this exact lane because it
forces identity-provider RNG failures before durable account/default writes
without inventing vault-wide transactionality, temp-file staging, rollback
deletion logic, workflow changes, dependency changes, or refimpl changes.

## Hostile Cryptographer Review

Findings:

- This change improves failure ordering for TUI bootstrap identity provider RNG
  failures only.
- This change does not prove all identity/provider RNG paths complete.
- This change increases in-memory lifetime for generated bootstrap identity
  secrets during the durable commit sequence.
- This change does not prove side-channel resistance.
- This change does not update formal models and does not close protocol-binding
  residuals.

## Red-Team Review

Abuse cases covered:

- forced KEM failure cannot leave partial vault/account/default/identity state;
- forced signature failure cannot leave partial vault/account/default/identity
  state;
- forced failure cannot emit setup-success output;
- normal no-cfg behavior cannot be disabled by setting the seam selector.

Remaining red-team residuals:

- refimpl provider RNG remains deferred;
- X25519 / ephemeral generation remains deferred;
- side-channel and full secret-material lifecycle review remain future work.

## Production SRE Review

Operator-visible failure remains fail-closed and sanitized:

- failure surfaces as TUI init reject with `identity_init_failed`;
- setup-success output is absent;
- no partial vault/default state is left for identity RNG failures;
- normal TUI bootstrap still succeeds in no-cfg builds.

No backup, restore, qsl-backup, status, plan, rollback subtree, systemd, timer,
fstab, or `/backup/qsl` mutation occurred.

## Side-Channel Caveat

No side-channel-free claim is made. No constant-time proof is made. No
memory-erasure completeness claim is made. No secret-material-complete claim is
made.

## Formal-Model Mapping Residual

Classification: `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.

Formal checks were run and remained green, but this TUI bootstrap ordering
change is not mapped into a formal model in this lane.

## External-Review Readiness

Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.

The evidence improves internal review readiness for one bounded qsc TUI
bootstrap failure-order invariant. No external-review-complete claim is made.

## Release-Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
secret-material-complete claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto
claim is made.

## Assurance Gap Review Trigger

Classification: `ASSURANCE_GAP_REVIEW_REQUIRED_AFTER_CURRENT_CHAIN`.

NA-0472 completes the direct TUI bootstrap pre-generation residual selected by
NA-0471. The default successor is:

`NA-0473 -- QSL Identity / Provider RNG Assurance Gap Review Plan`

unless public-safety or implementation evidence proves a higher-priority
residual before closeout.

## Validation

Startup validation:

- qwork proof files verified without rerunning qwork.
- PR #1213 verified merged at `eb93fc18c3cd`.
- current main public-safety PASS.
- root cargo audit PASS.
- nested qsc fuzz lock audit PASS.
- inherited qsc provider RNG, key lifecycle, and provider-error tests PASS.
- qsl-backup SHA/source-count boundary PASS.

Implementation validation:

- `cargo fmt --check` PASS after one formatting recovery.
- cfg NA-0472 test PASS after one test-harness assertion recovery.
- no-cfg NA-0472 test PASS.
- inherited qsc cfg/no-cfg provider RNG tests PASS.
- qsc `key_lifecycle_zeroization` PASS.
- qsc `handshake_provider_error_no_mutation` PASS.
- qsc stable `send_commit` PASS.
- refimpl `pqkem768` PASS.
- `sh -n scripts/ci/qsc_adversarial.sh` PASS.
- `bash -n scripts/ci/qsc_adversarial.sh` PASS.
- root cargo audit PASS.
- nested qsc fuzz lock audit PASS.
- dependency probes completed; pqcrypto inverse probes returned expected
  package-ID absence.
- formal model scripts PASS.

Recovered failures are recorded in `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Scope guard

Allowed changed paths for this implementation PR:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/tui_account_bootstrap_transactionality.rs`
- `docs/governance/evidence/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_harness.md`
- `tests/NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `vault/mod.rs`, `main.rs`, `handshake/mod.rs`, unrelated TUI command,
qshield-cli, refimpl, dependency, Cargo, lockfile, workflow, fuzz target,
vector, formal model, qsl-server, qsl-attachments, website, public doc, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, or `/backup/qsl` mutation is included.

## Backup-impact statement

No backup was run. No restore was run. No qsl-backup mutation occurred. No
backup status file was mutated. No backup plan file was mutated. No rollback
subtree or `/backup/qsl` mutation occurred.

qsl-backup read-only proof at startup:

- SHA matched the expected value.
- script-local ops source inclusion count was 1.

Same-host proof roots are local operational evidence only. They are not
off-host backup, not disaster recovery, and not restore proof.

## Successor selection

Selected successor after successful implementation merge and post-merge
public-safety:

`NA-0473 -- QSL Identity / Provider RNG Assurance Gap Review Plan`

Rationale:

- NA-0472 completes the direct TUI bootstrap pre-generation implementation
  selected by NA-0471.
- D-0927 and D-0931 both keep Assurance Gap Review as the default after the
  direct residual chain.
- A full assurance review is now needed across KEM, B1 signing, A2 no-output,
  lazy identity, legacy/public-record, CLI rotation, TUI bootstrap, route,
  contact, attachment, key lifecycle, provider-error, formal/model,
  side-channel, external-review readiness, and release-claim boundaries.

## Next recommendation

Merge NA-0472 only after required checks pass. If post-merge public-safety is
green, perform the optional closeout restoring NA-0473 as the sole READY item
without implementing NA-0473.
