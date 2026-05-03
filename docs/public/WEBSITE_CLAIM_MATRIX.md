Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# Website Claim Matrix

Directive: QSL-DIR-2026-05-03-025 / NA-0245

Audit posture: public website truthfulness and repo-sync audit only. This matrix does not authorize website implementation changes.

Retrieval timestamp for public website pages: 2026-05-03T16:09:26Z unless a row notes a more specific timestamp.

## Pages Checked

| Page title | URL | Retrieval notes |
| --- | --- | --- |
| Quantum Shield Labs - Post-Quantum Security for the AI Age | https://quantumshieldlabs.dev/ | Codex Web Module, public unauthenticated page, no form submission |
| Quantum Security Blog | Quantum Shield Labs | https://quantumshieldlabs.dev/blog/ | Codex Web Module, public index only |
| Quantum Security Risk Calculator | Quantum Shield Labs | https://quantumshieldlabs.dev/quantum-risk-calculator/ | Codex Web Module, page inspected without starting/submitting assessment |
| CrawDaddy Security - Quantum & On-Chain Security Scanner | https://quantumshieldlabs.dev/agent/ | Public text extraction only; no hiring, payment, or form submission |
| SELARIX - The Sovereign Fortress | https://quantumshieldlabs.dev/selarix.html | Public text extraction only; no dashboard mutation |
| QuantumShield API - Security Intelligence for the Agentic Web | https://quantumshield-api.vercel.app/ | Codex Web Module, public landing page only; no endpoint calls |
| BTC Battle - Real-Time Whale War | https://base-signal.vercel.app/ | Codex Web Module, public landing page only; no wallet connection |
| crypto-scanner | https://pypi.org/project/crypto-scanner/ | Codex Web Module, public package page |
| aGDP.io Agent page | https://agdp.io/agent/2037 | Codex Web Module, public page rendered limited agent detail |
| mbennett-labs GitHub profile | https://github.com/mbennett-labs | Codex Web Module, public profile only |

## Classification Legend

- SUPPORTED: consistent with repo evidence.
- PARTIALLY_SUPPORTED: some claim elements are supported, but public wording needs qualification.
- UNSUPPORTED: no repo or public evidence found in this audit supports the claim.
- OUTDATED: time-sensitive wording appears stale against the current date or current page state.
- NEEDS_DISCLAIMER: wording can be truthful only with explicit limits.
- OUT_OF_SCOPE_FOR_QSL_PROTOCOL: claim may belong to another product, service, or business line, not qsl-protocol.
- EXTERNAL_PRODUCT_OR_SERVICE: claim belongs to a linked site/package/product and must not be presented as qsl-protocol proof.

## Claim Matrix

