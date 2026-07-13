# NA-0641 Testplan — QSC Feature Plan Doc Lane (D577, D-1264)

Goals: G4, G5
Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-13

## Objective

DOCS/governance lane: author DOC-PROG-003 (the QSC feature plan), fix BOTH
dangling DOC-PROG-002 references (§4 + §8, per the D577-A1 amendment), add the
DOC-CTRL-001 index entry. No code; the "tests" are document-validation checks.

## Validation matrix

| # | Check | Method | Expected |
|---|-------|--------|----------|
| 1 | DOC-PROG-003 exists, house form | file present under `docs/program/`; header has Goals/Status/Owner/Last-Updated/Authority; provenance block; DRAFT in title + footer | PASS |
| 2 | Content spec coverage | §1 current state, §2 sharpening, §3 tiers (3 sub-tiers), §4 self-host split, §5 build order, §6 cross-refs, §7 maintenance rule — all present | PASS |
| 3 | Authorization boundary stated | "authorizes NO implementation" in the provenance block AND §5 AND the footer | PASS |
| 4 | DOC-PROG-002 diff minimal | `git diff docs/program/DOC-PROG-002*` = exactly 2 parenthetical `(DOC-PROG-003)` insertions (§4 line ~121, §8 line ~201) + the Last-Updated header line; zero deletions of prose | PASS |
| 5 | Both dangling refs resolved | grep `qsc feature plan` in DOC-PROG-002 → every occurrence adjacent to `DOC-PROG-003` | PASS |
| 6 | DOC-CTRL-001 entry | new §4.5 Program Documents subsection with the DOC-PROG-003 row + location; External bundles renumbered §4.6; repo-wide grep for external `§4.5` refs = none broken | PASS |
| 7 | Scope guard | `git diff --name-only` ⊆ D577 allowed-path list; no source/`formal/`/vectors/canonical/`.github` | PASS |
| 8 | Overclaim scan | grep DOC-PROG-003 for comparative/superiority language ("more secure", "most secure", "better than", "unbreakable", "anonymous") | no comparative security claim |
| 9 | goal-lint | local run, synthesized event, `Goals: G4, G5` in PR body | exit 0 |

## Out of scope

No feature/GUI/source code (D577 Tier-5); no ledger edit (the OPTIONAL note
was skipped — rationale in the as-built §6); no claim movement; the NA-0635
prekey gate untouched.
