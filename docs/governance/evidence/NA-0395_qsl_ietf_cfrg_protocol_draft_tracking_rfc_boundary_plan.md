Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0395 QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-215

## Executive Summary

NA-0395 maps QSL's current protocol and evidence posture against stable RFCs
and active IETF/IRTF draft work for TLS, HPKE, MLS, and post-quantum or
hybrid key establishment. This is a governance evidence and boundary lane only.

Targeted official-source verification succeeded for:

- RFC 8446 / TLS 1.3.
- RFC 9180 / HPKE.
- RFC 9420 / MLS.
- TLS hybrid and post-quantum key exchange drafts.
- HPKE post-quantum and hybrid KEM drafts.
- CFRG hybrid KEM draft work.
- MLS post-quantum ciphersuite draft work.
- replaced and expired predecessor drafts needed to avoid stale mapping.

The result is conservative:

- QSL does not claim TLS 1.3 implementation, TLS compliance, HPKE
  implementation, HPKE compliance, MLS implementation, or MLS compliance.
- QSL has direct implementation and harness evidence for its own QSP/Suite-2
  and qsc/qshield surfaces, plus formal/model and reference/oracle support.
- QSL's current Suite-2 evidence is adjacent to KEM, hybrid, suite-id,
  transcript, and key-schedule concerns, but it is not evidence that QSL
  implements TLS, HPKE, MLS, or any active Internet-Draft.
- Internet-Drafts remain draft/watch inputs only unless and until a future lane
  explicitly updates the boundary after RFC publication and code/evidence
  review.

No critical protocol-boundary blocker was found. The exact selected successor
is:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

## Live NA-0395 Scope

Live `NEXT_ACTIONS.md` lists NA-0395 as READY with the objective to create a
qsl-protocol governance plan that tracks IETF/CFRG RFC versus Internet-Draft
boundaries for TLS, HPKE, MLS, and post-quantum hybrid protocol work while
making no draft-as-final, implementation, production-readiness, or public-claim
expansion.

Allowed qsl-protocol mutation for this evidence packet is limited to:

