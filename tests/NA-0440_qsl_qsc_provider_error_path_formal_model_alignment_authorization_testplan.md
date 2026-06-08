# NA-0440 qsc Provider Error Path Formal / Model Alignment Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0440 classifies qsc provider-error formal/model alignment,
consumes NA-0436/NA-0437/NA-0439 evidence without overclaim, selects exactly one
NA-0441 successor, and changes only the five authorized governance paths.

## Protected invariants

- READY_COUNT remains 1.
- READY remains NA-0440 until optional closeout.
- D-0867 exists once after this lane.
- D-0868 remains absent until optional closeout.
- `pq_decap_failed` evidence remains bounded to the NA-0436 deterministic test
  and NA-0439 adversarial-script integration.
- `pq_encap_failed` remains defensive-branch documentation only.
- No executable coverage claim is made for `pq_encap_failed`.
- Existing formal/model checks remain supporting evidence unless directly
  mapped to implementation semantics.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_plan.md`
- `tests/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_testplan.md`
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
- READY NA-0440.
- NA-0439 DONE.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0865 exists once.
- D-0866 exists once.
- D-0867 exists once after patching.
- D-0868 absent.
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
- `docs/governance/evidence/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_testplan.md`

## Link, leak, classifier, and PR-body checks

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
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- PR body does not contain prohibited public-claim phrases.

## Dependency and regression checks

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
- the NA-0436 `pq_decap_failed` test still passes and emits its markers;
- qsc `send_commit` test still passes;
- provider `pqkem768` test still passes;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes;
- formal model checks pass.

## Local qsc adversarial script validation

Run if feasible:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

Required:

- existing stable Rust adversarial phases pass;
- `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP` appears;
- `handshake_provider_error_no_mutation` runs and passes before cargo-fuzz;
- if local cargo-fuzz is unavailable, record exact output and rely on PR CI
  qsc-adversarial-smoke for cargo-fuzz-backed smoke proof.

## Formal/model classification check

Confirm evidence states:

- `FORMAL_MODEL_SUPPORTING_ONLY` selected.
- `FORMAL_MODEL_ALIGNMENT_EVIDENCE_GAP` documented.
- `FORMAL_MODEL_DIRECT_ALIGNMENT_PRESENT` rejected.
- `FORMAL_MODEL_ALIGNMENT_AMBIGUOUS` rejected.
- primary authorization classification is
  `PROVIDER_ERROR_FORMAL_MODEL_SUPPORTING_ONLY_NO_ACTION`.
- no formal model file mutation is authorized.
- selected NA-0441 successor is
  `NA-0441 -- QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`.

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
- `pq_encap_failed` defensive branch documentation does not claim executable
  coverage;
- `pq_decap_failed` test/adversarial evidence remains bounded to that marker.

## Post-merge checks

After merge, verify:

- READY remains NA-0440 until optional closeout.
- D-0867 exists on main.
- D-0868 remains absent until optional closeout.
- public-safety is green on the evidence merge commit.
- no qwork post-merge command was run by Codex.
