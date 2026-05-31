Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0395 QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-215

## Objective

Validate that NA-0395 creates a qsl-protocol governance evidence map comparing
QSL's current evidence posture against authoritative RFCs and active
IETF/CFRG draft boundaries for TLS, HPKE, MLS, and post-quantum or hybrid key
establishment, while preserving all no-runtime, no-protocol, no-crypto,
no-dependency, no-workflow, no-public-claim, no-backup-mutation, and
sibling-repo boundaries.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0395 until optional closeout.
- NA-0394 is DONE.
- D-0770 and D-0771 exist once.
- D-0772 is absent at start and exists once after the evidence patch.
- D-0773 remains absent until optional closeout.
- No runtime, protocol, crypto, dependency, workflow, public docs, website,
  qsl-server, qsl-attachments, qshield runtime, qstart/qresume, backup,
  response archive, local tool, or secret-handling path is changed.
- RFC/draft mapping is not treated as implementation, compliance,
  certification, validation, production readiness, public-internet readiness,
  or external review.
- Internet-Drafts are not treated as final standards.

## Allowed Scope

- `docs/governance/evidence/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan.md`
- `tests/NA-0395_qsl_ietf_cfrg_protocol_draft_tracking_rfc_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, scripts/helpers, Cargo
files, runtime/protocol/crypto implementation paths, qsc/qsp/qsl
implementation, qshield runtime, qsl-server, qsl-attachments, qsc-desktop,
website, public docs, README, START_HERE, backup scripts/timers/fstab/services,
response archives, request/directive/journal history roots, qstart/qresume
tooling, branch protection, public-safety configuration, and secret-bearing
content.

## NA-0394 Inheritance Requirements

Verify the evidence map records:

- NA-0394 selected NA-0395 as the IETF/CFRG RFC-versus-draft successor.
- NA-0394 found no critical high blocker.
- ML-KEM and ML-DSA remain evidence-incomplete, not standards conformance or
  certification proof.
- SLH-DSA and HQC remain not implemented / not claimed.
- Migration posture remains governance-planned.
- The future Project Goal / Operating Principles canon lane is carried forward
  only as a future governance candidate.

## Official Source Verification Requirements

Verify official-source coverage for:

- RFC 8446 / TLS 1.3.
- RFC 9180 / HPKE.
- RFC 9420 / MLS.
- TLS hybrid/PQ key exchange drafts.
- HPKE PQ/hybrid drafts.
- CFRG hybrid KEM drafts.
- MLS PQ ciphersuite drafts.
- replaced or expired predecessor drafts needed to avoid stale citations.

## Source Citation Requirements

Each cited source must include:

- title.
- authority/publisher.
- URL.
- access date.
- source tier.
- stability classification.
- current status.
- QSL relevance.
- claim-boundary implication.

Official RFC Editor and IETF Datatracker sources must be used where available.
Vendor summaries must not replace official sources.

## QSL Protocol Evidence Inventory Requirements

The evidence map must inventory, read-only:

- qsc/qsp/protocol evidence.
- qshield demo/reference/oracle evidence.
- formal/model evidence.
- reference/oracle vectors.
- metadata runtime harnesses.
- governance evidence.
- qsl-server/qsl-attachments service boundaries.
- public-claim boundaries.

Implementation must not be inferred from names only.

## RFC 8446 Mapping Requirements

Verify the map records:

- RFC 8446 official status.
- TLS 1.3 relevance to handshake, transcript, key schedule, exporters, and
  negotiation vocabulary.
- QSL current QSP/Suite-2 evidence.
- QSL does not claim TLS 1.3 implementation or TLS compliance.
- missing TLS-specific evidence.
- future candidate action.

## RFC 9180 Mapping Requirements

Verify the map records:

- RFC 9180 official status.
- HPKE relevance to KEM/KDF/AEAD terminology.
- QSL KEM-adjacent evidence.
- QSL does not claim HPKE implementation or HPKE compliance.
- missing HPKE-specific evidence.
- future candidate action.

## RFC 9420 Mapping Requirements

Verify the map records:

- RFC 9420 official status.
- MLS relevance to secure group messaging.
- QSL secure-messaging-adjacent evidence.
- QSL does not claim MLS implementation or MLS compliance.
- missing MLS-specific evidence.
- future candidate action.

## TLS Hybrid / PQ Draft Requirements

Verify the map records:

- `draft-ietf-tls-hybrid-design` current status.
- `draft-ietf-tls-ecdhe-mlkem` current status.
- `draft-ietf-tls-mlkem` current status.
- replaced predecessor draft status.
- no draft-as-final claim.
- QSL draft-watch-only or governance-planned posture.

## HPKE PQ / Hybrid Draft Requirements

Verify the map records:

- `draft-ietf-hpke-pq` current status.
- replaced predecessor draft status.
- any metadata/status uncertainty.
- no HPKE PQ/hybrid implementation claim.
- no draft-as-final claim.

## CFRG Hybrid KEM Draft Requirements

Verify the map records:

- `draft-irtf-cfrg-hybrid-kems` current IRTF/CFRG status.
- research-group draft boundary.
- QSL hybrid-KDF relevance.
- no final-standard or implementation claim.

## MLS PQ Ciphersuite Draft Requirements

Verify the map records:

- `draft-ietf-mls-pq-ciphersuites` current status.
- replaced predecessor draft status.
- any metadata/status uncertainty.
- no MLS PQ ciphersuite implementation claim.
- no draft-as-final claim.

## No-Draft-As-Final Requirements

Verify:

- Internet-Drafts are classified as active, expired, replaced, WG draft, RG
  draft, or evidence-incomplete as applicable.
- RFC Editor queue status is not treated as RFC publication.
- Drafts can create watch or governance candidates only.

## Evidence Status Matrix Requirements

The consolidated matrix must include:

- TLS 1.3 / RFC 8446.
- HPKE / RFC 9180.
- MLS / RFC 9420.
- TLS hybrid/PQ draft.
- HPKE PQ/hybrid draft.
- CFRG hybrid KEM draft.
- MLS PQ ciphersuite draft.
- qsc handshake / suite-id evidence.
- formal/model handshake evidence.
- reference/oracle vectors.
- qshield demo evidence.
- qsl-server/qsl-attachments service boundary.
- public-claim readiness.
- external-review readiness.

Each row must include official source, source status, QSL current evidence,
evidence class, confidence, claim allowance, missing evidence, future lane, and
priority.

## Claim Boundary Requirements

Verify the map explicitly preserves:

- no TLS compliance claim.
- no HPKE compliance claim.
- no MLS compliance claim.
- no compliance or certification claim.
- no draft-as-final claim.
- no production-ready claim.
- no public-internet-ready claim.
- no external-review-complete claim.
- no metadata-free, anonymity, or untraceable claim.
- no bug-free or perfect-crypto claim.
- source discovery is not external review.
- RFC/draft mapping is not implementation.

## Public Paper Boundary Requirements

Verify the map states any public technical paper remains future-gated until
PQC mapping, IETF/RFC boundary, code/crypto audit status, metadata/privacy
status, service/backup/restore/key status, public-claim audit, and external
review readiness are current.

## Successor Selection Requirements

Verify exactly one successor is selected.

Expected normal successor:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

If source verification or critical protocol-boundary evidence blocks safe
continuation, the map must instead select one directive-authorized blocker
successor and explain why.

## Future Project Goal Canon Carry-Forward Requirements

Verify the map records `QSL Project Goal and Operating Principles Canon
Authorization Plan` as a future governance candidate only and does not promote
it over the selected NA-0396 successor.

## Backup-Impact Requirements

Verify the backup-impact statement records:

- only tracked qsl-protocol governance/testplan/traceability/journal paths are
  changed.
- no backup script/timer/fstab/service/key/target/restore path changes occur.
- no backup-plan update is required for NA-0395.
- future durable RFC/draft reports or public technical paper evidence stores
  require separate backup-impact review.
- same-host continuity is not complete disaster recovery.

## Required Local Checks

Required validation includes:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0772 / D-0773 count check.
- representative local-ops helper fixtures.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks.
- metadata runtime no-secret harnesses if directly runnable.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli build/test if feasible.
- scope guard with exact allowed changed paths.
- link-check.
- leak-scan.
- overclaim scan.
- goal-lint / PR-body preflight.

## CI Expectations

- public-safety remains required.
- Required PR checks must pass before merge.
- Post-merge public-safety must pass.
- No admin bypass, direct push, squash, rebase, force-push, amend, or
  branch-deletion command is allowed.

## Successor Handoff

If NA-0395 merges and closeout runs, restore exactly one READY item:

`NA-0396 -- QSL Dependency / Advisory Watch Trigger Policy Plan`

NA-0396 must not be implemented by NA-0395 closeout.
