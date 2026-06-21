Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0513 Remote qsc Staging Strategy Authorization Testplan

## Objective

Verify that NA-0513 triages the inherited D404 red remote checks, consumes
NA-0512/D404 inheritance, selects an exact future remote qsc staging/smoke
strategy, and preserves authorization-only boundaries with no remote action.

## Scope invariants

- NA-0513 advances G4 without regressing G1, G2, G3, or G5.
- Packet A red check triage must complete before any NA-0513 mutation.
- `remote-handshake`, `remote-relay`, and `relay-ui-integration` must not be
  ignored, treated as passed, and no public-readiness evidence is claimed from
  them.
- No remote action by Codex in NA-0513.
- No SSH execution by Codex in NA-0513.
- No scp/sftp/rsync execution by Codex in NA-0513.
- No binary transfer in NA-0513.
- No remote E2E.
- No qsc send/receive.
- No remote source checkout/build.
- No package installation.
- No sudo/admin action.
- No key generation or installation.
- No SSH config or known_hosts mutation.
- No qwork/qstart/qresume mutation.
- No qsl-backup execution or mutation.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- no public-readiness claim is made.
- no production-readiness claim is made.
- exactly one READY item remains mandatory.

## Required Packet A checks

Packet A must save proof for:

- REST check-runs for `72994b2882e7`.
- PR #1297 metadata.
- PR #1297 changed files.
- branch-protection required status checks.
- workflow run/job metadata for `remote-handshake`, `remote-relay`, and
  `relay-ui-integration`.
- job logs or metadata sufficient to classify each red check.
- workflow file names and event types.

Continuation is allowed only if all three red checks are:

- not required branch-protection contexts.
- not public-safety aggregate inputs.
- not NA-0512 closeout acceptance requirements.
- consistent with remote residual / unstaged scenario gaps or otherwise clearly
  unrelated to current qsl-protocol correctness.
- not evidence of unsafe remote mutation, backup/qwork/qsl-backup exposure, or
  claim drift.

## Static validation

Run:

- `git diff --check`
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0513 evidence.
- proof that evidence contains no private-key block headers.
- proof that evidence did not include private key/passphrase/token/password
  material.

## Required local validation

Run:

- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Required acceptance markers

- `NA0513_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0513_D404_RED_CHECK_TRIAGE_COMPLETED_OK`
- `NA0513_D404_PUBLIC_SAFETY_GREEN_OK`
- `NA0513_D404_RED_REMOTE_CHECKS_NONBLOCKING_RESIDUAL_OK`
- `NA0513_NA0512_D404_INHERITANCE_CONSUMED_OK`
- `NA0513_STAGING_OPTIONS_REVIEWED_OK`
- `NA0513_EXACT_FUTURE_COMMAND_FAMILY_SELECTED_OK`
- `NA0513_REDACTION_STOP_RULES_SELECTED_OK`
- `NA0513_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0513_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0513_REMOTE_PREBUILT_QSC_BINARY_STAGING_SMOKE_READY`
- `NA0513_SELECTED_NA0514_SUCCESSOR_OK`
- `NA0513_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0513_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0513_NO_BINARY_TRANSFER_OK`
- `NA0513_NO_REMOTE_E2E_OK`
- `NA0513_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0513_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0513_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0513_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0513_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0513_ONE_READY_INVARIANT_OK`

## Future NA-0514 marker plan

- `NA0514_REMOTE_STAGING_SCOPE_CONSUMED_OK`
- `NA0514_LOCAL_QSC_BINARY_BUILT_OR_SELECTED_OK`
- `NA0514_LOCAL_QSC_BINARY_HASH_RECORDED_OK`
- `NA0514_REMOTE_QSC_BINARY_STAGED_OK`
- `NA0514_REMOTE_QSC_BINARY_HASH_MATCH_OK`
- `NA0514_REMOTE_QSC_SMOKE_OK`
- `NA0514_REMOTE_QSC_RETENTION_DECISION_OK`
- `NA0514_NO_REMOTE_E2E_OK`
- `NA0514_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0514_NO_PACKAGE_INSTALL_OK`
- `NA0514_NO_SUDO_ADMIN_OK`
- `NA0514_NO_BACKUP_EXPOSURE_OK`
- `NA0514_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0514_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0514_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

1. Correctness under stress: Packet A explicitly distinguishes required checks,
   public-safety inputs, and residual scheduled checks before allowing staging
   authorization.
2. Minimality: NA-0513 changes only governance evidence, testplan, decision,
   traceability, and journal paths.
3. Maintainability: the selected NA-0514 command family, redaction rules, stop
   rules, and markers are explicit and reusable.
4. Coverage quality: validation proves markers, exact scope, no private
   material, no overclaim wording, existing qsc/formal/corpus/audit health, and
   public-safety behavior.
5. Cross-lane stability: macOS/Linux required context policy remains unchanged;
   skipped full suites are not reclassified as passing and no workflow is
   mutated.
