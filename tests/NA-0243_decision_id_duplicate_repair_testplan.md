Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0243 Decision-ID Duplicate Repair Test Plan

## Objective

Repair the historical duplicate D-0110 decision entry before NA-0243 implementation so decision IDs are unique and future governance entries can be allocated deterministically.

## Protected Invariant

- Exactly one decision entry may use a given `D-####` ID.
- The canonical earlier D-0110 store-safety decision remains unchanged.
- The later YubiKey/keyslot roadmap entry is renumbered to D-0451 without changing runtime, protocol, security, queue, or implementation semantics.

## Scope Guard

Allowed changed paths for this prerequisite repair:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0243_decision_id_duplicate_repair_testplan.md`

Forbidden changed paths include `NEXT_ACTIONS.md`, `.github/**`, `scripts/**`, Cargo manifests/locks, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, branch protection, public-safety configuration, and runtime/protocol/crypto/demo/service code.

## Duplicate D-0110 Proof Before Repair

The canonical parser must show:

```text
D-0110 2
D-0451 0
DUPLICATE_COUNT 1
DUPLICATE D-0110 2
```

The two entries are:

- `### D-0110` for QSC store safety policy.
- `- **ID:** D-0110` for staged QSC YubiKey/keyslot support.

## D-0451 Repair Proof After Patch

The canonical parser must show:

```text
D-0110 1
D-0451 1
D-0452 0
DUPLICATE_COUNT 0
```

## No Runtime / Protocol / Code Changes

This repair is governance-only. It must not touch runtime, protocol, crypto, demo, service, public-safety, branch-protection, Cargo, qsl-server, qsl-attachments, qsc-desktop, or website paths.

## Queue Unchanged

The queue parser must show:

```text
READY_COUNT 1
READY NA-0243 Skipped-Key and Receive-Decryption Reject No-Mutation Hardening
```

`NEXT_ACTIONS.md` must remain unchanged in Packet 0.

## Decision Parser Expected Output

The canonical decision parser must report one entry each for D-0110, D-0439 through D-0451, zero entries for D-0452 and D-0453, and zero duplicate decision IDs.

## CI Expectations

- `goal-lint` passes with `Goals: G4`.
- `git diff --check` passes.
- Markdown link validation passes.
- Leak-safe added-line scan reports no added sensitive markers.
- `cargo audit --deny warnings` passes.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` passes.
- Required GitHub contexts, including `public-safety`, pass normally before merge.
