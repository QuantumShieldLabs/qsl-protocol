Goals: G1, G2, G3, G4, G5

# Public Docs Canon And Evidence Landing Page

Status: Authoritative
Owner: QSL maintainers
Last-Updated: 2026-07-10
Replaces: n/a
Superseded-By: n/a

## What QSL Is

QSL is a research-stage post-quantum messaging effort that publishes its
security evidence, demo limits, and remaining gaps in the open. The public
claim is evidence-bound: every strong statement should point to a proof source
or to a visible `NOT_READY` boundary.

Start with:

- [README.md](../../README.md)
- [Current Progress](PROGRESS.md)
- [Release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md)
- [External review package](EXTERNAL_REVIEW_PACKAGE.md)
- [Website claim matrix](WEBSITE_CLAIM_MATRIX.md)
- [Traceability](../../TRACEABILITY.md)
- [Decisions](../../DECISIONS.md)

## Current Progress

The current Progress log's latest entry is the July 15, 2026 catch-up entry:

- [Progress index](PROGRESS.md)
- [QSL Progress - 2026-07-15](progress/2026-07-15.md)
- [QSL Progress - 2026-07-10](progress/2026-07-10.md)
- [QSL Progress - 2026-06-25](progress/2026-06-25.md)

The 2026-07-15 entry covers two arcs merged since 2026-07-10: the first public
record of ENG-0038 — a handshake authentication flaw found by internal
adversarial review, fixed, its class retired, and bounded-model-checked with
known unmodeled slices on the public ledger (no vulnerability-free or
unqualified formal-verification claim; external review remains uncommissioned) —
and the NA-0640..NA-0646 product-path arc (bounded full-stack e2e, durable relay
queue, opt-in acknowledged-pull client, TUI retirement, and the core extraction
to a linkable library), each with its bounded framing and with the claim
boundary unchanged. The 2026-07-10 entry summarizes the Suite-2
cryptographic-core completion arc (single-root DH+PQ composition, the CI-gated
ProVerif symbolic analysis, and the RFC 7748 §6.1 non-contributory-DH guard) and
the docs truth-up that records it, with the claim boundary unchanged.

Review is invited on the evidence, limits, claim boundaries, corrections,
residual gaps, and next steps. This section is an engineering-evidence summary:
no public readiness, no production readiness, no public internet readiness, no
external-review-complete claim, no crypto-complete claim, no identity-complete
claim, no trust-complete claim, no replay-proof claim, no downgrade-proof
claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto
claim.

## Current Bounded qsc Evidence

The latest repository sync adds public navigation for bounded qsc evidence, not
public readiness. The evidence now includes:

- direct qsc client-to-client E2EE work using synthetic data;
- same-host qsc tests and retained-qsc staging/restaging checks;
- SSH reverse-forward marker/ACK evidence and Build-to-Inspiron qsc E2EE
  success;
- selected wrong-peer, stale/replaced-peer, replay, and corrupt-delivery
  negative cases;
- repeated-run cleanup/freshness proof under a controlled lab setup;
- public-safety and advisories gates, including the quinn-proto
  RUSTSEC-2026-0185 remediation baseline;
- bounded formal/model checks, corpus validators, and secret-material scans.

Use the [release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md) for
the evidence-to-gap map and the
[external review package](EXTERNAL_REVIEW_PACKAGE.md) for reviewer orientation.
The implementation record is
[NA-0539 public evidence sync](../governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md).

## What QSL Is Not

- Not a production messaging service.
- Not a public readiness claim.
- Not a public internet service readiness claim.
- Not completed external review.
- Not crypto-complete, identity-complete, or trust-complete.
- Not replay-proof or downgrade-proof.
- Not secret-material-complete, not side-channel-free, not vulnerability-free,
  not bug-free, and not perfect-crypto.
- Not anonymity, metadata-free messaging, or untraceability.
- Not production deployment approval for qsl-server or qsl-attachments.
- Not evidence that runtime identifier rotation, runtime default padding,
  complete runtime sanitized-error coverage, or production retention/deletion
  behavior is complete.

## Evidence Map

