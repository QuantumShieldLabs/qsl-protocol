Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0491 Closeout and NA-0492 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate closeout-only governance for NA-0491 after PR #1253 post-merge
public-safety success, then restore NA-0492 as the sole READY successor without
implementation, validator, corpus, vector, input, qsc, Cargo, workflow,
dependency, formal, refimpl, service, public, backup, or restore mutation.

## Protected invariants

- qwork proof files are read, not regenerated.
- Codex does not run qwork, qstart, qresume, or qshell.
- PR #1253 is merged at `f03f897c6681`.
- PR #1253 post-merge public-safety is completed success before closeout
  mutation.
- PR #1253 qsc-adversarial-smoke is completed success.
- PR #1253 qsc-linux-full-suite and macos-qsc-full-serial are completed success
  or accepted only under explicit repository policy.
- D-0971 exists once before closeout.
- D-0972 is absent before closeout and exists once after closeout.
- D-0973 remains absent.
- exactly one READY item exists.
- NA-0491 is DONE before NA-0492 is READY.
- no implementation mutation in closeout.
- no validator script mutation in closeout.
- no corpus/vector/input mutation in closeout.
- no qsc source/fuzz target/Cargo/script/workflow mutation in closeout.
- no dependency/lockfile/formal/refimpl/service/public/backup mutation in
  closeout.
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no validator-complete claim beyond bounded NA-0491
  implementation evidence, no vector-complete claim, no replay-proof claim, no
  downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, no bug-free claim, and no perfect-crypto claim is introduced.

## Allowed closeout scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Required evidence

Run and record:

```bash
gh pr view 1253 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergeCommit,mergedAt,title,url,statusCheckRollup
gh api /repos/QuantumShieldLabs/qsl-protocol/commits/f03f897c6681cbf2afecba6d33afd44a7545091e/check-runs?per_page=100
gh pr diff 1253 --repo QuantumShieldLabs/qsl-protocol --name-only
```

Required:

- PR #1253 state is `MERGED`.
- PR #1253 merge commit begins with `f03f897c6681`.
- PR #1253 changed exactly the six NA-0491 implementation/governance paths.
- public-safety on `f03f897c6681` is completed success.
- qsc-adversarial-smoke on `f03f897c6681` is completed success.
- no required check completed failure.

## Closeout validation

Run after the closeout patch:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed-file <allowed> --forbidden-file <forbidden>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
```

Required:

- exact five-path closeout scope.
- link-check PASS.
- leak-scan PASS.
- added-line overclaim scan has zero affirmative findings.
- classifier reports docs/governance scope only.
- PR body preflight PASS.
- goal-lint PASS.

## Inherited validation

Run or inherit from pre-closeout validation:

```bash
cargo fmt --check
python3 -m py_compile scripts/audit/validate_binding_fuzz_corpus_no_secrets.py
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --allow-missing --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 formal/run_model_checks.py
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- all required inherited commands pass.
- cargo audit green is dependency-health evidence only.
- no checked-in corpus or vector is added.
- no source, fuzz target, Cargo, lockfile, workflow, formal, refimpl, service,
  public, backup, restore, or qsl-backup path is mutated.

## Post-merge validation

After the closeout PR merges:

- verify `main` equals `origin/main`.
- verify READY_COUNT 1.
- verify READY NA-0492.
- verify NA-0491 DONE.
- verify D-0972 once.
- verify D-0973 absent.
- verify closeout merge commit public-safety success.
- verify qsc-adversarial-smoke success or accepted skipped by repository policy
  for closeout.

## Post-fix hardening review

Report:

- correctness under stress.
- minimality.
- maintainability.
- coverage quality.
- cross-lane stability for macOS/Linux affected checks.
