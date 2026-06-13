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

## D328 Assurance Addendum Recovery Checks

The D329 governance recovery must verify and preserve these assurance sections
in the NA-0469 evidence trail without mutating runtime, source, executable test,
dependency, workflow, Cargo, lockfile, fuzz, vector, formal, refimpl,
qsl-server, qsl-attachments, qshield, qshield-cli, website, or public
documentation paths.

### Best-known-method review

Required:

- classification `BEST_KNOWN_METHOD_FOR_SCOPE`;
- cfg-only seam recorded as least invasive for NA-0469;
- no dependency, provider trait, workflow, or runtime change needed;
- production semantics unchanged when cfg absent;
- bounded CLI rotation evidence only;
- stronger evidence needed for public or production claims.

### Hostile cryptographer review

Required:

- top three concerns recorded;
- each concern classified as in-scope, out-of-scope, or future evidence;
- concerns include risk of overclaiming bounded identity evidence as complete
  coverage, qsc/refimpl provider boundary risk, and transcript,
  identity-binding, and formal mapping residual.

### Red-team review

Required:

- top three concerns recorded;
- concerns include rotation failure under active attacker or stale public
  record, relay/server observation of partial identity changes, and
  rollback/replay/peer-reset confusion.

### Production SRE review

Required:

- top three concerns recorded;
- concerns include operational incident from identity rotation failure,
  logs/diagnostics and user confusion, and missing rollback/recovery playbook
  before release claims.

### Side-channel caveat

Required same-line denial wording:

- No side-channel-free claim. No constant-time proof. No memory-erasure completeness proof. No all secret-material lifecycle proof.

### Formal-model mapping residual

Required:

- existing formal models are supporting evidence only;
- CLI identity rotation provider RNG failure is not directly modeled unless an
  exact model mapping is found;
- absent exact mapping, classification `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.

### External-review readiness

Required:

- classification `EXTERNAL_REVIEW_READINESS_INCREMENTAL`;
- not external-review-complete;
- future package needs protocol spec, threat model, positive test vectors,
  negative vectors, state-machine mapping, and claim boundaries.

### Release-claim boundary

Required same-line denial wording:

- No public-readiness claim. No production-readiness claim. No crypto-complete claim. No identity-complete claim. No provider-RNG-complete claim. No RNG-failure-complete claim. No side-channel-free claim. No vulnerability-free claim. No perfect-crypto claim. Cargo audit is dependency-health evidence only.

### Assurance Gap Review trigger

Required:

- classification `ASSURANCE_GAP_REVIEW_REQUIRED_AFTER_CURRENT_CHAIN`;
- NA-0470 remains a high-priority direct residual;
- after NA-0470 closes, default next lane should be an Assurance Gap Review
  unless NA-0470 proves a higher-priority residual;
- answer the question: "What would a hostile cryptographer, a red-team
  engineer, and a production SRE attack next?"

### Successor-selection assurance check

Required:

- NA-0470 remains READY and is not executed by the recovery;
- NA-0470 consumes D-0927 before execution;
- exactly one READY item remains;
- D-0927 exists once;
- D-0928 remains absent.

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
