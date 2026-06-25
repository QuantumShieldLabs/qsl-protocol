# QSL Protocol

Goals: G1, G2, G3, G4, G5

**Post-quantum messaging needs evidence, not slogans. QSL is building that
evidence in public.**

QSL is a research-stage, post-quantum-first secure messaging protocol and demo
client effort. This public repository is for review, reproduction, and
iteration. It is not production-ready, public-ready, or a public internet
service readiness claim.

## Start With The Evidence

- Public evidence landing page: [docs/public/INDEX.md](docs/public/INDEX.md)
- Release-readiness map: [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md)
- External review package: [docs/public/EXTERNAL_REVIEW_PACKAGE.md](docs/public/EXTERNAL_REVIEW_PACKAGE.md)
- Demo acceptance criteria: [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md)
- Service production-gate map: [docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md](docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md)
- Metadata phase-2 evidence: [NA-0291 identifier/padding harness](docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) and [NA-0293 sanitized-error/retention harness](docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md)

## Why This Matters

Post-quantum messaging needs more than new algorithm labels. A credible
protocol has to prove how negotiation fails closed, how replay and identity
boundaries behave, what metadata remains visible, how demos can be reproduced,
and where service hardening is still incomplete.

QSL keeps those claims and gaps together: specifications, vectors, model
checks, demo acceptance criteria, service-boundary maps, and governance records
are linked so reviewers can inspect both the design intent and the proof that
currently exists.

## What This Is

- A research-stage protocol and client effort focused on fail-closed security
  behavior.
- A public evidence trail for Suite-2, SCKA, downgrade resistance, metadata
  minimization work, demos, and service-boundary hardening.
- A reviewer-facing repo that links claims to tests, vectors, model checks,
  evidence audits, decisions, and traceability records.
- A place to contribute evidence quality: negative tests, reproduction notes,
  documentation clarity, and claim-boundary review.

## What This Is Not

- Not a production messaging service.
- Not a public internet service readiness claim.
- Not a completed external security review or certification artifact.
- Not an anonymity system.
- Not metadata-free or untraceable messaging.
- Not a claim that metadata phase-2 is complete.
- Not a claim that runtime identifier rotation, runtime default padding,
  production retention/deletion behavior, or production service deployment is
  complete.
- Not a website or live marketing claim.

## What Is Proven Now

- Deterministic Suite-2 vector categories, reference implementation checks,
  bounded formal/model checks, and selected fail-closed negative paths are
  mapped in the [release-readiness evidence map](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md).
- Bounded qsc evidence now includes same-host qsc tests, a direct remote qsc
  E2EE workflow using synthetic data, retained-qsc staging/restaging checks,
  SSH reverse-forward marker/ACK proof, Build-to-Inspiron qsc E2EE success,
  selected wrong-peer and stale/replaced-peer negatives, selected replay and
  corrupt-delivery negatives, and repeated-run cleanup/freshness evidence.
- Non-production demos, stress/soak evidence, KT-negative demo proof, attachment
  demo proof, and reviewer reproduction paths are linked from the
  [demo acceptance criteria](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md) and the
  public evidence map.
- qsl-server and qsl-attachments have local hardening evidence and explicit
  production gates in the [service production-gate map](docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md).
- Metadata phase-2 has bounded fixture-harness proof for identifier/padding
  policy and sanitized-error/retention policy in [NA-0291](docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) and [NA-0293](docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md).

## What Is Not Proven Yet

- Production readiness.
- Public readiness.
- Public internet service readiness.
- External review completion.
- No crypto completeness, identity completeness, or trust completeness.
- No replay-proof or downgrade-proof status.
- No secret-material completeness, side-channel freedom, vulnerability freedom,
  bug freedom, or perfect crypto.
- Anonymity, metadata-free messaging, or untraceability.
- Runtime identifier rotation or runtime default padding beyond harness proof.
- Complete runtime sanitized-error coverage or production retention/deletion
  behavior.
- Production service deployment, observability, runbook, backup/restore,
  public-ingress, or incident-response proof.
- Website or live marketing claims backed by a separate website implementation
  lane.

## How To Review Or Help

- Inspect claims through [docs/public/INDEX.md](docs/public/INDEX.md), then
  follow each evidence link to the exact proof or gap.
- Run a bounded demo path from [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md).
- Review claim boundaries in [docs/public/EXTERNAL_REVIEW_PACKAGE.md](docs/public/EXTERNAL_REVIEW_PACKAGE.md).
- Propose missing negative tests, reproduction improvements, or clearer
  evidence navigation.
- Use [TRACEABILITY.md](TRACEABILITY.md), [DECISIONS.md](DECISIONS.md), and
  [NEXT_ACTIONS.md](NEXT_ACTIONS.md) to follow goals, accepted decisions, and
  the active queue.

## Public License And Repo Posture

- Source in this repository is public and licensed under `AGPL-3.0-only`; see
  `LICENSE`.
- This public repository includes specifications, conformance vectors, and
  research-stage reference implementations.
- Any future commercial services, hosted offerings, or support agreements are
  separate from this repository and do not replace the AGPL terms on the source
  published here.

## Public Semantics Summary

- Trust model is per-device and fail-closed (`VERIFIED` is not `TRUSTED`).
- Delivery semantics are explicit: `accepted_by_relay` is distinct from
  `peer_confirmed`.
- Receipt policy modes are explicit: `off`, `batched`, `immediate`.
- Multi-device routing currently uses `primary_only` until fanout is explicitly
  introduced.

## Security And Policy References

- Security reporting: [SECURITY.md](SECURITY.md)
- Support pathways: [SUPPORT.md](SUPPORT.md)
- Contribution workflow: [CONTRIBUTING.md](CONTRIBUTING.md)
- Documentation map: [docs/DOCS_MAP.md](docs/DOCS_MAP.md)
