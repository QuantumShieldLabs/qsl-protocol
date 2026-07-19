# NA-0658 — ENG-0044 vault-protections restore: test plan & results (D-1281)

Lane: NA-0658 per QSL-DIR-2026-07-19-594 (D594, APPROVED 2026-07-19; F1 single
implementation PR + one decision D-1281; F2 typed results, no new code strings; the
escalating-delay draft schedule accepted as design-lock starting values; directive
sha256 `7a70f6bf…`, 550 lines). Base: main `aa4ad0cf` (the PR #1597 seating merge,
qwork-proven). All new coverage is library-level (the NA-0649 pattern) in ONE new
integration test file `qsl/qsl-client/qsc/tests/NA_0658_vault_protections.rs`
(11 tests, 1 new result set); the compiled CLI binary is exercised ONLY by the
byte-identity spot-check (family 5). Delay tests never sleep — the design-locked
clock seam (`unlock_guarded_at` / `protection_status_at`, plain u64 unix-second
parameters; zero cfg, zero release-behavior difference) drives fabricated
timelines.

## Family 1 — escalating delay (default-on)

- `delay_schedule_matches_the_accepted_rails` — the pure schedule fn: failures 1–2
  free (0 s); from the 3rd: 5, 10, 20, 40, 80, 160, then 300 capped, monotonic
  non-decreasing through u32::MAX. **PASS.**
- `escalating_delay_engages_counts_and_grows` — wrong attempts through the guarded
  path increment the persisted counter (Rejected {1,0} → {2,0} → {3,5}); an
  in-window attempt with the CORRECT passphrase returns Delayed {3, remaining=4},
  decrypts nothing (process passphrase still unset) and does NOT increment; after
  the window the next failures escalate {4,10} → {5,20}. Retry-after exposed as
  typed VALUES throughout. **PASS.**
- `correct_passphrase_after_window_unlocks_and_resets` — correct passphrase after
  the window: Unlocked; process passphrase set + unlocked flag set (the R3 mirror);
  counter and delay reset, persisted (`failed_unlocks=0`); the real-clock wrappers
  (`unlock_guarded`, `protection_status`) exercised. **PASS.**
- `persistence_across_simulated_restart_continues_count_and_delay` — with state
  persisted mid-escalation (count 3), ALL in-process state cleared via `lock(None)`
  (the simulated restart; disk is the only carrier): the raw counter file holds
  `failed_unlocks=3` + `last_failure_unix_s=<t>`; a fresh load continues the count
  and the delay (status {3, 4}; Delayed on attempt; next failure {4,10}) — no
  reset. **PASS.**
- `clock_rollback_fails_safe` — a now-earlier-than-last-failure reading yields the
  FULL current delay (5 s, never shortened) on both the status and unlock paths;
  forward time elapses normally. **PASS.**

## Family 2 — wipe-after-N (explicit opt-in)

- `unarmed_default_never_wipes` — UNARMED DEFAULT PROVEN SAFE: 10 wrong attempts
  with no limit armed are all Rejected (never Wiped), the vault file remains, and
  a correct passphrase after the final window unlocks. **PASS.**
- `armed_wipe_triggers_exactly_at_threshold_with_restored_marker` — arm(3); two
  failures leave the vault present; the 3rd returns Wiped with the restored
  `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS` as the typed marker VALUE; the vault
  file is gone (the historical tombstone dance), BOTH state files cleared, the
  lock() postconditions hold, and the restored marker is observed via the in-app
  marker QUEUE on the pre-existing `vault_unlock` event
  (`reason=failed_unlock_limit_reached`; the deleted `tui_unlock` event is NOT
  restored). **PASS.**
- `arm_bounds_enforced_and_disarm_persists_off` — bounds 1..=100 restored and
  enforced (`0`/`101` → `vault_attempt_limit_invalid`; 1 and 100 accepted;
  `attempt_limit=100` persisted); disarm persists `attempt_limit=off` and further
  failures past the previously-armed limit do not wipe. **PASS.**

## Family 3 — lock() as ONE operation (R3)

- `lock_is_one_operation_idempotent_and_relockable` — after unlock (passphrase
  set, flag true, live VaultSession): the single `lock(Some(session))` leaves
  `has_process_passphrase()` false, `vault_unlocked()` false, the session consumed
  (its existing Drop zeroize erases key material); a second `lock(None)` is a
  no-op; a subsequent unlock works through BOTH the guarded path and the existing
  `unlock_with_passphrase` — the safe mid-process re-lock the GUI idle timer
  needs. **PASS.**

## Family 4 — destroy (deliberate, instant, token-confirmed)

