Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-15
Replaces: n/a
Superseded-By: n/a

# Website Claim Matrix

Directive: QSL-DIR-2026-05-03-025 / NA-0245 (original audit); audit half refreshed under QSL-DIR-2026-07-15-583 / NA-0647 (D-1270). The NA-0539/NA-0541 wording-policy sections below are preserved unchanged from the original.

Audit posture: public website truthfulness and repo-sync audit only. This matrix does not authorize website implementation changes.

Audit source and snapshot convention (NA-0647 convention change, stated explicitly): this refresh audits the WEBSITE REPOSITORY SOURCE at the current production commit rather than live-page retrieval. Audited source: the quantumshieldlabs.org website repository, `main` at commit `21a908a4` (the WEB-0006 Phase-A closeout merge, 2026-07-16 UTC; the site auto-deploys from `main`, and Phase-A closeout verified production serving this content). Repo-side evidence is read at qsl-protocol `main` commit `ac7e850c`. Audit rows are date/commit-stamped snapshots; a light row re-touch after the website's Phase B content sync is expected.

## NA-0539 Repository Claim Policy Addendum

NA-0539 updates selected repository public docs only. It does not create or
mutate `public/` or `website/` paths, does not update an external website, and
does not authorize website implementation.

| Permitted public wording | Forbidden wording | Evidence source | Required qualifier |
| --- | --- | --- | --- |
| QSL has bounded evidence for a direct remote qsc E2EE workflow using synthetic data. | QSL is production ready, public ready, or public-internet ready. | D446 and [NA-0537 evidence](../governance/evidence/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_harness.md). | Engineering evidence under controlled lab conditions; not production readiness. |
| QSL has repeated-run cleanup/freshness evidence under a controlled lab setup. | QSL has complete crypto, complete identity, complete trust, or completed external review. | D446 and [NA-0537 evidence](../governance/evidence/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_harness.md). | Selected qsc proof only; external review remains invited and incomplete. |
| QSL has fail-closed evidence for selected wrong-peer, stale/replaced-peer, replay, and corrupt-delivery cases. | QSL is replay proof, downgrade proof, vulnerability free, bug free, or has perfect crypto. | D441, D419, [NA-0535 evidence](../governance/evidence/NA-0535_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_port_diagnostic_implementation_harness.md), and [NA-0523 evidence](../governance/evidence/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_harness.md). | Selected negative cases only; not universal proof. |
| QSL has public-safety/advisories gate evidence and quinn-proto RUSTSEC-2026-0185 remediation in current lockfiles. | QSL has vulnerability-free, bug-free, or secret-material-complete status. | D453, NA-0541 startup proof, root `Cargo.lock`, and nested qsc fuzz lockfile checks. | Advisory posture is gate-backed and time-sensitive. |
| We invite review of evidence, limits, and next steps. | External review is complete or certification is complete. | [External review package](EXTERNAL_REVIEW_PACKAGE.md) and D-1068. | Review invitation only; findings and dispositions must be recorded separately. |

Required no-claim boundaries:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no identity-complete claim;
- no trust-complete claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no secret-material-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## NA-0541 Progress Claim Policy Addendum

NA-0541 creates repository Progress pages and performs the D-1070 public
accuracy sweep. It does not create or mutate `public/` or `website/` paths,
does not deploy a website, does not create automation, and does not authorize
external website implementation.

| Permitted public wording | Forbidden wording | Evidence source | Required qualifier |
| --- | --- | --- | --- |
| QSL publishes public Progress entries that summarize merged evidence, accepted decisions, corrections, limits, and handoffs. | The Progress entry is a release certificate, public readiness, production readiness, or public internet readiness. | D-1070, D-1071, D-1072, [Progress index](PROGRESS.md), and [June 25 Progress entry](progress/2026-06-25.md). | Engineering evidence summary only; no release approval. |
| A stale public reference was corrected to match merged evidence and current checks. | The whole site is verified, all statements are universally correct, or future pages are pre-approved. | NA-0541 correction ledger and validation evidence. | Correction scope is the exact D-1070 public path bundle and publication-time evidence. |
| Review of evidence, limits, corrections, and next steps is invited. | External review is complete, certification is complete, or reviewer findings are accepted. | [External review package](EXTERNAL_REVIEW_PACKAGE.md) and [Progress index](PROGRESS.md). | Review invitation only; findings and dispositions require separate evidence. |
| Operator-local SSD maintenance context may be summarized as local operational context. | SSD cleanup is protocol assurance, security proof, deployment proof, or release readiness. | D453/D454 inherited operator-local context and NA-0541 read-only verification. | Not protocol/security evidence and not a product/service claim. |

