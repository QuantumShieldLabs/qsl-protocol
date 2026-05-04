Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04

# NA-0249 Formal Downgrade No-Mutation Test Plan

Goals: G3, G4

## Objective

Validate that NA-0249 expands executable formal/model-check evidence for Suite-2 downgrade resistance and no-state-mutation reject invariants without changing protocol implementation semantics or forbidden runtime/service surfaces.

## Protected Invariants

- If both peers support Suite-2, a weaker or inconsistent negotiated/committed suite must reject fail-closed.
- Rejected inputs must not mutate modeled accepted/durable state.
- Model claims must not outrun implementation evidence.
- `public-safety` remains required and green.
- `NA-0249` remains READY after Packet A; closeout is a separate packet.

## Scope Guard

Allowed changed paths:

- `formal/**`
- `tools/refimpl/quantumshield_refimpl/tests/**` only if directly required to bind model evidence to existing behavior
- `docs/governance/evidence/NA-0249_formal_downgrade_no_mutation_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0249_formal_downgrade_no_mutation_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent with evidence pattern

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo metadata, qsp, qsc/qsl/qsl-client implementation paths, apps, tools/refimpl source, tools/actors, inputs, qsc-desktop, qsl-server, qsl-attachments, website, protocol runtime implementation, KT implementation, SCKA implementation, public-safety helper/configuration, and branch-protection settings.

## Formal Model Checks

Run:

```bash
python3 formal/run_model_checks.py
```

Acceptance:

- the existing bounded SCKA model remains green;
- the new Suite-2 negotiation model runs from the same entry point;
- the output reports downgrade rejects, capability commitment rejects, AD mismatch rejects, and no-mutation assertions;
- any assertion failure exits nonzero.

## Downgrade Reject Model Proof

`formal/model_suite2_negotiation_bounded.py` must enumerate mutually Suite-2-capable attempts and assert that weaker or unknown negotiated suite values reject with `REJECT_S2_SUITE_MISMATCH`.

Acceptance:

- valid Suite-2 commitments are the only accepted attempts;
- downgrade reject count is greater than zero;
- rejected downgrade attempts leave accepted/durable state unchanged.

## Capability And Transcript Mismatch Proof

The model must assert deterministic reject for:

- committed capability bits that do not match actual Suite-2 support;
- local or peer transcript suite views that do not match the negotiated suite.

Acceptance:

- capability commitment reject count is greater than zero;
- AD mismatch reject count is greater than zero;
- repeated invalid input produces the same reject reason and same state snapshot.

## Existing SCKA No-Mutation Proof

The existing SCKA bounded model must continue to assert reject no-mutation. NA-0249 additionally requires explicit party snapshot equality and durable-record equality on rejected delivery.

Acceptance:

- `formal/model_scka_bounded.py` rejects still return the unchanged world;
- reject paths compare both modeled party snapshots and durable records;
- existing SCKA stats remain stable unless intentionally changed by a later decision.

## Governance Checks

The canonical queue parser must report:

```text
READY_COUNT 1
READY NA-0249 Formal Verification Expansion for Suite-2 Downgrade and No-Mutation Invariants
```

The canonical decision parser must report:

- D-0110 exists once.
- D-0439 through D-0464 exist once each.
- D-0465 is absent.
- No duplicate decision IDs exist.

## Required Local Validation

Run:

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

- goal-lint using a synthetic PR event payload;
- markdown inventory and link validation runbook;
- leak-safe added-line scan if established.

## PR And CI Acceptance

Acceptance:

- changed paths stay inside the NA-0249 allowlist;
- no forbidden paths are touched;
- all required CI contexts attach and pass normally;
- `formal-scka-model` remains green;
- `public-safety` remains required and green;
- no admin bypass, direct push, check spoofing, branch-protection exception, squash merge, or rebase merge is used.
