# NA-0671 — Vault-KDF (Argon2id) derivations per relay operation (D-1298, directive 607)

**Class:** MEASUREMENT. This lane ships no runtime change and no remedy. **A green CI check is
not the evidence — the captured numbers below are.** (The qsc full suite SKIPS on PRs per
NA-0633; the numbers come from a LOCAL `--release` run.)

**Harness:** `qsl/qsl-client/qsc/tests/NA_0671_vault_kdf_cost.rs`
(`#[ignore]`-by-default; stands up two real in-process `qsl-server` relays).

**Reproduce:**
```
cargo test -p qsc --release --test NA_0671_vault_kdf_cost -- --ignored --nocapture
```

---

## 0. Headline (decision-bearing)

On the shipped parameters (Argon2id m=19456, t=2, p=1 ≈ **19.4 ms/derivation, RELEASE**), a
single relay operation performs, against an **open** relay:

| operation | KDF derivations (measured) | Argon2id @19.4 ms | Argon2id @41.1 ms (OWASP min) |
|---|---|---|---|
| **SEND** | **20** | **388 ms** | 822 ms |
| **PULL** (legacy) | **15** | **291 ms** | 616.5 ms |
| **ACK** (lease pull+ack) | **18** | **349 ms** | 739.8 ms |

> **The 2026-07-22 audit inferred 7–9 derivations/send (~135–175 ms). The instrument says 20
> (~388 ms) — a ~2.5× UNDERCOUNT.** The measured send-latency is not merely inside the 90–180 ms
> "would restore urgency" band the NA-0664 handoff named; it is **more than double the top of it.**
> The whole vault decision (envelope encryption vs session-key caching vs raising Argon2id to the
> OWASP minimum) must be sized against **20 / 15 / 18**, not 7–9. **41.1 ms is a multiply-by
> constant (OWASP minimum); NO KDF parameter was changed in this lane.**

Numbers are **stable and reproduced across a fresh process** (two independent runs, identical
counts) and **cross-checked against wall-clock** (§4). Cold == warm for every op — **nothing
amortises** (§5).

---

## 1. The counter hazard, named — why "KDF calls" ≠ "Argon2 cost" (OBS-3; this paragraph is load-bearing)

`PERF_KDF_CALLS.fetch_add(1)` (`vault/mod.rs:790`) sits at the **TOP of `derive_runtime_key`,
BEFORE the `match env.key_source`**. It therefore counts **vault OPENS (entries to
`derive_runtime_key`), NOT Argon2id hashes.** A keychain vault (`key_source==2`) or a locked
early-return bumps the counter while doing **zero Argon2id work** — exactly the NA-0664 pattern
(`PERF_VAULT_FILE_READS` at `:758` likewise counts attempts, not successes).

**Consequence, and what this lane did about it:** the count equals the Argon2id count **only** on a
`key_source==1` (passphrase) vault that is **unlocked** so every entry reaches the hash
(`:804`). This lane measured **exactly that vault** (§2) and **cross-checked every delta against
wall-clock** (§4): every counted derivation cost ~18–20 ms of wall time — a real Argon2id — so
there were **no sub-millisecond no-op "KDF calls" to exclude.** **A future macOS/keychain-side
measurement must NOT reuse these counts as Argon2 cost** — on a `key_source==2` vault the same
counter would report opens that never hash. This paragraph lives here, in the evidence that
qualifies it; it is **not** filed as an ENG.

---

## 2. The vault measured, stated explicitly

**`key_source==1` (passphrase), UNLOCKED.** Created in-process via
`vault_init_with_passphrase`-equivalent (`qsc vault init --key-source passphrase`), unlocked in
the measuring process via `unlock_with_passphrase` + `set_vault_unlocked(true)`,
`QSC_DISABLE_KEYCHAIN=1`. **A keychain vault was NOT used** — it would make the counter lie (§1).

**Profile: RELEASE.** `cfg!(debug_assertions)` reported `RELEASE`; the per-derivation cost is
~19.4 ms (a debug build would be ~an order of magnitude slower — the exact confusion NA-0664 turned
on). Every number in this file is a release number.

