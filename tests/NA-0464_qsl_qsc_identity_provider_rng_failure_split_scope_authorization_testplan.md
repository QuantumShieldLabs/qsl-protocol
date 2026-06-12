Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0464 qsc Identity Provider RNG Failure Split-Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0464 authorization-only lane for qsc identity-provider RNG
failure split scope. The lane must consume NA-0463, inventory identity
provider RNG candidate surfaces, select one exact successor, preserve no
implementation mutation, preserve public-claim caveats, and keep exactly one
READY item.

Selected classification:

`IDENTITY_SPLIT_LAZY_IDENTITY_NEXT`

Selected successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

## Protected invariants

- NA-0464 is authorization-only.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable-test,
  fuzz-target, vector, formal-model, refimpl, qsl-server, qsl-attachments,
  qshield runtime, qshield-cli, website, public-doc, README, START_HERE,
  qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup-status,
  backup-plan, rollback, or backup-tree mutation occurs.
- KEM, B1, and A2 evidence remains bounded background evidence only.
- Lazy identity is selected only as the next bounded implementation target.
- Legacy/public-record upgrade remains residual.
- CLI identity rotation remains residual.
- TUI account bootstrap identity generation remains residual.
- X25519 / ephemeral RNG remains residual.
- refimpl provider RNG remains residual.
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
- Exactly one READY item remains mandatory.

## Allowed scope

Allowed NA-0464 mutation paths:

- `docs/governance/evidence/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_plan.md`
- `tests/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above.

In particular, no mutation is allowed to qsc source, qsc executable test source,
runtime code, crypto code, dependencies, Cargo manifests, lockfiles, workflows,
fuzz target source, vectors, formal models, refimpl, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup
status, backup plan, rollback, or backup tree paths.

## qwork proof checks

Required proof files:

- `/srv/qbuild/work/NA-0464/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0464/.qwork/startup.qsl-protocol.json`

Required:

- `.kv` proof has `startup_result=OK`.
- `.kv` proof has lane NA-0464 and repo qsl-protocol.
- `.kv` proof has clean worktree, index, and untracked values.
- `.kv` proof has READY_COUNT 1 and queue top READY NA-0464.
- JSON proof is valid and mirrors `.kv`.
- Proof HEAD and proof origin/main match live local refs before fetch.
- Codex does not run qwork, qstart, or qresume.

## queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0464 before the evidence PR.
- NA-0463 through NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- Latest decision before patch is D-0914.
- D-0913 exists once.
- D-0914 exists once.
- D-0915 is absent before patch and exists once after patch.
- D-0916 is absent during the evidence PR.
- Duplicate decision count is zero.

## identity inventory checks

Read-only inventory must cover:

- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/tests/`

Required candidate classifications:

- lazy identity key generation:
  `LAZY_IDENTITY_PROVIDER_RNG_IMPLEMENTATION_READY`
- legacy/public-record identity upgrade:
  `LEGACY_IDENTITY_UPGRADE_DEFER_AFTER_LAZY`
- CLI identity rotation:
  `CLI_ROTATE_IDENTITY_DEFER_AFTER_LAZY`
- TUI account bootstrap identity generation:
  `TUI_ACCOUNT_BOOTSTRAP_IDENTITY_DEFER_AFTER_LAZY`

Required primary classification:

`IDENTITY_SPLIT_LAZY_IDENTITY_NEXT`

Required successor:

`NA-0465 -- QSL qsc Lazy Identity Provider RNG Failure Test Seam Implementation Harness`

## scope guard

Run:

```bash
git diff --name-only origin/main...HEAD
git ls-files --others --exclude-standard
```

