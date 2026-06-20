Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0503 qsc Same-Host Client-to-Client E2E Scope Authorization Testplan

## Purpose

Verify that NA-0503 is authorization-only, consumes NA-0502/D388 inheritance,
inventories current qsc client-to-client/E2E surfaces, compares successor
options, completes stewardship reviews, selects exactly one NA-0504 successor,
and preserves all no-implementation and no-overclaim boundaries.

## Scope under test

Allowed NA-0503 evidence paths:

- `docs/governance/evidence/NA-0503_qsl_qsc_same_host_client_to_client_e2e_scope_authorization_plan.md`
- `tests/NA-0503_qsl_qsc_same_host_client_to_client_e2e_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden in NA-0503:

- qsc source/test/fuzz/Cargo mutation.
- implementation mutation.
- helper/script/workflow mutation.
- dependency/lockfile mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- remote SSH or two-machine setup.
- any public/security/completion claim forbidden by the NA-0503 evidence doc.

## Required checks

1. qwork proof-file verification
   - Proof files exist under `/srv/qbuild/work/NA-0503/.qwork/`.
   - Proof `.kv` and `.json` agree on lane, repo, path, clean worktree,
     `READY_COUNT=1`, READY NA-0503, and requested lane status READY.
   - Proof HEAD and `origin_main` match live pre-fetch refs.
   - Codex did not run `qwork`, `qstart`, or `qresume`.

2. Startup queue and decision checks
   - READY count is 1.
   - READY item is NA-0503.
   - NA-0502, NA-0501, and NA-0500 are DONE.
   - D-0993 exists once.
   - D-0994 exists once.
   - D-0995 is absent before patch and exists once after patch.
   - D-0996 is absent before closeout.
   - Duplicate decision count is zero.

3. Inheritance checks
   - D388 response exists.
   - NA-0502 implementation evidence and testplan are consumed.
   - `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`
     exists and is treated as inherited implementation evidence only.
   - NA-0502 selected same-host client-to-client E2E as the next broad
     user-realistic assurance lane.

4. Surface inventory checks
   - qsc CLI/process invocation patterns are inventoried.
   - identity/bootstrap tests are inventoried.
   - send/receive/handshake tests are inventoried.
   - binding negative tests are inventoried.
   - provider-error/no-mutation tests are inventoried.
   - key lifecycle / zeroization tests are inventoried.
   - diagnostic/no-output tests are inventoried.
   - temp-root/TestIsolation patterns are inventoried.
   - mock relay/local inbox patterns are inventoried.
   - public-record/trusted-pin behavior is inventoried.
   - replay/stale/corrupt reject evidence is inventoried.
   - deterministic/hermetic feasibility is recorded.
   - dependency/workflow/source-change needs are recorded.

5. Option review checks
   - All eight directive options are selected, rejected, or deferred.
   - Risk reduced, evidence gap, feasibility, scope risk, public-claim risk,
     external-review value, future paths, and P0/P1/P2 risks are recorded.

6. Stewardship review checks
   - Hostile Cryptographer Review complete.
   - Red-Team Review complete.
   - Production SRE Review complete.
   - Release-Claim Boundary Review complete.
   - Side-channel caveat and formal-model residuals are preserved.
   - External-review readiness does not become external-review-complete.
   - Assurance gap trigger is preserved.

7. Authorization decision checks
   - Exactly one primary classification is selected.
   - Selected classification is
     `SAME_HOST_CLIENT_TO_CLIENT_E2E_IMPLEMENTATION_READY`.
   - Selected successor is NA-0504.
   - NA-0504 block includes allowed paths, forbidden paths, objective,
     deliverables, acceptance criteria, validation markers, and stop
     conditions.

## Validation commands

Run and record:

```bash
git diff --check
git diff --name-only origin/main...HEAD
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run local policy checks:

- exact five-path scope guard for the NA-0503 evidence PR.
- local markdown link existence check.
- added-line/new-file leak scan.
- added-line/new-file overclaim scan.
- classifier.
- PR body preflight.
- goal-lint preflight.

## Expected NA-0503 markers

- `NA0503_QWORK_PROOF_FILES_OK`
- `NA0503_D388_INHERITANCE_CONSUMED_OK`
- `NA0503_E2E_SURFACE_INVENTORY_OK`
- `NA0503_OPTION_REVIEW_OK`
- `NA0503_STEWARDSHIP_REVIEWS_OK`
- `NA0503_PRIORITIZATION_MATRIX_OK`
- `NA0503_SAME_HOST_E2E_IMPLEMENTATION_READY`
- `NA0503_NA0504_SUCCESSOR_SELECTED_OK`
- `NA0503_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0503_NO_REMOTE_SSH_SCOPE_OK`
- `NA0503_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0503_NO_DEPENDENCY_WORKFLOW_HELPER_MUTATION_OK`
- `NA0503_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0503_NO_PUBLIC_PRODUCTION_CRYPTO_OVERCLAIM_OK`
- `NA0503_ONE_READY_INVARIANT_OK`

## Future NA-0504 marker plan

- `NA0504_CLIENT_TO_CLIENT_SCOPE_CONSUMED_OK`
- `NA0504_TWO_INDEPENDENT_CLIENT_ROOTS_OK`
- `NA0504_ALICE_BOB_IDENTITY_SETUP_OK`
- `NA0504_PUBLIC_RECORD_TRUST_EXCHANGE_OK`
- `NA0504_SEND_RECEIVE_FLOW_OK`
- `NA0504_REPLY_FLOW_OK`
- `NA0504_NEGATIVE_REJECT_NO_MUTATION_OK`
- `NA0504_STDOUT_STDERR_NO_SECRET_OUTPUT_OK`
- `NA0504_NO_REMOTE_SSH_SCOPE_OK`
- `NA0504_NO_QSC_SOURCE_CHANGE_OK`
- `NA0504_NO_DEPENDENCY_CHANGE_OK`
- `NA0504_NO_WORKFLOW_CHANGE_OK`
- `NA0504_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0504_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0504_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0504_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0504_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0504_ONE_READY_INVARIANT_OK`

## Pass criteria

- NA-0503 evidence doc and testplan exist.
- D-0995 exists once.
- TRACEABILITY includes NA-0503 authorization evidence.
- Rolling journal includes directive, proof-root, queue, recovery, validation,
  selected successor, and claim-boundary notes.
- Changed paths are exactly the five allowed evidence paths.
- All required local validation commands pass or are stopped with directive
  evidence if a stop condition is reached.
- PR body includes a standalone `Goals: G1, G2, G3, G4, G5` line.
- Exactly one READY item remains before NA-0503 closeout.

## Fail criteria

- qwork proof is missing, stale, or contradicted by live pre-fetch state.
- Codex runs `qwork`, `qstart`, or `qresume`.
- More than one READY item exists.
- D-0994 is absent.
- D-0995 already exists before patch or does not exist exactly once after patch.
- Any forbidden path is mutated.
- Any implementation mutation occurs.
- Any remote SSH or two-machine setup is attempted.
- Any dependency, lockfile, workflow, script, helper, corpus/vector/input,
  formal, refimpl, service, public-doc, backup, or qsl-backup mutation occurs.
- Any required validation command conclusively fails.
- Any public/security/completion overclaim is introduced.
