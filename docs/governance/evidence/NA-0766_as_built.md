# NA-0766 — THE INVITE-FLOW LANE — AS BUILT

**Spine decision:** D-1407 · **Desktop decision:** D-0043 · **Date:** 2026-08-27
**Bases, re-derived bare and unpiped at the NAMED github remotes at the edit:**
qsl-protocol `76e46f92582e39023f169cd96fc629a9c4ae60c3` ·
qsl-desktop `15a673aa57d11a1b03acb24ad8a56ba15d56e9a8`
**Branch (both repos):** `na0766-invite-flow`

ASCII ARMOR: " -- " = em dash.

## 1. INPUTS, EVERY ONE VERIFIED BEFORE IT WAS READ

All 64 digits compared mechanically on each, with a NEGATIVE CONTROL proving the
comparator discriminates (one flipped digit -> `FAILED`, exit 1) and the tamper
proven to have LANDED first (`cmp` rc=1 at byte 64) so the arm is not vacuous.

| artifact | sha256 |
|---|---|
| `BRIEF_invite_flow_20260827.md` | `1fad5848b74343be1d5200cd12da4b88e8199989f04c98751f85e469d04875f6` |
| `RULING_NA0766_build_20260827.md` | `1323219622866141e095fa2eb3930eafecc9a0d20641d68151cc6cbc5698c8a9` |
| `MOCKUP_laneC_invite_nav_v9_20260827.html` | `8eee371fae11be7d7d1be56c572c226434da2c770ab1ec981ae7a084097b1fe7` |
| `CLOSEOUT_NA0764_NA0765_20260827.md` | `f9b519f0cd0504b917fcc9ade8640fc086917c6b8733e8429792587c3097ddd7` |
| `STOP_NA0766_003_20260827T151315Z.md` | `07f72075e6e8feb46e620683523d27bc1951d63cf9b6d48995eeffd71c001a18` |
| `PHASE0_NA0766_redsets_20260827T153511Z.md` | `f2db75df898fdd27323a86e3ac919103e06458c63644889e6d3988b155289a98` |

All banked 444 under `/srv/qbuild/operator/NA-0766/`, cmp-verified against their
sources BYTE FOR BYTE (not by a digest of the destination), with the immutability
control run on BOTH arms and **the arms shown to differ** -- the same append that
succeeds on a 644 copy is refused by the 444 bank. A 444 fixture can swallow a
tamper silently; only the differing arms prove the control is live.

## 2. THE FIFTEEN, AS BUILT

| # | item | verdict |
|---|---|---|
| 1 | "review invites" link replaces the outstanding-count hint | BUILT |
| 2 | remove the corner X from every invite modal (2 controls) | BUILT |
| 3 | remove every Back (2 controls) | BUILT |
| 4 | one full-width Close per modal | BUILT (already true for the chooser) |
| 5 | remove the "Review invites (n of 10)" pill | BUILT |
| 6 | one window, NO TRANSFORM | BUILT |
| 7 | the empty code slot's sentence | BUILT |
| 8 | the caption + accent parenthetical | BUILT |
| 9 | delete the private-note hint line | BUILT |
| 10 | a name is REQUIRED | BUILT |
| 11 | Activate & Copy moves to the bottom | BUILT |
| 12 | post-activation locks | BUILT -- and it was a real defect, see §4 |
| 13 | the copy cycle | ALREADY TRUE -- reported, not rebuilt; now pinned |
| 14 | remove the "New invite" button | BUILT |
| 15 | Connect full width with Close beneath | BUILT |
| Q1 | the failure view's foot takes the item-11 shape | BUILT (ruling's one line) |

## 3. WHAT WAS TOUCHED

**qsl-desktop.** Product: `ui/index.html`, `ui/main.js`, `ui/style.css` -- THREE
files, so **SR-15 NOT TRIGGERED**, and unlike NA-0765 that answer was not
contingent on any open ruling. Tests: `src-tauri/tests/na0766_invite_flow.rs`
(new, 8 seals), `src-tauri/tests/design_polish.rs` (7 re-aims),
`f_k_invite_create.json`, `f_l_invite_redeem.json`,
`f_n_contacts_autoconnect.json`, `scripts/ci/EXPECTED_TEST_INVENTORY.txt`.
Records: `DECISIONS.md` (D-0043).

**qsl-protocol.** RECORDS ONLY -- **ZERO product bytes**. `NEXT_ACTIONS.md`,
`DECISIONS.md`, `TRACEABILITY.md`, `docs/ops/IMPROVEMENT_LEDGER.md`,
`docs/ops/PREDICTION_LEDGER.md`, and this file.

No `.github/**`. No relay/server. No pin change. Registered commands 43 -> 43:
every verb this lane needed was already registered, which is what made a
`ui/`-only edit set sufficient where NA-0765's Rename was not.

## 4. THE FOUR FINDINGS, EACH WITH ITS MEASUREMENT

**(a) A GREEN SEAL THAT COULD NO LONGER FAIL, AND ITS CONTRADICTION ALSO GREEN.**
`design_polish.rs` asserted a retired control's label was PRESENT and was "the
ONLY post-activate action". v4 removed the control. The assertion kept passing
because `html_says` collapses whitespace but does not strip HTML comments, and
the label's only remaining occurrence was inside the comment recording the
removal. Its counterpart 700 lines below asserted the OPPOSITE and also passed,
because ITS needles were on the shipped forms.
PROVEN BOTH ARMS: raw `index.html` -> present (exactly one occurrence);
comments stripped -> ABSENT (zero). POSITIVE CONTROL: real markup survives
stripping. NEGATIVE CONTROL: a string present nowhere is absent in both.
Re-measured independently by the Director. Ruled Q5 = (B).
⇒ **A COMMENT THAT DOCUMENTS A REMOVAL RE-PLANTS THE REMOVED THING'S NEEDLE.**
Sealed as `na0766_a_comment_cannot_satisfy_a_copy_seal`. ⚠ It fired on this
lane's OWN first draft: a self-check found **TEN** re-plantings written by the
seat while documenting these very removals. All ten corrected to describe rather
than spell. Prediction rows **340**, **341**.

