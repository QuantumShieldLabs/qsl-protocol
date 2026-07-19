# NA-0656 as-built — qsl-desktop satellite registration + DOC-PROG-004 v0.1.0 → v0.2.0 (D592, D-1279)

Result class: SATELLITE_REGISTRATION_PASS (stopped at the open PR; the
operator merges). Directive: QSL-DIR-2026-07-18-592 (D592, APPROVED
2026-07-18, UNAMENDED — sha256 `00483f5f…`, 585 lines; F1/F2/F3 resolved to
the drafted defaults, rulings recorded in the NA-0656 promotion block).
Base: main `557bb8b2` (the #1593 seating merge; qwork-proven). Proof root:
`/srv/qbuild/tmp/NA0656_qsl_desktop_satellite_registration_20260719T002928Z/`.

## §1 Phase 0 (all green; neither directive STOP fired)

- qwork proof (startup.qsl-protocol.kv): startup_result=OK; lane=NA-0656;
  head==origin_main==main==`557bb8b2` (re-verified live post-fetch);
  worktree/index/untracked clean; ready_count=1; queue_top_ready=NA-0656;
  requested_lane_status=READY; shared_target_ready=yes.
- Disk 52% (<95%); /backup/qsl mounted.
- D-1279 canonical count 0 pre-lane (decision_id_counter.py exit 0); highest
  canonical D-1278; the 7 pre-existing repo-wide `D-1279` mentions were ALL
  NEXT_ACTIONS.md promotion text (expected, verified).
- Anchored `^Status: READY` ×1 = NA-0656; STATE as seated.
- Main health on `557bb8b2`: 7 push workflows success; formal-ci +
  qsc-adversarial in progress = their normal windows.
- **STOP check 1 — qsl-desktop EMPTY:** `git ls-remote` returned ZERO refs;
  API size 0; public. NOT fired.
- **STOP check 2 — PVR:** `private-vulnerability-reporting` enabled=true.
  NOT fired.
- Baked repo-truth facts re-verified: merges `b3cfd5df`, `e46cb6b3`,
  `0a8e0843`, `345edcd9` each `git merge-base --is-ancestor` HEAD OK;
  ENG-0044 `Status: open`; `pub fn vault_init_with_passphrase` and
  `pub fn identity_ensure` present; `"vault_exists"` present as an init
  error code (not a function); `timeline/mod.rs` zero `content` matches
  (TimelineEntry: id/peer/direction/byte_len/kind/ts/target_device_id/
  state/status — no content field); DOC-PROG-004 existed ONLY as v0.1.0;
  DOC-CTRL-001 §4.5 row read v0.1.0.
- Commit identity: the operator's GH007 standing ruling applied —
  repo-local user.email `238594419+Tebbens4832@users.noreply.github.com`.

## §2 The rename + byte-exact landing (Phase 1)

- Baseline captured pre-mutation: `git show HEAD:docs/program/DOC-PROG-004_QSC_GUI_Phase_Roadmap_v0.1.0_DRAFT.md`
  → proof-root `baseline_v0.1.0.md`, sha256 `46f5e40a…` (157 lines).
- Tracked rename: `git mv` v0.1.0 → v0.2.0 filename; staged summary:
  `rename docs/program/{…v0.1.0_DRAFT.md => …v0.2.0_DRAFT.md} (51%)`.
- Appendix A extracted MECHANICALLY from the directive (awk between the
  BEGIN/END markers, exclusive) → `appendixA_raw.md`; exactly ONE
  `<LANDING-BASE>` token present.
- LB resolved: `<LANDING-BASE>` → `557bb8b2` (the Phase-0 base). LN NOT
  exercised (promotion seated NA-0656/D-1279 exactly as drafted).
- Landed file: 266 lines, sha256 `7920fbf3…`; **cmp-proven BYTE-EXACT**
  against the LB-resolved appendix (`sed` of the raw extract piped to
  `cmp` — empty output, exit 0). Zero `<LANDING-BASE>` tokens remain.

## §3 Fidelity diff-proof (Phase 4 core): 16 hunks, every one mapped

`git diff --no-index -U0 baseline_v0.1.0.md <landed v0.2.0>` = EXACTLY 16
hunks (proof-root `fidelity_v010_v020_U0.diff`; a U2 rendering is also
preserved). The v0.1.0 text outside these hunks is byte-untouched (the diff
is exhaustive by construction). Hunk → class:

| # | Hunk header | Content | Class |
|---|---|---|---|
| 1 | `@@ -5 +5 @@` | Last-Updated line | R0 |
| 2 | `@@ -11 +11 @@` | title version v0.1.0 → v0.2.0 | R0 |
| 3 | `@@ -20,0 +21,8 @@` | provenance blockquote: the v0.2.0 revision paragraph | R0 |
| 4 | `@@ -27,0 +36 @@` | the `**Revised:**` line | R0 |
| 5 | `@@ -32,0 +42,5 @@` | the `**v0.2.0 verified against:**` paragraph | R0 (carries **LB** → `557bb8b2`) |
| 6 | `@@ -46,0 +61 @@` | the L9 locked-decision row | R1 |
| 7 | `@@ -65,0 +81,16 @@` | Completed-section entries (steps 1/2/3a, NA-0655 parallel, step 3b) | R6 |
| 8 | `@@ -67,0 +99 @@` | step-1 status line | R6 |
| 9 | `@@ -72,0 +105,2 @@` | step-2 status line | R6 |
| 10 | `@@ -81,5 +115,24 @@` | step-3 rewrite | **R6+R7 composite** (see below) |
| 11 | `@@ -87,3 +140,5 @@` | gate D-A block: pending → DECIDED (2026-07-17) | R1 |
| 12 | `@@ -97,0 +153,9 @@` | step-4 ENG-0044 design refinement (2026-07-17) | R4 |
| 13 | `@@ -105,0 +170,21 @@` | step-5 skeleton/onboarding additions (2026-07-17/18) | R5 |
| 14 | `@@ -120,0 +206,8 @@` | step-7 message-history design (2026-07-17) | R3 |
| 15 | `@@ -129,0 +223,16 @@` | the Horizon section (2026-07-17) | R2 |
| 16 | `@@ -154 +263 @@` | trailer version | R0 |

Hunk 10 sub-map (the D592 class definitions anticipated this composite —
R6 explicitly includes "the status parts of the step-3 rewrite" and R7 is
the Registration record; the two are contiguous, so U0 merges them):
- removed lines (the v0.1.0 3a/3b bullets) + added lines `- **3a (operator):
  DONE 2026-07-17** …` through `…(D-1279)** — the registration record:` = **R6**;
- added lines `**Registration record (v0.2.0, 2026-07-18).**` through
  `…private vulnerability reporting ENABLED.` = **R7**.

Class census: R0 ×6, R1 ×2, R2 ×1, R3 ×1, R4 ×1, R5 ×1, R6 ×3 + hunk-10
part, R7 × hunk-10 part; LB ×1 (in hunk 5); LN ×0. ZERO hunks outside the
enumerated classes.

## §4 Manifest re-check (landed v0.2.0)

9 locked-decision rows (L1–L9); step headings 1–9 all present; gate D-A
DECIDED ×1 + gate D-B present (open); `## Horizon` ×1; 6 parallel-track data
rows; 7 corrections; highest NA reference = NA-0656 (this lane) — zero
references above it.

## §5 Coupled edits (Phase 2, exactly D592 Appendix B)

- **B1 DOC-CTRL-001**: the §4.5 row pair → `— v0.2.0 —` + the v0.2.0
  Location; `Last Updated: 2026-07-16` → `2026-07-18`. The doc's own
  Version stays v1.0.1. Nothing else (diff = the three lines).
- **B2 DOC-PROG-003**: the §6 bullet → `…Landed per D586 (NA-0650); revised
  to v0.2.0 per D592 (NA-0656).`; `Last-Updated` → `2026-07-18
  (NA-0656/D592: DOC-PROG-004 v0.2.0 back-pointer refresh, §6)`. Nothing
  else (diff = the two lines).

## §6 PART 1 registration — D578-terms cross-check

The landed Registration record (step 3 of the v0.2.0 doc) + D-1279 mirror
the qsl-server registration terms (D578: "qsl-server is a SATELLITE repo,
NOT a peer… ALL directive/queue/decision authority lives in the qsl-protocol
spine; qsl-server has no qwork, no directives dir, no queue-as-authority, no
guardrail hooks"; D-1265: "qsl-server has no queue/directive authority of
its own… THIS repo carries the governance closeout only"): term-for-term —
satellite-not-peer ✓; all authority in the spine ✓; no qwork/directives/
queue-as-authority/guardrail hooks ✓; cross-repo lanes spine-governed (code
PRs there, closeout here) ✓; PLUS the two qsl-desktop-specific additions the
directive prescribes: the rev-pinned qsc dependency clause (bump-lane
pattern per ENG-0041/D-1266 + ENG-0046/D-1277) and the four-item
owed-at-bootstrap list (F3: CI gate + branch protection; community-health
set; pointer CLAUDE.md; repo-local DECISIONS log) with the live-verified
registration-time state (public, EMPTY, PVR ENABLED, 2026-07-18).

## §7 Changed-path census (the 9-path allow-list, exact)

1. `docs/program/DOC-PROG-004_QSC_GUI_Phase_Roadmap_v0.1.0_DRAFT.md` (D, by rename)
2. `docs/program/DOC-PROG-004_QSC_GUI_Phase_Roadmap_v0.2.0_DRAFT.md` (A, by rename)
3. `docs/master/DOC-CTRL-001_Documentation_Master_Index_Release_Packet_v1.0.1_DRAFT.md` (M)
4. `docs/program/DOC-PROG-003_QSC_Feature_Plan_Tiered_Feature_Set_and_Build_Order_v0.1.0_DRAFT.md` (M)
5. `DECISIONS.md` (M — the D-1279 append)
6. `TRACEABILITY.md` (M — one row)
7. `NEXT_ACTIONS.md` (M — the queue flip)
8. `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` (M — the lane entry)
9. `docs/governance/evidence/NA-0656_as_built.md` (A — this file; `git add -f`)
10. `tests/NA-0656_qsl_desktop_satellite_registration_testplan.md` (A)

(Ten git paths = the 9-path allow-list with the rename counted as its two
sides.) Boundaries: NO code/test/dependency/lockfile/workflow change; NO
qsl-desktop commit/settings; NO claim-boundary/public-docs/README/
release-plan/ledger/DOC-CTRL-002 change.

## §8 NOT claimed

No GUI exists; the bootstrap has NOT landed (all four owed items remain
owed); no qsl-desktop content or settings changed; no implementation is
authorized by the revised doc (its authorization-boundary paragraph landed
verbatim). Claim boundary UNCHANGED.
