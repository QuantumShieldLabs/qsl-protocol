# NA-0659 as-built — GUI skeleton + onboarding SLICE A (D595; spine D-1282; qsl-desktop D-0002)

Result class: GUI_SKELETON_SLICE_A_PASS (both PRs open at the stop: qsl-desktop #2 head 9a1f40c; the spine closeout PR). Base spine `81143dcd` (the #1599 seating merge,
qwork-proven); qsl-desktop base `09a6ad71` (fresh clone, protection `rust`
strict enforce_admins verified binding at Phase 0; PVR enabled;
delete_branch_on_merge=true).

## §1 The scaffold (F1/F3 as ruled)

Root Cargo.toml: the NA-0657 F3 package-at-root placeholder REPLACED by the
workspace root (members = ["src-tauri"], resolver 2); placeholder src/main.rs
deleted. src-tauri = package `qsl-desktop-app` (lib `qsl_desktop_app` + binary
`qsl-desktop`), edition 2021, AGPL-3.0-only. tauri.conf.json: identifier
`org.quantumshieldlabs.qsldesktop`; frontendDist ../ui; withGlobalTauri true
(the F3 zero-npm static frontend uses the injected global, no @tauri-apps/api
package); CSP default-src 'self'; bundle.active false; single window "main"
1024x700. capabilities/default.json = core:default for "main".
Frontend: ui/{index.html,style.css,main.js} — static vanilla, ZERO
npm/node/bundler/JS-deps (F3 ruling + rationale recorded). One generated
64x64 RGBA icons/icon.png (tauri::generate_context! requires it even with
bundling off — discovered at first build).
Resolved at lock: tauri 2.11.5, tauri-build (2.x per lock), tauri-runtime-wry 2.11.4, wry 0.55.1, tao 0.35.3 — the investigation-proven family.

## §2 The qsc pin + the lock-alignment event (recorded honestly)

qsc = git dep at rev `81143dcd3b4a7beead7d0f4e742717a4310e2409` (= the
Phase-0 spine main; contains ba4099bd = the NA-0658 protection surface;
ancestry proven). Future advances = the ENG-0041/0046 deliberate bump-lane
pattern.

LOCK-ALIGNMENT EVENT (first build, ENG-0017 CLASS OBSERVED LIVE): the fresh
workspace lock resolved the RustCrypto PRE-RELEASE family to newer releases
than the spine lock pins, and `quantumshield_refimpl` (a qsc dependency)
FAILED TO COMPILE against them (ml-dsa `SigningKey::from_expanded` gone in
0.1.1; then ml-dsa 0.1.0-rc.7 itself failing against release pkcs8/spki).
Remedy: TWELVE `cargo update -p <pkg> --precise <ver>` alignments, every one
TOWARD the spine's locked version (never past it):
ml-dsa 0.1.1→0.1.0-rc.7; ml-kem 0.2.3→0.2.1; pkcs8 0.11.0→0.11.0-rc.11;
spki 0.8.0→0.8.0-rc.4; sha3 0.11.0→0.11.0-rc.8; signature 3.0.0→3.0.0-rc.10;
keccak 0.2.0→0.2.0-rc.2; der 0.8.1→0.8.0; digest 0.11.3→0.11.2;
block-buffer 0.12.1→0.12.0; crypto-common 0.2.2→0.2.1;
hybrid-array 0.4.13→0.4.8. After alignment the full tree (qsc + refimpl +
tauri) compiles clean. Implication recorded for future pin-advance lanes:
qsl-desktop lock maintenance must hold the qsc-graph crypto family AT the
spine's locked versions (the spine's ENG-0017 maturity risk propagates to
every satellite consumer of qsc).

## §3 The core-call layer (the four startup rules)

(a) `bootstrap()` (src/lib.rs) — dirs created 0700, QSC_CONFIG_DIR set,
init_output_policy(false), set_marker_routing(InApp),
install_panic_redaction_hook — called in run() before Builder; test
`rule_a_bootstrap_env_dirs_and_routing`.
(b) `MarkerBuffer` (src/markers.rs) cap 1024, drop-oldest + visible dropped
counter; `CoreGateway::call` drains after EVERY call; tests
`rule_b_drain_after_every_call` + `overflow_drops_oldest_and_counts`.
(c) lock state only via qsc protection::{lock, unlock_guarded}; source-scan
test `rule_c_no_raw_global_symbols_in_src`.
(d) `CoreGateway` (src/gateway.rs): async Mutex single-flight +
spawn_blocking + in_flight flag (UI busy indicator + per-invoke disabled
controls); test `rule_d_strictly_serial_single_flight` (max observed
concurrency 1 of 4 contenders).

## §4 The launch state machine + surfaces

