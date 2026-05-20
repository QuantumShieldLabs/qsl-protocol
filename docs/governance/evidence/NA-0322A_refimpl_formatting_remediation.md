Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322A Refimpl Formatting Remediation

Goals: G1, G2, G3, G4, G5

## Objective

Remediate the single pre-existing rustfmt drift that blocked NA-0322 workspace
formatting validation.

Exact formatted file:

`tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`

## Root Cause

During NA-0322 recovery, clean `origin/main` workspace formatting validation
failed only because the NA-0310 refimpl vector-oracle test split a string
constant across two lines. `cargo fmt --package qshield-cli -- --check` passed,
so the blocker was outside NA-0322 qshield/demo measurement scope.

## Before / After Formatting Proof

Before remediation, `cargo fmt --check` reported a diff only for:

`tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`

The rustfmt change joins the `VECTOR_FILE` string constant onto one line. The
diff is rustfmt-only. No semantic change is intended; the proof here is limited
to the textual rustfmt diff and the unchanged oracle test behavior.

After remediation:

- workspace `cargo fmt --check` must pass;
- the NA-0310 refimpl oracle test must pass;
- changed-path scope must include only the formatted refimpl test plus
  governance/evidence/testplan files.

## Scope Boundary

Allowed changed paths for this remediation PR:

- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `docs/governance/evidence/NA-0322A_refimpl_formatting_remediation.md`
- `tests/NA-0322A_refimpl_formatting_remediation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden:

- any other `tools/refimpl/**` file;
- qshield runtime source or tests;
- qsc/qsp/protocol/crypto/key-schedule implementation;
- qsl-server or qsl-attachments implementation;
- Cargo/dependency/workflow/public-safety changes;
- runtime timing mitigation, jitter, batching, cover traffic, or service timing
  implementation.

## Claim Boundary

This remediation is formatter-only. It does not implement timing measurement,
timing mitigation, traffic-shape mitigation, runtime instrumentation, qshield
behavior, qsl-server behavior, or qsl-attachments behavior.

It introduces no anonymity, metadata-free, untraceable, timing-hidden,
traffic-shape-hidden, production-readiness, public-internet-readiness, or
external-review-complete claim.

## Tests And Validation

Required local validation:

- `cargo fmt --check`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- queue/decision/scope/link/leak/goal-lint checks

## NA-0322 Resume Recommendation

After this remediation merges and post-merge public-safety is green, close
NA-0322A and restore NA-0322 as the sole READY item. NA-0322 should then use
D-0626 for its implementation decision and D-0627 for closeout, preserving
D-0623 through D-0625 for the NA-0322A prerequisite sequence.