| ID | Source URL / page title / retrieval timestamp | Claim excerpt or paraphrase | Category | Classification | Repo evidence | Recommended action | Priority | Notes / uncertainty |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| WCM-001 | https://quantumshieldlabs.dev/ / Quantum Shield Labs - Post-Quantum Security for the AI Age / 2026-05-03T16:09:26Z | The site headline frames Quantum Shield Labs as post-quantum security for the AI agent economy. | protocol / quantum / PQC | NEEDS_DISCLAIMER | ROADMAP.md says QuantumShield remains research-stage and not production-ready. GOALS.md defines release-readiness gates. | Add a visible QSL protocol status section stating research-stage, non-production, and release gates before any protocol-readiness implication. | MUST_FIX | The brand claim can stand for the company, but must not imply qsl-protocol release readiness. |
| WCM-002 | https://quantumshieldlabs.dev/ / homepage / 2026-05-03T16:09:26Z | The live projects section says production infrastructure is running and not demos/prototypes. | production / live / not-demo | EXTERNAL_PRODUCT_OR_SERVICE + NEEDS_DISCLAIMER | ROADMAP.md says qsl-protocol is not production-ready. DEMO_ACCEPTANCE_CRITERIA.md says the demo is non-production. | Separate external live products from qsl-protocol. Add "These live products are not QSL protocol production-readiness evidence." | MUST_FIX | This is the highest confusion risk because it appears under the same QSL brand. |
| WCM-003 | https://quantumshieldlabs.dev/agent/ / CrawDaddy Security / 2026-05-03T16:09:26Z | CrawDaddy is described as a live autonomous scanner for repos, smart contracts, tokens, and wallet risk. | CrawDaddy / agent economy | EXTERNAL_PRODUCT_OR_SERVICE | No local qsl-protocol evidence proves CrawDaddy runtime claims. qsl-protocol evidence is limited to protocol/demo/governance status. | Keep CrawDaddy on the site, but label it as an external product/service and avoid using it as QSL protocol proof. | SHOULD_FIX | Public pages and linked services provide external context; this audit did not authenticate or run paid scans. |
| WCM-004 | https://quantumshield-api.vercel.app/ / QuantumShield API / 2026-05-03T16:09:26Z | Linked API page claims live x402 payments and token/wallet/security intelligence services. | qsl-server / relay / attachment / service claims | EXTERNAL_PRODUCT_OR_SERVICE | TRACEABILITY.md distinguishes qsl-server as transport-only and qsl-attachments as opaque ciphertext-only. No local evidence maps this API to qsl-protocol runtime. | Put API under external products and do not conflate it with qsl-server, relay, attachment, or QSL protocol transport proof. | MUST_FIX | Do not imply the API is the qsl-protocol relay or an attachment service. |
| WCM-005 | https://quantumshieldlabs.dev/ / homepage projects and https://base-signal.vercel.app/ / BTC Battle / 2026-05-03T16:09:26Z | BTC Battle is presented as a live Base mini app / real-time whale-war visualization. | demo / app / GUI | EXTERNAL_PRODUCT_OR_SERVICE | qsc desktop evidence is bounded to qsl/qsl-client/qsc-desktop/README.md and DOC-QSC-010. No repo evidence maps BTC Battle to qsc or QSL protocol. | Keep in an unrelated live projects area; do not use it as evidence for protocol demo, QSC GUI, or release readiness. | NICE_TO_HAVE | Low direct QSL protocol risk if separated clearly. |
| WCM-006 | https://quantumshieldlabs.dev/ / homepage and https://quantumshieldlabs.dev/quantum-risk-calculator/ / risk calculator / 2026-05-03T16:09:26Z | Risk calculator promises quick risk scoring and PDF-style output for healthcare readiness. | healthcare / PQC consulting | EXTERNAL_PRODUCT_OR_SERVICE + NEEDS_DISCLAIMER | No qsl-protocol evidence validates consulting deliverables. ROADMAP.md and DEMO_ACCEPTANCE_CRITERIA.md are protocol/demo truth sources, not consulting proof. | Make clear this is a consulting/lead-generation tool, not a QSL protocol certification, conformance result, or security proof. | SHOULD_FIX | Do not submit calculator/contact forms in audit lanes. |
| WCM-007 | https://quantumshieldlabs.dev/ / homepage healthcare section / 2026-05-03T16:09:26Z | Homepage deadline copy says NIST deprecates RSA/ECC after 2030 and discusses 2030/2035 transition. | NIST / deadline / compliance | PARTIALLY_SUPPORTED + NEEDS_DISCLAIMER | Local repo does not own NIST deadline truth. NIST IR 8547 is an initial public draft and distinguishes deprecated-after-2030 from disallowed-after-2035 treatment by algorithm/security strength. | Rewrite to cite draft transition status precisely. Avoid blanket RSA/ECC-after-2030 language without scope. | MUST_FIX | Official NIST pages support migration urgency, but public copy should avoid overbroad compliance wording. |
| WCM-008 | https://quantumshieldlabs.dev/ / homepage healthcare section / 2026-05-03T16:09:26Z | Homepage copy presents 2027-2030 as the expected fault-tolerant quantum-computer timeline. | NIST / deadline / compliance | NEEDS_DISCLAIMER | PROJECT_CHARTER.md identifies long-term quantum-capable adversaries. Local repo does not prove a 2027-2030 CRQC timeline. | Recast as risk planning language with source attribution and uncertainty; avoid deterministic deadline phrasing. | SHOULD_FIX | Timeframes are external threat intelligence, not qsl-protocol evidence. |
| WCM-009 | https://quantumshieldlabs.dev/ / homepage and https://pypi.org/project/crypto-scanner/ / PyPI / 2026-05-03T16:09:26Z | crypto-scanner scans code for quantum-vulnerable cryptography and powers CrawDaddy repo analysis. | GitHub/code/evidence links | EXTERNAL_PRODUCT_OR_SERVICE | No local qsl-protocol evidence maps PyPI package behavior to QSL protocol conformance. | Link as a separate open-source tool; do not imply protocol conformance or release-readiness evidence. | SHOULD_FIX | PyPI classifies the package as beta. |
| WCM-010 | https://quantumshieldlabs.dev/blog/ / Quantum Security Blog / 2026-05-03T16:09:26Z | Blog index claims expert analysis on PQC, AI agent security, HNDL, and healthcare implications. | public roadmap / journey / resources | EXTERNAL_PRODUCT_OR_SERVICE | qsl-protocol evidence sources are GOALS.md, ROADMAP.md, DECISIONS.md, TRACEABILITY.md, demo criteria, and conformance docs. Blog posts are public education, not repo evidence. | Add an evidence page that points protocol readers to qsl-protocol docs and separates blog commentary from implementation proof. | NICE_TO_HAVE | Full blog article audit was out of scope; index/excerpts were sufficient for category classification. |
| WCM-011 | https://quantumshieldlabs.dev/selarix.html / SELARIX / 2026-05-03T16:09:26Z | SELARIX is described as an agentic economy build-out with autonomous agents, treasury, and live agent status. | CrawDaddy / SELARIX separation | EXTERNAL_PRODUCT_OR_SERVICE + NEEDS_DISCLAIMER | No qsl-protocol evidence maps SELARIX to Suite-2, QSC demo, qsl-server, or qsl-attachments proof. | Add separation guidance: SELARIX is a company/product ecosystem, not QSL protocol proof. | MUST_FIX | Strong brand proximity creates a conflation risk. |
| WCM-012 | https://quantumshieldlabs.dev/ / homepage journey / 2026-05-03T16:09:26Z | Timeline labels remain "Active - March 2026" and "Live as of March 2026" on 2026-05-03. | public roadmap / journey | OUTDATED | NEXT_ACTIONS.md shows live queue has advanced to NA-0245 on 2026-05-03; ROADMAP.md is current as of 2026-04-30. | Refresh timeline labels or use date-stamped last-verified cards. | SHOULD_FIX | Search cache showed different counters than the opened page, reinforcing the need for explicit verification dates. |
| WCM-013 | https://quantumshieldlabs.dev/ / contact section / 2026-05-03T16:09:26Z | Contact section says QSL is accepting limited healthcare consulting engagements for Q2 2026. | healthcare / PQC consulting | OUTDATED + EXTERNAL_PRODUCT_OR_SERVICE | Not a qsl-protocol claim; no local repo evidence validates consulting capacity. | Replace with current availability or remove the dated claim. | SHOULD_FIX | Current date is 2026-05-03, so Q2 2026 is already underway. |
| WCM-014 | https://quantumshieldlabs.dev/ / homepage resources / 2026-05-03T16:09:26Z | Resources section claims free tools, educational content, and open-source libraries for the quantum security community. | GitHub/code/evidence links | PARTIALLY_SUPPORTED | Public links include PyPI/GitHub. Local repo evidence supports qsl-protocol public development but not every linked tool. | Add a curated evidence-links section distinguishing qsl-protocol, external tools, and educational content. | NICE_TO_HAVE | Broadly acceptable if the evidence taxonomy is clear. |
| WCM-015 | qsl-protocol evidence gap observed from homepage/project sections / 2026-05-03T16:09:26Z | No visible public section was found for qsl-protocol research-stage status, Suite-2 boundaries, demo acceptance, or QSC GUI limitations. | QSL protocol / Suite-2 / Triple Ratchet / demo / GUI | NEEDS_DISCLAIMER | GOALS.md defines Suite-2 release gates. ROADMAP.md says research-stage. DEMO_ACCEPTANCE_CRITERIA.md says non-production. qsc-desktop README and DOC-QSC-010 define GUI prototype limits. | Add a QSL protocol status page or homepage band with research-stage, non-production status and links to repo evidence. | MUST_FIX | This is an omission rather than an affirmative claim, but the omission makes overinterpretation likely. |
| WCM-016 | qsl-protocol evidence gap observed from homepage/project sections / 2026-05-03T16:09:26Z | No anonymity or full metadata-elimination claim was observed, but broad "security layer" wording can be read expansively. | metadata / privacy / anonymity / minimization | NEEDS_DISCLAIMER | DOC-G5-001 says the demo is not an anonymity system. DOC-G5-003 says stable ids, timing, and sizes remain observable. | Add safe privacy copy: metadata minimization is a research/demo lane; no anonymity or metadata elimination claim. | MUST_FIX | Required to prevent future copy from overstating G5. |
| WCM-017 | qsl-protocol evidence gap observed from homepage/project sections / 2026-05-03T16:09:26Z | No "True Triple Ratchet" or proven Triple-Ratchet production claim was observed on the checked public pages. | QSL protocol / Suite-2 / Triple Ratchet | SUPPORTED | GOALS.md and TRACEABILITY.md show Suite-2 / Triple-Ratchet-style work is gated by release evidence and CI. | Preserve this boundary. If added later, phrase as research-stage Suite-2 design and conformance work until release gates are met. | MUST_FIX | The absence of this overclaim is good; protect it in implementation backlog. |
| WCM-018 | https://quantumshieldlabs.dev/ / contact privacy sentence / 2026-05-03T16:09:26Z | Contact form says submitted information will not be shared. | metadata / privacy / anonymity | OUT_OF_SCOPE_FOR_QSL_PROTOCOL + NEEDS_DISCLAIMER | DOC-G5-001 and DOC-G5-003 cover protocol/demo metadata, not website form privacy operations. | Keep website privacy/legal claims separate from QSL protocol metadata-minimization claims; add a proper privacy policy if form remains. | SHOULD_FIX | This audit did not submit the form or verify backend handling. |

## Repo Evidence Consulted

- GOALS.md
- PROJECT_CHARTER.md
- ROADMAP.md
- NEXT_ACTIONS.md
- DECISIONS.md
- TRACEABILITY.md
- docs/demo/DEMO_ACCEPTANCE_CRITERIA.md
- docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md
- docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
- docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
- qsl/qsl-client/qsc-desktop/README.md
- docs/design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md
- PR evidence for #708, #729, #731, #734, #736, and #737

## Top MUST_FIX Items

1. Separate live product claims from qsl-protocol research/demo status.
2. Add a QSL protocol status section with non-production and release-gate wording.
3. Add metadata/privacy boundary language that disclaims anonymity and metadata elimination.
4. Prevent qsl-server, qsl-attachments, QuantumShield API, and CrawDaddy/SELARIX conflation.
5. Tighten NIST deadline wording to match draft/current guidance and source status precisely.
