# NA-0670 — constant-time bearer-token comparison (audit C-2): test plan and executed results

**Lane:** NA-0670 · **Directive:** QSL-DIR-2026-07-23-606 (D606) · **Repo under test:** qsl-server (satellite) · **Implementation PR:** #64 (merged `5235c2bf` over base `b4f86a3c`).

---

## ⚠ 0. WHAT CI CAN AND CANNOT PROVE HERE

The qsl-server implementation PR changes `src/lib.rs`, so the required **`rust`** check RUNS (`cargo test -q`) and its green **is** evidence for the **behavioural** half — auth accepts the right token and rejects wrong ones, including the same-length case. **CI cannot prove the constant-TIME property**: that is a **structural** claim (fixed work over the full 32-byte digest, no data-dependent early return), verified by **reading** the code. **No timing measurement is run or claimed** (D606 §3.4). If one were ever run it would be indicative, not proof.

---

## 1. Harness

- Local: `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, run with `CARGO_TARGET_DIR=/srv/qbuild/cache/targets/qsl-server` set **explicitly** (per ENG-0064 — the two-repo seat otherwise leaks the spine's target dir).
- CI: `.github/workflows/ci.yml` job `rust` — `cargo fmt`, `cargo test -q`, `cargo clippy`. GitHub 2-vCPU Linux runner.

---

## 2. The behavioural change under test

`auth_ok`'s `provided == token` → `ct_eq_secret(provided, token)`:

```rust
fn ct_eq_secret(a: &str, b: &str) -> bool {
    let da = Sha256::digest(a.as_bytes());
    let db = Sha256::digest(b.as_bytes());
    let mut diff = 0u8;
    for i in 0..32 { diff |= da[i] ^ db[i]; }
    diff == 0
}
```

Structural acceptance (read-verified): the loop runs all 32 bytes unconditionally; `diff` accumulates by `|=`; the sole return is `diff == 0` **after** the loop. No early return, no data-dependent branch. `Sha256::digest` processes the full input (the acknowledged constant per-deployment offset, filed ENG-0063).

---

## 3. The added test — the only one that exercises the fold (§7.1)

`auth_enabled_wrong_token_same_length_401_no_mutation`: configured `"topsecret"`, provided `"topsecreX"` (both 9 bytes) → assert push 401 `ERR_UNAUTHORIZED`, then correct-token pull → 204 (queue unmutated).

**Why this is the sharp case, stated plainly:** the pre-existing `auth_enabled_wrong_token_401_no_mutation` uses `"wrong"` (5 bytes) vs `"topsecret"` (9 bytes). With the buggy `==`, `str::eq` compares length first and rejects **before comparing a single byte** — so that test **passes against the buggy code** and proves nothing about the fold. The same-length case forces the byte comparison. **Its limit: it proves the fold's ANSWER (reject + no mutation), not its timing.**

---

## 4. Regression — existing behaviour preserved (§7.1)

These pass **unchanged** (assertions not weakened):
- `auth_enabled_correct_token_allows_roundtrip` — right token accepted.
- `auth_enabled_wrong_token_401_no_mutation` — wrong token 401, queue unmutated.
- `auth_enabled_missing_token_401_no_mutation` — missing header 401, no mutation.
- `auth_disabled_allows_push_pull` — `relay_token = None` still open.

The two `_no_mutation` tests ARE the "rejects precede any queue mutation" acceptance (D606 §3.2).

---

## 5. Scope verification (§7.2)

- `git diff Cargo.toml Cargo.lock` — **empty** (no dependency change; `subtle` NOT added).
- Changed files: exactly `src/lib.rs`, `DECISIONS.md`, `TRACEABILITY.md`.
- `route_key_for` / route-token path, `src/store.rs`, `src/main.rs`, `.github/**`, `new_with_auth*` — untouched.
- Phase 0: `provided == token` was at `src/lib.rs:507` at the seating base `b4f86a3c`; the `==` body-sweep confirmed `:507` the sole secret comparison (no second fix owed).

---

## 6. ⚠ The pre-existing flaky test surfaced by this lane (ENG-0065) — measured, not hand-waved

`tests::logs_do_not_contain_raw_channel` is **unrelated to `auth_ok`** (it checks that request logging redacts the raw channel). It flakes under high core-count contention because it reads its capture buffer immediately after `handle.abort()` without synchronising on the server task having emitted its on-response `channel_id=` log.

| Condition | Failures | Note |
|---|---|---|
| pristine base `b4f86a3c` (13 tests), parallel | **0 / 20** | flake not reproduced without the change |
| with NA-0670's test (14 tests), parallel, 6 cores | **8 / 25** | always `logs_do_not_contain_raw_channel`, never an auth test |
| single-threaded (`--test-threads=1`) | **0** | deterministic |
| the log test in isolation | **0** | passes alone |
| `RUST_TEST_THREADS=2` (GitHub 2-vCPU runner) | **0 / 30** | CI-relevant parallelism |
| `RUST_TEST_THREADS=4` | **0 / 30** | |

**Conclusion:** the added 14th server-spawning test raises parallel CPU load past a threshold **on a 6-core box only**, surfacing a latent race in a different test; CI runs at ≤ 2 vCPU, where the required `rust` check is **reliably green**. Surfaced, not caused. Out of this lane's scope to fix (touch only `auth_ok`/helper/the one test); filed as ENG-0065.

---

## 7. Executed suite results

### 7.1 Local `cargo test` (14 tests)
- Single-threaded: **14 passed, 0 failed** — including the new same-length test and the four required auth tests.
- `cargo fmt --check`: clean. `cargo clippy --all-targets -- -D warnings`: exit 0. Dependency diff: empty.

### 7.2 CI required `rust` check
- PR #64, head `82df531`: **`success`, 1m36s** — run head SHA == PR head == the exact reviewed commit. This green is evidence for the behavioural half.

---

## 8. What this plan does not cover

- The constant-TIME property by measurement — deliberately not attempted; structural/read-verified only.
- Directory-durability or power-loss behaviour — not in scope (that was NA-0669's C-6).
- The ENG-0063 precompute refinement, the ENG-0064 seat fix, the ENG-0065 test-synchronisation fix, and the ENG-0066 TRACEABILITY back-fill — all filed, none done here.
