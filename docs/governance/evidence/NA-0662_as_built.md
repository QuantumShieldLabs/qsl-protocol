# NA-0662 as built — GUI slice-A design pass ROUND 3 (D598; D-1285 + D-0005)

Lane: NA-0662. Directive: QSL-DIR-2026-07-19-598 (D598, APPROVED+AMENDED
2026-07-19, sha256 `bb9dc338…`, 1073 lines; verified at Phase 0). Cross-repo
FULL ritual: qsl-desktop PR #5 (branch `na0662-gui-slice-a-design-round3`
off `e818ad9a` EXACTLY, 10 files, carrying repo-local D-0005) + this spine
governance closeout (D-1285). Base spine `17bba8bc` (the #1606 seating
merge; qwork-proven; 9/9 push runs SUCCESS — formal-ci and qsc-adversarial
were still running at seating and were WAITED OUT to green before any
push). F1 executed as the operator-ruled ALTERNATIVE (tauri.conf.json
windows[0] ONLY); F2 as the DEFAULT (UI-side validation only).

## 0. qwork invariants (observed)

startup_result=OK; lane=NA-0662; repo=qsl-protocol; created;
head==origin_main==main==`17bba8bc` (yes); worktree/index/untracked clean;
ready_count=1; queue_top_ready=NA-0662; requested_lane_status=READY;
shared_target_ready=yes; proof 2026-07-20T03:36:19Z. ALL PASS. Spine base
contains `1a3d4d48` (merge-base ancestor proven); D-1284 canonical ×1;
D-1285 absent; STATE `READY=NA-0662 | HIGHEST_NA=0662 | HIGHEST_D=1284` ×1.
Disk 54% (<95%); /backup/qsl mounted. qsl-desktop fresh clone main ==
`e818ad9a` EXACTLY; `rust` SUCCESS on it; protection ["rust"]/strict/
enforce_admins LIVE (re-verified after PR creation); D-0004 top / D-0005
absent; every drafted per-item anchor re-verified (all MATCH).

## 1. The design authority, landed (item 12)

