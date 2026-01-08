# QSL Protocol (Pre-Release)

QSL is a research-grade protocol specification and reference artifact set focused on post-quantum secure messaging primitives and conformance vectors.

## Status
- Pre-release / research / evolving.
- Demo and reference artifacts only; not production-ready.

## What’s in this repo
- `docs/canonical/` — normative protocol specifications.
- `docs/privacy/` — metadata posture and transport guidance.
- `docs/public/` — public-release runbook and scrub controls.
- `inputs/suite2/vectors/` — Suite-2 conformance vectors.
- `DECISIONS.md`, `TRACEABILITY.md`, `NEXT_ACTIONS.md` — governance and execution records.

## Build / test (local)
Run only what you need for the change you are making.
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/run_4b.sh` (Suite-2 interop harness)
- `./scripts/ci/metadata_conformance_smoke.sh` (demo relay posture)

## Security
See `SECURITY.md` for responsible disclosure.

## License
See `LICENSE` for terms.
