# NA-0740 — AS BUILT: the ENG-0142 pre-repair measurement lane

Base main `9d11e2bd97af37d221854555a2e79848873692a7`. **Measurement and filing only — zero product
source bytes, zero committed files edited, nothing repaired.** Ruled at **R338**; decision **D-1375**.

⚠ Every figure below was measured when this file was written, not carried from a draft.

## 1. THE RIG, AND BOTH ENVIRONMENT KNOBS

| item | value |
|---|---|
| `qsl-server` rev | `37ec82072cbbd68e4eaba83e192282fbcb96e5b4` — the rev the AWS box runs, re-derived not adopted |
| `qsl-server` binary sha256 | `f1cb515357b3d5bbeadfe4b1b892e56a30ef05add97405392572e849e4877b9d` |
| `qsc` binary sha256 (built at the base) | `3f9a3fea4124b71cc9d471c058ec8e0fd17b0b7c982c129b3b45272e0328244e` |
| **`MAX_BODY_BYTES`** | **65536** — matched to AWS; the relay's own `/v1/server-info` echoes it |
| **`PULL_LEASE_SECS`** | **60** — the production value; ⚠ `/v1/server-info` does NOT advertise it, so it was proven from the store's own `leased_until` column |
| run tag | `na0740-20260816T055932Z` — embedded in every mailbox token, so a row from any other run is impossible by construction |
| transport | **loopback, plain HTTP**, locally generated bearer, **zero secrets read, no relay contacted** |

Auth gate proven live before any measurement: `GET /v1/server-info` → **401** unauthenticated, **200**
with the bearer. Advertised `retention.ttl_secs=604800`.

## 2. THE HARNESS — all written from scratch; NO committed script was edited or copied

| file | sha256 | lines |
|---|---|---|
| `harness/census.py` | `8d77a2bebc3ab22a7cdc4f304b60cb103390477e3b6d0abb71f45d2d0b1b2f56` | 77 |
| `harness/env.sh` | `1406650ce70aee1a3e26fc2612b81348e353aca106e09e2dfb1ab673750f41da` | 41 |
| `harness/q1_invite_flow.sh` | `78331e917e72eb0ba5cd34b7b30e4ef04b4bab92a7f88b409a25df3d3c0836e8` | 81 |
| `harness/q1ii_receive_residue.sh` | `9ea2cee759f5b820eb5dae1a060907f964c8691ce1633a6abd3692c9be3c88ec` | 68 |
| `harness/q2_receive_column.sh` | `118ceda70f9291eaf16ec91e02e7947c783cca69b3a12aa3148ca5f25e5969f2` | 42 |
| `harness/q2d_encoding_proof.sh` | `e17e47ea23feb1524889d339689e9a6cf329dbc9631593b0552fe56205de7894` | 77 |
| `harness/q3i_delay.sh` | `92ea1ea7a06ccda3b6e3da93e4be14cad2f3b9474d67fae6e889a44ef7ffedbc` | 57 |
| `harness/q4_lease_progress.sh` | `a85c65163033235062080dca6425177088107dfceaea20d26098acf0b181a61c` | 68 |
| `harness/s2_frames_and_poll.sh` | `de8031bde9a9f10b5bd86b2809f15fdd7ff5427e8e4b360b0c6554a3ae341b9a` | 91 |
| `harness/s4_delay_and_q3ii.sh` | `7b37917024a15bd8977c9f123d6ca20b80b4fc1f05bd5ef9e209a5dd06c39839` | 111 |
| **`SEAL_001_Q1_INVITE_FLOW.md`** (sealed 444 before its runs) | `ad1f7c5e307346806b485e00dc569db810ea23ffabef142d4a40137a9d0453d5` | 65 |
| **`SEAL_002_Q2_Q3.md`** (sealed 444 before its runs) | `2feaf6b6dc22f3e28d6bc0c7f32a83a7c05c88225bf8e04621b541e8544e2aaa` | 62 |
| `runs/store_snapshot_FINAL.sqlite` | `5716d6d4ab999f0e2363851a6161d346e72811dca6dd3308881243541a5132d7` | (binary) |

⚠ The store snapshot was taken with **`VACUUM INTO`, never `cp`** (a `cp` of a live SQLite DB preserves
an EMPTY file that `ls -l` and `sha256sum` both call healthy), and **asserted before being called
evidence**: live row/route counts equal to the snapshot's, both non-zero.

## 3. THE Q2 FRAME-CLASS DISCRIMINATOR TABLE

