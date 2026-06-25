Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# External Review Package

## Executive Summary

QSL Suite-2 is a research-stage, non-production, Triple-Ratchet-style hybrid messaging design. The current repository contains meaningful evidence for always-hybrid per-message key derivation, SCKA epoch behavior, fail-closed downgrade checks, KT verifier behavior, no-state-mutation reject coverage, metadata conformance negatives, a bounded one-command demo with KT-negative and attachment demo proof markers, a guided desktop GUI demo surface, service production-boundary planning, website claim-boundary audits, and executable formal/model checks.

This package is for external review and release-readiness assessment. It is not production release approval. It does not claim anonymity, metadata elimination, "quantum-proof" security, or a proven true Triple Ratchet. It records what a reviewer can reproduce today, what the evidence supports, and which release gates remain open.

NA-0288 adds a metadata phase-2 and external review readiness gap plan. NA-0289
refreshes this package against that gap plan and the NA-0287 service
production-gate map. That refresh is reviewer-orientation work only; it does
not mean external review is complete.

NA-0290 adds a docs/governance design for the first metadata phase-2 identifier
rotation / opaque handle and padding-default executable harness. It is not an
implementation and it does not complete metadata phase-2.

NA-0291 adds an executable harness and vector file for deterministic
identifier / opaque-handle policy and padding-default policy fixtures. It is
design-only proof for those policies: runtime rotation, runtime default
padding, and metadata phase-2 completion remain open.

NA-0292 adds docs/governance design for metadata phase-2 sanitized-error
expansion and retention/purge metadata policy. It is planning evidence only:
sanitized-error expansion, retention/purge runtime policy, and metadata phase-2
completion remain open.

NA-0293 adds an executable harness and vector file for deterministic
sanitized-error and retention/purge metadata policy fixtures. It is bounded
policy-harness proof only: broader runtime sanitized-error normalization,
production retention or deletion behavior, and metadata phase-2 completion
remain open.

NA-0294 refreshes README, START_HERE, and public evidence navigation so
reviewers can reach evidence receipts and visible `NOT_READY` boundaries more
quickly. It is navigation and claim-boundary work only. It does not change
protocol, crypto, runtime, service, website, workflow, Cargo, dependency,
branch-protection, or public-safety configuration, and it does not complete
external review.

NA-0539 syncs selected repository public docs with current bounded qsc evidence:
direct remote qsc E2EE using synthetic data, same-host qsc tests, retained-qsc
freshness, SSH reverse-forward marker/ACK proof, Build-to-Inspiron qsc E2EE
success, selected replay/corrupt negatives, selected wrong-peer and
stale/replaced-peer negatives, repeated-run cleanup/freshness, public-safety and
advisories gates, quinn-proto RUSTSEC-2026-0185 remediation, bounded
formal/model checks, corpus validators, and secret-material scans. This is
review orientation, not completed external review and not a production/public
readiness claim.

## Current Posture

- Research-stage protocol and demo system.
- Not production-ready.
- Not anonymity or metadata-free.
- Not untraceable.
- Demo and desktop evidence are non-production and bounded to the documented local surfaces.
- This package is refreshed for reviewer orientation, not for reviewer
  acceptance, findings, disposition, or completion evidence.
- External live products under the same brand are not QSL protocol release-readiness proof.
- qsl-server and qsl-attachments remain deferred from NA-0539 public evidence
  sync and NA-0541 Progress publication; they must not be inferred as
  integrated production services.

## What Is Currently Proven