**Why in-process (F2, the subprocess constraint settled):** `perf_snapshot`'s counters are
PROCESS-GLOBAL atomics. Every existing full-relay-op e2e
(`NA_0640_full_stack_e2e`, `same_host_client_to_client_e2e`, `receive_e2e`, `NA_0644_ack_client`)
drives the qsc CLI as **spawned subprocesses**, whose counters die with the child and are
unreadable from the parent. The MEASURED operation therefore calls the qsc **library**
(`transport::send_execute`, `transport::receive_execute`) inside the test process; the one-time
SETUP (identities, trust, warm-up) uses the CLI subprocesses — its derivations are not counted.
An in-process `send_execute` is behaviourally identical to one cold `qsc send` (the CLI reads all
state from the config dir on disk), so **this measures the shipped cold-per-command cost.**

**Scaffold named (F2 estimate settled against reality):** **NO existing scaffold fit.** The only
in-process passphrase-vault scaffold (`NA_0649_gui_surface.rs`) exercises only the vault + identity
surface, never a relay operation. This harness **reused `tests/common/mod.rs`**
(`start_qsl_server_with_store`, `init_mock_vault`, `qsc_std_command`) plus the NA_0640-style
subprocess setup, and added a **new** in-process send/pull/ack driver. Total file ≈ 370 lines —
**the "standalone, none fit" class F2 anticipated (~100–150+ lines), NOT the ~30-line reuse case.**

---

## 3. The measurement table (per SEND / PULL / ACK, per auth mode, cold + warm — NOT aggregated)

RELEASE. `reads`/`decrypts`/`enc_writes` are the sibling `perf_snapshot` counters. `@19.4` and
`@41.1` = `KDF × per-derivation-ms`. Wall figures are representative (run 1); counts are identical
across both runs.

| operation | profile | auth mode | KDF cold | KDF warm | wall ms (cold) | file_reads | decrypts | enc_writes | @19.4 ms | @41.1 ms |
|---|---|---|---|---|---|---|---|---|---|---|
| SEND | release | open | 20 | 20 | 412.8 | 20 | 20 | 3 | 388.0 | 822.0 |
| PULL | release | open | 15 | 15 | 300.8 | 15 | 15 | 1 | 291.0 | 616.5 |
| ACK (lease) | release | open | 18 | 18 | 364.9 | 18 | 18 | 1 | 349.2 | 739.8 |
| SEND | release | token (vault-stored bearer) | 19 | 19 | 388.1 | 19 | 19 | 3 | 368.6 | 780.9 |
| SEND | release | token (bearer via `QSC_RELAY_TOKEN` **env**) | 18 | — | 449.8 | 18 | 18 | 3 | 349.2 | 739.8 |
| PULL | release | token (vault-stored bearer) | 14 | 14 | 419.6 | 14 | 14 | 1 | 271.6 | 575.4 |
| ACK (lease) | release | token (vault-stored bearer) | 16 | 16 | 352.6 | 16 | 16 | 1 | 310.4 | 657.6 |

**Isolated slices (measured):**

| slice | KDF | wall ms | note |
|---|---|---|---|
| positive control — one lone `secret_get` (present key) | 1 | ~18–24 | §4 |
| positive control — a **second** consecutive `secret_get` | 1 | ~18 | §5 (no amortisation) |
| `timeline_append` (a `secret_get` + `secret_set` on `timeline.json`) | 2 | ~44–50 | §7 |

---

## 4. Instrument validity — positive control + counter-vs-timing consistency

**Positive control (a negative result is only evidence if the instrument could return positive):**
a single lone `secret_get` on a **present** key showed **KDF-delta = 1** and **~19.4 ms wall** in
every run (open: 24.2/17.7 ms; token: 17.8 ms). The instrument returns positive; it is not stuck
at zero. **Had it shown 0, or 0 ms, no operation numbers would have been reported.**

**Counter-vs-timing cross-check (mandatory):** for every operation, `wall_ms / 19.4` tracks the
measured KDF count within ~3–6%:

| op (open) | measured KDF | implied KDF (wall/19.4) |
|---|---|---|
| SEND | 20 | 21.3 / 20.6 |
| PULL | 15 | 15.5 / 15.2 |
| ACK | 18 | 18.8 / 18.5 |

The small excess of wall-time over `KDF×19.4` is the non-Argon2 work (HTTP to the in-process relay,
serialization, ratchet crypto). **No counted "KDF call" cost sub-millisecond wall time — there were
NO no-op counts to exclude, and none were averaged in.** Counter and timing **agree**; had they
disagreed anywhere, **the timing would govern** — it did not need to.

---

## 5. Amortisation verdict (cold vs warm) — §1c confirmed by measurement

For **every** operation, **cold == warm** (SEND 20=20, PULL 15=15, ACK 18=18; token likewise),
and two consecutive lone `secret_get`s each cost **1** derivation (~19 ms each). **Nothing caches
the derived 32-byte key within a process** — `PROCESS_PASSPHRASE` caches only the passphrase, and
`load_vault_runtime` re-derives on every call. Therefore:

- The **CLI-cold** cost (the shipped product: one process per command) = the full count per op.
- A hypothetical **long-lived GUI/daemon** would amortise **nothing** — a warm process still pays
  20/15/18 per op. **A session-key cache or envelope encryption is the only thing that changes
  this number** (next lane's input, not this lane's work).

---

## 6. Absent-secret probes — the NAMED sub-result (OBS-2), per operation

On an **open** relay the client still resolves a relay auth token and CA file, and **each get
returns `None` yet pays a full ~19.4 ms Argon2id.** Confirmed empirically by the auth-mode deltas
(not asserted from the code):

- **SEND, open: 3 absent-secret probes** — `secret_get("tui.relay.token")` = None,
  `secret_get("tui.relay.token_file")` = None, `secret_get("tui.relay.ca_file")` = None. That is
  **3 × 19.4 ms = 58.2 ms of "probing for what isn't there," ≈ 15% of the send's KDF time.**
  - open → token (vault bearer): **20 → 19** (−1): the token now resolves present, so the
    token-file probe is **short-circuited** (`if let Some(token) = account_secret …`).
  - open → token (env bearer): **20 → 18** (−2): the env token short-circuits **both** the token
    and token-file vault probes; only the CA-file absent probe remains.
- **PULL, open: 3 absent probes** (same pattern; open 15 → token 14 = −1 token-file short-circuit).
- **ACK, open:** re-resolves the auth token on **both** the pull and the ack leg (open 18 → token
  16 = **−2** short-circuits), so the ack path adds a second absent-probe set.

**Why this is a first-class number, not a footnote:** "don't probe for what isn't there" is a
**distinct, cheaper remedy class** than the vault-format work — it removes ~3 derivations/op with
**zero** vault-format change. **But note the ceiling:** eliminating all 3 absent probes takes SEND
from 20 → 17 (388 → 330 ms). It is a cheap partial, not a fix; the cost is dominated by the
**present**-secret re-derivations (contacts/session/timeline), which only key-caching removes (§8).

---

## 7. `timeline_append` isolated

A standalone `secret_get("timeline.json")` + `secret_set("timeline.json", …)` — the exact pair
`timeline_append_entry` performs internally — costs **2 derivations (~44–50 ms)**. SEND's
`finalize_send_commit` calls `timeline_append_entry_for_target`, so **~2 of the SEND's 20
derivations (10%, ~39 ms) are the timeline append** — a genuine "second cold vault open inside
SEND," confirming the §2.4 suspicion. (The `enc_writes=3` on SEND vs `enc_writes=1` on PULL is
consistent with SEND persisting session + outbox-state + timeline.)

---

## 8. Mechanism — which sites account for the count

Each derivation is one independent `load_vault_runtime → derive_runtime_key → Argon2id`; the
positive control proves each `secret_get`/`secret_set` = exactly 1. The SEND path traverses these
vault sites (from the source map; the **total 20 and the auth=3 / timeline=2 slices are measured**):

- **trust + block gates:** `enforce_cli_send_contact_trust`, `enforce_peer_not_blocked` →
  `contacts` loads (`contacts.json`, `contact_requests.json`).
