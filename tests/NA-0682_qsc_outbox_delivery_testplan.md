# NA-0682 TESTPLAN — MESSAGING EPIC SLICE 3: the outbox, the delivery states, and the ack only the recipient can produce

Lane NA-0682 · directive **QSL-DIR-2026-07-27-617 (D617)**, sha256
`f4cca70cf78f85346a0afdf2745d5aeb03ce353bfc4c0fc37f96f4d3e3f7be34`, 642 lines ·
implementation evidence **D-1317** · closeout **D-1318** · result class
`QSC_OUTBOX_DELIVERY_PASS`.

Built against the Slice-2 tree at `a3b0f758`, relay pin `131d63f4`.

---

## §A — AUTOMATED COVERAGE

### A.1 `src/msgqueue/mod.rs` — 32 unit tests: the store, the states, the FIFO policy

The FIFO/backoff **policy** is separated from crypto+network behind a `MessageSender` trait,
so it is testable with no relay, no vault and no ratchet. That separation is why several
invariants below are one-line assertions instead of arguments.

| property | tests |
|---|---|
| record round-trips under its own AAD; FIFO survives the power-of-ten filename boundary; `msg_id` is 128 CSPRNG bits of lowercase hex; a raw contact label never reaches a filename | 4 |
| AAD binds contact **and** seq: a record moved between contacts, or renumbered, or under another store key, or truncated — all refuse | 4 |
| in-flight state: a fresh row is unpacked; a packed one replays; the bytes survive a store round trip byte-identically; the three fields clear together | 3 |
| **ratchet safety**: packed exactly once across four attempts; abandoning a packed row advances the ratchet FIRST; a failed commit keeps the row QUEUED rather than dropping the advance | 3 |
| drain: happy path in FIFO order; **msg2 never overtakes msg1**; **a stuck contact does not stall another**; a successful send commits **exactly once**; a retryable failure mutates **no** session state; 413 fails only its own message | 6 |
| dedup `(session, msg_id)`: unseen→recorded→seen, idempotent; scoped per contact; survives reload and is encrypted at rest; a seen-record moved between contacts refuses | 4 |
| paused is a sub-state of QUEUED with a way out; backoff climbs the ruled 5s→5m ladder and caps forever; jitter is stable per record | 3 |
| **A7** causes distinct in both vocabularies · **A8** locked vault + unreachable relay say the honest thing · **A10** only a revoked session is permanent | 5 |

### A.2 `src/adversarial/payload.rs` — 5 tests: control classification (C6 / F1)

Legacy `v1` shapes matched exactly and unchanged · `v2` shapes matched when they carry the
namespace marker · an unknown type **or a newer version** carrying the marker is **IGNORED** ·
⚠ **a user message that merely looks like a control payload is still DELIVERED** · a foreign
namespace is not ours.

### A.3 `tests/NA_0682_kill_in_the_send_window.rs` — **A1, the deliverable**

| requirement | test |
|---|---|
| **A1 — kill the process INSIDE the persist-before-network window; the row survives and drains** | `a1_killing_the_process_inside_the_send_window_leaves_a_queued_row_that_drains` |
| **A2 — crash-before-network leaves a QUEUED row** | `a2_a_send_that_never_reaches_the_network_still_leaves_a_queued_row` |

**Method, and the property that makes it deterministic** (D617 C13): a proxy stalls
`/v1/push` indefinitely; the test **polls until the proxy reports the client is inside the
window** and only then SIGKILLs. The window is arbitrarily wide and its entry is an
observation, not a timing assumption. The QUEUED row is asserted **while the push is
stalled** — the invariant is observed in the window, not inferred afterwards from recovery.

⚠ The proxy is written in that file. `NA_0644_ack_client.rs` was **not touched** (D617 §6):
it is the sole evidence for the lease/dedup contract.

### A.4 End-to-end, against a real relay in process

`a_second_message_while_one_is_stuck_is_queued_not_dropped` (**A3/A6**, and it replaces the
test that asserted the C4 defect) · `discard_burns_state_and_prevents_nonce_reuse_on_next_send`
(**F2** + the nonce barrier at the CLI) · `the_off_knob_still_suppresses_acks_after_f6_turned_them_on_by_default`
(**F6 requirement (a)**) · `delivered_receipt_roundtrip` (**A4**) · the
`relay_{drop,dup,reorder}_no_mutation` trio, still driving real fault injection.

---

