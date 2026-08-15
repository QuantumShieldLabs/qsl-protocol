# NA-0736 — AS BUILT — THE RECEIVE-SIDE DELIVERY LANE

**Lane:** NA-0736 · **Decision:** D-1371 · **Ruling:** R335 · **Base:** main `5201c275`, verified
UNMOVED by `git ls-remote` against the **NAMED** GitHub remote, run **bare and unpiped**, rc 0,
exactly one line; open-PR set **measured EMPTY**.
**Class:** FILING ONLY — **zero product source bytes**, no script, no workflow, no test.

⚠ **REDACTION, STATED UP FRONT.** Concrete relay route tokens and the sealed run's `run_tag` are
**not reproduced in this file**. A route token is an addressing capability; `AGENTS.md:121` makes
governance records class-only; and `qsc` itself redacts `mailbox=redacted` / `from=<redacted>` in the
markers this lane reads, precisely because those markers upload as public CI artifacts. **Shapes and
hashes land here; instances live in the sealed operator stop** (`STOP_NA0736_001`, sha256
`b69e81a12eaf64ae61c85cfdd01d12b68b3f7e79070858175bd0a6143db45f70`), from which every digit below is
recomputable.

---

## 1. THE QUESTION AND THE ANSWER

**Question (Director's brief §3):** a frame the relay demonstrably holds is not delivered to the peer
that owns the mailbox — did the client's pull return **zero items**, or did it return items that were
**filtered out before unpack**?

**Answer: ZERO ITEMS.** `receive` polled a mailbox nothing had ever pushed to. Nothing was filtered,
nothing was destroyed, and the product behaved correctly at every step.

---

## 2. THE ADDRESSING ASYMMETRY — three commands, three rules

| command | how it names the mailbox | source |
|---|---|---|
| `send --to <peer>` | `relay_peer_route_token(to)` — resolves the **contact's** stored route token | `qsc/src/lib.rs:1565`, `:1592` → `qsc/src/contacts/mod.rs:70-81` |
| `handshake poll` | `relay_self_inbox_route_token()` — the vault's own inbox token. **No override exists.** | `qsc/src/handshake/mod.rs:2422-2425` |
| `receive` | `--mailbox` present ⇒ **the raw argument, verbatim**; absent ⇒ `relay_self_inbox_route_token()` | `qsc/src/transport/mod.rs:257-263` |

```
:257   let mailbox = match mailbox {
:258       Some(raw) => normalize_route_token(raw.as_str()).map_err(|code| CliError::code(code))?,
:260       None => relay_self_inbox_route_token().map_err(|code| CliError::code(code))?,
:263   };
```

`relay inbox-set --token` writes `tui.relay.inbox_token` (`qsc/src/main.rs:544`), which is exactly
the key `relay_self_inbox_route_token()` reads (`qsc/src/contacts/mod.rs:60-68`).

⚠ **The shape of the mistake.** On one line of `scripts/demo/qsc_remote_handshake_smoke.sh`, `--to`
and `--from` take a peer **LABEL** (resolved through contacts) while `--mailbox` takes a raw **ROUTE
TOKEN** (used as-is). `send --to "$proto_bob"` is correct; `receive --mailbox "$proto_bob"`
(`:375`, and `:388` for the mirror direction) is not. Two argument namespaces that look identical,
adjacent, with no type distinction and no error.

---

## 3. THE PROOF BY HASH

The `recv_start` marker publishes `route_token_hash8(mailbox)` = `hex(sha512(mailbox)[0..4])`
(`qsc/src/contacts/mod.rs:5-9`) of the **resolved** value. Computed over the four candidates at the
sealed run's `run_tag`, and **recomputed independently by the Director** (R335 §0):

| hash | preimage shape | appears in the sealed logs? |
|---|---|---|
| `f4c89d20` | `bob-${run_tag}` — the identity **LABEL** | ✅ **== `bob_recv.log` `mailbox_hash`** |
| `f9fa4170` | `route_token_bob_${run_tag}` — the **ROUTE TOKEN** | ❌ absent from every log |
| `a53c4170` | `alice-${run_tag}` — the identity **LABEL** | ✅ **== `alice_recv.log` `mailbox_hash`** |
| `f20f7f9f` | `route_token_alice_${run_tag}` — the **ROUTE TOKEN** | ❌ absent from every log |

⚠ **Compare all eight digits.** `a53c4170` and `f9fa4170` share the trailing `4170`; a four-digit
comparison crosses the two candidates.

**And the sender's side is proven by the diagnostic's own success:** the predecessor lane's raw
`GET /v1/pull`, made with the bearer and each **route token**, returned exactly one item per mailbox.
Had the sends addressed the labels, that pull would have found nothing.

---

## 4. THE FORK, CLOSED BY EXHAUSTIVE ELIMINATION

`recv_none` fires at `qsc/src/transport/mod.rs:428` only when `total == 0`; the pull loop breaks at
`:517-518` on `items.is_empty()`. **Every path an item can take emits a marker before it can be
dropped:**

| per-item path | marker it must emit | site |
|---|---|---|
| dedup skip (already seen) | `recv_dup_skipped` | `:527` |
| unpack succeeds | `qsp_unpack ok=true` | `:544` |
| unpack fails | `qsp_unpack ok=false code=…` | `:1199` |

Both sealed receive logs read, in full: `receipt_policy` → `session_load ok=true` → `recv_start …
mailbox_hash=… max=1` → `recv_ack_mode mode=lease` → `recv_none`. `recv_ack_mode` is emitted at
`:349`, **before** the pull; `recv_none` at `:428`, after. **Nothing between them**, at **rc 0** —
so no error path was taken either (an `Err` at `:515` would propagate through `?` and exit non-zero).
⇒ **the item loop never executed.**

⇒ **ENG-0142 is unreachable in this run** — its region (`:1147-1211`) sits *inside* that loop.
⇒ **ENG-0134 is refuted** — its recorded mechanism is an *aborting* pull; this pull completed.

---

## 5. THE DIFFERENCE TABLE — product `receive` vs the raw diagnostic pull

The wire call is fully specified at `qsc/src/transport/mod.rs:3018-3063`.

| # | axis | product `receive` | raw diagnostic | verdict |
|---|---|---|---|---|
| **1** | **route-token VALUE (the mailbox key)** | the identity **LABEL**, verbatim from `--mailbox` | the **ROUTE TOKEN** | ⚠⚠ **LIVE — THE CAUSE** |
| 2 | header name | `X-QSL-Route-Token` (`:3040`) | `x-qsl-route-token` | EXCLUDED — HTTP field names are case-insensitive; both got HTTP 200 |
| 3 | endpoint + API version | `GET {base}/v1/pull?max=1&ack=lease` (`:3031`) | `GET /v1/pull` | EXCLUDED — identical |
| 4 | ack / lease mode | `ack=lease`, proven by `recv_ack_mode mode=lease` in both logs | `pull_ack_lease_v1` | EXCLUDED — identical |
| 5 | bearer auth | `Authorization: Bearer …` (`:3041-3043`) | bearer | EXCLUDED — identical; a bad bearer yields 401/403 ⇒ `relay_unauthorized` (`:3057`), an **error**, not silence |
| 6 | `max` | `max=1`; `want = max.saturating_sub(0).max(1)` (`:512`) | — | EXCLUDED — one item was present; `max=1` returns it |
| 7 | `--from` filter | consumed at `:321` (protocol-active, passed) and in `qsp_unpack_for_peer` (`:533`) | none | EXCLUDED **as cause** — applied only *after* items return; zero items ⇒ never reached. ⚠ Unreachable here, **not exonerated** for a run that gets past row 1 |
| 8 | client-side predicates after the pull | dedup `:525-531`, unpack `:533` | none | EXCLUDED — each emits a marker first; none present (§4) |
| 9 | TLS trust / CA | shared `relay_http_client()` | same box | EXCLUDED — a CA failure returns a code (`:3035`), not silence |

---

## 6. WHY NOTHING WENT RED

`route_token_is_valid` (`qsc/src/adversarial/route.rs:21-28`) accepts any non-empty ASCII
`[A-Za-z0-9_-]` string of length **22..=128**. Measured: a 3-character label (`bob`) is **REFUSED**
with `QSC_ERR_ROUTE_TOKEN_INVALID`; the sealed run's label measures **25 characters** and is
**ACCEPTED**, passing through `normalize_route_token` (trim + validate) unchanged. In CI the label is
longer still — `run_tag` is `${GITHUB_RUN_ID}-${GITHUB_RUN_ATTEMPT}-${scenario}-${seed}`.
**The guard catches the toy case and misses every realistic one.**

Then `:3056` maps `HttpStatus::NO_CONTENT` to `Ok(Vec::new())`: **"this is not your mailbox" and
"your mailbox is empty" are the same observable at the client**, at rc 0.

---

## 7. THE CONSUMER CENSUS — why this is a fixture defect

Measured at `5201c275` over `qsl/qsl-client/qsc/tests/*.rs`:

| measurement | value |
|---|---|
| test files passing `--mailbox` | **40** |
| total `--mailbox` call sites | **99** |
| sites passing a bare identity label | **1**, and it is a negative control |
| delivery assertions reached through `--mailbox` (`tests/receive_e2e.rs`) | `recv_commit` at `:257`, `:311`, `:487`; `qsp_unpack ok=true` at `:486`, `:528` |

Every site that reaches a pull passes a route-token-shaped value (`ROUTE_TOKEN_BOB` is literally
`"route_token_bob_abcdefghijklmnopqr"`). The single bare-label site, `tests/unlock_gate.rs:170`, is a
**locked-vault negative control** asserting `code=vault_locked` — it refuses before addressing
anything.

⇒ **`scripts/demo/qsc_remote_handshake_smoke.sh` is the only consumer in the tree that passes an
identity label to `--mailbox`.** The contract is settled and green-tested; one fixture is wrong about
it.

⚠ **This corrects a false sentence of this lane's own.** STOP 001 §6 asserted *"the `--mailbox`
override has ZERO green coverage anywhere in the tree"* — measured **FALSE**, and recorded as false
in **WF-0086**. The claim had been quantified over *the tree* from a sweep of `scripts/demo/` and
`.github/workflows/` alone: **an instrument narrower than its claim (SR-21).**

---

## 8. THE COVERAGE MEASUREMENT (WF-0086)

| script | invokes `qsc receive` | run by | asserts a received message? |
|---|---|---|---|
| `scripts/demo/qsc_demo_local.sh` | yes, `:235` / `:241`, loopback relay, **no `--mailbox`** | `.github/workflows/demo-packaging.yml:55` | ⚠ **NO** — both end in **`\|\| true`**; the script carries 0 golden comparisons, 0 `exit 1`, 0 `recv_commit`/`qsp_unpack` assertions, and the workflow is **55 lines** with that run as its **last step** |
| `scripts/demo/qsc_remote_handshake_smoke.sh` | yes, `:375` / `:388`, **remote** relay, `--mailbox` | `.github/workflows/remote-handshake-tests.yml:81`,`:92` | asserts correctly at `:395`/`:396`, but those sit behind `:351` (**ENG-0191**, unsatisfiable by construction since April) and **have not executed in 187 days** |
| `scripts/demo/qsc_remote_relay_smoke.sh` (340 lines) | no — `receive`, `unpack`, `handshake` each occur **0** times | `.github/workflows/remote-relay-tests.yml:97` | n/a |

⇒ **NO CI JOB ASSERTS A MESSAGE RECEIVED OVER A REAL RELAY.** One runs receive and asserts nothing;
the other asserts and never runs it. ⚠ **The decisive asymmetry:** the receive that *is* exercised
omits `--mailbox` and so takes the correct self-inbox resolution, while the only consumer that
supplies `--mailbox` has never reached its own receive.

---

## 9. THE INSTRUMENTATION MEASUREMENT (ENG-0193)

| marker | occurrences | where |
|---|---|---|
| `relay_push_diagnostic` | **28** | `qsc/src/transport/mod.rs` |
| `relay_pull_diagnostic` | **0** | **tree-wide** |

The push marker carries status class, status code, error class, diagnostic class, timeout phase,
response-body presence and length, route-header presence, auth presence, qsc error and attempt
number. The pull's entire observable surface is `recv_start` (an 8-hex mailbox hash), an optional
`recv_ack_mode`, and then `recv_none` or `recv_commit` — even though `:3048-3062` already
distinguishes 200 / 204 / 401 / 403 / 400 / 413 / 429 / other and discards all of it.

⚠ **ENG-0192 was found by computing sha512 over four candidate strings**, because that hash is the
only thing the receive path publishes about where it looked.

---

## 10. PREMISES MEASURED, INCLUDING THE ONES THAT FAILED

| premise | source | verdict |
|---|---|---|
| main `5201c275` UNMOVED, bare + unpiped, named remote | brief §2(a) | ✅ CONFIRMED, rc 0, one line |
| open-PR set empty | WF-0068 derivation | ✅ CONFIRMED |
| NA-0735's nine sealed shas intact at 444 | brief §2(b) | ✅ **9/9 `OK`, rc 0**; mtimes diffed before/after, identical |
| `NA-0735` not reserved in repo truth | brief §0 | ✅ CONFIRMED — **0 occurrences tree-wide**; but **consumed operator-side**, so 0736 is the next free id |
| smoke `:372` / `:375` / `:385` / `:388` line numbers | brief §3 | ✅ **ALL FOUR CORRECT** — ⚠ this lane's first needle searched the literal `send_ab_1` and returned 0, because the script writes `"send_ab_${i}"` in a loop; re-measured from bytes |
| `qsc_remote_relay_smoke.sh` is 340 lines; `receive`/`unpack`/`handshake` occur 0 times | brief §6 | ✅ CONFIRMED |
| ENG-0134 / ENG-0142 carry no `- Severity:` and no `Status:` | brief §5 | ✅ CONFIRMED — 0 and 0 in both entries |
| "1 item each — bob's 15250 B, alice's 23043 B" | brief §3 | ⚠ **FALSE as stated** — those are the **JSON response-body file sizes**; the items measure **4279 B** and **6436 B**, matching `handshake_send` **A1** and **B1**, against user payloads of **17 B** / **15 B** ⇒ **the retained frames are handshake frames** |
| "Three demo scripts DO receive and NO workflow runs them" | brief §6 | ⚠ **FALSE** — **two** invoke `receive`, and **both are workflow-run**. The conclusion survives for a stronger reason (§8) |
| "`--mailbox` has ZERO green coverage anywhere in the tree" | **this lane, STOP 001 §6** | ⚠ **FALSE** — 40 files, 99 sites (§7) |

---

## 11. ⚠⚠ WHAT IS **NOT** ESTABLISHED (R335 §5)

This suite's documented history is **four causes, each hiding the next**. This lane found the fifth.
**Nothing entitles a reader to assume it is the last.**

- ⚠ **(a) The user messages' fate is UNESTABLISHED.** Both sends returned HTTP 200 with
  `send_commit send_seq=1`, so both were pushed to the route-token mailboxes. The diagnostic pull of
  those mailboxes returned "exactly 1 item" each — **and a max-limited pull is not a mailbox
  census.** Established: **at least one** handshake frame was present. **NOT** established: that the
  user payloads are present, or that they are absent. **Write neither.**
- ⚠ **(b) Why is an A1 handshake frame still in bob's mailbox at all?** The handshake completed on
  both sides. Lease-without-ack, dedup-on-redelivery and consumption-leaves-the-item are all live
  candidates. ⚠ **No mechanism is invented** (WF-0080's precedent). Open observation.

---

## 12. WHAT WAS NOT DONE

No product source byte changed. No repair, no workaround. The committed script **not edited**. The
coverage gate **not built** — it would be born red, and REQUIRED-vs-ADVISORY is the operator's call.
`ROADMAP.md` **not edited**. ENG-0134 and ENG-0142 **not rewritten** — bullets and notes added beside
their unaltered text. NA-0735's sealed evidence **not written to**; its mailboxes **not pulled**;
**zero HTTP requests to the relay were made by this lane**. No secret read, printed or written.
**#1745 not touched. ENG-0191 not repaired.** The operator merges; the seat does not.
