Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0501 qsc Key Lifecycle / Zeroization Expansion Scope Authorization Testplan

## Objective

Verify that NA-0501 is authorization-only, consumes NA-0500/D384/PR #1273 inheritance, inventories current qsc key lifecycle / zeroization evidence, compares successor options including same-host client-to-client E2E, and selects exactly one NA-0502 successor without implementation mutation.

## Scope under test

Allowed NA-0501 changed paths:

- `docs/governance/evidence/NA-0501_qsl_qsc_key_lifecycle_zeroization_expansion_scope_authorization_plan.md`
- `tests/NA-0501_qsl_qsc_key_lifecycle_zeroization_expansion_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden NA-0501 changes:

- qsc source, qsc tests, qsc fuzz targets, qsc Cargo files, root Cargo files, dependencies, lockfiles, workflows, scripts, helpers, corpus/vector/input files, formal models, refimpl files, service paths, public docs, backup paths, qsl-backup, qwork, qstart, or qresume.

## Required checks

1. qwork proof-file verification:
   - `.kv` and `.json` startup proof files exist and agree.
   - proof HEAD and proof origin/main matched live refs before fetch.
   - queue top was READY NA-0501.
   - Codex did not run `qwork`, `qstart`, or `qresume`.

2. Inheritance verification:
   - NA-0500 is DONE.
   - NA-0501 is READY.
   - D-0989 and D-0990 exist once.
   - D-0991 was absent before patch and exists once after patch.
   - D384 response exists.
   - PR #1273 merged and changed only `NEXT_ACTIONS.md`.

3. Evidence inventory verification:
   - qsc identity KEM/signing secret lifecycle reviewed.
   - qsc KEM secret-key and shared-secret lifecycle reviewed.
   - transcript, confirm-key, pending-session material reviewed.
   - X25519 / ephemeral secret residual reviewed.
   - pending-handshake reject cleanup reviewed.
   - session-store and temp-root artifact behavior reviewed.
   - provider-error no-mutation, NA-0500 diagnostic/no-output, corpus/vector validator, refimpl, and formal bounded evidence reviewed.

4. Option review verification:
   - at least eight candidate options are evaluated.
   - same-host client-to-client E2E is considered and either selected or deferred with evidence-based rationale.
   - process/tooling lane is selected only if an active blocker exists.

5. Decision verification:
   - primary classification is recorded.
   - selected successor is exact.
   - future allowed and forbidden paths are recorded.
   - future markers and validation commands are recorded.
   - exactly one READY remains mandatory.

## Selected authorization result

Expected primary classification:

`KEY_LIFECYCLE_ZEROIZATION_EXPANSION_TEST_READY`

Expected successor:

`NA-0502 -- QSL qsc Key Lifecycle Zeroization Expansion Test Implementation Harness`

Expected future implementation path:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`

## Validation commands

Run before the NA-0501 PR:

```bash
git diff --check
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run repository-specific scope, link, leak, overclaim, classifier, PR body, and goal-lint checks.

## Pass criteria

- Changed paths are exactly within the five allowed NA-0501 evidence paths.
- No implementation mutation is present.
- No qsc source/test/fuzz/Cargo mutation is present.
- No corpus/vector/input mutation is present.
- No workflow/script/helper/dependency/lockfile mutation is present.
- No formal/refimpl/service/public/backup mutation is present.
- D-0991 exists exactly once.
- The closeout decision ID remains unallocated before optional closeout.
- READY_COUNT remains 1 and READY remains NA-0501 before closeout.
- Local validation commands pass.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no secret-material-complete claim is introduced.
- no zeroization-complete claim is introduced.
- no memory-erasure-complete claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Post-fix hardening review checklist

- Correctness under stress: the selected successor must remain testable without source/dependency/workflow mutation.
- Minimality: NA-0501 must remain governance-only and touch only allowed files.
- Maintainability: the future NA-0502 test should be a new bounded integration test file unless implementation evidence proves a narrower existing-test extension is safer.
- Coverage quality: the authorization must not treat marker scans, source-level `zeroize` calls, or bounded formal models as complete secret-material or memory-erasure proof.
- Cross-lane stability: macOS/Linux CI expectations and qsc-adversarial boundaries must remain unchanged.

## Stop conditions

Stop if qwork proof is missing or stale, qwork is run by Codex, queue is not READY NA-0501 at startup, D-0990 is absent, D-0991 already exists at startup, `/` usage is at or above 95 percent, any forbidden mutation is attempted, root or nested audit is red, inherited qsc tests fail, validator scans fail, qsl-backup source-list proof regresses, more than one READY appears, public-safety is red or missing where required, or any forbidden public/completion/free/perfect claim is introduced.