## §B — NEGATIVE CONTROLS: TWENTY RUN, TWENTY OBSERVED RED

A test is not evidence until it has been seen to fail. Each control broke the property, the
red was **recorded**, and the source was restored **byte-identical** (`cmp`-verified).

| # | property | control | red produced |
|---|---|---|---|
| 1 | AAD binds the contact | drop `contact_key` from the AAD | **Alice's record decrypted under BOB's AAD** — `unwrap_err() on an Ok value: QueuedMessage { peer: "alice", seq: 3 }` |
| 2 | in-flight bytes survive a restart | `#[serde(skip)]` on `ciphertext` | `left: None, right: Some([0,1,2,250,251,255])` |
| 3 | **ratchet advance before drop** | revert `retire_packed` | `left: 0, right: 1` commits |
| 4 | fail-closed on a failed commit | same | record marked terminal with its advance dropped |
| 5 | commit exactly once | commit twice on success | `left: 4, right: 2` |
| 6 | no mutation on transport failure | commit on a failed push | `left: 1, right: 0` |
| 7 | **commit-before-send (A1/A2)** | make `enqueue_at` not persist | both A1 and A2 RED; A2: `crash-before-network must leave a QUEUED row, left: 0 right: 1` |
| 8 | F6's off knob works | make the policy ignore `off` | `mode=immediate`, ack sent anyway |
| 9 | C11 changed nothing shipped | ran the four guard files | 22 green, `git status` on `tests/` **empty** |
| 10 | **replay is rejected AS A DUPLICATE** (migrated guard) | delete `if from == to { state_duplicate }` in `timeline/mod.rs` | `code=state_invalid_transition` — the duplicate is no longer recognised as one |
| 11 | ⚠ **an ack naming an unknown id transitions NOTHING** (new guard) | weaken the strict matcher to a position fallback | `message_state_transition from=SENT to=DELIVERED ok=true` — **a forged ack marked a real message DELIVERED** |
| 12 | **the F6 default is OFF, both halves** | flip both halves ON | both pins red; `with_meta_does_not_enable_receipts` correctly stayed green (it asserts caller-verbatim behaviour, not the default) |
| 13 | **a local relay-config fault is NAMED** | disable local config-fault classification | `family3_ca_config_failures` RED — *expected relay_ca_file_unreadable, got `msgqueue_enqueued …`* |
| 14 | ⚠ **the three CA causes are DISTINCT** | collapse `UNREADABLE → MISSING` | RED — `code=relay_ca_file_missing` where `unreadable` is required. **Distinctness, not presence** |
| 15 | **auth ≠ trust** | collapse `401\|403 → relay_tls_untrusted` | `family3_trust_failure…` **and** `family2_explicit_ca_is_additive…` RED |
| 16 | auth ≠ trust, **on the MIGRATED guard** | control 15 re-run post-migration | RED — output shows `QSC_ROUTING` + `qsp_pack`, proving the migrated fixture really attempts the send |
| 17 | wrong-device receipts are IGNORED | make `confirm_target_matches_channel` always match | `timeline_delivery_contract_na0217f` RED — the wrong-device receipt was APPLIED (`from=SENT to=DELIVERED`) |
| 18 | ⚠ **an already-packed row is NEVER re-packed** | disable skip-pack so a retry re-packs | `packed_message_is_not_repacked_after_primary_switch` RED — *"an already-packed row was RE-PACKED … this burns a message key"*. **Also guards replay-identical-bytes** |
| 19 | routing follows the CURRENT primary | make `primary_device` ignore `primary_device_id` | `primary_only_routing_marker_changes_after_primary_switch` RED — missing device2 marker |
| 20 | (control 15 restored) | source restored byte-identical, `cmp`-verified | GREEN — every control above restored clean |

⚠ **Control 11 is the one to read twice.** With strict matching removed, an ack carrying an id
this client never minted marked a real message **DELIVERED**. That is the honest-delivery-claim
hazard in its actual shape — the shipped code refuses it, and control 11 is what proves the
refusal is doing work rather than being an accident of the fixture.

⚠ **Control 7's red arrived one assertion earlier than designed, and that is informative.**
With no durable row the drain has nothing to read, so the process never reaches the push and
the "window never entered" guard fires first. **Commit-before-send is therefore STRUCTURAL,
not merely ordered: the drain's only input is the store, so no code path can push an
uncommitted message.** The in-window assertion is belt-and-braces on top of that.

---