Progress evidence and correction wording policy:

- tie each dated Progress entry to merged PRs, accepted decisions, in-tree
  evidence/testplans, verified checks, and a publication-time handoff;
- list corrected public paths and whether each correction is factual,
  claim-safety, Progress architecture, or out of scope;
- do not copy raw proof logs, private material, credentials, route-token or
  capability values, raw SSH configuration, backup material, or private
  topology into public content;
- do not convert selected negative cases, CI gates, local maintenance, or
  repeated daily summaries into broader security-completion claims.

## Pages Checked

All ten routes of the current quantumshieldlabs.org site, audited from website-repo source at commit `21a908a4` (page components plus the shared sections, modals, and the `src/links.js` evidence-link map they import).

| Page title | URL | Audit notes |
| --- | --- | --- |
| Home | https://quantumshieldlabs.org/ | Source audit: Hero, ProblemStatement, TechnicalCapabilities, SocialProof, PerformanceMetrics, IntegrationCompatibility, BusinessValue, CustomerTypes, TechnicalComparison, ROICalculator, FAQ, WhitepaperDownload, CTASection sections plus modal bodies |
| Technology | https://quantumshieldlabs.org/technology | Source audit: architecture, primitive table, scope/boundary sections, source links |
| Security Posture | https://quantumshieldlabs.org/security-posture | Source audit: show/not-claim/review sections and proof links |
| Progress and Proof | https://quantumshieldlabs.org/progress | Source audit: repo map, boundary banner, evidence cards, proof-lane sections |
| Run Demos | https://quantumshieldlabs.org/run-demos | Source audit: demo boundary, local-demo and remote-demo instructions; the local demo script was additionally smoke-run in qsl-protocol (see WCM-110) |
| Security & Compliance | https://quantumshieldlabs.org/compliance | Source audit: compliance boundary, algorithm table, reality-check sections |
| About Us | https://quantumshieldlabs.org/about | Source audit: mission/approach, milestones, contact |
| Resources | https://quantumshieldlabs.org/resources | Source audit: evidence-link cards |
| Privacy Policy | https://quantumshieldlabs.org/privacy | Source audit: website-vs-protocol boundary section; website-operations content otherwise out of protocol scope |
| Terms of Service | https://quantumshieldlabs.org/terms | Source audit: legal content; website-operations content out of protocol scope |

## Classification Legend

- SUPPORTED: consistent with repo evidence.
- PARTIALLY_SUPPORTED: some claim elements are supported, but public wording needs qualification.
- UNSUPPORTED: no repo or public evidence found in this audit supports the claim.
- OUTDATED: time-sensitive wording appears stale against the current date or current page state.
- NEEDS_DISCLAIMER: wording can be truthful only with explicit limits.
- OUT_OF_SCOPE_FOR_QSL_PROTOCOL: claim may belong to another product, service, or business line, not qsl-protocol.
- EXTERNAL_PRODUCT_OR_SERVICE: claim belongs to a linked site/package/product and must not be presented as qsl-protocol proof.

## Claim Matrix

Refreshed row series (WCM-101+). The prior WCM-001..018 rows audited the retired quantumshieldlabs.dev site and are superseded by this table; the wording policies above are unchanged and still govern. All rows below are stamped website-source `21a908a4` / qsl-protocol `ac7e850c` / 2026-07-15.

