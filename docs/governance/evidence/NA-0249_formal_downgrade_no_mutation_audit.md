Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04

# NA-0249 Formal Downgrade No-Mutation Audit

Goals: G3, G4

## Objective

Record the executable formal/model expansion for Suite-2 downgrade resistance and no-state-mutation reject invariants. This lane adds model evidence only; it does not change protocol, runtime, crypto, demo, service, website, CI, branch-protection, public-safety, Cargo, qsc/qsl app, qsc-desktop, qsl-server, qsl-attachments, KT implementation, or SCKA implementation behavior.

## Model Surface Added

`formal/model_suite2_negotiation_bounded.py` adds a bounded, zero-dependency Python model for the Suite-2 required negotiation slice described by `DOC-CAN-003` section 2.

The model enumerates the case where both peers support Suite-2 and explores:

- committed local/peer Suite-2 capability bits;
- negotiated suite values for Suite-2, Suite-1B, and an unknown-suite placeholder;
- local and peer transcript-bound negotiated-suite views;
- an empty negotiation state and an already accepted Suite-2 state.

The model accepts only the exact Suite-2 case where:

- both committed capability bits match the actual Suite-2 support state;
- the negotiated suite is Suite-2; and
- both transcript views match the negotiated Suite-2 value.

It rejects:

- weaker or unknown committed suites with `REJECT_S2_SUITE_MISMATCH`;
- inconsistent capability commitments with `REJECT_S2_CAPABILITY_COMMITMENT_MISMATCH`;
- inconsistent transcript suite views with `REJECT_S2_AD_MISMATCH`.

## No-State-Mutation Proof

The negotiation model snapshots modeled accepted/durable state as:

```text
(accepted_suite, accepted_capability_commitment, durable_accept_count)
```

For every rejected attempt, the model repeats the same invalid input and asserts:

- deterministic repeated reject outcome;
- returned state equals the pre-reject state;
- restored snapshot equals the pre-reject snapshot;
- no accepted-suite or durable accept-count mutation occurs.

The existing SCKA bounded model also now asserts explicit party snapshot equality and durable-record equality on reject, in addition to full party equality.

## Model Stats

Local command:

```bash
python3 formal/run_model_checks.py
```

Observed output during implementation:

```text
OK: SCKA bounded model checks passed
Explored states: 926
Transitions: 925
Unique visited: 926
OK: Suite-2 negotiation downgrade/no-mutation model checks passed
Negotiation attempts: 108
Accepted outcomes: 2
Rejected outcomes: 214
Downgrade rejects: 4
Capability commitment rejects: 162
AD mismatch rejects: 48
No-mutation assertions: 428
```

The two accepted outcomes are the exact valid Suite-2 attempt applied to the empty and already accepted modeled states. All weaker-suite, capability-mismatch, and transcript-suite mismatch attempts reject.

## Existing Formal No-Regression

`formal/run_model_checks.py` still runs the existing bounded SCKA model through the same CI entry point. The runner now executes both:

- `formal.model_scka_bounded.explore(...)`;
- `formal.model_suite2_negotiation_bounded.check_suite2_negotiation_model()`.

No workflow or public-safety configuration changed.

## Scope And Limitations

This is a faithful formal approximation of the current Suite-2 required negotiation rule, not a full production proof.

The new model:

- abstracts authenticated capability evidence into capability commitment booleans;
- abstracts transcript/AD binding into local and peer suite views;
- does not model AEAD, KDFs, transcript hashing, secrecy, authentication, or key schedule security;
- does not claim coverage for non-Suite-2 fallback lanes where Suite-2 is not mutually supported;
- does not replace conformance vectors or refimpl reject tests.

These limitations are intentional. The model claim is limited to downgrade reject and no-state-mutation behavior for the mutually Suite-2-capable, Suite-2-required negotiation slice.

## Repo Evidence Mapping

- `GOALS.md` G3 requires fail-closed downgrade resistance.
- `GOALS.md` G4 requires maintained executable model checks as a release gate.
- `DOC-CAN-003` section 2 defines the Suite-2 no-implicit-downgrade and AD mismatch reject requirements.
- `TRACEABILITY.md` maps G3 downgrade resistance to CAT-S2-DOWNGRADE-001 and existing refimpl/vector evidence.
- NA-0240 added bounded formal SCKA persistence/rollback/no-mutation model evidence.
- NA-0241 added executable downgrade/transcript and capability reject no-mutation evidence.
- NA-0243 added durable Suite-2 receive/decrypt no-mutation evidence.
- NA-0248 identified formal verification expansion for downgrade/no-mutation invariants as a release-readiness gap.

## Commands

Baseline pre-edit checks passed:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Expected final validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
```

Also run:

- canonical queue parser;
- canonical decision parser;
- goal-lint using a synthetic PR event payload;
- markdown inventory and link validation;
- leak-safe added-line scan.

Post-edit local validation passed on 2026-05-04:

- `git diff --cached --check`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- canonical queue parser
- canonical decision parser
- manual markdown link-integrity runbook with `TOTAL_MISSING 0`
- staged added-line leak-safe scan with `v1_path_pattern count 0`, `hex32plus_pattern count 0`, and `sensitive_marker count 0`

## Recommendations

- Keep future formal claims narrow and tied to executable model states, transitions, and limitations.
- If later work needs a full authenticated transcript model, introduce it as a separate lane with explicit scope rather than widening this bounded model.
- Continue binding formal evidence to concrete vector/refimpl reject tests in later lanes when the harness surface is already available and the added test would not require implementation drift.
