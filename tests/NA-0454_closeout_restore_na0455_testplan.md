Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0454 Closeout and NA-0455 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate closeout of NA-0454 after PR #1177 and restore the selected NA-0455
provider RNG fake/test-seam split-scope authorization successor without
implementing NA-0455.

## Required checks

- PR #1177 is MERGED.
- PR #1177 merge commit is `390fce0d5d8c`.
- Post-merge public-safety on `390fce0d5d8c` is completed success.
- NA-0454 is DONE.
- NA-0455 is READY.
- READY_COUNT is 1.
- D-0895 exists once.
- D-0896 exists once after the closeout patch.
- D-0897 is absent before future NA-0455 work.
- Duplicate decision count is zero.

## Closeout scope guard

Changed paths must be limited to:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0454_closeout_restore_na0455_testplan.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz
target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
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
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha 390fce0d5d8c --repo QuantumShieldLabs/qsl-protocol --report-only
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

## Future NA-0455 boundary

NA-0455 is split-scope authorization only.

NA-0455 may inspect qsc/refimpl/provider RNG surfaces read-only and must select
or reject exact future implementation paths before any fake, seam, executable
test, runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz, vector, or
formal-model mutation.

Provider-dependent qsc RNG and refimpl/provider RNG remain residual gaps after
NA-0454 closeout.

## Public claim boundary

NA-0454 closeout and NA-0455 restoration are internal governance evidence only.

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

NA-0454 is DONE and NA-0455 is the sole READY item. NA-0455 is not implemented
by this closeout.
