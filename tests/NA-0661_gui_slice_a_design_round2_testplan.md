# NA-0661 testplan — GUI slice-A design pass ROUND 2 (D597; D-1284 + D-0004)

Scope: qsl-desktop presentation/copy + the item-13 webview state reset +
the item-14 full-bleed shell + the item-15 native menu; the spine carries
governance records only. What destroy/erase/unlock/wipe/autolock DO is
byte-for-byte the NA-0660 behavior.

## 1. Automated (all green at the stop)

- `cargo test -q` (workspace, qsl-desktop): 40 passed / 0 failed /
  1 ignored (the seed hook). Suites: lib unit 5; tests/design_round2.rs
  17 (NEW round-2 pins: two-check checklist; meter/list/dot absent;
  confirm-below-passphrase; "Your identity"; one-line code (nowrap +
  fitCode); ceremony one-line instruction both flows + erase prose
  deleted; autolock helper verbatim; Arm/Disarm tiers; guest-warning
  deleted; true disabled tier without opacity; status-banner component +
  §2 copy + role tokens; item-13 reset calls (reload ×2 +
  clearCeremonyState in show() + resetDestroyFlow + no alias pre-fill);
  full-bleed grids + the Settings icon rail + no Back pseudo-entry;
  migrated token values + old values absent; the menu tree + R1 gating;
  the two spec files present); tests/design_system.rs 6 (all D596
  disciplines retained; EXACTLY one removal — common_passwords_list_is_
  sound, its subject deleted by item 3); slice_a_flows 7(+1 ignored) and
  slice_a_rules 5 — both files BYTE-IDENTICAL to base
  (sha `be700518…` / `a53137ec…`).
- `cargo fmt --all -- --check` CLEAN; `cargo clippy -q --workspace --
  -D warnings` CLEAN; `cargo metadata --locked` OK; `cargo audit` exit 0
  (unchanged lock; 17 known gtk3 warnings).
- Scope proofs: forbidden paths absent from the diff (Cargo.toml ×2,
  Cargo.lock, tauri.conf.json, .github/workflows, capabilities, icons,
  community files); qsc-symbol set head == base; no new marker strings;
  zero-networking scan green; publication scan class pass, zero
  overclaims; `git diff --check` clean.
- Byte-exact spec landing: cmp(docs/DESIGN_SPEC.md, directive Appendix C
  extract) OK, sha256 `34ced51b…`; cmp(docs/DESIGN_SPEC_AppendixD.md,
  Appendix D extract) OK AND cmp against the operator's source file OK,
  sha256 `a7d45a0a…`.

## 2. Headless-visual (the working rig, recorded for all future GUI lanes)

    xvfb-run -a -s "-screen 0 1280x800x24" dbus-run-session -- bash -c '
      GDK_BACKEND=x11 \
      WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 \
      LIBGL_ALWAYS_SOFTWARE=1 \
      QSLD_DATA_DIR=<state-dir> XDG_CONFIG_HOME=<cfg-dir> <binary> &
      sleep 20
      xwininfo -name "QuantumShield Chat" >/dev/null
      xwd -name "QuantumShield Chat" -out /dev/null   # framebuffer sync
      scrot <fresh-filename>.png                       # scrot NEVER overwrites
      xwininfo -root -tree > <name>_windows.txt'

  MANDATORY: `GDK_BACKEND=x11` (the seat0 session now exposes
  `wayland-0`; without the pin the window opens on the REAL compositor);
  the WEBKIT/LIBGL software knobs + ~20 s settle; the xwd roundtrip
  before scrot; a FRESH scrot filename every run. States: s0 = empty
  data dir + empty XDG_CONFIG_HOME; s0+notice = cfg dir containing
  `qsc/vault.qsv`; s1/s2 = the slice_a_flows `seed_acceptance_dir`
  ignored hook (QSLD_SEED_DIR + QSLD_SEED_MODE=vault|vault_identity).
  Four shots + xwininfo title asserts landed in the proof root.

## 3. Operator eyes (input-driven; no input driver on the host)

Beside their D.x blocks per §D.8: the Settings three-column view (D.3),
Identity pane (D.4), Vault and Security + banners (D.5), the wizard
step-2 card (D.7), the destroy expanded form and live erase screen
(§5), checklist going green + Create enabling (§3), tooltips/active
states, and per-entry menu activation (item 15; R1 state gating:
Settings/Lock now disabled until unlocked). THE ITEM-13 REPRO (exact
shape, from the as-built §3): destroy → reload-to-S0 → re-onboard →
Vault and Security shows the ceremony COLLAPSED and EMPTY; the erase
variant; the mid-typing pane-switch/lock variants; the wizard alias
field empty on a fresh identity.

## 4. Merge choreography (operator)

qsl-desktop PR #4 FIRST (merge commit; `rust` green), then the spine
closeout PR. Then the operator FLIES THE BUILD before round-3 findings
(the merged-first review rule).
