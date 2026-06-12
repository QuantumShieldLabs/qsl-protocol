Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0466 qsc Legacy Identity Public-Record Provider RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0466 authorization-only lane for qsc legacy/public-record
identity provider RNG failure scope. The lane must consume NA-0465 lazy identity
evidence, classify legacy/public-record upgrade truthfully, select exactly one
NA-0467 successor, and perform no implementation, runtime, crypto, dependency,
Cargo, lockfile, workflow, executable test, fuzz-target, vector, formal-model,
refimpl, service, backup, public-doc, README, START_HERE, or website mutation.

## Protected invariants

- Exactly one READY item remains.
- NA-0466 is READY before the evidence PR.
- D-0918 exists once before the evidence PR.
- D-0919 is absent before the evidence PR and exists once after the evidence
  patch.
- D-0920 remains absent before optional closeout.
- NA-0465 inheritance remains lazy-identity-only evidence.
- Legacy/public-record upgrade is classified without implementing code.
- CLI identity rotation remains deferred.
- TUI account bootstrap remains deferred.
- X25519 / ephemeral generation remains backlog.
- refimpl provider RNG remains deferred.
- No dependency, Cargo, lockfile, workflow, qsc source, executable qsc test,
  fuzz target, vector, formal model, refimpl, qsl-server, qsl-attachments,
  qshield runtime, qshield-cli, website, public-doc, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup-status, backup-plan,
  rollback, or backup-tree mutation occurs.
- No backup is run.
- No restore is run.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No KEM-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.

## Allowed scope

Allowed mutation paths:

- `docs/governance/evidence/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to runtime source, crypto source, dependencies, Cargo manifests,
lockfiles, workflows, executable test source, fuzz target source, vectors,
formal models, refimpl, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree paths.

## qwork proof-file validation

Codex must not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

```bash
test -f /srv/qbuild/work/NA-0466/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0466/.qwork/startup.qsl-protocol.json
```

Required:

- `.kv` contains `startup_result=OK`.
- `.kv` contains `lane=NA-0466`.
- `.kv` contains `repo=qsl-protocol`.
- `.kv` contains `path=/srv/qbuild/work/NA-0466/qsl-protocol`.
- `.kv` contains clean worktree, index, and untracked state.
- `.kv` contains `ready_count=1`.
- `.kv` contains `queue_top_ready=NA-0466`.
- `.kv` contains `requested_lane_status=READY`.
- JSON is valid and mirrors the `.kv` proof.
- proof HEAD equals live HEAD before fetch.
- proof `origin/main` equals live `origin/main` before fetch.
- fetch does not advance `origin/main` beyond the qwork proof.

Required marker:

- `NA0466_QWORK_PROOF_FILE_VERIFIED_OK`

## Queue and decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- `READY_COUNT 1`.
- `READY NA-0466`.
- NA-0465 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- latest decision before patch is D-0918.
- D-0917 exists once.
- D-0918 exists once.
- D-0919 is absent before patch.
- D-0920 is absent before optional closeout.
- duplicate decision count is zero.

## Inherited validation

Run:

```bash
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
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Required:

- all pass;
- lazy identity evidence remains bounded background only;
- A2 evidence remains no-output-only background evidence;
- B1 evidence remains background evidence;
- KEM evidence remains background evidence;
- route/contact/attachment and base RNG failure evidence remains background
  evidence;
- no broader signature, identity, provider-RNG, RNG-failure, or crypto claim is
  introduced.

## Legacy/public-record classification checks

Required evidence:

- legacy migration path is in `qsl/qsl-client/qsc/src/identity/mod.rs` and
  calls `hs_sig_keypair()`;
- public-record empty-`sig_pk` upgrade path is in
  `qsl/qsl-client/qsc/src/identity/mod.rs` and calls `hs_sig_keypair()`;
- selected path is RNG-bearing;
- selected path starts from existing identity/public-record state;
- selected path needs separate future labels from lazy identity;
- future no-partial-upgrade invariant preserves existing KEM state and public
  record while writing no new signature secret or upgraded public record;
- future implementation can be bounded to `identity/mod.rs` plus one qsc test
  file;
- CLI rotation and TUI bootstrap remain deferred;
- refimpl remains deferred.

Required markers:

- `NA0466_LEGACY_PUBLIC_RECORD_TARGET_INVENTORY_OK`
- `NA0466_LEGACY_PUBLIC_RECORD_PROVIDER_RNG_RELEVANT_OK`
- `NA0466_LEGACY_PUBLIC_RECORD_IMPLEMENTATION_READY_OK`

## Future successor proof

Required selected successor:

`NA-0467 -- QSL qsc Legacy Identity Public-Record Provider RNG Failure Test Seam Implementation Harness`

Required future paths:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/legacy_identity_public_record_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Required future labels:

- `QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR`
- `QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR`

Required marker:

- `NA0466_SUCCESSOR_NA0467_SELECTED_OK`

## Dependency health

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Required:

- root audit PASS;
- nested qsc fuzz lock audit PASS;
- `rustls-webpki` inverse tree present;
- `ml-kem` inverse tree present or recorded truthfully;
- pqcrypto inverse probes recorded truthfully as zero-match if absent;
- cargo audit green is dependency-health evidence only.

## Scope guard

Run a name-only diff against `origin/main` and verify changed paths are exactly:

```text
DECISIONS.md
TRACEABILITY.md
docs/governance/evidence/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_plan.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0466_qsl_qsc_legacy_identity_public_record_provider_rng_failure_scope_authorization_testplan.md
```

Required:

- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal/
  refimpl/service/public/backup paths changed.

## Link, leak, and overclaim checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Required:

- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.
- added-line overclaim scan has no affirmative public/security overclaim.

## Formatting and static checks

Run:

```bash
git diff --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- all pass.

## Additional Rust validation

Run:

```bash
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required:

- both pass.

## qsc adversarial smoke

Run if feasible without environment drift:

```bash
scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record the exact output and rely on PR CI
qsc-adversarial-smoke.

## PR body preflight and goal-lint

Required PR body fields:

- `Goals:`
- `Impact:`
- `No-regression:`
- `Tests/Vectors:`

The PR body must mention:

- qsc legacy/public-record identity provider RNG scope authorization;
- selected classification;
- selected successor;
- no implementation mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation;
- no public overclaim;
- no identity-complete claim.

Run local PR body preflight and goal-lint with a synthetic pull-request event.

## Public-safety

Before merge:

- required PR checks must pass;
- public-safety must be green or accepted by repo policy.

After merge:

- public-safety must be green on the merge commit.

## Closeout boundary

This evidence PR does not close NA-0466. Optional closeout may run only after
this PR merges and post-merge public-safety is green. Optional closeout must
restore NA-0467 as READY and must not implement NA-0467.
