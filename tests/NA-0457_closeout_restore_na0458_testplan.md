Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0457 Closeout and NA-0458 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate closeout of NA-0457 after the evidence PR merged and restore the
selected NA-0458 qsc KEM provider RNG fake/test-seam implementation harness as
the sole READY item without implementing NA-0458.

## Required checks

- PR #1183 merged at `82d99b26a50d`.
- PR #1183 head was `0afe4cf65279`.
- Post-merge public-safety completed success on `82d99b26a50d`.
- D-0901 exists once.
- D-0902 exists once.
- D-0903 is absent before a future NA-0458 directive.
- NA-0457 is DONE.
- NA-0458 is READY.
- READY_COUNT is exactly one.
- No NA-0458 implementation mutation occurs.

## Selected successor

`NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`

Future NA-0458 exact scope is limited to:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Scope guard

Changed paths must be limited to:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0457_closeout_restore_na0458_testplan.md`

No qsc source, runtime, crypto, dependency, Cargo, lockfile, workflow,
executable test, fuzz target, vector, formal model, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume, qshell, backup, restore, qsl-backup, backup
status, backup plan, rollback, or backup tree mutation is allowed in this
closeout.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

## Public claim boundary

NA-0457 closeout and NA-0458 restoration are bounded internal evidence only.

No public-readiness claim is allowed.

No production-readiness claim is allowed.

No public-internet-readiness claim is allowed.

No external-review-complete claim is allowed.

No crypto-complete claim is allowed.

No RNG-failure-complete claim is allowed.

No provider-RNG-complete claim is allowed.

No side-channel-free claim is allowed.

No vulnerability-free claim is allowed.

No bug-free claim is allowed.

No perfect-crypto claim is allowed.

Cargo audit green remains dependency-health evidence only.

## Expected result

NA-0457 is DONE, NA-0458 is READY, D-0902 exists once, D-0903 is absent, and
exactly one READY item remains.