| Evidence area | Current proof | Review boundary |
| --- | --- | --- |
| Dependency/advisory health | `cargo audit --deny warnings` passed locally during the June 25, 2026 NA-0541 validation, and the root plus nested qsc fuzz lockfiles retained `quinn-proto 0.11.15`. | Advisory-clean proof only; time-sensitive and not a production security audit. |
| `public-safety` and `advisories` required gates green | Latest checked `origin/main` run for `9e7e389b6c42` completed `public-safety` and `advisories` successfully after PR #1354. | Required-check integrity, not a substitute for external review. |
| KT verifier fail-closed evidence | D-0440 and PR #708 record canonical KT verification for STH signatures, inclusion and consistency proofs, pinned logs, bundle signatures, and responder-side evidence binding. | Bounded to current refimpl/actor KT paths and documented disabled/non-production mode. |
| SCKA persistence and monotonicity | D-0445, PR #727, and [NA-0240 evidence](../governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md) cover restart persistence, rollback rejection, tombstones, one-time consumption, and reject no-mutation checks. | Evidence over current SCKA model/refimpl paths; not universal future-code proof. |
| Downgrade and no-mutation evidence | D-0447, PR #729, and [NA-0241 evidence](../governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md) cover transcript/capability rejects, no mutation, and demo negative acceptance. | Demo downgrade evidence remains bounded; unsupported demo surfaces are not faked. |
| KT consistency no-mutation | D-0449, PR #731, and [NA-0242 evidence](../governance/evidence/NA-0242_kt_consistency_no_mutation_audit.md) prove accepted KT state equality after selected rejected KT attempts. | Snapshot evidence is refimpl verifier evidence and is now carried into the demo runner by NA-0259. |
| Skipped-key and receive/decrypt no-mutation | D-0452, PR #734, and [NA-0243 evidence](../governance/evidence/NA-0243_skipped_key_decrypt_no_mutation_audit.md) cover skipped-key body-auth and receive body-auth rejects. | Bounded to current Suite-2 session snapshot tests. |
| Metadata conformance negatives | D-0454, PR #736, [NA-0244 evidence](../governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md), [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md), and [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md) define residual leakage and executable negative checks. | Supports metadata minimization wording only; no anonymity or metadata-elimination claim. |
| One-command demo acceptance | D-0458, PR #740, and [NA-0246 evidence](../governance/evidence/NA-0246_one_command_demo_acceptance_audit.md) prove loopback demo init, register, establish, send, receive/decrypt, and bounded rejects. | Non-production demo only; later rows record the bounded KT-negative and attachment additions. |
| KT-negative public demo readiness | D-0485, PR #768, and [KT-negative demo readiness](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md) prove canonical KT verifier rejects, accepted-state no-mutation, and explicit non-production disabled-shape boundary through the demo runner. | Demo-only verifier path; no production KT deployment or live qshield KT evidence input claim. |
| Attachment public demo readiness | D-0487, PR #770, and [attachment demo readiness](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md) prove encrypted descriptor/payload fetch/decrypt, descriptor-bound integrity validation, tampered-ciphertext reject, and opaque relay boundary through the qshield demo. | Demo-only qshield path; no qsl-server/qsl-attachments production service claim and no cross-host/private-network attachment proof. |
| Desktop GUI guided demo readiness | D-0460, PR #742, [NA-0247 evidence](../governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md), [NA-0258 evidence](../governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md), the [qsc desktop README](../../qsl/qsl-client/qsc-desktop/README.md), and [DOC-QSC-010](../design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md) validate the bounded sidecar shell surface plus one provisioned-host Linux AppImage/screenshot proof. | Guided prototype readiness only; no signed installer, macOS package, production release, or keychain active-ops claim. |
| Clean-host reviewer reproduction | D-0500, the [clean-host reviewer reproduction runbook](../demo/CLEAN_HOST_REVIEWER_REPRODUCTION.md), and [NA-0265 evidence](../governance/evidence/NA-0265_clean_host_reviewer_reproduction_audit.md) prove the public demo evidence can be rebuilt and rerun from a fresh source workdir at exact commit `1e7d0a63be31`. | Clean local source proof with Cargo registry/git cache reuse recorded; remote thin-client proof was preflighted but not counted as completed; no production readiness claim. |
| Service production-boundary evidence | D-0544, PR #827, [NA-0287 evidence](../governance/evidence/NA-0287_service_production_gate_evidence_map.md), and [service boundary plan](QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md) map qsl-server/qsl-attachments local hardening evidence and production gates. | Local service-hardening evidence only; no production relay, production attachment service, public internet readiness, or external review completion claim. |
| Metadata phase-2 gap plan | D-0546 and [NA-0288 gap plan](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md) classify metadata and review readiness as executable proof, docs-only planning, not-ready gaps, future gates, and out-of-scope claims. | Planning evidence only; no anonymity, metadata-free, untraceable, production-readiness, or external-review-complete claim. |
| Metadata phase-2 identifier/padding design | D-0553 and [NA-0290 design](../governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md) scope a future executable harness for rotating opaque delivery handles and padding defaults. | Design evidence only; identifier rotation and padding defaults are not implemented and metadata phase-2 remains incomplete. |
| Metadata phase-2 identifier/padding harness | D-0555, [NA-0291 evidence](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md), [NA-0291 testplan](../../tests/NA-0291_metadata_phase2_identifier_padding_harness_testplan.md), [NA-0291 vectors](../../inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json), and `scripts/ci/metadata_phase2_identifier_padding_harness.sh` prove deterministic policy fixtures. | Harness proof only; runtime identifier rotation, runtime default padding, anonymity, metadata-free messaging, untraceability, production readiness, and external review completion remain not ready. |
| Metadata phase-2 sanitized-error/retention design | D-0557, [NA-0292 design](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md), and [NA-0292 testplan](../../tests/NA-0292_metadata_phase2_sanitized_errors_retention_testplan.md) scope a future executable harness for broader sanitized-error and retention/purge metadata policy. | Design evidence only; sanitized-error expansion, retention/purge policy implementation, runtime behavior, anonymity, metadata-free messaging, untraceability, production readiness, and external review completion remain not ready. |
| Metadata phase-2 sanitized-error/retention harness | D-0559, [NA-0293 evidence](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md), [NA-0293 testplan](../../tests/NA-0293_metadata_phase2_sanitized_errors_retention_harness_testplan.md), [NA-0293 vectors](../../inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json), and `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh` prove deterministic policy fixtures. | Harness proof only; broader runtime sanitized-error normalization, production retention/deletion behavior, anonymity, metadata-free messaging, untraceability, production readiness, and external review completion remain not ready. |
| Public evidence navigation | D-0564, [README.md](../../README.md), [START_HERE.md](../../START_HERE.md), [public docs index](INDEX.md), [NA-0294 audit](../governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md), and [NA-0294 testplan](../../tests/NA-0294_public_evidence_navigation_refresh_testplan.md) make the reviewer path faster and clearer. | Navigation and claim-boundary proof only; no stronger release, production, service, website, metadata, anonymity, or external-review-complete claim. |
| Direct remote qsc E2EE and cleanup/freshness | D446, [NA-0537 evidence](../governance/evidence/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_harness.md), and [NA-0537 testplan](../../tests/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_testplan.md) record repeated controlled-lab qsc E2EE, retained-qsc freshness, selected wrong-peer/stale negatives, cleanup, and no stale state reuse. | Synthetic controlled-lab qsc proof only; no production readiness, public internet readiness, external review completion, crypto completeness, or service integration claim. |
| Remote qsc negative boundaries | D441, D419, [NA-0535 evidence](../governance/evidence/NA-0535_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_port_diagnostic_implementation_harness.md), and [NA-0523 evidence](../governance/evidence/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_harness.md) record selected wrong-peer, stale/replaced-peer, replay, and corrupt-delivery negative cases. | Selected negative cases only; no replay-proof, downgrade-proof, identity-complete, trust-complete, vulnerability-free, bug-free, or perfect-crypto claim. |
| Reverse-forward diagnostic marker/ACK | D439 and [NA-0534 evidence](../governance/evidence/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_harness.md) record marker traversal and ACK for the controlled diagnostic path. | Transport diagnostic proof only; no qsl-server/qsl-attachments integration or public internet service readiness claim. |
| Public repository evidence sync | D-1068 and [NA-0539 evidence](../governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md) update selected repository docs with bounded evidence, limits, and review invitation. | Repository documentation sync only; no website implementation, public/ path, website/ path, external review completion, production readiness, or public readiness claim. |
| Public Progress and site-wide accuracy sweep | D-1070, D-1071, D-1072, [Progress index](PROGRESS.md), and [June 25 Progress entry](progress/2026-06-25.md) create the repository Progress log and record the first site-wide public accuracy sweep. | Public engineering summary and correction ledger only; not a release certificate, website deployment, public readiness, production readiness, or external review completion claim. |
| External review package refresh | D-0548, this package, the [release readiness map](RELEASE_READINESS_EVIDENCE_MAP.md), [NA-0289 audit](../governance/evidence/NA-0289_external_review_package_refresh_audit.md), and [NA-0289 testplan](../../tests/NA-0289_external_review_package_refresh_testplan.md) align the reviewer-facing evidence references after NA-0287 and NA-0288. | Refresh evidence only; reviewer acceptance, findings, disposition, and external review completion remain `NOT_READY` until separately recorded. |
| Website truthfulness audit | D-0456 and [NA-0245 evidence](../governance/evidence/NA-0245_website_truthfulness_audit.md) map public website claims to repo truth and separate external products from protocol evidence. | Audit/plan only; no website implementation change. |
| Triple-Ratchet-style claim boundary | D-0462, PR #744, and [Suite-2 claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) authorize research-stage Triple-Ratchet-style wording and prohibit unsupported production/proven/anonymity claims. | External terminology is definitional only; it does not certify QSL. |
| Formal downgrade/no-mutation evidence | D-0464, PR #746, [formal README](../../formal/README.md), and [NA-0249 evidence](../governance/evidence/NA-0249_formal_downgrade_no_mutation_audit.md) run bounded SCKA and Suite-2 negotiation models. | Bounded model evidence; not a full cryptographic or production proof. |

