# NA-0658 as-built — ENG-0044 vault-protections restore (D594, D-1281)

Executed per QSL-DIR-2026-07-19-594 (D594, APPROVED 2026-07-19, sha256
`7a70f6bf…`, 550 lines; F1 = the executed D585/NA-0649 single-implementation-PR
shape with ONE decision D-1281; F2 = typed results — retry-after/attempt data as
VALUES, code strings limited to pre-existing + the enumerated restored originals;
the escalating-delay draft schedule ACCEPTED as design-lock starting values,
landed UNCHANGED). Base: main `aa4ad0cf` (the PR #1597 seating merge; operator
qwork proof verified: HEAD == origin/main, worktree/index/untracked clean,
ready_count=1, queue_top_ready=NA-0658). Branch `na0658-eng0044-restore`.

## §1 Phase 0 — CONFIRM-LIVE (all green, no STOP)

- Directive sha256 `7a70f6bf…` verified on disk; 550 lines.
- Queue: STATE `READY=NA-0658 | HIGHEST_NA=0658 | HIGHEST_D=1280`; anchored
  `^Status: READY` ×1 = the NA-0658 block. D-1281 next-and-absent (canonical
  `- **ID:** D-1280` ×1 = the highest; D-1281 referenced NOWHERE in
  DECISIONS.md).
- ENG-0044 OPEN on the ledger (filed 2026-07-14 by NA-0645, D-1268).
- The six deleted symbol families ABSENT from all Rust sources at base (git grep:
  hits only in governance records): parse_vault_attempt_limit_config,
  vault_security_state_*, wipe_vault_file_best_effort, destroy_with_passphrase,
  TUI_AUTOLOCK_SECRET_KEY, QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS.
- Live-surface anchors present: PROCESS_PASSPHRASE (vault/mod.rs:743) +
  set_process_passphrase/has_process_passphrase (:1062/:1075);
  VAULT_UNLOCKED_THIS_RUN + set_vault_unlocked/vault_unlocked (lib.rs:145–153);
  pub VaultSession + Drop zeroize (vault/mod.rs:720/728); the eight fs_store
  pub(crate) helpers at their recorded lines.
- Source commits verified: implementations last present at `2efc9dab` (NA-0645
  Phase 1), deleted by `86c0858d` (NA-0645 Phase 2; 19 files, −10,542 lines).
- Main health at `aa4ad0cf`: 8/9 push workflows green, formal-ci in its normal
  ProVerif window; public-ci GREEN. Disk 214G free, rw mount.
- Live suite baseline at base: `cargo check -p qsc --all-targets` 0/0; full
  `cargo test -p qsc` = 412 passed / 0 failed / 1 pre-existing-ignored across all
  108 result sets, exit 0 (the NA-0654 record reproduced live); the NA-0640 e2e
  2/0 within the run. BASE CLI binary built and stashed (`2184bee1…`).

## §2 Phase 1 — history extraction + design lock

The `2efc9dab` implementations extracted read-only into the proof root
(main.rs 3,453 lines; store/mod.rs 255; vault/mod.rs 1,178; ownership.rs 1,229;
the three deleted tests 133/105/389 lines) and their contracts recorded against
the D594 HISTORY RECORD — all consistent (default OFF; counter only-when-
configured; wipe-only consequence; no delay; destroy tokenless with the confirm
flow in the TUI UI; files `vault_security.txt` `attempt_limit=1..=100|off` +
`vault_unlock_failures.txt` `failed_unlocks=N`; 0600 perms under store locks).

DESIGN LOCK (proof-root DESIGN_LOCK.md, quoted in the response): placement =
NEW `src/vault/protection.rs` submodule (lib.rs NOT touched — tighter than the
allowed list; `crate::set_vault_unlocked` reachable from the submodule);
restored consts re-homed to store/mod.rs (file names pub(crate); bounds + the
wipe marker pub — the house externally-consumed pattern); the schedule landed AT
the accepted draft values (1–2 free, 5 s doubling, 300 s cap); the counter file
gains ONE additive keyed line `last_failure_unix_s=<T>` (absent-tolerant;
invisible to the historical line-scan parser; RULING recorded: the field key is
the directive-mandated additive timestamp field — persistence-format internal,
never marker vocabulary, not a new code string in the F2 sense); the clock seam
= `_at` variants taking plain u64 unix seconds (zero cfg, zero release-behavior
difference); the token = `DestroyConfirmToken::confirm_with_passphrase`
(passphrase commitment, zeroized on Drop; tokenless calls uncompilable;
wrong-value refused); the final marker/string set exactly as enumerated below.
No STOP fired: no dependency, no ErrorCode variant, no new marker string, no
main.rs/cmd/ touch, no out-of-scope path.

