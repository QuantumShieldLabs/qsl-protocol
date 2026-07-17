# NA-0649 as built — qsc GUI-surface lane (D585, D-1272)

Lane: NA-0649 per QSL-DIR-2026-07-16-585 (D585, operator-issued), seated by promotion
PR #1579. Base: main `fb1ef2bc` (the #1579 seating merge), recorded at Phase 0 from a
FRESH GitHub clone (never the qbuild mirror; hash proof in the lane proof root).
Decision: D-1272. Result class: QSC_GUI_SURFACE_PASS.

## §1 What the lane adds (LIBRARY surface only; zero CLI behavior change)

Closes the three blocking findings of the 2026-07-16 GUI-readiness investigation so the
upcoming GUI (separate satellite repo, in-process library consumer of the NA-0646
extraction surface) can drive onboarding without shelling out or writing secrets to
disk.

**The exact pub surface added or widened:**

- B1 (new, `src/vault/mod.rs`):
  `pub fn vault_init_with_passphrase(passphrase: &str) -> Result<(), &'static str>`
- B3 (new, `src/identity/mod.rs`):
  `pub fn identity_ensure(self_label: &str) -> Result<IdentityPublicRecord, ErrorCode>`
- B2 (widened `pub(super)` → `pub`, NO body changes, `src/identity/mod.rs`):
  - `pub struct IdentityPublicRecord { pub kem_pk: Vec<u8>, pub sig_pk: Vec<u8> }`
    (fields widened for external-crate reads; serialized shape UNCHANGED — the
    fingerprint stays DERIVED, never a stored field)
  - `pub fn identity_read_self_public(self_label: &str) -> Result<Option<IdentityPublicRecord>, ErrorCode>`
  - `pub fn identity_fingerprint_from_identity(kem_pk: &[u8], sig_pk: &[u8]) -> String`
  - `pub fn format_verification_code_from_fingerprint(fingerprint: &str) -> String`
- Operator-approved D585 scope amendment (widened `pub(crate)` → `pub`, NO body
  changes, `src/model/mod.rs`): `pub enum ErrorCode` and `pub fn as_str(self) -> &'static str`.

## §2 The operator-approved D585 scope amendment (recorded per the operator's wording)

`src/model/mod.rs` was added to the allowed paths for EXACTLY the two-word visibility
widening (`pub(crate) enum ErrorCode` → `pub`; `pub(crate) fn as_str` → `pub`). No body
change, no behavior change, no vocabulary change, no new variants, no other item in
that file touched. Rationale: the GUI-consumability purpose of B2/B3 requires an
externally nameable error type, and the `private_interfaces` lint empirically breaks
the `cargo check --all-targets` 0/0 gate (proven on rustc 1.95.0 before the amendment
was requested: widening `identity_read_self_public` alone produced
`warning: type 'model::ErrorCode' is more private than the item …`). B2/B3 keep the
house ErrorCode signatures as D585 specifies. All other boundaries unchanged.

## §3 B1 mechanics — routed through the existing private internals

