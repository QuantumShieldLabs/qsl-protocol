Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0245 Website Truthfulness Audit Evidence

Directive: QSL-DIR-2026-05-03-025 / NA-0245

## Scope

This is a docs-only public-claims audit and update plan. No website implementation files, website source repositories, protocol/runtime code, demo/service implementation code, qsl-server files, qsl-attachments files, qsc-desktop files, public-safety helpers, branch-protection settings, workflow files, scripts, Cargo manifests, or lockfiles were edited by this audit lane.

## Websites / Pages Checked

Checked without login, purchases, or form submission:

- https://quantumshieldlabs.dev/
- https://quantumshieldlabs.dev/blog/
- https://quantumshieldlabs.dev/quantum-risk-calculator/
- https://quantumshieldlabs.dev/agent/
- https://quantumshieldlabs.dev/selarix.html
- https://qsl-dashboard.vercel.app
- https://quantumshield-api.vercel.app/
- https://base-signal.vercel.app/
- https://pypi.org/project/crypto-scanner/
- https://agdp.io/agent/2037
- https://github.com/mbennett-labs
- https://github.com/mbennett-labs/crawdaddy-security

Website source location noted from operator input and read-only confirmation:

- https://github.com/Tebbens4832/QuantumShield
- `gh repo view` showed the repository is private, default branch `main`, and was not modified.

## Commands / Tools Used

- Codex Web Module:
  - opened the public homepage, blog index, risk calculator, linked API, BTC Battle, PyPI package, public profile, and public linked stats pages.
  - searched official NIST pages for IR 8547 and PQC transition wording.
- Local read-only commands:
  - `curl -Ls https://quantumshieldlabs.dev/ | rg -o 'href="[^"]+"' | sort -u`
  - Python stdlib text extraction for `/agent/`, `/selarix.html`, and the swarm dashboard; no raw HTML files were committed.
  - `gh repo view Tebbens4832/QuantumShield --json ...` read-only source-location confirmation.
- Repo evidence reads:
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

## Summary of Major Claim Risks

1. Production-readiness overclaim risk:
   - The homepage uses live-product language that can be read as not-demo/not-prototype language for the QSL brand.
   - Repo truth says qsl-protocol remains research-stage and not production-ready.
2. External-product conflation risk:
   - CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, consulting, and the playbook appear under the same QSL brand.
   - Those claims are not qsl-protocol release evidence.
3. Metadata/privacy overclaim risk:
   - No direct anonymity overclaim was observed, but the site lacks the current G5 boundary language.
   - Repo truth says current demo metadata posture is not anonymity and does not eliminate timing, size, stable-id, or relay-visible metadata.
4. Demo/GUI overclaim risk:
   - The site does not visibly explain that the protocol demo is non-production or that QSC desktop is a bounded prototype with keychain-backed active ops deferred.
5. NIST/deadline precision risk:
   - The site has useful PQC migration urgency, but deadline wording should distinguish draft guidance, 2030 deprecation framing, and 2035 disallowance framing precisely.
6. qsl-server/qsl-attachments boundary drift risk:
   - The linked QuantumShield API could be misread as qsl-server or qsl-attachments evidence unless explicitly separated.

## Claim-Matrix Summary

Artifact:

- docs/public/WEBSITE_CLAIM_MATRIX.md

Rows:

- 18 total rows
- MUST_FIX: production/live wording, QSL protocol status omission, metadata/privacy boundary, CrawDaddy/SELARIX/API separation, NIST wording precision, Triple-Ratchet boundary preservation
- SHOULD_FIX: stale dated claims, consulting availability, risk calculator/consulting separation, external package taxonomy
- NICE_TO_HAVE: unrelated live-project taxonomy and blog/evidence-link clarity

## Repo Evidence Consulted

Key evidence findings:

- ROADMAP.md says QuantumShield remains research-stage and not production-ready.
- GOALS.md makes release readiness dependent on G1-G5 gates, conformance vectors/tests, formal checks, and metadata profile evidence.
- DEMO_ACCEPTANCE_CRITERIA.md says the demo is a non-production acceptance surface and must not claim production readiness.
- CONFORMANCE_VECTOR_PRIORITIZATION.md prioritizes fail-closed, no-mutation, metadata, and demo acceptance vectors before release claims.
- DOC-G5-001 says the metadata profile is not an anonymity system and does not hide IP-level metadata.
- DOC-G5-003 says stable identifiers, timing, and size signals remain observable in the current demo profile.
- qsc-desktop README and DOC-QSC-010 define QSC desktop as a bounded prototype with passphrase-backed active ops only, keychain-backed active ops deferred, and handshake/session-establish UI out of scope.
- TRACEABILITY.md keeps qsl-server transport-only and qsl-attachments opaque ciphertext-only boundaries explicit.

Recent PR evidence:

- PR #708: fail-closed KT verifier implementation merged.
- PR #729: demo negative acceptance and downgrade/no-mutation hardening merged.
- PR #731: KT consistency no-mutation hardening merged.
- PR #734: skipped-key and receive-decrypt no-mutation hardening merged.
- PR #736: metadata conformance negative expansion merged.
- PR #737: NA-0245 restored as sole READY item.

## Recommended Rewrite Priorities

1. Add a QSL protocol status section with research-stage / non-production wording.
2. Re-scope live product cards to external products/services, not qsl-protocol proof.
3. Add metadata and privacy boundary language from DOC-G5-001 and DOC-G5-003.
4. Add demo/GUI status language from DEMO_ACCEPTANCE_CRITERIA.md, qsc-desktop README, and DOC-QSC-010.
5. Rewrite NIST/deadline claims with source status and exact transition wording.
6. Add evidence links and last-verified timestamps.

## Explicit No-Website-Implementation Statement

NA-0245 performed no website implementation changes. The website source repo was identified read-only as `Tebbens4832/QuantumShield` and was not cloned for mutation, edited, pushed, or deployed by this lane.

## Uncertainty / Inaccessible-Page Notes

- The Codex Web Module rendered the aGDP agent page with limited detail; the homepage and CrawDaddy page carried enough public claim text to classify the agent claims.
- Some live counters differed between a web search cache and the live opened homepage. This supports the recommendation to add explicit last-verified dates.
- This audit did not authenticate to Virtuals ACP, Telegram, Gumroad, wallets, dashboards, or private GitHub content.
- This audit did not submit contact or risk-calculator forms.
- This audit did not execute paid scans or API endpoints.

## Future Work

- Execute a website implementation lane after this audit, limited to the website repository, with static copy checks for overclaim phrases and evidence-link validation.
- Add a public evidence page that links qsl-protocol release posture, goals, decisions, traceability, demo acceptance, metadata profile, and public-safety status.
- Add a periodic public-claims audit cadence so live counters, availability windows, NIST deadline copy, and external-product boundaries do not drift.
