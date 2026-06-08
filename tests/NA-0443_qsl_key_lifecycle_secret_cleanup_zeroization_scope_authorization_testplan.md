# NA-0443 QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0443 consumes NA-0441/NA-0442 findings, classifies qsc,
refimpl, and qshield-cli secret-material surfaces separately, selects exactly
one NA-0444 evidence-policy successor, and preserves no implementation mutation
or public overclaim.

## Protected invariants

- READY_COUNT remains 1.
- NA-0443 is READY until optional closeout.
- D-0873 exists once after the evidence patch.
- D-0874 remains absent until optional closeout.
- NA-0441 and NA-0442 findings are consumed.
- F-0441-02 is selected as the current lane focus.
- F-0441-03 RNG failure behavior remains next candidate.
- F-0441-06 qshield-cli demo-local material remains claim-boundary/backlog.
- Selected primary classification is
  `KEY_LIFECYCLE_SECRET_CLEANUP_EVIDENCE_POLICY_NEXT`.
- Selected successor is
  `NA-0444 -- QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_plan.md`
- `tests/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_testplan.md`
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
workflow mutation, branch-protection mutation, public technical paper work, and
NA-0444 implementation.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0443.
- NA-0442 DONE.
- NA-0441 DONE.
- NA-0440 DONE.
- NA-0439 DONE.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0871 exists once.
- D-0872 exists once.
- D-0873 exists once after patching.
- D-0874 absent.
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
- `docs/governance/evidence/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_testplan.md`

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
- inherited provider-error no-mutation test passes and emits NA-0436 markers;
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

## Scope classification checks

Confirm:

- qsc runtime classification is
  `QSC_SECRET_CLEANUP_EVIDENCE_POLICY_NEEDED`;
- refimpl classification is
  `REFIMPL_SECRET_CLEANUP_SCOPE_INCLUDED` for evidence policy only;
- qshield-cli classification is
  `QSHIELD_DEMO_KEY_MATERIAL_CLAIM_BOUNDARY_ONLY`;
- primary classification is
  `KEY_LIFECYCLE_SECRET_CLEANUP_EVIDENCE_POLICY_NEXT`;
- selected successor is NA-0444 evidence-policy authorization;
- no implementation mutation is authorized.

## Public claim boundary

Confirm:

- no production-readiness claim is introduced;
- no public-internet-readiness claim is introduced;
- no external-review-complete claim is introduced;
- no crypto-complete claim is introduced;
- no secret-material-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed;
- cargo audit green is dependency-health evidence only;
- model checks passing remains bounded evidence, not full correctness proof;
- qshield-cli demo evidence is not represented as service readiness;
- secret-material lifecycle gaps are called gaps, not completions.

## Post-merge checks

After merge, verify:

- READY remains NA-0443 before optional closeout.
- D-0873 exists on main.
- public-safety is green on the evidence merge commit.
- no qwork post-merge command was run by Codex.
