Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0617 — ENG-0002 Attachment Single-Send-Per-Session Clarification + Resend Fix

## Summary

NA-0617 resolves ledger ENG-0002 (DOC-G5-005 §9 rank 6) under directive
QSL-DIR-2026-07-07-554 (D554). The ledger asked whether a second `file send` returning
`REJECT_QATTSVC_SESSION_STATE` is by design (one attachment per session) or a client
limitation. The design-lock established it is neither, once "session" is disambiguated —
and it uncovered one concrete, in-scope client footgun, which this lane fixes (the
directive's operator-authorized source carve-out; see D-1229). Client-side attachment
journal reuse logic + tests + docs only — no protocol/wire/crypto/state-machine semantic
change, no attachment wire/descriptor/object/padding format change, no dependency/workflow
change, no qsl-attachments or qsl-server change.

Result classification: `ATTACHMENT_RESEND_CONSUMED_SESSION_FAIL_CLOSED_TO_FRESH_SESSION`.

## The two-layer session model (the core clarification)

"Session" means two different things in the attachment plane, and ENG-0002's phrasing
conflated them:

- **L1 — the qsl-attachments SERVICE upload session.** Single-object **by design**:
  create session -> upload parts -> commit; on a successful commit the service persists
  the committed object and **removes the session** (`remove_session`). Any later use of a
  committed/aborted/expired session resolves to "unknown"/"not open" and fails closed with
  `REJECT_QATTSVC_SESSION_STATE`. This is the standard resumable-upload pattern (one upload
  = one object) and is correct; it is unchanged by this lane.
- **L2 — the qsc CLIENT session** (a config dir / identity used across `file send`
  invocations). This is **not** limited to one attachment. Each distinct `file send` mints
  its own L1 service session. `REJECT_QATTSVC_SESSION_STATE` is L1 fail-closed behavior on
  session reuse — **not** an L2 per-session cap.

## Design-lock: the exact reject trigger and the footgun

`attachment_id` is random per new record; the outbound journal record is found by
`(peer, source_path)`. A "second `file send`" splits into cases:

- **Distinct file (different source path):** no reusable record -> fresh random
  `attachment_id` -> fresh L1 session -> upload -> commit -> **succeeds**. L2 holds.
- **Same file, after the first send committed and was accepted by the relay
  (`ACCEPTED_BY_RELAY`, i.e. no `--receipt`):** the prior record was found and reused; the
  send-time in-flight guard only catches `PEER_CONFIRMED`/`AWAITING_CONFIRMATION`, so
  `ACCEPTED_BY_RELAY` fell through; `session_ref` still pointed at the **already-committed
  and destroyed** L1 session, so the client re-committed against a dead session and
  surfaced a raw `REJECT_QATTSVC_SESSION_STATE`. **This is the footgun** — a client
  reusing a consumed session, not an L2 cap. (With `--receipt`, the record is
  `AWAITING_CONFIRMATION` and the resend is cleanly refused as `attachment_send_inflight`.)

Outbound states partition by whether the L1 session is still usable:

| state | L1 session | correct resend behavior |
| --- | --- | --- |
| `SESSION_CREATED`, `UPLOADING` | alive | reuse -> resume |
| `AWAITING_CONFIRMATION` | consumed, delivery in-flight | reuse -> guard blocks (`attachment_send_inflight`) |
| `COMMITTED`, `ACCEPTED_BY_RELAY`, `PEER_CONFIRMED` | consumed/done | do NOT reuse -> fresh session |

## Fix (smallest change)

`attachment_find_outbound_by_source` (`qsl/qsl-client/qsc/src/attachments/mod.rs`, the sole
caller is the send path) now excludes the **consumed-session** states from reuse:

```rust
let session_reusable = !matches!(
    rec.state.as_str(),
    "PEER_CONFIRMED" | "COMMITTED" | "ACCEPTED_BY_RELAY"
);
```

A re-send of an already-delivered file then finds no reusable record, mints a fresh
`attachment_id` + fresh encryption context (fresh CEK + nonce prefix) + fresh L1 session,
and succeeds. Resume (`SESSION_CREATED`/`UPLOADING`) and in-flight blocking
(`AWAITING_CONFIRMATION`) are preserved because those states remain reusable.

## Why fail-closed strictness is preserved

- L1 invariant intact: still exactly one committed object per service session.
- No nonce reuse: each fresh record derives a new CEK + nonce prefix.
- No protocol/wire/crypto/state-machine (SCKA/ratchet) change; client journal logic only.
- Recovers a previously-stuck state: a `COMMITTED`-but-relay-send-failed transfer was
  unrecoverable before (every retry rejected); it now re-sends cleanly.

## Tests (deterministic, real service in-process via `common::start_attachment_server`)

`qsl/qsl-client/qsc/tests/na_0617_attachment_single_send_per_session.rs` (4/4 pass):

1. `distinct_files_multi_send_in_one_session_each_succeed` — L2 headline: two distinct
   files in one qsc session each commit and are accepted by the relay.
2. `same_file_resend_after_delivery_starts_fresh_session` — footgun fixed: the second send
   of the same file succeeds with no `REJECT_QATTSVC_SESSION_STATE`.
3. `interrupted_upload_resumes_and_commits` — resume preserved (`UPLOADING` still reused).
4. `same_file_resend_while_awaiting_confirmation_is_blocked_inflight` — in-flight blocking
   preserved (`AWAITING_CONFIRMATION` still found and guarded).

Negative control (pre-fix behavior, class-only): with the predicate reverted to exclude
only `PEER_CONFIRMED`, test (2) fails with `event=file_xfer_reject
code=REJECT_QATTSVC_SESSION_STATE`, demonstrating the test genuinely guards the fix.

Regression: the full `attachment_streaming_na0197c` suite (17 pass, 1 local-only ignored),
including `attachment_upload_resume_and_invalid_resume_token_fail_closed` (which exercises
`UPLOADING` reuse via a bad-resume-token rejection) and
`attachment_e2e_resume_and_peer_confirm_after_persistence`, passes unchanged.

## Claim boundary

Research/demo. No public/production/security-complete/attachment-complete/bug-free/
vulnerability-free claim. This lane fixes one client resend footgun and documents the
two-layer model; it does not alter the L1 service contract or any protocol/crypto path.
