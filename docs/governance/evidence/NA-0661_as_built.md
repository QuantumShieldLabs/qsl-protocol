# NA-0661 as built — GUI slice-A design pass ROUND 2 (D597; D-1284 + D-0004)

Lane: NA-0661. Directive: QSL-DIR-2026-07-19-597 (D597, APPROVED+AMENDED
2026-07-19, sha256 `0bdde81a…`, 1229 lines). Cross-repo FULL ritual:
qsl-desktop PR #4 (branch `na0661-gui-slice-a-design-round2`, head
`415b357`, 9 files, +592/−296 counting modified files plus three new
files, carrying repo-local D-0004) + this spine governance closeout
(D-1284). Base spine `a5af1b49` (the #1604 seating merge; qwork-proven;
9/9 push runs SUCCESS including formal-ci and qsc-adversarial —
re-verified green before the desktop PR opened). Base qsl-desktop
`4f2a3d87` EXACTLY (the PR #3 merge; `rust` SUCCESS; protection
["rust"]/strict/enforce_admins verified at Phase 0 and after PR
creation).

## 0. The design authority, landed

- `docs/DESIGN_SPEC.md` = directive Appendix C, byte-exact (cmp against
  the directive-extracted copy), sha256 `34ced51b…`, 113 lines.
- `docs/DESIGN_SPEC_AppendixD.md` = directive Appendix D, byte-exact
  (cmp against BOTH the directive-extracted copy AND the operator's
  source file `~/work/QSC_DESIGN_SPEC_AppendixD_reference_markup.md`),
  sha256 `a7d45a0a…`, 222 lines.
- Landing shape per the resolved F1: a SEPARATE appendix file, so each
  cmp proof stays intact. Both files are BINDING on future GUI lanes
  until revised by directive.

## 1. The token migration (spec §1 governs)

Old → new (`ui/style.css` `:root`): page `#14161a` → `#1D1D1F`; card
`#1c1f25` → `#252528`; field `#101216` → `#1A1A1C`; hairline `#2a2e36` →
`#3A3A3E`; text `#d7dae0` → `#E8E8E8` + secondary `#A8A8A8` (new role) +
muted `#8a919d` → `#7A7A7A`; the accent becomes the §1 role trio
(`#1C2A3E`/`#2E5A8E`/`#8FBAF0`) + primary fill `#3D7BC4` (old single
accent `#4f8cc9` gone); the danger family becomes the §1 trio
(`#3A1D1D`/`#8A3A3A`/`#F0A0A0`; old filled `#b0524d` family gone);
success text `#8FDCA8`; neutral-status `#2A2A2E`; title 21px→17px/600;
body 14px→13px; radius 10/6 → 12/8. Amber stays (R2: the attempts alert
remains amber; red RESERVED for the armed-erasure state). The D596
discipline greps stay green: every font-size/padding/margin/gap outside
`:root` references a token; zero color literals outside `:root`; every
button exactly one tier or nav role.

Type-scale rule applied and recorded (the §D.8 deviation ledger, item by
item, below): where §1 names a role, §1 wins (h1/h2 page-titles land
17px/600 though D.x annotations say /500; section-heads 14/600); where a
D.x annotation names a size absent from §1, the D value lands as a named
token (`--fs-welcome` 15px, `--fs-listhead` 16px, `--fs-micro` 11px for
the steps line + status bar, `--fs-glyph` 19px rail icons). Spec §2's own
banner CSS (padding 10px 14px) and the D.x-annotated component paddings
land verbatim via named `--sp-x*` tokens (6/7/10/13/14/20/22 px) so the
spacing grep stays mechanical — recorded as the spec-internal tension
between §1's scale sentence and §2's component CSS; §2 governs its
component.

## 2. Items 1–15 as landed (per-item pointer + demonstration)

1. Confirm directly below Passphrase — `ui/index.html` field-stack (two
   labels, nothing between; test-pinned). SHOT: `accept_shot_s0.png`
   beside spec §3/D.6. MATCH.
2. Strength meter REMOVED (div, estimator, CSS). SHOT: s0 (absent);
   test-pinned absent. MATCH.
3. Checklist exactly two checks; COMMON_PASSWORDS array + markers + gate
   logic + its design_system.rs soundness test REMOVED (the one
   sanctioned amendment). SHOT: s0 shows the two rows at rest;
   going-green is input-driven → OPERATOR EYES; the gate logic is
   two-condition only (source-pinned). MATCH at rest.
