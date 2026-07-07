Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0609 — Hostile-Analyst Metadata Minimization and Implementation-Attack Hardening Plan

## Summary

NA-0609 is a planning/analysis lane executed under directive
QSL-DIR-2026-07-06-546 (D546). It authors DOC-G5-005, a bounded hardening plan that
synthesizes the NA-0608 LAN hostile-analyst findings, the existing G5 metadata
corpus, and the open ledger items into a prioritized backlog of future
implementation lanes, and sets external/formal-review preconditions. It changes no
source and takes no runtime/LAN action; every concrete change it names returns as
its own lane.

Result classification: `HOSTILE_ANALYST_METADATA_IMPL_ATTACK_HARDENING_PLAN_ESTABLISHED`.

This is a plan, not an implementation and not an external/formal review. No
public-readiness, production-readiness, security-completion, crypto-complete,
attachment-complete, metadata-free, anonymity, or bug-free claim is made.

## Required Markers

- NA0609_D1215_CONSUMED_OK
- NA0609_D1216_CONSUMED_OK
- NA0609_FRESH_QWORK_PROOF_OK
- NA0609_CURRENT_MAIN_HEALTH_OK
- NA0609_D1217_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609_DOC_G5_005_AUTHORED_OK
- NA0609_SEVEN_AREAS_COVERED_OK
- NA0609_RANKED_BACKLOG_OK
- NA0609_LEDGER_FOLDED_OK
- NA0609_NO_SOURCE_MUTATION_OK
- NA0609_NO_LAN_RUNTIME_AUTHORIZED_OK
- NA0609_METADATA_MATRIX_INHERITED_OK
- NA0609_SUCCESSOR_NA0610_SELECTED_OK
- NA0609_ONE_READY_INVARIANT_OK
- NA0609_PRIVATE_MATERIAL_SCAN_OK
- NA0609_NO_METADATA_FREE_CLAIM_OK
- NA0609_RESULT_CLASSIFICATION_SELECTED_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0609 from `2026-07-07T01:38:19Z`
(regenerated via the WF-0004 drop-first workflow) verified before mutation; HEAD ==
origin/main == main == `2aba992cd84e`; worktree clean; READY_COUNT 1 with READY
NA-0609; D-1215 once, D-1216 once, D-1217 absent.

## Inheritance

D-1215 (NA-0609C closeout) and D-1216 (NA-0609D) consumed once each and Accepted.
The plan inherits the NA-0608 metadata matrix (size/count/timing exposed residual on
the attachment plane), the NA-0609B audit (handshake seam sound), the ENG-0003 fix
(constant-time MAC), and the ENG-0004 false-positive correction.

## Plan Deliverable

`docs/design/DOC-G5-005_Hostile_Analyst_Metadata_and_Implementation_Attack_Hardening_Plan_v0.1.0_DRAFT.md`
covers the seven objective areas — traffic-analysis metadata (message vs attachment
plane), implementation-attack surface, relay/qsl-attachments compromise models,
malformed-input test-expansion scope, attachment-plane padding/bucketing
feasibility, error/retry normalization, and external/formal-review readiness — and
produces a prioritized six-item backlog (§9), each with severity and a recommended
lane shape. It extends DOC-G5-001/002/004 and the NA-0137 roadmap (which cover the
message plane) to the attachment plane and the implementation-attack surface, and
respects the DOC-G5-001 non-goals (no anonymity/mixnet/"metadata eliminated" claim).

## Ledger Folding

The plan's new backlog items are recorded as ledger ENG-0005 (constant-time
comparison sweep), ENG-0006 (error/retry normalization review), and ENG-0007
(attachment-plane metadata mitigation feasibility). Existing ENG-0001 (self-label
footgun) and ENG-0002 (attachment single-send) are ranked in DOC-G5-005 §9 (ranks 5
and 6). The rank-1 item (malformed-input negative-test expansion) is the selected
NA-0610 successor.

## Successor

Selected successor: `NA-0610 -- Malformed Envelope/Descriptor/Object Adversarial
Negative-Test Expansion` (DOC-G5-005 §9 rank 1: bounded, test-only, closes the
NA-0608 "not separately exercised" coverage gap). Not implemented in this lane.

## Boundary And Claim

This lane mutated only docs/governance/ledger paths (the plan, the ledger, this
evidence doc, the testplan, and the governance spine per phase). It changed no
`.rs`, test, Cargo, workflow, canonical spec, `.claude`, or hook file; it authorized
no LAN/runtime/qscwork action and implemented no hardening item. No endpoint, port,
token, capability, key, seed, plaintext, ciphertext body, or raw private material is
published. No public-readiness, production-readiness, security-completion,
crypto-complete, attachment-complete, metadata-free, anonymity, untraceability,
side-channel-free, or bug-free claim is made; residual metadata is documented and
ranked, not eliminated.
