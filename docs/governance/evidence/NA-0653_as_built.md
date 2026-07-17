# NA-0653 As-Built — main-push public-ci link repair (D589, D-1276)

Status: Complete (at the lane PR stop; the Phase-5 exit condition is verified post-merge)
Owner: QSL governance
Last-Updated: 2026-07-17

Directive: QSL-DIR-2026-07-17-589 (D589), APPROVED 2026-07-17 with F1 resolved
to the drafted default (the fail-closed byte-proof rail); directive sha256
`8aec94051555a1c395142d98fa43af01cb02bf70510e24390644f6b90af41865` (412 lines).
Seating: promotion PR #1587, merge `a2d7c1c1` (base for this lane; qwork-proven).
Proof root: `/srv/qbuild/tmp/NA-0653_public_ci_link_repair_20260717T223023Z/`.

## 1. Phase 0 — CONFIRM-LIVE (all verified, none asserted)

- qwork proof (`/srv/qbuild/work/NA-0653/.qwork/startup.qsl-protocol.kv`):
  startup_result=OK; lane=NA-0653; repo=qsl-protocol;
  head==origin_main==main==`a2d7c1c1d7e3a144bc6d591d1900962aba27e575`;
  worktree/index/untracked all clean; ready_count=1; queue_top_ready=NA-0653;
  requested_lane_status=READY; shared_target_ready=yes.
- Disk 50% (< 95%); /backup/qsl mounted.
- D-1276 canonical count 0 pre-landing (highest accepted D-1275); anchored
  `Status: READY` count 1 = NA-0653; STATE line
  `READY=NA-0653 | HIGHEST_NA=0653 | HIGHEST_D=1275`.
- THE LIVE FIX-SET RE-ENUMERATION (the gate's own finding, two independent
  sources, in agreement):
  - CI: the current latest main-push public-ci run **29617565812** on
    `a2d7c1c1` (completed, failure): `DENY_HITS_FILES=0`, `HC_COUNT=0`,
    `TOTAL_MISSING 7` — the seven files below, each with one missing relative
    reference targeting the qsc-desktop README deleted at NA-0651 (D-1274).
  - Replica: the "Markdown link check (relative links)" step's embedded Python
    extracted mechanically from `.github/workflows/public-ci.yml` lines 397–429
    at base (10-space YAML indent stripped; sha256
    `fa5ec03381792f11a5003fc588166b157ddfb3afa2de40a92486938aee69199b`), run at
    base: the SAME seven, `TOTAL_MISSING 7`, exit 2, over the full scan set of
    **721 files** (README.md + docs/**/*.md). These seven were the ONLY missing
    links repo-wide — the seven repairs provably reach zero.
- F1 premise re-verified per file: ZERO fix-set docs under docs/public/; every
  flagged line sits in an end-of-doc "Related Evidence" (six) or "Evidence
  Consulted" (NA-0250) list, OUTSIDE every boundary-styled section. The
  NA-0250 line-96 mention of the old README is a FENCED command transcript
  (inside "Exact Commands Run") — fence-stripped by the gate, not a link,
  byte-untouched by this lane.

## 2. The fix set and the repair (Phase 1)

Each file had exactly ONE flagged reference line; each became the D587-F2
as-merged history prose — the reviewer-package references-list precedent line
copied byte-exactly: the bullet's display text preserved byte-for-byte, then
the byte-identical 114-byte suffix retargeting to git history and the
superseded DOC-QSC-009/010. Zero markdown link syntax introduced; the old link
syntax reproduced nowhere in this lane's tracked-markdown output (described
instead, per the NA-0651 friction lesson — the gate's parser strips fences
only and matches link syntax inside inline spans).

| # | File | Line | Display text (preserved) |
|---|------|------|--------------------------|
| 1 | docs/demo/DESKTOP_SIDECAR_ADVERSARIAL_STRESS.md | 173 | QSC desktop prototype README |
| 2 | docs/demo/NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md | 193 | QSC desktop prototype README |
| 3 | docs/demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md | 239 | qsc desktop prototype README |
| 4 | docs/governance/evidence/NA-0250_external_review_release_readiness_audit.md | 142 | qsc desktop README |
| 5 | docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md | 181 | qsc desktop prototype README |
| 6 | docs/governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md | 227 | QSC desktop prototype README |
| 7 | docs/governance/evidence/NA-0264_desktop_sidecar_stress_audit.md | 240 | QSC desktop prototype README |

