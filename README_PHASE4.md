Goals: G4

# QuantumShield Phase 4 Starter (Local + CI)

Classification: Supporting (developer quick start). This file does not define protocol semantics.
For authoritative workflow and “what to do next,” use START_HERE.md and NEXT_ACTIONS.md.

This scaffold runs Phase 4A gates locally on Ubuntu and can be wired into CI.

## Quick start (Ubuntu)

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements-ci.txt

scripts/ci/run_4a.sh
```

Outputs land in:
- `artifacts/<RUN_ID>/4A/*`
- `artifacts/<RUN_ID>/phase4_errata/*` (if Phase 3 ledger references missing/mismatched packaged items)

## Notes

- Phase 2 canonical specs are frozen. Do not modify QSP/QSE. Conflicts are reported as Phase 3 errata.
- Phase 3 lock (`inputs/phase3/phase3.lock.json`) is generated from the Phase 3 ZIP and used for deterministic verification (including nested zips).

