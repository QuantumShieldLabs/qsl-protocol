# NA-0641 As-Built — QSC Feature Plan Landed as DOC-PROG-003 (D577, D-1264)

Goals: G4, G5
Status: Supporting evidence (lane closeout)
Owner: QSL governance
Last-Updated: 2026-07-13

## 1. What this lane landed

A DOCS/governance lane, per QSL-DIR-2026-07-13-577 (D577, APPROVED, as amended
2026-07-13 A1). It authored the QSC Feature Plan as a program document and
wired it into the spine. It built NO feature, changed NO source, and moved NO
claim.

Artifacts:
- `docs/program/DOC-PROG-003_QSC_Feature_Plan_Tiered_Feature_Set_and_Build_Order_v0.1.0_DRAFT.md`
  (NEW) — the plan: §1 honest current-state inventory; §2 strategic sharpening;
  §3 the tiered set (3.1 Tier 1 build / 3.2 Tier 2 differentiator UIs / 3.3
  Tier 3 defer); §4 the self-host split; §5 build order; §6 spine
  cross-references; §7 maintenance rule. DRAFT status, DOC-PROG-002 house form
  (Goals/Status/Owner/Last-Updated/Authority header; provenance block;
  subordination footer). The provenance block states the authorization
  boundary: the document authorizes NO implementation; each feature's build is
  its own future lane with its own directive.
- `docs/program/DOC-PROG-002_QSL_Product_Strategy_Niche_Positioning_v0.1.0_DRAFT.md`
  — the TWO dangling-reference fixes ONLY (§2 below).
- `docs/master/DOC-CTRL-001_Documentation_Master_Index_Release_Packet_v1.0.1_DRAFT.md`
  — the index entry (§3 below).

## 2. The DOC-PROG-002 reference fixes (minimal, as directed)

Both dangling "qsc feature plan" citations now point at DOC-PROG-003. Each fix
is a parenthetical insertion; zero words were removed; no other content line
changed. The whole DOC-PROG-002 diff is three lines:

1. §4 (third bullet, "Client feature parity where it serves the niche"):
   `The qsc feature plan identifies` → `The qsc feature plan (DOC-PROG-003)
   identifies` — the D577 base-scope fix.
2. §8 ("Relationship to other documents"): `**The N3 lane-family plan and the
   qsc feature plan**` → `**The N3 lane-family plan and the qsc feature plan
   (DOC-PROG-003)**` — the A1 amendment fix (the second dangling reference,
   found at directive landing).
3. Header `Last-Updated: 2026-07-11` → `2026-07-13` (currency metadata for
   the edit; not content).

Deliberately NOT done (stayed minimal): §4's illustrative parity list and §8's
"those enter the repo via their own design lanes" phrasing were left verbatim
— DOC-PROG-003 §2 records why the target set is narrower than §4's list, and
§6 records that the plan half of the §8 sentence is now landed. Rewriting
DOC-PROG-002 to match would have exceeded the minimal-pointer-fix boundary
(a D577 STOP condition).

## 3. The DOC-CTRL-001 index entry (first program-doc entry — new convention)

Phase-0 fact: DOC-CTRL-001 indexed NO DOC-PROG document (DOC-PROG-002's own
landing note records that, by DOC-PROG-001 precedent, program docs were listed
in none of the four indexes, and that operator-chosen indexing "establishes a
new convention"). D577 directs the index entry, so the convention is now
established:

- New registry subsection **§4.5 "Program Documents (strategy / product
  planning — Supporting, non-normative)"** with the DOC-PROG-003 row (title,
  version, status, location) and a preamble stating program docs are
  Supporting, subordinate to the spine/canonical/queue/ledger, and define no
  execution queue.
- The prior §4.5 (External bundles) renumbered to §4.6 — repo-wide grep found
  ZERO external references to DOC-CTRL-001 §4.5, so the renumber breaks
  nothing.
