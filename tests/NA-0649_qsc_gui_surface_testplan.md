# NA-0649 testplan — qsc GUI-surface lane (D585, D-1272)

Lane: NA-0649 (QSL-DIR-2026-07-16-585, operator-issued; seated by promotion PR #1579).
Base: main `fb1ef2bc` from a fresh clone. All product assertions are LIBRARY-LEVEL per
D585; the compiled-binary suite is untouched (zero edits to existing test files); the
CLI byte-identity spot-check is a lane-run capture harness (proof-root), not a
committed test.

## T1 — D585 test group 1: `vault_init_with_passphrase` (library)

Vehicle: `qsl/qsl-client/qsc/tests/NA_0649_gui_surface.rs` (new; the only test-file
change in the lane).

- T1a roundtrip: in-process init creates a vault a subsequent `unlock_with_passphrase`
  opens; a wrong passphrase is rejected (`vault_locked`); the existing `vault_init`
  success marker is emitted (observed via the KEEP InApp queue); the default inbox
  route token (`tui.relay.inbox_token`) is seeded non-empty.
  **Result: PASS** (`vault_init_with_passphrase_roundtrip_no_unlock_side_effect`).
- T1b NO unlock side effect: `has_process_passphrase()` false AND `vault_unlocked()`
  false after init alone (asserted in T1a before any unlock call). **Result: PASS.**
- T1c second call: `Err("vault_exists")` as a value. **Result: PASS**
  (`vault_init_with_passphrase_second_call_vault_exists`).
- T1d empty passphrase: `Err("vault_passphrase_required")`, no mutation on reject.
  **Result: PASS** (`vault_init_with_passphrase_empty_passphrase_rejected`).

## T2 — D585 test group 2: `identity_ensure` (library)

- T2a fresh unlocked store creates; the fingerprint matches the subsequent
  `identity_show` marker; the second call returns the SAME record with store-bytes
  stability (byte-compared `identities/self_default.json`). **Result: PASS**
  (`identity_ensure_creates_then_is_idempotent`).
- T2b locked store → the EXISTING vault_locked behavior:
  `ErrorCode::IdentitySecretUnavailable` (`identity_secret_unavailable`), the existing
  marker with `vault_locked`, and ZERO mutation (no record minted). **Result: PASS**
  (`identity_ensure_locked_store_keeps_existing_vault_locked_behavior`).
- T2c second-identity guard preserved EXACTLY: a different label is refused with
  `ErrorCode::IdentitySelfAmbiguous` + the `identity_self_ambiguous` marker (NA-0616).
  **Result: PASS** (`identity_ensure_preserves_second_identity_guard`).

## T3 — D585 test group 3: widened accessors, external-crate-shaped

- The integration test (external crate by construction) reads {fp, kem_pk, sig_pk} and
  the verification code as VALUES via `identity_read_self_public`,
  `identity_fingerprint_from_identity`, `format_verification_code_from_fingerprint`,
  `IdentityPublicRecord`'s pub fields; verifies the code shape (4-4-4-4-checksum,
  Crockford); cross-checks the fp against the `identity_show` marker; and MATCHES
  `ErrorCode` externally (`Err(ErrorCode::ParseFailed)` on an invalid label — the
  operator-approved amendment demonstrated). **Result: PASS**
  (`widened_accessors_expose_identity_as_data`).

## T4 — D585 test group 4: CLI byte-identity spot-check (lane-run, reduced prover)

Vehicle: proof-root `na0649_reduced_prover.sh` (the NA-0646 prover pattern reduced to
the touched neighborhoods; a full 14-case run not required per D585). Corpus: v1
`vault init --non-interactive --key-source passphrase --passphrase-file` (fresh store),
v2 `vault init --passphrase-file` (fresh store), v3 `vault_exists` negative (fixture),
i1 `identity show` (unlocked fixture), r1 `relay token-set` (unlocked fixture);
stdout+stderr+exit captured under `env -i LC_ALL=C TZ=UTC`; ONE fixture created with
the BASE binary and reused so key material is identical.

- **Result: PASS — `diff -r` EMPTY across all 5 cases** (base binary `713f4339…` at
  `fb1ef2bc` vs final-tree binary `7ab1f5f7…`; binaries differ, so the comparison is
  non-vacuous).
- **WF-0017 non-vacuity: PASS** — deliberate success-marker perturbation turned the
  differ RED (exit 1) on exactly the two vault-init cases (`red_demo_diff.txt`
  preserved); reverted; green re-proven on the final tree.

## T5 — Repo gates (final tree)

- `cargo check -p qsc --all-targets`: 0 errors / 0 warnings. **PASS.**
- Full local `cargo test -p qsc` (niced, `--test-threads=3`): **412 passed / 0 failed
  / 1 pre-existing-ignored across all 108 result sets, exit 0** — the NA-0646 baseline
  (405/0/1 across 107 sets) + exactly the 7 new tests as the one new set; the NA-0640
  e2e green UNCHANGED within the run (**2 passed / 0 failed, 118.30s**; zero e2e
  edits). **PASS.**
- Scope guard: changed paths ⊆ the D585 allowed list + the operator-approved
  `src/model/mod.rs` amendment (two-word visibility widening only). NO main.rs, NO CLI
  behavior, NO marker vocabulary, NO deps/lockfile, NO qsc-desktop, NO
  retired-ingress/`QSC_DESKTOP_SESSION_PASSPHRASE`, NO GUI code. **PASS.**
- Validation defaults: `git diff --check` clean; `cargo fmt --check` — touched files
  fmt-clean except the 8 deviations INHERITED verbatim in `src/vault/mod.rs` (13 at
  base in the same region; zero new); root audit 386 deps / 0 advisories; fuzz audit
  287 deps / 0 advisories; `cargo metadata --locked` OK; `sh -n`/`bash -n`
  `scripts/ci/qsc_adversarial.sh` OK. **PASS.**

Raw run logs and captures: lane proof root
`/srv/qbuild/tmp/NA0649_qsc_gui_surface_20260716T221842Z/` (proof-root-only; class
summaries here per policy).
