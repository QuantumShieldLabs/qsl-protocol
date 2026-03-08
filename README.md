# QSL Protocol

QSL is a research-stage, post-quantum-first secure messaging protocol and demo client effort.

This repository is public for review and iteration, but it is not production-ready.

## Start here
- Repository onboarding front door: `START_HERE.md`
- Documentation front door: `docs/INDEX.md`
- Canonical public/release posture: `docs/public/INDEX.md`

## What this project is
- A protocol and client effort focused on fail-closed security behavior.
- A transparency-first implementation path with deterministic test and CI evidence.
- A public documentation set that tracks current behavior, limitations, and safety posture.

## What this project is not
- Not a production messaging service.
- Not a completed security audit/certification artifact.
- Not a guarantee of feature parity with mature production messengers.

## Public semantics summary
- Trust model is per-device and fail-closed (`VERIFIED` is not `TRUSTED`).
- Delivery semantics are explicit: `accepted_by_relay` is distinct from `peer_confirmed`.
- Receipt policy modes are explicit: `off`, `batched`, `immediate`.
- Multi-device routing currently uses `primary_only` until fanout is explicitly introduced.

## Security and policy references
- Security reporting: `SECURITY.md`
- Support pathways: `SUPPORT.md`
- Contribution workflow: `CONTRIBUTING.md`
- Documentation map: `docs/DOCS_MAP.md`
