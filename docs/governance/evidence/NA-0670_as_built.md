# NA-0670 — as built

**Lane:** NA-0670 · **Directive:** QSL-DIR-2026-07-23-606 (D606, approved 2026-07-23, sha256 `ef3bd063f936564935b86e806e459e2bc93be6a4b52fdba685611e43e8a177ca`, 284 lines) · **Decisions:** D-1297 (spine closeout) · D-0014 (qsl-server satellite) · **Finding:** 2026-07-22 independent audit **C-2** (the last unfixed HIGH, and the only one outside the governance spine).

**Result class:** `QSL_SERVER_C2_CONSTANT_TIME_BEARER_PASS`.

**Shape:** ONE qsl-server implementation PR (**#64**, merged `5235c2bf` over base `b4f86a3c`) — `src/lib.rs` + `DECISIONS.md` (D-0014) + `TRACEABILITY.md`, merge-commit only, single required `rust` check — then this SEPARATE spine governance closeout (D-1297).

---

## ⚠ 1. THE CI GREEN ON THE IMPLEMENTATION PR *IS* EVIDENCE — AND THE DISTINCTION IS THE POINT

The qsl-server implementation PR changed `src/lib.rs`, so `classify_ci_scope` returns runtime/non-docs and the required **`rust`** check RAN and could have gone RED. It went green on head `82df531` (`success`, 1m36s) — **run head SHA == PR head == the exact reviewed commit**, not a stale run. That green is **real evidence for the BEHAVIOURAL half**: auth still accepts the right token and rejects wrong ones, including the same-length case, under CI.

This is the **inverse** of:
- the qsl-server **queue-promotion** PR and this **spine closeout** PR — both `docs_only`, so their greens prove nothing about the code; and
- the four docs-only lanes NA-0664/0666/0667/0668, whose skipped suites made their greens empty.

### 1.1 — the structural claim is NOT measured, and is not claimed to be

The constant-time property itself — **fixed work over the full 32-byte digest, no data-dependent early return** — is a claim about **structure**, verified by **reading the code**. It is **not** a measured claim. No microbenchmark was run; a microbenchmark on a loaded build host would not reliably show the difference, and asserting it did would be the near-miss pattern this project records. **§3.4 held: no timing measurement was run or claimed.** Holding that line here is the near-miss rule working before there is a near-miss.

---

## 2. What landed (qsl-server `src/lib.rs`, one function + one helper + one test)

`auth_ok`'s final `provided == token` (`str::eq`, which short-circuits on the first differing byte) became `ct_eq_secret(provided, token)`. A new private helper was added, and one behavioural test. `DECISIONS.md` gained D-0014; `TRACEABILITY.md` gained the NA-0670 row. **`Cargo.toml`/`Cargo.lock` diff is empty.**

The three-caller reject-before-mutation structure is undisturbed: `auth_ok` remains the **first statement** of every gated handler (`server_info`, `push`, `pull`, `pull/ack`), so the two `_no_mutation` tests still assert that rejects precede any queue mutation.

---

## 3. The fix — SHA-256 both sides, then the in-house XOR-accumulate fold

The call site (with the recorded maintainer note on why NOT `subtle`):

```rust
            // Constant-time credential check. `str::eq` short-circuits on the first
            // differing byte -> a remote timing oracle on a network-exposed bearer
            // token (cf. the client's `hs_ct_eq_32`, ENG-0003, and this file's own
            // `route_key_for`). Reducing both sides to a fixed 32-byte digest first
            // also removes length dependence: the fold does identical work for every
            // input. NB: `subtle` is present in Cargo.lock only via rustls under the
            // reqwest DEV-dependency, so it is NOT in the production graph -- do not
            // reach for it here; `sha2` is already a direct dependency of this crate.
            ct_eq_secret(provided, token)
```

The helper:

```rust
/// Constant-time equality of two secrets, folded over a fixed 32-byte SHA-256
/// digest so the comparison leaks neither content nor length. Same XOR-accumulate
/// shape as the client handshake's `hs_ct_eq_32` (ENG-0003).
fn ct_eq_secret(a: &str, b: &str) -> bool {
    let da = Sha256::digest(a.as_bytes());
    let db = Sha256::digest(b.as_bytes());
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= da[i] ^ db[i];
    }
    diff == 0
}
```

- The loop iterates **all 32 bytes unconditionally**, ORs the per-byte XOR into `diff`, and only after the full loop returns `diff == 0`. **No early return; no data-dependent branch.**
- The idiom is the qsc client's `hs_ct_eq_32` (**ENG-0003**), and the relay's own `route_key_for` (`src/lib.rs:486`) already hashes the OTHER secret (the route token). So **the relay now treats both of its secrets the same way** — the strongest form of the fix (D606 §1b): not "a constant-time compare was added" but "the relay treats both secrets alike."

---

## 4. Why the in-house fold and NOT `subtle`, and NOT HMAC (recorded so the absences are decisions)

- **`subtle::ConstantTimeEq`** — declined (§2b/§2c). `subtle 2.6.1` is in `Cargo.lock` **only** via rustls under the reqwest DEV-dependency; a direct `[dependencies]` use would newly pull it into the shipped relay. Decisively, though: `ConstantTimeEq::ct_eq` is defined for **equal-length** slices, so on the raw `&[u8]` it would force a **length-visible branch** for unequal lengths — it would NOT close the length leak. Hashing-first closes it for free.
- **HMAC with a random per-process key** — declined (§2d). HMAC's random key defends against **precomputation**; here the attacker already holds the candidate token, and if they have one to precompute against they can simply **send** it. Plain SHA-256 delivers the property actually needed — comparison time independent of matching-prefix length — with no added machinery.

---

## 5. The length leak — CLOSED (a property of hashing first, not extra work)

Hashing normalises both inputs to 32 bytes, so the fold is length-independent for free. A bare `ct_eq` on the raw bytes would not close it (§4). **Residual, honestly:** `Sha256::digest(token)` runs in time proportional to the token's block count — a **constant per-deployment offset** on every request, not a per-guess oracle and not content-revealing. The refinement that erases even that (precompute at construction) is deferred and **filed as ENG-0063**, not folded in.

---

## 6. The one added test, and its precise limit

`auth_enabled_wrong_token_same_length_401_no_mutation`: configured token `"topsecret"`, provided `"topsecreX"` — **both 9 bytes**. Asserts push → 401 `ERR_UNAUTHORIZED`, then a correct-token pull → 204 (queue unmutated).

**Why it is the sharp one:** the pre-existing `auth_enabled_wrong_token_401_no_mutation` uses `"wrong"` (5 bytes) vs `"topsecret"` (9 bytes) — different lengths — so the old `==` rejected on **length** before comparing a single byte, and **that test passes against the buggy code**. The same-length case is the only behavioural test that exercises the fold. **Its limit, stated in-comment:** it proves the fold returns the right ANSWER (reject + no mutation), not that it runs in constant TIME.

---

## 7. Acceptance (D606 §3)

1. **Structural constant-time** — CONFIRMED by reading: fixed work over the full 32-byte digest, no data-dependent early return; the only early returns left are on non-secret header presence/format.
2. **Existing behaviour preserved under real CI** — the required `rust` check is GREEN on `82df531`. The four required tests pass unchanged (`auth_enabled_correct_token_allows_roundtrip`, `auth_enabled_wrong_token_401_no_mutation`, `auth_enabled_missing_token_401_no_mutation`, `auth_disabled_allows_push_pull`) plus the new same-length test. The two `_no_mutation` tests ARE the "rejects precede any queue mutation" acceptance.
3. **The one added test** — added, passes, limit recorded (§6).
4. **No timing proof required or claimed** — none run, none asserted (§1.1).
5. **Scope held** — one function; no file outside D606 §4; the only things filed are the closeout ENG lines (§10).

---

## 8. CI + local gate

- **CI (required `rust`):** `success`, 1m36s, head `82df531` (PR #64). `cargo test -q` on GitHub's 2-vCPU Linux runner.
- **Local gate** (`CARGO_TARGET_DIR=/srv/qbuild/cache/targets/qsl-server`, set explicitly per ENG-0064): `cargo fmt --check` clean; `cargo test` **14/14**; `cargo clippy --all-targets -- -D warnings` exit 0; `git diff Cargo.toml Cargo.lock` empty.

---

## 9. Phase 0 — anchors re-verified at the seating base `b4f86a3c`

| Anchor | D606 | Verified |
|---|---|---|
| `provided == token` | `src/lib.rs:507` | `:507` exactly |
| `use sha2::{Digest, Sha256};` | already imported | `:10` |
| `route_key_for` (hashes the route token) | `:486` | `:486`, `Sha256::digest` |
| `sha2` direct dep / `subtle` absent from `Cargo.toml` | yes / yes | `sha2 = "0.10"` at `:13`; `subtle` absent |
| DECISIONS top D-0013, D-0014 absent | yes | confirmed |
| `==` body-sweep: `:507` sole secret comparison | §1c | `lib.rs:47,55,62,192,672` + `store.rs:35,42,409,474` all numeric zero/threshold — `:507` is the only secret compare; **no second fix owed** |

---

## 10. The four ledger filings (none fixed — recording, not a second fix)

- **ENG-0063** — precompute `Sha256::digest(token)` at `AppState` construction (removes the secret from the per-request path; erases the block-count offset). Refinement, rides `new_with_auth*`. (D606 §5.)
- **ENG-0064** — the two-repo seat leaked `qsl-protocol`'s `CARGO_TARGET_DIR` into the `qsl-server` seat (classified inherited value as "explicit" and preserved it), so `qsl-server` built into `/srv/qbuild/cache/targets/qsl-PROTOCOL/…` and its registered per-repo cache went unused. First real exercise of the NA-0667 two-repo seat path. Worked around with an explicit per-build `CARGO_TARGET_DIR` pin; `qwork` NOT touched.
- **ENG-0065** — `tests::logs_do_not_contain_raw_channel` flakes under core contention (reads its buffer without synchronising on the server's on-response log). Measured: pristine 0/20, with-change 8/25 on 6 cores (always that test), 0/30 at 2 & 4 threads → CI reliably green. Surfaced not caused by the required new test. The "always passes where merges are decided" class.
- **ENG-0066** — `qsl-server` `TRACEABILITY.md` ends at NA-0012; D-0011/0012/0013 have no rows. Only the NA-0670 row was added; the three-row back-fill is filed, not done.

---

## 11. What this lane did NOT do

- Did **not** add any dependency, in particular **not `subtle`**; `Cargo.toml`/`Cargo.lock` byte-unchanged.
- Did **not** touch `route_key_for` or the route-token path (named as evidence, not a target), `src/store.rs`, `src/main.rs`, any route/wire/API/schema/env, any `.github/**`, or the `new_with_auth*` constructor chain.
- Did **not** fix the four filed items — they are refinements/defects for their own lanes.
- Did **not** run or claim any timing measurement.
- Did **not** back-fill the three missing qsl-server TRACEABILITY rows (ENG-0066) — out of scope.
- The executor did **not** self-promote; stopped at the open implementation PR, then (after the operator merged it) at this open closeout PR.
