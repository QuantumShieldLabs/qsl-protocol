# NA-0660 testplan — GUI slice-A design pass (D596; D-1283 + D-0003)

Scope: presentation + the acknowledgment rule ONLY. The controlling
invariant is NEGATIVE — no semantic moved: the two slice-A test files are
byte-frozen inputs to this plan, not subjects of it.

| # | Check | Method | Result |
|---|-------|--------|--------|
| 1 | Slice-A acceptance re-proof (interruption matrix, unlock lifecycle, deferred path) | `slice_a_flows.rs` + `slice_a_rules.rs` byte-identical to base (absent from the diff) and green | PASS (7+5 tests; 24/0/1 suite) |
| 2 | Fresh-profile settings key set unchanged | The frozen flows assertion `keys == ["autolock_minutes"]` + the skip-when-empty alias serialization | PASS (unmodified test green) |
| 3 | Type-scale discipline (item 1) | `font_sizes_only_from_scale_tokens` — every font-size outside `:root` uses `var(--fs-…)` | PASS |
| 4 | Spacing-scale discipline (item 2) | `spacing_only_from_scale_tokens` — every padding/margin/gap outside `:root` is `var(--sp-…)`/0/auto | PASS |
| 5 | Button tiers (item 3) | `every_button_is_tiered_or_nav` — every `<button>` exactly one tier or a named nav role | PASS (19 buttons) |
| 6 | Accent/color discipline (item 4) | `colors_only_in_token_block` — zero hex/rgb(a) literals outside `:root` | PASS |
| 7 | Acknowledgment rule (item 5) | ONE shared `acknowledge()` helper wired to autolock/wipe-arm/wipe-disarm/alias; flash + status line | Code-reviewed; flash behavior = OPERATOR EYES |
| 8 | Display-name binding (item 6) | `display_name_and_identifier_binding` + live xwininfo assertion under xvfb | PASS ("QuantumShield Chat"; identifier unchanged) |
| 9 | Passphrase checklist data (item 7) | `common_passwords_list_is_sound` — ≥100 (149), lowercase, sorted, unique, marker-anchored | PASS |
| 10 | Checklist gating Create (item 7) | UI logic (updateReqs → disabled) | Code-reviewed + S0 screenshot (gated at rest); going-green = OPERATOR EYES |
| 11 | Appendix A verbatim copy (items 7/8/12) | `appendix_a_copy_verbatim` — the verbatim lines present; "Danger zone" absent | PASS |
| 12 | Identity pane zero-new-core-calls (item 9) | qsc-symbol set head == base (23/23, grep in proof root); command count 17 unchanged | PASS |
| 13 | Unlock-entry capture rule (item 11) | Code pointer: `showUnlockScreen()` / the unlock handler in ui/main.js; the reset-on-success fact re-verified in qsc protection.rs | Code-reviewed; amber alert render = OPERATOR EYES |
| 14 | Zero networking / no raw globals | The frozen `zero_networking_in_src_and_ui` + `rule_c` scans | PASS |
| 15 | No dependency/lockfile/workflow motion | Cargo.toml/Cargo.lock/ci.yml absent from the diff; `cargo metadata --locked` OK; audit exit 0 unchanged lock | PASS |
| 16 | fmt / clippy | `cargo fmt --all -- --check`; `cargo clippy --workspace -- -D warnings` | PASS (both clean) |
| 17 | Publication boundary | added-line publication scan | PASS (zero pattern hits) |
| 18 | CI (the `rust` context) | The check ran and passed on qsl-desktop PR #3; protection re-verified binding | PASS |
| 19 | Launch-state visuals | xvfb-run + dbus-run-session screenshots ×4 into the proof root | PASS (captured; virtual-display caveat) |

OPERATOR-EYES enumeration (no input driver on the host; every behavior
test-proven where the harness allows): checklist items going green + the
Create gate releasing; the This-is-you card (hero code, alias field,
disclosure); the Identity pane + rail dot + active states; the V&S
restructure, amber alert + dismiss, acknowledgment flashes; the
destroy/erase expanded forms; tooltips; the main-window empty state. The
operator flies the redesigned app after the merges, before slice B.

Main-health deviation: zero workflow runs spawned for the seating merge
`499be55b` (NA-0645-class); substantive tree = `35948ec0` (9/9 green);
remedy = the operator-sanctioned NA-0237D-shape bootstrap PR (owed).
