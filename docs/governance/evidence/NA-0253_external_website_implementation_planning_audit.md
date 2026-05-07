Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0253 External Website Implementation Planning Audit

Directive: QSL-DIR-2026-05-07-042 / NA-0253

## Objective

Convert the NA-0251 qsl-protocol website handoff into an operator-ready
external website implementation directive package, without editing the external
website repository or qsl-protocol website source.

## Artifacts Consulted

- [GOALS.md](../../../GOALS.md)
- [ROADMAP.md](../../../ROADMAP.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [Website implementation handoff](../../public/WEBSITE_IMPLEMENTATION_HANDOFF.md)
- [Website claim matrix](../../public/WEBSITE_CLAIM_MATRIX.md)
- [Website update plan](../../public/WEBSITE_UPDATE_PLAN.md)
- [Suite-2 Triple-Ratchet claim boundary](../../public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md)
- [External review package](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [Release-readiness evidence map](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [Demo acceptance criteria](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [Conformance vector prioritization](../../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [Metadata threat model](../../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [Envelope transport profile](../../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [NA-0251 handoff audit](NA-0251_website_implementation_handoff_audit.md)

## Implementation Risks

| Risk | Boundary |
| --- | --- |
| Production-readiness overclaim | QSL protocol remains research-stage and non-production. |
| Proven true Triple Ratchet overclaim | Current support is research-stage Triple-Ratchet-style wording only. |
| Quantum-proof overclaim | Current evidence supports bounded design/test claims, not absolute security claims. |
| Metadata/privacy overclaim | Metadata minimization is supported; anonymity and metadata elimination are not. |
| External-product conflation | CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk calculator, playbook, and consulting must not be QSL protocol proof. |
| Demo/GUI overclaim | Demo and QSC desktop are non-production bounded surfaces with open KT-negative, attachment, and native-package gaps. |
| Roadmap/release-readiness drift | Partial evidence must not be marketed as release approval. |
| Stale time-sensitive wording | Timelines, availability windows, and NIST/deadline claims need current source checks in the website lane. |

## Page-By-Page Planning Summary

- Homepage: add a visible QSL protocol research-stage / non-production status
  band, separate live product claims, avoid broad not-demo/not-prototype wording
  for QSL protocol, and add last-verified dates.
- Protocol status: add evidence-linked QSL protocol posture, G1/G3/G5 context,
  release-gate wording, and current gap visibility.
- Demo/GUI: state non-production demo acceptance and guided prototype
  boundaries; do not claim KT-negative demo, attachment demo, native package,
  production GUI, or active keychain readiness.
- Metadata/privacy: use metadata-minimization copy only; state the current demo
  is not anonymity and residual metadata remains observable.
- Suite-2 / Triple-Ratchet: use research-stage Triple-Ratchet-style wording and
  prohibit proven/production/equivalence claims.
- External products: label CrawDaddy, SELARIX, QuantumShield API, BTC Battle,
  crypto-scanner, risk calculator, and consulting as external products/services
  unless direct qsl-protocol evidence exists.
- Healthcare/PQC consulting: keep service claims separate from protocol proof
  and make NIST/deadline copy precise and source-linked.
- Evidence links: add date-stamped links to goals, roadmap, decisions,
  traceability, demo, metadata, Suite-2 claim boundary, release-readiness map,
  external review package, and audits.
- Roadmap/release readiness: state partial release evidence and preserve open
  gaps.

## Safe / Prohibited Copy Summary

Safe copy may say:

- QSL protocol is research-stage and non-production.
- QSL Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design.
- Current evidence is release-gated and bounded to named repo artifacts.
- The demo is a non-production acceptance surface.
- QSL has a metadata minimization lane.
- The current demo profile is not an anonymity system and residual metadata
  remains observable.
- External products under the same brand do not prove QSL protocol production
  readiness.

Prohibited copy for QSL protocol, Suite-2, demo, or GUI claims includes:

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

## Evidence Map Summary

The directive maps public website topics to qsl-protocol evidence:

- goals and release gates: GOALS.md
- current posture: ROADMAP.md
- governance decisions: DECISIONS.md through D-0473
- goal-to-evidence mapping: TRACEABILITY.md
- website risk: WEBSITE_CLAIM_MATRIX.md and WEBSITE_UPDATE_PLAN.md
- Suite-2 public copy: SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
- release posture and reviewer package: RELEASE_READINESS_EVIDENCE_MAP.md and
  EXTERNAL_REVIEW_PACKAGE.md
- demo posture: DEMO_ACCEPTANCE_CRITERIA.md
- conformance priorities: CONFORMANCE_VECTOR_PRIORITIZATION.md
- metadata/privacy: DOC-G5-001 and DOC-G5-003
- prior website handoff/audit evidence: NA-0245, NA-0250, and NA-0251 reports

## Explicit No External Repo Edits Statement

NA-0253 performed no external website repository edits. It did not clone,
modify, push, deploy, or otherwise mutate the external website repository. It
also did not edit qsl-protocol website source, protocol/runtime/crypto/demo/
service code, qsc/qsl apps, tools, inputs, formal, qsc-desktop, qsl-server,
qsl-attachments, `.github`, scripts, Cargo files, public-safety configuration,
or branch-protection settings.

## Future Implementation Lane Recommendation

Recommended future external website lane:

- Title: Public Website Evidence-Boundary Implementation
- Scope: verified external website repository only
- First artifacts: static overclaim phrase scan, public link check, screenshot
  or build-preview review, claim-matrix MUST_FIX checklist, and scope proof
- Must protect: no QSL protocol production-readiness claim, no proven true
  Triple Ratchet claim, no quantum-proof claim, no anonymity or
  metadata-elimination claim, no external-product conflation, and no
  qsl-server/qsl-attachments/API conflation

## Known Limitations

- The exact external website repository is not identified by the qsl-protocol
  artifacts inspected in this lane.
- Live website copy may have changed after the NA-0245 audit.
- External NIST/deadline wording must be rechecked in the website implementation
  lane.
- This audit is not production release approval, external cryptographic review
  completion, or website deployment proof.
- KT-negative demo readiness, attachment demo readiness, native desktop package
  proof, production relay hardening, qsl-attachments hardening, metadata phase
  2, and complete cross-host conformance reproduction remain open gaps.
