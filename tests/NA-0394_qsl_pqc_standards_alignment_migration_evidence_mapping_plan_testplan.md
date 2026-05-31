Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0394 QSL PQC Standards Alignment / Migration Evidence Mapping Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-213

## Objective

Validate that NA-0394 creates a qsl-protocol governance evidence map comparing
QSL's current evidence posture against official PQC standards and migration
guidance while preserving all no-runtime, no-crypto, no-dependency, no-workflow,
no-public-claim, no-backup-mutation, and sibling-repo boundaries.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0394 until optional closeout.
- NA-0393 is DONE.
- D-0768 and D-0769 exist once.
- D-0770 is absent at start and exists once after the evidence patch.
- No runtime, protocol, crypto, dependency, workflow, public docs, website,
  qsl-server, qsl-attachments, qshield runtime, backup, response archive, local
  tool, or secret-handling path is changed.
- Standards mapping is not treated as implementation, compliance,
  certification, validation, production readiness, public-internet readiness, or
  external review.

## Allowed Scope

- `docs/governance/evidence/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan.md`
- `tests/NA-0394_qsl_pqc_standards_alignment_migration_evidence_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, scripts/helpers, Cargo files,
runtime/protocol/crypto implementation paths, qsc/qsp/qsl implementation,
qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs,
README, START_HERE, backup scripts/timers/fstab/services, response archives,
request/directive/journal history roots, qstart/qresume tooling, branch
protection, public-safety configuration, and secret-bearing content.

## NA-0393 Inheritance Requirements

Verify the evidence map records:

- NA-0393 selected NA-0394 as the PQC standards alignment successor.
- NA-0393 found `NO_CRITICAL_HIGH_BLOCKER_SELECTED`.
- NA-0392 findings F-0392-01 through F-0392-10 feed future candidate analysis.
- IETF/CFRG RFC versus draft boundary remains the expected next group if no
  blocker is found.

## Official Source Verification Requirements

Verify official-source coverage for:

- NIST FIPS 203 / ML-KEM.
- NIST FIPS 204 / ML-DSA.
- NIST FIPS 205 / SLH-DSA.
- NIST / NCCoE migration guidance.
- NIST KEM transition guidance.
- NCSC migration guidance.
- CISA quantum-readiness guidance.
- NSA CNSA 2.0 or equivalent official guidance.
- NIST HQC backup-KEM status.

## Source Citation Requirements

Each cited source must include:

- title.
- authority/publisher.
- URL.
- access date.
- source tier.
- stability classification.
- QSL relevance.
- claim-boundary implication.

Vendor summaries must not replace available official sources.

## QSL Evidence Inventory Requirements

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

## FIPS 203 Mapping Requirements

Verify the map records:

- FIPS 203 official final status.
- QSL ML-KEM relevance.
- current direct code/harness/formal/reference evidence.
- missing conformance/audit/review evidence.
- no compliance/certification/validation/production-readiness claim.
- no code change required by NA-0394.

## FIPS 204 Mapping Requirements

Verify the map records:

- FIPS 204 official final status.
- QSL ML-DSA relevance.
- current direct code/harness evidence.
- missing conformance/audit/review evidence.
- no compliance/certification/validation/production-readiness claim.
- no code change required by NA-0394.

## FIPS 205 Mapping Requirements

Verify the map records:

- FIPS 205 official final status.
- SLH-DSA relevance.
- absence of direct current QSL implementation evidence.
- `NOT_IMPLEMENTED` / `NOT_CLAIMED` posture.
- future signature-agility option only.

## HQC Status Requirements

Verify the map records:

- official NIST HQC backup-KEM status.
- no verified final HQC/FIPS 206 standard in NA-0394 evidence.
- no current QSL HQC implementation claim.
- future watch/migration implication only.

## Migration Guidance Requirements

Verify the map records:

- NIST/NCCoE inventory, discovery, prioritization, and migration implications.
- NIST KEM usage/transition implications.
- NCSC migration timeline implications.
- CISA/NIST/NCCoE quantum-readiness implications.
- NSA CNSA 2.0 non-claim boundary.
- missing QSL inventory and migration posture evidence.

## Evidence Status Matrix Requirements

The consolidated matrix must include:

- ML-KEM / key establishment.
- ML-DSA / signatures.
- SLH-DSA / stateless hash signatures.
- HQC / backup KEM.
- hybrid / suite negotiation.
- formal/model alignment.
- reference/oracle vectors.
- qshield demo evidence.
- qsl-server/qsl-attachments service boundary.
- migration/inventory evidence.
- public claim readiness.
- external review readiness.

Each row must include source, current evidence, evidence class, confidence,
claim allowance, missing evidence, future lane, and priority.

## Claim Boundary Requirements

Verify the map explicitly preserves:

- no compliance claim.
- no certification claim.
- no FIPS validation claim.
- no standards conformance claim from mapping alone.
- no production-ready claim.
- no public-internet-ready claim.
- no external-review-complete claim.
- no metadata-free, anonymity, or untraceable claim.
- no bug-free or perfect-crypto claim.

## Public Paper Boundary Requirements

Verify the map states any public technical paper remains future-gated until PQC
mapping, IETF/RFC boundary, code/crypto audit status, service/backup/restore/key
status, public-claim audit, and external review readiness are current.

## Successor Selection Requirements

Verify exactly one successor is selected:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

If source verification or critical evidence mapping blocks safe continuation,
the map must instead select one of the directive-authorized blocker successors
and explain why.

## Backup-Impact Requirements

Verify the backup-impact statement records:

- only tracked qsl-protocol governance/testplan/traceability/journal paths are
  changed.
- no backup script/timer/fstab/service/key/target/restore path changes occur.
- no backup-plan update is required for NA-0394.
- future durable PQC reports or durable audit stores require separate
  backup-impact review.
- same-host continuity is not complete disaster recovery.

## Required Local Checks

Required validation includes:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- helper `--help` checks for local-ops helpers.
- representative helper fixture checks.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks.
- metadata no-secret checks if directly runnable.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli build/test if feasible.
- qsc NA-0313 harness if directly runnable.
- queue/decisions checks.
- scope guard.
- link-check.
- leak-scan.
- classifier proof for changed path set.
- goal-lint / PR-body preflight.

## CI Expectations

- public-safety remains a required check.
- public-safety must be green before merge and after merge.
- No admin bypass, direct push, squash, rebase, force-push, amend, or
  branch-deletion command is allowed.

## Successor Handoff

If NA-0394 merges and public-safety is green, optional closeout may restore:

`NA-0395 -- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan`

The closeout must not implement NA-0395.
