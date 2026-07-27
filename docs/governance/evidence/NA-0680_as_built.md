# NA-0680 — AS-BUILT: onboarding & settings polish (D615)

**Result class:** `ONBOARDING_SETTINGS_POLISH_PASS`
**Directive:** `QSL-DIR-2026-07-26-615_onboarding_settings_polish.md`, sha256
`32a15f3f9bb2542b2d9117d1ef72c8b6d158dc316c79bfc32c9bda7195de8e9c`, 399 lines.
**Repos:** `qsl-desktop` (5 merged PRs, decisions D-0015…D-0020) + this spine closeout.
**Testplan (the instrument):** `tests/NA-0680_onboarding_settings_polish_testplan.md`.

---

## 0. What this lane was, and what it turned out to be

It was scoped as broad-but-shallow visual polish across four surfaces plus a set of vault findings.
It delivered that. It also became, unintentionally, **the clearest measurement this project has of
the gap between a green test suite and a working application** — and it revised a ratified contract
along the way.

Headline numbers: **97 tests pass / 1 ignored / 0 fail**; **26 negative controls**; **6 desktop
PRs**; **6 desktop decisions**; **1 D595 contract revision**; **4 ENG findings**; and **8 defects
found by a human looking at the screen that CI could not see**.

---

## 1. ⚠ The census changed the lane before a line was written

Three of the intent's nineteen rulings rested on premises the tree contradicts, and a fourth would
have shipped a false sentence. All four were written from the running app rather than the source.

| ruling | premise | reality |
|---|---|---|
| **R-15** (highest priority) | destroy-pane failures may count toward wipe-after-N | **NO.** The counter increments at exactly one place — `protection.rs:167` inside `unlock_guarded_at` — and `destroy_with_passphrase` never routes through it. **No backend, no split.** |
| **R-19** | "a small heuristic on top of the existing meter" | **There is no meter.** D597 deleted it and `design_round2.rs:41` enforces its absence. **Removed from the lane entirely** by ruling; needs its own design session. |
| **R-1** | the shipped focus ring is `border-color + box-shadow` | **There is no focus `box-shadow`.** It is a 2px `outline`. A lane that faithfully "removed the glow" would have edited nothing. |
| **R-17** | `vault_locked` means the vault is locked | **In the destroy pane it means WRONG PASSPHRASE** — Settings is unlock-gated. The suggested wording would have been false at the site the finding named. Appendix F.8 therefore maps **by site, not by code**. |

**R-3 was verified TRUE and shipped** (`self_alias` has three call sites, none outward), with a
needle added because the messaging epic is building invites now — the same defect class NA-0675
had just paid off.

---

## 2. ⚠ R-14 took FOUR fixes and three diagnoses, and only the last was structural

The single most instructive thread in the lane. The reported defect — "Delete vault?" clipped after
autolock — was diagnosed and "fixed" three times before the cause was found.

| attempt | diagnosis | why it was wrong |
|---|---|---|
| 1 (drafted) | the window is 20px too short | hand-measured bumps re-apply the construction that caused it |
| 2 (D-0017) | heights are measured against empty conditional elements; make the table a **floor** | ⚠ **the floor CAUSED six of the seven instances the flight found** — it holds a window open when content is shorter. My inference, never an operator instruction. |
| 3 (D-0019) | the card is **stretched** so `scrollHeight` measures the window; children **shrink** so content squashes | correct — and **FACT 2 reframes the original defect: "Delete vault?" was never below a scroll fold, it was SQUASHED** |
| 4 (D-0020) | the sync existed and was **not called** from five of six writers | the structural fix: one writer, which resizes |

⚠ **The lesson recorded at D-0020: I wrote the class and implemented the instance.** D615 says the
sync runs "after **ANY** write to a conditional element". D-0017 wired it at the one write the
finding happened to name. **The general rule was correct, written by me, and in the document I was
working from** — writing it felt like having handled the general case.

**The two real causes are a CSS default and an inherited layout rule** (`align-items: stretch`, and
the absence of any `flex-shrink` declaration). Neither appears in any diff, which is why no
diff-scoped review could have surfaced them.

---

## 3. ⚠ D-0018 — a D595 contract revision, authorised as such

**ENG-0076:** R-7 made the onboarding name mandatory. The identity record is written when the step
**opens** (`identity_ensure` → `identity_self_kem_keypair` → `identity_write_public_record`), so a
kill between opening and Continue left a nameless keypair, resume resolved **S2**, and R-7's gate
was silently bypassed. **A GATE-1 regression: I added a requirement at one entry point and left a
hole at another.**

**The signal, and why not the obvious one.** `self_alias`-absent was proposed and **withdrawn**:
`skip_serializing_if = "String::is_empty"` omits an empty alias, so key-absent also matches "name
cleared in Settings" **and** every pre-R-7 profile — including the operator's own live profile,
which would have been re-routed through onboarding, exactly what D615's F4 forbids.
**`settings.json`'s EXISTENCE is unambiguous** because no write path precedes Continue — traced,
not assumed, and **pinned** so a future pre-Continue write cannot break it silently.

