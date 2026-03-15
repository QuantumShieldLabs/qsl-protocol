# NA-0197A Descriptor Contract Evidence

Status: DRAFT  
Owner: QSL governance  
Date: 2026-03-15

## 1. Pre-existing source inventory

Already explicit before `NA-0197A`:
- `DOC-ATT-001` chose the control-plane/data-plane split, qsl-server non-ownership, legacy-path temporary coexistence, and the high-level descriptor responsibilities.
- `NA-0197_attachment_validation_and_rollout_plan.md` defined the next-item ordering and validation categories.
- `NEXT_ACTIONS.md`, `DECISIONS.md`, and `TRACEABILITY.md` already promoted `NA-0197A` as the next blocker.

Still too vague before `NA-0197A`:
- exact descriptor field list and field domains
- exact distinction between peer-visible, local-only, and service-only fields
- exact transcript-bound compare set
- exact confirmation-handle semantics
- exact legacy coexistence and invalid mixed-mode rules
- exact reject taxonomy and no-mutation behavior
- exact ownership mapping for later service/client items

## 2. Canonical doc ID proof

Before creating `DOC-CAN-005`, a repo-wide search for `DOC-CAN-005` and the chosen filename returned no matches.

Search scope:
- `docs/**`
- `tests/**`
- `README.md`
- `DECISIONS.md`
- `TRACEABILITY.md`

## 3. Freeze summary

`DOC-CAN-005` freezes the missing contract pieces in implementation-grade form:
- exact payload identity: `t = "attachment_descriptor"`, `v = 1`
- exact transmitted field set, domains, and optionality
- exact part-size and retention registries
- exact transcript-bound compare set
- exact confirmation-handle derivation and delivery-state linkage
- exact legacy coexistence and mixed-mode invalidity rules
- exact reject classes and no-mutation requirements
- exact source-of-truth split for `NA-0197B`, repo-local service implementation, and `NA-0197C`

## 4. Source mapping

| Need | Source after `NA-0197A` |
|---|---|
| architecture decision | `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md` |
| descriptor schema and control-plane contract | `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md` |
| validation ladder and rollout order | `tests/NA-0197_attachment_validation_and_rollout_plan.md` |
| decision freeze record | `DECISIONS.md` |
| queue/implementation linkage | `TRACEABILITY.md` |

## 5. Closeout criterion

`NA-0197A` can close on path `M1` only if `DOC-CAN-005` is precise enough that:
- `NA-0197B` can define the service contract without semantic guesswork about descriptor fields or reject behavior
- `NA-0197C` can later implement client journaling/confirmation behavior without reopening control-plane meaning
- no smaller direct descriptor-gap item remains