Decoded frame bytes. ⚠ The encoding was **measured before any byte claim**: `InboxPullItem.data` is a
**JSON array of u8 integers** (serde's default for `Vec<u8>`), decode step `bytes(data)`, round-tripped
on a seat-authored frame carrying `\x00\x01\xFE\xFF` with the non-vacuity **asserted in the harness**,
then the store BLOB proven **FULL-BODY equal** to the wire bytes on two product-authored frames.

| class | decoded head8 | len | authored by | mailbox | what `receive` does |
|---|---|---|---|---|---|
| handshake A1 | `51 48 53 4D 00 01 01 C8` | 4279 | `handshake init` | responder's ordinary inbox | `qsp_env_decode_failed`, rc 1, ABORT |
| handshake B1 | `51 48 53 4D 00 01 02 C8` | 6436 | `handshake poll` (responder) | initiator's ordinary inbox | `qsp_env_decode_failed`, rc 1, ABORT |
| handshake A2 | `51 48 53 4D 00 01 03 C8` | 3364 | `handshake poll` (initiator) | responder's ordinary inbox | `qsp_env_decode_failed`, rc 1, ABORT |
| handshake A2 (invite flow) | `51 48 53 4D 00 01 03 6E` | 3364 | `invite finish` | inviter's ordinary inbox | `qsp_env_decode_failed`, rc 1, ABORT |
| invite envelope (INIT) | `01 01 01 0C 45 01 04 A0` | 7464 | `invite redeem` | the invite slot | ⚠ **UNMEASURED** — `receive` does not poll that mailbox on any default path |
| invite reply (RESP) | `01 02 02 00 23 61 6C 69` | 6479 | `invite accept` | redeemer's ordinary inbox | `qsp_env_decode_failed`, rc 1, ABORT |
| QSP message envelope | `01 00 00 01 00 00 00 00` | 1024 | `send` | recipient's ordinary inbox | **DELIVERS**: `qsp_unpack ok=true` → `recv_item` → `relay_ack sent=1 acked=1` → `recv_commit count=1` |

⚠⚠ **All three envelope classes share the first byte `0x01` and differ only in the second** ⇒ a
one-byte discriminator cannot separate a user's message from an invite frame. ⚠ **Byte 7 of a
handshake frame is not stable** — `C8` on the plain path, `6E` on the invite path — so nothing may key on it.

## 4. THE Q1 CENSUSES — complete accounting after every command

```
==== CENSUS [C0_provisioned]  now=1786860063 ====
-- routes: 0
-- messages: 0
-- ACCOUNTING: routes=0 rows=0 unexplained_routes=0 unexplained_rows=0

==== CENSUS [C1_after_create]  now=1786860067 ====
-- routes: 0
-- messages: 0
-- ACCOUNTING: routes=0 rows=0 unexplained_routes=0 unexplained_rows=0

==== CENSUS [C2_after_redeem]  now=1786860076 ====
-- routes: 1
   route 55f90a5c489be511…  name=invite_slot  created=1786860076 touched=1786860076
-- messages: 1
   seq=1 mailbox=invite_slot len=7464 head8=01 01 01 0C 45 01 04 A0 enq=1786860076 RESIDENT msg_id=8b822f87-792…
-- ACCOUNTING: routes=1 rows=1 unexplained_routes=0 unexplained_rows=0

==== CENSUS [C3_after_accept]  now=1786860084 ====
-- routes: 2
   route 55f90a5c489be511…  name=invite_slot  created=1786860076 touched=1786860078
   route e8a3e60af118cff1…  name=bob_inbox  created=1786860083 touched=1786860083
-- messages: 2
   seq=1 mailbox=invite_slot len=7464 head8=01 01 01 0C 45 01 04 A0 enq=1786860076 LEASED(until=1786860138, +54s) msg_id=8b822f87-792…
   seq=2 mailbox=bob_inbox len=6479 head8=01 02 02 00 23 61 6C 69 enq=1786860083 RESIDENT msg_id=fdd517b2-ba0…
-- ACCOUNTING: routes=2 rows=2 unexplained_routes=0 unexplained_rows=0

==== CENSUS [C4_after_finish]  now=1786860091 ====
-- routes: 3
   route 55f90a5c489be511…  name=invite_slot  created=1786860076 touched=1786860078
   route e8a3e60af118cff1…  name=bob_inbox  created=1786860083 touched=1786860085
   route c8e205a17b8567d2…  name=alice_inbox  created=1786860091 touched=1786860091
-- messages: 3
   seq=1 mailbox=invite_slot len=7464 head8=01 01 01 0C 45 01 04 A0 enq=1786860076 LEASED(until=1786860138, +47s) msg_id=8b822f87-792…
   seq=2 mailbox=bob_inbox len=6479 head8=01 02 02 00 23 61 6C 69 enq=1786860083 LEASED(until=1786860145, +54s) msg_id=fdd517b2-ba0…
   seq=3 mailbox=alice_inbox len=3364 head8=51 48 53 4D 00 01 03 6E enq=1786860091 RESIDENT msg_id=17423d11-1eb…
-- ACCOUNTING: routes=3 rows=3 unexplained_routes=0 unexplained_rows=0

```

## 5. THE MARKER SEQUENCES, VERBATIM

**Q1(ii) — `receive` on the redeemer's ordinary inbox (residue: the invite reply), rc 1:**

```
QSC_MARK/1 event=receipt_policy mode=batched batch_window_ms=250 jitter_ms=0 file_confirm_mode=complete_only
QSC_MARK/1 event=session_load ok=true format=v3
QSC_MARK/1 event=recv_start transport=relay mailbox=redacted mailbox_hash=5cb420a9 from=alice max=8
QSC_MARK/1 event=recv_ack_mode mode=lease
QSC_MARK/1 event=qsp_unpack code=qsp_env_decode_failed ok=false
QSC_MARK/1 event=error code=qsp_env_decode_failed
```

**Q1(ii) — `receive` on the inviter's ordinary inbox (residue: the A2 handshake frame), rc 1:**

```
QSC_MARK/1 event=receipt_policy mode=batched batch_window_ms=250 jitter_ms=0 file_confirm_mode=complete_only
QSC_MARK/1 event=recv_start transport=relay mailbox=redacted mailbox_hash=94b7b4ce from=bob max=8
QSC_MARK/1 event=recv_ack_mode mode=lease
QSC_MARK/1 event=qsp_unpack code=qsp_env_decode_failed ok=false
QSC_MARK/1 event=error code=qsp_env_decode_failed
```

**Q3(i) — `handshake poll` over one QSP message frame, rc 0:**

```
QSC_MARK/1 event=handshake_pending peer=peer present=false role=none key=handshake.pending.self.peer state=absent
QSC_MARK/1 event=handshake_reject reason=handshake_magic
```

**Q3(ii) — `invite finish` with an ordinary message ahead of the invite reply, rc 1 (entire stdout):**

```
QSC_MARK/1 event=error code=handshake_envelope_malformed
```

**Q3(i) — `receive` after the lease expired: the message arrives INTACT (nothing was destroyed):**

```
QSC_MARK/1 event=receipt_policy mode=batched batch_window_ms=250 jitter_ms=0 file_confirm_mode=complete_only
QSC_MARK/1 event=session_load ok=true format=v3
QSC_MARK/1 event=recv_start transport=relay mailbox=redacted mailbox_hash=ddcdb4cd from=peer max=8
QSC_MARK/1 event=recv_ack_mode mode=lease
QSC_MARK/1 event=session_load ok=true format=v3
QSC_MARK/1 event=qsp_unpack ok=true version=5.0
QSC_MARK/1 event=ratchet_recv_advance msg_idx=1
QSC_MARK/1 event=session_store ok=true format=v3 enc=aead
QSC_MARK/1 event=meta_bucket bucket=1024 orig=1024 capped=1024 metric=envelope_len
QSC_MARK/1 event=recv_item idx=1 size=27 id=<redacted>
QSC_MARK/1 event=message_state_transition from=CREATED to=RECEIVED id=<redacted> ok=true
QSC_RECEIPT mode=batched status=queued kind=message peer=peer
QSC_MARK/1 event=relay_ack sent=1 acked=1
QSC_MARK/1 event=session_load ok=true format=v3
QSC_MARK/1 event=session_load ok=true format=v3
QSC_MARK/1 event=session_store ok=true format=v3 enc=aead
QSC_MARK/1 event=receipt_send kind=delivered bucket=small msg_id=<redacted>
QSC_RECEIPT mode=batched status=sent kind=message peer=peer
QSC_MARK/1 event=recv_commit count=1
```

## 6. THE CLAIM BOUNDARY

Loopback, plain HTTP, no TLS, **no relay contacted, zero secrets read**, `qsl-server` rev `37ec8207`,
`qsc` at `9d11e2bd`, `MAX_BODY_BYTES=65536`, `PULL_LEASE_SECS=60`, **n=1 per arm**. **NOT a CI claim** —
nothing here turns any suite green. ⚠ What `receive` does on an **invite ENVELOPE** is **UNMEASURED**.
⚠ The **head-of-line consequence** — a real message queued *behind* an undecodable frame being blocked —
was **NOT re-demonstrated here**; it follows from the measured abort by construction
(`transport/mod.rs:1249`) and was measured at NA-0738. **This lane measured the abort, not the blocked
message behind it.** Both were offered and **DECLINED at R338 §5**; either is revivable in one line.

## 7. SEALED EXPECTATIONS

**E1/E2/E3/E4 (the Director's): 4 HITs.** **S1/S2/S3+S6/S7/S8 (the seat's): 5 HITs.**
⚠ **One seat NON-RESULT, recorded not tidied** (SR-16 row 39): a delay checkpoint whose antecedent had
silently expired, on a peer pair that could never have delivered because the seeded-session test
fallback keys on the LOCAL contact label and the labels were asymmetric. Both rebuilt; honest figure 59 s.
⚠⚠ **And one caught by the DIRECTOR, not the seat** (SR-16 row 40): the drafted records said the
topology correction targeted *the prior amendment's wording*; measured, `inviter` occurs **zero** times
in the ENG-0142 entry and the landed wording *"the user's ORDINARY inbox"* was **correct**. The error
under correction was the **NA-0740 brief's own E1**. Corrected at R338 §1 before landing.

