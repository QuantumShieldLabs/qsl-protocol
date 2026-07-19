# NA-0660 as-built — GUI slice-A design pass (D596; spine D-1283; qsl-desktop D-0003)

Result class: GUI_SLICE_A_DESIGN_PASS (both PRs open at the stop: qsl-desktop
#3 head `713f58a`; this spine closeout). Base spine `499be55b` (the #1601
seating merge, qwork-proven: startup_result=OK, head==origin/main==main,
clean, ready_count=1, queue_top_ready=NA-0660, shared_target_ready=yes).
qsl-desktop base `cc906e54` EXACTLY (the PR #2 merge; fresh clone in the
proof root; protection `rust` strict enforce_admins verified binding at
Phase 0 and at the stop). Directive executed AS AMENDED AT APPROVAL
(sha256 `508ac660…`, 711 lines; F1/F2 to the drafted defaults; the three
encoded rules binding). Proof root:
`/srv/qbuild/tmp/NA0660_gui_slice_a_design_20260719T180356Z/`.

## §0 Phase 0 + the main-health deviation (recorded honestly)

Sequencing precondition verified PAID: desktop main == `cc906e54` (contains
9a1f40c), then spine `35948ec0` (D-1282 canonical ×1; ENG-0047 filed).
Disk 52%; /backup/qsl mounted. D-1283 next-and-absent (DECISIONS ×0,
TRACEABILITY ×0; the NEXT_ACTIONS references are the promotion block's own
forward-naming). DEVIATION: GitHub spawned ZERO workflow runs for the
seating merge `499be55b` (API total_count=0 throughout the lane — the
NA-0645-class missed-push-run incident). The substantive tree equals
`35948ec0` (a NEXT_ACTIONS.md-only merge atop it) whose 9/9 push runs
concluded SUCCESS including formal-ci (watched to conclusion). The operator
directed proceed at Phase 0 ("Report Phase 0, then proceed"). The remedy —
the NA-0237D-shape sanctioned bootstrap PR — stays OWED TO THE OPERATOR'S
SANCTION and was not executor-initiated.

## §1 The design system (items 1–5), landed FIRST

ONE `:root` token layer in ui/style.css:
- Type scale: `--fs-hero 26 / --fs-title 21 / --fs-glyph 18 / --fs-section
  15 / --fs-body 14 / --fs-hint 12` + `--fw-title 650 / --fw-section 600`.
  Every `font-size:` outside the token block references `var(--fs-…)` —
  test-enforced (`font_sizes_only_from_scale_tokens`).
- Spacing scale: `--sp-1..--sp-6 = 4/8/12/16/24/32`. Every
  padding/margin/gap outside the block is `var(--sp-…)`, `0`, or `auto` —
  test-enforced (`spacing_only_from_scale_tokens`). Widths/heights are
  dimensions, not rhythm, and stay literal (recorded reading).
- Accent discipline: ONE `--accent` carries primary actions, the
  `:focus-visible` ring on every focusable control, and the active rail
  pill/bar. `--danger/-strong/-fg` is the separate destructive family and
  is never the accent. Greens/ambers (`--ok`, `--amber`) are STATUS colors
  (checklist-satisfied, the saved flash, the failed-attempts alert), never
  emphasis; the old green `.pq` emphasis line was RECOLORED to body muted —
  no third emphasis color exists. ALL color literals (hex + rgba) confined
  to the token block — test-enforced (`colors_only_in_token_block`).
- Button tiers: `.primary` (filled accent; at most one per surface by
  design), `.secondary` (outline), `.danger` (filled red family,
  unmistakable). Every `<button>` carries exactly one tier OR one of the
  two named nav roles (`rail-btn`, `cat`) — test-enforced
  (`every_button_is_tiered_or_nav`; the nav-role reading recorded: rail and
  settings-rail buttons are navigation, covered by the item-10 active-state
  rules, not action tiers).
- The NO-SILENT-STATE-CHANGES rule: ONE shared `acknowledge(btn, flash,
  statusEl, statusText)` helper (ui/main.js) — swaps the control's label to
  a "✓ Saved"-style flash with the `.acked` status look for 1.4 s AND
  updates the section's persistent status line to the new reality. Wired:
  autolock save ("✓ Saved" + "Locks after N minutes of inactivity."), wipe
  arm ("✓ Armed" + the armed status line), wipe disarm ("✓ Off" + the off
  status line), alias save ("✓ Saved" + "Shown as: X (local only)").
  Microcopy factual ("Saved", never "Protected"). The wizard's alias entry
  saves as part of Done (advancing IS the step's acknowledgment; the
  editable control under the rule lives in Settings — recorded reading).