## §B2 — GUARD MIGRATION AND NEW GUARDS (operator-ruled, STOP 019)

**Recorded, never silent.** Each entry states what changed, why, and the control that proves the
replacement exercises the property rather than merely passing.

### B2.1 `replay_ack_does_not_advance_state` — MIGRATED (two edits, one cause)

⚠ **Retired form and reason.** The test learned the message id by scraping `id=` out of the
`event=timeline_item` **diagnostic marker**, and asserted the reject marker echoed that id back.
Both halves depended on an identifier reaching the diagnostic surface.

**Why that broke:** the marker layer redacts by **value shape** — `len() >= 24 && has_digit`
(`src/output/mod.rs:292`). NA-0682 widened `msg_id` from 16 to 32 hex chars **specifically to
stop emitting `sha512(plaintext)[..8]`**, a fingerprint of the message body (the C17 leak, closed
by F1). The old id had slipped under the threshold; the new one does not. So the scrape returned
the literal string `<redacted>` and the test built its acks against it.

⚠ **The failure mode is the lesson: the sentinel PARSES AS A VALID IDENTIFIER.** The test did not
fail at the scrape — it proceeded and failed three steps later, in another subsystem, with a
misleading code.

**New form.** `first_party_sent_msg_id()` reads the id **the test itself minted** from the
sender's own queue record filename (`msgqueue_v1/<contact>/<seq:020>_<msg_id>.rec`, which persists
in state `SENT`). No marker, no redactor, no new shipped surface. The helper additionally asserts
it never returns the redaction sentinel.

**Assertion change, recorded separately because it is a second edit:** the expected reject text no
longer embeds the raw id. It could only ever have passed **while the marker printed the id — i.e.
it asserted the leak F1 closed**. The property is unchanged and now asserted in three parts:
(a) rejected, and rejected **as a duplicate** (not as unknown); (b) ⚠ the marker **does not** echo
the raw id — the leak staying closed is now asserted *on purpose* instead of depended upon;
(c) zero mutation — still exactly one entry, still `DELIVERED`.

**Control:** §B row 10. **The property did not weaken; the coupling to redaction was removed.**

### B2.2 `ack_for_unknown_msg_id_transitions_nothing` — NEW

**Grounds, recorded because they are the point:** the property was real and the shipped code
already held it — but it was evidenced **only by an accident**. A degraded scrape fed a garbage id
into ack-apply and the correct refusal appeared as an incidental line inside a *failing* run.
⚠ **Sole-evidence-by-accident is exactly what the audit discipline exists to eliminate**, so the
property is now asserted deliberately.

Asserts: an ack whose `msg_id` matches no entry is refused with `state_unknown`, is **not**
reported as a duplicate, produces no `receipt_recv` / `delivered_to_peer` / `peer_confirmed`, and
leaves the real message in `SENT`. The forged id is the **same shape** as a real one (32 lowercase
hex) so the refusal is provably about the id being *unknown*, not malformed.
`state_unknown` and `state_duplicate` stay **distinct causes with distinct words**, both tested.

**Control:** §B row 11.

### B2.3 The F6 default pins — NEW (Condition 4)

`message_state_tests::receipt_default_is_off_recipient_half` and
`transport::receipt_sender_default_tests::sender_requests_no_receipt_by_default` pin the two
halves **separately and on purpose**: F6 has two independent switches and flipping one leaves the
wire noisy while the feature looks disabled. A third test proves an **explicit** request is still
honoured — the mechanism ships, only the default waits. ⚠ **These pins are designed to go red when
ENG-0086 lands.**

**Control:** §B row 12.

### B2.4 `receipt_mode_off_suppresses_an_ack_the_sender_actually_requested` — REWRITTEN

⚠ **Found green but VACUOUS during the Condition-5 re-verification, and reported rather than
ticked off.** Its previous form (named for F6 turning acks on by default) sent **without**
requesting a receipt and then observed that no ack appeared. Once Option D restored the
off-by-default posture, that would have passed **with the off knob completely broken** — no ack
was possible in the first place.

**New form:** the sender genuinely requests a receipt, the recipient is *also* handed
`--emit-receipts`, and the recipient's `--receipt-mode off` must still suppress the ack — proving
the recipient's own policy is authoritative (F6 requirement (a)). Step (3) then shows the **same
fixture** producing an ack when the knob is not off: **a negative result is only evidence if the
instrument could have returned positive.** The non-vacuity step is the built-in control.

---

