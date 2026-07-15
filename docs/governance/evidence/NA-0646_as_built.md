# NA-0646 as-built — Extract qsc-core (D582): PR-A crate split + PR-B exit->Result

Lane: NA-0646, QSL-DIR-2026-07-14-582 (D582, APPROVED), seated by promotion PR #1572.
Base main `d3f4df7d`. Design-lock confirmed against live code + operator-signed-off
2026-07-14 (five precision corrections accepted). Decision D-1269.

## §1 PR-A — the crate split (PR #1573, merged `abb10cab`, 2026-07-15)

Move + visibility ONLY, SEMANTIC DELTA ZERO. src/lib.rs 7 → 2,310 lines (17 pub mod
decls incl. cmd/ wholesale; use blocks + consts + root re-imports verbatim; the 2,155
enumerated main.rs lines at the LIB ROOT so `use super::*`/`use crate::…` keep
resolving — module bodies untouched). main.rs → the 642-line thin bin. 210
machine-verified visibility widenings (74 fn / 30 enum / 11 struct / 8 const / 72
field / 15 mod lines) incl. the D581 KEEP items as the pub seed GUI surface.

Proof: byte-identity prover (14 cases) BEFORE(pre-move) vs AFTER(final tree) `diff -r`
EMPTY, determinism proven by double-run; purity machine-checked (11 module files = 119
changed lines ALL visibility-only; 2,836 verbatim + 94 widened + 46 wiring; 0
unexplained; 0 lines lost); `git diff --color-moved` 2,192 out / 2,183 in; cargo check
--all-targets 0/0; full local suite 405/0/1-pre-existing-ignored across 107 result
sets (= the NA-0645 422 baseline minus exactly the 17 adversarial/envelope unit tests
that ran TWICE under the old double-compile — reconciled BY NAME, no test lost); the
NA-0640 e2e green UNCHANGED (2/0, 116.53s). CI 35 pass / 0 fail / 2 PR-skips.

## §2 PR-B — exit->Result (this PR)