Required changed paths are exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0464_qsl_qsc_identity_provider_rng_failure_split_scope_authorization_testplan.md`

## markdown link check

Run the repository relative markdown link check.

Required result:

- `TOTAL_MISSING 0`.

## leak scan

Run added-line leak scan for sensitive endpoint, token, auth header, and
long-hex output patterns.

Required result:

- zero findings.

Evidence wording must use short SHAs in prose and descriptive pattern wording
for endpoint or long-hex scans.

## overclaim scan

Run added-line public-claim overclaim scan.

Required result:

- zero affirmative overclaims.

- No added line may claim public readiness.
- No added line may claim production readiness.
- No added line may claim public-internet readiness.
- No added line may claim external review completion.
- No added line may claim crypto completion.
- No added line may claim KEM completion.
- No added line may claim signature completion.
- No added line may claim identity completion.
- No added line may claim RNG-failure completion.
- No added line may claim provider-RNG completion.
- No added line may claim side-channel-free status.
- No added line may claim vulnerability-free status.
- No added line may claim bug-free status.
- No added line may claim perfect-crypto status.
- No added line may claim secret-material completion.
- No added line may claim metadata-free behavior.
- No added line may claim anonymity.
- No added line may claim untraceability.
- No added line may claim backup completion.
- No added line may claim off-host backup completion.
- No added line may claim disaster recovery completion.
- No added line may claim restore proof.

## PR body preflight

Verify PR body includes:

- `Goals: G1, G2, G3, G4, G5`
- Impact
- No-regression
- Tests/Vectors

Verify PR body states:

- qsc identity provider RNG split-scope authorization;
- selected classification;
- selected successor;
- no implementation mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation;
- no public overclaim.

## local validation commands

Run:

```bash
git diff --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
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
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If feasible, run:

```bash
scripts/ci/qsc_adversarial.sh
```

If local `cargo fuzz` is unavailable, record exact output and rely on PR CI
qsc-adversarial-smoke for cargo-fuzz-backed evidence.

## dependency health

Required:

- root cargo audit passes;
- nested qsc fuzz lock cargo audit passes;
- `rustls-webpki` inverse tree resolves;
- `ml-kem` inverse tree resolves;
- pqcrypto inverse probes are recorded as inventory and may return zero matches.

- Cargo audit output is dependency-health evidence only.
- Cargo audit output must not be used as public-readiness proof.
- Cargo audit output must not be used as production-readiness proof.
- Cargo audit output must not be used as public-internet-readiness proof.
- Cargo audit output must not be used as external-review-complete proof.
- Cargo audit output must not be used as crypto-complete proof.
- Cargo audit output must not be used as signature-complete proof.
- Cargo audit output must not be used as identity-complete proof.
- Cargo audit output must not be used as RNG-failure-complete proof.
- Cargo audit output must not be used as provider-RNG-complete proof.
- Cargo audit output must not be used as vulnerability-free proof.
- Cargo audit output must not be used as bug-free proof.
- Cargo audit output must not be used as perfect-crypto proof.
- Cargo audit output must not be used as side-channel-free proof.

## public-safety

Before merge:

- PR checks must be attached and green or accepted skipped/neutral by repo
  policy.
- public-safety must be green before merge.

After merge:

- public-safety must complete success on the evidence merge commit before
  optional closeout begins.

Use REST polling, not watch mode.

## acceptance criteria

- NA-0464 evidence doc exists and records the selected lazy identity successor.
- NA-0464 testplan exists.
- D-0915 exists once.
- TRACEABILITY is updated.
- Rolling journal is updated.
- Changed paths are limited to the five allowed NA-0464 paths.
- No implementation mutation occurs.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- A2/B1/KEM/inherited qsc tests are green.
- refimpl pqkem768 is green.
- formal checks are green.
- qsc adversarial syntax is green.
- local qsc adversarial smoke is run if feasible; otherwise missing local
  cargo-fuzz is recorded and PR CI qsc-adversarial-smoke supplies required
  evidence.
- No backup or restore is run.
- No qsl-backup, status, plan, rollback, or backup tree path is mutated.
- No public overclaim is introduced.
- Exactly one READY item remains.
