# NA-0650 as-built — DOC-PROG-004 GUI Phase Roadmap landing (D586, D-1273)

Lane: NA-0650 (docs-only LITE-CEREMONY, single PR + single decision per DOC-OPS-006 §9,
certified in D586). Directive: QSL-DIR-2026-07-16-586 (D586), APPROVED 2026-07-16 AS
AMENDED AT APPROVAL (A1 filename house sibling pattern; A2 this lane seats as NA-0650),
sha256 84d06a39e86609681a9b0e9659b47d25ec87becbcb5f143684069e0699018b01. Base: main
`6ae8e8ff` (the #1581 seating merge; qwork proof startup_result=OK, head==origin/main,
ready_count=1, queue_top_ready=NA-0650). Decision: D-1273.

## 1. Phase 0 (verified live)

- qwork proof files present and clean (kv + json + cargo-target env; shared_target_ready=yes).
- HEAD == origin/main == main == `6ae8e8ff`; worktree/index/untracked clean.
- STATE `READY=NA-0650 | HIGHEST_NA=0650 | HIGHEST_D=1272`; exactly one anchored
  `^Status: READY` (the NA-0650 block). D-1272 canonical ×1; D-1273 canonical ×0
  (next-and-absent).
- Main-health on `6ae8e8ff`: 7 of 9 push runs completed/success at Phase 0
  (macos-build, suite2-ci, demo-packaging, qshield-ci, public-ci, Code Quality,
  Push on main); formal-ci + qsc-adversarial in progress (ProVerif ~31–38 min normal),
  monitored during the lane as CI wait-work and re-verified green before the PR
  (result recorded in §5).
- DOC-PROG-004 referenced NOWHERE outside the queue/promotion text (unclaimed).
- Disk 49% (<95% gate); /backup/qsl mounted.
- Citation liveness (D586 Phase 1): DOC-QSC-008/009/010 all exist in docs/design/;
  NA-0649 block Status: DONE at D-1272; ENG-0044 "OWED to the GUI phase" present in
  the ledger; DOC-CTRL-001 §4.5 (line 185) and DOC-PROG-003 §6 (line 191) anchors
  present. No contradiction; no STOP fired.

## 2. The landing (adjustment classes A/B/C, nothing else)

`docs/program/DOC-PROG-004_QSC_GUI_Phase_Roadmap_v0.1.0_DRAFT.md` (NEW, 157 lines) =
the D586 appendix body VERBATIM under exactly:
- **A (house wrapper):** the DOC-PROG-003-form header block (Goals G4, G5; Status
  Supporting; Owner QSL governance; Last-Updated 2026-07-16 initial landing; the
  Authority paragraph); the provenance blockquote (names D586-as-amended, NA-0650,
  D-1273; carries the authorizes-NO-implementation boundary; gives "landed by this
  lane" its antecedent); the subordination footer.
- **B (title):** `# QSC GUI Phase — Roadmap (v1)` → `# DOC-PROG-004 — QSC GUI Phase
  Roadmap (v1) v0.1.0 DRAFT` (the A1 sibling title form; "(v1)" carried).
- **C (landing base):** "(landing base: current main at draft time)" → "(landing
  base: `6ae8e8ff`, recorded live at landing)"; the historical `fb1ef2bc`
  verification base stays as written.

## 3. THE FIDELITY PROOF (every hunk mapped; zero content deltas outside A/B/C)

`diff -u` of the extracted directive appendix (proof root `appendix_source.md`,
126 lines, extracted from the directive by exact title/banner bounds) against the
landed file = **exactly 2 hunks**:

- **Hunk 1 (@@ -1,8 +1,32 @@):** deletion of the appendix title line + addition of
  the class-A header block and provenance blockquote (A), the class-B title line (B),
  and the one-line "Verified against" change (C). Every added/removed line in this
  hunk is enumerated to a class; the Date/Status/Supersedes lines and all following
  body lines are context (unchanged).
- **Hunk 2 (@@ -124,3 +148,10 @@):** addition of the subordination footer (A) after
  the final corrections line (context, unchanged).

The 126-line body between the wrapper and the footer is byte-untouched (diff shows no
other hunk). Raw diff (also at proof root fidelity_diff.txt):