## What Is Not Proven

- Production readiness.
- Public readiness.
- Public internet service readiness.
- External cryptographic review completion.
- Reviewer scope acceptance, findings, dispositions, and residual-risk signoff.
- No crypto completeness, identity completeness, or trust completeness.
- No replay-proof or downgrade-proof status.
- No secret-material completeness, side-channel freedom, vulnerability freedom,
  bug freedom, or perfect crypto.
- "Proven true Triple Ratchet" status.
- Anonymity or metadata elimination.
- Untraceability or traffic-analysis resistance.
- Metadata phase-2 completion.
- Production retention or deletion guarantees from NA-0293 policy fixtures.
- Production KT deployment readiness or live qshield KT evidence ingestion.
- Production attachment readiness, production attachment authentication, retention, resume, quota, or multi-node durability.
- Cross-host/private-network attachment proof.
- Production desktop release readiness.
- Production relay or qsl-server operation.
- qsl-attachments production service operation.
- Complete conformance reproducibility across local Linux, CI Linux, and macOS for every release claim.

## Reproducible Commands

Run from the repository root.

| Command | Purpose | Local result |
| --- | --- | --- |
| `cargo audit --deny warnings` | Dependency/advisory health. | PASS during NA-0541 validation on 2026-06-25; advisory posture remains time-sensitive. |
| `cargo tree -i rustls-webpki --locked` | Confirms locked `rustls-webpki` dependency path. | PASS; `rustls-webpki v0.103.13` through `rustls v0.23.36`. |
| `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` | qsc send/commit regression proof. | PASS; 3 tests passed. |
| `python3 formal/run_model_checks.py` | Bounded SCKA and Suite-2 negotiation model checks. | PASS; 926 SCKA states and 428 negotiation no-mutation assertions. |
| `scripts/ci/demo_cli_smoke.sh` | One-command demo acceptance. | PASS; ended with `DEMO_ACCEPTANCE_OK` and `demo-cli-smoke: OK`. |
| Clean-source command set in [clean-host reviewer reproduction](../demo/CLEAN_HOST_REVIEWER_REPRODUCTION.md) | Fresh-clone reviewer reproduction. | PASS on 2026-05-11; clean source proof emitted `NA0265_REVIEWER_REPRODUCTION_OK`. |
| `scripts/ci/metadata_conformance_smoke.sh` | Metadata conformance negative smoke. | PASS; ended with `metadata-conformance-smoke: OK`. |
| `scripts/ci/metadata_phase2_identifier_padding_harness.sh` | Metadata phase-2 identifier/padding policy fixture harness. | PASS when the NA-0291 harness markers are emitted; design-only markers preserve runtime claim boundaries. |
| [NA-0292 sanitized-error/retention design](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md) | Metadata phase-2 sanitized-error and retention/purge design review. | DOCS_ONLY planning evidence; NA-0293 now carries the bounded executable policy-harness proof. |
| `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh` | Metadata phase-2 sanitized-error and retention/purge policy fixture harness. | PASS when the NA-0293 harness markers are emitted; harness-only markers preserve runtime and production-retention claim boundaries. |

