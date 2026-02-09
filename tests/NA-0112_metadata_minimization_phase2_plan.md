# NA-0112 Metadata Minimization Phase 2 Plan

## Scope & assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- No server-side or workflow changes are in scope.
- Phase 2 extends existing metadata-minimization controls with deterministic, bounded, explicit knobs.

## Threat model notes (timing/size/burst)
- Timing leakage: observer infers activity from poll/send cadence.
- Size leakage: observer infers content classes from payload length.
- Burst leakage: observer infers user behavior from queue/batch spikes.
- Receipt-class leakage: observer distinguishes ACK/receipt traffic by class/shape.

## Parameter table (defaults + max bounds)
- Poll interval:
  - default: deterministic fixed interval (implementation-defined explicit default)
  - max bound: finite, validated upper bound (reject on exceed)
- Size buckets:
  - default: explicit standard bucket class
  - max bound: finite max pad target/bucket class
- Batch count:
  - default: bounded count per tick
  - max bound: strict maximum batch count
- Batch latency:
  - default: bounded latency window
  - max bound: strict maximum latency
- Cover traffic:
  - default: disabled
  - when enabled: explicit, deterministic, bounded, and marked

## Marker schema expectations
- `QSC_MARK/1 event=meta_tick tick=<n> interval_ms=<ms>`
- `QSC_MARK/1 event=meta_bucket bucket=<name_or_size>`
- `QSC_MARK/1 event=meta_batch count=<n> bytes=<n>`
- `QSC_MARK/1 event=meta_cover enabled=true`
- Reject paths emit deterministic `event=error code=<...>` markers and do not mutate state.

## Test vectors (deterministic replay, bounds, reject/no-mutation)
- Deterministic replay:
  - same seed/inputs produce identical `meta_tick`/`meta_bucket`/`meta_batch` marker sequence.
- Bounds enforcement:
  - reject when batch count/latency exceeds limits.
  - reject when bucket/pad target exceeds allowed bound.
- Reject/no-mutation:
  - on parameter reject, no queue/state mutation occurs.
- No-secret output:
  - marker/output scan excludes token/secret/private credential leakage.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Regression suite proves determinism, bounds, reject/no-mutation, and no-secret outputs.

## Rollback
- Revert NA-0112 implementation commits if determinism or bounds invariants regress.
- Preserve fail-closed defaults (no silent cover traffic, bounded/explicit controls only).