## §3 The landed pub surface (src/vault/protection.rs)

```rust
pub enum GuardedUnlockOutcome {
    Unlocked,
    Rejected { failed_unlocks: u32, retry_after_s: u64 },
    Delayed  { failed_unlocks: u32, retry_after_s: u64 },
    Wiped    { marker: &'static str },
}
pub struct VaultProtectionStatus {
    pub failed_unlocks: u32,
    pub wipe_after: Option<u32>,
    pub retry_after_s: u64,
}
pub fn unlock_guarded(passphrase: &str) -> Result<GuardedUnlockOutcome, &'static str>;
pub fn unlock_guarded_at(passphrase: &str, now_unix_s: u64)
    -> Result<GuardedUnlockOutcome, &'static str>;
pub fn unlock_delay_schedule_s(failed_unlocks: u32) -> u64;
pub fn protection_status() -> Result<VaultProtectionStatus, &'static str>;
pub fn protection_status_at(now_unix_s: u64) -> Result<VaultProtectionStatus, &'static str>;
pub fn wipe_after_failed_unlocks_arm(limit: u32) -> Result<(), &'static str>;
pub fn wipe_after_failed_unlocks_disarm() -> Result<(), &'static str>;
pub fn wipe_after_failed_unlocks_limit() -> Result<Option<u32>, &'static str>;
pub fn lock(session: Option<VaultSession>);
pub struct DestroyConfirmToken { /* private commitment, zeroized on Drop */ }
impl DestroyConfirmToken { pub fn confirm_with_passphrase(passphrase: &str) -> Self; }
pub fn destroy_with_passphrase(passphrase: &str, token: DestroyConfirmToken)
    -> Result<(), &'static str>;
```

Restored consts (src/store/mod.rs, where the originals lived):
`VAULT_SECURITY_CONFIG_NAME`/`VAULT_UNLOCK_COUNTER_NAME` pub(crate);
`VAULT_ATTEMPT_LIMIT_MIN`/`MAX` and `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS`
pub. TUI_AUTOLOCK_SECRET_KEY NOT restored. Private machinery restored from
`2efc9dab` with the historical bodies (parse/load/store/clear under the same
fs_store rails: config_dir + ensure_store_layout + safe-parents + shared/
exclusive locks + write_atomic + 0600 perms + fsync;
wipe_vault_file_best_effort's tombstone dance verbatim; destroy's
validate-by-decrypt + keychain-removal + zero-overwrite-then-remove-then-fsync
ordering verbatim).

## §4 The final Δ-enumeration (restore-vs-redesign, as-built)

- **Δ1 DEFAULT POSTURE**: TUI-era OFF (no config = no protection; the counter
  was not even incremented without a configured limit) → tracking + escalating
  delay ALWAYS ON through the guarded path (fails safe: unreadable protection
  state refuses the attempt with `vault_attempt_limit_io`). Guarded-unlock
  success additionally sets the unlocked flag — the R3 mirror of lock(); the
  TUI-era success path set only the process passphrase + TUI-local state.
- **Δ2 CONSEQUENCE**: TUI-era sole consequence was wipe-at-limit (no delay
  mechanism existed) → the default consequence is lock-and-escalating-delay;
  wipe is a SEPARATE explicit opt-in. The TUI-era wipe path also cleared TUI
  account-local state (wipe_account_local_state_best_effort) — TUI machinery,
  NOT restored; the library wipe = vault file + both state files + lock()
  postconditions + the restored marker.
- **Δ3 COUNTER SEMANTICS**: TUI-era incremented ONLY when a limit was
  configured → every wrong guarded-path attempt counts, always; delay-window
  refusals do NOT increment (no inflation from impatient retries). The
  historical any-failure-counts rule is kept (any inner-unlock Err through the
  guarded path counts). Success-reset only-when-needed (the historical
  reset_unlock_failure_counter early-return) is kept.
- **Δ4 PERSISTENCE FORMAT**: historical file names + bounds restored; the
  counter file gains the additive `last_failure_unix_s=` line (absent-tolerant
  parse both directions).
- **Δ5 PLACEMENT**: machinery re-homed from the deleted main.rs (CLI binary)
  into the library submodule; main.rs untouched; the existing pub(crate)
  fs_store infra reused as-is (the NA-0646 groundwork paid off — no main.rs
  access needed).
- **Δ6 DESTROY CALL SHAPE**: historical destroy_with_passphrase(pass) had no
  token (the TUI UI held the confirm flow) → the restored signature REQUIRES
  DestroyConfirmToken (passphrase-committed, zeroizing; mismatch refused with
  the pre-existing generic `vault_locked`, state unchanged). REFINED
  postconditions added: protection-state files cleared + process left locked —
  the historical destroy did neither.
