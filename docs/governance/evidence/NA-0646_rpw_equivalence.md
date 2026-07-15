# receive_pull_and_write — control-flow equivalence (NA-0646 PR-B, D582 §hard-spot)

Extracted before (PR-A merged, `abb10cab`) and after (PR-B head) function bodies:
`rpw_before.rs` (463 lines, from transport/mod.rs:429) / `rpw_after.rs` (458 lines,
from :428). Full textual diff reproducible with `diff rpw_before.rs rpw_after.rs`.

## Signature

`fn receive_pull_and_write(&ctx, max) -> ReceivePullStats` →
`-> CliResult<ReceivePullStats>`; the success tail `stats` → `Ok(stats)`. The sole
caller chain (receive_execute → run → main) propagates Err with no side effects and
the bin adapter exits(1) — the same terminal outcome the in-place exit had.

## The 8 direct exit sites → `return Err(CliError::code(<same code>))`, same statement

| # | site (D582 census) | before | after | equivalence |
|---|---|---|---|---|
| 1 | pull failure (loop head) | `Err(code) => print_error_marker(code)` | `Err(code) => return Err(CliError::code(code))` | same match arm; no statement between the arm and fn abort in either version |
| 2 | session-store-failed (inside the commit_unpack_state closure) | `emit_marker("error", …); print_error_marker("qsp_session_store_failed")` — NOTE: the ORIGINAL double-emits the error marker (direct emit + funnel emit) before exiting | `emit_marker("error", …); return Err(CliError::code("qsp_session_store_failed"))` — the site keeps the direct emit; the adapter emits the Code marker | double emission REPRODUCED exactly (site emit + adapter emit = the original's two identical lines); abort at the same statement |
| 3 | unpack reason (dup path) | `print_error_marker(reason)` | `return Err(CliError::code(reason))` | same arm; reason string unchanged |
| 4 | legacy-retired | `print_error_marker("legacy_receive_retired_post_w0")` | `return Err(CliError::code(…))` | same statement |
| 5 | unpack reason (second path) | `print_error_marker(reason)` | `return Err(CliError::code(reason))` | same arm |
| 6 | **recv_write_failed — the ENG-0042 seam** | ratchet key durably consumed at commit_unpack_state, THEN payload write; on write failure `print_error_marker("recv_write_failed")` | identical ordering; on write failure `return Err(CliError::code("recv_write_failed"))` | **ENG-0042 PRESERVED, not fixed**: the commit still precedes the write; on failure the fn aborts at the same point with the key consumed and the payload unwritten — one message lost per crash, exactly as before (and as filed) |
| 7 | ack/receipt code site | `print_error_marker(code)` | `return Err(CliError::code(code))` | same statement |
| 8 | attachment-resume failure | `Err(reason) => print_error_marker(reason)` at the call-site match | `attachment_resume_pending_for_peer(ctx, service_url)?` — the fn now returns CliResult, its internal soft errors mapped `CliError::code(<same reason>)` at their origin | the same reason string reaches the adapter's single emit; emission was last-before-exit in the original, and nothing emits during propagation |

## The diverging helper calls → `?` at the same statements

- `commit_unpack_state()` ×9, `record_seen_and_queue_ack(…)` ×9,
  `queue_or_send_receipt(…)` ×3, `flush_batched_receipts(…)` ×1: each previously
  contained (or reached) a funnel exit; each now returns CliResult and is `?`-ed at
  the SAME statement. Before: an error inside them killed the process mid-statement.
  After: Err propagates from the same statement boundary out of the loop and fn.
  **No flush, commit, ack, or emission code between any converted site and the fn
  exit executes in either version** — a `?` early-return skips the receipt-batch
  flush and pending-ack flush exactly as the exit did.

## Loop-state and durability semantics (ENG-0042-adjacent, unchanged)

- `pending_receipts` / `pending_acks` are process-local Vecs: abandoned identically
  (process death before; fn abort + drop after — Drop for Vec<String> is silent).
- Per-item durable commits that completed before the failing statement remain on
  disk in both versions. An Err from this fn leaves partially-committed loop state —
  the documented ENG-0042-adjacent contract for the future GUI caller. NOT fixed here.
- NA-0644 ack-eligibility invariant untouched: record_seen_and_queue_ack still runs
  only after the item's durable commit, and a failed seen-write now propagates
  (fail-closed) exactly where it used to exit (fail-closed).

## Destructor delta (audited crate-wide)

Early-return runs destructors that process::exit skipped. The crate has exactly two
Drop impls: `LockGuard` (flock LOCK_UN — the kernel released the lock on process
death anyway; no output) and `VaultSession` (in-memory zeroize; no output). Neither
emits bytes nor mutates persistent state beyond what process death already implied.
No thread::spawn exists in the crate, so no cross-thread divergence.

## Verdict

Every converted site aborts at the same statement with the same emissions in the
same order (site-level emissions stay at their sites; single-code emissions move to
the bin adapter, which fires before anything else can print). The prover's S1-S6
cases — including the dynamic-reason and recv-write families — plus the full suite
and the NA-0640 e2e guard this empirically; this document is the structural argument.
