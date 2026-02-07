# NA-0103 Metadata Minimization Plan

## Scope & assumptions
- Client-only changes in qsc.
- No server/workflow changes.

## Threat model (what metadata we reduce)
- Timing, size, and batch frequency observability.

## Fixed-interval polling design (bounds + costs)
- Explicit-only; bounded interval and max polls.

## Padding/bucketing scheme (size classes)
- Document size classes and max padding overhead.

## Batching rules (max batch size/count)
- Bounded; deterministic for CI/demo mode.

## Deterministic mode (seeded)
- Seeded, no wall-clock leakage in artifacts unless explicitly enabled.

## Marker schema expectations
- Markers include cadence/bucket/batch with deterministic fields.

## Test vectors + no-mutation checks
- Bounds enforced; determinism; no mutation on reject; no secrets.

## Verification checklist + rollback
- qsc fmt/test/clippy; CI green.
- Revert CLI flags/tests if needed.