## Evidence Artifact Index

- [GOALS.md](../../GOALS.md)
- [ROADMAP.md](../../ROADMAP.md)
- [TRACEABILITY.md](../../TRACEABILITY.md)
- [DECISIONS.md](../../DECISIONS.md)
- [Demo acceptance criteria](../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [Conformance vector prioritization](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [Engineering velocity policy](../governance/ENGINEERING_VELOCITY_POLICY.md)
- [Workday autopilot policy](../governance/WORKDAY_AUTOPILOT_POLICY.md)
- [Metadata threat model](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [Envelope transport profile](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [qsc desktop prototype README](../../qsl/qsl-client/qsc-desktop/README.md)
- [Desktop GUI active-ops boundary](../design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md)
- [Formal model README](../../formal/README.md)
- [NA-0240 SCKA evidence](../governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md)
- [NA-0241 downgrade/demo evidence](../governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md)
- [NA-0242 KT evidence](../governance/evidence/NA-0242_kt_consistency_no_mutation_audit.md)
- [NA-0243 skipped-key evidence](../governance/evidence/NA-0243_skipped_key_decrypt_no_mutation_audit.md)
- [NA-0244 metadata evidence](../governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md)
- [NA-0245 website truthfulness evidence](../governance/evidence/NA-0245_website_truthfulness_audit.md)
- [NA-0246 demo acceptance evidence](../governance/evidence/NA-0246_one_command_demo_acceptance_audit.md)
- [NA-0247 desktop GUI readiness evidence](../governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md)
- [NA-0248 claim-boundary evidence](../governance/evidence/NA-0248_suite2_triple_ratchet_evidence_audit.md)
- [NA-0249 formal evidence](../governance/evidence/NA-0249_formal_downgrade_no_mutation_audit.md)
- [NA-0250 audit](../governance/evidence/NA-0250_external_review_release_readiness_audit.md)
- [NA-0250 testplan](../../tests/NA-0250_external_review_release_readiness_testplan.md)
- [KT-negative public demo readiness](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md)
- [NA-0259 KT-negative demo evidence](../governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md)
- [Attachment public demo readiness](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md)
- [NA-0260 attachment demo evidence](../governance/evidence/NA-0260_attachment_demo_readiness_audit.md)
- [NA-0261 public evidence refresh audit](../governance/evidence/NA-0261_public_evidence_refresh_audit.md)
- [Clean-host reviewer reproduction](../demo/CLEAN_HOST_REVIEWER_REPRODUCTION.md)
- [NA-0265 clean-host reviewer reproduction evidence](../governance/evidence/NA-0265_clean_host_reviewer_reproduction_audit.md)
- [Server and attachments production-boundary plan](QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md)
- [NA-0287 service production-gate evidence map](../governance/evidence/NA-0287_service_production_gate_evidence_map.md)
- [NA-0288 metadata phase-2 and external review gap plan](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md)
- [NA-0290 metadata phase-2 identifier/padding design](../governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md)
- [NA-0291 metadata phase-2 identifier/padding harness](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md)
- [NA-0291 metadata phase-2 identifier/padding harness testplan](../../tests/NA-0291_metadata_phase2_identifier_padding_harness_testplan.md)
- [NA-0291 metadata phase-2 identifier/padding policy vectors](../../inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json)
- [NA-0292 metadata phase-2 sanitized-error/retention design](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md)
- [NA-0292 metadata phase-2 sanitized-error/retention testplan](../../tests/NA-0292_metadata_phase2_sanitized_errors_retention_testplan.md)
- [NA-0293 metadata phase-2 sanitized-error/retention harness](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md)
- [NA-0293 metadata phase-2 sanitized-error/retention harness testplan](../../tests/NA-0293_metadata_phase2_sanitized_errors_retention_harness_testplan.md)
- [NA-0293 metadata phase-2 sanitized-error/retention policy vectors](../../inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json)
- [NA-0294 public evidence navigation refresh audit](../governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md)
- [NA-0294 public evidence navigation refresh testplan](../../tests/NA-0294_public_evidence_navigation_refresh_testplan.md)
- [NA-0289 external review package refresh audit](../governance/evidence/NA-0289_external_review_package_refresh_audit.md)
- [NA-0289 external review package refresh testplan](../../tests/NA-0289_external_review_package_refresh_testplan.md)

## Recent PR Evidence Table

| PR | Evidence summary | Status |
| --- | --- | --- |
| #708 | Fail-closed KT verifier implementation and evidence. | Merged |
| #727 | SCKA persistence and monotonicity vector hardening. | Merged |
| #729 | Demo negative acceptance and downgrade/no-mutation hardening. | Merged |
| #731 | KT consistency reject no-mutation hardening. | Merged |
| #734 | Skipped-key and receive-decryption reject no-mutation hardening. | Merged |
| #736 | Metadata conformance negative expansion. | Merged |
| #740 | One-command public demo acceptance runner. | Merged |
| #742 | Desktop GUI prototype validation and public demo readiness. | Merged |
| #744 | Suite-2 Triple-Ratchet-style evidence and claim boundary. | Merged |
| #746 | Formal downgrade/no-mutation model expansion. | Merged |
| #768 | KT-negative public demo readiness proof. | Merged |
| #770 | Attachment public demo readiness proof. | Merged |
| #827 | Service production-gate evidence map and deployment-boundary plan. | Merged |
| #829 | Metadata phase-2 and external review readiness gap plan. | Merged |
| #830 | NA-0288 closeout and NA-0289 restoration. | Merged |
| #1348 | NA-0537 closeout and NA-0538 restoration after repeated-run remote qsc evidence. | Merged |
| #1349 | NA-0538 authorization for selected public evidence sync path bundle and claim policy. | Merged |
| #1350 | NA-0538 closeout and NA-0539 restoration. | Merged |
| #1351 | NA-0539 public evidence sync implementation. | Merged |
| #1352 | NA-0539 closeout and NA-0540 restoration. | Merged |
| #1353 | NA-0540 daily Progress cadence and site accuracy authorization. | Merged |
| #1354 | NA-0540 closeout and NA-0541 restoration. | Merged |

## Review Questions For External Reviewers

1. Does the evidence support the current research-stage Suite-2 / Triple-Ratchet-style wording without overstating production readiness?
2. Are the bounded model checks faithful enough to the canonical downgrade and SCKA invariants, given the documented abstractions?
3. Which KT verifier cases should be promoted from refimpl proof into public demo or conformance-vector proof next?
4. Which metadata residuals create the highest public-claim risk before release readiness?
5. Which commands should be made easier to reproduce across clean Linux and macOS hosts?
6. Which gaps block external security review from being treated as complete?
7. Which qsc evidence links should be promoted or simplified for independent
   reproduction without weakening the no-overclaim boundaries?

## Reviewer Checklist

Ready to review:

- research-stage Suite-2 / Triple-Ratchet-style claim boundary;
- canonical specs, Suite-2 vectors, and bounded formal/model checks;
- KT verifier, SCKA, downgrade, skipped-key, and receive reject evidence;
- non-production qshield demo, KT-negative demo, attachment demo, desktop
  prototype, and clean-host reviewer reproduction evidence;
- local qsl-server and qsl-attachments hardening evidence as production-gate
  inputs only;
- metadata phase-2 gap classifications and residual leakage disclosures.

Not ready to treat as complete:

- external review completion;
- reviewer scope acceptance, findings, disposition, or residual-risk signoff;
- production protocol, relay, attachment service, desktop, or public internet
  readiness;
- public readiness;
- no crypto completeness, identity completeness, or trust completeness;
- no replay-proof, downgrade-proof, secret-material-complete,
  side-channel-free, vulnerability-free, bug-free, or perfect-crypto status;
- metadata phase-2 completion;
- anonymity, metadata-free messaging, or untraceability claims.

Expected reviewer outputs if external review starts:

- accepted review scope and reviewed commit;
- findings with severity, affected evidence area, and reproduction notes;
- disposition for each finding, including fixes, deferrals, and residual risk;
- explicit statement of any claim wording that must remain narrowed.

## Known Gaps And Recommended Next Work

| Gap | Recommended next work |
| --- | --- |
| Website evidence-boundary implementation is not done. | Prepare a bounded handoff package before editing the external website repository. |
| Production KT deployment is not proven. | Keep KT-negative demo wording limited to canonical verifier/vector proof until live KT service and qshield evidence-ingestion lanes exist. |
| Production attachment readiness is not proven. | Keep attachment wording limited to encrypted qshield demo descriptor/payload proof and local qsl-attachments hardening evidence until deployment-profile proof exists. |
| Production desktop release readiness is not proven. | Keep native package/screenshot evidence bounded to the provisioned-host prototype proof. |
| Metadata phase-2 remains open. | Use [NA-0291](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) and [NA-0293](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md) as bounded policy-harness proof only. Next, improve public evidence navigation while keeping runtime, deployment, and anonymity gaps explicit. |
| Reviewer findings and dispositions are not recorded. | Use this refreshed package as orientation material, then record accepted scope, findings, dispositions, and residual risk in a separate evidence lane. |
| External cryptographic review is not complete. | Send this package plus canonical specs, vectors, and model limits to reviewers and record findings separately. |
| Public Progress is new. | Review the [Progress index](PROGRESS.md), [June 25 entry](progress/2026-06-25.md), and correction ledger for factual or claim-safety gaps before treating the summary as reviewer handoff material. |

## Safe Public Wording

Safe:

- "QSL Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design."
- "Current evidence is non-production and release-gated."
- "The repository contains executable evidence for selected Suite-2, KT, SCKA, downgrade, metadata, demo, attachment-demo, GUI, and formal/model-check properties."
- "Metadata minimization work is in progress; the current demo profile is not an anonymity system."
- "Service hardening evidence is local and production-gated; production service claims remain future work."
- "The public Progress log summarizes merged evidence, accepted decisions, corrections, and limits; it is not a release certificate."

Do not claim:

- production-ready QSL protocol
- proven true Triple Ratchet
- quantum-proof communications
- anonymous or metadata-free messaging
- untraceable messaging
- production KT readiness
- production attachment readiness
- production relay or qsl-server readiness
- external cryptographic review completion
