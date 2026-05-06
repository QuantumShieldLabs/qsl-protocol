Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-06
Replaces: n/a
Superseded-By: n/a

# NA-0251 Website Implementation Handoff Audit

Directive: QSL-DIR-2026-05-06-037 / NA-0251

## Objective

Prepare a bounded qsl-protocol handoff package for later public website implementation in the external website repository.

This audit records the evidence consulted, implementation risks, page-by-page handoff summary, safe/prohibited wording, evidence map, and limitations. It does not edit the external website repo or qsl-protocol website source.

## Artifacts Consulted

- [GOALS.md](../../../GOALS.md)
- [ROADMAP.md](../../../ROADMAP.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [DECISIONS.md](../../../DECISIONS.md)
- [Website claim matrix](../../public/WEBSITE_CLAIM_MATRIX.md)
- [Website update plan](../../public/WEBSITE_UPDATE_PLAN.md)
- [Suite-2 Triple-Ratchet claim boundary](../../public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md)
- [External review package](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [Release-readiness evidence map](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [Demo acceptance criteria](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [Conformance vector prioritization](../../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [Metadata threat model](../../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [Envelope transport profile](../../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [NA-0245 website truthfulness audit](NA-0245_website_truthfulness_audit.md)
- [NA-0250 external review release-readiness audit](NA-0250_external_review_release_readiness_audit.md)

## Website Implementation Risks

| Risk | Boundary |
| --- | --- |
| Production-readiness overclaim | QSL protocol remains research-stage and non-production. |
| "Proven true Triple Ratchet" overclaim | Current support is research-stage Triple-Ratchet-style wording only. |
| Quantum-proof overclaim | Current evidence supports bounded design and test evidence, not absolute security claims. |
| Metadata/privacy overclaim | Metadata minimization is supported; anonymity and metadata elimination are not. |
| External-product conflation | CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk calculator, playbook, and consulting must not be QSL protocol proof. |
| Demo/GUI overclaim | Demo and QSC desktop are non-production bounded surfaces with open KT-negative, attachment, and native-package gaps. |
| Roadmap/release-readiness drift | Partial release evidence must not be marketed as release approval. |
| Stale time-sensitive wording | Timelines, availability windows, and NIST/deadline claims need current source checks in the website lane. |

## Page-By-Page Handoff Summary

- Homepage: add a visible research-stage / non-production QSL protocol status band, separate live product claims, remove broad not-demo/not-prototype implications for QSL protocol, and add last-verified dates.
- Project/protocol status: add evidence-linked QSL protocol posture, G1/G3/G5 context, release-gate wording, and current gap visibility.
- Demo/GUI: state non-production demo acceptance and guided prototype boundaries; do not claim KT-negative demo, attachment demo, native package, production GUI, or active keychain readiness.
- Metadata/privacy: use metadata-minimization copy only; state the demo is not anonymity and residual metadata remains observable.
- Suite-2 / Triple-Ratchet: use research-stage Triple-Ratchet-style wording and prohibit proven/production/equivalence claims.
- External products: label CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk calculator, and consulting as external products/services unless direct qsl-protocol evidence exists.
- Healthcare/PQC consulting: keep service claims separate from protocol proof and make NIST/deadline copy precise and source-linked.
- Evidence links: add date-stamped links to goals, roadmap, decisions, traceability, demo, metadata, Suite-2 claim boundary, release-readiness map, external review package, and audits.
- Roadmap/release readiness: state partial release evidence and preserve open gaps.

## Safe Wording Summary

Safe copy may say:

- QSL protocol is research-stage and non-production.
- QSL Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design.
- Current evidence is release-gated and bounded to named repo artifacts.
- The demo is a non-production acceptance surface.
- QSL has a metadata minimization lane.
- The current demo profile is not an anonymity system and residual metadata remains observable.
- External products under the same brand do not prove QSL protocol production readiness.

## Prohibited Wording Summary

Do not use these for QSL protocol, Suite-2, demo, or GUI claims:

- production-ready
- proven true Triple Ratchet
- quantum-proof
- metadata-free
- anonymity
- anonymous messaging
- untraceable communications
- deployment-ready
- not demos / not prototypes
- external review complete
- attachment demo ready
- production relay ready

## Public Evidence Map Summary

The handoff maps each public website topic to a qsl-protocol evidence source:

- goals and release gates: GOALS.md
- current posture: ROADMAP.md
- governance decisions: DECISIONS.md
- goal-to-evidence mapping: TRACEABILITY.md
- website risk: WEBSITE_CLAIM_MATRIX.md and WEBSITE_UPDATE_PLAN.md
- Suite-2 public copy: SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
- release posture and reviewer package: RELEASE_READINESS_EVIDENCE_MAP.md and EXTERNAL_REVIEW_PACKAGE.md
- demo posture: DEMO_ACCEPTANCE_CRITERIA.md
- conformance priorities: CONFORMANCE_VECTOR_PRIORITIZATION.md
- metadata/privacy: DOC-G5-001 and DOC-G5-003
- prior audits: NA-0245 and NA-0250 evidence reports

## Explicit No External Repo Edits Statement

NA-0251 performed no external website repository edits. It did not clone, modify, push, deploy, or otherwise mutate the external website repository. It also did not edit qsl-protocol website source, protocol/runtime/crypto/demo/service code, qsc/qsl apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, `.github`, scripts, Cargo files, public-safety configuration, or branch-protection settings.

## Future Implementation Lane Recommendation

Recommended next external website lane:

- Title: Public Website Evidence-Boundary Implementation
- Scope: external website repository only
- First artifacts: static overclaim phrase scan, link check, screenshot review, claim-matrix MUST_FIX checklist, and scope proof
- Must protect: no QSL protocol production-readiness claim, no proven true Triple Ratchet claim, no quantum-proof claim, no anonymity or metadata-elimination claim, no external-product conflation, and no qsl-server/qsl-attachments/API conflation

## Known Limitations

- This handoff was prepared from qsl-protocol evidence, not from a fresh authenticated edit of the external website repository.
- Live website copy may have changed after the NA-0245 audit.
- External NIST/deadline wording must be rechecked in the website implementation lane.
- The handoff is not production release approval, external cryptographic review completion, or website deployment proof.
- KT-negative demo readiness, attachment demo readiness, native desktop package proof, production relay hardening, qsl-attachments hardening, metadata phase-2, and complete cross-host conformance reproduction remain open gaps.
