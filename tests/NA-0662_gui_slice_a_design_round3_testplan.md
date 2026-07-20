# NA-0662 testplan — GUI slice-A design pass ROUND 3 (D598; D-1285 + D-0005)

Scope: qsl-desktop presentation + window-sizing behavior + the TWO
sanctioned deltas (autolock 60/0-never; the 30-second erase countdown
gate); the spine carries governance records only. Everything else
destroy/erase/unlock/wipe DO is byte-for-byte the NA-0661 behavior
(commands.rs untouched; the four existing test files byte-frozen).

## 1. Automated (all green at the stop)

- `cargo test -q` (workspace, qsl-desktop): 56 passed / 0 failed /
  1 ignored (the seed hook). Suites: lib unit 5; design_round2 17 and
  design_system 6 — BYTE-UNMODIFIED and green (the drafting collision
  check held: the quoted phrases land as CSS content so the frozen
  markup needles survive; the ceremony head rides the existing h3);
  slice_a_flows 7(+1) and slice_a_rules 5 — BYTE-IDENTICAL to base
  (sha `be700518…` / `a53137ec…`); tests/design_round3.rs 16 NEW
  (additive): no native number inputs + spinner CSS + 64px; the 0–1440 /
  1–100 ranges + the visible-validation source pins (validateNum,
  .invalid danger border, both messages); default-60 + zero-valid via
  the settings API + the `autolock_minimum_one_minute` string ABSENT;
  the never-fire guard BEFORE the elapsed comparison (index-ordered) +
  both 60 mirrors; the banner state machine (danger at 0, accent above,
  the E.3 copy verbatim); the quoted-phrase CSS (content '"', danger
  text) + the byte-frozen markup needles; the ceremony card on BOTH
  surfaces + full-width fields + no label-wrap + the E.4 token DNA; the
  16px one-line arm checkbox; helper-directly-under-banner (order
  asserts); the 420px settings code box; --fs-glyph 21 + rail svg 21 +
  status-line 12px secondary; the mode map (7 surfaces → 3 modes) + the
  E.1 size table via the pub API + the F1 conf pins (visible:false,
  560x660) + the attachment-mechanism needles; the compact
  card-fills-window CSS; "Delete vault?"/link-danger/#C87A7A token + no
  reserved feedback gap + the old wording ×0; the countdown panel
  markup + verbatim copy + EXACTLY ONE erase_all invoke inside the
  interval after the zero gate + abort-on-every-transition; the
  three-file authority (Appendix E present with the E.1 table; zero
  surviving contradiction needles; [E.x] citations present).
- `cargo fmt --all -- --check` CLEAN; `cargo clippy -q --workspace --
  -D warnings` CLEAN; `cargo metadata --locked` OK; `cargo audit` exit 0
  (unchanged lock; 17 known gtk3 warnings).
- Scope proofs: the diff = exactly the D598 allowed paths (docs ×3,
  ui ×3, lib.rs, settings.rs, tauri.conf.json windows[0]-only,
  design_round3.rs NEW, DECISIONS.md); forbidden paths ABSENT
  (Cargo.toml ×2, Cargo.lock, workflows, capabilities, commands.rs,
  community files); qsc-symbol set head == base (23/23); no new marker
  strings; zero-networking scan green (roots src/+ui/ re-verified);
  publication scan class pass, zero overclaims; `git diff --check`
  clean.
- Byte-exact spec landing: cmp(docs/DESIGN_SPEC_AppendixE.md, directive
  Appendix E extract) OK + cmp vs the operator's source OK; sha256
  `5175f3bc…`, 128 l. Amended: DESIGN_SPEC.md `34ced51b…`→`074244be…`
  (143 l); AppendixD `a7d45a0a…`→`5f5d3a2e…` (244 l).

## 2. Headless-visual (the NA-0661 rig recipe, plus round-3 notes)

    xvfb-run -a -s "-screen 0 2560x1600x24" dbus-run-session -- bash -c '
      GDK_BACKEND=x11 \
      WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 \
      LIBGL_ALWAYS_SOFTWARE=1 \
      QSLD_DATA_DIR=<state-dir> XDG_CONFIG_HOME=<cfg-dir> <binary> &
      sleep 25
      xwininfo -name "QuantumShield Chat" > <name>_xwininfo.txt
      xwd -name "QuantumShield Chat" -out /dev/null   # framebuffer sync
      scrot <fresh-filename>.png
      xwininfo -root -tree > <name>_windows.txt'

  ROUND-3 RIG NOTES (additive to the NA-0661 recipe, which stands):
  a. The Xvfb screen is 2560x1600 now — the WM-less server parks the
     window ORIGIN at screen center, so a 1280x800 screen cuts compact
     windows off; crop review copies to the xwininfo rect (PIL).
  b. Window-size captures reflect the REQUESTED size — assert geometry
     via xwininfo per mode: wizard 560x660, unlock/erase 460x420. The
     560 width doubles as the runtime-sizing proof (conf minWidth 800
     would clamp a config-sized window).
  c. Centering is NOT verifiable under the WM-less rig (operator eyes).
  d. tao set_visible(true) == gtk show_all(): NEVER prove menu-hiding
     with hide_menu() under a deferred show — the landed mechanism is
     menu-by-ATTACHMENT (remove_menu/set_menu), which the captures
     verify directly (no menubar in any compact shot).
  States: s0 = empty data dir + empty XDG_CONFIG_HOME; s0+notice = cfg
  dir containing `qsc/vault.qsv`; s1/s2 = the slice_a_flows
  `seed_acceptance_dir` ignored hook (QSLD_SEED_DIR +
  QSLD_SEED_MODE=vault|vault_identity). Four shots + xwininfo geometry
  asserts landed in the proof root
  (/srv/qbuild/tmp/NA0662_gui_slice_a_design_round3_20260720T033911Z/shots/).

## 3. Operator eyes (input-driven; no input driver on the host)

Beside their E.x blocks per §E.8: the V&S pane (E.2 fields + the
invalid-entry visible rejection; E.3 banner BOTH states — save 0 and see
the danger banner, save 60 and see the accent banner; E.4 destroy
ceremony card + full-width fields + quoted phrase; E.7
checkbox/helper/code-box/legibility); the erase ceremony card + the
quoted phrase (E.4); THE COUNTDOWN (E.5): confirm → the form is
REPLACED, 30→0 with both number and label updating → let it complete
once on a THROWAWAY vault (the erase fires only at zero, then reloads
to S0) AND abort once via Cancel (form restored empty, nothing erased)
AND once by closing the window (nothing erased on relaunch); the unlock
screen (E.6 danger link opens Erase directly; error inline only when
present); the window modes (E.1): wizard/unlock/erase compact + NO menu
bar + the card fills; unlock → main RESIZES to 1024x700 WITH the menu
bar present (the resize-on-entry proof) and Settings keeps it;
Ctrl+C/Ctrl+V and the right-click context menu still work in compact
fields (paste without a menu bar); autolock at 0 NEVER locks (leave the
main window idle past a saved small interval first to see it lock, then
save 0 and verify it does not).

## 4. Merge choreography (operator)

qsl-desktop PR #5 FIRST (merge commit; `rust` green), then the spine
closeout PR. Then the operator FLIES THE BUILD (the merged-first review
rule; §3 above is the flight checklist).
