# NA-0634 — D571 fold-ins (as-built): full-identity provisioning + required sig-pin + canonical combiner

Directive: QSL-DIR-2026-07-11-571 (D571, REV 4, APPROVED), Decision 2 (in-lane fold-ins). Completes the
C1 authenticated interim (NA-0633 shipped C1-only). Decision: D-1258. Wire/behavior/crypto/auth change,
fail-closed. Base main `9e717a59`. Scope held: **only `qsl/qsl-client/qsc/` was touched** — no Suite-2 core
(`establish.rs`), no Suite-2 vectors, no canonical/formal/qsp/server/attachments change.

## What was built (Decision 2)

### (a) Full-identity provisioning + `sig_fp` population
- The single verification code now binds **both** identity public keys:
  `identity_fingerprint_from_identity(kem_pk, sig_pk) = QSCFP- + hex(sha512(kem_pk ‖ sig_pk)[..16])`
  (`identity/mod.rs`). `identity_self_fingerprint`, `identity show`, `identity rotate` all emit this
  combined code; `identity show`/`rotate` also emit `identity_sig_pk=<hex>` (`main.rs`).
- `contacts add --fp <code> --kem-pk <hex> --sig-pk <hex>` verifies `fingerprint(kem_pk, sig_pk) == code`
  at add-time (fail-closed: `contacts_kem_pk_bad_hex` / `contacts_sig_pk_bad_hex` / `contacts_identity_fp_mismatch`;
  a signing key without the KEM key is refused: `contacts_sig_pk_requires_kem_pk`) and **POPULATES `sig_fp`**
  (= `fingerprint(sig_pk)`) on the contact + device record (`contacts/mod.rs`, `cmd/mod.rs`, `main.rs`). The
  `sig_fp`/`kem_pk` store fields already existed (NA-0633) — no store schema change.

### (b) Required responder sig-pin at B1
- New `hs_require_sig_identity_pin` (mirrors `hs_require_primary_identity_pin`): at the initiator's B1
  processing, `fingerprint(resp.sig_pk)` MUST equal the contact's populated `sig_fp`, else fail closed
  (`responder_sig_mismatch`); a **missing** pin is also a fail-closed reject (`responder_sig_unpinned`) —
  closing the ENG-0038 never-populated-`sig_fp` weakness (the old check was OPTIONAL and passed on `None`).
- The responder's primary pin now recomputes `identity_fingerprint_from_identity(init.kem_pk, init.sig_pk)`
  (the combined code) — so the responder→initiator direction binds the initiator's signing key via the
  combined primary pin too.

### (c) Canonical KDF combiner
- `hs_root_combine(domain, session_id, tag, contributions, ctx)` — one KMAC over an ordered, length-prefixed
  list of labeled contributions (fixed domain key `QSC.HS.ROOT.COMBINE.v1`, HKDF-extract style). `hs_pq_init_ss`
  now combines `[ss_pq, resp_kem_ss]`; `hs_dh_init_from_shared` combines `[dh_shared]` — replacing C1's
  incremental `resp_kem_ss` append. Both parties derive identically. It feeds the **unchanged** Suite-2 core
  (`init_from_base_handshake`), introduces **no** prekey/three-DH structure, and is structured so future X3DH
  DH products / prekey KEM secrets extend the list without a redesign (NA-0635 work — out of scope here).

## Design decisions (for review — some are judgment calls)
1. **Verification-code format change** (operator-acknowledged, correct pre-release): the code went from
   `fingerprint(kem_pk)` to `fingerprint(kem_pk, sig_pk)`. Consequence: existing contacts need re-provisioning,
   and a **legacy KEM-only identity's code CHANGES when it is upgraded to a full identity** (it gains a signing
   key) — while the KEM identity/secret is preserved. This surfaced as the `legacy_identity_public_record` test
   update (assert_eq → assert_ne).
2. **Responder→initiator sig-pin left OPTIONAL** (only initiator→responder became required). Rationale: the
   responder's primary pin now uses the combined code, which already binds `init.sig_pk`; the separate sig-pin
   is redundant. A strict reading of "retire the whole asymmetry class" might want it required too.
3. **Combiner uses a fixed-domain-key ordered list** (HKDF-extract style), NOT a literal copy of the Suite-2
   *chained*-KMAC style the directive referenced — chosen for the X3DH extensibility the directive also asked
   for. Different construction, both sound; the Suite-2 core is untouched either way.

## Scope caveat
- **`sha2` added to `qsc` `[dev-dependencies]`** (Cargo.toml) — test-only, so the wrong-signing-key negative
  can compute `fingerprint(bob.kem, mallory.sig)` in-test. Outside the 6-src + tests scope; no shipped-binary
  impact.

## Verification
- **`tests/NA_0634_full_identity_provisioning.rs`:** `na0634_positive_full_identity_roundtrip` (genuine full
  identity establishes) + `na0634_wrong_signing_key_rejected` (a responder that passes C1 with the correct KEM
  key but a mismatched signing key is REJECTED at B1 with `responder_sig_mismatch`, no session). The negative's
  framing is slightly contrived — alice pins a hybrid `{bob.kem, mallory.sig}` — because a genuine "stole the
  KEM key but not the signing key" responder can't be built via the CLI without identity-file hacks; it proves
  the property either way.
- **`tests/NA_0633_eng0038_reproduction.rs` stays green** (wrong responder REJECTED + genuine establishes); the
  initiator direction is not regressed.
- **Full `cargo test -p qsc`: 525 passed + the 3 initial failures resolved** — 1 real test-update
  (`legacy_identity_public_record…seam_inactive`, fixed) and 2 **load-flaky** e2e tests
  (`na_0617_attachment…fresh_session`, `two_client_local_runbook…is_honest`) that PASS clean single-threaded on
  a quiet box. No real regressions. (The full suite is skipped on PRs — this local run is the gate; CI
  `qsc-linux-full-suite` re-runs on main-push.)
- ~33 handshake/provisioning test files migrated to provision `--sig-pk` (script + manual for the
  differently-shaped files).

## Claim boundary — UNCHANGED
No post-compromise / PQ / "proven-secure" claim moves (still gated by the A1–A8 abstractions, the formal model,
and independent external review). The fold-ins make the shipped handshake meet DOC-CAN-003 §6.3 in BOTH
directions (both KEM + signing identity pinned to the single verified code); any bidirectional-auth wording is
claim-adjacent and fail-closed. This is the authenticated **interim** — the Signal-shaped prekey end-state is
D571 Decision 3 (NA-0635, GATED), not built here.