```diff
--- /srv/qbuild/tmp/NA0650_gui_phase_roadmap_doc_20260717T024703Z/appendix_source.md	2026-07-16 21:47:34.944436269 -0500
+++ docs/program/DOC-PROG-004_QSC_GUI_Phase_Roadmap_v0.1.0_DRAFT.md	2026-07-16 21:47:51.746334534 -0500
@@ -1,8 +1,32 @@
-# QSC GUI Phase — Roadmap (v1)
+Goals: G4, G5
+
+Status: Supporting (product planning; subordinate to canonical specs, GOALS, the queue, and independent review)
+Owner: QSL governance
+Last-Updated: 2026-07-16 (NA-0650/D586: initial landing)
+Authority: Non-normative product planning. Does NOT override START_HERE, GOALS,
+the canonical specs, NEXT_ACTIONS, the IMPROVEMENT_LEDGER, or any recorded
+decision. Where this document and any of those disagree, THEY win and this
+document is corrected.
+
+# DOC-PROG-004 — QSC GUI Phase Roadmap (v1) v0.1.0 DRAFT
+
+> Provenance: Records the operator-approved QSC GUI Phase Roadmap of 2026-07-16,
+> landed per QSL-DIR-2026-07-16-586 (D586, as amended at approval: A1 filename,
+> A2 lane number) by lane NA-0650 (D-1273). "Landed by this lane" in the Status
+> line below refers to NA-0650. The body below is the operator-approved text
+> verbatim under exactly three landing adjustments — the title form, this
+> wrapper, and the landing-base resolution — proven hunk-by-hunk in
+> docs/governance/evidence/NA-0650_as_built.md.
+>
+> **Authorization boundary: this document authorizes NO implementation.** It is
+> the PATH, not the build. Each step is its own future lane with its own
+> operator-approved directive. No lane may cite this document as execution
+> authority; NEXT_ACTIONS.md remains the only execution queue.
+
 
 **Date:** 2026-07-16. **Status:** operator-approved 2026-07-16; landed by this lane.
 **Supersedes:** all conversational roadmap sketches. This document is the path.
-**Verified against:** qsl-protocol main `fb1ef2bc` at time of writing (landing base: current main at draft time) (queue, ENG ledger, NA-0645/0646
+**Verified against:** qsl-protocol main `fb1ef2bc` at time of writing (landing base: `6ae8e8ff`, recorded live at landing) (queue, ENG ledger, NA-0645/0646
 closeouts, DOC-PROG-003), qsl-server main `8e4ea278`, the 2026-07-16 GUI-readiness
 investigation, and the locked design decisions below.
 
@@ -124,3 +148,10 @@
 5. Satellite bootstrap split into repo creation + spine governance lane (3a/3b).
 6. Platform-target decision (D-A) surfaced explicitly before the skeleton lane.
 7. R8 same-profile constraint made a named v1 documentation requirement.
+
+---
+
+*End DOC-PROG-004 v0.1.0 DRAFT. Subordinate at all times to the canonical
+specs, GOALS, the queue, the ledger, and independent review. This document
+records a plan; it authorizes no implementation and makes no comparative
+security claim.*
```

## 4. THE MANIFEST (positively verified on the landed file)

- Locked decisions: **8** (`| L1 |`..`| L8 |` table rows).
- Standing constraints: **4** bullets (ENG-0044 spine-lane placement WITH the recorded
  rationale "L2 makes satellite-side restoration impossible"; external review = THE
  release gate; R8 same-profile; lane discipline).
- Steps: **9** (`### Step 1`..`### Step 9`), Completed record present (NA-0649 at
  D-1272 / QSC_GUI_SURFACE_PASS / merge `6e4f7a93`).
- Decision gates: **D-A ×1** (platform target, before step 4), **D-B ×1** (contact-add
  design, before step 6).
- Parallel-tracks table: **6** data rows with triggers.
- Corrections: **7** numbered items.
- NA numbers: body carries only historical refs (NA-0215, NA-0645, NA-0648, NA-0649);
  NA-0650 appears ×4, ALL in the class-A wrapper (the landing lane's provenance, not
  future-lane numbering); **zero** NA references > 0650. No hard future-lane numbers.

## 5. Coupled wiring + validation (results recorded at closeout)

- DOC-CTRL-001: ONE §4.5 row (DOC-PROG-004, mirroring the DOC-PROG-003 row form) +
  Last Updated 2026-07-13 → 2026-07-16. Nothing else.
- DOC-PROG-003: ONE §6 spine-wiring bullet (DOC-PROG-004) + Last-Updated header bump.
  No §5 change, no prose deletion.
- IMPROVEMENT_LEDGER: UNTOUCHED (deliberate, D577 precedent, per D586).
- Validation and the final main-health/push-run results: see the lane response file
  and the PR checks (recorded there at closeout).

Classify; do not overclaim: this lane landed a PLAN. It authorizes no implementation;
no GUI exists; the external-review gate is restated, unchanged.
