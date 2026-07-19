# NA-0659 — GUI skeleton + onboarding SLICE A — test plan (D595, D-1282/D-0002)

Repo under test: QuantumShieldLabs/qsl-desktop (branch na0659-gui-skeleton-slice-a,
off `09a6ad71`). Spine side is governance-only; zero spine code. All commands run
at the qsl-desktop workspace root on rustc 1.95.0.

## 1. The four startup rules (D595 Phase 2; src-tauri/tests/slice_a_rules.rs)

| Rule | Test | Proves |
|---|---|---|
| (a) env+policy+routing once before threads | `rule_a_bootstrap_env_dirs_and_routing` | QSC_CONFIG_DIR fixed to `<data>/qsc`; both dirs 0700; a subsequent core call's marker lands in the queue (InApp branch taken) |
| (b) drain after EVERY call, bounded | `rule_b_drain_after_every_call` + `markers::tests::overflow_drops_oldest_and_counts` | qsc queue empty after a gateway call; drained line in the app buffer; cap drops oldest + counts visibly |
| (c) lock state only via NA-0658 surface | `rule_c_no_raw_global_symbols_in_src` | source scan: `set_vault_unlocked` / `set_process_passphrase` appear nowhere in src-tauri/src |
| (d) strictly serial single-flight | `rule_d_strictly_serial_single_flight` | 4 concurrent gateway callers; observed max concurrency == 1 |
| slice-A boundary | `zero_networking_in_src_and_ui` | source scan: reqwest/hyper/http(s)/ws tokens appear nowhere in src-tauri/src or ui/ |

## 2. The acceptance matrix (D595 Phase 6; src-tauri/tests/slice_a_flows.rs)

Backend level, against the REAL qsc surface on disk. "Kill + relaunch" is honest
at this level because the launch state machine derives everything from disk (no
onboarding-complete flag exists): a fresh `resolve_launch_state` IS what a
relaunched process computes. Delay/wipe timing uses the NA-0658 clock-injection
seam (`unlock_guarded_at`) — zero wall-clock sleeps.

| Item | Test |
|---|---|
| (c′) deferred path → honest disconnected | `c_prime_deferred_path_to_honest_disconnected` (S0 → vault → identity → S2; settings key allowlist proves no server key can exist) |
| (d) interruption matrix | `d_interruption_matrix` (kill-after-vault → S1 → unlock → identity resumes idempotently; kill-after-identity → S2; records byte-equal) |
| (e) unlock lifecycle | `e_unlock_lifecycle_typed_feedback_delay_and_reentry` (Rejected 1/2 free → 3rd engages 5 s → Delayed carries remaining wait as VALUE → correct-after-window unlocks + resets → lock() one-call → re-entry) |
| (e) armed wipe | `e_armed_wipe_lands_s0` (arm 2 → Wiped at 2 with the restored marker value → vault gone → S0) |
| unarmed default safe | `unarmed_default_never_wipes` (10 failures; vault intact; still unlockable) |
| forgotten-passphrase erase | `erase_all_lands_s0_and_never_touches_cli_dir` (locked erase → S0; CLI profile byte-untouched; CLI-dir guard refuses) |
| tokened destroy | `destroy_requires_correct_passphrase_then_lands_s0` (wrong pass refused, vault intact; correct pass + token destroys → S0, left locked) |

Unit: `settings::tests::{default_is_fifteen_minutes, roundtrip_and_zero_rejected,
settings_key_allowlist}` (autolock default/bounds; the non-secret key allowlist).

## 3. Visual acceptance (xvfb; no input driver exists on the build host)

`accept_screens.sh` (proof root): pre-seeded data dirs (the ignored
`seed_acceptance_dir` hook) + `xvfb-run` + `scrot`:
- `accept_shot_s0.png` — fresh dir → the wizard, step 1 (vault card)
- `accept_shot_s0_cli_notice.png` — S0 with a CLI vault present → the courtesy notice
- `accept_shot_s1.png` — vault-only dir → the unlock screen
- `accept_shot_s2.png` — vault+identity dir → the unlock screen (S2 gate)

Enumerated as AWAITING OPERATOR EYES (input-driven surfaces, logic test-proven
above): wizard step 2 card rendering; the unlock countdown rendering; the wiped
notice; the erase screen; the main window three-pane + status line; the Settings
Vault/Security pane; the About pane. Virtual-display caveat: all runs under
xvfb-run; physical-display behavior is NOT claimed.

## 4. (g) CI

The extended workflow (job id `rust` KEPT; apt step = investigation §4.1 set)
must run and pass on the qsl-desktop PR; branch protection re-verified binding
post-PR (`required_status_checks.contexts == ["rust"]`, strict, enforce_admins).

## 5. Gates (Phase 7)

cargo fmt --all -- --check; cargo test -q; cargo clippy -q -- -D warnings (the
exact CI commands); cargo metadata --locked; cargo audit on the NEW workspace
lock (report count; red ⇒ STOP); scope guard (allowed paths only; community-health
files byte-untouched); git diff --check; publication scans over added text files;
goal-lint local on the spine PR (literal `Goals:` line from creation).
