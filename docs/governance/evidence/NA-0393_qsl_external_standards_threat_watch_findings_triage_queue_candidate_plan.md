Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0393 QSL External Standards / Threat Watch Findings Triage and Queue Candidate Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-212

## Executive Summary

NA-0393 triages the ten NA-0392 source-cited external standards / threat /
technology watch findings into explicit queue candidates and selects the exact
NA-0394 successor. NA-0393 used the NA-0392 source-cited evidence as input and
did not perform a new external watch sweep.

No official CRITICAL/HIGH immediate blocker was selected for active
qsl-protocol code or dependencies. The selected successor is:

`NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`

This successor is an evidence-mapping lane only. It must not change runtime
code, crypto implementation, dependencies, workflows, public docs, website
content, qsl-server, qsl-attachments, qshield runtime, backup scripts, off-host
backup state, or secret-handling paths.

## Live NA-0393 Scope

Live `NEXT_ACTIONS.md` lists NA-0393 as READY with the objective to triage
NA-0392 source-cited findings into explicit queue candidates, claim-boundary
updates, evidence gaps, and future directive recommendations without
code/runtime/workflow/dependency mutation and without automatic READY
promotion.

Allowed qsl-protocol mutation for the triage packet is limited to:

- `docs/governance/evidence/NA-0393_qsl_external_standards_threat_watch_findings_triage_queue_candidate_plan.md`
- `tests/NA-0393_qsl_external_standards_threat_watch_findings_triage_queue_candidate_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope includes runtime, service, protocol, crypto, dependency,
workflow, public docs, website, qsl-server, qsl-attachments, qshield runtime,
backup script/timer/fstab/service, qstart/qresume, response archive, and secret
handling changes.

Acceptance criteria:

- READY_COUNT remains exactly one with READY NA-0393 until closeout.
- NA-0392 is DONE.
- D-0766 and D-0767 exist once.
- D-0768 is added once by NA-0393.
- All NA-0392 findings are triaged.
- Critical/high blocker review is recorded.
- Queue candidates are grouped.
- One exact NA-0394 successor is selected.
- Public-claim and external-review boundaries remain protected.

## Inherited NA-0392 Source-Watch Findings

NA-0392 completed a first bounded, source-cited sweep covering:

- PQC standards and migration.
- IETF/CFRG/protocol RFCs and drafts.
- Rust advisory and dependency health.
- Upstream crypto/security project release and security notes.
- Code/crypto research venues.
- Secure messaging and metadata privacy.
- Backup/restore/key custody.
- External review, disclosure, and public-claim readiness.
- Bounded adjacent and public narrative context.

NA-0392 recorded findings F-0392-01 through F-0392-10. It found no official
CRITICAL/HIGH immediate blocker affecting active qsl-protocol code or
dependencies, did not create durable external-watch report storage outside the
authorized governance evidence, and selected NA-0393 for explicit triage.

## Findings Extraction Method

Extraction source:

`docs/governance/evidence/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep.md`

Supporting sources reviewed:

- `tests/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep_testplan.md`
- `tests/NA-0392_closeout_restore_na0393_testplan.md`
- `DECISIONS.md` D-0766 and D-0767
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0393 did not use web browsing because the NA-0392 evidence already contained
source citations, source tiers, stability classifications, public-claim
implications, and blocker status sufficient for triage. No citation gap blocked
classification.

## Full Findings Triage Matrix

| Finding ID | Title / source domain | Source tier | Stability classification | Severity | Confidence | Affected project area | Current QSL evidence status | Public-claim implication | Security implication | Recommended next action | Proposed NEXT_ACTIONS candidate title | Candidate priority | Blocker? | Rationale | Required future scope | Forbidden future scope |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| F-0392-01 | PQC final standards | Tier 1 | `FINAL_STANDARD` | CLAIM_BOUNDARY | HIGH | qsc/qsp/protocol/crypto; qsl-protocol governance; public technical paper | partial | Do not claim standards conformance, validation, or migration completion from the source sweep. | Final standards are authoritative anchors for mapping, but they do not prove current QSL behavior. | Map QSL protocol/evidence posture to FIPS 203/204/205 and explicit non-claims. | QSL PQC Standards Alignment / Migration Evidence Mapping Plan | NEXT_CANDIDATE | No | Foundational Tier 1 standards affect long-term QSL credibility and claim discipline. | Governance evidence map, citations, claim boundaries, no implementation change. | Runtime code, crypto implementation, dependencies, workflows, public docs, website. |
| F-0392-02 | PQC migration guidance | Tier 1 | `OFFICIAL_GUIDANCE` | BACKLOG_CANDIDATE | HIGH | qsl-protocol governance; public technical paper; external review | partial | Do not claim migration completion or operational readiness. | Migration guidance creates planning obligations, not an immediate code blocker. | Include NIST/NCCoE, NCSC, CISA/NCCoE, and NSA guidance in the PQC map. | QSL PQC Standards Alignment / Migration Evidence Mapping Plan | NEXT_CANDIDATE | No | Migration guidance pairs naturally with F-0392-01 and should be handled first. | Governance-only evidence map and future candidate inventory. | Production rollout, dependency updates, crypto/key-schedule changes, public claims. |
| F-0392-03 | HQC backup KEM status | Tier 1 | `OFFICIAL_GUIDANCE` | BACKLOG_CANDIDATE | HIGH | qsl-protocol governance; qsc/qsp/protocol/crypto | absent | Do not treat HQC/FIPS 206 as a final QSL standard. | Future backup-KEM tracking matters, but no immediate active-code impact was shown. | Track HQC/FIPS 206 status inside the PQC evidence map and keep it watch-only. | QSL PQC Standards Alignment / Migration Evidence Mapping Plan | BACKLOG_CANDIDATE | No | HQC status belongs in the same PQC alignment map but should not outrank final FIPS standards. | Citation-backed status note and future watch marker. | Any HQC implementation, dependency change, or conformance claim. |
| F-0392-04 | IETF/CFRG PQ/hybrid drafts | Tier 4 | `INTERNET_DRAFT` | BACKLOG_CANDIDATE | MEDIUM | qsl-protocol governance; qsc/qsp/protocol/crypto | modeled | Drafts are not final standards and cannot anchor current public claims. | Draft movement can affect future protocol design, but no official final blocker exists. | Create a future RFC/draft boundary lane after PQC mapping. | QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan | BACKLOG_CANDIDATE | No | Needs tracking, but draft status makes it lower priority than final PQC standards. | Governance watch plan separating RFCs from drafts. | Treating drafts as final, changing wire/protocol semantics, public claims. |
| F-0392-05 | Rust/dependency advisory health | Tier 2 | `ADVISORY_FEED` | INFO | HIGH | dependency/advisory; qsl-protocol governance | proven | Do not overstate dependency safety beyond green audit evidence at a point in time. | No active CRITICAL/HIGH dependency blocker was found; ongoing trigger policy remains useful. | Keep cargo audit linkage and design an advisory trigger policy later. | QSL Dependency / Advisory Watch Trigger Policy Plan | WATCH_ONLY | No | Startup `cargo audit --deny warnings` and rustls-webpki v0.103.13 remain green. | Policy-only trigger mapping and advisory applicability rules. | Cargo.toml, Cargo.lock, dependency updates, workflow changes unless future scope authorizes. |
| F-0392-06 | Upstream crypto/security project watch | Tier 2 | `PROJECT_RELEASE_NOTES` | INFO | MEDIUM | dependency/advisory; external review; qsl-protocol governance | partial | Do not imply QSL inherits upstream assurance automatically. | Upstream notes require applicability review before action. | Fold into the future advisory trigger and code/crypto watch lanes. | QSL Dependency / Advisory Watch Trigger Policy Plan | WATCH_ONLY | No | Useful supporting source category, not an immediate blocker. | Project-note watch criteria and applicability rubric. | Dependency changes, implementation changes, public assurance claims. |
| F-0392-07 | Code/crypto research watch | Tier 3/4 | mixed | BACKLOG_CANDIDATE | MEDIUM | qsc/qsp/protocol/crypto; external review; public technical paper | partial | Do not claim external review or research-backed assurance from category discovery. | Research can identify audit topics, side-channel questions, and proof gaps. | Create a later code/crypto research triage and audit follow-up plan. | QSL Code / Crypto Research Watch and Audit Follow-Up Plan | BACKLOG_CANDIDATE | No | Important but broad; should follow foundational standards mapping. | Research triage filters, audit-candidate taxonomy, no implementation. | Changing crypto/security semantics from untriaged research, public claims. |
| F-0392-08 | Metadata privacy and secure messaging | Tier 1/3 mixed | mixed | CLAIM_BOUNDARY | HIGH | public docs/website; qshield demo/runtime; qsl-protocol governance; public technical paper | not claimed | Metadata-free, anonymity, and untraceable properties remain not claimed. | Sources reinforce that traffic shape, timing, sender metadata, and delivery architecture need separate evidence. | Keep public-claim boundaries and queue a future metadata privacy claim plan. | QSL Metadata Privacy / Secure Messaging Claim Boundary Plan | BACKLOG_CANDIDATE | No | High claim risk, but no current public-doc mutation or active blocker exists in NA-0393. | Claim-boundary evidence, future lane prerequisites, no runtime change. | Website/public claims, qshield runtime changes, anonymity or metadata-free assertions. |
| F-0392-09 | Backup/restore/key custody | Tier 2 and official docs | `PROJECT_RELEASE_NOTES` / official docs | CLAIM_BOUNDARY | HIGH | backup/restore; local-ops; public technical paper | blocked | Same-host continuity is not disaster recovery; off-host backup, restore, and key custody remain future-gated. | External guidance informs future backup/key lanes; operator input remains a blocker for off-host chain. | Preserve backup caveats and defer external guidance mapping until operator prerequisites unblock or a dedicated lane is selected. | QSL Backup / Restore / Key Custody External Guidance Mapping Plan | BACKLOG_CANDIDATE | No | No backup mutation occurred and no operator input changes the off-host blocked state. | Governance mapping, backup-impact review if durable outputs move. | Backup scripts/timers/fstab/services, target setup, key handling, real restore. |
| F-0392-10 | Disclosure, external review, and public claims | Tier 1/2 | `OFFICIAL_GUIDANCE` | BACKLOG_CANDIDATE | HIGH | external review; public docs/website; public technical paper | not claimed | Source discovery is not external review; public paper and website claims remain gated. | Future disclosure/readiness criteria are needed before public technical claims expand. | Queue a future external review/disclosure/public-claim readiness plan after standards and audit prerequisites improve. | QSL External Review / Disclosure / Public Claim Readiness Plan | BACKLOG_CANDIDATE | No | Important governance work, but not a blocker while public claims remain unchanged. | Governance readiness criteria and prerequisite map. | Public paper drafting, website updates, external-review-complete claims. |

## Critical / High Blocker Review

Blocker selection criteria were limited to official Tier 1 or Tier 2 evidence
showing an active vulnerability/advisory affecting QSL dependencies or code, an
official standards/guidance change making the current queue unsafe, a direct
public-claim risk requiring immediate correction, or current red cargo audit/CI
evidence.

Result:

`NO_CRITICAL_HIGH_BLOCKER_SELECTED`

Rationale:

- NA-0392 did not identify an official CRITICAL/HIGH immediate blocker for
  active qsl-protocol code or dependencies.
- Startup `cargo audit --deny warnings` passed.
- Startup `cargo tree -i rustls-webpki --locked` reported v0.103.13.
- Startup public-safety on origin/main completed success.
- NA-0393 changed no public docs, website, runtime, workflow, dependency, or
  backup implementation path.

## Queue Candidate Grouping

| Group | Candidate title | Findings covered | Severity / urgency | Evidence completeness | Queue status | Reason |
|---|---|---|---|---|---|---|
| A | QSL PQC Standards Alignment / Migration Evidence Mapping Plan | F-0392-01, F-0392-02, F-0392-03 | CLAIM_BOUNDARY / NEXT | High for FIPS and migration sources; HQC is watch-status | READY successor | Final NIST standards and official migration guidance are foundational and can be mapped without code changes. |
| B | QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan | F-0392-04 | BACKLOG | Medium; draft status requires caution | BACKLOG | RFC/draft boundary matters, but drafts should follow final PQC standards mapping. |
| C | QSL Dependency / Advisory Watch Trigger Policy Plan | F-0392-05, F-0392-06 | INFO / WATCH | High for local audit result; medium for applicability | WATCH_ONLY or BACKLOG | No active advisory blocker; future trigger policy remains useful. |
| D | QSL Code / Crypto Research Watch and Audit Follow-Up Plan | F-0392-07 | BACKLOG | Partial | BACKLOG | Research categories need filters before audit lanes are selected. |
| E | QSL Metadata Privacy / Secure Messaging Claim Boundary Plan | F-0392-08 | CLAIM_BOUNDARY | Partial | BACKLOG | Important claim-risk topic, but no current public claim expansion occurred. |
| F | QSL Backup / Restore / Key Custody External Guidance Mapping Plan | F-0392-09 | CLAIM_BOUNDARY | Blocked/partial | BACKLOG | Off-host backup chain still needs operator input; no backup mutation is authorized here. |
| G | QSL External Review / Disclosure / Public Claim Readiness Plan | F-0392-10 | BACKLOG | Partial | BACKLOG | Needed before public claims expand, but not an immediate blocker. |
| H | QSL Public Technical Position Paper Evidence Prerequisite Plan | F-0392-01 through F-0392-10 | CLAIM_BOUNDARY | Incomplete for paper start | WATCH_ONLY | Paper work remains future-gated until prerequisite evidence is stronger. |

## PQC Standards Alignment Successor Analysis

Candidate group A should be selected next because:

- FIPS 203, FIPS 204, and FIPS 205 are final Tier 1 standards.
- Official PQC migration guidance makes standards mapping a strategic evidence
  need even without code changes.
- QSL needs an explicit map from current protocol/evidence posture to final
  standards, migration guidance, HQC watch status, and exact non-claims.
- This work is governance/evidence-only and can preserve all runtime,
  dependency, workflow, public-doc, and backup boundaries.
- The resulting map is foundational for later code/crypto audit cadence,
  IETF/RFC boundary work, public-claim readiness, external review readiness,
  and public technical paper prerequisites.

## Advisory / Dependency / Security Watch Successor Analysis

Advisory/dependency work should not outrank PQC standards mapping now.

Evidence:

- `cargo audit --deny warnings` is green in startup/preflight evidence.
- `rustls-webpki` is v0.103.13.
- NA-0392 identified no active official CRITICAL/HIGH advisory blocker.
- A trigger-policy lane is useful, but it can remain backlog/watch-only until a
  red advisory, dependency applicability finding, or future live scope promotes
  it.

## Metadata Privacy / Public Claim Successor Analysis

Metadata/privacy and claim-boundary work should not outrank PQC standards
mapping now.

Evidence:

- Metadata-free, anonymity, and untraceable properties remain not claimed.
- qshield evidence remains demo/non-production bounded evidence.
- qsl-server and qsl-attachments production/public-internet proof remains
  future-gated.
- No website or public-doc claim was changed by NA-0393.
- The topic should remain a backlog candidate before public paper or website
  expansion.

## Backup / Restore / Key Custody Successor Analysis

Backup/restore/key custody external guidance should not outrank PQC standards
mapping now.

Evidence:

- Off-host target and host-identity operator input remains absent.
- Same-host continuity remains explicitly not disaster recovery.
- NA-0393 changes no backup scripts, timers, fstab, services, keys,
  passphrases, restore paths, remote targets, or monitoring configuration.
- Future durable external-watch report storage outside tracked governance
  evidence would require a separate backup-impact review.

## Public Technical Paper Readiness Analysis

Public technical paper work remains future-gated. NA-0393 does not draft,
outline, publish, or update public paper content.

Prerequisites before paper drafting should include:

- PQC standards alignment map.
- IETF/RFC versus draft boundary.
- Code/crypto audit and research-watch triage status.
- Metadata/privacy claim boundary.
- Backup/restore/key/off-host status.
- qsl-server/qsl-attachments production boundary.
- External review/disclosure readiness.
- Website/public-claim audit.

## Selected Successor

Selected:

`NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`

Rationale:

PQC standards and official migration guidance are the highest-confidence
foundational findings from NA-0392. A governance-only evidence map can advance
G1/G2/G3/G4/G5 claim discipline without changing crypto implementation or
runtime behavior.

Rejected alternatives:

- `NA-0394 -- QSL External Standards / Threat Watch Critical Finding Blocker Resolution`: rejected because no immediate official CRITICAL/HIGH active-code or dependency blocker was found.
- `NA-0394 -- QSL External Standards Watch Findings Evidence Gap Resolution`: rejected because NA-0392 evidence was sufficient to triage all ten findings.
- Advisory/dependency trigger policy next: rejected for immediate priority because cargo audit and public-safety are green.
- Metadata/privacy next: rejected for immediate priority because no current public claim expansion occurred.
- Backup/restore/key custody next: rejected for immediate priority because off-host prerequisites remain blocked pending operator input.
- Public technical paper next: rejected because prerequisites remain incomplete.

## Future Path / Scope Bundle

Future NA-0394 allowed paths, if the selected successor is restored:

- `docs/governance/evidence/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan.md`
- `tests/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0394 may use targeted read-only source verification only if live
scope authorizes it.

