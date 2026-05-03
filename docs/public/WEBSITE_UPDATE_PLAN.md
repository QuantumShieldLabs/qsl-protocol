Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# Website Update Plan

Directive: QSL-DIR-2026-05-03-025 / NA-0245

This is a plan only. It does not authorize website implementation, marketing rewrites in production, website source edits, protocol/runtime changes, or service changes.

## Executive Summary

The public website is accessible and contains strong external-product claims for CrawDaddy, SELARIX, the QuantumShield API, BTC Battle, the risk calculator, and healthcare PQC consulting. The qsl-protocol repo truth is narrower: QuantumShield remains a research-stage protocol and demo system, not production-ready. The current website needs a clear information architecture that separates company/product ecosystem claims from QSL protocol evidence.

The most urgent update is not a visual redesign. It is public-claim boundary repair:

- Live products must not imply qsl-protocol production readiness.
- Protocol, Suite-2, Triple-Ratchet-style, demo, GUI, metadata, qsl-server, and qsl-attachments claims must be source-linked to repo evidence or explicitly deferred.
- CrawDaddy, SELARIX, consulting, and the QuantumShield API should remain visible only as external products/services with their own evidence boundaries.

## Recommended Site Information Architecture

Use five clear lanes:

1. Company overview: Quantum Shield Labs, healthcare PQC consulting, and external products.
2. Products and tools: CrawDaddy, SELARIX, QuantumShield API, BTC Battle, risk calculator, crypto-scanner, and playbook, each labeled as external to qsl-protocol unless evidence says otherwise.
3. QSL protocol: research-stage protocol status, goals G1-G5, Suite-2 release gates, conformance vectors, formal checks, and public-safety gate status.
4. Demo and GUI: non-production demo acceptance status, one-command demo backlog, QSC desktop prototype limitations.
5. Evidence: direct links to GOALS.md, ROADMAP.md, NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md, demo criteria, conformance priorities, and privacy profile docs.

## Homepage Update Plan

Recommended homepage changes:

- Keep the company headline, but add one visible status card or band:
  - "QSL protocol status: research-stage and non-production."
  - "Live product claims below are separate from qsl-protocol release readiness."
- Replace broad "not demos, not prototypes" wording with scoped language:
  - safe wording: "The products below include external live tools and services. QSL protocol itself remains research-stage until its release gates are met."
- Add last-verified dates to counters and timelines.
- Move the QSL protocol evidence section above or adjacent to live projects so readers see the boundary before product claims.
- Avoid hero or section wording that implies production-grade quantum-secure communications.

## Projects / Tools Update Plan

For each project card, add fields:

- Product owner: QSL, external, or linked repo.
- Protocol relation: "not qsl-protocol evidence", "uses qsl-protocol demo", or "qsl-protocol evidence" only when supported.
- Evidence link: public repo, package page, live dashboard, or qsl-protocol evidence doc.
- Status date: last verified timestamp.
- Boundary note: "does not prove QSL protocol production readiness" where applicable.

Apply this especially to:

- CrawDaddy Security
- SELARIX
- QuantumShield API
- BTC Battle
- Quantum Risk Calculator
- PQC Healthcare Playbook
- crypto-scanner

## QSL Protocol Status Section

Add a standalone section with this safe shape:

- Status: research-stage protocol and demo system.
- Release readiness: not claimed.
- Current goals: G1 always-hybrid per-message keys, G3 fail-closed downgrade resistance, G5 metadata minimization lane, plus G4 verification as release gate.
- Current evidence:
  - GOALS.md
  - ROADMAP.md
  - NEXT_ACTIONS.md
  - DECISIONS.md
  - TRACEABILITY.md
  - demo acceptance criteria
  - conformance vector prioritization
- Safe copy:
  - "QSL protocol work is public and evidence-driven, but it is not production-ready."
  - "Suite-2 and related protocol claims remain bound to repo release gates."

Do not say:

- "Production-ready QSL protocol"
- "Proven Triple Ratchet"
- "Quantum-proof communications"
- "Metadata eliminated"
- "Anonymous messaging"

## Demo / GUI Status Section

Add a demo status panel:

- Demo status: non-production acceptance surface.
- One-command demo acceptance: planned successor lane, not complete under NA-0245.
- Positive flow target: local loopback peers, valid establish/send/receive/decrypt.
- Negative flow target: invalid auth, malformed input, replay/downgrade rejects where current demo surface supports them.
- CI context references: demo-cli-build, demo-cli-smoke, metadata-conformance-smoke.

Add a QSC desktop GUI panel:

- Prototype only.
- Linux/macOS bounded Tauri shell.
- Passphrase-backed active operations only.
- Keychain-backed active operations deferred.
- Handshake/session-establish UI out of scope.
- qsc sidecar owns client-core truth.

