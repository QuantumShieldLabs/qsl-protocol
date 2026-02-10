# QSC TUI Invariants

## Purpose

This document captures non-negotiable TUI safety and UX invariants for NA-0120 and implementation follow-on work.

## Must Never Happen

- Implicit send, retry, recover, or commit.
- Over-claiming delivery state in UI.
- Plaintext secrets or file contents written at rest by UI behavior.
- Secrets in UI markers/logs.
- Focus stealing from asynchronous updates.
- Content previews in navigation pane.
- Status spam outside Status domain boundaries.
- Multi-select outside Files/Logs contexts.

## Required Behavior Instead

- All sensitive or state-changing operations require explicit command-bar intent.
- Message state rendering follows NA-0118 truth model.
- File transfer state rendering follows NA-0119 truth model.
- Redaction is applied when locked/unavailable state is shown.
- Update bursts are bounded and represented as counters when not focused.
- Status information stays contained to Status panel semantics.

## Delivery Truth Invariant (NA-0118)

- UI must not show a stronger message state than the underlying persisted state machine.
- No `DELIVERED` claim without the explicit ACK-backed condition from NA-0118.

## File Truth Invariant (NA-0119)

- UI must not display complete/verified file status before integrity and bounded-transfer conditions are actually met.
- Partial transfers must remain partial; reject paths remain rejected.

## Lock/Redaction Invariant

- When locked, sensitive details are replaced by deterministic redactions.
- Unlock transitions are explicit and visible.

## Determinism Invariant

- UI event handling and command outcomes must remain deterministic for equivalent inputs.
- Headless marker contracts must remain stable.

## Scope Guard

These invariants govern UI behavior only and do not authorize protocol/server semantic changes.