4. "Your identity" heading + step label — D.7. Input-locked screen →
   OPERATOR EYES; source + test pin.
5. One-line verification code — `.verify-code` nowrap + `fitCode()`
   shrink-to-fit (17px mono, floor 11px), shared by `#identity-code` and
   `#settings-code`. Input-locked surfaces → OPERATOR EYES; source +
   test pin.
6. Ceremony one-line instruction on destroy AND erase (spec §5): the
   instruction is its own `<p class="instruction">` line; the erase
   screen lost its extra hint paragraph and keeps NO passphrase field
   (semantics unchanged); the destroy collapsed state already matched.
   The D596 Appendix-A verbatim needles all still pass. Erase screen is
   click-reachable only → OPERATOR EYES for the live screen; the S1/S2
   shots show the unlock gate it hangs off. Source + test pin.
7. Autolock helper verbatim, no number restated — test-pinned string.
   OPERATOR EYES for the pane.
8. Arm = destructive tier / Disarm = secondary — markup + tier test.
   OPERATOR EYES for the pane.
9. Duplicated guest-warning paragraph DELETED — test-pinned absent; the
   checkbox line carries the warning (period dropped per D.5 copy).
10. TRUE disabled tier — `button:disabled` = neutral bg + muted text, no
    border emphasis, no opacity dim; placed LAST in the button cascade
    (the first s0 screenshot CAUGHT a real cascade bug — equal
    specificity let `.primary` win over `:disabled`, rendering the
    disabled Create accent-filled; fixed by rule order; the §7
    screenshot standard did exactly its job). SHOT: s0 +
    s0_cli_notice show the disabled tier. MATCH.
11. Rail identity dot REMOVED — rail = Chats, Contacts, gear (D.2);
    Settings still lands on Identity via the gear (R3 recorded the
    D596-F1 supersession). SHOT: main window is input-locked → the S0
    shots prove the shell; test pins absence. OPERATOR EYES for the
    main rail.
12. Status-banner component (§2 verbatim structure/CSS/copy via tokens):
    `#wipe-state` (danger "Armed — erases after {N} failed attempts" /
    neutral "Off — wrong attempts never erase the vault") and
    `#autolock-status` (accent "Locks after {N} minutes of inactivity"),
    icons inline-SVG outline ~17px (warning triangle / shield-check /
    lock), swapped as ONE component by `setBanner()`; `acknowledge()`
    flashes the control while the banner is the durable truth. Amber
    alert unchanged (R2). Input-locked pane → OPERATOR EYES; source +
    test pin. Pluralization kept for the 1-attempt/1-minute edge
    (recorded as within-reason copy morphology on the §2 templates).
13. STATE-HYGIENE FIX (the defect the operator reported, verified at
    drafting by read-only trace): destroy/erase completion now performs
    `window.location.reload()` (F2 default — provable by construction:
    all durable state is backend-side); independent of the reload, EVERY
    screen transition runs `clearCeremonyState()` (clears vault-pass,
    vault-confirm, unlock-pass, erase-phrase, destroy-pass,
    destroy-phrase; collapses the destroy flow), pane navigation runs
    `resetDestroyFlow()`, cancel collapses+clears, open clears, and the
    wizard identity step NEVER pre-fills a prior alias (the main.js:207
    pre-fill is gone; the field starts empty). The in-memory survivors
    (self_alias, vaultAlertCount, vaultAlertDismissed,
    observedFailedUnlocks, unlockNext) die with the document on reload.
    PROOF: source pins (tests/design_round2.rs
    `item13_state_reset_pinned`) + the repro script for the operator
    (below). HONEST RESIDUE — FILED as ENG-0048: `destroy_vault` leaves
    `settings.json` (autolock minutes + alias, both non-secret) on disk
    by landed D-0002 semantics; `erase_all` removes it. Changing what
    destroy removes was out of this lane's scope (the "NO change to what
    destroy/erase DO" boundary); whether destroy should also clear app
    settings is the operator's semantics decision.
