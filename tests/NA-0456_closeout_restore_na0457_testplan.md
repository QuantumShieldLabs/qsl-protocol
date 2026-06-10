Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0456 Closeout and NA-0457 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate closeout of NA-0456 after the evidence PR merged and restore the
selected NA-0457 qsc provider RNG fake/test-seam strategy authorization lane as
the sole READY item without implementing NA-0457.

## Required checks

- PR #1181 merged at `9bf5e002657c`.
- PR #1181 head was `e153ceab27ac`.
- Post-merge public-safety completed success on `9bf5e002657c`.
- D-0899 exists once.
- D-0900 exists once.
- D-0901 is absent before a future NA-0457 directive.
- NA-0456 is DONE.
- NA-0457 is READY.
- READY_COUNT is exactly one.
- No implementation mutation occurs.

## Scope guard

Changed paths must be limited to:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0456_closeout_restore_na0457_testplan.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree mutation is allowed.

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

NA-0456 closeout and NA-0457 restoration are bounded internal evidence only.

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

NA-0456 is DONE, NA-0457 is READY, D-0900 exists once, D-0901 is absent, and
exactly one READY item remains.
