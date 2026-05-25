# NA-0359 Closeout and NA-0360 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25
Replaces: n/a
Superseded-By: n/a

## Objective

Validate that NA-0359 closes only after the qsl-protocol no-secret
restore-drill dry-run harness merges with required checks green, and that the
queue restores exactly one READY successor:
`NA-0360 -- Metadata Runtime Key Custody / Key Recovery Implementation
Authorization Plan`.

## Protected Invariants

- NA-0359 is DONE.
- Exactly one READY item exists after closeout: NA-0360.
- D-0700 and D-0701 each exist once; D-0702 is absent.
- NA-0360 is not implemented by this closeout.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  mutated.
- No backup script, timer, fstab, local backup config, off-host target, key
  file, passphrase path, restore path, deploy path, rollback path, workflow,
  dependency, website, README, START_HERE, docs/public path, branch-protection,
  public-safety configuration, service path, or secret-handling path is changed.
- No backup, restore, deploy, rollback, off-host operation, restore target
  creation/mount/copy, key generation, key upload, passphrase collection,
  private-key inspection, or secret material handling is performed.
- Local continuity is not described as complete disaster recovery.
- Off-host encrypted backup is not described as complete.
- The NA-0359 dry-run harness is not described as real restore execution.
- Key custody/key recovery is not described as implemented.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, hidden all metadata, restore-drill completion, or disaster-recovery
  completion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0359_closeout_restore_na0360_testplan.md`

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
  key-upload, private-key-inspection, passphrase-collection, or off-host backup
  setup paths.

## Closeout Requirements

1. Record qsl-protocol PR #980 merge evidence for NA-0359.
2. Mark NA-0359 DONE.
3. Restore `NA-0360 -- Metadata Runtime Key Custody / Key Recovery
   Implementation Authorization Plan` as the sole READY item.
4. Add D-0701 with explicit no-NA-0360-implementation language.
5. Link D-0701, D-0700, this testplan, selected successor, PR #980, and
   backup-plan impact in TRACEABILITY.
6. Keep backup-plan impact explicit: no current backup-plan update is required
   for qsl-protocol governance-only closeout paths under `/srv/qbuild/work`,
   while any future key custody/recovery, recovery envelope, restore drill,
   off-host target/tool implementation, durable artifact, monitoring artifact,
   source-list, script, timer, fstab, system-service, backup, restore, deploy,
   rollback, or public-claim mutation remains backup-plan and local-ops
   authorization gated.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the allowed Packet Q paths.
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

NA-0360 must begin as a key custody / key recovery implementation
authorization plan only. Any later secret material, key generation, key upload,
passphrase collection, private-key inspection, recovery envelope, off-host
target, restore target, backup-plan update, local-ops mutation,
retention/purge operation, monitoring/alerting setup, deployment, rollback,
real restore, backup operation, or public-claim change remains blocked until
exact future authorization and no-secret evidence requirements are explicit.