- `docs/DESIGN_SPEC_AppendixE.md` NEW = directive Appendix E BYTE-EXACT:
  cmp against the directive-extracted copy OK; cmp against the operator's
  `~/work/QSC_DESIGN_SPEC_AppendixE_round3.md` OK (informational); sha256
  `5175f3bc…`, 128 lines (both MATCH the directive's binding values). NEVER edited.
- `docs/DESIGN_SPEC.md` `34ced51b…` (113 l) → `074244be52a3ee227d59d9a0…`
  (143 l). Amendment map (each citing its [E.x] inline):
  a. header: the three-file precedence note (E governs on disagreement);
  b. §2 Usage: the AUTOLOCK line gains the 0-state (status-danger +
     warning icon, bold, the VERBATIM copy) [E.3]; the red-reserved line
     records the R2 extension (autolock-0 banner, quoted phrases, the
     "Delete vault?" link) [E.3–E.6];
  c. §4: the Settings code-box 420px/centered rule [E.7];
  d. §5: the shared ceremony-card treatment [E.4]; the instruction line
     quoted + danger mono [E.4]; full-width fields [E.4]; the erase
     countdown gate sentence [E.5];
  e. §6: the two window modes + menu-bar visibility rule [E.1]; number
     inputs (no spinners, ~64px, visible validation) [E.2]; autolock
     60/0-never [E.3]; the helper-under-banner position [E.3].
- `docs/DESIGN_SPEC_AppendixD.md` `a7d45a0a…` (222 l) → `5f5d3a2ef1c2…`
  (244 l). Amendment map:
  a. header: the round-3 precedence note;
  b. D.1: status bar 11px text-muted → 12px #A8A8A8 [E.7]; the
     one-window/wizard-card-on-canvas exception bullet REPLACED by the
     E.1 two-mode rule (sizes, menu visibility, card-fills, shortcuts/
     context-menu retention) [E.1];
  c. D.2: rail icon 19px → ~21px [E.7];
  d. D.4: the code-box Settings 420px note [E.7];
  e. D.5: `.num` "width 56px" → ~64px/no-spinners/validation [E.2]; the
     checkline 16px one-line note [E.7]; autolock example value 15 → 60
     + the 0-state danger-banner comment [E.3]; the helper
     directly-under-banner note [E.3]; the destroy ceremony-card
     wrapper comment [E.4];
  f. D.6: the wizard-card comment → fills the 560x660 compact window
     [E.1].
- No-surviving-needle greps (test-pinned in design_round3.rs): "11px
  text-muted", `value="15"`, "Locks after 15 minutes", "width 56px",
  `max="720"` — all ×0 in both amended files. The round-2 frozen pins'
  identity lines (titles, §7/D.8 headings) preserved.

## 2. The two sanctioned behavior deltas (as diffs, summary)

(a) settings.rs — EXACTLY the item-2 set:
    - `AUTOLOCK_DEFAULT_MINUTES: u32 = 15` → `= 60`
    - save(): the two-line 0-reject (`autolock_minimum_one_minute`)
      REMOVED — no range bound remains backend-side (F2 DEFAULT)
    - doc comment updated to the 60/0-never semantics
    - in-file tests amended (the sanctioned surface):
      `default_is_fifteen_minutes` → `default_is_sixty_minutes` (pins 60);
      `roundtrip_and_zero_rejected` → `roundtrip_and_zero_accepted`
      (0 saves AND loads back as 0). `settings_key_allowlist` and
      `self_alias_absent_defaults_empty` byte-untouched.
    - main.js mirrors follow: `autolock_minutes: 60` (line ~115) and
      `let autolockMinutes = 60` (timer block); the timer gains the
      BINDING never-fire guard `if (autolockMinutes === 0) return;`
      BEFORE the `autolockMinutes * 60 * 1000` comparison (source-pinned
      with ordering assert). The dead `autolock_minimum_one_minute`
      mapErr entry removed with the backend error.
(b) main.js — the countdown gate (item 11b): btn-erase now validates the
    phrase (the landed check, byte-same string), REPLACES the form with
    the E.5 countdown panel, counts 30→0 updating number + label
    ("Erasing in {N} seconds…"), and invokes `erase_all` ONLY in the
    zero branch of the interval; Cancel (`btn-erase-countdown-cancel`,
    the only action), closing the window, or ANY state transition
    (clearCeremonyState → eraseCountdownAbort) aborts with nothing
    erased and the form restored EMPTY (D597 hygiene).
    `src-tauri/src/commands.rs` is BYTE-UNTOUCHED (absent from the
    diff): erase_all, its phrase check, and its scope unchanged.

## 3. Items 1–11 as landed (pointer + E.8 demonstration)

1. Spinners/validation (E.2): both fields `type="text"
   inputmode="numeric"` (test pins type="number" ×0); `input.num` 64px
   centered + `appearance: textfield` + the two ::-webkit spin-button
   rules; autolock min=0 max=1440 (720 gone), wipe 1–100; shared
   `validateNum()` — inline message ("Enter a whole number from …") +
   `input.invalid` danger border; never clamped/ignored. SHOT: the V&S
   pane is input-locked → OPERATOR EYES beside E.2; source+test pins.
2. Autolock 60/0-never (E.3): §2 above; banner state machine in
   `renderAutolockBanner` — >0 accent+lock (landed copy), ==0
   status-danger + warning icon + fw600 with the VERBATIM E.3 sentence
   (test-pinned). NEVER-LOCKS PROOF (honest method — no real-time-wait
   hook is sanctioned): (i) settings API accepts+persists 0
   (`autolock_default_sixty_and_zero_valid`, runtime test); (ii) the
   guard precedes the elapsed comparison (source pin with index
   ordering); (iii) at 0 the interval body returns before the
   `>= 0*60000` comparison that would otherwise be immediately true —
   the lock_now path is unreachable at 0 by control flow. Banner both
   states + live no-lock behavior → OPERATOR EYES.
3. Quoted danger phrases (E.4): `.instruction code` renders
   `color: var(--danger-text)` with `::before/::after { content: '"' }`
   — the quotes are IN the rendered text while the round-2 byte-frozen
   markup needles (`Type <code>destroy my vault</code> to confirm`,
   design_system + design_round2) stay intact; the E.4 `.phrase`
   selector lands as `.instruction code` per the spec's adaptation
   rule. Typed phrase values unchanged (no quotes typed). OPERATOR
   EYES for the rendered surfaces; source+test pins.
4. Equal full widths (E.4): destroy-pass label-unwrapped →
   `.field-label` above + `class="full"` (100%); destroy-phrase +
   erase-phrase also `full`. Test-pinned. OPERATOR EYES (expanded form
   is input-driven).
5. Ceremony card ×2 (E.4): `.ceremony-card` (bg var(--bg) #1D1D1F,
   1px var(--danger-border) #8A3A3A, radius 12, padding 20/22;
   `.ceremony-head` 17/600 #F0A0A0 mb 8; body mb 10) wraps BOTH the
   Settings destroy surface and the erase screen (`.card ceremony-card`
   — the later-rule cascade override; the settings head stays
   `<h3 class="danger-h ceremony-head">` because the frozen
   `>Destroy vault</h3>` needle binds the element). Verbatim landed
   copy survives (needles green). Collapsed/expanded mechanics + D597
   hygiene rules unchanged. OPERATOR EYES beside E.4.
6. Arm checkbox (E.7): `label.inline` white-space nowrap + 16px box;
   the label element is the clickable surface. Test-pinned; OPERATOR
   EYES.
7. Helper/spacing (E.3/E.7): #autolock-error moved ABOVE the banner
   (the validation slot at the field); the helper sits DIRECTLY under
   the banner (order test-pinned with a no-error-between assert);
   `.error`/`.feedback` reserved min-heights REMOVED so empty surfaces
   collapse — the banner gaps are the `.pane` 12px scale gap. Wipe
   error likewise moved beside its control row. OPERATOR EYES.
8. Code box (E.7): `#settings-code { max-width: 420px; margin: 0 auto }`
   — wizard rendering untouched (shared class unmodified). Test-pinned;
   OPERATOR EYES (input-locked pane).
9. Legibility (E.7): `--fs-glyph` 19→21px + `.rail-btn svg` 21px (moved
   together); `.status-line` `--fs-micro`/muted → `--fs-hint` (12px) /
   `--fg-secondary` (#A8A8A8) — token discipline intact (design_system
   greps green). Rail/status visible only on the main window → OPERATOR
   EYES; token pins green.
10. Compact windows + menu (E.1; §4 below for geometry): TWO modes on
    the pinned tauri 2 core window API (set_min_size → set_size →
    center, the directive's order), applied ON MODE CHANGE ONLY
    (WindowModeState latch) riding ui_surface_changed; scr-wiped
    (absent from the E.1 table; reachable only FROM unlock) inherits
    CompactGate — recorded as the total-mapping completion, not scope
    growth. MENU BY ATTACHMENT (see §6 finding): compact modes
    `remove_menu()`, Full re-attaches the app-wide menu (`set_menu` +
    `show_menu`) — zero dependency motion, manifests byte-untouched.
    The compact card FILLS its window (screen padding var(--sp-x20),
    align-items stretch, card width 100%): no vertical void, no
    centered-card-in-a-void — visible in the shots. Shortcuts +
    right-click context-menu paste on compact screens: the webview's
    native editing behavior, no menu dependency → OPERATOR EYES (no
    input driver; recorded per the honest split).
11. Unlock + gate (E.5/E.6): link-forgot → "Delete vault?" as
    `.link-danger` (12px, `--danger-link` #C87A7A tokenized,
    underlined; landed destination unchanged — main.js routing
    byte-same); "Forgot your passphrase?" ×0 in ui/ (test-pinned); the
    unlock error inline-only (`.feedback` min-height removed; E.6
    12px); failed-attempts/delay copy byte-unchanged. Countdown per §2b.
    SHOT: s1/s2 unlock beside E.6 (the danger link + no reserved gap
    visible). Countdown/abort surfaces are input-driven → OPERATOR EYES
    + the §5 proofs.

## 4. Window-geometry table (xwininfo, virtual display)

| State (launch-reachable) | Mode          | Requested | xwininfo   | Menu bar in shot |
|--------------------------|---------------|-----------|------------|------------------|
| s0 wizard                | CompactWizard | 560x660   | 560x660    | ABSENT           |
| s0 + CLI notice          | CompactWizard | 560x660   | 560x660    | ABSENT           |
| s1 unlock                | CompactGate   | 460x420   | 460x420    | ABSENT           |
| s2 unlock                | CompactGate   | 460x420   | 460x420    | ABSENT           |
| main/Settings (Full)     | Full          | 1024x700  | input-locked → OPERATOR EYES (menu present; resize-on-entry) |

The 560-wide geometry is itself the runtime-sizing proof: the conf's
untouched minWidth 800 would clamp a config-sized window to 800 — only
the backend set_min_size+set_size path can produce 560/460. The
resize-on-entering-main transition and Full-mode geometry need input →
OPERATOR EYES, mode mapping + sizes API-pinned
(`window_modes_and_menu_visibility`). Centering: under Xvfb there is NO
window manager — placement is not authoritative (the WM-less server
parks the origin at screen center); center() is issued per the E.1 rule
and centering is verified by OPERATOR EYES on the real desktop.

## 5. Countdown + autolock-0 proofs (the honest method, recorded)

- Abort proof (source-pinned + suite green): `eraseCountdownAbort()`
  clears the interval, restores the EMPTY form, and is reachable from
  (i) the Cancel button, (ii) clearCeremonyState — i.e., EVERY screen
  transition (the pin asserts the call inside clearCeremonyState), and
  (iii) window close (the timer dies with the document; nothing armed
  persists — all countdown state is in-memory). Nothing is erased on
  any abort path: the single erase_all invoke is unreachable from them.
- Complete proof (source-pinned): exactly ONE `invoke("erase_all")` in
  main.js (count-pinned == 1), located INSIDE the interval callback
  AFTER the `if (eraseCountdownLeft > 0) return;` zero gate
  (index-ordering-pinned) — the command is invocable ONLY at countdown
  zero. The interval decrements from 30 with number+label re-rendered
  each tick (copy pinned verbatim).
- A shortened-interval test hook was NOT added (not sanctioned); the
  live 30-second run and the abort feel are OPERATOR EYES at flight.
- Autolock-0: §3 item 2. The danger banner at 0 and the no-lock
  behavior are operator-eyes; the semantics are test+source-proven.

## 6. Environment/API findings (recorded for future GUI lanes)

- NEW FINDING — tao `set_visible(true)` on Linux is gtk `show_all()`:
  it resurrects EVERY hidden child widget, the menubar included. A
  `hide_menu()` that returned Ok (and read back is_menu_visible=false)
  was undone whenever the F1 deferred first show processed — timing-
  dependent and unfixable by ordering (the show request and the
  main-thread menu closure travel different queues; a bounded
  is_visible poll cannot close the race on a busy startup loop).
  DETERMINISTIC MECHANISM CHOSEN: menu visibility by ATTACHMENT —
  compact modes `remove_menu()` (a destroyed menubar widget cannot be
  resurrected), Full re-attaches via `set_menu(app.menu())` +
  `show_menu()`. Still exclusively the pinned tauri 2 core menu API.
  Proven by the headless captures (menu ABSENT in all four compact
  shots after the fix; present before it).
- The F1 fail-open fallback: a 5-second setup thread shows the window
  if the frontend never reports a surface (an invisible app on a boot
  fault is the worse failure). Under the software-rendered rig the
  webview boots slowly, so the fallback CAN fire before the first
  report; the report then applies the mode to the already-visible
  window (menu attachment is mode-driven, so the end state is
  identical). On real hardware the report wins by seconds. Recorded
  honestly; no snap is possible either way at the conf's compact
  initial size.
- The NA-0661 rig recipe held VERBATIM (GDK_BACKEND=x11, the webkit
  software knobs, ~25 s settle, xwd sync, fresh scrot names,
  dbus-run-session). New rig notes: the Xvfb screen was bumped to
  2560x1600 because the WM-less server parks the window's origin at
  screen center (a 1280x800 screen cuts compact windows off); shots are
  PIL-cropped to the xwininfo rect for review (the uncropped originals
  and `_windows.txt` trees are retained beside them).

## 7. Proofs and gates

- Suite 56 passed / 0 failed / 1 ignored (seed hook): lib 5 +
  design_round2 17 + design_round3 16 (NEW, additive) + design_system 6
  + slice_a_flows 7(+1) + slice_a_rules 5.
- slice_a_flows.rs `be700518…` and slice_a_rules.rs `a53137ec…`
  BYTE-IDENTICAL to base (git-diff-empty + sha) and green;
  design_system.rs + design_round2.rs BYTE-IDENTICAL to base and green
  (zero amendments — the drafting collision check held).
- fmt --check CLEAN; clippy --workspace -D warnings CLEAN; metadata
  --locked OK; cargo audit EXIT 0 on the UNCHANGED lock (the same 17
  known gtk3-bindings warnings).
- Scope: the diff = docs ×3 + ui ×3 + lib.rs + settings.rs +
  tauri.conf.json + tests/design_round3.rs (NEW) + DECISIONS.md
  (D-0005) — exactly the D598 allowed paths. Cargo.toml (both),
  Cargo.lock, workflows, capabilities/default.json, commands.rs,
  .gitignore, icons, community files: ABSENT from the diff.
- tauri.conf.json diff confined to windows[0] (key-level):
  `visible: (absent)→false`, `width: 1024→560`, `height: 700→660`;
  title/label/minWidth/minHeight and every key outside windows[0]
  byte-identical. (minWidth/minHeight retained deliberately: they act
  only on the never-shown pre-first-paint window; the backend applies
  per-mode minimums before the first show.)
- qsc-symbol set head == base (23/23; comm -13 empty). No new marker
  strings (zero 'marker' tokens in added src lines). Zero-networking
  scan green (slice_a_rules, scan roots src/ + ui/ re-verified; docs/
  is outside the scan roots — the touched prose was additionally
  publication-scanned). git diff --check clean.
- Publication scan (staged added lines + both new files): class PASS,
  zero pattern hits, zero overclaims — claim discipline held on the
  danger banner, countdown copy, "Delete vault?", the validation
  messages, and every spec amendment.
- Protection ["rust"]/strict/enforce_admins verified INTACT after PR
  creation; the `rust` check runs on PR #5.

## 8. §7/E.8 deviations (each with its reason)

1. Screenshot coverage is launch-reachable only (s0, s0+notice, s1,
   s2): the host has NO input driver (the standing D596/D597 split) —
   every input-driven surface (V&S validation, banner states, ceremony
   cards, countdown mid-count/abort, main-window Full mode + menu
   present + resize-on-entry, compact paste/context-menu) is
   test/source-proven above and enumerated for OPERATOR EYES at the
   post-merge flight.
2. E.6 annotates h1 mb 14 / button mt 12; the unlock card keeps the
   round-2 uniform 12px card gap (accepted round-2 precedent; within
   the E.8 within-reason standard).
3. E.4 annotates the countdown Cancel/body spacing loosely; the panel
   lands mb 12 (--sp-3) with body mb 10 (--sp-x10) — on the scale.
4. Under Xvfb the window origin is WM-less-parked, so "centered" is
   asserted by the issued center() call + operator eyes, not by the rig
   (recorded in §4).
5. The wizard card at 560x660 has residual in-card space below the
   Create button (content-height dependent); the card fills the window
   per E.1 — no void OUTSIDE the card exists. Recorded as within
   reason.

## 9. Not claimed

Any security property improved; server connectivity (slice B, OWED);
physical-display behavior (virtual-display caveat stands); the live
30-second wall-clock countdown feel (operator eyes). The claim boundary
is UNCHANGED. External review remains THE release gate.