## 3. The F1 byte-proof rail (Phase 2) — ALL HELD

- Whole-diff minimality: `git diff --numstat` = exactly `1 1` for each of the
  seven files (+7/−7 total; no other hunk in any of them).
- Boundary-styled sections sha256-IDENTICAL before/after (extraction rule:
  heading line through the line before the next `## ` heading; for the
  label-form section, the label line through its trailing blank):

| Section | Lines | sha256 (pre == post) |
|---|---|---|
| DESKTOP_SIDECAR_ADVERSARIAL_STRESS "What Is Not Proven" | 136–150 | `b30e99e4f359e60928541a12a6437e2266b35c524b1292b0f272aa093a52197e` |
| PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS "Safe public wording" | 15–21 | `921da69b9568e84c309ce4e66b2af161f1efe1c4b228e37ab5211fa8dbcfa190` |
| NA-0258 "Non-Production Claim Boundary" | 176–191 | `294937068ec263a9085228be1c82f4d212660ab62d916cc7682903e5fe24e3ae` |
| NA-0264 "Non-Production Claim Boundary" | 210–224 | `6b0a12d7f48d2a362e3b68ddabf7eefaa99b76bf6ba061a341afa3f3631ef52a` |

- Pre-edit whole-file sha256 of all seven files recorded in the proof root
  (`files_pre_edit.sha256`); the post-edit delta in each file is exactly the
  one repaired line, per the numstat above.
- The NA-0250 fenced transcript line (96) present verbatim post-edit; zero
  hits for it in that file's diff.

## 4. Acceptance (Phase 3) — A/B/C/D

- **A (gate replica at head):** `TOTAL_MISSING 0`, exit 0, repo-wide over the
  same 721-file scan set. (At base the same replica byte-copy printed the same
  7 as CI with exit 2 — replica-vs-CI agreement proven before editing.)
- **B (minimality):** the numstat and section-hash proofs of §3.
- **C (repair form):** the canonical suffix occurs in exactly the seven
  repaired files plus the NA-0651 precedent line in
  docs/public/EXTERNAL_REVIEW_PACKAGE.md (8 file-hits under docs/, no strays);
  the added lines contain ZERO bracket-paren link syntax (grep over the diff's
  added lines: 0).
- **D (untouched surfaces):** the lane diff = the seven docs + the
  governance/closeout set ONLY (NEXT_ACTIONS.md, DECISIONS.md D-1276,
  TRACEABILITY.md, the journal, this as-built, the testplan); zero diff under
  .github/, scripts/, formal/, vectors/, or any source tree; root
  Cargo.toml/Cargo.lock absent from the diff; docs/public/** absent from the
  diff (the reviewer package byte-untouched).

## 5. Validation defaults

- `git diff --check` clean.
- `cargo metadata --locked --format-version=1` OK.
- `cargo fmt --check`: exactly the KNOWN 145 pre-existing diffs at base (zero
  lane Rust — recorded, not fixed; the pre-existing rustfmt drift is a filed
  micro-lane candidate from NA-0651).
- Root `cargo audit` exit 0 (386 crate dependencies); nested qsc fuzz
  `cargo audit` exit 0 (287 crate dependencies).
- `sh -n` and `bash -n` on scripts/ci/qsc_adversarial.sh OK.
- Added-line scans over the diff (private-material / prohibited / overclaim
  patterns): 0 hits — the seven added lines are identical history prose.
- goal-lint local OK with a synthesized event payload; the PR body carries the
  literal `Goals: G4` line from creation.

## 6. Result

**PUBLIC_CI_LINK_REPAIR_PASS** — with the explicit note that the lane's
Phase-5 EXIT CONDITION (main-push public-ci GREEN, printing `TOTAL_MISSING 0`,
on the lane PR's merge commit) is the operator-merge-pending step at this
stop: the link check is push-only and never executes on the PR, so the
pre-merge proof is acceptance A. NOT claimed: any historical readiness claim
re-validated; the gate weakened (byte-untouched); any GUI exists; ENG-0046
moved. Claim boundary UNCHANGED, byte-proven per the F1 rail. No ledger edit:
the issue was deliberately surfaced-unfiled by NA-0652 for operator
disposition, and this lane fixes it — nothing remains owed.
