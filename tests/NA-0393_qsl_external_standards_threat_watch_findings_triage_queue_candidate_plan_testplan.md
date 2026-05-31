Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0393 QSL External Standards / Threat Watch Findings Triage Queue Candidate Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0393 consumes the NA-0392 source-cited external standards /
threat / technology watch findings, records an explicit triage matrix, groups
future queue candidates, selects one exact NA-0394 successor, and preserves all
runtime, dependency, workflow, public-claim, sibling-repo, backup, and secret
boundaries.

## Protected Invariants

- Exactly one READY item remains present before closeout.
- NA-0393 remains READY until optional closeout.
- NA-0392 remains DONE.
- D-0766 and D-0767 remain present exactly once.
- D-0768 is absent at start and present exactly once after the NA-0393 evidence patch.
- D-0769 remains absent before closeout.
- No finding auto-promotes READY.
- No public/readiness/privacy claim is strengthened.
- Triage is not implementation.
- Source watch is not external review.

## Allowed Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0393_qsl_external_standards_threat_watch_findings_triage_queue_candidate_plan.md`
- `tests/NA-0393_qsl_external_standards_threat_watch_findings_triage_queue_candidate_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

NA-0393 must not mutate:

- `.github/**`
- workflows
- Cargo files or dependencies
- runtime, protocol, crypto, qsc, qsp, qsl, qshield runtime, qsc-desktop
- qsl-server
- qsl-attachments
- website or public docs
- README or START_HERE
- backup scripts, timers, fstab, services, restore paths, remote targets, keys, or passphrases
- qstart/qresume tools
- response archives except the final D212 response file
- durable external-watch report files outside authorized governance evidence

## NA-0392 Inheritance Requirements

Verify the evidence records:

- NA-0392 source-cited findings F-0392-01 through F-0392-10.
- NA-0392 source tiers and stability classifications.
- NA-0392 public-claim cautions.
- NA-0392 no-critical/high-blocker result.
- NA-0392 no durable external-watch report outside governance evidence.
- NA-0392 selected NA-0393 as the triage successor.

## Finding Extraction Requirements

Verify each finding records:

- finding ID;
- title;
- source domain;
- source tier;
- stability classification;
- stated QSL implications;
- public-claim implication;
- blocker status;
- candidate lane or recommended action.

If a finding is ambiguous, it must be classified as evidence-incomplete rather
than used to justify implementation.

## Triage Taxonomy Requirements

Verify each finding is assigned:

- severity;
- confidence;
- affected project area;
- current QSL evidence status;
- public-claim implication;
- security implication;
- recommended next action;
- proposed NEXT_ACTIONS candidate title;
- candidate priority;
- blocker yes/no;
- rationale;
- required future scope;
- forbidden future scope.

## Blocker Review Requirements

Immediate blocker successor is allowed only if official Tier 1 or Tier 2
evidence shows an active high-impact issue affecting QSL code/dependencies,
official guidance makes the current queue unsafe, a current public claim needs
immediate correction, or cargo audit/CI is red.

Expected result:

`NO_CRITICAL_HIGH_BLOCKER_SELECTED`

## Queue Candidate Grouping Requirements

Verify candidate grouping covers:

- QSL PQC Standards Alignment / Migration Evidence Mapping Plan.
- QSL IETF / CFRG Protocol Draft Tracking and RFC Boundary Plan.
- QSL Dependency / Advisory Watch Trigger Policy Plan.
- QSL Code / Crypto Research Watch and Audit Follow-Up Plan.
- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan.
- QSL Backup / Restore / Key Custody External Guidance Mapping Plan.
- QSL External Review / Disclosure / Public Claim Readiness Plan.
- QSL Public Technical Position Paper Evidence Prerequisite Plan.

Each group must state findings covered, urgency, evidence completeness, likely
scope, likely forbidden scope, claim-boundary impact, and READY/BACKLOG/watch
classification.

## PQC Successor Analysis Requirements

Verify the evidence considers:

- NIST FIPS 203, FIPS 204, and FIPS 205 as final standards.
- NIST/NCCoE, NCSC, CISA/NCCoE, and NSA migration guidance.
- HQC/FIPS 206 status as watch-only unless finalized.
- No standards conformance, validation, migration completion, or certification claim.
- No crypto implementation change.
- Governance/evidence-only scope.

## Advisory / Dependency Analysis Requirements

Verify:

- `cargo audit --deny warnings` remains green.
- `rustls-webpki` remains v0.103.13 or newer safe version.
- No active official CRITICAL/HIGH advisory blocker is selected.
- Advisory trigger policy remains a future candidate.

## Metadata / Privacy Claim Analysis Requirements

Verify:

- Metadata-free, anonymity, and untraceable properties remain not claimed.
- qshield evidence remains demo/non-production bounded evidence.
- qsl-server/qsl-attachments production/public-internet proof remains future-gated.
- No website or public-doc claim is changed.

## Backup / Restore / Key Analysis Requirements

Verify:

- Same-host continuity is not treated as disaster recovery.
- Off-host backup remains blocked pending deliberate no-secret operator input.
- No backup script/timer/fstab/service/key/restore/remote-target mutation occurs.
- Future durable report storage requires separate backup-impact review.

## Public Technical Paper Boundary Requirements

Verify:

- No paper is drafted.
- Paper work is future-gated.
- Prerequisites include PQC map, IETF/RFC boundary, code/crypto audit status,
  metadata/privacy claim boundary, backup/restore/key/off-host status,
  qsl-server/qsl-attachments production boundary, external review readiness, and
  website/public-claim audit.

## Successor Selection Requirements

Expected normal successor:

`NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`

Alternate critical successor:

`NA-0394 -- QSL External Standards / Threat Watch Critical Finding Blocker Resolution`

Alternate evidence-gap successor:

`NA-0394 -- QSL External Standards Watch Findings Evidence Gap Resolution`

NA-0394 must not be implemented by NA-0393.

## Public-Claim Boundary Requirements

High-risk phrases are allowed only when negated, prohibited, caveated,
future-bound, or exact bounded triage wording. Unsafe matches must be zero.

## Backup-Impact Requirements

Expected classification: no backup-plan update required if changed paths remain
limited to qsl-protocol governance/testplan/traceability/journal files.

Future durable external-watch report stores outside tracked governance evidence
require separate backup-impact review.

## Required Local Checks

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- helper help and representative fixture checks
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks
- metadata runtime no-secret harnesses if directly runnable
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1` if directly runnable
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli test/build if feasible
- queue and decisions proof
- exact scope guard
- link-check
- leak-scan
- classifier proof
- goal-lint / PR-body preflight

## CI Expectations

Required qsl-protocol CI must pass normally before merge. `public-safety` must
remain required and green before merge and after merge.

No admin bypass, direct push, squash, rebase, force-push, amend-after-PR, branch
deletion command, or delete-branch flag is authorized.

## Successor Handoff

If Packet R merges and Packet S is eligible, restore exactly one READY
successor:

`NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`

Closeout must not implement NA-0394.
