Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19

# NA-0499 QSL Side-Channel / Secret-Material Lifecycle Assurance Scope Authorization Testplan

## Purpose

This testplan records validation for NA-0499, an authorization-only core assurance lane. It verifies that the lane consumed NA-0498/D377 inheritance, inventoried qsc/refimpl/formal secret-material lifecycle surfaces, classified existing evidence, selected exactly one NA-0500 successor, and preserved all no-implementation and no-public-claim boundaries.

## Scope

Allowed mutation paths:

- `docs/governance/evidence/NA-0499_qsl_side_channel_secret_material_lifecycle_assurance_scope_authorization_plan.md`
- `tests/NA-0499_qsl_side_channel_secret_material_lifecycle_assurance_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation paths include qsc source/tests/fuzz/Cargo, corpus/vector/input files, qsc-adversarial scripts, validator scripts, workflows, dependencies, lockfiles, formal models, refimpl, services, public docs, backup paths, qsl-backup, and qwork/qstart/qresume/qshell paths.

## Required evidence checks

1. qwork proof-file verification:
   - qwork `.kv` and `.json` proof files exist.
   - Proof lane is `NA-0499`, repo is `qsl-protocol`, startup result is OK, worktree/index/untracked are clean, ready count is 1, queue top is `NA-0499`, and requested lane status is READY.
   - Proof HEAD and proof origin/main match live pre-fetch HEAD and origin/main.

2. Startup repo health:
   - `git status --porcelain=v1 --branch` is clean before fetch.
   - `/` disk usage is below 95%.
   - `origin/main` equals or descends from `89a50cfa5ecb`.
   - current main public-safety is green.
   - qsl-backup SHA and source count match the read-only boundary.

3. Queue and decision gates:
   - READY_COUNT is 1.
   - READY item is `NA-0499`.
   - `NA-0498`, `NA-0497`, and `NA-0496` are DONE.
   - D-0985 exists once as a decision entry.
   - D-0986 exists once as a decision entry.
   - D-0987 is absent before patch and exists once after patch.
   - D-0988 is absent.
   - duplicate decision entry count is zero.

4. Inheritance:
   - D377 response exists and is read.
   - NA-0498 evidence doc and testplan are read.
   - D-0985 and D-0986 are read.
   - NA-0499 queue block is read.
   - Inherited direct evidence includes key lifecycle zeroization, provider-error no-mutation, qsc binding negative tests, vector consumer, corpus validator, and qsc-adversarial validator integration.
   - Inherited supporting-only evidence includes bounded formal model, refimpl provider-boundary evidence, and governance/no-claim evidence.

5. Core assurance review:
   - Secret-material lifecycle inventory covers qsc identity/key lifecycle, provider-error/no-mutation/no-output evidence, KEM/signature/transcript binding reject paths, CLI/TUI/stdout/stderr/log diagnostics, TUI bootstrap pre-generation caveats, stored identity/public-record/trusted-pin/session/temp-root artifacts, refimpl KEM/signature boundaries, corpus/vector/validator boundaries, formal model limitations, qsc/refimpl/formal mapping limitations, backup/off-host/restore/key-custody residuals, and external-review readiness boundaries.
   - Existing evidence is classified into direct zeroization, direct no-output, direct no-mutation, direct validator, direct qsc reject, direct refimpl provider-boundary, formal bounded, governance-only, supporting-only, residual-open, and not-covered categories.
   - Option review covers the eight candidate successor lanes.
   - Hostile cryptographer, red-team, production SRE, and release-claim reviews are present.
   - Prioritization matrix compares at least the required candidate set.
   - Exactly one primary classification is selected.
   - Selected successor is `NA-0500 -- QSL qsc Secret-Material Diagnostic / No-Output Boundary Test Implementation Harness`.

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
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Expected outcome

- D-0987 records NA-0499 side-channel and secret-material lifecycle assurance scope authorization.
- `TRACEABILITY.md` records the authorization-only evidence row.
- Rolling journal records proof root, startup proof, inheritance, inventory, selected successor, validation, and no forbidden mutation.
- Changed paths are limited to the five allowed evidence paths.
- No implementation mutation occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No corpus/vector/input mutation occurs.
- No workflow/script/helper mutation occurs.
- No dependency/lockfile mutation occurs.
- No formal/refimpl/service/public/backup mutation occurs.
- No public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no KEM-complete claim, no signature-complete claim, no identity-complete claim, no provider-RNG-complete claim, no secret-material-complete claim, no zeroization-complete claim, no memory-erasure-complete claim, no side-channel-free claim, no replay-proof claim, no downgrade-proof claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim is introduced.

## Post-fix hardening review

Correctness under stress:
The decision is based on current main evidence, qwork proof files, queue/decision proof, inherited NA-0498/D377 evidence, and read-only inspection across qsc/refimpl/formal/corpus/diagnostic/storage/backup boundaries.

Minimality:
Only five governance/testplan/journal paths are changed. NA-0499 does not implement NA-0500.

Maintainability:
The evidence doc separates lifecycle inventory, evidence classification, option review, hostile cryptographer/red-team/SRE review, prioritization, and future NA-0500 markers so the next implementation lane has exact scope.

Coverage quality:
The validation set includes scope guard, link/leak/overclaim/classifier checks, goal-lint, dependency health, formal checks, validator scans, qsc vector consumer, qsc key lifecycle zeroization, and qsc provider-error no-mutation.

Cross-lane stability:
The authorization preserves macOS/Linux expectations by avoiding implementation changes and requiring inherited qsc/fmt/audit/formal checks before PR merge.
