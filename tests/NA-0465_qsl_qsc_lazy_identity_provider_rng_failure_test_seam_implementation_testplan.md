Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0465 qsc Lazy Identity Provider RNG Failure Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0465 bounded implementation lane for a qsc cfg-only lazy
identity provider RNG failure test seam. The lane must force selected lazy
identity KEM and signature key-generation failure only when
`qsc_rng_failure_test_seam` is active, prove sanitized failure, prove no partial
identity state, prove no dependent handshake state/output, preserve normal
no-cfg production semantics, and preserve all residual boundaries selected by
NA-0464.

## Protected invariants

- The lazy identity seam is compiled only under
  `--cfg qsc_rng_failure_test_seam`.
- Normal builds do not read `QSC_RNG_FAILURE_TEST_SEAM`.
- Forced lazy identity KEM keypair failure returns sanitized
  `identity_secret_unavailable` / `rng_failure_forced`.
- Forced lazy identity signature keypair failure returns sanitized
  `identity_secret_unavailable` / `rng_failure_forced`.
- Forced lazy identity failure writes no identity KEM secret.
- Forced lazy identity failure writes no identity signature secret.
- Forced lazy identity failure writes no self public record.
- Forced lazy identity failure writes no selected identity state.
- Forced lazy identity failure writes no pending handshake state.
- Forced lazy identity failure writes no session state.
- Forced lazy identity failure emits no handshake output.
- KEM provider RNG evidence remains background only.
- B1 signing provider RNG evidence remains background only.
- A2 signing provider RNG evidence remains background only.
- Legacy/public-record upgrade remains residual.
- CLI identity rotation remains residual.
- TUI account bootstrap identity generation remains residual.
- X25519 / ephemeral generation remains residual.
- refimpl provider RNG remains residual.
- qshield-cli demo RNG remains residual.
- formal/model RNG and fuzz/vector RNG remain residual.
- No dependency, Cargo, lockfile, workflow, refimpl, qsl-server,
  qsl-attachments, qshield runtime, qshield-cli, public docs, README,
  START_HERE, fuzz target, vector, or formal model mutation occurs.
- No public-readiness claim is made.
- No production-readiness claim is made.
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
- Exactly one READY item remains mandatory.

## Allowed scope

Implementation paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`

Governance paths:

- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to handshake source, main CLI source, TUI command source, refimpl,
dependencies, Cargo manifests, lockfiles, workflows, fuzz target source,
vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree paths.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
```

Required tests:

- `lazy_identity_kem_rng_failure_writes_no_identity_state`
- `lazy_identity_sig_rng_failure_writes_no_identity_state`
- `common_na0465_markers`

Required labels:

- `QSC.IDENTITY.LAZY.KEM_KEYPAIR`
- `QSC.IDENTITY.LAZY.SIG_KEYPAIR`

Compatibility label preserved:

- `QSC.KEM.KEYPAIR`

Required markers:

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

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
```

Required test:

- `lazy_identity_rng_failure_seam_inactive_without_cfg`

Required marker:

- `NA0465_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Required behavior:

- setting `QSC_RNG_FAILURE_TEST_SEAM=QSC.IDENTITY.LAZY.KEM_KEYPAIR` has no
  effect without cfg;
- normal lazy identity public-record write occurs;
- normal lazy identity KEM secret write occurs;
- normal lazy identity signature secret write occurs;
- normal dependent pending handshake state write occurs;
- normal A1 handshake output occurs;
- no `rng_failure_forced` appears;
- no `identity_secret_unavailable` appears.

## no partial identity-state assertions

The cfg forced-failure tests must assert:

- Alice vault bytes are unchanged after forced failure.
- Alice self public record is absent after forced failure.
- Alice identity KEM secret is absent after forced failure.
- Alice identity signature secret is absent after forced failure.
- Alice pending handshake secret is absent after forced failure.
- Alice legacy pending handshake file is absent after forced failure.
- Alice session blob for Bob is absent after forced failure.
- Bob relay channel is empty after forced failure.

## no identity secret write assertions

The cfg forced-failure tests must decrypt the mock vault and assert:

- `identity.kem_sk.alice` is absent.
- `identity.sig_sk.alice` is absent.

## no public-record write assertions

The cfg forced-failure tests must assert:

- `identities/self_alice.json` is absent.

## no selected identity write assertions

The cfg forced-failure tests must assert:

- no identity public record is written;
- no identity secret is written;
- Alice vault bytes are unchanged;
- output does not contain `identity_fp=`.

## dependent handshake/output absence assertions

The cfg forced-failure tests must assert:

- `handshake.pending.alice.bob` is absent from the mock vault;
- `handshake_pending_alice_bob.json` is absent;
- `qsp_sessions/bob.qsv` is absent;
- output contains no `handshake_send`;
- output contains no `handshake_complete`;
- relay output is empty.

## inherited A2/B1/KEM provider RNG tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required result:

- all pass;
- A2 evidence remains no-output-only background evidence;
- B1 evidence remains background evidence;
- KEM evidence remains background evidence;
- no broader signature, identity, provider-RNG, RNG-failure, or crypto claim is
  introduced.

## inherited RNG residual tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required result:

- all pass;
- previous route/contact/attachment and initial RNG seam behavior remains
  intact.

## inherited provider-error/key-lifecycle tests

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Required result:

- all pass.

## root audit

Run:

```bash
cargo audit --deny warnings
```

Required result:

- PASS.
- Treat as dependency-health evidence only.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required result:

- PASS.
- Treat as dependency-health evidence only.

## qsc adversarial check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run the qsc adversarial smoke script locally and record exact
output. If local cargo-fuzz is unavailable, record exact output and rely on PR
CI qsc-adversarial-smoke.

## formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required result:

- both pass.

## scope guard

Run:

```bash
git diff --name-only origin/main...HEAD
git ls-files --others --exclude-standard
```

Required changed paths are exactly:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/lazy_identity_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## public claim boundary

- No added line may make a public-readiness claim.
- No added line may make a production-readiness claim.
- No added line may make a public-internet-readiness claim.
- No added line may make an external-review-complete claim.
- No added line may make a crypto-complete claim.
- No added line may make a signature-complete claim.
- No added line may make an identity-complete claim.
- No added line may make an RNG-failure-complete claim.
- No added line may make a provider-RNG-complete claim.
- No added line may make a side-channel-free claim.
- No added line may make a vulnerability-free claim.
- No added line may make a bug-free claim.
- No added line may make a perfect-crypto claim.
- No added line may make a secret-material-complete claim.
- No added line may make a metadata-free claim.
- No added line may make an anonymity claim.
- No added line may make an untraceable claim.
- No added line may make a backup-complete claim.
- No added line may make an off-host-backup-complete claim.
- No added line may make a disaster-recovery claim.
- No added line may make a restore-proof claim.

## closeout prerequisites

Closeout to NA-0466 is allowed only after:

- NA-0465 implementation PR is merged;
- public-safety is green on the NA-0465 merge commit;
- D-0917 exists once on main;
- NA-0465 is still READY before closeout;
- selected NA-0466 successor preserves no-runtime, no-crypto, no-dependency,
  no-workflow, and no-public-overclaim boundaries;
- exactly one READY item remains.