- **session-active gate:** `protocol_active_or_reason_for_send_peer` → session load
  (`qsp_session_store_key_v1`).
- **pack:** `qsp_pack` → session load/advance.
- **relay client build:** `relay_auth_token` (token + token-file) + `relay_ca_file` (ca) = the 3
  absent probes (§6).
- **finalize:** `qsp_session_store` (session key load + store) + `timeline_append` (get + set) +
  send-state (file, not vault).

**Why 20 >> the audit's 7–9:** the gates, pack, and finalize each **independently re-load**
contacts/session with **no request-scoped cache**, so the same logical secret is derived several
times per send. The static audit counted logical secrets; the instrument counts **actual
`derive_runtime_key` entries.** The redundant re-loads are the gap.

---

## 9. Near-miss disposition (hypothesis was 7–9 → measured 20; the hard case handled)

The armed hypothesis was **7–9 derivations/SEND**. The first pass measured **20** — a **refutation**,
not a confirmation (the safer direction, but still checked). The **second look under changed
conditions** was taken **before** relying on the number:

1. **Fresh process:** re-ran the whole harness; every count reproduced **identically** (20/15/18
   open; 19/14/16 token).
2. **Changed operation & auth mode:** the open→token deltas (20→19→18, 15→14, 18→16) match the
   token/token-file short-circuit **mechanism exactly** — the count is mechanism-driven, not
   coincidental.
3. **Cold vs warm:** identical, ruling out any process-warmup artefact.
4. **Wall-clock:** `wall/19.4` tracks the count within ~3–6% (§4) — the derivations are real
   Argon2id, not counter inflation (a double-count would show ~800 ms; SEND showed ~412 ms).

The number is **stable and mechanism-explained.** (Directive §8's specific STOP — "SEND measures
7/8/9 and the second look has not been done" — did not trigger; the measured value is 20, and the
second look was done regardless.)

---

## 10. Implications — the NEXT lane's input (findings only; this lane ships no remedy)

1. **Size the vault decision against 20 / 15 / 18**, not 7–9. Per-SEND Argon2id ≈ **388 ms**
   (release, current params) and would be **822 ms** at the OWASP minimum. This exceeds the
   "urgency" band; raising Argon2id **without** first removing the redundant re-derivations would
   push a send toward ~0.8 s.
2. **Three remedy classes, now sized:**
   - **(a) Session-key cache / envelope encryption** — memoise the derived key (or decrypt the
     vault once per process/op). Removes the per-`secret_get` re-derivation entirely → collapses
     SEND from 20 toward ~1. **Addresses the whole cost.** Highest leverage.
   - **(b) Request-scoped de-duplication of loads** — the gates/pack/finalize re-load
     contacts/session several times; a per-operation cache collapses the ~20 to a handful without a
     vault-format change. **Middle cost, middle leverage.**
   - **(c) "Don't probe for what isn't there"** (§6) — skip the relay token/token-file/ca-file gets
     when unconfigured. ~3 derivations/op, ~58 ms/send, zero format change. **Cheapest, but a
     partial** (20 → 17).
3. **Auth mode barely moves the needle** (open 20 vs token 18–19): the cost is dominated by the
   **present-secret** re-derivations, so absent-probe elimination (c) alone is insufficient. The
   core defect is **no key caching** (§5).

---

## 11. Scope closeout

- **No `src/` change.** `secret_get`/`secret_set`/`load_vault_runtime`/`derive_runtime_key`, the
  `PERF_*` counters, `PROCESS_PASSPHRASE`, and `KDF_M_KIB`/`KDF_T`/`KDF_P` are untouched.
- **No dependency change** (`Cargo.toml`/`Cargo.lock` diff empty).
- **No remedy shipped.** Caching / envelope encryption / KDF-parameter changes are explicitly the
  next lane's input, recorded above as findings.
- **Anchors re-verified at the seating base** (`vault/mod.rs:335` `perf_snapshot`, `:790`
  `PERF_KDF_CALLS.fetch_add` inside `derive_runtime_key`, `:37-39` KDF params) — exact.