## §B3 — THE NA_0663 REGRESSION: a FALSE DIAGNOSIS on a Tier-1 trust property

⚠ **The most consequential defect this lane produced after the nonce barrier, and it was found by
the operator's stop condition, not by me.**

**Symptom.** A TLS trust-configuration failure (unreadable CA file) and a rejected access token
(401) were both reported to the user as **"queued — will send when the relay is reachable."**
Green at base (11/11), red in the lane. ⚠ **An operator facing an untrusted certificate — possibly
an active interception — was told to wait for the network.** That is precisely the confusion
`NA_0663_relay_tls_trust` exists to prevent.

**Cause — structural, not a lost code.** Full map:
`docs/governance/evidence/NA-0682_send_path_swallow_map.md`. In short: the classifier was never
broken. Making O1 true put a durable per-contact FIFO between the transport and the user, and a
message queued **behind** a not-due or paused message is **never attempted** — so the send path had
no cause to report for it and fell back to a generic line. ⚠ `relay_ca_file_missing` survived only
because it happened to be the **first** sub-case in its test.

**Fix (operator-ruled shape).** Enqueue stays unconditional (O1 — refusing to enqueue would make
the user's typed message lossable, the exact silent loss O1 prevents; fail-closed governs
*transmission*, and enqueueing transmits nothing). Local, message-independent relay-config faults
are classified with **no network attempt** and named precisely; the row is **PAUSED with the trust
cause**, which a settings fix resumes; when this message was not attempted, the **head's** pause
cause is reported. ⚠ *"Will send when the relay is reachable"* is now **reserved for the transient
class only**. Existing taxonomy strings reused verbatim; no new user-facing strings.

**Controls:** §B rows 13-16 — each breaks the **classification**, never a string.

**Guard migration (operator clause C), recorded:** the forcing assertion is
`NA_0663_relay_tls_trust.rs:494`. Sub-case (iii) shared a config with (i) and (ii), so under the
new queue it was attempted only if the head's ~5 s backoff had expired. ⚠ **This was a FLAKE, not
a clean impossibility — byte-identical source passed 11/11 twice and failed a third run**, which is
worse than a deterministic failure because it would have entered CI as intermittent red. Sub-case
(iii) now runs on its own config. **Nothing weakened:** (i) and (ii) still assert refused/DNS are
not reported as trust failures, (iii) still asserts auth is distinct from trust; only the fixture
is isolated, so the assertion tests cause distinctness rather than queue timing. Red-capability
re-proven on the migrated form (§B row 16), then three consecutive 11/11 runs.

---

## §B4 — THE TWO FULL-SUITE RESIDUALS (operator-ruled, STOP 021)

The lane's first settled full suite was **511 passed / 2 failed**. Both residuals were guards red
from **real behaviour changes**, both **green at base**, and both were stopped on rather than fixed
in passing.

### B4.1 `timeline_delivery_contract_na0217f` — the ENG-0087 class, instance #2

⚠ **The class I had filed as "unenumerated" produced its second instance in the very next full
run**, presenting completely differently: `state_unknown` from `qsc util receipt-apply` rather than
a wrong reject code. Same mechanism — `--receipt delivered` gives a 32-hex id, the `timeline_item`
marker redacts it, the scrape returns the literal `<redacted>`.

**Migrated** to first-party acquisition (`first_party_sent_msg_id`, reading the sender's own queue
record). ⚠ **The file's scrape helper now returns STATE ONLY**, so the class cannot recur there.
Control: §B row 17. Three consecutive green runs.

### B4.2 `trust_model_v2_phase_c_na0177` — routing markers are emitted at PACK time

The test sent twice on one config — under primary d1, then after switching to d2 — and asserted the
second send printed routing for d2. Under the queue, send 1 (dead relay) leaves an **already-packed**
row, the second invocation drains *that* row, and `attempt_one` correctly **skips packing**, so no
routing marker is emitted at all.

**Migrated** to isolated fixtures per sub-assertion (the NA_0663 pattern), so each assertion attaches
to a message that packs after its fixture's final switch. **Property unchanged.** Control: §B row 19.

⚠ **And the semantic underneath it was ruled, not absorbed:** an already-packed message is **not**
re-routed by a primary switch — see **D-1319** and the `DESIGN_outbox_delivery_v1` amendment. It is
now asserted on purpose by `packed_message_is_not_repacked_after_primary_switch` (§A), whose control
(§B row 18) also guards **replay-identical-bytes**. Three consecutive green runs for both targets.