- **Δ7 TUI VOCABULARY NOT RESTORED**: tui_unlock / tui_autolock /
  tui_lock_state / tui.autolock.minutes stay deleted; the wipe marker rides the
  PRE-EXISTING `vault_unlock` event; the idle timer + minutes setting are
  GUI-side (step 5).
- **Δ8 ENFORCEMENT BOUNDARY**: the guard binds ONLY the new guarded entry
  point (the GUI path); the CLI unlock ingresses stay byte-identical and
  un-guarded (spot-check-proven), consistent with the roadmap honesty note.

## §5 The marker/string census (F2 discipline)

NEW marker strings: NONE. All new data travels as typed values (type/variant/
field names are types, not marker vocabulary — the approval ruling).
- Pre-existing reused: `vault_locked`, `vault_unlock` (event), `ok`, `false`,
  `reason`, `vault.qsv`.
- Restored originals landed WITH their surface: the
  `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS` const;
  `vault_attempt_limit_io`, `vault_attempt_limit_invalid`, `vault_wipe_failed`,
  `vault_erase_failed` (the historical error strings, verbatim);
  `failed_unlock_limit_reached` (the historical wipe-marker reason);
  `vault_security.txt`, `vault_unlock_failures.txt`, `attempt_limit=`, `off`,
  `failed_unlocks=`, the `vault.qsv.tombstone.<pid>` prefix (historical
  file/format vocabulary).
- Directive-mandated additive file field: `last_failure_unix_s=` (persistence
  format internal; the §2 ruling).
- NOT restored: tui_unlock, tui_autolock, tui_lock_state, tui.autolock.minutes.

## §6 Tests (families 1–4: 11 tests, one new result set; family 5 lane-run)

See `tests/NA-0658_eng0044_vault_protections_testplan.md` for the per-test
record. Summary: family 1 (schedule rails; escalation + no-decrypt/no-increment
window refusals; reset-on-success incl. the real-clock wrappers; persistence
across simulated restart via the on-disk pair; clock rollback fails safe),
family 2 (unarmed default proven safe over 10 failures; armed wipe exactly at
threshold with the restored marker via the in-app queue + lock() postconditions;
bounds + disarm persistence), family 3 (lock() one-op postconditions, idempotent,
re-unlock through both paths), family 4 (destroy refusals — wrong/empty
passphrase, wrong token VALUE — with state proven unchanged; irreversibility +
left-locked; keychain branch classified compile-covered-only, headless). All 11
PASS first-run and on the final tree. Family 5: 7-case byte-identity diff EMPTY
(base `2184bee1…` vs final-tree `74d87d07…`); red-demo RED on exactly `u1`,
reverted, green re-proven (revert byte-exactness proven by sha-identical rebuild
`433f8105…` pre/post revert, pre-format).

## §7 Acceptance A–E

- **A** `cargo check -p qsc --all-targets` head: 0 errors / 0 warnings. PASS.
- **B** Full `cargo test -p qsc` head: 423/0/1 across 109 result sets, exit 0 =
  the base-derived 412/0/1×108 + EXACTLY the one new 11-test set; the NA-0640
  e2e 2/0 within the run, zero e2e edits. PASS.
- **C** Untouched-surface proof: Cargo.toml/Cargo.lock ABSENT from the diff;
  `cargo metadata --locked` OK; zero diff under .github/, formal/, vectors/,
  src/main.rs, src/cmd/; the 19-file env-ingress KEEP-anchor set: only the
  explicitly-allowed vault/mod.rs in the diff (the 5-line module decl), its four
  ingress items sha-identical base vs head; `git diff --stat` confined to the
  allowed list (4 source/test files + the governance set). PASS.
- **D** Byte-identity spot-check + red-demo green (§6). PASS.
- **E** Validation defaults: git diff --check clean; scope guard exact; queue/
  decision proof (§1 pre-landing; the closeout flips recorded in this PR);
  marker census (§5); added-line private-material/prohibited/overclaim scans
  clean; fmt --check = exactly the 145 known pre-existing diffs (zero lane);
  root audit 386/0 exit 0; fuzz audit 287/0 exit 0; sh -n/bash -n OK; goal-lint
  local OK (literal `Goals: G4` in the PR body from creation). PASS.

## §8 NOT claimed

No GUI exists; no idle timer runs anywhere (the timer + minutes setting are the
step-5 skeleton lane's); an offline copy of the vault file is NOT defended by
these protections (passphrase strength + Argon2id only — the roadmap honesty
note stands); the keychain-removal branch is not runtime-exercised headless; the
external-review gate has not moved; no public/production/crypto-complete/
bug-free/vulnerability-free claim. Claim boundary UNCHANGED.