The five funnels return CliError; exit semantics live ONLY in the bin's single
Err->emit+exit adapter (`exit_on`, main.rs). **The lib contains ZERO `process::exit`
sites (audited: `grep -r process::exit src/` matches only main.rs — the adapter's two
arms + util_sanitize's usage exit(2), which stays in the bin per D582 decision 4;
clap's own exit(2) untouched).**

Funnel dispositions (D582 decision 4 — the byte-safest patterns):
- print_error_marker / print_error: DELETED from the lib; the ~214 sites return
  `Err(CliError::code(<same code>))`; the adapter emits the byte-identical
  `QSC_MARK/1 event=error code=…` marker then exits(1). Nothing emits during
  propagation, so the marker stays the LAST line exactly as before.
- require_unlocked -> CliResult: emits the code+kv marker AT SITE (already-emitted),
  returns Err(CliError::Emitted); 38 callers' guard blocks collapsed to `…?;`.
- protocol_inactive_exit -> protocol_inactive_error(reason) -> CliError: emits the
  dynamic-reason marker at site (already-emitted); 5 callers.
- file_xfer_reject -> CliError: emits BOTH markers at site (the file_xfer_reject
  marker THEN the error marker, the original chain order); 26 callers.

The signature cascade: ~80 fn signatures became CliResult<…> (cascade census in the
PR diff). Where soft and fatal error paths coexisted, the ORIGINAL fan-out is
preserved exactly rather than flattened:
- `AttachmentSendError{Soft(String), Fatal(CliError)}` — the upload path's soft
  errors still become file_xfer_reject markers at the caller; the one fatal exit
  (protocol-inactive) propagates. From impls keep every existing `?` untouched.
- `ReceiptSendError{Soft(&'static str), Fatal(CliError)}` — receipt-send soft
  failures still emit *_send_failed and CONTINUE; the encode-failure funnel
  propagates.
- `read_send_state -> CliResult<Result<u64, ()>>` — the soft parse-failure arm still
  produces the send_state_parse_failed outcome; the fatal safe-parents funnel
  propagates.
- `fault_injector_from_env -> CliResult<Option<FaultInjector>>` — absent scenario is
  None as before; the missing/invalid-seed funnels propagate.

## §3 The hard spot — receive_pull_and_write

See `NA-0646_rpw_equivalence.md` (in this directory): the 8 in-loop exits map 1:1 to
`return Err` at the SAME statements; the diverging helpers (commit_unpack_state ×9,
record_seen_and_queue_ack ×9, queue_or_send_receipt ×3, flush_batched_receipts) are
`?`-ed at the same statements; no flush/commit code runs between any converted site
and the fn exit in either version; **ENG-0042 is PRESERVED, not fixed** (the ratchet
commit still precedes the payload write; the recv_write_failed abort leaves the same
on-disk state); the session-store site's pre-existing DOUBLE emission is reproduced
exactly (site emit + adapter emit). Destructor delta audited: the crate's two Drop
impls (LockGuard flock, VaultSession zeroize) are output-free and equivalent to
process-death cleanup; the crate has no thread::spawn.

## §4 The prover (the load-bearing proof) + WF-0017 non-vacuity

- BEFORE re-captured at PR-A's MERGED state (`abb10cab`) — byte-identical to the
  PR-A-era baseline, as expected.
- **PR-B byte-identity: PASS on the first run** — `diff -r before_prb after_prb`
  EMPTY across all 14 cases (stdout + stderr + exit codes; S6 proves util_sanitize
  stayed exit(2)).
- **Non-vacuity (WF-0017): DEMONSTRATED** — the require_unlocked kv order was
  deliberately reversed; the differ went RED (exit 1) on exactly the S3 case:
  `op=contacts_add reason=explicit_unlock_required` vs the reversed order; the
  red diff is preserved (`prover/red_demo_diff.txt`); the demo was REVERTED and
  the 14/14 green re-proven on the reverted tree.

## §5 Gates (PR-B final tree)

- `cargo check --all-targets -p qsc`: 0 errors / 0 warnings.
- Full local `cargo test -p qsc` (niced, --test-threads=3 per operator direction):
  **405 passed / 0 failed / 1 ignored (pre-existing) across all 107 result sets, exit 0** — identical to the PR-A baseline by count and set (zero test-file changes in PR-B; the prover script was PR-A's addition).
- The NA-0640 e2e within the run: **2 passed / 0 failed (117.65s)** (zero e2e edits this lane).
- Every `unused Result` warning was chased to zero: 18 dropped-Result sites
  (the NA-0644 dedup-recording sites among them) got explicit `?` — no silently
  swallowed error paths remain.

## §6 Boundaries held / not claimed

NO behavior change (the prover is the proof); NO protocol/crypto/wire change; NO
GUI code; NO CoreCtx de-globaling (OUTPUT_POLICY, VAULT_UNLOCKED_THIS_RUN,
PROCESS_PASSPHRASE, MARKER_QUEUE, FAULT_IDX + the ~40 env reads stay process-global);
NO ENG-0042 fix (preserved exactly); NO dependency change; NO tui.* rename; NO
state-dependent lib-level tests added (behavior tests keep driving the compiled
binary). NOT claimed: a GUI (next lane), de-globaled context (later lane), ENG-0042
closure (its own lane).

## §7 Named residue (D582 Phase 5)

(a) CoreCtx de-globaling — later lane. (b) the ~30 raw println!/eprintln! sites that
bypass marker routing — GUI-lane residue. (c) the clap-value-enum split IF a separate
qsc-core PACKAGE is ever wanted — deferred. (d) reqwest-is-blocking — Tauri
spawn_blocking, GUI lane. Plus, new this lane: (e) the ~15 `#[allow(dead_code)]`
allowances on the KEEP surface come off when the GUI consumes it.
