Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0619 — Ratchet Liveness Feasibility + Design Test Plan (docs-only)

## Scope

Docs-only design lane (D-1234) under directive QSL-DIR-2026-07-08-556 (D556). Produces
`docs/design/DOC-G5-008` + governance. No source/test/Cargo change; no normative spec change.
NA-0619 marks DONE and restores NA-0620 (ENG-0012 Stage 1 DH-ratchet implementation) as the
sole READY successor.

## Required Markers

- NA0619_D1232_CONSUMED_OK
- NA0619_D1233_CONSUMED_OK
- NA0619_FRESH_STARTUP_PROOF_OK
- NA0619_D1234_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0619_DESIGN_STUDY_C1_C2_STATICRK_PARSE_CONFIRMED_OK
- NA0619_DOC_G5_008_TEN_SECTIONS_PRESENT_OK
- NA0619_NO_SOURCE_NO_NORMATIVE_SPEC_CHANGE_OK
- NA0619_ENG0012_DESIGN_COMPLETE_OK
- NA0619_CLAIM_BOUNDARY_NO_PCS_TRIPLE_RATCHET_CLAIM_OK
- NA0619_SUCCESSOR_NA0620_ENG0012_STAGE1_RESTORED_OK
- NA0619_ONE_READY_INVARIANT_OK
- NA0619_LIVE_QUEUE_HEADER_UPDATED_OK
- NA0619_D1235_ABSENT_OK
- NA0619_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0619) and main health; D-1232/D-1233 consumed
   once each and Accepted; D-1234 absent before implementation.
2. Design study confirms (read-only) C-1 (no X25519 / no DH state), C-2 (no sender boundary),
   the static-`rk` bootstrap, and that parse permits DH-only boundaries — all against live code.
3. DOC-G5-008 present with the ten required sections (problem+proof, feasibility, trigger
   policy, DH-only vs co-scheduled, sender construction + state additions, qsc wiring,
   conformance vectors, counter interaction, staged plan, open questions) + the G5 note.
4. Scope guard: changed set is docs/governance only — no `*.rs`, no `Cargo.*`, no normative
   spec (DOC-CAN-003/004 unchanged), no `.github`/`.claude`.
5. Governance: D-1234 recorded once; D-1235 absent; ENG-0012 advanced to design-complete with
   the DOC-G5-008 reference (remains open — implementation pending); NA-0619 DONE; NA-0620
   (ENG-0012 Stage 1) restored as the sole READY successor; `qsl_evidence_helper.py queue`
   reports `READY_COUNT 1 / NA-0620`; LIVE QUEUE STATE updated.
6. Claim boundary: the doc makes no post-compromise / Triple-Ratchet / quantum-secure claim and
   binds the project against such claims until Stages 1–2 land.
7. goal-lint Goals line; private-material scan on added lines.

## Result

`RATCHET_LIVENESS_DESIGN_COMPLETE`. ENG-0012 design-complete (implementation pending, NA-0620
Stage 1). No source/spec change. Sole READY successor NA-0620 begins at D-1235.