`resolve_launch_state` (src/state.rs): S0 = !exists(<data>/qsc/vault.qsv)
(the F2 app-level probe; coupling filed as ENG-0047); S1/S2 =
identity_read_self_public("self") None/Some (pub, read-only, no unlock);
corrupt identity record resolves S1 (fail-toward-unlock). Commands
(src/commands.rs): launch_state, cli_vault_present, vault_create (empty/
mismatch refused; init + guarded unlock), identity_ensure/show (fingerprint +
verification code + purpose line + PQ line), unlock_attempt (typed DTO
mirroring GuardedUnlockOutcome — values, not strings), lock_now (the one-call
lock()), protection_status (+ pub bounds), wipe_arm/disarm (bounds-checked),
settings_get/set (non-secret allowlist), destroy_vault (typed phrase +
passphrase → DestroyConfirmToken → destroy_with_passphrase), erase_all
(typed phrase; app-level removal ONLY; CLI-dir guard; recreates the empty
0700 qsc dir; lands S0), marker_stats, core_busy, app_info.
UI (ui/main.js): wizard cards 1–2 with progress dots (step 1 not
revisitable — no Back exists), strength meter labeled guidance-only,
no-recovery warning shown BEFORE commit, CLI-vault courtesy notice (S0
only), unlock screen with live countdown from the typed retry_after_s,
wiped-notice screen → S0, two-step erase (link → typed-phrase screen),
three-pane main window with the single honest status line, "Add your first
contact" stub, Settings rail (Server stubbed honestly; Vault & Security
live; Appearance/Notifications stubbed; About), idle autolock 15-min
default adjustable, wizard-exempt, firing lock_now.
Identity export: ABSENT everywhere (banked decision).

## §5 Test + acceptance results

cargo test: 16 passed / 0 failed / 1 ignored (the documented scripted-
acceptance seed hook), across: unit (settings 3 + markers 1), slice_a_rules
(5: the four startup rules + the zero-networking scan), slice_a_flows (7:
c_prime_deferred_path_to_honest_disconnected, d_interruption_matrix,
e_unlock_lifecycle_typed_feedback_delay_and_reentry, e_armed_wipe_lands_s0,
unarmed_default_never_wipes, erase_all_lands_s0_and_never_touches_cli_dir,
destroy_requires_correct_passphrase_then_lands_s0). All delay/wipe timing
via the NA-0658 clock seam (unlock_guarded_at) — zero wall-clock sleeps.
cargo fmt --all -- --check CLEAN; cargo clippy -q -- -D warnings CLEAN
(exit 0); cargo metadata --locked OK; git diff --check clean.
cargo audit on the NEW workspace lock: EXIT 0 — ZERO vulnerabilities; 17
allowed warnings, all RUSTSEC-2024-041x 'gtk-rs GTK3 bindings no longer
maintained' (the family every Tauri-v2-on-Linux app carries; reported, not
silenced).
Publication scans (added lines + new files): zero overclaims; pattern hits
dispositioned — http_url 513 + long_hex 514 = the Cargo.lock registry URLs
+ checksums + the pin rev; secret_assignment 1 = the 'let token =' Destroy-
ConfirmToken binding (a type construction, no secret value).
Scope guard EXACT: 25 files changed = the D595 slice-A allowed paths; the
six community-health/governance files 0-delta each.
VISUAL ACCEPTANCE (xvfb; virtual-display caveat stands): the SSH/Xvfb
context has NO session D-Bus and GTK blocks ~25 s inside
g_application_register -> g_dbus_proxy_new_sync (gdb-backtrace-diagnosed;
the app itself was NEVER at fault). Working recipe RECORDED for all future
GUI lanes: xvfb-run -a ... dbus-run-session -- <app>. Screenshots in this
proof root: accept_shot_s0.png (the wizard step-1 card: progress dots,
no-recovery warning BEFORE commit, strength meter area, Create vault);
accept_shot_s0_cli_notice.png (the courtesy notice rendered with the exact
D595 wording); accept_shot_s1.png / accept_shot_s2.png (the unlock gate
with the Forgot-your-passphrase link). AWAITING OPERATOR EYES (no input
driver on the host; xdotool/xte absent, installing not authorized): the
step-2 identity card, the unlock countdown rendering, the wiped notice,
the erase screen, the main window three panes + status line, the Settings
Vault/Security and About panes — every flow behind them is test-proven
above.
CI (g): the extended workflow ran on qsl-desktop PR #2 (the first execution
of the apt-set form; conclusion recorded in the response at the stop);
branch protection re-verified binding required context ["rust"] strict
enforce_admins at Phase 0 and unchanged by the lane (no settings mutation).

## §6 Deviations / discoveries

- The icons/icon.png requirement (tauri::generate_context! fails without it,
  bundling off notwithstanding) — one generated 314-byte RGBA PNG added;
  not in the D595 enumerated tree, recorded here.
- The lock-alignment event (§2) — the D595 "record resolved versions" duty
  widened to the 12 precise pins; surfaced, not silent.
- The wiped notice renders the honest generic form ("the armed number of
  failed unlock attempts") — the typed Wiped outcome carries no N and the
  post-wipe app cannot read the erased config; fidelity note.
- Acceptance visual coverage bounded by the host: no input driver
  (xdotool/xte absent; installing = not authorized) — launch-state
  screenshots only; input-driven surfaces enumerated for operator eyes;
  ALL flows proven at the backend level by test.