14. FULL-BLEED SHELL per D.1–D.3: grid `52px | 210px | 1fr` (main) and
    `52px | 160px | 1fr` (Settings), 1px `#3A3A3E` hairlines only, no
    outer padding/inset, status bar the full-width last row (6px 14px,
    micro type); SETTINGS IS A VIEW — the icon rail is PRESENT and LIVE
    in the Settings screen (gear active; Chats returns to main;
    Contacts returns + shows the honest stub note); rail items 34×34
    radius 8, icons 19px inline SVG (message/users/gear per D.2); the
    list pane gains the D.2 list-header (Chats 16/500 + the
    NON-INTERACTIVE search affordance, landed as the mockup shows,
    claiming nothing) and "No conversations yet."; the content pane
    becomes the D.2 welcome block (shield-lock 38px accent-fill,
    "Welcome to QuantumShield Chat" 15/500, sub-line, primary CTA); the
    wizard card (max-width 440, 22/24 padding) stays the ONE centered
    exception. SHOTS: s0/s0_cli_notice/s1/s2 show the full-bleed canvas
    + centered cards; the main window/Settings shells are input-locked →
    OPERATOR EYES beside D.2/D.3; structure test-pinned.
15. NATIVE MENU (pinned tauri 2 core menu API only — ZERO
    crate/feature/lockfile motion, proven by the untouched
    Cargo.toml/Cargo.lock): File (Settings, Lock now, — , Quit
    [predefined]), Edit (Cut, Copy, Paste, Select all [all
    predefined/native]), View (Reload [webview reload — the item-13
    mechanism], Full screen [toggle]), Help (About [predefined native,
    factual metadata: display name + version + the honesty line]).
    WORKING ENTRIES ONLY; R1: Settings/Lock now start disabled and are
    enabled exactly while an unlocked surface shows, driven by the
    frontend's surface reports through the new app-layer
    `ui_surface_changed` command (zero qsc symbols, zero markers); menu
    events reach the frontend as `menu-open-settings`/`menu-lock-now`
    (guarded again frontend-side). SHOTS: the menubar (File Edit View
    Help) renders in-window and is visible in ALL FOUR screenshots.
    Entry-by-entry activation needs input → OPERATOR EYES at flight;
    the tree is source-pinned (`menu_tree_pinned`).

## 3. The item-13 repro for the operator (exact shape)

1. Create a vault; complete the wizard; open Settings → Vault and
   Security; click "Destroy vault…"; type the passphrase and
   `destroy my vault`; click [Destroy permanently].
2. EXPECT: the app reloads into the S0 wizard (no flash of stale UI).
3. Re-onboard (new vault + identity, leave the alias empty), open
   Settings → Vault and Security.
4. EXPECT: the destroy ceremony is COLLAPSED and EMPTY; the wizard's
   alias field was EMPTY at step 2 of onboarding; no failed-attempts
   alert appears unless earned in THIS session.
5. Erase variant: from the unlock screen → "Forgot your passphrase?" →
   type `erase everything` → [Erase everything]; EXPECT the reload into
   S0; after re-onboarding the erase screen (via Forgot) has an EMPTY
   phrase field; the alias is empty everywhere (erase removes
   settings.json).
6. Also per §5: begin typing in the destroy form, navigate to another
   pane (or lock via File > Lock now) and return — EXPECT collapsed and
   empty every time.

## 4. Proofs and gates

- Suite 40 passed / 0 failed / 1 ignored (the seed hook): lib 5 +
  design_round2 17 (NEW) + design_system 6 (7 − the one sanctioned
  removal) + slice_a_flows 7(+1 ignored) + slice_a_rules 5.
- slice_a_flows.rs sha256 `be700518…` and slice_a_rules.rs `a53137ec…`
  BYTE-IDENTICAL to base (sha-compared) — the interruption matrix,
  unlock lifecycle, and deferred path re-proven unbroken.
- fmt --check CLEAN; clippy --workspace -D warnings CLEAN; metadata
  --locked OK; cargo audit EXIT 0 on the UNCHANGED lock (the same 17
  known gtk3-bindings warnings; zero vulnerabilities).
- Scope: 9 files = exactly the D597 allowed paths (ui ×3, lib.rs,
  design_system.rs [the one removal], design_round2.rs NEW, docs ×2 NEW,
  DECISIONS.md). Cargo.toml/Cargo.lock/tauri.conf.json/workflows/
  capabilities/community files ABSENT from the diff. qsc-symbol set at
  head == base (comm −13 = 0 new). No new marker strings. git diff
  --check clean.