## §2 The screen work (items 6–13)

6. `tauri.conf.json windows[0].title = "QuantumShield Chat"`; the About
   first line composes `QuantumShield Chat (qsl-desktop <version>)` from
   the new `APP_DISPLAY_NAME` const via the `display_name` AppInfoDto
   field; the static `<title>` matches. Identifier
   `org.quantumshieldlabs.qsldesktop`, productName, and the `qsl-desktop`
   binary are UNCHANGED (test-pinned; the identifier anchors the app data
   dir — changing it orphans vaults, a D596 STOP condition). Asserted live:
   xwininfo shows `"QuantumShield Chat"` with WM_CLASS `qsl-desktop`.
7. Passphrase step: fields STACKED in a tight `--sp-2` stack (meter under
   the passphrase field, retained, its hex colors moved to token classes
   lvl0–4); the checklist directly under confirm — ① 12+ characters
   ② Passphrases match ③ Not a commonly-used password — items flip to
   green `✓` as satisfied; Create is DISABLED until all three hold (a UI
   gate; the core vault_create contract untouched — its empty/mismatch
   errors remain as defense in depth). The list: 149 lowercase, sorted,
   unique entries in ui/main.js between BEGIN/END markers
   (test-enforced: `common_passwords_list_is_sound`; screened against the
   zero-networking scan tokens; membership checked case-insensitively).
   NO uppercase/symbol theater. The VERBATIM line and the no-recovery warn
   box retained (`appendix_a_copy_verbatim`).
8. "This is you": the verification code is the HERO (`--fs-hero`, centered,
   bordered, selectable); ownership + private-part lines lead; the purpose
   line is the approved codes-catch-man-in-the-middle wording
   (`VERIFY_PURPOSE_LINE`); the PQ line is the plain-English
   "Designed to stay secure even against future quantum computers."
   (`PQ_LINE`; the single-character sentence-case capitalization of the
   approved phrase is recorded as typographic judgment); the ML-KEM-768 +
   ML-DSA-65 naming moved to `MECHANISM_LINE` behind the "Show technical
   details" disclosure together with the full fingerprint; the optional
   self-alias field ("What should this device call you?", maxlength 32,
   hint "Local only — leave empty to be shown as 'You'."); the VERBATIM
   reassurance line closes the card.
9. The Identity pane (F1): FIRST in the Settings rail; shows the editable
   alias (F2; saves under the rule), the hero verification code, the
   purpose + PQ lines, and the same technical-details disclosure — fed
   EXCLUSIVELY by the existing `identity_show` + `settings_get`
   (17 commands unchanged; the qsc-symbol set at head EQUALS base 23/23 —
   zero new core calls). The rail identity dot (circular, the alias's
   first character upper-cased, "Y" for the empty "You" default) sits
   above the gear; both the dot and the gear open Settings landing on
   Identity (the first category). Defensive honest empty state if identity
   is absent.
10. Rail: ONE CSS tooltip pattern (`.tip[data-tip]:hover::after`) on all
    four main-rail entries (Chats / Contacts — coming later / Identity /
    Settings); active state = `--accent-soft` pill + 3px accent bar on the
    main rail, `--accent-soft` + inset accent bar on the Settings rail.
