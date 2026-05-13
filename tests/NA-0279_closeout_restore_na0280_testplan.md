Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0279 Closeout and NA-0280 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0279 DONE after the qsl-server
rate/global-cap design PR merged and restores exactly one READY successor,
NA-0280, without implementing NA-0280.

## Protected invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0280.
- NA-0279 is DONE.
- D-0529 exists once.
- D-0528 remains present once.
- No qsl-server implementation change occurs in qsl-protocol closeout.
- No qsl-server tests/harness change occurs in qsl-protocol closeout.
- No qsl-attachments implementation change occurs.
- No production-readiness claim is introduced.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0279_closeout_restore_na0280_testplan.md`

## Forbidden scope

Forbidden paths include `.github/**`, `scripts/**`, `Cargo.toml`,
`Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`,
`tools/**`, `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
`qsl-attachments/**`, `website/**`, and any external website repository.

## Queue checks

Expected:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT `1`.
- READY is `NA-0280 — qsl-server Executable Rate-Limit / Global Route-Cap Harness`.
- NA-0279 is DONE.

## Decision checks

Expected:

- D-0528 exists once.
- D-0529 exists once.
- D-0530 is absent.
- duplicate decision count is zero.

## Scope checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0279_closeout_restore_na0280_testplan.md`

No implementation, workflow, script, Cargo, website, branch-protection, or
public-safety configuration paths are changed.

## Public-safety and cost-control checks

Expected:

- Starting post-Packet-E main public-safety is success.
- Closeout PR required checks attach and pass normally.
- Docs/governance-only cost-control may skip heavy full-suite jobs.
- Public-safety remains required before and after merge.

## Local validation

Run:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- overclaim scan for production and public-claim phrases
- branch inventory to confirm no branch deletion
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event or helper-supported equivalent.

## Success criteria

- Closeout PR merges normally.
- Post-merge main has READY_COUNT `1`, READY NA-0280, NA-0279 DONE, D-0529
  present once, and public-safety required/green.
- NA-0280 remains future executable qsl-server harness work and is not
  implemented inside the closeout.