- `destroy_refuses_wrong_passphrase_empty_passphrase_and_wrong_token` — wrong
  passphrase refused (validate-by-full-decrypt, state unchanged); empty passphrase
  refused (the historical guard); correct passphrase with a token committed to a
  DIFFERENT value refused with NO destruction — the wrong-VALUE path (the
  absent-token call shape does not compile: the signature requires
  `DestroyConfirmToken`). Vault intact, still unlockable, seeded secret unchanged
  after all three refusals. **PASS.**
- `destroy_with_token_is_irreversible_and_leaves_locked` — correct passphrase +
  matching token: the vault file is gone (the historical zero-overwrite at
  recorded length THEN remove THEN fsync ordering); post-destroy unlock fails
  (`vault_missing`), protected data unreadable, protection-state files cleared,
  the process left locked (both globals false). **PASS.**
- Keychain-removal coverage (key_source == 2): NOT exercisable headless —
  `QSC_DISABLE_KEYCHAIN=1` forces the passphrase source in the suite environment.
  Classified honestly: the keychain branch of the restored machinery is
  compile-covered and byte-restored from `2efc9dab`, not runtime-exercised here.

## Family 5 — CLI byte-identity spot-check + WF-0017 red-demo (lane-run)

Vehicle: proof-root `na0658_reduced_prover.sh` (the NA-0646 prover pattern reduced
to the touched neighborhoods; not committed — `scripts/` is outside the D594
allowed paths). Corpus (7 cases, stdout+stderr+exit under `env -i LC_ALL=C
TZ=UTC`, one fixture created with the BASE binary and reused): `v1` vault init
prover form (fresh), `v2` vault init plain (fresh), `v3` vault init
vault_exists-negative (fixture), `u1` unlock via the existing file ingress
(`vault unlock --non-interactive --passphrase-file`), `v4` vault status, `r1`
relay token-set (the secret_set vehicle — no `secret` CLI subcommand exists; the
NA-0649 precedent vehicle), `i1` identity show (control).

- **Result: PASS — `diff -r` EMPTY across all 7 cases**, base binary
  (`2184bee1…`, built at `aa4ad0cf` pre-change) vs the final-tree binary
  (`74d87d07…`) — differing binaries, so the comparison is non-vacuous.
- **WF-0017 non-vacuity: PASS** — a deliberate perturbation of the pre-existing
  `vault_unlock` success marker turned the differ RED (exit 1) on exactly the
  predicted `u1` case and no other (`red_demo_diff.txt` preserved in the proof
  root); reverted; green re-proven on the final tree (and the pre-format rebuild
  after the revert was sha256-identical to the pre-revert head binary,
  `433f8105…` both, proving the revert byte-exact).

## Repo gates (final tree)

- `cargo check -p qsc --all-targets`: 0 errors / 0 warnings, base AND head. **PASS.**
- Full local `cargo test -p qsc` (niced, `--test-threads=3`): base-derived
  baseline at `aa4ad0cf` = **412 passed / 0 failed / 1 pre-existing-ignored across
  all 108 result sets, exit 0** (the NA-0654 record reproduced live); head =
  **423 passed / 0 failed / 1 pre-existing-ignored across 109 result sets, exit
  0** — the baseline + EXACTLY the one new 11-test set; per-set normalized
  (timing-stripped) results otherwise IDENTICAL; the NA-0640 e2e 2/0 within both
  runs, zero e2e edits. **PASS.**
- Scope guard: changed paths ⊆ the D594 allowed list (src/vault/protection.rs NEW,
  src/vault/mod.rs = the 5-line module decl ONLY, src/store/mod.rs = the restored
  consts ONLY, the one new test file, the governance/closeout set; lib.rs NOT
  touched — tighter than allowed). Zero diff under .github/, formal/, vectors/,
  src/main.rs, src/cmd/; Cargo.toml/Cargo.lock ABSENT from the diff; of the 19
  env-ingress KEEP-anchor files only the explicitly-allowed vault/mod.rs is in the
  diff and its four ingress items (unlock_with_passphrase_env,
  passphrase_env_allowed, passphrase_from_allowed_env, DESKTOP_PASS_ENV_KEY) are
  sha-identical base vs head. **PASS.**
- Validation defaults: `git diff --check` clean; `cargo fmt --check` = exactly the
  145 KNOWN pre-existing diffs at base (zero from lane files; the store/mod.rs
  EOF-blank deviation verified PRE-EXISTING at base and inherited untouched);
  `cargo metadata --locked` OK; root cargo audit 386 deps / 0 advisories, exit 0;
  nested fuzz audit 287 deps / 0 advisories, exit 0; `sh -n`/`bash -n`
  scripts/ci/qsc_adversarial.sh OK. **PASS.**

Raw run logs and captures: lane proof root
`/srv/qbuild/tmp/NA0658_eng0044_vault_protections_restore_20260719T032031Z/`
(proof-root-only; class summaries here per policy).
