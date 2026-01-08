# QSL Protocol (Public Release — Draft)

QSL is a research-grade protocol specification and reference artifact set focused on post-quantum secure messaging primitives and conformance vectors.

## Status
- Draft / research; not production-ready.
- No formal security audit has been published for this release.

## Scope
- Suite-2 protocol specification and SCKA control plane (see `docs/canonical/`).
- Privacy and transport posture guidance (see `docs/privacy/`).
- Conformance vector corpus for Suite-2 (see `inputs/suite2/vectors/`).

## Repository map
- `docs/canonical/` — normative protocol specifications.
- `docs/privacy/` — metadata posture and transport guidance.
- `docs/public/` — public-release runbook and scrub controls.
- `inputs/suite2/vectors/` — Suite-2 conformance vectors.
- `DECISIONS.md`, `TRACEABILITY.md`, `NEXT_ACTIONS.md` — governance and execution records.

## How to use vectors (high level)
Use the Suite-2 vector corpus to validate that an implementation derives identical transcript bindings and key schedule outputs for the same inputs. The vectors are deterministic and intended for conformance checks, not performance benchmarking.

## Security
See `SECURITY.md` for responsible disclosure and reporting guidance.

## License
See `LICENSE` for terms.

## Public export note
This repository is generated from an allowlist export; no secrets or operational endpoints are intended to be present.