Future NA-0394 forbidden scope:

- runtime code;
- crypto implementation;
- qsc/qsp/qsl implementation;
- qshield runtime;
- qsl-server;
- qsl-attachments;
- dependencies;
- workflows;
- public docs/website;
- backup scripts/timers/fstab/services;
- response archives;
- secret handling;
- external claims.

## Public-Claim / External-Review / Website Boundary

NA-0393 records:

- Triage is not implementation.
- Findings are not proof of standards conformance.
- Standards mapping is not certification.
- Source watch is not external review.
- No website or public-doc update occurred.
- No public-internet readiness claim is made.
- No metadata-free, anonymity, or untraceable claim is made.
- No bug-free or perfect-crypto claim is made.
- No off-host backup or disaster-recovery completion claim is made.

## Future Validation / Marker Plan

Future NA-0394 markers:

- `NA0394_PQC_STANDARDS_ALIGNMENT_PLAN_OK`
- `NA0394_FIPS203_REFERENCE_OK`
- `NA0394_FIPS204_REFERENCE_OK`
- `NA0394_FIPS205_REFERENCE_OK`
- `NA0394_NIST_MIGRATION_GUIDANCE_OK`
- `NA0394_NCSC_MIGRATION_GUIDANCE_OK`
- `NA0394_CISA_NSA_GUIDANCE_OK`
- `NA0394_HQC_BACKUP_ALGORITHM_STATUS_OK`
- `NA0394_NO_COMPLIANCE_CLAIM_OK`
- `NA0394_NO_CERTIFICATION_CLAIM_OK`
- `NA0394_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0394_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0394_NO_METADATA_FREE_CLAIM_OK`
- `NA0394_NO_ANONYMITY_CLAIM_OK`
- `NA0394_NO_UNTRACEABLE_CLAIM_OK`
- `NA0394_NO_RUNTIME_CHANGE_OK`
- `NA0394_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0394_NO_DEPENDENCY_CHANGE_OK`
- `NA0394_NO_SECRET_MATERIAL_OK`

## Rejected Alternatives

- Implement PQC/code/crypto/runtime changes now.
- Change workflows now.
- Change dependencies now.
- Write durable external-watch report storage now.
- Start the public technical paper now.
- Treat source discovery as external review.
- Treating standards mapping as certification or standards conformance proof is rejected.
- Promote multiple READY items from findings.

## Backup-Plan Impact Statement

NA-0393 changes only tracked qsl-protocol governance, traceability, journal, and
testplan paths. No backup-plan update is required for this triage evidence
packet.

Future durable external-watch reports, durable audit report stores, response
archive/index changes, new history roots, or evidence paths outside tracked
qsl-protocol governance require separate backup-impact review. Same-host
continuity remains not disaster recovery.

## Next Recommendation

Merge the NA-0393 triage evidence if validation and CI remain green. If Packet S
closeout is eligible, restore exactly one READY successor:

`NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`
