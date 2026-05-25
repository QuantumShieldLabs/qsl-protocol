# NA-0355 Closeout and NA-0356 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-24
Replaces: n/a
Superseded-By: n/a

## Objective

Close NA-0355 after the off-host encrypted backup target/tool selection plan
merges, and restore exactly one READY successor:
`NA-0356 -- Metadata Runtime Key Custody / Key Recovery Prerequisite Plan`.

## Protected Invariants

- NA-0355 is DONE.
- Exactly one READY item exists after closeout: NA-0356.
- D-0692 and D-0693 each exist once; D-0694 is absent.
- NA-0356 is not implemented by this closeout.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  mutated.
- No backup script, timer, fstab, local backup config, off-host target, key
  file, passphrase path, restore path, deploy path, rollback path, workflow,
  dependency, website, README, START_HERE, or docs/public path is changed.
- Local continuity is not described as complete disaster recovery.
- Off-host encrypted backup is not described as complete.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, or hidden all metadata.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0355_closeout_restore_na0356_testplan.md`

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
  key-upload, or off-host backup setup paths.

## Closeout Requirements

1. Record qsl-protocol PR #972 merge evidence for NA-0355.
2. Mark NA-0355 DONE.
3. Restore `NA-0356 -- Metadata Runtime Key Custody / Key Recovery
   Prerequisite Plan` as the sole READY item.
4. Add D-0693 with explicit no-NA-0356-implementation language.
5. Link D-0693, the closeout testplan, selected successor, and backup-plan
   impact in TRACEABILITY.
6. Keep backup-plan impact explicit: no current backup-plan update is required
   for qsl-protocol governance-only paths under `/srv/qbuild/work`, while any
   future off-host target/tool/key operation remains backup-plan and local-ops
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
delete-branch flag, branch-protection mutation, or public-safety mutation is
allowed.

## Successor Handoff

NA-0356 must begin as a key custody/key recovery prerequisite plan only. Any
later off-host implementation remains blocked until key custody, key recovery,
secret handling, backup-plan update, local-ops authorization, restore drill,
retention/purge, monitoring/alerting, operator runbook boundaries, and exact
implementation authorization are explicit.
