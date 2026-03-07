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

- Remote smoke validation (local relay, deterministic):
  - scripts/demo/qsc_remote_relay_smoke.sh now runs bounded multi-send and captures QSC_MARK relay_event lines.
  - script clears outbox pre-flight via `qsc send abort` to avoid outbox_exists in happy-path.
  - if happy-path hits outbox_exists, script retries once after abort (bounded).
  - Happy-path (seed=1): deliver_count>0 and drop/reorder/dup=0.
  - Drop-reorder (seed=7): deliver_count>0 and (drop_count>0 or reorder_count>0).
  - Local relay validation commands (example):
    - RELAY_URL="http://127.0.0.1:<port>" ./scripts/demo/qsc_remote_relay_smoke.sh --scenario happy-path --seed 1 --out <out>
    - RELAY_URL="http://127.0.0.1:<port>" ./scripts/demo/qsc_remote_relay_smoke.sh --scenario drop-reorder --seed 7 --out <out>
