# NA-0357 Closeout and NA-0358 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25
Replaces: n/a
Superseded-By: n/a

## Objective

Close NA-0357 after the restore-drill prerequisite plan merges, and restore
exactly one READY successor:
`NA-0358 -- Metadata Runtime Restore Drill Implementation Authorization Plan`.

## Protected Invariants

- NA-0357 is DONE.
- Exactly one READY item exists after closeout: NA-0358.
- D-0696 and D-0697 each exist once; D-0698 is absent.
- NA-0358 is not implemented by this closeout.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  mutated.
- No backup script, timer, fstab, local backup config, off-host target, key
  file, passphrase path, restore path, restore target, deploy path, rollback
  path, workflow, dependency, website, README, START_HERE, or docs/public path
  is changed.
- Local continuity is not described as complete disaster recovery.
- Off-host encrypted backup is not described as complete.
- Restore-drill prerequisite planning is not described as restore execution.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, hidden all metadata, restore-drill completion, key custody
  implementation, or disaster recovery completion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0357_closeout_restore_na0358_testplan.md`

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
  key-upload, private-key-inspection, passphrase-collection, restore-target,
  restore-copy, restore-mount, or off-host backup setup paths.

## Closeout Requirements

1. Record qsl-protocol PR #976 merge evidence for NA-0357.
2. Mark NA-0357 DONE.
3. Restore `NA-0358 -- Metadata Runtime Restore Drill Implementation
   Authorization Plan` as the sole READY item.
4. Add D-0697 with explicit no-NA-0358-implementation language.
5. Link D-0697, the closeout testplan, selected successor, and backup-plan
   impact in TRACEABILITY.
6. Keep backup-plan impact explicit: no current backup-plan update is required
   for qsl-protocol governance-only paths under `/srv/qbuild/work`, while any
   future restore target, durable restore artifact, key material, recovery
   envelope, repository, remote target, retention/purge policy, monitoring
   artifact, source-list, script, timer, fstab, system-service mutation,
   backup, restore, deploy, rollback, or public-claim change remains
   backup-plan and local-ops authorization gated.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the allowed Packet S paths.
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
delete-branch flag, branch-protection mutation, or public-safety mutation is
allowed.

## Successor Handoff

NA-0358 begins as restore-drill implementation authorization planning only. Any
later restore execution, restore target creation, restore mount/copy, off-host
implementation, key handling, recovery envelope creation, backup-plan update,
local-ops mutation, retention/purge operation, monitoring/alerting setup,
deployment, rollback, or public-claim change remains blocked until exact future
authorization and no-secret evidence requirements are explicit.
