Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0510 Remote Host Read-Only Capability Probe Implementation Testplan

## Objective

Verify that NA-0510 consumes NA-0509 / D398 inheritance, validates fresh qwork
proof, runs exactly one bounded read-only SSH capability probe to `inspiron` as
`qslcodex`, records redacted evidence, classifies the result, selects the next
remote-focused successor, and preserves all no-mutation and no-overclaim
boundaries.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0510 advances G4 without regressing G1, G2, G3, or G5.
- One remote SSH invocation maximum.
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
- No sudo/admin action except negative `sudo -n true` check.
- No package installation.
- No remote file creation, write, marker write, read/delete marker cycle, or
  deletion.
- No remote qsc send/receive, qsc key generation, remote source checkout/build,
  service action, qwork/qstart/qresume, or qsl-backup execution.
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

- `docs/governance/evidence/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_harness.md`
- `tests/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local capture of the authorized read-only SSH output.

## Forbidden scope

- more than one remote SSH invocation.
- scp, sftp, or rsync to remote.
- ssh-keygen or ssh-keyscan.
- key generation or key installation.
- local/system SSH config mutation.
- known_hosts or authorized_keys mutation.
- remote host mutation.
- remote account creation.
- sudo/admin action beyond the negative `sudo -n true` probe.
- remote file write, marker write, or deletion.
- remote qsc send/receive, source checkout/build, package installation, service
  action, qwork/qstart/qresume, or qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow, script, helper, validator, or dependency mutation.
- corpus, vector, input, or internal-manifest mutation.
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, service,
  public-doc, website, README, START_HERE, backup, backup status, backup plan,
  rollback, archive, move, or delete mutation.

## Required evidence doc

Required path:

- `docs/governance/evidence/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_harness.md`

Expected sections:

- executive summary.
- live NA-0510 scope.
- qwork proof-file verification.
- NA-0509 / D398 inheritance.
- local pre-SSH proof.
- exact remote command.
- remote probe output summary.
- remote output redaction review.
- probe result classification.
- account identity proof.
- non-root / no-sudo proof.
- workdir proof.
- backup exposure proof.
- qwork / qsl-backup absence proof.
- no remote E2E proof.
- no remote file write proof.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Release-Claim Boundary Review.
- successor selection.
- future scope bundle.
- future validation / marker plan.
- public claim / website / external review boundary.
- backup-impact statement.
- rejected alternatives.
- next recommendation.

## qwork proof checks

Expected:

- qwork proof files exist and are copied to the proof root.
- `.kv` proof fields match required NA-0510 values.
- `.json` proof mirrors `.kv`.
- proof HEAD equals live HEAD before fetch.
- proof origin/main equals live origin/main before fetch.
- Codex does not run qwork, qstart, or qresume.

## Inheritance checks

Expected:

- D398 response exists and is consumed.
- NA-0509 is DONE.
- NA-0510 is READY.
- D-1007 exists once.
- D-1008 exists once.
- D-1009 is absent before the patch and exists once after the patch.
- duplicate decision ID count is zero.
- NA-0509 selected `REMOTE_READ_ONLY_CAPABILITY_PROBE_IMPLEMENTATION_READY`.
- NA-0509 authorized exactly one future bounded read-only SSH probe.
- NA-0509 did not authorize marker write/read/delete, toolchain/disk probes, or
  remote E2E.

## Local pre-SSH checks

Expected parsed `ssh -G inspiron` fields:

- hostname is `inspiron`.
- user is `qslcodex`.
- identityfile basename is non-empty.
- `identitiesonly=yes`.
- `passwordauthentication=no`.
- `batchmode=yes`.
- `stricthostkeychecking=yes` or `true`.
- `forwardagent=no`.
- `forwardx11=no`.
- `clearallforwardings=yes`.
- no private key block marker.
- no passphrase/token/password assignment pattern.
- `ssh -G remote` is not run.

## Remote command checks

Expected command:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'
```

Expected remote script command family:

- `set -u`
- `hostname`
- `id -un`
- `id -u`
- `id -Gn`
- `pwd`
- `printf` of fixed markers and `$HOME`
- `test -d "$HOME/qsl-remote-test"`
- `test -w "$HOME/qsl-remote-test"`
- `sudo -n true` as a negative capability check only
- `test -e /backup/qsl`
- `test -r /backup/qsl`
- `command -v qwork`
- `command -v qsl-backup`

