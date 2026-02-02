# NA-0090 — Remote scenario fault injection plan

## Scope & assumptions

## Fault injection algorithm (deterministic mapping)

## Marker schema (relay_event)

## Test vectors (happy vs drop-reorder)

## Remote demo validation steps

## Rollback

## Executed evidence

- Added tests:
  - qsl/qsl-client/qsc/tests/remote_fault_injection.rs
    - remote_scenario_happy_path_has_deliver_only
    - remote_scenario_drop_reorder_emits_hostile_markers
    - remote_scenario_determinism_replay
- Fault action mapping:
  - scenario=drop-reorder, idx = 1..N, k = seed + idx
  - k % 4 == 0 → action=reorder
  - k % 4 == 1 → action=drop
  - else → deliver (no extra marker)
- Commands (local gates):
  - cargo fmt -p qsc -- --check
  - cargo test -p qsc --locked
  - cargo clippy -p qsc --all-targets -- -D warnings
