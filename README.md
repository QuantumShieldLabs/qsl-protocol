# QSL Protocol

QSL is a research-stage, post-quantum-first secure messaging protocol and demo
client effort.

This repository is public for review and iteration, but it is not production-ready.

## Why this matters

Post-quantum messaging needs more than swapping in new algorithms. A credible
protocol has to prove how negotiation fails closed, how replay and identity
boundaries behave, what metadata remains visible, how demos can be reproduced,
and where service hardening is still incomplete.

QSL is building that evidence in public: specifications, vectors, model checks,
demo acceptance criteria, and governance records are kept together so reviewers
can inspect both the design intent and the proof that currently exists.

## Start here
- Repository onboarding front door: [START_HERE.md](START_HERE.md)
- Documentation front door: [docs/INDEX.md](docs/INDEX.md)
- Canonical public/release posture: [docs/public/INDEX.md](docs/public/INDEX.md)
- Current evidence map: [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md)
- External review package: [docs/public/EXTERNAL_REVIEW_PACKAGE.md](docs/public/EXTERNAL_REVIEW_PACKAGE.md)
- Demo acceptance criteria: [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md)

## Public license and repo posture
- Source in this repository is public and licensed under `AGPL-3.0-only`; see `LICENSE`.
- This public repository includes specifications, conformance vectors, and research-stage reference implementations.
- Any future commercial services, hosted offerings, or support agreements are separate from this repository and do not replace the AGPL terms on the source published here.

## What this project is
- A protocol and client effort focused on fail-closed security behavior.
- A transparency-first implementation path with deterministic test and CI evidence.
- A public documentation set that tracks current behavior, limitations, and safety posture.
- A reviewer-facing evidence trail for Suite-2, SCKA, downgrade resistance,
  metadata minimization, demo reproduction, desktop prototype boundaries, and
  service-hardening gaps.

## What this project is not
- Not a production messaging service.
- Not a completed security audit/certification artifact.
- Not a guarantee of feature parity with mature production messengers.
- Not an anonymity system or metadata-free messaging claim.
- Not a claim that external review is complete.

## What is proven now
- Covered Suite-2 vector categories, bounded formal/model checks, and selected
  fail-closed negative paths are documented in the
  [release readiness evidence map](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md).
- The non-production demo acceptance path is documented in
  [demo acceptance criteria](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md) and related
  demo evidence.
- qsl-server and qsl-attachments work is tracked as service-boundary hardening,
  not production approval; see the
  [server and attachments production-boundary plan](docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md).

## How to inspect evidence
- Start with the evidence map, then read the
  [external review package](docs/public/EXTERNAL_REVIEW_PACKAGE.md) for the
  current reviewer bundle and known gaps.
- Use [TRACEABILITY.md](TRACEABILITY.md) to follow goals to specs, tests,
  implementation surfaces, and recent evidence.
- Use [DECISIONS.md](DECISIONS.md) to inspect accepted security and governance
  decisions, including claim boundaries and queue transitions.

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
