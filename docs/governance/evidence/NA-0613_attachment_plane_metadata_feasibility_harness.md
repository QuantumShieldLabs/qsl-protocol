Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0613 — Attachment-Plane Metadata Mitigation Feasibility and Design (read-only)

## Summary

NA-0613 is a read-only feasibility+design study (DOC-G5-005 §9 rank 4 / ledger
ENG-0007) executed under directive QSL-DIR-2026-07-07-550 (D550) as a LITE-CEREMONY
lane (single PR, single decision D-1223). It authors DOC-G5-006, which inventories the
attachment-plane residual metadata channels, proposes mitigations extending the
message-plane bucketing model, and ranks them with a cost/benefit matrix. No source,
wire, or attachment-contract change; every mitigation is deferred to its own lane.

Result classification: `ATTACHMENT_PLANE_METADATA_MITIGATION_DESIGN_ESTABLISHED`.

The key feasibility finding: object-size/part-count bucketing (M1) is **client-side
feasible against the service/network observer without an attachment-contract change**,
because the descriptor (carrying the true `plaintext_len`) is peer-only inside the
encrypted envelope while the service sees only the opaque padded ciphertext object.
This is not a metadata-free/anonymity/unlinkability claim; the honest residual is
documented.

## Required Markers

- NA0613_D1221_CONSUMED_OK
- NA0613_D1222_CONSUMED_OK
- NA0613_FRESH_QWORK_PROOF_OK
- NA0613_CURRENT_MAIN_HEALTH_OK
- NA0613_D1223_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0613_LITE_CEREMONY_CERTIFIED_OK
- NA0613_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0613_DOC_G5_006_AUTHORED_OK
- NA0613_CHANNEL_INVENTORY_C1_C4_OK
- NA0613_THREAT_OBSERVATION_SEPARATION_OK
- NA0613_M1_CLIENT_FEASIBLE_NO_CONTRACT_CHANGE_OK
- NA0613_COST_BENEFIT_MATRIX_OK
- NA0613_HONEST_RESIDUAL_RECORDED_OK
- NA0613_ENG0007_RESOLVED_OK
- NA0613_ENG0010_ENG0011_FILED_OK
- NA0613_SUCCESSOR_NA0614_SELECTED_OK
- NA0613_PRIVATE_MATERIAL_SCAN_OK
- NA0613_RESULT_CLASSIFICATION_SELECTED_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0613 from `2026-07-07T03:39:58Z`
(drop-NA-0612/qwork-NA-0613) verified before mutation; HEAD == origin/main == main ==
`694a8904dad3`; worktree clean; READY_COUNT 1 with READY NA-0613; D-1221 once, D-1222
once, D-1223 absent; DOC-G5-006 confirmed free.

## Inheritance

D-1221 (NA-0611) and D-1222 (NA-0612) consumed once each and Accepted. The study uses
the NA-0608 attachment-plane metadata matrix and the message-plane bucketing model.

## Method

Read-only inspection of the qsc attachment sizing/chunking code
(`attachments/mod.rs`), the descriptor size fields and their AAD/confirm binding, and
the message-plane `transport::meta_bucket_for_len` precedent. Each residual channel is
classified by observer (peer vs service/network); candidate mitigations are assessed
for benefit, cost, placement (qsc vs qsl-attachments), and contract compatibility.

## Findings (see DOC-G5-006 for the full design)

- Channels: C1 object ciphertext size (exact, service-visible), C2 part count (coarse),
  C3 part-size-class-by-plaintext (3-way band), C4 upload/fetch timing/pattern
  (service/deployment).
- Grounded facts: fixed ladder p64k/p256k/p1024k; `ciphertext_len = plaintext_len +
  part_count*tag` (no object padding today); size fields bound in per-part AAD and
  confirm MAC.
- M1 (object-size padding to a ladder) + M2 (part-count bucketing) + M3 (class
  decoupling): client-side feasible, no contract change — RECOMMENDED as the top
  implementation lane (NA-0614; ENG-0010).
- M4 (timing/cover): mostly qsl-attachments/deployment, cross-repo — DEFERRED
  (ENG-0011).
- Honest residual: bucketing discloses a size range; object count and access
  timing/pattern remain; correlation and cover-traffic out of scope. No metadata
  elimination.

## ENG-0007 Resolution And Successor

ENG-0007 resolved-into-findings; ENG-0010 (recommended M1/M2/M3 client bucketing) and
ENG-0011 (deferred timing/cover) filed. Because M1 is a feasible, high-value,
client-only, contract-compatible mitigation, the successor proceeds as planned to
NA-0614 (its implementation lane).

## Boundary And Claim

This lane mutated only docs/design/evidence/ledger/governance paths; it changed no
`.rs`, test, Cargo, workflow, spec, attachment-contract, `.claude`, or hook file, and
applied no fix. No runtime/LAN action occurred. No endpoint, port, token, capability,
key, seed, plaintext, ciphertext body, or raw private material is published. No
public-readiness, production-readiness, security-completion, crypto-complete,
metadata-free, anonymity, unlinkability, or traffic-analysis-resistant claim is made;
the design reduces and buckets residual metadata and documents what remains.
