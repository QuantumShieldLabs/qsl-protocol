# NA-0077 Demo Packaging Plan

## Scope & assumptions

## Demo topology

## Scenario matrix (seeded)

## Deterministic marker expectations

## CI validation strategy

## Verification checklist

## Rollback

## Evidence

- bash -n scripts/demo/qsc_demo_local.sh
- ./scripts/demo/qsc_demo_local.sh --help
- ./scripts/demo/qsc_demo_local.sh --dry-run --scenario drop+reorder --seed 7
- Deterministic output: dry-run prints stable command lines for a given seed
