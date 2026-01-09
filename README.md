# QSL Protocol (Public Release — Draft)

QSL is a research-grade protocol specification and reference artifact set focused on post-quantum secure messaging primitives and conformance vectors.

## Status

- Draft / research; not audited; not production-ready.
- Protocol development is now public for scrutiny; breaking changes may occur.

## Repository contents (high-level)
- `START_HERE.md` — contributor entry point and operating rules.
- `specs/` — security objectives and protocol notes.
- `docs/` — documentation set (including `docs/INDEX.md` as the entry index).
- `inputs/` — test inputs and referenced bundles used by CI/harnesses.
- `tests/` — test plans and evidence scaffolding.
- `test-harness/` and `tests/harness/` — harness runners/adapters.
- `apps/qshield-cli/` — demo/reference CLI (non-production).
- `formal/` — bounded formal model checks.
- `schemas/` — JSON schemas for vectors/interop sets.
- `scripts/ci/` — CI entry scripts and validators.

Historical note:
- `v0.1.x-draft` tags were earlier public “allowlist” cuts focused on docs + vectors. `v0.2.0-draft` is the public primary development cutover.

- Draft / research; not production-ready.
- No formal security audit has been published for this release.
- Latest tagged release: v0.2.0-draft (https://github.com/QuantumShieldLabs/qsl-protocol/releases/tag/v0.2.0-draft).

## Scope
- Suite-2 protocol specification and SCKA control plane (see `docs/canonical/`).
- Privacy and transport posture guidance (see `docs/privacy/`).
- Conformance vector corpus for Suite-2 (see `inputs/suite2/vectors/`).

## Repository map
- `docs/canonical/` — normative protocol specifications.
- `docs/privacy/` — metadata posture and transport guidance.
- `docs/public/` — public-release runbook and scrub controls.
- `docs/INDEX.md` — documentation index (entry point).
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
