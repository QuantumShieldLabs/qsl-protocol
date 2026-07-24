# NA-0671 — vault-KDF (Argon2id) derivations per relay operation: test plan and executed results

**Lane:** NA-0671 · **Directive:** QSL-DIR-2026-07-23-607 (D607), sha256 `57ec7b6977533ed83e8525296629b5737d6f6a7d31cd07aedecb9f1044d5af4b` · **Class:** MEASUREMENT (test/harness + evidence only) · **Decisions:** D-1298 (measurement, PR #1636 merged `3a8080b3` from head `b5c3be96`) + D-1299 (closeout).

---

## ⚠ 0. WHAT CI CAN AND CANNOT PROVE HERE

The measurement harness is `#[ignore]`-by-default and the qsc full suite **skips on PRs** (NA-0633). **A CI green is NOT the evidence — the captured numbers are.** The number is produced by a LOCAL `--release` run and recorded in `docs/governance/evidence/NA-0671_vault_kdf_cost.md`. CI compiles the harness (the `rust`/Analyze jobs) but does not run it.

---

## 1. Harness

- File: `qsl/qsl-client/qsc/tests/NA_0671_vault_kdf_cost.rs` (`#[ignore = "measurement harness"]`).
- Reuses `tests/common/mod.rs`: `start_qsl_server_with_store` (real in-process `qsl-server` relay, open or token, with a durable lease store), `init_mock_vault`, `qsc_std_command`.
- Run: `cargo test -p qsc --release --test NA_0671_vault_kdf_cost -- --ignored --nocapture` with `CARGO_TARGET_DIR` set explicitly (per ENG-0064).

## 2. Method (why in-process; setup vs measured op)

`vault::perf_snapshot()` reads **process-global** atomic counters, so a subprocess-driven CLI child's counters are unreadable from the parent — every existing full-relay-op e2e is subprocess-driven and cannot be read. Therefore: **subprocess CLI for the (unmeasured) session setup** (identity → `relay inbox-set` → contact + device-trust, then a warm-up round-trip and mailbox stocking, mirroring NA_0640), then **in-process `transport::send_execute` / `transport::receive_execute` for the MEASURED op**, each bracketed `perf_snapshot()` → op → `perf_snapshot()` + `std::time::Instant`. An in-process `send_execute` is behaviourally one cold `qsc send` (the CLI reads all state from disk), so this measures the shipped cold-per-command cost. Two real relays (open + token) with one authenticated pair each; the vault is `key_source==1` (passphrase), unlocked (`unlock_with_passphrase` + `set_vault_unlocked(true)`).

## 3. Validation discipline (executed)

- **Counter hazard (OBS-3):** `PERF_KDF_CALLS.fetch_add(1)` sits at the TOP of `derive_runtime_key` before the `match key_source`, so it counts **vault opens, not Argon2id hashes** — equal to the hash count only on the unlocked passphrase vault measured here.
- **Positive control:** one lone `secret_get` on a present key → **KDF-delta = 1, ~19.4 ms** (the instrument returns positive; a 0 or 0-ms result would have halted reporting).
- **Wall-clock cross-check (mandatory):** `wall_ms / 19.4` tracks the measured KDF count within ~3–6% for every op → every counted derivation is a real ~19 ms Argon2id; **no sub-millisecond no-op counts to exclude**; the timing governs on disagreement (it did not disagree).
- **Near-miss rule:** hypothesis was 7–9/SEND; measured 20 **refutes** it. Second look taken regardless — a fresh process reproduced every count identically; the open→token deltas match the token/token-file short-circuit mechanism.

## 4. PASS conditions (all met)

- [x] SEND, PULL, ACK measured **separately**, cold and warm, on a `key_source==1` unlocked vault.
- [x] Each cross-checked against wall-clock; no no-op counts; timing governs.
- [x] Amortisation verdict: **cold == warm** → nothing caches the derived key (§1c confirmed).
- [x] `timeline_append` isolated (2 derivations; included in SEND).
- [x] **Both** auth modes reported separately; absent-secret-probe count named per op (OBS-2).
- [x] Argon2id latency at **both** 19.4 ms and 41.1 ms; arithmetic shown; **no KDF parameter changed**.
- [x] Profile stated in plain words: **RELEASE**.
- [x] Reused scaffold **named**; F2 estimate settled ("standalone, none fit").
- [x] Evidence artifact carries every §4a plain-words requirement; `git add -f`.
- [x] No `src/` runtime change; `git diff Cargo.toml Cargo.lock` **empty**; no remedy shipped.

## 5. Executed results (RELEASE, passphrase vault, unlocked; reproduced across two fresh processes)

| operation | auth mode | KDF cold | KDF warm | wall ms (cold) | reads | decrypts | enc_writes | @19.4 ms | @41.1 ms |
|---|---|---|---|---|---|---|---|---|---|
| SEND | open | 20 | 20 | 412.8 | 20 | 20 | 3 | 388.0 | 822.0 |
| PULL | open | 15 | 15 | 300.8 | 15 | 15 | 1 | 291.0 | 616.5 |
| ACK (lease) | open | 18 | 18 | 364.9 | 18 | 18 | 1 | 349.2 | 739.8 |
| SEND | token (vault bearer) | 19 | 19 | 388.1 | 19 | 19 | 3 | 368.6 | 780.9 |
| SEND | token (env bearer) | 18 | — | 449.8 | 18 | 18 | 3 | 349.2 | 739.8 |
| PULL | token (vault bearer) | 14 | 14 | 419.6 | 14 | 14 | 1 | 271.6 | 575.4 |
| ACK (lease) | token (vault bearer) | 16 | 16 | 352.6 | 16 | 16 | 1 | 310.4 | 657.6 |

Isolated slices: positive control (present-key `secret_get`) = **1** derivation (~18–24 ms); a second consecutive `secret_get` = **1** (no amortisation); `timeline_append` (`get`+`set`) = **2** (~44–50 ms).

**Absent-secret probes (OBS-2):** SEND on an open relay = **3** absent probes (relay token / token-file / ca-file, all `None`, ~58 ms, ~15% of the send), confirmed by open→token-env (−2: token+token-file skipped) and open→token-vault (−1: token-file short-circuited).

## 6. Result

**VAULT_KDF_DERIVATIONS_PER_RELAY_OP_MEASUREMENT_PASS.** The audit's inferred 7–9/send was a ~2.5× undercount; the measured cost (SEND 20 ≈ 388 ms release) is past the top of the 90–180 ms "restore urgency" band. The number is the subsequent vault-decision lane's input, recorded — with the sequencing finding (cache/de-dup before raising Argon2id) and the three sized remedy classes — not started. No remedy ships in this lane.
