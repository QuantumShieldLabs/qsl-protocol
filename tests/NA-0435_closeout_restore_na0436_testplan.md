# NA-0435 Closeout and NA-0436 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0435 closes only after PR #1139 merges and post-merge
public-safety is green, and that the selected NA-0436 narrowed decap-only test
implementation harness is restored as the sole READY item without implementing
NA-0436.

## Preconditions

- PR #1139 is MERGED.
- PR #1139 merge commit is `0f2c841d2707`.
- Post-merge public-safety completed success on `0f2c841d2707`.
- Post-merge `qsc-adversarial-smoke` completed success on `0f2c841d2707`.
- Before closeout, READY_COUNT is 1 and READY is NA-0435.
- D-0857 exists once.
- D-0858 is absent before closeout.
- NA-0434 remains BLOCKED.

## Allowed closeout scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0435_closeout_restore_na0436_testplan.md`

Forbidden changed paths include runtime, crypto, dependency, Cargo, lockfile,
workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback,
and `/backup/qsl` paths.

## Queue checks

- Confirm NA-0435 is marked DONE.
- Confirm NA-0434 remains BLOCKED.
- Confirm NA-0436 is READY.
- Confirm READY_COUNT is exactly 1.
- Confirm no NA-0436 implementation is included in this closeout.

## Decision checks

- Confirm D-0858 exists once.
- Confirm D-0859 is absent.
- Confirm no duplicate decision IDs exist.
- Confirm D-0858 records:
  - PR #1139 merge evidence.
  - post-merge public-safety success.
  - selected strategy `NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`.
  - selected successor `NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`.
  - no runtime/crypto/dependency/workflow/test/vector mutation in closeout.
  - no backup/restore.
  - no public overclaim.

## NA-0436 successor checks

Confirm restored NA-0436 includes:

- Objective to implement the exact NA-0435-authorized narrowed
  `pq_decap_failed` no-mutation test.
- Explicit `pq_encap_failed` caveat preservation.
- Allowed scope including only:
  - `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
  - qsl-protocol governance evidence/testplan paths for NA-0436
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Forbidden scope for runtime, crypto, dependency, Cargo, lockfile, workflow,
  fuzz target, vector, public surface, qsl-server, qsl-attachments, qshield
  runtime, backup/local-ops, qsl-backup, and qwork-tool mutation.
- Acceptance criteria requiring exact narrowed test pass, encap caveat
  preservation, dependency audits, public-safety, and exactly one READY item.

## Validation commands

Required local validation:

- `git diff --check`
- exact allowed-path scope guard
- queue helper
- decision proof
- link check
- leak scan
- contextual overclaim/claim-boundary scan
- PR body preflight
- goal-lint with standalone `Goals: G1, G2, G3, G4, G5`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo tree -i rustls-webpki --locked`
- `cargo tree -i ml-kem --locked`
- pqcrypto inverse tree absence probes
- `cargo fmt --check`

Required before PR merge:

- required PR checks are green, including public-safety.
- changed paths are exactly the five closeout paths.
- no NA-0436 implementation file is changed.
- no qwork, qstart, qresume, backup, restore, or sudo command was run.

## Public claim checks

- Confirm this closeout is internal governance evidence only.
- Confirm no public readiness, production readiness, public-internet readiness,
  external-review completion, crypto-complete, side-channel-free, bug-free,
  vulnerability-free, or perfect-crypto claim is made.
- Confirm cargo audit green is dependency-health evidence only.
