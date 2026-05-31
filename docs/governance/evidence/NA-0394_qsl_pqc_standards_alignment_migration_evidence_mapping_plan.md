Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0394 QSL PQC Standards Alignment / Migration Evidence Mapping Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-213

## Executive Summary

NA-0394 maps QSL's current protocol, code, harness, formal, reference,
service-boundary, and governance evidence against current official
post-quantum cryptography standards and migration guidance. The mapping is an
evidence and claim-boundary lane only.

Targeted official-source verification succeeded for:

- NIST FIPS 203 / ML-KEM.
- NIST FIPS 204 / ML-DSA.
- NIST FIPS 205 / SLH-DSA.
- NIST / NCCoE migration guidance.
- NCSC migration timeline guidance.
- CISA / NIST / NCCoE quantum-readiness guidance.
- NSA CNSA 2.0 guidance.
- NIST HQC backup-KEM status.

QSL has direct code and harness evidence for ML-KEM and ML-DSA use in current
Suite-2 / qsc / reference-implementation areas, but NA-0394 does not classify
that posture as standards conformance, validation, certification, production
readiness, public-internet readiness, or external review completion. SLH-DSA
and HQC remain not claimed as QSL current implementations. Migration guidance
is currently governance-planned and evidence-incomplete, not operationally
complete.

No immediate blocker was selected. The exact selected successor is:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

## Live NA-0394 Scope

Live `NEXT_ACTIONS.md` lists NA-0394 as READY with the objective to create a
qsl-protocol governance evidence map comparing QSL's current protocol/evidence
posture with NIST PQC standards and migration guidance, while making no
standards conformance, certification, production-readiness, or
crypto-implementation-change claim.

Allowed qsl-protocol mutation for the mapping packet is limited to:

- `docs/governance/evidence/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan.md`
- `tests/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope includes runtime, service, protocol, crypto, dependency,
workflow, public docs, website, qsl-server, qsl-attachments, qshield runtime,
backup script/timer/fstab/service, qstart/qresume, response archive, and secret
handling changes.

Acceptance criteria for this packet:

- READY_COUNT remains exactly one with READY NA-0394 until closeout.
- NA-0393 is DONE.
- D-0768 and D-0769 exist once.
- D-0770 is added once by NA-0394.
- Official PQC sources are cited with source tiers and stability
  classifications.
- QSL current evidence posture is mapped conservatively.
- Missing evidence and public-claim boundaries are recorded.
- Exactly one NA-0395 successor is selected.
- Public-safety and required validation remain green.

## Inherited NA-0393 Triage Rationale

NA-0393 triaged NA-0392 findings F-0392-01 through F-0392-10. It selected PQC
standards alignment because final NIST standards and official migration
guidance are foundational for QSL credibility, later public claims, code/crypto
audit planning, and eventual public technical paper readiness.

NA-0393 recorded:

- `NO_CRITICAL_HIGH_BLOCKER_SELECTED`.
- PQC standards and migration as the next evidence-mapping group.
- IETF/CFRG RFC versus draft boundary as the expected next group after PQC
  mapping.
- Dependency/advisory, code/crypto research, metadata privacy, backup/restore,
  external-review, public-claim, and public-paper prerequisites as future
  candidates.

NA-0394 inherits those boundaries and does not implement crypto or change
runtime behavior.

## Authoritative PQC Source Verification

Access date for all sources in this section: 2026-05-31.

| Source | Authority / publisher | URL | Tier | Stability classification | QSL relevance | Claim-boundary implication |
|---|---|---|---|---|---|---|
| FIPS 203, Module-Lattice-Based Key-Encapsulation Mechanism Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/203/final | Tier 1 | `FINAL_STANDARD` | Authoritative ML-KEM standard. QSL Suite-2 and qsc evidence references ML-KEM-768 for key establishment. | Direct code names are not enough for standards conformance, validation, certification, or production-readiness claims. |
| FIPS 204, Module-Lattice-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/204/final | Tier 1 | `FINAL_STANDARD` | Authoritative ML-DSA standard. qsc/reference evidence includes ML-DSA-65 signing paths. | Current evidence does not prove standards conformance, validation, certification, or external review. |
| FIPS 205, Stateless Hash-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/205/final | Tier 1 | `FINAL_STANDARD` | Authoritative SLH-DSA standard. Relevant to future signature agility and claim boundaries. | QSL does not claim current SLH-DSA support. |
| Migration to Post-Quantum Cryptography project | NIST NCCoE | https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc | Tier 1 | `OFFICIAL_MIGRATION_RESOURCE` | Official migration project emphasizing discovery, inventory, prioritization, and migration practices. | QSL governance mapping is not migration completion or operational readiness. |
| SP 800-227, Recommendations for Key-Encapsulation Mechanisms | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/227/final | Tier 1 | `OFFICIAL_GUIDANCE` | KEM usage and transition guidance, including post-quantum KEM context. | Supports future KEM usage review; does not validate current QSL implementation. |
| PQC Migration Timelines | NCSC | https://www.ncsc.gov.uk/guidance/pqc-migration-timelines | Tier 1 | `OFFICIAL_GUIDANCE` | Official migration timeline guidance with staged inventory and migration expectations. | QSL needs inventory and planning evidence before public migration claims. |
| Quantum-Readiness: Migration to Post-Quantum Cryptography Fact Sheet | CISA, NIST, NCCoE | https://www.nccoe.nist.gov/publications/fact-sheet/quantum-readiness-migration-post-quantum-cryptography-fact-sheet | Tier 1 | `OFFICIAL_MIGRATION_RESOURCE` | Joint readiness guidance emphasizing cryptographic inventory and prioritized transition. | Current governance evidence is planning support only. |
| Commercial National Security Algorithm Suite 2.0 and Quantum Computing FAQ | NSA | https://media.defense.gov/2024/Dec/19/2003619020/-1/-1/0/CNSA%202.0%20FAQ.PDF | Tier 1 | `OFFICIAL_GUIDANCE` | Official migration guidance for CNSA 2.0 contexts and timelines. | QSL does not claim CNSA 2.0 compliance or deployment readiness. |
| NIST Post-Quantum Cryptography project page | NIST CSRC | https://csrc.nist.gov/Projects/post-quantum-cryptography | Tier 1 | `BACKUP_ALGORITHM_STATUS` | Official NIST status page noting HQC selection as an additional KEM. | HQC is watch-status for QSL; not current implementation or final QSL requirement. |
| NIST Selects HQC as Fifth Algorithm for Post-Quantum Encryption | NIST | https://www.nist.gov/news-events/news/2025/03/nist-selects-hqc-fifth-algorithm-post-quantum-encryption | Tier 1 | `BACKUP_ALGORITHM_STATUS` | Official NIST announcement identifying HQC as backup to ML-KEM and describing planned standardization. | Do not treat HQC as final, implemented, or required in QSL current posture. |

Citation gap / uncertainty:

- NA-0394 found official NIST HQC selection and planned standardization
  evidence, but did not verify a final HQC/FIPS 206 standard. HQC remains a
  backup-algorithm watch item in this map.

## QSL Current Crypto / Protocol / Evidence Inventory

Read-only inspection covered qsl-protocol artifacts under `qsl/`, `tools/`,
`apps/`, `formal/`, `inputs/`, `docs/canonical/`, `docs/governance/evidence/`,
`tests/`, and `scripts/`.

| Area | Evidence observed | Evidence class | Boundary |
|---|---|---|---|
| Canonical Suite-2 protocol docs | `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md` references X25519, ML-KEM-768, AES-256-GCM, SHA-512, KMAC-256, hybrid message-key derivation, and fail-closed Suite-2 binding rules. | `GOVERNANCE_PLANNED`; protocol design evidence | Draft canonical evidence is not standards conformance or runtime proof by itself. |
| Canonical SCKA docs | `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md` records ML-KEM-768 parameter sizes, SCKA operations, transactional fail-closed behavior, and deterministic fixture categories. | `GOVERNANCE_PLANNED`; harness planning | Draft design and fixture descriptions do not prove FIPS validation. |
| Reference implementation | `tools/refimpl/quantumshield_refimpl` has `pqcrypto-mlkem` and `ml-dsa` feature-gated code paths and tests. | direct implementation evidence; `REFERENCE_ORACLE_ONLY` support | Evidence is useful for internal alignment but not an external compliance claim. |
| qsc client | `qsl/qsl-client/qsc` depends on the reference implementation with `pqcrypto`; handshake code references ML-KEM and ML-DSA helpers. | direct implementation evidence; `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` | Needs conformance mapping, KATs, audit, and public-claim review before external claims. |
| Vectors / inputs | `inputs/suite2/vectors/*` includes Suite-2 / SCKA / KDF / negotiation / PQ reseed vector files, including an SCKA ML-KEM fixture set. | `HARNESS_ONLY`; `REFERENCE_ORACLE_ONLY` support | Harness vectors support regression evidence, not standards validation. |
| Formal models | `formal/model_qsc_handshake_suite_id_bounded.py`, `formal/model_suite2_negotiation_bounded.py`, and `formal/run_model_checks.py` cover bounded negotiation and no-mutation properties. | `FORMAL_MODEL_ONLY` support | Formal models do not prove cryptographic strength or final standards conformance. |
| qshield demo | `apps/qshield-cli` remains demo-local and uses explicit unauthenticated override boundaries. | `HARNESS_ONLY` / demo evidence | qshield demo evidence is not production proof. |
| Metadata runtime harnesses | Prior governance evidence records local measurement and no-secret harnesses. | `HARNESS_ONLY` | No metadata-free, anonymity, or untraceable claim is allowed. |
| qsl-server / qsl-attachments | PR #56 and PR #37 remain service-local prerequisite evidence only. | service-boundary evidence | Not production, public-internet, or external-review proof. |
| Governance evidence | NA-0392 and NA-0393 source-watch/triage evidence exists. | `GOVERNANCE_PLANNED` | Useful for queue selection and non-claims, not implementation proof. |

Searches found no direct SLH-DSA, SPHINCS+, or HQC implementation evidence in
current inspected qsl-protocol artifacts beyond governance/source-watch
mentions.

## FIPS 203 / ML-KEM Alignment Map

| Field | Result |
|---|---|
| Official standard status | FIPS 203 is a NIST final standard for ML-KEM. |
| QSL relevance | QSL Suite-2 and qsc evidence use ML-KEM-768 for key establishment and sparse continuous key agreement support. |
| Current QSL evidence | Canonical Suite-2/SCKA docs, qsc/reference code references, `pqcrypto-mlkem` dependency tree, Suite-2/SCKA vectors, and bounded formal negotiation evidence. |
| Classification | `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` with supporting `HARNESS_ONLY`, `FORMAL_MODEL_ONLY`, and `REFERENCE_ORACLE_ONLY` evidence. |
| Missing evidence | FIPS 203 conformance vector mapping, final provider/version review, KAT coverage, side-channel/constant-time review, full code/crypto audit, external review, deployment boundary, and public-claim audit. |
| Public-claim boundary | QSL may cite FIPS 203 as external standards context, but must not claim FIPS validation, standards conformance, certification, production readiness, or public-internet readiness from current evidence. |
| Future candidate action | Future code/crypto audit and conformance-evidence lane after IETF/CFRG boundary and audit prerequisites. |
| Code change required now | No. |
| Compliance / certification claim allowed | No. |
| Public technical paper implication | Future paper may cite FIPS 203 as authoritative context with explicit caveats and after prerequisite evidence is current. |

## FIPS 204 / ML-DSA Alignment Map

| Field | Result |
|---|---|
| Official standard status | FIPS 204 is a NIST final standard for ML-DSA. |
| QSL relevance | qsc/reference evidence includes ML-DSA-65 signing and verification paths for handshake-related authentication evidence. |
| Current QSL evidence | `tools/refimpl/quantumshield_refimpl` feature-gated ML-DSA code, qsc handshake references, helper marker evidence, and regression tests around Suite-2 negotiation/handshake boundaries. |
| Classification | `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` with supporting harness evidence. |
| Missing evidence | FIPS 204 conformance vector mapping, final crate/provider maturity review, domain separation/context binding review, side-channel/constant-time review, code/crypto audit, external review, and claim audit. |
| Public-claim boundary | QSL may cite FIPS 204 as external standards context only; no FIPS validation, standards conformance, certification, or deployment-readiness claim is supported. |
| Future candidate action | Future code/crypto audit and signature-binding review lane. |
| Code change required now | No. |
| Compliance / certification claim allowed | No. |
| Public technical paper implication | Future paper may cite FIPS 204 only with current evidence boundaries and after prerequisite review lanes. |

## FIPS 205 / SLH-DSA Alignment Map

| Field | Result |
|---|---|
| Official standard status | FIPS 205 is a NIST final standard for SLH-DSA. |
| QSL relevance | Relevant to future signature agility and public-claim discipline. |
| Current QSL evidence | No direct SLH-DSA implementation, harness, formal, or reference/oracle evidence found in the inspected qsl-protocol artifacts. |
| Classification | `NOT_IMPLEMENTED`; `NOT_CLAIMED`. |
| Missing evidence | Any SLH-DSA design decision, dependency/provider review, implementation evidence, vectors, formal/harness evidence, code/crypto audit, and claim review. |
| Public-claim boundary | QSL must not claim SLH-DSA support. |
| Future candidate action | Optional future signature-agility assessment after higher-priority protocol boundary and audit lanes. |
| Code change required now | No. |
| Compliance / certification claim allowed | No. |
| Public technical paper implication | Future paper may cite FIPS 205 as external standards context, not as current QSL capability. |

## HQC / Backup KEM Status Map

| Field | Result |
|---|---|
| Official status | NIST selected HQC as an additional backup KEM and announced planned standardization; NA-0394 did not verify a final HQC/FIPS 206 standard. |
| Stability classification | `BACKUP_ALGORITHM_STATUS`. |
| QSL relevance | HQC may affect future KEM agility and backup-KEM planning. |
| Current QSL evidence | No direct HQC implementation, harness, formal, or reference/oracle evidence found in inspected artifacts. |
| Classification | `GOVERNANCE_PLANNED`; `NOT_IMPLEMENTED`; `NOT_CLAIMED`. |
| Missing evidence | Final official standard verification, design decision, dependency/provider review, implementation, vectors, formal/harness support, code/crypto audit, and claim audit. |
| Claim boundary | Do not treat HQC as current QSL support, a final QSL requirement, or standards conformance proof. |
| Future candidate action | Keep HQC in standards-watch and future KEM-agility backlog; do not implement in NA-0394. |

## Migration Guidance Alignment Map

| Guidance source | Current QSL state | Classification | Missing evidence | Future candidate |
|---|---|---|---|---|
| NIST / NCCoE migration project | NA-0394 creates an evidence map and identifies inventory needs. | `GOVERNANCE_PLANNED` | Complete cryptographic asset inventory, algorithm-use matrix, migration priority model, owner/risk mapping, and operational rollout plan. | PQC standards gap remediation or code/crypto audit lane after RFC/draft boundary. |
| NIST SP 800-227 | KEM usage guidance is cited for future review. | `GOVERNANCE_PLANNED` | Detailed KEM API, encapsulation/decapsulation, randomness, failure handling, and transition review against QSL code. | Code/crypto audit follow-up. |
| NCSC timeline guidance | Timeline is acknowledged as planning context. | `GOVERNANCE_PLANNED` | QSL-specific inventory and migration milestones; public-claim boundaries for timeline readiness. | Migration posture lane after protocol boundary. |
| CISA / NIST / NCCoE quantum readiness | Inventory/discovery/prioritization implications are recorded. | `GOVERNANCE_PLANNED` | Asset inventory, dependency inventory, owner mapping, and readiness status. | Dependency/advisory and migration posture lanes. |
| NSA CNSA 2.0 | Guidance is cited as official context for relevant environments. | `NOT_CLAIMED` for QSL conformance | Applicability assessment and explicit non-claim boundary. | Future external-guidance mapping if QSL needs CNSA-specific positioning. |

## QSL Evidence Status Matrix

| Domain | Standard / guidance source | QSL current evidence | Evidence class | Confidence | Claim allowed? | Missing evidence | Future lane | Priority |
|---|---|---|---|---|---|---|---|---|
| ML-KEM / key establishment | FIPS 203; NIST SP 800-227 | Suite-2/SCKA docs, qsc/reference code, vectors, formal negotiation evidence. | `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` | Medium | Internal evidence mapping only; no conformance, validation, certification, or production claim. | KAT/conformance mapping, audit, external review, public-claim review. | Code / crypto audit follow-up. | High |
| ML-DSA / signatures | FIPS 204 | qsc/reference ML-DSA paths and tests. | `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` | Medium | Internal evidence mapping only; no conformance, validation, certification, or production claim. | KAT/conformance mapping, provider review, audit, context-binding review. | Code / crypto audit follow-up. | High |
| SLH-DSA / stateless hash signatures | FIPS 205 | No direct inspected implementation evidence. | `NOT_IMPLEMENTED`; `NOT_CLAIMED` | High | No current QSL support claim. | Any design, implementation, vectors, audit. | Future signature-agility assessment. | Medium |
| HQC / backup KEM | NIST HQC selection/status | Governance watch only. | `GOVERNANCE_PLANNED`; `NOT_IMPLEMENTED` | High | No current QSL support claim. | Final standard verification and any QSL design/evidence. | Standards watch / KEM agility. | Medium |
| Hybrid / suite negotiation | Suite-2 docs; future IETF/CFRG lane | Canonical draft docs, qsc tests, formal bounded models. | `FORMAL_MODEL_ONLY`; `HARNESS_ONLY`; direct code support incomplete for claims | Medium | No draft-as-final or production claim. | RFC/draft boundary map, wire/protocol audit, downgrade analysis. | NA-0395 IETF/CFRG boundary. | High |
| Formal/model alignment | QSL formal models | Bounded negotiation and no-mutation models. | `FORMAL_MODEL_ONLY` | Medium | Only bounded model claims. | Cryptographic proof, complete state-machine proof, external review. | Formal expansion / audit. | Medium |
| Reference/oracle vectors | Suite-2 inputs and refimpl tests | Deterministic fixtures and regression vectors. | `REFERENCE_ORACLE_ONLY`; `HARNESS_ONLY` | Medium | Regression evidence only. | Official KAT mapping and independent oracle validation. | Code / crypto audit follow-up. | High |
| qshield demo evidence | qshield CLI docs/tests | Demo-local harness boundaries. | `HARNESS_ONLY` | High | No production, public-internet, or security-complete claim. | Production architecture, auth, service, and external review evidence. | qshield demo boundary lane. | Medium |
| qsl-server / qsl-attachments service boundary | PR #56; PR #37; governance maps | Service-local prerequisite evidence. | `HARNESS_ONLY`; service-boundary evidence | High | No production/public-internet/external-review claim. | Production deployment, threat model, ops, external review. | Service production boundary lane. | Medium |
| Migration / inventory evidence | NIST/NCCoE, NCSC, CISA, NSA | This governance map plus prior watch/triage evidence. | `GOVERNANCE_PLANNED` | Medium | No migration-complete or readiness claim. | Asset inventory, prioritization, owner/risk mapping. | Migration posture lane. | High |
| Public claim readiness | Official sources plus QSL governance | Non-claim boundaries recorded. | `GOVERNANCE_PLANNED`; `NOT_CLAIMED` | High | No expanded public claims. | Website/public docs audit, external review readiness. | Public claim readiness lane. | High |
| External review readiness | Source discovery and internal evidence | Not externally reviewed by this lane. | `NOT_CLAIMED` | High | No external-review-complete claim. | Independent review plan, disclosure policy, evidence package. | External review readiness lane. | High |

## Claim Boundary and Public Technical Paper Implications

NA-0394 records these required boundaries:

- No compliance claim.
- No certification claim.
- No FIPS validation claim.
- No standards conformance claim from mapping alone.
- No production-ready claim.
- No public-internet-ready claim.
- No external-review-complete claim.
- No metadata-free, anonymity, or untraceable claim.
- No bug-free or perfect-crypto claim.
- No local continuity backup is presented as complete disaster recovery.
- No qshield demo proof is presented as production proof.
- No service-local qsl-server/qsl-attachments proof is presented as
  public-internet proof.

A future public technical paper may cite final standards as context only after:

- PQC mapping is current.
- Code/crypto audit status is current.
- IETF/RFC versus draft boundary is mapped.
- Service, backup, restore, and key status are current.
- Public-claim boundary audit is current.
- External review readiness is assessed.

## Gap and Future Queue Candidate Analysis

| Candidate | Source finding basis | Why next / why not next | Likely allowed scope | Likely forbidden scope |
|---|---|---|---|---|
| QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan | NA-0393 F-0392-04; NA-0394 hybrid/suite boundary gaps. | Selected next because standards mapping is now complete enough for the next external-watch group: RFC versus Internet-Draft boundaries. | Governance evidence, official RFC/draft citations, claim-boundary map, testplan, D/traceability/journal. | Runtime, wire, crypto, dependency, workflow, public-doc, website, backup, sibling-repo changes. |
| QSL Dependency / Advisory Watch Trigger Policy Plan | NA-0393 F-0392-05 and F-0392-06. | Not next because audit is green and no active advisory blocker was selected. | Policy-only applicability and trigger map. | Cargo/dependency/workflow changes unless future scope authorizes. |
| QSL Code / Crypto Research Watch and Audit Follow-Up Plan | NA-0393 F-0392-07; NA-0394 KAT/audit gaps. | Important but should follow RFC/draft boundary so audit targets are scoped truthfully. | Governance audit-topic map and no-implementation triage. | Crypto implementation or key-schedule changes. |
| QSL Metadata Privacy / Secure Messaging Claim Boundary Plan | NA-0393 F-0392-08. | Not next because no metadata-free/anonymity/untraceable public claim is active. | Claim-boundary governance evidence. | qshield runtime, website, public-claim expansion. |
| QSL Backup / Restore / Key Custody External Guidance Mapping Plan | NA-0393 F-0392-09. | Not next because off-host target/host-identity input remains blocked. | Governance mapping and backup-impact assessment. | Backup script/timer/fstab/service, keys, restore, remote target setup. |
| QSL External Review / Disclosure / Public Claim Readiness Plan | NA-0393 F-0392-10. | Not next because standards/RFC/audit prerequisites remain incomplete. | Readiness criteria and evidence prerequisites. | Claiming review completion or publishing public claims. |
| QSL Public Technical Position Paper Evidence Prerequisite Plan | NA-0392/NA-0393/NA-0394 combined gaps. | Not next because RFC/draft, audit, service, backup, and claim boundaries still need work. | Prerequisite checklist and evidence gate. | Drafting or publishing public technical paper claims. |
| QSL PQC Standards Gap Remediation Lanes | NA-0394 ML-KEM/ML-DSA evidence-incomplete gaps and SLH-DSA/HQC non-claims. | Not next because no immediate critical blocker was found; remediation should be scoped after protocol boundary and audit plans. | Governance and test/vector planning only unless future scope authorizes exact files. | Any unscoped crypto/runtime/dependency changes. |

## Selected Successor

Selected successor:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

Rationale:

- Official PQC source verification succeeded.
- No immediate standards-alignment blocker was found that requires an emergency
  PQC gap-resolution successor.
- The next external-watch group from NA-0393 is RFC versus Internet-Draft
  protocol boundary work.
- This successor helps prevent draft-as-final, production-readiness, and
  public-claim overstatements before future audit or paper work.

Rejected successors:

- `NA-0395 -- QSL PQC Standards Alignment Critical Evidence Gap Resolution`:
  rejected because NA-0394 found evidence gaps but no immediate critical
  blocker.
- `NA-0395 -- QSL PQC Standards Source Verification / Evidence Mapping Blocker
  Resolution`: rejected because official-source verification was sufficient for
  conservative mapping.

## Future Path / Scope Bundle

If the selected successor is restored, expected NA-0395 allowed paths are:

- `docs/governance/evidence/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan.md`
- `tests/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0395 forbidden scope:

- runtime code.
- crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- dependencies.
- workflows.
- public docs/website.
- backup scripts.
- response archives.
- external claims.

Future NA-0395 may use targeted web/source verification only to cite and
classify RFCs and Internet-Drafts if live scope authorizes it.

## Public Claim / External Review / Website Boundary

Evidence mapping is not implementation. Standards mapping is not compliance,
certification, validation, or external review. Source discovery is not external
review. NA-0394 makes no website or public-doc update and authorizes no
production, public-internet, metadata-free, anonymity, untraceable, bug-free, or
perfect-crypto claim.

## Future Validation / Marker Plan

If the selected normal successor is restored, future NA-0395 should include
markers equivalent to:

- `NA0395_IETF_CFRG_PROTOCOL_BOUNDARY_PLAN_OK`
- `NA0395_RFC8446_REFERENCE_OK`
- `NA0395_RFC9180_REFERENCE_OK`
- `NA0395_RFC9420_REFERENCE_OK`
- `NA0395_TLS_HYBRID_DRAFT_CAVEAT_OK`
- `NA0395_HPKE_PQ_DRAFT_CAVEAT_OK`
- `NA0395_CFRG_HYBRID_KEM_DRAFT_CAVEAT_OK`
- `NA0395_MLS_PQ_DRAFT_CAVEAT_OK`
- `NA0395_NO_DRAFT_AS_FINAL_STANDARD_OK`
- `NA0395_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0395_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0395_NO_METADATA_FREE_CLAIM_OK`
- `NA0395_NO_ANONYMITY_CLAIM_OK`
- `NA0395_NO_UNTRACEABLE_CLAIM_OK`
- `NA0395_NO_RUNTIME_CHANGE_OK`
- `NA0395_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0395_NO_DEPENDENCY_CHANGE_OK`
- `NA0395_NO_SECRET_MATERIAL_OK`

## Rejected Alternatives

NA-0394 rejects:

- implementing PQC changes now.
- changing qsc/qsp/qsl crypto now.
- changing dependencies now.
- changing workflows now.
- writing public docs now.
- starting a public technical paper now.
- treating mapping as compliance.
- treating mapping as certification.
- treating source discovery as external review.
- treating qshield demo evidence as production proof.
- treating service-local qsl-server/qsl-attachments evidence as public-internet
  proof.

## Backup-Plan Impact Statement

NA-0394 changes only tracked qsl-protocol governance/testplan/traceability/
journal files. No backup scripts, timers, fstab entries, services, keys,
passphrases, restore paths, remote targets, monitoring configuration, or
response archive files are changed.

No backup-plan update is required for this governance-only mapping. Future
durable PQC reports outside tracked governance evidence, durable audit stores,
or changes to local Codex history roots would require separate backup-impact
review.

Local `/backup/qsl` continuity remains same-host continuity and is not complete
disaster recovery.

## Next Recommendation

After NA-0394 merges and closes out, restore exactly one READY item:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

NA-0395 should be governance-only unless future live scope explicitly
authorizes exact additional paths. It should separate final RFCs from
Internet-Drafts and preserve all non-claim boundaries.

## Source List

| Title | Publisher | URL | Access date | Source tier | Stability classification | Relevance |
|---|---|---|---|---|---|---|
| FIPS 203, Module-Lattice-Based Key-Encapsulation Mechanism Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/203/final | 2026-05-31 | Tier 1 | `FINAL_STANDARD` | ML-KEM authority for QSL key-establishment mapping. |
| FIPS 204, Module-Lattice-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/204/final | 2026-05-31 | Tier 1 | `FINAL_STANDARD` | ML-DSA authority for QSL signature mapping. |
| FIPS 205, Stateless Hash-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/205/final | 2026-05-31 | Tier 1 | `FINAL_STANDARD` | SLH-DSA authority for future signature-agility boundaries. |
| Migration to Post-Quantum Cryptography | NIST NCCoE | https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc | 2026-05-31 | Tier 1 | `OFFICIAL_MIGRATION_RESOURCE` | Migration inventory/prioritization guidance. |
| SP 800-227, Recommendations for Key-Encapsulation Mechanisms | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/227/final | 2026-05-31 | Tier 1 | `OFFICIAL_GUIDANCE` | KEM transition and usage guidance. |
| PQC Migration Timelines | NCSC | https://www.ncsc.gov.uk/guidance/pqc-migration-timelines | 2026-05-31 | Tier 1 | `OFFICIAL_GUIDANCE` | Migration timeline planning context. |
| Quantum-Readiness: Migration to Post-Quantum Cryptography Fact Sheet | CISA, NIST, NCCoE | https://www.nccoe.nist.gov/publications/fact-sheet/quantum-readiness-migration-post-quantum-cryptography-fact-sheet | 2026-05-31 | Tier 1 | `OFFICIAL_MIGRATION_RESOURCE` | Readiness and inventory guidance. |
| Commercial National Security Algorithm Suite 2.0 and Quantum Computing FAQ | NSA | https://media.defense.gov/2024/Dec/19/2003619020/-1/-1/0/CNSA%202.0%20FAQ.PDF | 2026-05-31 | Tier 1 | `OFFICIAL_GUIDANCE` | CNSA 2.0 migration context and non-claim boundary. |
| Post-Quantum Cryptography project page | NIST CSRC | https://csrc.nist.gov/Projects/post-quantum-cryptography | 2026-05-31 | Tier 1 | `BACKUP_ALGORITHM_STATUS` | HQC selection/status context. |
| NIST Selects HQC as Fifth Algorithm for Post-Quantum Encryption | NIST | https://www.nist.gov/news-events/news/2025/03/nist-selects-hqc-fifth-algorithm-post-quantum-encryption | 2026-05-31 | Tier 1 | `BACKUP_ALGORITHM_STATUS` | HQC backup-KEM standardization status. |