- Publication scan (added lines + 3 new files): class PASS, zero
  overclaims, zero pattern hits.
- The zero-networking scan (slice-A suite) green.

## 5. Headless-visual record (the §7/§D.8 split, honest)

- SHOT (in this proof root `shots/`): `accept_shot_s0.png` (items 1, 2,
  3-at-rest, 10, 14-canvas, 15-menubar ↔ §3/D.6),
  `accept_shot_s0_cli_notice.png` (the courtesy notice in the migrated
  tokens), `accept_shot_s1.png`/`accept_shot_s2.png` (the unlock gate;
  pixel-identical pre-input by design), each with `*_windows.txt`
  xwininfo capture asserting the "QuantumShield Chat" title.
- OPERATOR EYES (no input driver on the host; unchanged precedent): the
  checklist going green + Create enabling, the "Your identity" card +
  one-line code, the Settings three-column view (D.3), the Identity
  pane (D.4), the Vault and Security pane + banners (D.5), the destroy
  expanded form, the item-13 repro (§3 above), the erase screen live,
  per-entry menu activation, tooltips/active states. Every behavior
  behind them is source-pinned/test-proven above.
- ENVIRONMENT FINDINGS (recorded for every future GUI lane; the rig
  regressed mid-day and was fully diagnosed):
  a. The seat0 desktop session now exposes `wayland-0` in
     XDG_RUNTIME_DIR; GDK inside Xvfb then prefers Wayland (libwayland
     falls back to `wayland-0` even with WAYLAND_DISPLAY unset) and the
     window opens on the REAL compositor, not in Xvfb. FIX (mandatory
     rig env): `GDK_BACKEND=x11`. Diagnostic runs that landed on the
     real desktop were killed immediately; nothing persisted.
  b. WebKitGTK first paint under Xvfb needs the software knobs
     (`WEBKIT_DISABLE_COMPOSITING_MODE=1`,
     `WEBKIT_DISABLE_DMABUF_RENDERER=1`, `LIBGL_ALWAYS_SOFTWARE=1`) and
     ~20 s settle; an `xwininfo -name` + `xwd -name` roundtrip before
     `scrot` makes the framebuffer capture deterministic.
  c. `scrot` NEVER overwrites an existing file — it silently writes
     `_000`-suffixed siblings; stat the exact output you asked for
     (this masked healthy captures during diagnosis).
  d. A stale `xdg-document-portal` FUSE mount (`/run/user/1000/doc`,
     epoch mtime) was found broken and unmounted (`fusermount3 -u`)
     during diagnosis; not the root cause but a real host-state repair,
     recorded.
- The virtual-display caveat stands; physical-display behavior is NOT
  claimed. The screenshot rig for future lanes is captured verbatim in
  the testplan.

## 6. §D.8 deviations (each with its reason)

1. About stays FUNCTIONAL (not muted-unbuilt as D.3 draws it) — the
   pre-dispositioned ruling; muting a working pane would be dishonest.
2. The D.2 search affordance lands NON-INTERACTIVE exactly as drawn —
   mockup-bound; it claims nothing and does nothing.
3. Wizard-card background = §1 card `#252528`, not D.6's annotated
   `#1D1D1F` — the appendix's own precedence sentence (layout from D,
   values from §1), pre-dispositioned.
4. Page-title/section-head weights land §1's 600 where D.x annotations
   say 500 (h1/h2/h3) — §1 governs values.
5. D-only sizes (welcome 15, listhead 16, steps/status-bar 11, rail
   glyph 19) land as named tokens — §1 is silent on those roles; the
   mockup is the operator-approved look.
6. §2 banner copy templates keep singular/plural morphology
   ("1 failed attempt", "1 minute") — within-reason copy quality.
7. Component paddings from §2/D.x (10×14 banner, 6×14 status bar, 7×14
   nav cells, 13 code-box, 22×24 wizard card, 6×10 search, 12/14/10
   list header) land verbatim via named `--sp-x*` tokens — the spec's
   own component CSS overrides its scale sentence for its components.

## 7. Not claimed

Any security property improved; server connectivity (slice B, OWED);
physical-display behavior; menu behavior beyond the wired entries. The
claim boundary is UNCHANGED. External review remains THE release gate.
