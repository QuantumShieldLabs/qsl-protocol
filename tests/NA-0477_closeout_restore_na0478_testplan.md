Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0477 Closeout and NA-0478 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only closeout for NA-0477 and restoration of the exact
NA-0478 formal model implementation successor selected by D-0942.

## Preconditions

- NA-0477 authorization PR #1224 is merged.
- PR #1224 merge commit is `8719ef7e`.
- Post-merge public-safety on `8719ef7e` is completed success.
- D-0942 exists once.
- D-0943 is absent before this closeout patch.
- READY_COUNT is 1 before closeout and READY NA-0477.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden Scope

- no formal model mutation.
- no qsc runtime/source mutation.
- no crypto implementation mutation.
- no dependency, Cargo manifest, lockfile, or workflow mutation.
- no executable test, fuzz target, or vector mutation.
- no refimpl mutation.
- no qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
  public docs, README, or START_HERE mutation.
- no qwork, qstart, qresume, or qshell mutation.
- no backup, restore, qsl-backup, backup status, backup plan, rollback,
  systemd, timer, fstab, or backup tree mutation.
- no public technical paper work.
- no durable Director State Index output.

## Queue Checks

Required after patch:

- NA-0477 is DONE.
- NA-0478 is READY.
- READY_COUNT is 1.
- D-0942 exists once.
- D-0943 exists once.
- D-0944 is absent.
- duplicate decision count is zero.

## Successor Checks

Required selected successor:

`NA-0478 -- QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

Required future allowed paths:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_harness.md`
- `tests/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Local Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0477_closeout_restore_na0478_testplan.md
scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0477_closeout_restore_na0478_testplan.md
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
python3 formal/run_model_checks.py
```

## Public Claim Boundary

- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No formal-proof-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## PR Checks

The closeout PR body must include:

```md
Goals: G1, G2, G3, G4, G5

Impact:
Marks NA-0477 DONE and restores NA-0478 as the sole READY successor.

No-regression:
Preserves no-runtime/no-crypto/no-dependency/no-workflow/no-public-overclaim boundaries.

Tests/Vectors:
Governance closeout only; no executable tests or vectors are mutated.
```