Forbidden remote commands include qsc, git, cargo, rustc, uname, df, qwork,
qstart, qresume, qsl-backup, service/systemctl, package managers, file writes,
marker writes, deletion, and remote E2E.

## Result classification checks

Exactly one classification must be selected:

- `REMOTE_READ_ONLY_PROBE_PASS`
- `REMOTE_PROBE_CONNECTION_FAILURE`
- `REMOTE_PROBE_AUTH_FAILURE`
- `REMOTE_PROBE_ALIAS_OR_HOST_DRIFT`
- `REMOTE_PROBE_WRONG_ACCOUNT`
- `REMOTE_PROBE_PRIVILEGE_BOUNDARY_FAILURE`
- `REMOTE_PROBE_BACKUP_EXPOSURE_FAILURE`
- `REMOTE_PROBE_QWORK_QSLBACKUP_BOUNDARY_FAILURE`
- `REMOTE_PROBE_WORKDIR_FAILURE`
- `REMOTE_PROBE_OUTPUT_REDACTION_FAILURE`
- `REMOTE_PROBE_AMBIGUOUS_STOP`

NA-0510 expected result for the captured evidence:

- `REMOTE_READ_ONLY_PROBE_PASS`

## Required marker proof

Evidence/testplan/decision must contain:

- `NA0510_REMOTE_PROBE_SCOPE_CONSUMED_OK`
- `NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK`
- `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0510_REMOTE_NOT_ROOT_OK`
- `NA0510_REMOTE_NO_SUDO_OK`
- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0510_NO_REMOTE_E2E_OK`
- `NA0510_NO_REMOTE_FILE_WRITE_OK`
- `NA0510_NO_SSH_KEY_GENERATION_OK`
- `NA0510_NO_SSH_CONFIG_MUTATION_OK`
- `NA0510_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0510_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0510_ONE_READY_INVARIANT_OK`

If NA-0511 is selected, future marker plan must include:

- `NA0511_REMOTE_READ_ONLY_PROBE_CONSUMED_OK`
- `NA0511_REMOTE_MARKER_PROBE_SCOPE_SELECTED_OK`
- `NA0511_REMOTE_TOOLCHAIN_DISK_SCOPE_SELECTED_OK`
- `NA0511_NO_REMOTE_ACTION_IN_AUTHORIZATION_OK`
- `NA0511_NO_REMOTE_E2E_SCOPE_OK`
- `NA0511_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0511_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0511_ONE_READY_INVARIANT_OK`

## Redaction checks

Expected:

- no private key block appears in evidence/testplan/decision/traceability/
  journal changes.
- no passphrase, token, password, credential, production endpoint, or backup
  material appears in added lines.
- no full `ssh -G` output appears in checked-in files.
- no full identity path appears in checked-in files.
- host, groups, `$HOME`, and `pwd` are redacted or summarized.
- raw probe output remains proof-root-local.

## Validation commands

Required local validation:

- `git diff --check`.
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof.
- private-key-block scan.
- material-pattern scan.
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`.
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`.
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`.
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`.
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`.
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`.
- `python3 formal/run_model_checks.py`.
- `cargo audit --deny warnings`.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.

Forbidden validation commands:

- a second SSH command.
- scp, sftp, or rsync to remote.
- ssh-keygen.
- ssh-keyscan.
- sudo locally.
- qwork, qstart, or qresume.
- qsl-backup.

## Successor checks

Expected selected successor after pass:

- `NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan`

Expected rationale:

- NA-0510 passed the read-only probe.
- marker write/read/delete remains unproven and was not authorized by D398.
- toolchain/disk capability remains unproven and was not authorized by D398.
- remote E2E remains deferred until marker/staging prerequisites are authorized
  and evidenced.

## Post-fix hardening review checks

Correctness under stress:

- verify the probe result is not overread as protocol evidence.
- verify future lanes must recheck drift-sensitive remote boundaries.

Minimality:

- exactly five allowed files changed.
- no implementation code, dependency, workflow, corpus, formal, service, public,
  or backup paths changed.

Maintainability:

- evidence is organized by proof category.
- raw sensitive/topology-bearing output remains proof-root-local.

Coverage quality:

- marker proof confirms all required NA-0510 markers.
- negative no-mutation boundaries are scanned in added lines.

Cross-lane stability:

- Linux/macOS qsc full-suite checks are not required for docs/governance scope
  when policy-skipped, but the affected local qsc validation bundle remains
  required.
