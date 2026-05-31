Status: Supporting
Owner: QSL Local Ops
Last-Updated: 2026-05-30

# NA-0390 Closeout / Restore NA-0391 Testplan

## Objective

Close NA-0390 after the routine audit cadence temp-output harness merged and
restore exactly one READY successor:

`NA-0391 -- QSL External Standards / Threat / Technology Watch Authorization Plan`

This closeout must not implement NA-0391.

## Protected Invariants

- Exactly one READY queue item exists after closeout.
- NA-0390 is DONE.
- D-0762 exists once.
- D-0763 exists once.
- D-0764 is absent.
- public-safety remains required and green.
- No runtime, service, protocol, crypto, workflow, dependency, backup script,
  timer, fstab, public docs, README, START_HERE, qsl-server, qsl-attachments,
  qshield runtime, or qsc-desktop path changes.
- No secret handling, off-host setup, restore operation, or backup mutation.
- No public, readiness, privacy, external-review, bug-free, or perfect-crypto
  claim expansion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0390_closeout_restore_na0391_testplan.md`

## Forbidden Scope

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
- `docs/public/**`
- `README.md`
- `START_HERE.md`
- backup scripts, timers, fstab, service units, source lists, keys, targets,
  restore paths, or monitoring configs

## Queue Validation Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- Expected:
  - `READY_COUNT 1`
  - `READY NA-0391`
  - NA-0390 is DONE in `NEXT_ACTIONS.md`

## Decision Validation Requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0762 --select D-0763 --select D-0764`
- Expected:
  - D-0762 count 1
  - D-0763 count 1
  - D-0764 count 0
  - duplicate decision count 0

## Scope Guard Requirements

- Changed files must be exactly the allowed closeout paths.
- Forbidden path count must be zero.
- No NA-0391 implementation path may be added.

## Link / Leak / Claim Requirements

- Link-check must report no missing local markdown links.
- Leak-scan must report zero high-confidence secret findings.
- PR body preflight and goal-lint must pass.
- Claim scan must allow only negated, future-gated, or boundary wording.

## CI Requirements

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- Required GitHub checks must complete green before merge.
- Post-merge public-safety on `origin/main` must complete success.

## Successor Handoff

NA-0391 may authorize a future read-only, source-cited external standards /
threat / technology watch process. This closeout does not browse the web, does
not perform that watch, and does not start public technical paper work.