## Metadata / Privacy Language Section

Use bounded G5 wording:

- "QSL has a metadata minimization lane."
- "The current demo profile is not an anonymity system."
- "Stable identifiers, timing, and size signals remain observable in the current demo profile."
- "Loopback-only defaults, token authorization, bounded queues, padding support, and leak-safe evidence are enforced where current demo conformance covers them."

Avoid:

- "anonymous"
- "untraceable"
- "metadata-free"
- "metadata eliminated"
- "full traffic-analysis resistance"

Website form privacy claims should be separated from protocol metadata claims and should link to an actual website privacy policy if the form remains.

## Triple-Ratchet / Suite-2 Claim Boundary

Safe wording:

- "QSL maintains research-stage Suite-2 / Triple-Ratchet-style protocol work with explicit release gates."
- "Claims are backed by GOALS.md, conformance vectors, formal checks, and public-safety CI when applicable."
- "Release readiness is not claimed until all G1-G5 gates are met."

Avoid:

- "True Triple Ratchet is proven"
- "production Triple Ratchet"
- "post-quantum secure messaging is shipped"
- "Suite-2 is release-ready"

## Healthcare / PQC Consulting Claim Boundary

Healthcare consulting and PQC migration planning are external service claims. They may cite NIST/FIPS standards and healthcare needs, but they should not imply qsl-protocol is certified or production-deployed.

Recommended changes:

- Make NIST transition wording precise and source-linked.
- Distinguish draft transition guidance from final standards.
- Remove stale availability windows.
- Avoid "mandate" language unless tied to a named source and jurisdiction.
- Use "assessment", "roadmap", and "planning" rather than "certification" unless certification evidence exists.

## CrawDaddy / SELARIX Separation Guidance

CrawDaddy and SELARIX can remain part of the QSL public story, but not as protocol proof.

Required boundaries:

- CrawDaddy: autonomous scanner product/service, external to qsl-protocol.
- SELARIX: agentic economy / venture-studio ecosystem, external to qsl-protocol.
- QuantumShield API: external security-intelligence API, not qsl-server and not qsl-attachments.
- BTC Battle: unrelated live project, not qsl-protocol evidence.

Recommended label:

"External product/service. Does not prove QSL protocol production readiness."

## Evidence-Links Plan

Create a public Evidence page or section with:

- qsl-protocol public repo link.
- Latest READY item from NEXT_ACTIONS.md.
- Latest decision entry range and D-0456 audit entry.
- Current release posture from ROADMAP.md.
- Demo acceptance criteria.
- Metadata threat model and envelope/transport profile.
- QSC desktop prototype README and active-ops boundary doc.
- Public-safety required/green status summary, with last verified timestamp.

Evidence links should be date-stamped and should not include raw secret-like tokens, long hex dumps, or sensitive endpoint fragments.

## Implementation Backlog

1. Add homepage QSL protocol status band.
2. Add product/service taxonomy labels to project cards.
3. Add evidence page with qsl-protocol links and last-verified dates.
4. Rewrite NIST/deadline copy with precise source status.
5. Rewrite metadata/privacy copy with no-anonymity and no-metadata-elimination disclaimers.
6. Add demo/GUI status panels with current non-production limits.
7. Split CrawDaddy/SELARIX/API from QSL protocol proof.
8. Add stale-counter/timeline verification dates.
9. Add website privacy-policy link or narrow form privacy wording.
10. Add a review checklist that blocks production-readiness, anonymity, proven-Triple-Ratchet, qsl-server, and qsl-attachments overclaims.

## No-Production-Overclaim Language

Recommended reusable language:

> QSL protocol is research-stage and non-production. Public repository evidence tracks goals, decisions, conformance priorities, demo acceptance, and CI gates. External live products under the Quantum Shield Labs brand do not prove QSL protocol release readiness.

This is a short compliant excerpt for implementation planning; the final website copy should be reviewed in the website implementation lane.

## Next Recommended Implementation Lane

Recommended next website lane after governance closeout:

- Title: NA-0246W - Public Website Evidence-Boundary Implementation
- Scope: website repository only, plus qsl-protocol evidence updates only if separately authorized.
- Must protect:
  - no QSL protocol production-readiness claim
  - no anonymity or metadata-elimination claim
  - no proven Triple-Ratchet claim
  - qsl-server and qsl-attachments boundaries remain truthful
  - CrawDaddy/SELARIX/API/consulting remain separated from qsl-protocol evidence
- First tests/artifacts:
  - static copy scan for banned overclaim phrases
  - link check for evidence URLs
  - screenshot review of homepage status band and product taxonomy labels
  - claim-matrix diff showing all MUST_FIX rows addressed or explicitly deferred
