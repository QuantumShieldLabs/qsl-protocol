# DOC-QSC-004 Demo Full-Run Addendum v1.0.0 DRAFT

## Purpose
Define the full local demo run (relay + two clients) with deterministic, shareable logs under the Security Lens charter.

## Quickstart (full run)
```
./scripts/demo/qsc_demo_local.sh --scenario happy-path --seed 1
./scripts/demo/qsc_demo_local.sh --scenario drop+reorder --seed 7
```

## Scenarios (seeded)
- happy-path
- drop
- reorder
- drop+reorder
- seeded-replay

## Artifacts produced
- alice.markers
- bob.markers
- relay.log
- summary.txt (scenario + seed + outcome)

## Sharing guidance
Outputs are QSC_MARK lines only; safe to share (no secrets).

## Troubleshooting
If cargo cache permissions are restricted, use isolated CARGO_HOME/CARGO_TARGET_DIR under _forensics.
