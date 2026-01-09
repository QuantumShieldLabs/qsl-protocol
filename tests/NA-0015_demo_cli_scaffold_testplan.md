# NA-0015 — Demo CLI scaffold (coverage note)

Goals: G5 (supports G1–G4)

This file exists to satisfy the goal-lint coupling rule for NA-0015 PR1.
It records that the demo CLI scaffold adds non-production UX surfaces only
and does not change protocol behavior.

Executable coverage (PR3):
- Demo CLI crypto smoke: `scripts/ci/demo_cli_smoke.sh` (two stores, register, establish, send, recv).
- Sessionful Suite-2 establish → send/recv demo script.
- Local relay request/response flow verification.

References:
- NEXT_ACTIONS.md: NA-0015
- apps/qshield-cli/README.md
