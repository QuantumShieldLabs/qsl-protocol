Status: Supporting
Owner: QSL Governance / QA
Last-Updated: 2026-06-21

Goals: G1, G2, G3, G4, G5

# NA-0515 Build-to-Inspiron remote qsc client-to-client E2EE scope authorization testplan

## Objective

Validate that NA-0515 is an authorization-only lane that consumes NA-0514/D407, reviews retained remote qsc evidence, selects exact future Build-to-Inspiron E2EE scope, records D-1019, updates traceability and the rolling journal, and preserves all no-remote-action and no-public-claim boundaries.

## Scope checks

- Changed paths must be exactly:
  - `docs/governance/evidence/NA-0515_qsl_build_to_inspiron_remote_qsc_client_to_client_e2ee_scope_authorization_plan.md`
  - `tests/NA-0515_qsl_build_to_inspiron_remote_qsc_client_to_client_e2ee_scope_authorization_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- No qsc source/test/fuzz/Cargo files may change.
- No workflow, script, helper, dependency, lockfile, corpus, vector, formal, refimpl, service, public, backup, qsl-server, qsl-attachments, qshield, or qshield-cli files may change.

## Startup verification

- qwork proof files exist and are read only.
- qwork is not run by Codex.
- `startup_result=OK`.
- lane and repo are `NA-0515` and `qsl-protocol`.
- proof path is `/srv/qbuild/work/NA-0515/qsl-protocol`.
- proof `HEAD` and `origin/main` match live `HEAD` and live `origin/main` before fetch.
- worktree, index, and untracked state are clean before work.
- `READY_COUNT` is 1 and the sole READY item is NA-0515.
- NA-0514, NA-0513, and NA-0512 are DONE.
- D-1017 and D-1018 exist once.
- D-1019 is absent before patch and exists once after patch.
- Duplicate decision count is zero.

## Inheritance checks

Evidence must include:

- `REMOTE_PREBUILT_QSC_STAGING_SMOKE_PASS_RETAINED`.
- local source commit `6e0796de79c9abb4d3c5e18b46b004b5bd585167`.
- retained remote qsc sha256 `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- retained path `qsl-remote-test/bin/qsc`.
- remote owner `qslcodex`.
- remote `--help` smoke exit code 0.
- recorded cleanup command from NA-0514.
- no remote E2EE in NA-0514.
- no qsc send/receive in NA-0514.
- no remote source checkout/build or package install in NA-0514.

## Retained binary review checks

- Evidence accepts the retained binary only for authorizing the next bounded lane.
- NA-0516 must recheck hash/path/owner/provenance before E2EE.
- NA-0516 must stop on retained-binary mismatch.

## Future command family checks

- Evidence selects Build as Alice/local sender and Inspiron as Bob/remote receiver/replier by default.
- Evidence defines isolated local and remote roots.
- Evidence selects synthetic messages only.
- Evidence lists observed qsc CLI surfaces from same-host tests.
- Evidence permits bounded SSH and bounded synthetic-artifact transfer only in future NA-0516, not NA-0515.
- Evidence selects wrong-mailbox or wrong-peer no-mutation as the preferred negative boundary.
- Evidence defines cleanup or documented retention under qsl-remote-test only.

## Stewardship and claim-boundary checks

Evidence must include:

- Best-Known-Method Review.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Side-Channel Caveat through the release-claim boundary.
- Formal-Model Mapping Residual.
- External-Review Readiness.
- Release-Claim Boundary.
- Assurance Gap Review Trigger.

The evidence must include explicit claim-boundary statements:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## Static validation

Required static checks:

- `git diff --check`.
- exact five-path scope guard.
- deterministic relative markdown link-check.
- leak-scan against added/new NA-0515 evidence.
- overclaim scan against added/new NA-0515 evidence.
- docs/governance classifier.
- PR body preflight.
- goal-lint preflight.
- marker proof for required NA-0515 evidence markers.
- proof that evidence contains no private-key block headers or API token style fixtures.
- proof that evidence did not include private key, passphrase, token, password, production endpoint, or backup material.

## Required local validation

Run:

- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
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

Do not run SSH, scp, sftp, rsync, qsc send/receive, qsc remote E2EE, ssh-keygen, ssh-keyscan, sudo, qwork, qstart, qresume, or qsl-backup.

## Required markers

- `NA0515_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0515_NA0514_D407_INHERITANCE_CONSUMED_OK`
- `NA0515_RETAINED_REMOTE_QSC_REVIEWED_OK`
- `NA0515_E2EE_OPTIONS_REVIEWED_OK`
- `NA0515_EXACT_FUTURE_COMMAND_FAMILY_SELECTED_OK`
- `NA0515_REDACTION_STOP_RULES_SELECTED_OK`
- `NA0515_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0515_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0515_REMOTE_BUILD_TO_INSPIRON_E2EE_IMPLEMENTATION_READY`
- `NA0515_SELECTED_NA0516_SUCCESSOR_OK`
- `NA0515_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0515_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0515_NO_QSC_SEND_RECEIVE_OK`
- `NA0515_NO_REMOTE_E2EE_OK`
- `NA0515_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0515_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0515_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0515_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0515_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0515_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0515_ONE_READY_INVARIANT_OK`

## Future marker plan

Future NA-0516 evidence should include:

- `NA0516_REMOTE_E2EE_SCOPE_CONSUMED_OK`
- `NA0516_RETAINED_REMOTE_QSC_HASH_RECHECKED_OK`
- `NA0516_LOCAL_QSC_PROVENANCE_RECORDED_OK`
- `NA0516_RELAY_TRANSPORT_BOUNDARY_OK`
- `NA0516_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`
- `NA0516_INSPIRON_TO_BUILD_REPLY_OK`
- `NA0516_REMOTE_E2EE_SYNTHETIC_MESSAGES_ONLY_OK`
- `NA0516_REMOTE_E2EE_NO_SECRET_OUTPUT_OK`
- `NA0516_REMOTE_E2EE_NEGATIVE_BOUNDARY_OK`
- `NA0516_REMOTE_E2EE_CLEANUP_OR_RETENTION_OK`
- `NA0516_NO_PACKAGE_INSTALL_OK`
- `NA0516_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0516_NO_QWORK_QSLBACKUP_OK`
- `NA0516_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0516_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0516_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

- Correctness under stress: the lane remains fail-closed because NA-0516 must stop on stale proof, retained-binary mismatch, remote boundary failure, source/build ambiguity, secret-looking output, send/receive/reply failure, or cleanup ambiguity.
- Minimality: the patch is limited to governance evidence, testplan, decision, traceability, and rolling journal.
- Maintainability: future NA-0516 command family is derived from existing qsc test patterns and leaves exact runtime values to the implementation directive.
- Coverage quality: validation includes static guards, targeted qsc tests, corpus scans, formal checks, audits, formatting, and adversarial shell syntax.
- Cross-lane stability: no qsc source, workflows, dependencies, formal assets, services, public docs, or backup assets are mutated, preserving macOS/Linux consistency for affected areas.
