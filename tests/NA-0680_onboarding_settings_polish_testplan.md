# NA-0680 Test Plan — onboarding & settings polish (D615)

## Status

**FLOWN AND PASSED**, 2026-07-26. §A is automated and runs in `qsl-desktop`'s `cargo test`
(97 pass / 1 ignored / 0 fail). §B is the operator's live acceptance flight, flown in **two
rounds** — round 1 found seven defects that §A could not see, round 2 confirmed the fixes and
found one more, and a final spot-check closed it.

⚠ **§B is not a formality and this lane is the proof.** Every finding below was invisible to a
green suite: dead space, a clipped glyph, a window that measures itself, a link pushed out of
view. **§A pins mechanisms; only §B judges outcomes.** See §C.

---

## §A — Automated coverage (`qsl-desktop`, socket-free, in `cargo test`)

| needle file | what it pins |
|---|---|
| `design_polish.rs` | the lane's own surface — 22 needles, every one negative-controlled |
| `design_system.rs` | amended: the Appendix-A copy set (R-8's hint removed, "Your name") |
| `design_round2.rs` | amended: step-2 heading, the autolock note's removal, Disarm's tier, the status-line component |
| `design_round3.rs` | amended: the autolock state machine's PROPERTY on its new renderer, the code card's alignment, the window table as minima, the Appendix-E header |
| `slice_a_flows.rs` | amended under D-0018: the S1/S2 discriminator (2 tests) |

**Negative-control discipline (operator standing rule):** every needle this lane authored ships
with a proof it can fail — break the pinned property, observe **RED**, revert, observe **GREEN**.
**Twenty-six controls were run.** Evidence: `docs/governance/evidence/NA-0680_as_built.md` §5.

⚠ **Three controls caught defects in the NEEDLES rather than the code**, all the same shape — a
substring ban applied too widely (a `box-shadow` ban firing on the active-nav bar; a
`"Your identity"` ban firing on the mockup's own subtitle; a `"copy"` ban firing on the comment
explaining there is no copy button). All three were GREEN before their control ran. See §C.

---

## §B — Live acceptance flight (the operator flies it; there is no input driver on the build host)

Launcher: `/srv/qbuild/work/NA-0680/.qwork/reflight.sh` — throwaway profile via `QSLD_DATA_DIR`,
`GDK_BACKEND=x11`, seeds nothing.

⚠ **Every expectation was written down BEFORE each run.** A record that captures only surprises
cannot distinguish "passed" from "not run".

### B.1 — Round 1 (post-GATE-2)

| # | check | expected | observed | verdict |
|---|---|---|---|---|
| 1 | R-16: autolock from Settings redirects | Unlock screen appears | Unlock appeared, notice shown, link visible | **PASS — R-16 CLOSED** |
| 2 | Finding 5: kill before Continue | `settings.json` absent; resume AT the identity step | absent; resumed at the identity step | **PASS** |
| 3 | Unlock window sizing | no dead space | correct | **PASS** |
| 4 | code box, fresh path | not clipped | clean | **PASS** |
| 5 | onboarding copy / name gate / disclosure placement | per D615 | correct | **PASS** |
| 6 | seven sizing instances | content-driven | **6 too tall, 1 too wide** | **FAIL → Findings 1–4** |
| 7 | code box, resume path | not clipped | **clipped** | **FAIL** |

### B.2 — Round 2 (post-fixes)

⚠ **The decisive measurement was made falsifiable rather than judged.** Round 1 asked "is there
dead space?" — a judgement. Round 2 asked "do two surfaces report DIFFERENT heights?" — a number.

| check | expected | observed | verdict |
|---|---|---|---|
| vault-create height | ≠ 765 | **649** (584 inner ≈ the 585 measured by hand at round 4a) | **PASS** |
| identity height | ≠ vault-create | **636** — differs | **PASS — the measurement computes, not inherits** |
| Settings width | 772 + 28 chrome = 800; insets symmetric | **800, even** | **PASS** |
| code box, resume path | not clipped; same height as fresh | **636 — identical to fresh** | **PASS — path-independent** |
| erase confirm / countdown | no dead space | clean | **PASS** |
| wrong passphrase on Unlock | "Delete vault?" stays visible | **link vanished** | **FAIL → R-14, 3rd occurrence** |

### B.3 — Spot-check (post-fix)

| check | expected | observed | verdict |
|---|---|---|---|
| wrong passphrase on Unlock | feedback appears, window resizes, link stays visible | exactly that, no clip | **PASS** |

**Result class `ONBOARDING_SETTINGS_POLISH_PASS` is asserted on B.3.**

⚠ **One row in the round-2 checklist was mis-specified by the Director**, not failed by the app:
the "Vault erased" screen is reachable only from the armed-wipe path; the manual erase correctly
reloads to S0 → Create vault, because the user *chose* to erase. Recorded so the plan's history is
legible rather than tidy.

---

## §C — What this plan demonstrates about its own §A

⚠ **A green §A coexisted with six visibly wrong windows.** The clearest instance:
`every_window_tracks_its_content_in_both_directions` feeds its function **synthetic** values and
passed throughout, because the defect was in what *reached* the function — a card stretched to the
window, measuring itself.

**A test that pins a mechanism is not a test that pins an outcome.** Every finding in §B is a
pixel or a path, and the build host has **no input driver** (`xdotool`, `ydotool`, `wtype`, `xte`
all absent; Xvfb renders but cannot type). So:

- **the operator is the only instrument that can run §B**, and
- **§A's scope is now stated in the tests themselves** rather than implied.

This is the standing evidence for two owed items: the **input-driver lane** (which would move most
of §B into CI) and the **negative-control audit** (three needles here pinned the wrong thing while
green).

---

## §D — What a PASS does NOT assert

- **No claim about messaging, contacts, or the relay** — untouched by this lane.
- **No security-assurance claim.** The vault, unlock, destroy and erase *gates* are behaviour-frozen
  and were restyled, not strengthened.
- **No claim that §A would catch a recurrence of any §B finding.** Where a fix is structural — one
  writer for the unlock feedback, one derivation for the Settings width — §A pins the **class** and
  will catch it. Where the outcome is a pixel, it will not, and that is stated rather than implied.