Use the [release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md) for
the current `PROVEN`, `PARTIAL`, `DOCS_ONLY`, `FUTURE_GATE`, and `NOT_READY`
classification. It links goals G1 through G5 to current evidence, commands,
gaps, and next actions.

Key routes:

- Suite-2 and fail-closed evidence: [TRACEABILITY.md](../../TRACEABILITY.md)
- Formal/model checks: [formal README](../../formal/README.md)
- Claim-boundary package: [External review package](EXTERNAL_REVIEW_PACKAGE.md)
- Public visibility strategy: [Public attention and visibility strategy](PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md)
- Bounded claim wording: [Website claim matrix](WEBSITE_CLAIM_MATRIX.md)

## Demo Evidence

QSL demo evidence is non-production. It is useful because it shows bounded,
reproducible behavior and selected negative paths without treating the demo as
deployment proof.

- [Demo acceptance criteria](../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [Public demo touch and feel readiness](../demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [KT-negative public demo readiness](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md)
- [Attachment public demo readiness](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md)
- [Demo adversarial stress testing](../demo/DEMO_ADVERSARIAL_STRESS_TESTING.md)
- [Clean-host reviewer reproduction](../demo/CLEAN_HOST_REVIEWER_REPRODUCTION.md)

## Service Evidence

qsl-server and qsl-attachments evidence is local hardening and production-gate
mapping. It is not production relay, attachment service, public internet, or
managed-operations approval.

- [Service production-gate evidence map](../governance/evidence/NA-0287_service_production_gate_evidence_map.md)
- [Server and attachments production-boundary plan](QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md)
- [External review package service rows](EXTERNAL_REVIEW_PACKAGE.md)

## Metadata Phase-2 Evidence

Metadata phase-2 remains incomplete. Current proof is bounded to design and
policy-fixture harnesses.

- [Metadata phase-2 and external review gap plan](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md)
- [Identifier/padding design](../governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md)
- [Identifier/padding harness](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md)
- [Sanitized-error/retention design](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md)
- [Sanitized-error/retention harness](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md)

## External Review Package

The [external review package](EXTERNAL_REVIEW_PACKAGE.md) is reviewer
orientation material. It does not mean external review is complete. Treat it as
the route to accepted review scope, reviewed commit, findings, dispositions,
and residual-risk recording in a later evidence lane.

## Public Visibility Strategy

The [public attention and visibility strategy](PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md)
authorizes safe copy themes for later public surfaces:

- evidence over slogans;
- fail-closed behavior tied to proof;
- published limitations;
- non-production demo boundaries;
- service hardening as future-gated production evidence;
- metadata honesty without anonymity claims.

Website implementation remains future-gated. This repository update does not
edit the website or external website repository.

## Claim Boundaries

Safe wording must stay attached to evidence and gaps. Do not claim:

- production-ready QSL protocol;
- public internet service readiness;
- external review completion;
- anonymity;
- metadata-free messaging;
- untraceable messaging;
- quantum-proof communications;
- unbreakable or guaranteed secure messaging;
- proven true Triple Ratchet;
- production relay, attachment service, desktop, backup/restore, or deployment
  readiness.

Use [Suite-2 Triple-Ratchet claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md),
[Website claim matrix](WEBSITE_CLAIM_MATRIX.md), and
[Release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md) when writing
public copy.

## How To Help

- Review a claim in this page and follow it to the linked evidence.
- Run a bounded demo or reproduction command from the demo docs.
- Propose missing negative tests or clearer evidence receipts.
- File documentation fixes that keep `NOT_READY` boundaries visible.
- Use [CONTRIBUTING.md](../../CONTRIBUTING.md), [SUPPORT.md](../../SUPPORT.md),
  and [SECURITY.md](../../SECURITY.md) for contribution, support, and security
  reporting paths.

## Historical Public Docs

The following files are retained as compatibility stubs and should not be
treated as canonical:

- [Public release runbook](PUBLIC_RELEASE_RUNBOOK.md)
- [Public allowlist inventory](PUBLIC_ALLOWLIST_INVENTORY.md)
- [Public export manifest](PUBLIC_EXPORT_MANIFEST.md)
- [Public workspace and naming](PUBLIC_WORKSPACE_AND_NAMING.md)