11. Vault & Security: controls FIRST, one-line explanations UNDER, prose
    trimmed. The "Failed unlock attempts: N — vault locked/unlocked"
    narration is DELETED (silent at zero). The amber alert renders ONLY
    when the captured value > 0: "N failed unlock attempts since your last
    unlock" + Dismiss (app-local session acknowledgment; zero core
    mutation). THE CAPTURE RULE (binding, from the re-verified
    reset-on-success fact): `showUnlockScreen()` reads
    `protection_status()` at unlock-screen ENTRY into
    `observedFailedUnlocks`; every typed Rejected/Delayed outcome updates
    it; the successful unlock promotes it to `vaultAlertCount` — a
    post-unlock `protection_status()` read is never the alert's source.
    Wipe: limit control first in its row, Arm/Disarm beside it, the ack
    checkbox + consequence on ONE line, the persistent status line
    ("Off — wrong attempts never erase the vault." / "Armed — erases after
    N failed attempts."). Autolock: control + Save first, status line,
    hint under. Same commands, same 1..=100 bounds, same semantics.
12. Destroy/erase, one pattern: heading VERBATIM "Destroy vault" (no
    "Danger zone" — absence test-pinned); prose VERBATIM "Requires your
    passphrase. Permanently erases this vault — this cannot be undone.";
    expanded form passphrase → the ONE-line instruction (Type "destroy my
    vault" to confirm) above the input → [Destroy permanently] [Cancel].
    The erase screen inherits the IDENTICAL form: heading "Erase
    everything", ONE-sentence prose, the CLI-untouched + no-passphrase
    facts as a hint line, the same instruction/input/button-pair shape
    with its own phrase and NO passphrase field (it IS the
    forgotten-passphrase path). Both flows keep their EXACT NA-0659
    semantics (typed phrases, two-step deliberateness, app-level-only
    erase with the CLI-dir guard, the tokened destroy).
13. Empty state: inline SVG chat-bubble glyph (stroke currentColor; no
    binary asset, icons/ untouched), "Your conversations will live here.",
    the one primary CTA (still the honest not-yet-functional stub with its
    note), "no server configured" byte-untouched.

## §3 Storage + commands (app layer only)

`AppSettings` gains `self_alias: String` with
`#[serde(default, skip_serializing_if = "String::is_empty")]`: an empty
alias is OMITTED from settings.json, so a FRESH profile serializes exactly
the slice-A key set and the byte-frozen flows assertion
(`keys == ["autolock_minutes"]`) HOLDS unmodified; the key appears exactly
once when set (allowlist test extended to prove both facts + an
absent-key-compat test). NON-SECRET ruling recorded: a local display
label; trimmed backend-side; UI maxlength 32. `settings_set` widened to
carry both fields (no new command; count stays 17). `IdentityDto` gains
`mechanism_line`; `AppInfoDto` gains `display_name` — DTO shape only,
zero new qsc calls.

## §4 Proofs

- Frozen re-proof: `slice_a_flows.rs` / `slice_a_rules.rs` BYTE-IDENTICAL
  to base (absent from the diff; head shas `be700518…` / `a53137ec…`) and
  green — the interruption matrix, unlock lifecycle (clock-injected), and
  deferred path re-proven unbroken at the backend level.
- Suite: 24 passed / 0 failed / 1 ignored (the seed hook) = the 16-test
  baseline + EXACTLY 8 additive (7 in tests/design_system.rs + 1 settings
  compat). fmt --check CLEAN; clippy --workspace -D warnings CLEAN;
  cargo metadata --locked OK; git diff --check clean.
- cargo audit EXIT 0 on the UNCHANGED lock (Cargo.lock absent from the
  diff): the same 17 known RUSTSEC gtk3-bindings/proc-macro-error
  unmaintained warnings; zero vulnerabilities.
- Scope: 8 files = exactly the D596 allowed paths (commands.rs,
  settings.rs, tauri.conf.json title line, ui ×3, tests/design_system.rs,
  DECISIONS.md); Cargo.toml/Cargo.lock/ci.yml/.gitignore/build.rs/
  capabilities/icons/README/community-health/CLAUDE.md ALL absent (0
  forbidden paths, grep-proven); the qsc pin unchanged at `81143dcd…`.
- Zero networking: the frozen source-scan test green over src/ + ui/ (the
  password array contains no scan token).
- Publication scan (added lines + new files): class **pass**, ZERO hits on
  every pattern (overclaim / secret / url / hex / header classes all 0).
- Visual: `shots/accept_shot_s0.png` (stacked fields, checklist at rest,
  meter, verbatim length line, no-recovery box, gated Create),
  `accept_shot_s0_cli_notice.png` (the courtesy notice in the new design),
  `accept_shot_s1.png` / `accept_shot_s2.png` (the unlock gate in the
  system; identical pre-input by design — the S1/S2 difference is
  post-unlock routing), plus per-shot `*_windows.txt` xwininfo captures
  asserting the "QuantumShield Chat" title. Virtual-display caveat stands;
  physical-display behavior NOT claimed.
- AWAITING OPERATOR EYES (no input driver on the host; unchanged): the
  checklist going green + Create gating, the This-is-you card + alias, the
  Identity pane + rail dot active states, the V&S restructure + amber
  alert + acknowledgment flashes, the destroy/erase expanded forms, the
  tooltips, the main-window empty state. Every behavior behind them is
  test-proven where the harness allows; the operator flies the redesigned
  app after the merges, before slice B is drafted.

## §5 Deviations / discoveries

- The 499be55b missed-push-run incident (§0) — surfaced at Phase 0,
  operator-directed proceed, remedy owed to the operator.
- The S1/S2 screenshots are pixel-identical pre-input (the unlock gate is
  one screen; only `unlockNext` differs) — expected, recorded.
- No ledger edit: no new finding met the filing bar (the incident is an
  operator-remedy item per the NA-0645 precedent, not a ledger finding).