| ID | Source page(s) | Claim excerpt or paraphrase | Category | Classification | Repo evidence | Recommended action | Priority | Notes / uncertainty |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| WCM-101 | Home (Hero) | "Public AGPL research-stage specs and reference implementation. Production hardening is separate roadmap work." and "Current runtime target: ML-KEM-768 / ML-DSA-65 / X25519 / Ed25519". | protocol / status / PQC | SUPPORTED | README.md, docs/public/INDEX.md, ROADMAP.md (research-stage), the qsc/refimpl runtime boundary code, GOALS.md release gates. | Preserve. | NICE_TO_HAVE | The banner language matches the repo posture files. |
| WCM-102 | Home (Hero, SocialProof, TechnicalComparison) | QSL publishes drafts, vectors, demo criteria, release gates, and workflow evidence for inspection; website claims must tie back to public sources. | evidence / proof trail | SUPPORTED | docs/canonical (DOC-CAN-003/004), vectors and conformance docs, docs/demo/DEMO_ACCEPTANCE_CRITERIA.md, GOALS.md, live GitHub Actions history. | Preserve. | NICE_TO_HAVE | The interpretation rule (unsourced claims are roadmap/history or removed) matches this matrix's policy sections. |
| WCM-103 | Home (Hero boundary band, FAQ), Run Demos (boundary), Technology, Security Posture | "Bounded non-production proof exists for selected native package, KT-negative demo, and attachment demo paths. Still not claimed: production relay/service readiness, public-internet readiness, completed external cryptographic review, true-triple-ratchet completion, or full attachment integration." | claim boundary | SUPPORTED | Matches the NA-0539 required no-claim boundaries above; DEMO_ACCEPTANCE_CRITERIA.md; docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md; EXTERNAL_REVIEW_PACKAGE.md (review invited, not complete). | Preserve verbatim wherever copy is edited. | MUST_FIX | Protect this band in Phase B edits; it is the site's load-bearing boundary statement. |
| WCM-104 | Home (Hero proof links), Security Posture, Progress and Proof | Proof links point to the qshield-ci workflow (handshake contract + suite), the NA-0640 full-stack e2e as-built, and the NA-0642 qsl-server durability testplan; retired scheduled remote-relay lanes are no longer presented as live proof. | evidence links | SUPPORTED | All 15 `src/links.js` deep-link targets verified present at qsl-protocol `ac7e850c`, including docs/governance/evidence/NA-0640_as_built.md, tests/NA-0640_e2e_integration_full_stack_testplan.md, tests/NA-0642_qsl_server_durability_testplan.md; .github/workflows/ci.yml is named qshield-ci. | Keep evidence paths stable in qsl-protocol; a path move breaks site deep links. | SHOULD_FIX | Link-target existence was machine-checked in this refresh (NA-0647 evidence). |
| WCM-105 | Home (TechnicalCapabilities, PerformanceMetrics) | Capability cards separate implemented-in-refimpl vs current-draft vs designed/planned; no performance benchmarks are published ("no benchmark theater"). | capability status | SUPPORTED | The hybrid runtime boundary and ciphertext-only relay posture are implemented (NA-0640 e2e byte-verified round-trip through the real relay); metadata-minimization and deployment profiles are drafts (DOC-G5-001/-003); no benchmark numbers exist in the public repos. | Preserve the three-way status legend in future edits. | NICE_TO_HAVE | The "current proof-backed property" wording for the ciphertext-only relay is backed by the bounded NA-0640 evidence, not a universal claim. |
| WCM-106 | Technology, Home (FAQ) | The current architecture is research-stage Suite-2 / Triple-Ratchet-style hybrid messaging plus SCKA; the primitive table lists ML-KEM-768 + X25519, ML-DSA-65 + Ed25519, AES-256-GCM, KMAC-256, SHA-512 as the current runtime boundary. | architecture / primitives | SUPPORTED | DOC-CAN-003 (Suite-2 draft), DOC-CAN-004 (SCKA draft), the refimpl/qsc runtime boundary, docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md. | Preserve. | NICE_TO_HAVE | Draft-stage qualifiers are present on every architecture statement checked. |
| WCM-107 | Technology (Evidence-backed today) | "Conformance-vector and formal-verification lanes" are listed as evidence-backed today. | formal methods | SUPPORTED | formal/README.md (machine-checked models run in CI, including the D572 bounded QSC.HS.* handshake-authentication model with its recorded scope limits), DOC-G4-002 (Suite-2 DH+PQ composition ProVerif analysis), conformance vector lanes in CI. | Preserve; Phase B may add the bounded formal-methods story using the PROGRESS 2026-07-15 entry's wording, keeping its stated limits. | SHOULD_FIX | Formal-model claims must stay bounded: machine-checked within stated model limits, with known unmodeled slices recorded on the ledger; never "formally verified" without qualification. |
| WCM-108 | Security Posture | Relay path is ciphertext-only at transport boundaries; client identity/session state is encrypted at rest; send/receive gating is fail-closed on the handshake contract exercised by repository CI; qsl-attachments stores/fetches opaque ciphertext only. | security posture | SUPPORTED | NA-0640 e2e (real relay + real attachments service, byte-verified, bearer-token positive and negative cases); the qsc vault/fs_store encrypted-at-rest implementation; the qshield-ci handshake contract lane; qsl-attachments README. | Preserve. | NICE_TO_HAVE | Each bullet carries its bounded framing on-page; none claims production hardening (ENG-0039 remains open in qsl-protocol). |
| WCM-109 | Security & Compliance | NIST-standardized algorithm targets (FIPS 203 ML-KEM, FIPS 204 ML-DSA); CNSA 2.0 conformance explicitly not claimed; FIPS 140-3 certification not claimed; formal compliance validation remains future work. | compliance | SUPPORTED | The runtime targets ML-KEM-768/ML-DSA-65 (FIPS 203/204 parameter sets); no compliance-suite conformance artifacts exist in the repos, and the page says so. | Preserve. | NICE_TO_HAVE | The CNSA 2.0 non-claim (different parameter sets) is accurate for the current runtime. |
| WCM-110 | Run Demos (Local Demo) | "Local Demo - runs local inbox + two-party message path without remote relay credentials", instructing visitors to clone qsl-protocol and run `./scripts/demo/qsc_demo_local.sh`. | demo instructions | OUTDATED | NA-0647 smoke-run at qsl-protocol `ac7e850c` (2026-07-16 UTC): the script completes with exit 0 and prints DEMO DONE, but every qsc send/receive fails with `vault_locked` / `explicit_unlock_required` and no message is delivered (deliver_count=0, receive outputs empty). The script predates the qsc explicit vault-unlock requirement (`--unlock-passphrase-file` / `--unlock-passphrase-env`) and masks the failure. Smoke-run output recorded in docs/governance/evidence/NA-0647_as_built.md. | Fix the demo script in qsl-protocol (its own follow-up lane; flagged at NA-0647 closeout - not fixed in the docs lane); re-verify; then re-touch this row and, if wording changes, the page. | MUST_FIX | The page's boundary language is fine; the instructions as written do not currently produce a working demo. |
| WCM-111 | Run Demos (remote demo sections) | Remote relay demo and remote handshake notes: transport-health checks against the reader's own relay endpoint, explicitly "not handshake proof"; CI named as the preferred verification path. | demo instructions | SUPPORTED | scripts/demo/qsc_remote_relay_smoke.sh and qsc_remote_handshake_smoke.sh exist in qsl-protocol; the page's framing (transport health, not proof; CI preferred) matches repo posture. | Preserve. | NICE_TO_HAVE | Not exercised in this refresh (requires a reader-supplied relay endpoint); classification is source-and-framing based. |
| WCM-112 | Progress and Proof, Resources | The claim-matrix cards describe this document as a "historical claim audit of the prior quantumshieldlabs.dev site" whose "wording policies still govern", with "a refresh against the current site pending in qsl-protocol". | evidence links | OUTDATED | This NA-0647 refresh is that pending refresh; once merged, the "pending" wording is satisfied and the "historical .dev audit" description no longer matches this document. | Phase B: re-touch the two cards to describe the refreshed matrix (current-site audit, commit-stamped). | SHOULD_FIX | Accurate at website commit `21a908a4`; becomes stale the moment this refresh merges. Expected Phase-B touch. |
| WCM-113 | About, Home (BusinessValue, CustomerTypes, ROICalculator, CTASection) | Mission/audience framing: public drafts, public code, evaluation-before-hardening, future support/commercial paths stay separate from public AGPL access; milestones list 2024 formation and 2026 public repos with hardening/review/validation as future work. | company / posture | SUPPORTED | The three public AGPL repos exist; posture files (README, INDEX, ROADMAP) state research-stage and separate future hardening; no validation/support claim is made. | Preserve. | NICE_TO_HAVE | Segment cards describe evaluation audiences and design targets, each with an explicit non-claim qualifier. |
| WCM-114 | Privacy Policy, Terms of Service | Website-operations content (data handling, legal terms), including an explicit "Website vs. Protocol Claims" boundary section. | website operations | OUT_OF_SCOPE_FOR_QSL_PROTOCOL | The boundary section correctly separates website operations from protocol claims; protocol metadata posture remains governed by DOC-G5-001/-003. | Preserve the boundary section. | NICE_TO_HAVE | Website legal/operational statements are not audited as protocol claims. |
| WCM-115 | All pages (absence check) | No "vulnerability-free", "formally verified" (unqualified), "audited", production-readiness, public-internet-readiness, anonymity, or metadata-elimination claim was found anywhere on the checked source. | claim boundary (absence) | SUPPORTED | Full-text source sweep at `21a908a4`; matches every required no-claim boundary in the policy sections above. | Preserve. When Phase B adds the formal-methods/ENG-0038 story, use the bounded framing from the qsl-protocol PROGRESS 2026-07-15 entry (internal review; bounded model; known unmodeled slices; external review not yet commissioned). | MUST_FIX | The absence of overclaims is the site's strongest property; protect it through Phase B. |

