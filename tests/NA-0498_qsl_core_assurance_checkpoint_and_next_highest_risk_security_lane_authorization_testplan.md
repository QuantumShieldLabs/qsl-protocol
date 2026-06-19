Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19

# NA-0498 QSL Core Assurance Checkpoint and Next Highest-Risk Security Lane Authorization Testplan

## Purpose

This testplan records validation for NA-0498, an authorization-only core assurance checkpoint. It verifies that the checkpoint consumed NA-0497/D375 evidence, inventoried direct and residual security evidence, selected exactly one NA-0499 successor, and preserved all no-implementation and no-public-claim boundaries.

## Scope

Allowed mutation paths:

- `docs/governance/evidence/NA-0498_qsl_core_assurance_checkpoint_and_next_highest_risk_security_lane_authorization_plan.md`
- `tests/NA-0498_qsl_core_assurance_checkpoint_and_next_highest_risk_security_lane_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation paths include qsc source/tests/fuzz/Cargo, corpus/vector/input files, qsc-adversarial scripts, validator scripts, workflows, dependencies, lockfiles, formal models, refimpl, services, public docs, backup paths, qsl-backup, and qwork/qstart/qresume/qshell paths.

## Required evidence checks

1. qwork proof-file verification:
   - qwork `.kv` and `.json` proof files exist.
   - Proof lane is `NA-0498`, repo is `qsl-protocol`, startup result is OK, worktree/index/untracked are clean, ready count is 1, queue top is `NA-0498`, and requested lane status is READY.
   - Proof HEAD and proof origin/main match live pre-fetch HEAD and origin/main.

2. Startup repo health:
   - `git status --porcelain=v1 --branch` is clean before fetch.
   - `/` disk usage is below 95%.
   - `origin/main` equals or descends from `ea3c24d99a7e`.
   - current main public-safety is green.
   - qsl-backup SHA and source count match the read-only boundary.

3. Queue and decision gates:
   - READY_COUNT is 1.
   - READY item is `NA-0498`.
   - `NA-0497` and `NA-0496` are DONE.
   - D-0983 exists once as a decision entry.
   - D-0984 exists once as a decision entry.
   - D-0985 is absent before patch and exists once after patch.
   - D-0986 is absent.
   - duplicate decision entry count is zero.

4. Inheritance:
   - D375 response exists and is read.
   - NA-0497 evidence doc and testplan are read.
   - D-0983 and D-0984 are read.
   - Vector consumer test exists and validates the internal manifest as metadata/mapping evidence.
   - Internal-only/no-public-conformance-vector boundary is preserved.

5. Core assurance review:
   - Evidence inventory covers identity/provider RNG, KEM/signature/transcript binding, replay/downgrade/stale/rollback behavior, secret-material lifecycle, qsc/refimpl/formal mapping, fuzz/corpus/vector evidence, external-review readiness, public-claim boundary, CI/local ops, and backup/off-host/restore/key-custody residuals.
   - Highest-risk residual review classifies candidate residuals.
   - Hostile cryptographer, red-team, production SRE, side-channel caveat, formal mapping, external-review, and release-claim reviews are present.
   - Prioritization matrix compares at least the required candidate set.
   - Exactly one primary classification is selected.
   - Selected successor is `NA-0499 -- QSL Side-Channel / Secret-Material Lifecycle Assurance Scope Authorization Plan`.

## Validation commands

Run and capture results under the directive proof root:

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`
- `python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Expected outcome

- D-0985 records NA-0498 core assurance checkpoint and next highest-risk security lane authorization.
- `TRACEABILITY.md` records the authorization-only evidence row.
- Rolling journal records proof root, startup proof, recovered proof-shape issue, validation, selected successor, and no forbidden mutation.
- Changed paths are limited to the five allowed evidence paths.
- No implementation mutation occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No corpus/vector/input mutation occurs.
- No workflow/script/helper mutation occurs.
- No dependency/lockfile mutation occurs.
- No formal/refimpl/service/public/backup mutation occurs.
- No public-readiness claim, production-readiness claim, public-internet-readiness claim, crypto-complete claim, fuzz-complete claim, corpus-complete claim, vector-complete claim, replay-proof claim, downgrade-proof claim, side-channel-free claim, vulnerability-free claim, bug-free claim, or perfect-crypto claim is introduced.

## Post-fix hardening review

Correctness under stress:
The decision is based on current main evidence, qwork proof files, queue/decision proof, inherited NA-0497 evidence, and direct read-only inspection across the required domains.

Minimality:
Only five governance/testplan/journal paths are changed. NA-0499 is not implemented by this evidence PR.

Maintainability:
The evidence doc separates direct evidence, supporting-only evidence, governance-only evidence, process residuals, and open residuals so future lanes can inherit precise boundaries.

Coverage quality:
The validation set includes scope guard, link/leak/overclaim/classifier checks, goal-lint, dependency health, formal checks, vector-manifest JSON, validator scans, and the inherited qsc vector consumer test.

Cross-lane stability:
The checkpoint preserves macOS/Linux expectations by avoiding implementation changes and requiring inherited qsc/fmt/audit/formal checks before PR merge.