`vault_init`'s ingress-independent tail (KDF params → salt → `derive_key` → cipher →
nonce → default route-token generation → payload seed `tui.relay.inbox_token` →
encrypt → path resolve → `vault_exists` check → parent create/perms → envelope build →
atomic write → zeroize → the `vault_init` success marker) was extracted VERBATIM into
the private `fn vault_init_core(key_source, pass) -> Result<(), &'static str>`; the CLI
path (`vault_init`) now calls it and maps codes with `CliError::code` — the identical
`CliError` the deleted per-site `fail_with_marker_buffers` calls produced (that helper
became the str-returning `fail_core_buffers`, same zeroize discipline;
`handle_provider_error` now delegates to the new `provider_error_code` with the same
four code strings). The new pub entry pins `KeySource::Passphrase` (identical to what a
successful `vault init --passphrase-file` resolves in the default build: the `keychain`
feature is not a default feature, so `keychain_supported()` is compile-time false) and
reads NO argv/env/file/stdin/terminal. The intermediate `String`→bytes copy is the one
buffer, zeroized by the core on every path (success and each error). NO
`set_process_passphrase` / `set_vault_unlocked` on any init path (also asserted by
test). The empty-passphrase guard returns the EXISTING code `vault_passphrase_required`
(the CLI's code for the same condition; no new vocabulary). Retired-ingress paths
(argv/env, NA-0216B) and the `QSC_DESKTOP_SESSION_PASSPHRASE` machinery untouched —
`resolve_passphrase` and all `VaultInitArgs` handling byte-identical.

## §4 B3 mechanics — thin, guard-preserving

`identity_ensure` = `identity_read_self_public` (existing record returned, NO mutation,
no unlock needed — exactly today's read path) else `identity_self_kem_keypair` (the
existing lazy creation path VERBATIM: the NA-0616 second-identity guard fires unchanged
— proven by test; vault-level unlock enforcement unchanged — locked store yields the
existing `identity_secret_unavailable`/`vault_locked` behavior with zero mutation,
proven by test). The wrapper zeroizes the secret halves of the returned keypair before
dropping them. Rotation stays the separate explicit `identity_rotate` flow (untouched).

## §5 Tests (all library-level; the compiled-binary suite untouched)

New file `qsl/qsl-client/qsc/tests/NA_0649_gui_surface.rs` — 7 tests, the
external-crate-shaped consumer (integration test linking qsc as an external crate;
markers observed via the KEEP `MarkerRouting::InApp` queue; process-global state
serialized on a file-local mutex):

1. `vault_init_with_passphrase_roundtrip_no_unlock_side_effect` — creates a vault a
   subsequent `unlock_with_passphrase` opens; wrong passphrase rejected; the
   `vault_init` success marker observed; `has_process_passphrase()` false AND
   `vault_unlocked()` false after init alone; the default inbox route token seeded.
2. `vault_init_with_passphrase_second_call_vault_exists` — second call fails
   `vault_exists` as a value.
3. `vault_init_with_passphrase_empty_passphrase_rejected` — `vault_passphrase_required`,
   no mutation on reject.
4. `identity_ensure_creates_then_is_idempotent` — fresh unlocked store creates; fp
   matches the subsequent `identity_show` marker; second call returns the SAME record
   with store-bytes stability.
5. `identity_ensure_locked_store_keeps_existing_vault_locked_behavior` —
   `ErrorCode::IdentitySecretUnavailable` (as_str `identity_secret_unavailable`), the
   existing `vault_locked` marker, zero mutation.
6. `identity_ensure_preserves_second_identity_guard` — second label refused with
   `ErrorCode::IdentitySelfAmbiguous` + the `identity_self_ambiguous` marker (NA-0616
   guard unchanged).
7. `widened_accessors_expose_identity_as_data` — {fp, kem_pk, sig_pk} + verification
   code read as values; code shape verified (4-4-4-4-checksum); fp cross-checked
   against the `identity_show` marker; ErrorCode matched externally
   (`Err(ErrorCode::ParseFailed)` on an invalid label — the amendment's purpose
   demonstrated).

## §6 The CLI byte-identity spot-check (D585 test 4 — the NA-0646 prover pattern, reduced)

Lane-run harness (proof root `na0649_reduced_prover.sh`, not committed — `scripts/` is
outside the D585 allowed paths; the committed NA-0646 14-case prover remains the full
corpus). FIXED corpus over the touched neighborhoods, one fixture created ONCE with the
BASE binary and reused: `v1` `vault init --non-interactive --key-source passphrase
--passphrase-file` (fresh store), `v2` `vault init --passphrase-file` (fresh store),
`v3` the `vault_exists` negative (fixture store), `i1` `identity show` (unlocked
fixture), `r1` `relay token-set` (unlocked fixture). Captured stdout+stderr+exit under
`env -i LC_ALL=C TZ=UTC`.

- **PASS:** `diff -r` EMPTY across all 5 cases, BASE binary (`713f4339…`, built at
  `fb1ef2bc` pre-change) vs the FINAL-TREE binary (`7ab1f5f7…`) — differing binaries,
  so the empty diff is meaningful.
- **WF-0017 non-vacuity: DEMONSTRATED** — the `vault_init` success-marker string was
  deliberately perturbed; the differ went RED (exit 1) on exactly the two vault-init
  cases (diff preserved at proof root `red_demo_diff.txt`); the perturbation was
  reverted and the green re-proven. (Procedural note, recorded for honesty: the revert
  used `git checkout --` which also reverted the not-yet-staged B1 edits; they were
  re-applied identically — the re-built final binary is sha256-IDENTICAL to the
  pre-revert after-binary (`7ab1f5f7…` both), proving the re-application byte-exact.)

## §7 Gates (final tree)

- `cargo check -p qsc --all-targets`: **0 errors / 0 warnings.**
- Lane test file: **7 passed / 0 failed** (first green run after one compile fix:
  `expect_err` needs `Debug` on the Ok type, which `IdentityPublicRecord` deliberately
  does not gain — rewritten as matches; no product change).
- Full local `cargo test -p qsc` (niced, `--test-threads=3`): **412 passed / 0
  failed / 1 pre-existing-ignored across all 108 result sets, exit 0** — EXACTLY the
  NA-0646 baseline 405/0/1 across 107 sets (the directive's "NA-0644 baseline"
  phrasing predates the retire-TUI/core-extraction suite reshapes — pinned to the
  current baseline by the promotion block) + exactly the 7 new tests as the one new
  set (`NA_0649_gui_surface`: 7/0, 7.90s in-suite). The NA-0640 e2e within the run:
  **2 passed / 0 failed (118.30s)** — green UNCHANGED (zero e2e edits).
- Byte-identity: §6 (5/5 empty, red-demo non-vacuous).
- `cargo fmt --check`: the repo is NOT fmt-clean at base (~148 pre-existing
  deviations); the touched files: `src/identity/mod.rs`, `src/model/mod.rs`, and the
  new test file are fmt-CLEAN; `src/vault/mod.rs` had 13 pre-existing deviations in
  exactly the transcribed `vault_init` region at base, 8 remain after the verbatim
  transcription (the helper rename incidentally resolved 5; ZERO new deviations
  introduced — reformatting the inherited ones would have violated the verbatim
  discipline and widened the diff).
- `git diff --check` clean; `cargo metadata --locked` OK (Cargo.toml/Cargo.lock
  UNTOUCHED); root `cargo audit` 386 deps / 0 advisories; nested fuzz `cargo audit`
  287 deps / 0 advisories; `sh -n` + `bash -n` on `scripts/ci/qsc_adversarial.sh` OK.
- Push-run health on the base merge `fb1ef2bc` re-verified at Phase 0 (8/9 green,
  formal-ci then in progress; final state re-checked before the PR — see the response
  file).

## §8 Scope guard / boundaries held / NOT claimed

Changed files (complete): `qsl/qsl-client/qsc/src/vault/mod.rs`,
`qsl/qsl-client/qsc/src/identity/mod.rs`, `qsl/qsl-client/qsc/src/model/mod.rs` (the
operator-approved amendment, two words + a provenance comment),
`qsl/qsl-client/qsc/tests/NA_0649_gui_surface.rs` (new), this file, the testplan, and
the standard governance files (NEXT_ACTIONS, DECISIONS D-1272, TRACEABILITY, journal).
NO main.rs change; NO CLI flag/output/behavior change (byte-proven in the touched
neighborhoods); NO protocol/crypto/wire change; NO marker-vocabulary change (every code
string pre-existing); NO new deps, Cargo.toml/lock untouched; NO qsc-desktop touch
(retirement = NA-0650); NO retired-ingress / `QSC_DESKTOP_SESSION_PASSPHRASE` change
(their pinning tests untouched and green in the suite); NO GUI code.

NOT claimed: any GUI exists; any onboarding flow works end-to-end; the server-info
contract (separate cross-repo lane). Claim boundary UNCHANGED.