- Header `Last Updated: 2025-12-28` → `2026-07-13`.
- DOC-PROG-001/-002 remain unindexed (predating the convention); the new
  subsection notes this explicitly. Indexing them is a candidate follow-up
  micro-lane — NOT done here (D577 scope: "index the new doc").

## 4. Content fidelity to D577 (the five parts + the boundary)

- **Current state (§1)**: from the 2026-07-13 read-only investigation
  (verified against main `7d7c7550`): working 1:1 PQ-hybrid E2EE messenger;
  NONE of the seven parity features; no differentiator UI except the
  structural no-account posture; KT refimpl-only (stubbed verification);
  self-host partial (demo-class relay + bearer-token auth; no production
  durability/TLS/backup, no admission tokens).
- **Sharpening (§2)**: Signal builds/markets SPQR + KT ⇒ PQ/KT become table
  stakes; the durable edge = self-host + no-phone-number + PQ-native
  AUTHENTICATION (the NA-0634/NA-0636 line); do NOT chase the parity tail;
  the DOC-PROG-002 §6 overclaim ban carried over IN FULL and restated.
- **Tiers (§3)**: Tier 1 = self-host OPERATOR-FIRST (top priority),
  identity-verification UI, disappearing messages, basic search; Tier 2 =
  PQ-status indicator, KT verifier UI, guided/admission-token UX (ENG-0036);
  Tier 3 DEFER = reactions, quoting/replies, voice notes, groups, calls,
  stickers, polls, stories.
- **Self-host split (§4)**: qsc CLIENT admission UX (ENG-0036; admission
  control, never message security) vs CROSS-REPO qsl-server PRODUCTION relay
  (durability/authz/backup/TLS; ENG-0037 adjacent); near-term target =
  technical-operator-first, not non-technical onboarding.
- **Build order (§5)**: operator-path → identity-verify UI + PQ-status →
  disappearing + search → KT verifier UI + guided UX as they mature; Tier 3
  deferred; every step its own future lane/directive.

## 5. Validation (Phase 3)

- **Scope guard**: `git diff --name-only` vs the D577 allowed-path list —
  every changed path allowed; NO qsc/qsl-server/qsl-attachments source, NO
  `formal/`, vectors, canonical, `.github`, NO feature/GUI code. (Recorded in
  the lane response; re-runnable.)
- **Minimality of the DOC-PROG-002 edit**: the diff is exactly the three lines
  in §2 above.
- **Overclaim check**: DOC-PROG-003 scanned for comparative/superiority
  language ("more secure", "most secure", "better than Signal", "unbreakable",
  unqualified "anonymous") — no comparative security claim present; Signal
  statements are factual gap observations in the DOC-PROG-002 §3 style, and
  the document says so explicitly (§2 closing paragraph).
- **Private-material scan**: no operator-private paths, tokens, or
  non-repo material in any changed file.
- **goal-lint**: green locally against the lane diff (synthesized event; PR
  body carries `Goals: G4, G5`).

## 6. Ledger action — the OPTIONAL note was skipped, deliberately

D577 marks `docs/ops/IMPROVEMENT_LEDGER.md` "OPTIONAL — only if a small note
linking ENG-0036/0037 as self-host inputs is warranted". DOC-PROG-003 §3.2/§4/§6
already cite ENG-0036/ENG-0037 by ID with their threat-model boundaries, and
the ledger entries themselves already carry the operator-direction context. A
reciprocal ledger edit would add a maintenance surface without adding
information; the linking obligation is satisfied doc-side. SKIPPED.

## 7. Limits (stated, not hedged)

This lane recorded a PLAN. Nothing here builds, sequences into the queue, or
authorizes any feature; the tiered set binds future lanes only through the
operator's promotion of each build directive. The document states QSL's own
target properties and makes no comparative security claim. The claim boundary
is UNCHANGED. Internal scrutiny of the plan is not external review.
