Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18

# NA-0494 QSL Binding Fuzz Corpus Validator CI Integration Authorization Testplan

## Purpose

Validate that NA-0494 is authorization-only and selects the correct future CI
integration scope for the binding fuzz corpus secret-material validator.

## Scope

Allowed changed paths for NA-0494 evidence PR:

- `docs/governance/evidence/NA-0494_qsl_binding_fuzz_corpus_validator_ci_integration_authorization_plan.md`
- `tests/NA-0494_qsl_binding_fuzz_corpus_validator_ci_integration_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No workflow, script, helper, validator, corpus, vector, input, qsc source, qsc
fuzz target, Cargo, lockfile, dependency, formal, refimpl, service, public,
backup, qsl-backup, qwork, qstart, qresume, move, archive, or delete mutation is
allowed in this lane.

## Required startup proof

- qwork proof files exist under `/srv/qbuild/work/NA-0494/.qwork/`.
- `.kv` and `.json` proofs record startup OK, lane NA-0494, repo
  qsl-protocol, clean worktree/index/untracked state, exactly one READY item,
  READY NA-0494, and requested lane status READY.
- Proof HEAD and proof `origin/main` match live refs before fetch.
- `origin/main` equals or descends from `1c4fb02158e2`.
- D-0975 exists once.
- D-0976 exists once.
- D-0977 is absent before mutation.
- Duplicate decision count is zero.
- D364 response file exists.
- Startup public-safety is green.
- Disk usage for `/` is below 95%.

## Required inheritance proof

Verify and record:

- PR #1257 merged at `b5f140e5bd3a`.
- PR #1258 closeout merged at `1c4fb02158e2`.
- D363 pointer-file stop was recovered by D364.
- `ci-4d-evidence` transient `aead` fetch failure was rerun green.
- Checked-in binding corpus exists.
- Binding corpus seed count is exactly seven.
- Every seed is 8 bytes.
- Validator passes the binding corpus.
- Validator passes all qsc fuzz corpus.
- NA-0494 is authorization-only.
- No public-readiness claim is inherited. no crypto-complete claim is inherited.
  no fuzz-complete claim is inherited. no corpus-complete claim is inherited.
  no validator-complete claim beyond bounded internal evidence is inherited.
  no vector-complete claim is inherited. no replay-proof claim is inherited.
  no downgrade-proof claim is inherited. no side-channel-free claim is
  inherited. no vulnerability-free claim is inherited. no bug-free claim is
  inherited. no perfect-crypto claim is inherited.

## Required CI surface inventory

Inspect these surfaces without mutation:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- `scripts/ci/qsc_adversarial.sh`
- `scripts/ci/qsl_evidence_helper.py`
- `scripts/ci/classify_ci_scope.sh`
- `scripts/audit/run_goal_lint_pr.sh`
- `scripts/ci/run_4d.sh`
- `scripts/ci/run_4d_dur.sh`
- `.github/workflows/`
- `qsl/qsl-client/qsc/fuzz/corpus/`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

Expected current classification: `VALIDATOR_LOCAL_ONLY`.

Record for each candidate integration surface:

- path.
- current role.
- whether it already runs the validator.
- mutation type required.
- PR and main behavior.
- artifact/log value.
- first-integration suitability.

## Authorization expectations

Expected primary classification:
`BINDING_FUZZ_VALIDATOR_QSC_ADVERSARIAL_INTEGRATION_READY`.

Expected selected successor:
`NA-0495 -- QSL Binding Fuzz Corpus Validator qsc-Adversarial Integration Implementation Harness`.

Expected future implementation scope:

- mutate `scripts/ci/qsc_adversarial.sh`.
- use the existing validator script unchanged.
- scan `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`.
- scan `qsl/qsl-client/qsc/fuzz/corpus`.
- fail PRs on findings.
- reject missing binding corpus by default.
- use text log output for first CI integration.
- keep JSON proof in local validation/testplan evidence unless a later workflow
  lane authorizes artifact upload.
- make no workflow, helper, dependency, lockfile, qsc source, qsc fuzz target,
  corpus, vector, input, formal, refimpl, service, public, backup, or qwork
  mutation.

## Local validation commands

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/pr/pr_body.md"
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run an exact changed-path scope guard and require only the five allowed evidence
paths.

Run a local overclaim scan over added lines and require zero prohibited public
claim expansions.

Run goal-lint against the opened PR before merge.

## Required PR body metadata

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact: Authorizes the future qsc-adversarial validator integration without implementing it.
No-regression: No workflow/script/helper/corpus/source/Cargo/dependency/lockfile/public/backup mutation occurs in NA-0494.
Tests/Vectors: Governance validation, validator scans, formal checks, qsc/refimpl tests, audits, fmt, scope/link/leak/overclaim/goal-lint checks.
```

The PR body must state authorization-only scope, selected NA-0495 successor, no
implementation mutation, no workflow/script/helper mutation, no corpus/vector
input mutation, no qsc source/fuzz/Cargo mutation, no dependency/lockfile
mutation, and no public overclaim.

## Future NA-0495 required markers

- `NA0495_VALIDATOR_CI_SCOPE_CONSUMED_OK`
- `NA0495_VALIDATOR_QSC_ADVERSARIAL_STEP_INCLUDED_OK`
- `NA0495_VALIDATOR_FAILS_ON_FINDINGS_OK`
- `NA0495_VALIDATOR_SCANS_BINDING_CORPUS_OK`
- `NA0495_VALIDATOR_SCANS_ALL_QSC_CORPUS_OK`
- `NA0495_NO_WORKFLOW_CHANGE_OK`
- `NA0495_NO_DEPENDENCY_CHANGE_OK`
- `NA0495_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0495_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0495_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0495_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0495_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0495_ONE_READY_INVARIANT_OK`

## Acceptance criteria

- NA-0494 evidence doc exists with all required sections.
- D-0977 exists exactly once.
- TRACEABILITY maps NA-0494 to G1-G5.
- Rolling journal records qwork proof, SHAs, queue state, validation notes, disk
  watermark, and next-watch items.
- Changed paths are limited to the five allowed evidence paths.
- `READY_COUNT` remains 1 and READY remains NA-0494 before the optional
  closeout.
- PR checks pass before merge.
- Post-merge public-safety is green before optional closeout.