## Repo Evidence Consulted

All at qsl-protocol `main` commit `ac7e850c` unless noted:

- README.md, docs/public/INDEX.md, GOALS.md, ROADMAP.md, TRACEABILITY.md
- docs/governance/evidence/NA-0640_as_built.md and tests/NA-0640_e2e_integration_full_stack_testplan.md (bounded full-stack e2e: real relay + real attachments service, byte-verified, dev harness)
- tests/NA-0642_qsl_server_durability_testplan.md (qsl-server durable-queue evidence; repo evidence, not a deployed-relay claim)
- docs/governance/evidence/NA-0646_as_built.md (qsc core extracted to a linkable library with byte-identical CLI; the site's demo/CLI surface unchanged by construction)
- formal/README.md (machine-checked models run in CI, including the D572 bounded QSC.HS.* handshake-authentication model and its recorded limits)
- docs/design/DOC-G4-002_Suite2_DH_PQ_Composition_Symbolic_Analysis_ProVerif_v0.1.0_DRAFT.md
- docs/canonical/DOC-CAN-003 (Suite-2) and DOC-CAN-004 (SCKA) drafts; docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
- docs/demo/DEMO_ACCEPTANCE_CRITERIA.md; docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md
- docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md; docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
- .github/workflows/ci.yml (the qshield-ci workflow, including the handshake contract lane)
- the NA-0647 demo smoke-run record (docs/governance/evidence/NA-0647_as_built.md)
- Website source audited: the quantumshieldlabs.org repository at `main` commit `21a908a4` (all ten page components, shared sections, modals, and src/links.js)

## Top MUST_FIX Items

1. WCM-110: the Run Demos "Local Demo" instructions do not currently produce a working demo at qsl-protocol `ac7e850c` (the script pre-dates the qsc explicit vault-unlock requirement and masks the failure). Fix the script in a qsl-protocol follow-up lane, re-verify, then re-touch the row and page wording if needed.
2. WCM-103 / WCM-115: protect the site's boundary band and its absence-of-overclaims property through the Phase B content sync; any new formal-methods/ENG-0038 copy must reuse the bounded framing from the qsl-protocol PROGRESS 2026-07-15 entry.
3. WCM-112: after this refresh merges, Phase B should re-touch the Progress-and-Proof and Resources claim-matrix cards, which still describe this document as a pending historical .dev audit.