⚠ **The scope was corrected 7 → 2 by the operator's stop condition.** The change was first reported
as breaking seven `slice_a_flows.rs` tests. **Only two actually fail**; the other five pass in
isolation and were failing as a **cascade** — that file's shared `env_lock()` `Mutex` is *poisoned*
by the first panic. **The failure list overstated the blast radius 3.5×**, and five behavioural
contracts were about to be "amended" to accommodate a lock artefact. Filed as **ENG-0077**.

Both real amendments **are** R-7 supersessions, so no stop fired. D595's S1/S2 text is revised
**mark-don't-rewrite**, citing R-7 as the superseding decision.

---

## 4. What shipped

| gate | decision | content |
|---|---|---|
| 1 | **D-0015** | focus ring (outline→border), merged vault-create copy, accent callout, identity step per 07B, five mockups |
| 2 | **D-0016** | Identity + Vault & Security panes, content-driven sizing (first attempt), R-17 per-site wordings, R-18 countdown handle, R-11 remaining counter |
| — | **D-0017** | flight fixes: sizing class, code-box, disclosure removal, section padding |
| — | **D-0018** | the D595 contract revision (ENG-0076) |
| — | **D-0019** | re-flight: Facts 1 and 2 |
| — | **D-0020** | R-14 third occurrence: one writer for the unlock feedback |

**Design authority amended, marked never rewritten:** `DESIGN_SPEC.md` §2 (the red reservation
**refined**, not reversed — danger *text* permitted, danger *chrome* absolute to the destroy
ceremony), §3, §4, §6; Appendix E `[E.1]` (sizes → **minima**); **Appendix F.8** (the per-site
wordings); mockups 07B and 09 v2 in lockstep.

---

## 5. Negative-control discipline — 26 controls, and what they caught

Every needle authored: break the property → **RED** → revert → **GREEN**.

⚠ **Three caught defects in the NEEDLES, not the code** — all the same shape, a substring ban
applied too widely, and **all three were GREEN before their control ran**:
- a blanket `box-shadow` ban fired on `.settings-rail .cat.active`'s active-nav bar;
- a bare `"Your identity"` ban fired on **mockup 07B's own subtitle**;
- a `"copy"` ban fired on **the comment explaining there is no copy button**.

That produced shared `strip_html_comments` / `strip_rust_comments` / `strip_js_line_comments`
helpers. **A needle that bans a substring across a region must exclude the prose documenting the
ban, or it fires on its own rationale.**

⚠ **And one control silently no-opped and reported GREEN** — its edit did not match its target, so
the test "passed", which is indistinguishable from a test that cannot fail. Re-run with the edit
**asserted applied first**, it went RED correctly. **A control is itself an instrument; an
unverified one proves nothing.**

---

## 6. ⚠ The operator-flown findings — the evidence for two owed lanes

**Eight defects were found by a human looking at the screen. A green suite saw none of them.**

Seven sizing instances, a clipped glyph, a vanishing link, an unnecessary line of copy. The
sharpest case: `every_window_tracks_its_content_in_both_directions` feeds **synthetic** values and
passed on a build whose windows were wrong in six places, because **the defect was in what reached
the function**.

The build host has **no input driver** — `xdotool`, `ydotool`, `wtype`, `xte` all absent; Xvfb
renders but cannot type. So every behavioural GUI defect in this project is discoverable **only**
by the operator, never by CI. **That bounds what "CI green" can mean for the desktop**, and it is
the standing evidence for:

- the **input-driver lane** — would move most of §B into CI, leaving only judgement calls; and
- the **negative-control audit** — three needles here pinned the wrong thing while green.

Both were already on the owed list. This lane measured the gap rather than asserting it.

---

## 7. ENG findings filed

| id | finding | status |
|---|---|---|
| **ENG-0075** | `cargo test -q` hides WHICH tests ran — a deleted test file stays green at a lower total | FILED |
| **ENG-0076** | R-7 regression: resume bypassed the name gate | ✅ **FIXED** at D-0018, verified live |
| **ENG-0077** | `env_lock()` poisons on panic — one failure reports as many (overstated 3.5× here) | FILED |
| **ENG-0078** | a warning is not a remedy: `style.css`'s in-file caution was violated by the lane that read it | FILED |

⚠ **Grouped deliberately as ONE family** — *instruments that do not instrument*. A pipe's exit
code, a test total, a failure list and a code comment are all **proxies that represent without
enforcing**. Fix three and leave the principle unnamed, and the next proxy gets trusted the same
way. The remedy in every case is to make the machine do the checking.

---

## 8. What this PASS does not assert

- Nothing about messaging, contacts or the relay.
- No security-assurance claim: the vault gates were **restyled, not strengthened**, and are
  behaviour-frozen.
- **Not** that §A would catch a recurrence of every §B finding. Where the fix is structural the
  class is pinned; where the outcome is a pixel it is not, and the testplan says so.
- The R-19 strength-meter work is **not** in this lane and no part of it was started.