- `docs/governance/evidence/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan.md`
- `tests/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope includes runtime code, protocol implementation, crypto
implementation, dependency changes, workflow changes, qsl-server,
qsl-attachments, qshield runtime, website/public docs, README/START_HERE,
backup scripts/timers/fstab/services, qstart/qresume tools, response archives,
request/directive/history files, and secret handling.

Acceptance criteria:

- READY_COUNT remains exactly one with READY NA-0395 until closeout.
- NA-0394 is DONE.
- D-0770 and D-0771 exist once.
- D-0772 is added once by NA-0395.
- D-0773 remains absent until optional closeout.
- Official RFC/IETF/CFRG sources are cited with source tiers and stability
  classifications.
- QSL current protocol evidence posture is mapped conservatively.
- Missing evidence and claim boundaries are recorded.
- Exactly one NA-0396 successor is selected.
- Public-safety and required validation remain green.

## Inherited NA-0394 Rationale

NA-0394 mapped QSL's current posture against NIST PQC standards and migration
guidance. It classified ML-KEM and ML-DSA as
`IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE`, SLH-DSA and HQC as `NOT_IMPLEMENTED` /
`NOT_CLAIMED`, and migration posture as `GOVERNANCE_PLANNED`.

NA-0394 selected NA-0395 because, after final PQC standards mapping, QSL needs
an explicit RFC-versus-draft boundary for protocol ecosystems that may be cited
in future public technical material or used as future design inputs. NA-0394
made no crypto, runtime, dependency, workflow, public-doc, website, backup, or
sibling-repo changes.

The NA-0395 boundary objective is therefore:

- distinguish stable RFCs from active, expired, or replaced drafts;
- map QSL evidence to current protocol claims without overclaiming;
- keep draft tracking as watch/governance input, not final-standard input;
- identify future candidate lanes; and
- select a single NA-0396 successor.

The future Project Goal / Operating Principles canon lane requested by the
operator is carried forward as a future governance candidate only and does not
override NA-0395 or the selected NA-0396 successor.

## Authoritative RFC / Draft Source Verification

Access date for all sources in this section: 2026-05-31.

| Source | Authority / publisher | URL | Tier | Stability classification | Current status | QSL relevance | Claim-boundary implication |
|---|---|---|---|---|---|---|---|
| RFC 8446, The Transport Layer Security (TLS) Protocol Version 1.3 | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc8446 | Tier 1 official RFC source | `RFC` | Proposed Standard RFC, Standards Track | Baseline TLS 1.3 reference for handshake, transcript, key schedule, exporters, negotiation, and supported groups. | QSL may cite as protocol context only; no TLS implementation or TLS compliance claim is supported. |
| RFC 9180, Hybrid Public Key Encryption | RFC Editor / IETF / CFRG | https://www.rfc-editor.org/rfc/rfc9180 | Tier 1 official RFC source | `RFC` | Informational RFC from CFRG consensus | Baseline HPKE reference for KEM/KDF/AEAD suite structure and HPKE terminology. | QSL KEM-adjacent evidence is not HPKE implementation or HPKE compliance proof. |
| RFC 9420, The Messaging Layer Security (MLS) Protocol | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9420 | Tier 1 official RFC source | `RFC` | Proposed Standard RFC, Standards Track | Baseline MLS group key establishment and secure group messaging reference. | QSL secure messaging evidence is not MLS implementation or MLS compliance proof. |
| draft-ietf-tls-hybrid-design, Hybrid key exchange in TLS 1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-hybrid-design/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `FINAL_STANDARD_ADJACENT` | Active TLS WG draft; Datatracker shows RFC Editor queue / in progress, but not RFC publication. | Generic TLS 1.3 hybrid key-exchange design input. | Treat as work in progress until RFC publication; no draft-as-final or implementation claim. |
| draft-ietf-tls-ecdhe-mlkem, Post-quantum hybrid ECDHE-MLKEM Key Agreement for TLSv1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT` | Active TLS WG draft; Datatracker shows IETF Last Call ending 2026-06-09 and RFC Editor blocked. | Concrete TLS hybrid groups using ML-KEM with ECDHE. | Watch item only for QSL; QSL does not claim these TLS named groups. |
| draft-ietf-tls-mlkem, ML-KEM Post-Quantum Key Agreement for TLS 1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-mlkem/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT` | Active TLS WG draft; WG document with revised I-D needed after WGLC issue. | Pure ML-KEM TLS 1.3 key-establishment direction. | Watch item only; QSL Suite-2 ML-KEM evidence is not TLS ML-KEM draft implementation proof. |
| draft-ietf-hpke-pq, Post-Quantum and Post-Quantum/Traditional Hybrid Algorithms for HPKE | IETF Datatracker / HPKE WG | https://datatracker.ietf.org/doc/draft-ietf-hpke-pq/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `EVIDENCE_INCOMPLETE` for intended-status metadata conflict | Active HPKE WG draft. Datatracker metadata shows no intended RFC status, while the draft text says Standards Track. | Defines PQ and hybrid KEM algorithms for HPKE. | Record conflict and do not over-resolve; QSL must not claim HPKE PQ/hybrid implementation. |
| draft-irtf-cfrg-hybrid-kems, Hybrid PQ/T Key Encapsulation Mechanisms | IETF Datatracker / IRTF CFRG | https://datatracker.ietf.org/doc/draft-irtf-cfrg-hybrid-kems/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `IRTF_RESEARCH_GROUP_DRAFT` | Active CFRG RG document, intended Informational. | Generic hybrid-KEM combiner input for future design comparison. | Research-group draft input only; not a final standard or QSL implementation proof. |
| draft-ietf-mls-pq-ciphersuites, ML-KEM and Hybrid Cipher Suites for Messaging Layer Security | IETF Datatracker / MLS WG | https://datatracker.ietf.org/doc/draft-ietf-mls-pq-ciphersuites/ | Tier 1 official status source | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `EVIDENCE_INCOMPLETE` for intended-status metadata conflict | Active MLS WG draft; Datatracker shows WG chair go-ahead wait and revised I-D needed. Datatracker metadata says Proposed Standard while draft text says Informational. | PQ and hybrid MLS ciphersuite direction. | Record conflict and do not over-resolve; no MLS PQ ciphersuite implementation claim. |
| draft-kwiatkowski-tls-ecdhe-mlkem | IETF Datatracker | https://datatracker.ietf.org/doc/draft-kwiatkowski-tls-ecdhe-mlkem/ | Tier 1 official status source | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Replaced by `draft-ietf-tls-ecdhe-mlkem`; expired and archived. | Prevents stale citation to individual TLS ECDHE-MLKEM draft. | Cite current WG draft instead. |
| draft-stebila-tls-hybrid-design | IETF Datatracker | https://datatracker.ietf.org/doc/draft-stebila-tls-hybrid-design/ | Tier 1 official status source | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Replaced by `draft-ietf-tls-hybrid-design`; expired and archived. | Prevents stale citation to individual TLS hybrid design draft. | Cite current WG draft instead. |
| draft-barnes-hpke-pq | IETF Datatracker | https://datatracker.ietf.org/doc/draft-barnes-hpke-pq/ | Tier 1 official status source | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Replaced by `draft-ietf-hpke-pq`; expired and archived. | Prevents stale citation to individual HPKE PQ draft. | Cite current WG draft instead. |
| draft-mahy-mls-pq | IETF Datatracker | https://datatracker.ietf.org/doc/draft-mahy-mls-pq/ | Tier 1 official status source | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Replaced by `draft-ietf-mls-pq-ciphersuites`; expired and archived. | Prevents stale citation to individual MLS PQ draft. | Cite current WG draft instead. |

Citation gaps / uncertainty:

- `draft-ietf-hpke-pq` and `draft-ietf-mls-pq-ciphersuites` have status
  metadata that differs from intended-status text in the draft body. NA-0395
  records this as source-status uncertainty and does not over-resolve it.
- `draft-ietf-tls-hybrid-design` is in the RFC Editor queue, but NA-0395
  treats it as an Internet-Draft until an RFC Editor RFC page exists.

## QSL Current Protocol / Handshake / Suite-ID Evidence Inventory

Read-only inspection covered qsl-protocol artifacts under `qsl/`, `tools/`,
`apps/`, `formal/`, `inputs/`, `docs/canonical/`,
`docs/governance/evidence/`, `tests/`, `scripts/`, `DECISIONS.md`,
`TRACEABILITY.md`, and `NEXT_ACTIONS.md`.

| Area | Evidence observed | Evidence class | Boundary |
|---|---|---|---|
| Canonical Suite-2 protocol docs | `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md` defines QSP Suite-2, per-message hybrid keys, ML-KEM-768, X25519, KMAC-256, suite-id, transcript/AD binding, and key-schedule rules. | `GOVERNANCE_PLANNED`; protocol design evidence | This is QSL protocol evidence, not TLS, HPKE, MLS, or IETF draft implementation proof. |
| Canonical SCKA docs | `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md` defines SCKA ML-KEM-768 ADV/CTXT behavior, monotonicity, one-time targeting, fail-closed reject rules, and deterministic fixtures. | `GOVERNANCE_PLANNED`; harness planning | SCKA is QSL-specific and not an HPKE/MLS/TLS conformance claim. |
| qsc runtime and tests | `qsl/qsl-client/qsc/src/handshake/mod.rs` includes QHSM v2 suite-context handling and runtime provider helpers; `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs` covers suite context categories and transcript/key-context checks. | direct implementation evidence plus `HARNESS_ONLY` | Direct QSL/qsc evidence; still incomplete for public claims, IETF conformance, or external review. |
| Reference implementation | `tools/refimpl/quantumshield_refimpl` has `pqcrypto-mlkem` and `ml-dsa` feature-gated paths, QSP handshake logic, Suite-2 establishment helpers, and Suite-2 vector support. | direct implementation evidence; `REFERENCE_ORACLE_ONLY` support | Useful internal reference/oracle evidence; not final standards conformance or certification proof. |
| Formal models | `formal/model_suite2_negotiation_bounded.py`, `formal/model_scka_bounded.py`, `formal/model_qsc_handshake_suite_id_bounded.py`, and `formal/run_model_checks.py` cover bounded fail-closed and no-mutation slices. | `FORMAL_MODEL_ONLY` | Bounded models do not prove cryptographic secrecy, full authentication, or RFC compliance. |
| Vectors / inputs | `inputs/suite2/vectors/*` and `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json` cover KDF, transcript, downgrade, SCKA, KEM, establishment, parsing, interop, receive, and crash/restart categories. | `HARNESS_ONLY`; `REFERENCE_ORACLE_ONLY` | Regression and oracle evidence only; no RFC/draft implementation claim. |
| qshield demo | `apps/qshield-cli/README.md` explicitly describes qshield as non-production demo only with unauthenticated override boundaries. | `HARNESS_ONLY`; demo evidence | Demo evidence is not production, public-internet, external-review, or RFC-conformance proof. |
| Metadata runtime harnesses | Prior governance evidence records local measurement, no-secret, retry, jitter, batching, padding, and attachment-size harnesses. | `HARNESS_ONLY` | No metadata-free, anonymity, untraceable, timing-hidden, or traffic-shape-hidden claim. |
| qsl-server / qsl-attachments | qsl-server PR #56 and qsl-attachments PR #37 remain read-only service-local prerequisite evidence. | `HARNESS_ONLY`; service-boundary evidence | Not production, public-internet, or external-review proof. |
| Governance evidence | NA-0391 through NA-0394 source-watch, triage, PQC, and successor evidence exists. | `GOVERNANCE_PLANNED` | Queue and claim-boundary evidence only. |
| TLS / HPKE / MLS implementation evidence | Searches found governance references to RFCs/drafts and no direct QSL TLS 1.3, HPKE, or MLS implementation claim in the inspected runtime surfaces. | `NOT_CLAIMED` for TLS/HPKE/MLS | Use as context only; do not infer implementation from terminology overlap. |

## RFC 8446 / TLS 1.3 Boundary Map

| Field | Result |
|---|---|
| Official source status | RFC 8446 is a stable IETF Standards Track RFC for TLS 1.3. |
| QSL relevance | TLS 1.3 is a baseline for handshake, key share, supported groups, transcript hash, key schedule, exporter, and downgrade-boundary vocabulary. |
| Current QSL evidence | QSL has QSP/Suite-2 handshake, suite-id, transcript/AD, KEM, and key-schedule evidence in QSL-specific docs/code/tests/models. |
| QSL implements TLS 1.3? | `NOT_CLAIMED`. No current NA-0395 evidence proves a TLS 1.3 stack or RFC 8446 implementation. |
| Evidence class | QSL-specific direct/harness/formal/reference evidence only; RFC 8446 status is external context. |
| Missing evidence | TLS stack implementation mapping, RFC 8446 requirement matrix, TLS named-group handling, TLS transcript/key schedule proofs, TLS interoperability, RFC test vectors, audit, external review, and public-claim review. |
| Public-claim boundary | No TLS compliance, TLS implementation, production-ready, public-internet-ready, or external-review-complete claim is allowed. |
| Future candidate action | Keep TLS RFC/draft boundary as watch input; if future QSL work touches TLS, require a dedicated TLS conformance/evidence lane. |

## RFC 9180 / HPKE Boundary Map

| Field | Result |
|---|---|
| Official source status | RFC 9180 is a stable RFC for Hybrid Public Key Encryption from CFRG consensus. |
| QSL relevance | HPKE defines KEM/KDF/AEAD suite structure and terminology relevant to PQ/hybrid HPKE drafts and future comparison. |
| Current QSL evidence | QSL has KEM-adjacent ML-KEM-768, KDF, AEAD, and suite evidence, but no direct HPKE API or RFC 9180 suite implementation was found in inspected runtime surfaces. |
| QSL implements HPKE? | `NOT_CLAIMED`. |
| Evidence class | KEM-adjacent direct implementation and harness evidence; HPKE-specific evidence missing. |
| Missing evidence | HPKE mode/API mapping, HPKE KEM/KDF/AEAD ID mapping, sender/recipient setup tests, RFC vectors, HPKE auth-mode review, PQ HPKE draft applicability, audit, external review, and claim review. |
| Public-claim boundary | No HPKE compliance, HPKE implementation, HPKE PQ/hybrid implementation, production-ready, or external-review-complete claim is allowed. |
| Future candidate action | Future HPKE applicability/conformance lane only if QSL chooses to implement or depend on HPKE. |

## RFC 9420 / MLS Boundary Map

| Field | Result |
|---|---|
| Official source status | RFC 9420 is a stable IETF Standards Track RFC for MLS. |
| QSL relevance | MLS is the main IETF group secure messaging protocol and provides useful context for group keying, credentials, ciphersuites, exporter labels, and metadata boundaries. |
| Current QSL evidence | QSL has two-party and service-boundary secure messaging evidence, Suite-2 ratchet evidence, and metadata/privacy governance evidence, but no MLS protocol implementation evidence was found. |
| QSL implements MLS? | `NOT_CLAIMED`. |
| Evidence class | Secure-messaging-adjacent governance/harness/formal evidence; MLS-specific evidence missing. |
| Missing evidence | MLS architecture decision, tree/group-state implementation, MLS credential/ciphersuite mapping, RFC 9420 test vectors, interop, PQ MLS draft applicability, audit, external review, and claim review. |
| Public-claim boundary | No MLS compliance, MLS implementation, group-MLS compatibility, production-ready, or external-review-complete claim is allowed. |
| Future candidate action | Track MLS PQ ciphersuite drafts as watch input; implement no MLS work in NA-0395. |

## TLS Hybrid / PQ Draft Boundary Map

| Draft | Current official status | QSL relevance | QSL current state | Claim boundary | Future action |
|---|---|---|---|---|---|
| `draft-ietf-tls-hybrid-design` | Active TLS WG Internet-Draft; submitted to IESG and in RFC Editor queue, but not yet an RFC page. | Generic hybrid key-exchange framework for TLS 1.3. | `DRAFT_WATCH_ONLY`; QSL has QSL-specific hybrid evidence, not this draft implementation. | Do not treat as final; no TLS hybrid compliance claim. | Watch RFC publication and update boundary when final. |
| `draft-ietf-tls-ecdhe-mlkem` | Active TLS WG Internet-Draft, IETF Last Call in progress as of access date. | Concrete hybrid ECDHE-MLKEM TLS groups. | `DRAFT_WATCH_ONLY`; no TLS named-group implementation claim. | Do not claim X25519MLKEM768 or related TLS groups. | Watch status and IANA/RFC outcome. |
| `draft-ietf-tls-mlkem` | Active TLS WG Internet-Draft, WG document with revised I-D needed. | Pure ML-KEM TLS 1.3 key agreement. | `DRAFT_WATCH_ONLY`; QSL ML-KEM use is QSP/Suite-2, not TLS ML-KEM. | No pure-PQ TLS key agreement claim. | Watch draft status and applicability after finalization. |
| Replaced predecessors | `draft-stebila-tls-hybrid-design` and `draft-kwiatkowski-tls-ecdhe-mlkem` are expired/replaced. | Avoid stale references. | `NOT_CLAIMED`. | Do not cite replaced individual drafts as current. | Cite current WG drafts only. |

## HPKE PQ / Hybrid Draft Boundary Map

| Draft | Current official status | QSL relevance | QSL current state | Claim boundary | Future action |
|---|---|---|---|---|---|
| `draft-ietf-hpke-pq` | Active HPKE WG Internet-Draft; WG document. Intended-status metadata conflict recorded. | Defines PQ and PQ/traditional hybrid algorithms for HPKE. | `DRAFT_WATCH_ONLY`; QSL has KEM-adjacent evidence but no HPKE implementation. | No HPKE PQ/hybrid compliance or implementation claim. | Watch status; future HPKE lane only if scope authorizes. |
| `draft-barnes-hpke-pq` | Expired/replaced by `draft-ietf-hpke-pq`. | Avoid stale source use. | `NOT_CLAIMED`. | Do not cite as current. | Cite current WG draft. |

## CFRG Hybrid KEM Draft Boundary Map

| Draft | Current official status | QSL relevance | QSL current state | Claim boundary | Future action |
|---|---|---|---|---|---|
| `draft-irtf-cfrg-hybrid-kems` | Active IRTF CFRG RG document; intended Informational. | Generic hybrid PQ/traditional KEM combiner construction and terminology. | `GOVERNANCE_PLANNED`; `DRAFT_WATCH_ONLY`; QSL has QSL-specific hybrid KDF evidence. | Research-group draft is not final standard and not QSL implementation proof. | Watch RG/RFC status; compare future QSL hybrid design only in dedicated lane. |

## MLS PQ Ciphersuite Draft Boundary Map

| Draft | Current official status | QSL relevance | QSL current state | Claim boundary | Future action |
|---|---|---|---|---|---|
| `draft-ietf-mls-pq-ciphersuites` | Active MLS WG Internet-Draft; WG chair go-ahead wait and revised I-D needed. Intended-status metadata conflict recorded. | Registers ML-KEM and hybrid ciphersuites for MLS. | `DRAFT_WATCH_ONLY`; no MLS implementation evidence. | No MLS PQ ciphersuite implementation or MLS compliance claim. | Watch revised drafts and RFC outcome. |
| `draft-mahy-mls-pq` | Expired/replaced by `draft-ietf-mls-pq-ciphersuites`. | Avoid stale source use. | `NOT_CLAIMED`. | Do not cite as current. | Cite current WG draft. |

## QSL Evidence Status Matrix

| Domain | Official source | Source status | QSL current evidence | Evidence class | Confidence | Claim allowed? | Missing evidence | Future lane | Priority |
|---|---|---|---|---|---|---|---|---|---|
| TLS 1.3 / RFC 8446 | RFC 8446 | `RFC` | QSL-specific handshake/suite/key-schedule evidence only. | `NOT_CLAIMED` for TLS; QSL evidence is direct/harness/formal/reference for QSP only. | High | Context citation only. | TLS implementation, requirement mapping, vectors, interop, audit. | TLS applicability lane if needed. | Medium |
| HPKE / RFC 9180 | RFC 9180 | `RFC` | KEM-adjacent evidence, no HPKE implementation. | `NOT_CLAIMED`; KEM-adjacent direct/harness support. | High | Context citation only. | HPKE API/mode/suite mapping, vectors, audit. | HPKE applicability lane if needed. | Medium |
| MLS / RFC 9420 | RFC 9420 | `RFC` | Secure-messaging-adjacent evidence, no MLS implementation. | `NOT_CLAIMED`. | High | Context citation only. | MLS architecture, tree/group state, vectors, interop, audit. | MLS applicability lane if needed. | Medium |
| TLS hybrid/PQ drafts | TLS Datatracker drafts | active / replaced draft mix | QSL-specific hybrid evidence only. | `DRAFT_WATCH_ONLY`; `GOVERNANCE_PLANNED`. | High for source status | No draft-as-final or TLS group claim. | RFC finalization, QSL applicability review. | IETF/CFRG watch follow-up. | High |
| HPKE PQ/hybrid draft | HPKE Datatracker draft | active WG draft, metadata conflict noted | No HPKE implementation evidence. | `DRAFT_WATCH_ONLY`. | Medium | No HPKE PQ claim. | Final draft/RFC status and HPKE implementation review. | IETF/CFRG watch follow-up. | Medium |
| CFRG hybrid KEM draft | CFRG Datatracker draft | active IRTF RG draft | QSL-specific hybrid KDF evidence. | `DRAFT_WATCH_ONLY`; `GOVERNANCE_PLANNED`. | High | No final-standard claim. | RG/RFC outcome and design comparison. | Code/crypto research watch. | Medium |
| MLS PQ ciphersuite draft | MLS Datatracker draft | active WG draft, metadata conflict noted | No MLS implementation evidence. | `DRAFT_WATCH_ONLY`. | Medium | No MLS PQ claim. | Revised I-D, final status, MLS implementation review. | IETF/CFRG watch follow-up. | Medium |
| qsc handshake / suite-id evidence | QSL docs/code/tests | QSL internal evidence | qsc QHSM v2 suite-context code and NA-0313 tests. | direct implementation evidence; `HARNESS_ONLY`. | Medium | Internal QSL evidence only. | Full wire/protocol audit, external review. | Code/crypto audit follow-up. | High |
| Formal/model handshake evidence | QSL formal models | in-repo executable models | SCKA, negotiation, qsc suite-id bounded models. | `FORMAL_MODEL_ONLY`. | Medium | Bounded model claims only. | Stronger formal coverage and crypto proof. | Formal expansion lane. | Medium |
| Reference/oracle vectors | Suite-2 vectors/refimpl | in-repo regression evidence | KDF, transcript, SCKA, KEM, negotiation, interop, receive vectors. | `REFERENCE_ORACLE_ONLY`; `HARNESS_ONLY`. | Medium | Regression/oracle evidence only. | Official conformance mapping and independent review. | Code/crypto audit follow-up. | High |
| qshield demo evidence | qshield README/tests | non-production demo | Demo establish/send/recv, local relay, override boundary. | `HARNESS_ONLY`. | High | No production claim. | Production auth/service/ops review. | qshield demo boundary lane. | Medium |
| qsl-server / qsl-attachments boundary | PR #56 / PR #37 | read-only service-local evidence | Service harness evidence only. | `HARNESS_ONLY`; `BLOCKED_PENDING_SERVICE_PRODUCTION_EVIDENCE`. | High | No public-internet or external-review claim. | Production deployment, ops, threat model, external review. | Service production boundary lane. | Medium |
| Public-claim readiness | RFC/draft map plus governance evidence | evidence-only | Non-claim boundaries recorded. | `GOVERNANCE_PLANNED`; `NOT_CLAIMED`. | High | No expanded public claim. | Claim audit, external review, paper prerequisites. | Public claim readiness lane. | High |
| External-review readiness | source discovery and internal evidence | internal evidence only | No independent external review by NA-0395. | `NOT_CLAIMED`; `BLOCKED_PENDING_EXTERNAL_REVIEW`. | High | No external-review-complete claim. | Review plan, disclosure policy, external reviewers. | External review readiness lane. | High |

## Claim Boundary and Public Technical Paper Implications

NA-0395 records these required boundaries:

- No TLS compliance claim unless a future dedicated lane proves exact evidence.
- No HPKE compliance claim unless a future dedicated lane proves exact evidence.
- No MLS compliance claim unless a future dedicated lane proves exact evidence.
- No draft-as-final claim.
- No production-ready or public-internet-ready claim.
- No external-review-complete claim.
- No metadata-free, anonymity, untraceable, timing-hidden, or traffic-hidden
  claim.
- No bug-free or perfect-crypto claim.
- No compliance, certification, or validation claim from RFC/draft mapping.
- Source discovery is not external review.
- RFC/draft mapping is not implementation.

Future public technical paper work may cite RFCs and active drafts as context
only if it preserves RFC/draft caveats and current evidence boundaries. The
paper remains future-gated until PQC mapping, IETF/RFC boundary mapping,
code/crypto audit status, metadata/privacy status, backup/restore/key status,
qsl-server/qsl-attachments production boundary, public-claim audit, and
external review readiness are current.

## Gap and Future Queue Candidate Analysis

| Candidate | Source finding basis | Why next / why not next | Likely allowed scope | Likely forbidden scope |
|---|---|---|---|---|
| QSL Dependency / Advisory Watch Trigger Policy Plan | NA-0393 F-0392-05/F-0392-06; green cargo audit and rustls-webpki evidence; need advisory trigger discipline. | Selected next because PQC and IETF/RFC boundary maps are now complete enough and dependency/advisory triggers are the next triaged external-watch group. | Governance evidence, testplan, decisions, traceability, journal; advisory-source citations if authorized. | Dependency updates, Cargo changes, workflows, runtime, crypto, service, public docs, backup scripts. |
| QSL Code / Crypto Research Watch and Audit Follow-Up Plan | NA-0393 F-0392-07 plus NA-0395 missing audit/conformance evidence. | Important but should follow dependency/advisory trigger policy so advisory inputs are normalized first. | Governance/audit planning only. | Crypto changes, protocol changes, untriaged research-driven implementation. |
| QSL Metadata Privacy / Secure Messaging Claim Boundary Plan | NA-0393 F-0392-08 plus MLS metadata context and qshield demo boundaries. | Not next because current public claims remain bounded and service proof remains incomplete. | Claim-boundary evidence, metadata source mapping. | Runtime changes, website claims, anonymity/metadata-free assertions. |
| QSL Backup / Restore / Key Custody External Guidance Mapping Plan | NA-0393 F-0392-09 plus local backup same-host continuity status. | Not next because off-host target/host identity remains blocked pending operator input. | Governance mapping and backup-impact review. | Backup scripts, timers, fstab, target setup, real restore, key handling. |
| QSL External Review / Disclosure / Public Claim Readiness Plan | NA-0393 F-0392-10 and NA-0395 claim gaps. | Not next because evidence, advisory, code/crypto, metadata, service, and backup prerequisites remain incomplete. | Governance readiness criteria and prerequisite map. | Public paper, website updates, external-review-complete claims. |
| QSL Public Technical Position Paper Evidence Prerequisite Plan | Future public paper dependency on PQC/RFC/audit/service/backup/review evidence. | Not next because paper start remains premature. | Prerequisite map only. | Public drafting or publication. |
| IETF/CFRG evidence gap remediation lanes | NA-0395 found no critical blocker but did find missing TLS/HPKE/MLS conformance evidence and draft-status watch needs. | Not next because gaps are not blockers while claims remain bounded. | Narrow governance/conformance planning if later selected. | Protocol, wire, crypto, dependency, or public-claim changes without explicit future scope. |
| QSL Project Goal and Operating Principles Canon Authorization Plan | Operator carry-forward request. | Not next because the one-READY queue should continue through dependency/advisory trigger policy first. | Governance canon authorization only. | Overriding current READY queue, implementation, public claims. |

## Selected Successor

Selected successor:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

Rationale:

- NA-0394 completed PQC standards and migration mapping.
- NA-0395 completed RFC/draft boundary mapping without finding a critical
  protocol-boundary blocker.
- The next triaged external-watch group is dependency/advisory trigger policy.
- Cargo audit and rustls-webpki are currently green, so the correct next action
  is policy, not emergency remediation.

Rejected alternatives:

- `NA-0396 -- QSL IETF / CFRG Protocol Boundary Critical Evidence Gap Resolution`
  because no current claim or implementation path is blocked by a critical
  IETF/CFRG gap while claims remain bounded.
- `NA-0396 -- QSL IETF / CFRG Source Verification / Evidence Mapping Blocker Resolution`
  because official-source verification succeeded.
- Starting a public technical paper now because prerequisite evidence remains
  incomplete.
- Implementing protocol, wire, crypto, dependency, or workflow changes now
  because NA-0395 is an evidence-boundary lane only.

## Future Path / Scope Bundle

Future NA-0396 allowed paths if the normal successor is restored:

- `docs/governance/evidence/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan.md`
- `tests/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope:

- dependency changes.
- `Cargo.toml` or `Cargo.lock` changes.
- runtime code.
- crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- workflows.
- public docs or website.
- backup scripts.
- response archives.
- external claims.

Future NA-0396 may use targeted web only to cite or verify advisory sources if
live NA-0396 scope authorizes it.

## Public Claim / External Review / Website Boundary

NA-0395 does not update public docs or website content.

Required boundaries:

- RFC/draft mapping is not implementation.
- RFC mapping is not compliance.
- draft mapping is not final standard.
- source discovery is not external review.
- no website/public docs update.
- no production claim.
- no public-internet readiness claim.
- no metadata-free, anonymity, or untraceable claim.
- no bug-free or perfect-crypto claim.

## Future Validation / Marker Plan

Future NA-0396 markers if the normal successor is implemented later:

- `NA0396_DEPENDENCY_ADVISORY_TRIGGER_POLICY_PLAN_OK`
- `NA0396_RUSTSEC_WATCH_REFERENCE_OK`
- `NA0396_GHSA_WATCH_REFERENCE_OK`
- `NA0396_NVD_CVE_WATCH_REFERENCE_OK`
- `NA0396_CISA_KEV_WATCH_REFERENCE_OK`
- `NA0396_CARGO_AUDIT_LINKAGE_OK`
- `NA0396_NO_DEPENDENCY_CHANGE_OK`
- `NA0396_NO_WORKFLOW_CHANGE_OK`
- `NA0396_NO_RUNTIME_CHANGE_OK`
- `NA0396_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0396_NO_AUTOMATIC_READY_PROMOTION_OK`
- `NA0396_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0396_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0396_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Carry-Forward Note

Carry forward this future governance candidate only:

`QSL Project Goal and Operating Principles Canon Authorization Plan`

It should record QSL's north star, security-before-speed operating principle,
evidence-over-vibes rule, code and crypto excellence expectations,
no-overclaiming discipline, one-READY queue discipline, routine audits,
external awareness without hype, public technical paper timing, shorter safer
future directives, and Director/Codex/human roles. It must not override the
current READY item or selected NA-0396 successor unless a future directive
explicitly authorizes it.

## Rejected Alternatives

- Implementing TLS, HPKE, MLS, or IETF/CFRG draft behavior in NA-0395.
- Changing qsc/qsp/qsl protocol, wire, crypto, key schedule, or runtime code.
- Changing dependencies or workflows.
- Creating durable RFC/draft reports outside governance evidence.
- Updating public docs, website, README, or START_HERE.
- Treating Internet-Drafts as final standards.
- Treating RFC/draft mapping as conformance, compliance, certification,
  validation, production readiness, public-internet readiness, or external
  review.
- Promoting the Project Goal / Operating Principles canon candidate ahead of
  dependency/advisory trigger policy.

## Backup-Plan Impact Statement

NA-0395 changes only tracked qsl-protocol governance/testplan/traceability and
rolling-journal paths. It changes no backup script, timer, fstab, systemd unit,
source list, key material, passphrase, remote target, restore path, monitoring
configuration, or local qbuild tool.

No backup-plan update is required for NA-0395. Future durable RFC/draft reports,
durable external-watch outputs, public technical paper evidence stores, real
backup targets, key custody material, real restore drills, or backup source-list
changes require separate backup-impact review.

The local `/backup/qsl` status remains same-host continuity only and must not
be represented as complete disaster recovery.

## Next Recommendation

Merge this NA-0395 evidence map and, if post-merge public-safety is green,
close out NA-0395 by restoring exactly one READY successor:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

Do not implement NA-0396 in NA-0395.

## Source List

Access date for all sources: 2026-05-31.

| Title | Authority / publisher | URL | Source tier | Stability classification | Relevance |
|---|---|---|---|---|---|
| RFC 8446: The Transport Layer Security (TLS) Protocol Version 1.3 | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc8446 | Tier 1 | `RFC` | Stable TLS 1.3 baseline. |
| RFC 9180: Hybrid Public Key Encryption | RFC Editor / IETF / CFRG | https://www.rfc-editor.org/rfc/rfc9180 | Tier 1 | `RFC` | Stable HPKE baseline. |
| RFC 9420: The Messaging Layer Security (MLS) Protocol | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9420 | Tier 1 | `RFC` | Stable MLS baseline. |
| Hybrid key exchange in TLS 1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-hybrid-design/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `FINAL_STANDARD_ADJACENT` | TLS hybrid framework watch item. |
| Post-quantum hybrid ECDHE-MLKEM Key Agreement for TLSv1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-ecdhe-mlkem/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT` | TLS hybrid ML-KEM named-group watch item. |
| ML-KEM Post-Quantum Key Agreement for TLS 1.3 | IETF Datatracker / TLS WG | https://datatracker.ietf.org/doc/draft-ietf-tls-mlkem/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT` | TLS pure ML-KEM watch item. |
| Post-Quantum and Post-Quantum/Traditional Hybrid Algorithms for HPKE | IETF Datatracker / HPKE WG | https://datatracker.ietf.org/doc/draft-ietf-hpke-pq/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `EVIDENCE_INCOMPLETE` | HPKE PQ/hybrid watch item. |
| Hybrid PQ/T Key Encapsulation Mechanisms | IETF Datatracker / IRTF CFRG | https://datatracker.ietf.org/doc/draft-irtf-cfrg-hybrid-kems/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `IRTF_RESEARCH_GROUP_DRAFT` | CFRG hybrid KEM watch item. |
| ML-KEM and Hybrid Cipher Suites for Messaging Layer Security | IETF Datatracker / MLS WG | https://datatracker.ietf.org/doc/draft-ietf-mls-pq-ciphersuites/ | Tier 1 | `ACTIVE_INTERNET_DRAFT`; `WORKING_GROUP_DRAFT`; `EVIDENCE_INCOMPLETE` | MLS PQ ciphersuite watch item. |
| draft-kwiatkowski-tls-ecdhe-mlkem | IETF Datatracker | https://datatracker.ietf.org/doc/draft-kwiatkowski-tls-ecdhe-mlkem/ | Tier 1 | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Stale TLS ECDHE-MLKEM predecessor. |
| draft-stebila-tls-hybrid-design | IETF Datatracker | https://datatracker.ietf.org/doc/draft-stebila-tls-hybrid-design/ | Tier 1 | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Stale TLS hybrid predecessor. |
| draft-barnes-hpke-pq | IETF Datatracker | https://datatracker.ietf.org/doc/draft-barnes-hpke-pq/ | Tier 1 | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Stale HPKE PQ predecessor. |
| draft-mahy-mls-pq | IETF Datatracker | https://datatracker.ietf.org/doc/draft-mahy-mls-pq/ | Tier 1 | `EXPIRED_INTERNET_DRAFT`; `REPLACED_DRAFT` | Stale MLS PQ predecessor. |