**(b) AN ORDERED "COSMETIC" ITEM THAT WAS SLOT ACCOUNTING.** The activate handler
set `disabled = true`, then awaited `inviteRefresh()`, which RE-ASSIGNED the flag
from the relay and cap alone -- so with a relay configured and the cap unreached
the control came back and **a second press minted a second invite and burned a
second slot**. `I6` asserts the latch survives a refresh, aimed at that exact
path.

**(c) THE BRIEF'S WRONG-ADDRESS PREMISE, CORRECTED IN THE OPEN.** Sec 2 and sec
6(c) ordered "the ledger entry" amended for the owed "require a name" item.
Measured with controls: `require a name` = 0 and `skip-and-count` = 0 in
`docs/ops/IMPROVEMENT_LEDGER.md`, with `block/unblock` = 1 as the positive
control on the same needle. The item lives in exactly two places -- `DECISIONS.md`
(sentence case) and `NEXT_ACTIONS.md` (UPPERCASE, which is why a case-sensitive
census finds it in one and not the other). Both amended; **no new ENG id**
(ruling Q3 = A). Cause attributed to the Director at prediction row **338**.

**(d) AN ORDERED INSTRUMENT THAT COULD NOT REACH ITS BOUNDARY.** Ruling Q4
ordered the cap suppression proven "with a can-fail arm at the tenth-invite
boundary". Reaching it needs TEN live invites, hence a successful
`invite_create`, hence a relay -- and the desktop harness has **no fixture
relay** (`ENG-0226`, open). The runtime arm proves the window does not move with
the cap latched; the claim it cannot make is sealed STRUCTURALLY by
`na0766_the_cap_line_is_decided_once_at_open`, whose can-fail arm is the retired
defect itself and which fired on it. Prediction row **343**.

## 5. PHASE 0 -- TEN CONTROLS, ALL TEN AT THE PREDICTED ROW

Exact red sets banked 444 **BEFORE ANY EDIT**, with both trees confirmed clean at
banking time. Every `ui/` control ran the **REBUILD BRACKET** (`frontendDist`
embeds `ui/` at build time, so a perturbation without a rebuild is a silent
no-op). Every restore cmp-identical; tree clean after each.

| control | perturbation | fired at |
|---|---|---|
| C1 | re-insert the invite X | `na0766_i1...` "a retired control is back in the markup" |
| C2 | re-insert the code-entry Back | same row |
| C2b | (second red DECLARED IN ADVANCE) | `na0766_i8...` "exactly two controls at this foot" |
| C3 | break the invite overlay's Escape | `na0766_i2...` "Escape still closes the invite overlay" |
| C4a | drop the name term | `na0766_the_name_gate...` "all four causes" |
| C4b | drop the name term (running app) | `f_k` I3 first step: measured `enabled` |
| C5' | make ACTIVATION move an element | `f_k` I4 compare: `MOVED before=[...] after=[...]` |
| C6' | put the cap toggle back in the refresh | `na0766_the_cap_line...` "EXACTLY ONE place" |
| C7 | delete the copy revert | `f_k` "copy link reverts": TIMEOUT, last=`copied` |
| C8 | clear the mint latch | `f_k` I6: measured `activate:false` |
| C9a/b | make the link a button | seal + `f_n`: measured `tag:"BUTTON"` |
| C10 | plant a retired name in a comment | `na0766_a_comment...` "back in index.html" |

⚠ **TWO PHASE 0 CORRECTIONS, REPORTED AS MISSES.** C5's banked perturbation
("hide one element pre-activation") was MIS-SPECIFIED: `I4` measures a DELTA
across activation, so an element hidden in BOTH states is invisible to it by
construction -- the perturbation went red at a later row and `I4` itself passed.
Corrected to make activation itself move something; `I4` then fired at exactly
the compare row. C6's banked arm assumed the tenth-invite boundary was reachable
at runtime; it is not (§4d), and the structural seal replaced it. Both recorded
at prediction rows **342** and **343** rather than quietly reconciled.

## 6. FIGURES (SR-22 two-pass: predicted at Phase 0, measured here)

| figure | predicted | measured |
|---|---|---|
| desktop unit tests | 187 ± 2 | **189** passed / 0 failed / 15 ignored |
| gui-driver | 14 | **14/14**, 183.37 s |
| test inventory | 202 ± 2 | **204** (+8 / −0) |
| registered commands | 43 → 43 | **43 → 43** |
| product source files | 3 | **3** ⇒ SR-15 NOT TRIGGERED |
| SR-18 re-aims | 7 named at STOP 1 | **7 fired**, of which **1 was not predicted** |

`cargo fmt --all --check` clean. Baseline before any edit: 181 passed / 0 failed
/ 15 ignored, exit 0 -- run UNPIPED after a first attempt was piped through
`tail`, which truncated the counts (self-reported, prediction row **339**).

## 7. CLAIM BOUNDARY

This document asserts what was MEASURED in this seat at the bases named above.
It does NOT assert: that the gui-driver's coverage is end-to-end (the harness has
no fixture relay, `ENG-0226`, so no scenario completes a real mint or handshake
-- the post-mint transition is driven through the product's own
`inviteAdoptCode()` with a synthetic code, the same idiom `f_n` uses); that the
tenth-invite boundary was exercised at runtime (§4d); or anything about CI, which
this seat did not run. Nothing was merged: the operator merges, twice, protocol
first.
