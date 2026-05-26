Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-26

# NA-0365 Closeout Restore NA-0366 Testplan

## Objective

Close NA-0365 after the no-secret isolated restore fixture and harness merged,
and restore exactly one READY successor:
`NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool
Blocker Resolution`.

## Protected Invariants

- NA-0365 remains bounded qsl-protocol no-secret harness evidence only.
- NA-0366 is not implemented by this closeout.
- Exactly one READY item exists after the closeout patch.
- qsl-server, qsl-attachments, qshield runtime, qsc/qsp protocol behavior,
  dependencies, workflows, public docs, README, START_HERE, backup scripts,
  timers, fstab, service units, off-host targets, restore targets, keys,
  passphrases, recovery envelopes, deploy, rollback, backup, restore, and
  branch-protection settings are not changed.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  hidden-size, hidden-timing, hidden-traffic-shape, off-host-backup-complete,
  restore-drill-complete, real-restore-complete, real-key-custody-implemented,
  real-key-recovery-implemented, or disaster-recovery-complete claim is added.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0365_closeout_restore_na0366_testplan.md`

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
- runtime, protocol, crypto, demo, service, backup, restore, deploy, rollback,
  key, passphrase, recovery-envelope, branch-protection, or public-safety
  implementation paths

## Queue Requirements

- Before Packet Q, `READY_COUNT 1` and READY `NA-0365`.
- Before Packet Q, D-0712 exists once and D-0713 is absent.
- After patch, `READY_COUNT 1` and READY `NA-0366`.
- After patch, NA-0365 is DONE.
- After patch, D-0713 exists once and D-0714 is absent.

## Closeout Evidence Requirements

- Record qsl-protocol PR #992 merge evidence.
- Record the no-secret isolated restore harness result.
- Record the selected NA-0366 successor.
- Record that no NA-0366 implementation is authorized by the closeout.
- Record public-safety remains required and green before merge.

## Backup-Plan Impact Requirements

- No backup-plan update is required for this closeout because only
  qsl-protocol governance/testplan/journal paths are changed.
- Future real off-host target/tool work, real key custody/recovery work,
  real restore target work, source-list changes, scripts, timers, fstab,
  services, monitoring artifacts, durable backup/restore artifacts, deploy,
  rollback, and public-claim mutation remain backup-plan gated.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the Packet Q allow-list
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `bash scripts/ci/classify_ci_scope.sh <changed paths>`

## CI Expectations

- qsl-protocol branch protection must continue requiring `public-safety`.
- The closeout PR may merge only after required checks complete normally.
- The merge must use normal merge with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, or delete-branch flag is
  permitted.
- Post-merge public-safety must complete success on the closeout merge commit.

## Successor Handoff

NA-0366 starts from the real off-host encrypted backup target/tool blocker
resolution problem. It must not treat NA-0365 no-secret harness evidence as
real restore execution, real restore target creation, off-host backup
completion, real key custody, real key recovery, or disaster recovery
completion.
