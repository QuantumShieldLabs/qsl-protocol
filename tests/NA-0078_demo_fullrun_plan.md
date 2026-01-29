# NA-0078 Demo Full-Run Plan

## Scope & assumptions
- Local demo run uses qsc relay + two clients on localhost.
- Full run must not require sudo; no secrets in logs.

## Scenario matrix (seeded)
- happy-path (seed=1)
- drop+reorder (seed=7)

## Deterministic marker subset definition
- deterministic_subset.txt contains:
  - scenario, seed
  - marker counts for relay/alice/bob
  - event count summary derived from `event=` markers
- For same seed+scenario, deterministic_subset.txt must match exactly.

## CI strategy
- demo-packaging.yml runs a bounded full-run smoke with `timeout 60s` for happy-path.

## Verification checklist
- bash -n scripts/demo/qsc_demo_local.sh
- ./scripts/demo/qsc_demo_local.sh --help
- ./scripts/demo/qsc_demo_local.sh --dry-run --scenario drop+reorder --seed 7
- ./scripts/demo/qsc_demo_local.sh --scenario happy-path --seed 1 --out ./_demo_out/happy --timeout 30

## Executed evidence (NA-0078)
- bash -n scripts/demo/qsc_demo_local.sh
- ./scripts/demo/qsc_demo_local.sh --help
- ./scripts/demo/qsc_demo_local.sh --dry-run --scenario drop+reorder --seed 7
- ./scripts/demo/qsc_demo_local.sh --scenario happy-path --seed 1 --out ./_demo_out/happy --timeout 30

## Rollback
- Revert demo-packaging.yml full-run step and script changes.
