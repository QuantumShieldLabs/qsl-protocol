Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0467 qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded NA-0467 qsc legacy/public-record identity provider RNG
failure test seam implementation. The lane must implement only the selected
cfg-only legacy migration and public-record upgrade signature keypair failure
labels, prove no partial upgrade state, preserve normal no-cfg behavior, and
avoid public overclaims.

## Protected invariants

- Exactly one READY item remains before implementation closeout.
- D-0920 exists once before patch.
- D-0921 is absent before patch and exists once after patch.
- D-0922 remains absent before optional closeout.
- The seam is compiled only under `--cfg qsc_rng_failure_test_seam`.
- Normal builds do not read `QSC_RNG_FAILURE_TEST_SEAM`.
- Forced legacy migration failure occurs before migration writes.
- Forced public-record upgrade failure occurs before signature-secret and
  public-record update writes.
- Existing identity/public-record state remains stable.
- No signature secret is written on forced failure.
- No partial self public record is written on forced failure.
- Selected identity fingerprint remains stable.
- No dependent handshake state or relay output is produced on forced failure.
- Lazy identity, A2, B1, KEM, base RNG seam, key lifecycle, and provider-error
  evidence remains bounded background evidence.
- CLI identity rotation, TUI bootstrap, X25519, refimpl provider RNG,
  qshield-cli demo RNG, formal/model RNG, and fuzz/vector RNG remain residual.
- No dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model,
  refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
  public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore,
  qsl-backup, backup status, backup plan, rollback, or backup tree mutation
  occurs.

## Allowed scope

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to handshake source, main CLI source, TUI command source, dependencies,
Cargo manifests, lockfiles, workflows, fuzz target source, vectors, formal
models, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup,
restore, qsl-backup, backup status, backup plan, rollback, or backup tree paths.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `legacy_identity_migrate_sig_rng_failure_writes_no_partial_upgrade_state`
  passes.
- `public_record_upgrade_sig_rng_failure_writes_no_partial_upgrade_state`
  passes.
- required NA0467 forced-failure and boundary markers are emitted.

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `legacy_identity_public_record_rng_failure_seam_inactive_without_cfg` passes.
- `NA0467_PRODUCTION_SEMANTICS_UNCHANGED_OK` is emitted.
- no `rng_failure_forced` or `identity_secret_unavailable` appears in normal
  successful output.

## existing identity state stability assertions

Cfg tests must record pre-failure identity file bytes, vault bytes, and
identity fingerprint, then assert equality after forced failure.

Required marker:

- `NA0467_EXISTING_IDENTITY_STATE_STABLE_OK`

## no signature secret write assertions

Cfg tests must decrypt the temporary mock vault and prove
`identity.sig_sk.alice` is absent after forced failure.

Required marker:

- `NA0467_NO_NEW_SIGNATURE_SECRET_WRITE_OK`

## no public-record partial write assertions

Cfg tests must assert the selected `identities/self_alice.json` bytes are
unchanged after forced failure.

Required marker:

- `NA0467_NO_PARTIAL_SELF_PUBLIC_RECORD_WRITE_OK`

## no selected identity change assertions

Cfg tests must assert the selected Alice identity fingerprint is unchanged
after forced failure.

Required marker:

- `NA0467_NO_SELECTED_IDENTITY_CHANGE_OK`

## dependent handshake/output absence assertions

Cfg tests must assert no pending handshake vault secret, no legacy pending file,
no session blob, no `handshake_send`, no `handshake_complete`, and no relay A1
output after forced failure.

Required marker:

- `NA0467_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK`

## inherited lazy identity tests

Run cfg/no-cfg `lazy_identity_provider_rng_failure`. Required: PASS. Lazy
identity remains background evidence only.

## inherited A2/B1/KEM provider RNG tests

Run cfg/no-cfg:

- `a2_signature_provider_rng_failure`
- `b1_signature_provider_rng_failure`
- `kem_provider_rng_failure`

Required: PASS. These remain background checks only.

## inherited RNG residual tests

Run cfg/no-cfg:

- `rng_failure_residual_surfaces`
- `rng_failure_behavior`

Required: PASS.

## inherited provider-error/key-lifecycle tests

Run:

- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`

Required: PASS.

## root audit

Run:

```bash
cargo audit --deny warnings
```

Required: PASS. This is dependency-health evidence only.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required: PASS. This is dependency-health evidence only.

## qsc adversarial check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If local qsc adversarial smoke is feasible, run it. If cargo-fuzz is
unavailable locally, record the exact local output and rely on PR CI
qsc-adversarial-smoke.

## formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required: PASS.

## scope guard

Run a name-only diff against `origin/main` and verify changed paths are limited
to the allowed NA-0467 implementation/test/governance paths.

Forbidden-path checks must prove no mutation to handshake source, main CLI
source, TUI command source, refimpl, dependencies, Cargo manifests, lockfiles,
workflows, fuzz targets, vectors, formal models, qsl-server, qsl-attachments,
qshield runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status files, backup plan files,
rollback, or backup tree paths.

## public claim boundary

Required:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no signature-complete claim;
- no identity-complete claim;
- no RNG-failure-complete claim;
- no provider-RNG-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## closeout prerequisites

Do not close out NA-0467 until the implementation PR merges and post-merge
public-safety is green. The default selected successor is:

`NA-0468 -- QSL qsc CLI Identity Rotation Provider RNG Failure Scope Authorization Plan`
