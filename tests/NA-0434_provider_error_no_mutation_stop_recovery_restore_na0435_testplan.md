Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0434 Provider Error Stop Recovery and NA-0435 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that D278's NA-0434 implementation stop is preserved truthfully, that
NA-0434 is marked BLOCKED rather than DONE, and that NA-0435 is restored as the
sole READY successor without runtime, crypto, dependency, workflow, executable
test, fuzz target, or vector mutation.

## Protected invariants

- READY_COUNT remains 1.
- NA-0434 is BLOCKED, not DONE.
- NA-0435 is READY.
- D-0856 exists once and D-0857 remains absent.
- No duplicate decision IDs exist.
- D278 stop evidence is consumed without overstating implementation success.
- Public-claim boundaries remain conservative.

## Allowed scope

Allowed recovery mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0434_qsl_qsc_provider_error_no_mutation_test_implementation_stop_recovery.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0434_provider_error_no_mutation_stop_recovery_restore_na0435_testplan.md`

## Forbidden scope

Forbidden recovery mutation paths include runtime, crypto, dependency, Cargo,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, and `/backup/qsl` paths.

## D278 inheritance checks

- Confirm D278 response exists at
  `/home/victor/work/qsl/codex/responses/NA0434_20260607T013227Z_D278.md`.
- Confirm D278 proof root exists at
  `/srv/qbuild/tmp/NA0434_provider_error_no_mutation_test_impl_20260607T012707Z`.
- Confirm D278 reported STOP before repository mutation.
- Confirm the authorized test file was absent.
- Confirm D278 classification:
  `PROVIDER_ERROR_NO_MUTATION_RUNTIME_HOOK_NEEDED`.

## `pq_encap_failed` reachability evidence checks

- Confirm D278 provider probe reported:
  - `encap zero: None`
  - `encap ff: None`
  - `encap a5: None`
  - `encap inc: None`
- Confirm recovery evidence states wrong-length A1 KEM public keys are rejected
  during qsc frame decode before provider encapsulation.
- Confirm recovery evidence states correct-length malformed public-key byte
  patterns did not make `StdCrypto.encap` fail.

## `pq_decap_failed` partial feasibility evidence checks

- Confirm D278 provider probe reported:
  - `decap short sk: Some(InvalidKey)`
- Confirm recovery evidence treats this as partial feasibility only.
- Confirm recovery does not create a `pq_decap_failed`-only executable test.

## NA-0434 blocked-not-DONE check

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Expected: READY_COUNT 1 and READY NA-0435.
- Confirm `NEXT_ACTIONS.md` lists NA-0434 as BLOCKED.
- Confirm `NEXT_ACTIONS.md` does not list NA-0434 as DONE.

## NA-0435 READY check

- Confirm `NEXT_ACTIONS.md` contains:
  `NA-0435 -- QSL qsc Provider Error Path Test Hook / Defensive Branch Authorization Plan`.
- Confirm NA-0435 is the only READY item.
- Confirm NA-0435 forbids runtime, crypto, dependency, Cargo, lockfile,
  workflow, executable test, fuzz target, vector, public, backup, qsl-backup,
  status/plan, and qwork/qstart/qresume/qshell mutation.
- Confirm NA-0435 forbids public overclaim.

## qsl-protocol scope guard

- Run `git diff --name-only origin/main...HEAD`.
- Expected paths are exactly the six allowed recovery paths.
- Confirm no runtime/source/Cargo/lockfile/workflow/executable-test/fuzz-target
  or vector path is changed.

## dependency health checks

- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo tree -i rustls-webpki --locked`
- `cargo tree -i ml-kem --locked || true`
- `cargo tree -i pqcrypto-mlkem --locked || true`
- `cargo tree -i pqcrypto-traits --locked || true`
- `cargo tree -i pqcrypto-internals --locked || true`

Expected: root and nested audits are green; root pqcrypto package IDs are absent.

## public-safety requirements

- Verify public-safety is green before merge.
- Verify public-safety is green after merge.
- Do not use watch mode.
- Use bounded REST polling for PR and post-merge check proof.

## no public overclaim requirements

The recovery must introduce no public-readiness claim, no production-readiness
claim, no public-internet-readiness claim, no external-review completion claim,
no crypto-complete claim, no vulnerability-free claim, no perfect-crypto claim,
no side-channel-free claim, no bug-free claim, no off-host backup completion claim,
no disaster recovery completion claim, no restore proof claim, and no claim that
backup is complete.
