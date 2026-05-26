Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25
Replaces: n/a
Superseded-By: n/a

# NA-0362 Closeout and NA-0363 Restoration Testplan

## Objective

Close NA-0362 after the qsl-protocol off-host encrypted backup target/tool
implementation authorization merges with required checks green, and restore
exactly one READY successor:
`NA-0363 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool
No-Secret Implementation Harness`.

## Protected Invariants

- NA-0362 is DONE.
- Exactly one READY item exists after closeout: NA-0363.
- D-0706 and D-0707 each exist once; D-0708 is absent.
- NA-0363 is not implemented by this closeout.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  mutated.
- No backup script, timer, fstab, local backup config, off-host target,
  remote destination, repository, key file, passphrase path, recovery envelope,
  restore path, deploy path, rollback path, workflow, dependency, website,
  README, START_HERE, docs/public path, branch-protection, public-safety
  configuration, service path, or secret-handling path is changed.
- No backup, restore, deploy, rollback, remote/off-host operation, repository
  init, tool installation, restore target creation/mount/copy, key generation,
  key upload, passphrase collection, private-key inspection, recovery-envelope
  content creation, or secret material handling is performed.
- Local continuity is not described as complete disaster recovery.
- Off-host encrypted backup is not described as complete.
- The selected no-secret target/tool harness is not described as a real remote
  target, real repository, real backup, or real restore.
- The NA-0361 no-secret key custody/recovery harness is not described as real
  key custody or real key recovery implementation.
- The NA-0359 dry-run restore harness is not described as real restore
  execution.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, hidden all metadata, restore-drill completion, off-host backup
  completion, or disaster-recovery completion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0362_closeout_restore_na0363_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, service, deployment, restore, rollback,
  backup, branch-protection, public-safety, secret-handling, key-generation,
  key-upload, private-key-inspection, passphrase-collection,
  recovery-envelope, remote/off-host connection, repository-init, tool-install,
  or off-host backup setup paths.

## Closeout Requirements

1. Record qsl-protocol PR #986 merge evidence for NA-0362.
2. Mark NA-0362 DONE.
3. Restore `NA-0363 -- Metadata Runtime Off-Host Encrypted Backup Target /
   Tool No-Secret Implementation Harness` as the sole READY item.
4. Add D-0707 with explicit no-NA-0363-implementation language.
5. Link D-0707, D-0706, this testplan, selected successor, PR #986, and
   backup-plan impact in TRACEABILITY.
6. Keep backup-plan impact explicit: no current backup-plan update is required
   for qsl-protocol governance-only closeout paths under `/srv/qbuild/work`,
   while any future real target, repository, tool install/use, credential, key
   material, recovery envelope, durable backup artifact, restore target,
   monitoring artifact, source-list, script, timer, fstab, system-service,
   backup, restore, deploy, rollback, or public-claim mutation remains
   backup-plan and local-ops authorization gated.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the allowed Packet T paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint against the closeout PR body.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- classifier proof for the changed path set.
- changed-line overclaim scan.

## CI Expectations

The closeout PR must merge normally only after required checks complete green,
including `public-safety`. No admin bypass, direct push, squash, rebase,
delete-branch flag, branch deletion command, branch-protection mutation, or
public-safety mutation is allowed.

## Successor Handoff

NA-0363 must begin as a no-secret target/tool implementation harness only. Any
later real off-host target, remote connection, repository init, tool
installation, key generation, key upload, passphrase collection, private-key
inspection, recovery envelope, off-host repository, restore target,
backup-plan update, local-ops mutation, retention/purge operation,
monitoring/alerting setup, deployment, rollback, real restore, backup
operation, or public-claim change remains blocked until exact future
authorization and no-secret evidence requirements are explicit.
