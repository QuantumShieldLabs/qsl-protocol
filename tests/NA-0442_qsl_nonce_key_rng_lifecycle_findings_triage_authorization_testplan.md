# NA-0442 QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0442 consumes NA-0441 nonce/key/RNG lifecycle findings,
selects exactly one NA-0443 successor, and preserves no implementation mutation
or public overclaim.

## Protected invariants

- READY_COUNT remains 1.
- NA-0442 is READY until optional closeout.
- D-0871 exists once after the evidence patch.
- D-0872 remains absent until optional closeout.
- NA-0441 findings F-0441-01 through F-0441-06 are consumed.
- F-0441-02 and F-0441-03 are triaged explicitly.
- Selected primary classification is
  `NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`.
- Selected successor is
  `NA-0443 -- QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan`.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_plan.md`
- `tests/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, and public technical paper work.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0442.
- NA-0441 DONE.
- NA-0440 DONE.
- NA-0439 DONE.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0869 exists once.
- D-0870 exists once.
- D-0871 exists once after patching.
- D-0872 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_testplan.md`

## Link, leak, classifier, PR-body, and goal checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
```

Required:

- no whitespace errors;
- link check passes;
- added-line leak scan has zero findings;
- added-line overclaim scan has zero affirmative findings;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- goal-lint passes.

## Dependency, provider-error, and formal checks

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- adversarial script syntax checks pass;
- inherited provider-error no-mutation test still passes and emits NA-0436
  markers;
- qsc `send_commit` passes;
- provider `pqkem768` passes;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes;
- formal checks pass.

If local qsc adversarial execution reaches a local `cargo fuzz` availability
limit after pre-fuzz phases pass, record the exact output and rely on PR CI
qsc-adversarial-smoke as cargo-fuzz-backed proof.

## Findings triage checks

Confirm:

- all F-0441 findings are restated and normalized;
- F-0441-02 disposition is IMMEDIATE_SUCCESSOR;
- F-0441-03 disposition is NEXT_CANDIDATE;
- F-0441-01 disposition is BACKLOG;
- F-0441-04 is supporting-only / next-domain background;
- F-0441-05 is a coverage watch item;
- F-0441-06 remains claim-boundary/demo-local;
- selected primary classification is
  `NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`;
- selected successor is key lifecycle secret cleanup / zeroization scope
  authorization;
- no implementation mutation is authorized.

## Public claim boundary

Confirm:

- no production-readiness claim is introduced;
- no public-internet-readiness claim is introduced;
- no external-review-complete claim is introduced;
- no crypto-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed;
- cargo audit green is dependency-health evidence only;
- model checks passing remains bounded evidence, not full correctness proof;
- qshield demo evidence is not represented as service readiness;
- secret-material lifecycle gaps are called gaps, not completions.

## Post-merge checks

After merge, verify:

- READY remains NA-0442 before optional closeout.
- D-0871 exists on main.
- public-safety is green on the evidence merge commit.
- no qwork post-merge command was run by Codex.
