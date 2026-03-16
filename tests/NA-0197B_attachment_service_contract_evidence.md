# NA-0197B Attachment Service Contract Evidence

Goals: G4, G5

## 1. Gap summary from existing design state

Before `NA-0197B`, the repo already fixed:
- the separate attachment-plane architecture,
- qsl-server transport-only posture,
- the control-plane descriptor field meanings,
- the control-plane reject/coexistence matrix,
- and the later queue order from descriptor contract to service contract to runtime/client work.

What remained too vague for runtime implementation was:
- the canonical endpoint family,
- secret carriage outside the descriptor,
- create/upload/status/commit/abort/retrieval semantics,
- the service-side session/object state machine,
- the service-side reject taxonomy and no-mutation rules,
- and the operator/logging/metadata invariants for the future attachment runtime.

## 2. Canonical document ID proof

Before creating `DOC-CAN-006`, a repo-wide search for `DOC-CAN-006`, the chosen filename, `NA-0197B_attachment_service_contract_evidence`, and `D-0308` returned no matches.

## 3. Freeze summary

`DOC-CAN-006` freezes the missing service-plane pieces in implementation-grade form:
- canonical v1 endpoint family
- non-secret path element rules
- no-secret-in-canonical-URL rule for service APIs
- secret carriage via dedicated request headers plus response body issuance for newly minted secrets
- session creation/upload/status/commit/abort/retrieval semantics
- explicit session/object state machine
- deterministic service-side reject classes and no-mutation rules
- operator/logging/metadata/quota invariants
- source-of-truth handoff into the future `qsl-attachments` runtime lane

## 4. Source-of-truth mapping

| Concern | Source |
|---|---|
| architecture split and qsl-server posture | `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md` |
| control-plane descriptor fields and transcript semantics | `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md` |
| attachment service endpoint/state/reject/invariant rules | `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md` |
| rollout order | `tests/NA-0197_attachment_validation_and_rollout_plan.md` |

## 5. Closeout condition for NA-0197B

`NA-0197B` can close on path `N1` only if:
- `DOC-CAN-006` is precise enough that the future attachment runtime can implement the service plane without semantic guesswork,
- and the chosen attachment-surface repo exists with a sole READY runtime/governance lane.
