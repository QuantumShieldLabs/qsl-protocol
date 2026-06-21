Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0511 Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Testplan

## Objective

Verify that NA-0511 consumes NA-0510 / D401 inheritance, selects exact future
scope for one bounded marker write/read/delete plus toolchain/disk capability
probe, preserves no remote action in the authorization lane, selects NA-0512 as
the exact successor, and preserves all no-mutation and no-overclaim boundaries.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0511 advances G4 without regressing G1, G2, G3, or G5.
- No remote action by Codex in NA-0511.
- No SSH execution by Codex in NA-0511.
- No scp/sftp/rsync to remote.
- No ssh-keygen.
- No ssh-keyscan.
- No remote account creation.
- No SSH key generation or installation.
- No local SSH config mutation.
- No system SSH config mutation.
- No known_hosts mutation.
- No authorized_keys mutation.
- No remote host mutation.
- No sudo/admin action.
- No package installation.
- No marker write/read/delete in NA-0511.
- No remote toolchain command in NA-0511.
- No remote E2E.
- No remote qsc send/receive.
- No remote source checkout/build.
- No qwork/qstart/qresume mutation.
- No qsl-backup execution or mutation.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No crypto-complete claim is made.
- No replay-proof claim is made.
- No downgrade-proof claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.

## Allowed scope

- `docs/governance/evidence/NA-0511_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_scope_authorization_plan.md`
- `tests/NA-0511_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- remote action.
- SSH execution.
- scp, sftp, or rsync to remote.
- ssh-keygen or ssh-keyscan.
- key generation or key installation.
- local/system SSH config mutation.
- known_hosts or authorized_keys mutation.
- remote host mutation.
- remote account creation.
- sudo/admin action.
- package installation.
- remote marker write/read/delete.
- remote toolchain command execution.
- remote E2E.
- remote qsc send/receive, source checkout/build, service action,
  qwork/qstart/qresume, or qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow, script, helper, validator, or dependency mutation.
- corpus, vector, input, or internal-manifest mutation.
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, service,
  public, website, backup, backup-plan, qsl-backup, rollback, archive, move, or
  delete mutation.

## Required proof checks

Startup and inheritance:

- qwork proof files exist and are copied without rerunning qwork.
- qwork `.kv` and `.json` mirror required fields.
- proof `HEAD` and `origin_main` match live refs before fetch.
- fetch occurs only after proof/live match and disk proof below 95%.
- READY_COUNT is 1.
- READY item is NA-0511.
- NA-0510, NA-0509, and NA-0508 are DONE.
- D-1009 exists once.
- D-1010 exists once.
- D-1011 is absent before patch and present once after patch.
- duplicate decision count is 0.
- D400 and D401 responses exist and are consumed.
- NA-0510 read-only probe classification is `REMOTE_READ_ONLY_PROBE_PASS`.

Scope and content:

- exact five-path scope guard passes.
- evidence contains required sections.
- testplan contains required markers.
- D-1011 records the selected classification and selected successor.
- TRACEABILITY records NA-0511 evidence and no-claim boundaries.
- rolling journal records directive, timestamps, SHAs, READY proof, branch/PR
  state, validation, CI notes, disk watermark, and next-watch items.

Security/material:

- no private-key block appears in evidence/testplan/decision/traceability/journal.
- no private key, passphrase, token, password, production endpoint, or backup
  material is introduced.
- added-line overclaim scan finds no forbidden claim without same-line no-claim
  wording.
- response and PR body keep authorization-only wording.

Required local validation:

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0511 evidence
- private key block proof
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- binding corpus validator over `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- all qsc fuzz corpus validator over `qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Required acceptance markers

- `NA0511_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0511_D401_INHERITANCE_CONSUMED_OK`
- `NA0511_TIME_SENSITIVE_REMOTE_ASSURANCE_REVIEW_OK`
- `NA0511_MARKER_SCOPE_SELECTED_OK`
- `NA0511_TOOLCHAIN_DISK_SCOPE_SELECTED_OK`
- `NA0511_EXACT_FUTURE_COMMANDS_SELECTED_OK`
- `NA0511_EXPECTED_OUTPUTS_SELECTED_OK`
- `NA0511_REDACTION_RULES_SELECTED_OK`
- `NA0511_STOP_CONDITIONS_SELECTED_OK`
- `NA0511_NO_REMOTE_E2E_BOUNDARY_OK`
- `NA0511_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0511_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0511_REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`
- `NA0511_SELECTED_NA0512_SUCCESSOR_OK`
- `NA0511_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0511_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0511_NO_MARKER_WRITE_READ_DELETE_BY_CODEX_OK`
- `NA0511_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0511_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0511_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0511_ONE_READY_INVARIANT_OK`

## Future NA-0512 marker plan

The selected NA-0512 successor should prove:

- `NA0512_REMOTE_SCOPE_CONSUMED_OK`
- `NA0512_REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_EXECUTED_OK`
- `NA0512_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0512_REMOTE_NOT_ROOT_OK`
- `NA0512_REMOTE_NO_SUDO_OK`
- `NA0512_REMOTE_WORKDIR_EXISTS_OK`
- `NA0512_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0512_REMOTE_MARKER_WRITE_OK`
- `NA0512_REMOTE_MARKER_READ_OK`
- `NA0512_REMOTE_MARKER_DELETE_OK`
- `NA0512_REMOTE_MARKER_ABSENT_AFTER_DELETE_OK`
- `NA0512_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0512_REMOTE_QWORK_ABSENT_OK`
- `NA0512_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0512_REMOTE_TOOLCHAIN_STATUS_CAPTURED_OK`
- `NA0512_REMOTE_DISK_STATUS_CAPTURED_OK`
- `NA0512_NO_REMOTE_E2E_OK`
- `NA0512_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0512_NO_PACKAGE_INSTALL_OK`
- `NA0512_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0512_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0512_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

1. Correctness under stress: the selected successor rechecks identity, privilege,
   backup, qwork/qsl-backup, workdir, marker cleanup, and redaction before any
   later staging/E2E work.
2. Minimality: NA-0511 changes only governance evidence, testplan, decision,
   traceability, and journal files; it performs no remote action and changes no
   implementation path.
3. Maintainability: future command family, markers, redaction rules, and stop
   conditions are centralized in evidence and mirrored by this testplan.
4. Coverage quality: validation proves markers, exact scope, no private material,
   no overclaim wording, and existing qsc/formal/corpus/audit health.
5. Cross-lane stability: Linux/macOS full suites remain accepted only by current
   docs/governance policy, and no qsc source/workflow/dependency path changes.