---

## §C — WHAT THIS PLAN CANNOT SEE

*(The most useful section, per the NA-0680/NA-0681 pattern: the measurement, not the
coverage claim.)*

**C.1 — A1 proves crash-safety against PROCESS DEATH, not POWER LOSS.** `write_atomic`
fsyncs the file and best-effort fsyncs the directory, but **SIGKILL does not evict the page
cache**, so nothing here exercises the power-loss path. Operator-ruled into this section as a
LIMIT rather than a claim — the Slice-1 fsync lesson applied.

**C.2 — relay-level at-least-once protection still does not run at default settings.** F5
keeps `src/dedup/mod.rs` untouched and ENG-0043 out of scope. That module keys the **relay
envelope id** and is built **only in lease mode**, and lease is not the default — so the
`(session, msg_id)` dedup added here covers the INNER id only. **Knowingly accepted, filed,
not fixed.**

**C.3 — in-flight ratchet state now lives in TWO places.** Per-message fields in the message
queue (messaging) and the global `outbox.json` (attachments/file transfer). Accepted as the
trade against amending acceptance item A3 downward; **filed for a convergence lane.** ⚠ That
lane must preserve replay-identical-bytes AND the nonce barrier, and must know that F4's
separate store is the only reason `timeline_written_on_send_commit_only` does not collide
with O1.

**C.4 — three guards were RETIRED, not repointed.** `ratchet_durability_na0155`'s three tests
read `outbox.json` directly; that mechanism is gone from the default send path. Their
properties are re-proven — and each replacement was shown RED **before** the retirement, per
the binding condition — but **no end-to-end test now compares two ciphertexts byte-for-byte
across a retry**; the equivalent counts pack operations instead, from inside the module.

**C.5 — ⚠ AMENDED (2026-07-28): A12 IS exercised by a test. This limitation is WITHDRAWN.**

The original text read: *"argued structurally, not exercised … no test injects a forged ack at
the relay and observes the refusal … read as unproven-by-test."* **That is no longer true and
was stale in the UNDERSTATING direction** — it would have led a reviewer to discount coverage
that exists.

`NA_0682_kill_in_the_send_window::a12_a_relay_injected_ack_cannot_flip_a_message_to_delivered`
injects a forged plaintext ack directly into the mailbox (`server.enqueue_raw()`) and observes
the refusal, with a **non-vacuity assertion** (`qsp_unpack ok=false`) proving the refusal is the
AEAD rejecting it rather than the item never arriving. The ack rides inside the session AEAD, so
a relay without the session key cannot produce one `qsp_unpack` accepts — and that is now
**measured, not merely argued**.

Companion coverage added under the same property:
`message_state_model::ack_for_unknown_msg_id_transitions_nothing` covers the case where the ack
*is* well-formed but names an id this client never minted (§B2.2).

**C.6 — the 413 message does not yet name the relay's limit.** A9's "FAILED for that message
only" is proven; the "naming the relay's limit from `max_body_bytes`" half is not wired into
the user-facing line.

**C.7 — no timing or constant-time claim of any kind.** None was measured.

**C.8 — one rig, two vaults** is the tested topology (epic §4 Q2). Nothing here says anything
about NAT, real partitions, or two physical devices.

**C.9 — the marker layer still collapses 401 and 403.** Option B gives the queue a distinct
PAUSE cause from the HTTP status, but an operator reading raw logs still cannot tell them
apart. Narrow, filed, deliberately not fixed by rewriting `NA_0663`'s guard.

**C.10 — the three excluded `aws_file_*` suites did not run** (ENG-0079).

---

## §D — HOW TO RE-RUN

```
source /srv/qbuild/work/NA-0682/.qwork/cargo-target.qsl-protocol.env
cd <seat>/qsl-protocol
RUST_TEST_THREADS=2 cargo test -p qsc --lib msgqueue
RUST_TEST_THREADS=2 cargo test -p qsc --test NA_0682_kill_in_the_send_window
```

⚠ The full suite must **exclude** `aws_file_confirmation_replay_na0192b`,
`aws_file_medium_boundary_na0192a` and `aws_file_robustness_na0186` (ENG-0079).
⚠ `cargo fmt --all` must **not** be run (ENG-0050, RED at base at 146 locations) — format only
this lane's files. ⚠ `qsc-linux-full-suite` skips on pull requests, so CI green never covers
this suite; the local run is the evidence.
