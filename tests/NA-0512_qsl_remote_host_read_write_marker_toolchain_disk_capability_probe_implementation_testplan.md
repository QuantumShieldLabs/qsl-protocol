Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0512 Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Implementation Testplan

## Objective

Verify that NA-0512 consumes NA-0511 / D403 inheritance, executes exactly one
bounded remote marker/toolchain/disk capability probe, records the sanitized
result classification, preserves all no-mutation and no-overclaim boundaries,
and selects the next remote-focused successor from the observed toolchain/qsc
state.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0512 advances G4 without regressing G1, G2, G3, or G5.
- qwork proof files are read only; Codex does not run qwork, qstart, or qresume.
- Exactly one remote SSH invocation is executed.
- No second SSH invocation is executed.
- No scp, sftp, or rsync is executed.
- No ssh-keygen or ssh-keyscan is executed.
- No SSH key generation or installation occurs.
- No local/system SSH config mutation occurs.
- No known_hosts or authorized_keys mutation occurs.
- No remote account, service, package, key, or host setup mutation occurs.
- No sudo/admin action occurs except the authorized negative `sudo -n true`
  check.
- No remote file write occurs outside the one synthetic marker lifecycle.
- No marker artifact remains after deletion.
- No remote E2E occurs.
- No remote qsc send/receive or qsc key generation occurs.
- No remote source checkout/build/update occurs.
- No package installation occurs.
- No qwork/qstart/qresume mutation occurs.
- No qsl-backup execution or mutation occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No workflow/script/helper/dependency mutation occurs.
- No corpus/vector/input mutation occurs.
- No formal/refimpl/service/public/backup mutation occurs.
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

- `docs/governance/evidence/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_harness.md`
- `tests/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- any implementation code mutation outside the allowed governance/testplan
  paths.
- qsc source/test/fuzz/Cargo mutation.
- workflow/script/helper mutation.
- corpus/vector/input mutation.
- dependency/lockfile mutation.
- formal/refimpl/service/public/backup mutation.
- qsl-server, qsl-attachments, qshield, or qshield-cli mutation.
- qsl-backup, backup status, backup plan, backup data, rollback, archive, move,
  or delete mutation.
- public docs, website, README, or START_HERE mutation.
- branch protection mutation.
- D132 cleanup.

## Required proof checks

Startup and inheritance:

- qwork proof files exist and are copied without rerunning qwork.
- qwork `.kv` and `.json` mirror required fields.
- proof `HEAD` and `origin_main` match live refs before fetch.
- fetch occurs only after proof/live match and disk proof below 95%.
- READY_COUNT is 1.
- READY item is NA-0512.
- NA-0511, NA-0510, and NA-0509 are DONE.
- D-1011 exists once.
- D-1012 exists once.
- D-1013 is absent before patch and present once after patch.
- duplicate decision count is 0.
- D402 and D403 responses exist and are consumed.
- D402 selected `REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`.
- D403 recovered PR #1294 checks via one close/reopen and no empty commit.
- PR #1294 merged at `5f27d289e088`.
- PR #1295 merged at `6b18574cbf16`.

Local pre-SSH:

- `ssh -G inspiron` is parsed for safe fields.
- user is `qslcodex`.
- hostname is `inspiron`.
- identity basename is non-empty.
- identities-only, noninteractive credential, batch mode, strict host-key, no
  forwarding, and clear-forwarding settings are verified.
- no full private key path or private key contents are checked into evidence.

Remote probe:

- exactly one SSH invocation is recorded.
- exit code is captured.
- stdout/stderr are captured under the proof root.
- no `STOP_` marker appears for a passing probe.
- account marker appears.
- non-root marker appears.
- no-sudo marker appears.
- workdir exists/writable markers appear.
- marker write/read/delete/absent-after-delete markers appear.
- no backup exposure marker appears.
- qwork absence marker appears.
- qsl-backup absence marker appears.
- toolchain status captured marker appears.
- disk status captured marker appears.
- no remote E2E marker appears.
- no source build marker appears.
- no package install marker appears.

Scope and content:

- exact five-path scope guard passes.
- evidence contains required NA-0512 sections.
- testplan contains required markers.
- D-1013 records the selected classification and selected successor.
- TRACEABILITY records NA-0512 evidence and no-claim boundaries.
- rolling journal records directive, timestamps, SHAs, READY proof, branch/PR
  state, validation, CI notes, disk watermark, recovered failures, and
  next-watch items.

Security/material:

- raw remote stdout/stderr contain no private-key block markers.
- raw remote stdout/stderr contain no credential assignment pattern.
- raw remote stdout/stderr contain no production endpoint marker.
- raw remote stdout/stderr contain no backup material.
- raw remote stdout/stderr contain no qsl-backup execution marker.
- raw remote stdout/stderr contain no qwork run marker.
- raw remote stdout/stderr contain no source checkout/build markers.
- raw remote stdout/stderr contain no package install markers.
- checked-in evidence redacts or summarizes remote host, home, pwd, and disk
  topology.
- added-line overclaim scan finds no forbidden claim without same-line no-claim
  wording.

Required local validation:

- `git diff --check`
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0512 evidence.
- proof that evidence contains no private key blocks.
- proof that evidence does not include private key/passphrase/token/password
  material.
- proof that evidence does not include unnecessary unredacted private key paths.
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

## Selected successor marker plan

Selected NA-0513 successor should prove:

- `NA0513_NA0512_PROBE_CONSUMED_OK`
- `NA0513_TOOLCHAIN_ABSENCE_CONSUMED_OK`
- `NA0513_STAGING_STRATEGY_OPTIONS_REVIEWED_OK`
- `NA0513_EXACT_FUTURE_SCOPE_SELECTED_OK`
- `NA0513_NO_REMOTE_COMMAND_OK`
- `NA0513_NO_REMOTE_E2E_OK`
- `NA0513_NO_PACKAGE_INSTALL_OK`
- `NA0513_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0513_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0513_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

1. Correctness under stress: the probe rechecks identity, privilege, backup,
   qwork/qsl-backup, workdir, marker cleanup, toolchain/disk state, redaction,
   and no-E2E/no-build/no-install markers before accepting the classification.
2. Minimality: NA-0512 mutates only governance evidence, testplan, decision,
   traceability, and journal files; it changes no implementation path.
3. Maintainability: proof markers, classification, redaction expectations, and
   successor logic are centralized in evidence and mirrored by this testplan.
4. Coverage quality: validation proves markers, exact scope, no private
   material, no overclaim wording, and existing qsc/formal/corpus/audit health.
5. Cross-lane stability: Linux/macOS full suites remain accepted only by current
   docs/governance policy, and no qsc source/workflow/dependency path changes.
