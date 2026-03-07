# NA-0075 — qsc Relay Demo Transport Plan

## Scope & assumptions
- Relay-backed demo transport for qsc; charter-enforced explicit-only behavior.
- No implicit send/retry/recovery.

## Relay contract checklist (charter-mapped)
- Relay is a dumb pipe under hostile conditions.
- Deterministic markers for all relay events.
- No persistent mutation on transport failure.

## Fault injection matrix
- Drop: 0%, 10%, 30% (seeded)
- Jitter: 0ms, 50ms, 200ms (seeded)
- Reorder: window size 2, 5 (seeded)
- Duplicate: 0%, 5% (seeded)

## Test vectors
1) Drop failure: send attempt fails; no mutation; marker emitted.
2) Reorder: out-of-order delivery triggers explicit handling; no implicit recovery.
3) Duplicate delivery: idempotent reject; no mutation.
4) Jitter: deterministic replay with same seed yields identical markers.
5) Combined drop+reorder: explicit failure path only.
6) Seeded replay: marker sequence stable across runs.

## Deterministic marker expectations
- QSC_MARK/1 event=relay_* for all relay lifecycle events.
- prepare/attempt/commit markers remain explicit.

## Executed (Phase 1)
- Implemented relay serve/send commands with seeded fault injection.
- Added deterministic no-mutation tests for drop/dup.
- Added deterministic reorder tests (reorder-only, drop+reorder) and seeded replay checks:
  - reorder seed: 42, window=2
  - drop+reorder seed: deterministic search (drop_pct=35, window=2)
  - seeded replay: seed=123, window=2

## Evidence
- Commands and logs (isolated cargo cache):
  - OUT_DIR: /home/victor/work/qsl/_forensics/na0075_phase1_20260128T024744Z
  - cargo test -p qsc --locked |& tee /home/victor/work/qsl/_forensics/na0075_phase1_20260128T024744Z/test.txt
  - cargo clippy -p qsc --all-targets -- -D warnings |& tee /home/victor/work/qsl/_forensics/na0075_phase1_20260128T024744Z/clippy.txt
- Reorder/drop+reorder gate logs:
  - OUT_DIR: /home/victor/work/qsl/_forensics/na0074_qsc_clippyfix_20260128T032134Z
  - cargo test -p qsc --locked |& tee /home/victor/work/qsl/_forensics/na0074_qsc_clippyfix_20260128T032134Z/test.txt
  - cargo clippy -p qsc --all-targets -- -D warnings |& tee /home/victor/work/qsl/_forensics/na0074_qsc_clippyfix_20260128T032134Z/clippy.txt

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings
- CI required contexts green

## Rollback
- Disable relay subcommands; remove relay doc + plan; revert markers.
