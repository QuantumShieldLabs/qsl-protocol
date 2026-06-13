Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0468 qsc CLI Identity Rotation Provider RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded NA-0468 authorization lane. The lane must consume NA-0467,
classify qsc CLI identity rotation provider RNG failure scope, select one
truthful NA-0469 successor, record exact future implementation paths if
implementation-ready, avoid implementation mutation, and avoid public overclaim.

## Protected invariants

- Exactly one READY item remains before implementation closeout.
- D-0922 exists once before patch.
- D-0923 is absent before patch and exists once after patch.
- D-0924 remains absent before optional closeout.
- NA-0467 legacy/public-record evidence remains bounded background evidence.
- CLI identity rotation is authorization-only in NA-0468.
- No runtime code is mutated.
- No crypto code is mutated.
- No dependency, Cargo manifest, lockfile, workflow, executable test, fuzz
  target, vector, formal model, refimpl, qsl-server, qsl-attachments, qshield
  runtime, qshield-cli, website, public-doc, README, START_HERE,
  qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
  backup plan, rollback, or backup tree mutation occurs.
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

## Allowed scope

- `docs/governance/evidence/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to qsc source, runtime behavior, crypto behavior, dependencies, Cargo
manifests, lockfiles, workflows, executable implementation tests, fuzz target
source, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
backup plan, rollback, or backup tree paths.

## qwork proof verification

Read, but do not regenerate, the qwork proof files:

```text
/srv/qbuild/work/NA-0468/.qwork/startup.qsl-protocol.kv
/srv/qbuild/work/NA-0468/.qwork/startup.qsl-protocol.json
```

Required:

- startup result is OK.
- lane is NA-0468.
- repo is qsl-protocol.
- proof path is `/srv/qbuild/work/NA-0468/qsl-protocol`.
- proof HEAD equals live HEAD before fetch.
- proof origin/main equals live origin/main before fetch.
- clean-state fields are all yes.
- READY_COUNT is 1.
- queue top READY is NA-0468.
- requested lane status is READY.
- JSON mirrors the `.kv` proof for the required fields.

## PR #1204 merge proof

Run:

```bash
gh pr view 1204 --repo QuantumShieldLabs/qsl-protocol \
  --json number,state,mergedAt,mergeCommit,headRefOid,title,url,statusCheckRollup
```

Required:

- state is MERGED.
- merge commit begins with `a557440e8ab2`.

## Queue proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0468`.
- `NA-0467 DONE`.
- NA-0466 through NA-0435 remain DONE.
- NA-0434 and NA-0429 remain BLOCKED.

## Decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- latest decision is D-0923 after patch.
- D-0921 exists once.
- D-0922 exists once.
- D-0923 exists once after patch.
- D-0924 is absent.
- duplicate decision count is zero.

## Source inventory checks

Read-only inspect:

```bash
rg -n "IdentityCmd::Rotate|fn identity_rotate|hs_kem_keypair|hs_sig_keypair|identity_secret_store|identity_sig_secret_store|identity_write_public_record|reset_peers" qsl/qsl-client/qsc/src/main.rs qsl/qsl-client/qsc/src/identity/mod.rs
```

Required classification evidence:

- CLI `identity rotate` dispatch reaches `identity_rotate`.
- `identity_rotate` requires unlocked state.
- confirmed rotation calls KEM keypair generation.
- confirmed rotation calls signature keypair generation.
- KEM and signature keypair generation occur before identity secret stores.
- KEM and signature keypair generation occur before self public-record write.
- KEM and signature keypair generation occur before optional peer reset state
  mutation.
- future forced provider RNG failure is testable before durable rotation writes.

## Scope guard

Run a name-only diff against `origin/main` and include untracked files.

Required changed paths are exactly:

```text
DECISIONS.md
TRACEABILITY.md
docs/governance/evidence/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_plan.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0468_qsl_qsc_cli_identity_rotation_provider_rng_failure_scope_authorization_testplan.md
```

No other tracked or untracked repo path may be changed.

## Required governance validation

Run:

```bash
git diff --check
```

Run the manual local-link existence check from `AGENTS.md`.

Run added-line leak and overclaim scans. Required:

- no secret/token/auth/header/long-hex evidence leak.
- no affirmative public-readiness claim.
- no affirmative production-readiness claim.
- no affirmative external-review-complete claim.
- no affirmative crypto-complete claim.
- no affirmative signature-complete claim.
- no affirmative identity-complete claim.
- no affirmative RNG-failure-complete claim.
- no affirmative provider-RNG-complete claim.
- no affirmative side-channel-free claim.
- no affirmative vulnerability-free claim.
- no affirmative bug-free claim.
- no affirmative perfect-crypto claim.

Run PR body preflight and goal-lint. Required:

- PR body has `Goals:` near the top.
- PR body has Impact, No-regression, and Tests/Vectors fields.
- goal-lint detects the Goals line.

Run classifier if available. Required:

- classified as governance/docs-only or equivalent safe scope.

## inherited qsc provider RNG tests

Run cfg/no-cfg:

```bash
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
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required: PASS. These are inherited background checks and do not imply identity,
signature, RNG-failure, provider-RNG, or crypto completion.

## inherited provider-error/key-lifecycle tests

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Required: PASS.

## refimpl, dependency, format, and formal checks

Run:

```bash
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- refimpl pqkem768 PASS.
- root audit PASS.
- nested qsc fuzz lock audit PASS.
- cargo fmt PASS.
- formal model checks PASS.
- `pqcrypto-*` inverse probes may be expected zero-match inventory results.

## qsc adversarial script checks

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If local qsc adversarial smoke is feasible, run:

```bash
scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
qsc-adversarial-smoke.

## public-safety

Before merge, required PR checks must pass.

After merge, run:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha <merge-sha>
```

Required:

- `public-safety` completed success.
- no required attached check completed failure.
- no public-safety red or ambiguous result exists.

## Future NA-0469 markers

If NA-0469 implementation is restored, future implementation should emit:

- `NA0469_CLI_ROTATION_SCOPE_CONSUMED_OK`
- `NA0469_NEXT_SCOPE_SELECTED_OK`
- `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_FORCED_BY_TEST_ONLY_BOUNDARY_OK`
- `NA0469_CLI_ROTATE_PROVIDER_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK`
- `NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK`
- `NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK`
- `NA0469_CLI_ROTATE_NO_PARTIAL_SECRET_WRITE_OK`
- `NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0469_NO_DEPENDENCY_CHANGE_OK`
- `NA0469_NO_WORKFLOW_CHANGE_OK`
- `NA0469_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0469_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0469_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0469_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0469_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0469_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0469_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

Before declaring complete, report:

1. Correctness under stress.
2. Minimality, including exact five-path NA-0468 scope guard.
3. Maintainability of the selected future implementation shape.
4. Coverage quality, including why future tests can fail for the right reasons.
5. Cross-lane stability, including macOS/Linux consistency for affected areas.
